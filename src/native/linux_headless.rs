//! `Conf::headless` on Linux: a run with no window and no display server.
//!
//! EGL can hand out a context without a window system at all — a pbuffer
//! surface off `EGL_DEFAULT_DISPLAY` — so a test suite or a CI machine with
//! no X11 or Wayland socket can still drive `update`/`draw` and read the
//! framebuffer back. That is the whole of what this module does; the windowed
//! X11 and Wayland backends are untouched.
//!
//! The shape deliberately matches the macOS headless path: bind an offscreen
//! framebuffer of `window_width` x `window_height` *before* the event handler
//! is built, so the rendering backend adopts it as the default framebuffer and
//! default passes, `screen_size()` and readback all work unchanged above
//! miniquad. The dpi scale is 1 and `sample_count` is ignored.

use crate::{
    conf::Conf,
    event::EventHandler,
    native::{egl, gl, NativeDisplayData, Request},
};

use std::time::{Duration, Instant};

/// A headless run answers the clipboard like a machine with no clipboard,
/// rather than loading X11 just to have one.
struct HeadlessClipboard;

impl crate::native::Clipboard for HeadlessClipboard {
    fn get(&mut self) -> Option<String> {
        None
    }
    fn set(&mut self, _: &str) {}
}

/// The framebuffer a headless run draws into.
///
/// The pbuffer surface exists only to make a context current — EGL requires a
/// surface for that — and is never drawn to or swapped. Everything lands in
/// this framebuffer object instead, for the same reason the macOS path uses
/// one: its size is exactly `window_width` x `window_height` regardless of
/// what surface sizes the driver is willing to give, and its object ids never
/// change, so a resize re-specifies the renderbuffers' storage in place while
/// the backend keeps holding the fbo id.
struct OffscreenTarget {
    fbo: gl::GLuint,
    color: gl::GLuint,
    depth_stencil: gl::GLuint,
}

impl OffscreenTarget {
    /// A complete RGBA8 + depth24/stencil8 framebuffer, left bound.
    unsafe fn new(width: i32, height: i32) -> Self {
        let mut fbo = 0;
        gl::glGenFramebuffers(1, &mut fbo);
        gl::glBindFramebuffer(gl::GL_FRAMEBUFFER, fbo);

        let mut color = 0;
        gl::glGenRenderbuffers(1, &mut color);
        let mut depth_stencil = 0;
        gl::glGenRenderbuffers(1, &mut depth_stencil);

        let target = Self {
            fbo,
            color,
            depth_stencil,
        };
        target.allocate(width, height);

        gl::glFramebufferRenderbuffer(
            gl::GL_FRAMEBUFFER,
            gl::GL_COLOR_ATTACHMENT0,
            gl::GL_RENDERBUFFER,
            color,
        );
        gl::glFramebufferRenderbuffer(
            gl::GL_FRAMEBUFFER,
            gl::GL_DEPTH_STENCIL_ATTACHMENT,
            gl::GL_RENDERBUFFER,
            depth_stencil,
        );

        let status = gl::glCheckFramebufferStatus(gl::GL_FRAMEBUFFER);
        assert!(
            status == gl::GL_FRAMEBUFFER_COMPLETE,
            "miniquad: the headless framebuffer is incomplete (status {:#x})",
            status
        );
        target
    }

    /// (Re)specify both renderbuffers at a size. Storage may be respecified
    /// on an existing renderbuffer, so the ids the backend knows stay valid.
    unsafe fn allocate(&self, width: i32, height: i32) {
        gl::glBindRenderbuffer(gl::GL_RENDERBUFFER, self.color);
        gl::glRenderbufferStorage(gl::GL_RENDERBUFFER, gl::GL_RGBA8, width, height);
        gl::glBindRenderbuffer(gl::GL_RENDERBUFFER, self.depth_stencil);
        gl::glRenderbufferStorage(gl::GL_RENDERBUFFER, gl::GL_DEPTH24_STENCIL8, width, height);
        gl::glBindRenderbuffer(gl::GL_RENDERBUFFER, 0);
    }
}

#[derive(Debug)]
pub enum HeadlessError {
    /// `libEGL.so` is not installed.
    LibraryNotFound,
    /// The driver gave no display for `EGL_DEFAULT_DISPLAY`.
    NoDisplay,
    InitializeFailed,
    /// No config the driver offers can render to a pbuffer at RGBA8.
    NoConfig,
    CreateContextFailed,
    CreateSurfaceFailed,
    MakeCurrentFailed,
}

impl std::fmt::Display for HeadlessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            Self::LibraryNotFound => "libEGL.so could not be loaded",
            Self::NoDisplay => "eglGetDisplay(EGL_DEFAULT_DISPLAY) gave no display",
            Self::InitializeFailed => "eglInitialize failed",
            Self::NoConfig => "no EGL config can render to a pbuffer",
            Self::CreateContextFailed => "eglCreateContext failed",
            Self::CreateSurfaceFailed => "eglCreatePbufferSurface failed",
            Self::MakeCurrentFailed => "eglMakeCurrent failed",
        };
        write!(f, "miniquad: headless EGL: {what}")
    }
}

impl std::error::Error for HeadlessError {}

/// Open and initialise an EGL display that needs no window system.
///
/// Two ways, in order, because the obvious one is not enough on a headless
/// Linux box:
///
/// 1. **`EGL_PLATFORM_SURFACELESS_MESA`**, through `eglGetPlatformDisplayEXT`.
///    This names a platform that is not a window system, which is exactly the
///    situation.
/// 2. **`EGL_DEFAULT_DISPLAY`**, the fallback. On a driver without the
///    surfaceless extension — or a non-Mesa one that offers a usable default
///    anyway — this is still the right call.
///
/// The first is tried first because `EGL_DEFAULT_DISPLAY` is actively wrong on
/// a stock Linux image: the driver reads it as "the X11 display", hands back a
/// non-null `EGLDisplay`, and then `eglInitialize` fails because no X server
/// is listening. That failure is what a CI executor with no display hits, and
/// it looks like a broken driver rather than a missing window system.
///
/// Whichever opens, the initialised `EGLDisplay` is returned. Which route
/// answered is reported only when `MINIQUAD_HEADLESS_VERBOSE` is set: a
/// headless run's stdout and stderr are usually a test's captured output, and
/// an unconditional line there is indistinguishable from what the run was
/// meant to print.
unsafe fn open_display(egl_lib: &mut egl::LibEgl) -> Result<egl::EGLDisplay, HeadlessError> {
    let get_platform_display: Option<egl::GetPlatformDisplayExt> = {
        let name = std::ffi::CString::new("eglGetPlatformDisplayEXT").unwrap();
        (egl_lib.eglGetProcAddress)(name.as_ptr() as _)
            .map(|f| std::mem::transmute::<_, egl::GetPlatformDisplayExt>(f))
    };

    if let Some(get_platform_display) = get_platform_display {
        let display = get_platform_display(
            egl::EGL_PLATFORM_SURFACELESS_MESA as egl::EGLint,
            std::ptr::null_mut(),
            std::ptr::null(),
        );
        if display != egl::EGL_NO_DISPLAY
            && (egl_lib.eglInitialize)(display, std::ptr::null_mut(), std::ptr::null_mut()) != 0
        {
            report("EGL_PLATFORM_SURFACELESS_MESA");
            return Ok(display);
        }
    }

    let display = (egl_lib.eglGetDisplay)(egl::EGL_DEFAULT_DISPLAY);
    if display == egl::EGL_NO_DISPLAY {
        return Err(HeadlessError::NoDisplay);
    }
    if (egl_lib.eglInitialize)(display, std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
        return Err(HeadlessError::InitializeFailed);
    }
    report("EGL_DEFAULT_DISPLAY");
    Ok(display)
}

/// Name the EGL platform that answered, but only when asked.
///
/// A headless run is normally a child process whose output some test is
/// reading, so this stays quiet unless `MINIQUAD_HEADLESS_VERBOSE` is set.
fn report(platform: &str) {
    if std::env::var_os("MINIQUAD_HEADLESS_VERBOSE").is_some() {
        eprintln!("miniquad: headless EGL on {platform}");
    }
}

/// Make a current GLES2 context with no window system behind it.
///
/// Distinct from [`egl::create_egl_context`] in three ways that all follow
/// from there being no window: the display comes from [`open_display`] rather
/// than an X11 `Display*`, the config must be `EGL_PBUFFER_BIT` rather than
/// `EGL_WINDOW_BIT`, and the surface is a pbuffer. `sample_count` plays no
/// part: multisampling belongs to the surface, and nothing draws to it.
unsafe fn create_pbuffer_context(
    egl_lib: &mut egl::LibEgl,
    width: i32,
    height: i32,
) -> Result<(egl::EGLDisplay, egl::EGLContext, egl::EGLSurface), HeadlessError> {
    let display = open_display(egl_lib)?;

    // No depth or stencil is asked for, deliberately. Nothing is ever drawn to
    // the pbuffer — the depth and stencil that get used are the offscreen
    // framebuffer's own renderbuffer — so requiring them here would only
    // narrow the candidate set, and a driver that offers no pbuffer config
    // with 24/8 would fail for a buffer this run does not touch.
    #[rustfmt::skip]
    let cfg_attributes = [
        egl::EGL_SURFACE_TYPE, egl::EGL_PBUFFER_BIT,
        egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
        egl::EGL_RED_SIZE, 8,
        egl::EGL_GREEN_SIZE, 8,
        egl::EGL_BLUE_SIZE, 8,
        egl::EGL_ALPHA_SIZE, 8,
        egl::EGL_NONE,
    ];
    let mut config: egl::EGLConfig = egl::null_mut();
    let mut cfg_count: egl::EGLint = 0;
    if (egl_lib.eglChooseConfig)(
        display,
        cfg_attributes.as_ptr() as _,
        &mut config as *mut _,
        1,
        &mut cfg_count as *mut _,
    ) == 0
        || cfg_count < 1
    {
        return Err(HeadlessError::NoConfig);
    }

    let ctx_attributes = [egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE];
    let context = (egl_lib.eglCreateContext)(
        display,
        config,
        /* EGL_NO_CONTEXT */ egl::null_mut(),
        ctx_attributes.as_ptr() as _,
    );
    if context.is_null() {
        return Err(HeadlessError::CreateContextFailed);
    }

    #[rustfmt::skip]
    let surface_attributes = [
        egl::EGL_WIDTH, width as u32,
        egl::EGL_HEIGHT, height as u32,
        egl::EGL_NONE,
    ];
    let surface =
        (egl_lib.eglCreatePbufferSurface)(display, config, surface_attributes.as_ptr() as _);
    if surface == egl::EGL_NO_SURFACE {
        return Err(HeadlessError::CreateSurfaceFailed);
    }

    if (egl_lib.eglMakeCurrent)(display, surface, surface, context) == 0 {
        return Err(HeadlessError::MakeCurrentFailed);
    }

    Ok((display, context, surface))
}

/// `linux_x11::run`/`linux_wayland::run` without a window or a display server.
///
/// `update`/`draw` run at roughly 60 Hz, the same cadence a display would
/// give, so frame counts keep their meaning. No input or window event ever
/// arrives; `set_window_size` is honoured by re-specifying the offscreen
/// renderbuffers in place, and the run ends when the handler orders a quit
/// (or requests one and does not cancel it).
pub fn run<F>(conf: &Conf, f: &mut Option<F>) -> Result<(), HeadlessError>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    unsafe {
        let mut egl_lib = egl::LibEgl::try_load().map_err(|_| HeadlessError::LibraryNotFound)?;

        let mut width = conf.window_width;
        let mut height = conf.window_height;
        let (egl_display, egl_context, egl_surface) =
            create_pbuffer_context(&mut egl_lib, width, height)?;

        gl::load_gl_funcs(|proc| {
            let name = std::ffi::CString::new(proc).unwrap();
            (egl_lib.eglGetProcAddress)(name.as_ptr() as _)
        });

        // Before the handler is built, so the rendering backend adopts it as
        // the default framebuffer.
        let offscreen = OffscreenTarget::new(width, height);

        let (tx, rx) = std::sync::mpsc::channel();
        crate::set_display(NativeDisplayData {
            // A headless run has no monitor to be dense, so the dpi scale is
            // 1 and `high_dpi` is off however the caller set it: a screenshot
            // is then exactly `window_width` x `window_height` pixels.
            dpi_scale: 1.,
            high_dpi: false,
            blocking_event_loop: conf.platform.blocking_event_loop,
            ..NativeDisplayData::new(width, height, tx, Box::new(HeadlessClipboard))
        });

        let mut event_handler = (f.take().unwrap())();
        let mut update_requested = true;

        let frame = Duration::from_secs_f64(1.0 / 60.0);
        while !crate::native_display().lock().unwrap().quit_ordered {
            let started = Instant::now();

            while let Ok(request) = rx.try_recv() {
                match request {
                    Request::ScheduleUpdate => update_requested = true,
                    Request::SetWindowSize {
                        new_width,
                        new_height,
                    } => {
                        width = new_width as i32;
                        height = new_height as i32;
                        offscreen.allocate(width, height);
                        {
                            let mut d = crate::native_display().lock().unwrap();
                            d.screen_width = width;
                            d.screen_height = height;
                        }
                        event_handler.resize_event(width as f32, height as f32);
                    }
                    // Cursor, fullscreen, position, keyboard, IME: with no
                    // window there is nothing to do them to.
                    _ => {}
                }
            }

            // There is no window manager to send a close, so the quit
            // handshake happens here: ask the handler, and go unless it
            // cancelled.
            if crate::native_display().lock().unwrap().quit_requested {
                event_handler.quit_requested_event();
                let mut d = crate::native_display().lock().unwrap();
                if d.quit_requested {
                    d.quit_ordered = true;
                }
            }

            if !conf.platform.blocking_event_loop || update_requested {
                update_requested = false;
                event_handler.update();
                event_handler.draw();
                // There is no drawable to swap; make the frame observable to
                // whoever reads the framebuffer back.
                gl::glFlush();
            }

            if let Some(rest) = frame.checked_sub(started.elapsed()) {
                std::thread::sleep(rest);
            }
        }

        gl::glDeleteFramebuffers(1, &offscreen.fbo);
        gl::glDeleteRenderbuffers(1, &offscreen.color);
        gl::glDeleteRenderbuffers(1, &offscreen.depth_stencil);
        (egl_lib.eglMakeCurrent)(
            egl_display,
            egl::EGL_NO_SURFACE,
            egl::EGL_NO_SURFACE,
            egl::null_mut(),
        );
        (egl_lib.eglDestroySurface)(egl_display, egl_surface);
        (egl_lib.eglDestroyContext)(egl_display, egl_context);
        (egl_lib.eglTerminate)(egl_display);
    }
    Ok(())
}
