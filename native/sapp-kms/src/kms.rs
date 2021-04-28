mod gbm;

use crate::egl::*;
use crate::gl::*;
use gbm::*;

use std::option::Option::None;

use libc::{fd_set, printf, timeval};

extern "C" {
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn drmHandleEvent(fd: libc::c_int, evctx_0: drmEventContextPtr) -> libc::c_int;
    #[no_mangle]
    fn drmModeFreeConnector(ptr: drmModeConnectorPtr);
    #[no_mangle]
    fn drmModeFreeEncoder(ptr: drmModeEncoderPtr);
    #[no_mangle]
    fn drmModeGetResources(fd: libc::c_int) -> drmModeResPtr;
    #[no_mangle]
    fn drmModeAddFB(
        fd: libc::c_int,
        width: u32,
        height: u32,
        depth: u8,
        bpp: u8,
        pitch: u32,
        bo_handle: u32,
        buf_id: *mut u32,
    ) -> libc::c_int;
    #[no_mangle]
    fn drmModeRmFB(fd: libc::c_int, bufferId: u32) -> libc::c_int;
    #[no_mangle]
    fn drmModeSetCrtc(
        fd: libc::c_int,
        crtcId: u32,
        bufferId: u32,
        x: u32,
        y: u32,
        connectors: *mut u32,
        count_0: libc::c_int,
        mode: drmModeModeInfoPtr,
    ) -> libc::c_int;
    #[no_mangle]
    fn drmModeGetEncoder(fd: libc::c_int, encoder_id: u32) -> drmModeEncoderPtr;
    #[no_mangle]
    fn drmModeGetConnector(fd: libc::c_int, connectorId: u32) -> drmModeConnectorPtr;
    #[no_mangle]
    fn drmModePageFlip(
        fd: libc::c_int,
        crtc_id: u32,
        fb_id: u32,
        flags: u32,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmEventContext {
    pub version: libc::c_int,
    pub vblank_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub page_flip_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub page_flip_handler2: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: libc::c_uint,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub sequence_handler:
        Option<unsafe extern "C" fn(_: libc::c_int, _: u64, _: u64, _: u64) -> ()>,
}
pub type drmEventContext = _drmEventContext;
pub type drmEventContextPtr = *mut _drmEventContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeRes {
    pub count_fbs: libc::c_int,
    pub fbs: *mut u32,
    pub count_crtcs: libc::c_int,
    pub crtcs: *mut u32,
    pub count_connectors: libc::c_int,
    pub connectors: *mut u32,
    pub count_encoders: libc::c_int,
    pub encoders: *mut u32,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}
pub type drmModeRes = _drmModeRes;
pub type drmModeResPtr = *mut _drmModeRes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeModeInfo {
    pub clock: u32,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub vscan: u16,
    pub vrefresh: u32,
    pub flags: u32,
    pub type_0: u32,
    pub name: [libc::c_char; 32],
}
pub type drmModeModeInfo = _drmModeModeInfo;
pub type drmModeModeInfoPtr = *mut _drmModeModeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeEncoder {
    pub encoder_id: u32,
    pub encoder_type: u32,
    pub crtc_id: u32,
    pub possible_crtcs: u32,
    pub possible_clones: u32,
}
pub type drmModeEncoder = _drmModeEncoder;
pub type drmModeEncoderPtr = *mut _drmModeEncoder;
pub type drmModeConnection = libc::c_uint;
pub const DRM_MODE_UNKNOWNCONNECTION: drmModeConnection = 3;
pub const DRM_MODE_DISCONNECTED: drmModeConnection = 2;
pub const DRM_MODE_CONNECTED: drmModeConnection = 1;
pub type drmModeSubPixel = libc::c_uint;
pub const DRM_MODE_SUBPIXEL_NONE: drmModeSubPixel = 6;
pub const DRM_MODE_SUBPIXEL_VERTICAL_BGR: drmModeSubPixel = 5;
pub const DRM_MODE_SUBPIXEL_VERTICAL_RGB: drmModeSubPixel = 4;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_BGR: drmModeSubPixel = 3;
pub const DRM_MODE_SUBPIXEL_HORIZONTAL_RGB: drmModeSubPixel = 2;
pub const DRM_MODE_SUBPIXEL_UNKNOWN: drmModeSubPixel = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _drmModeConnector {
    pub connector_id: u32,
    pub encoder_id: u32,
    pub connector_type: u32,
    pub connector_type_id: u32,
    pub connection: drmModeConnection,
    pub mmWidth: u32,
    pub mmHeight: u32,
    pub subpixel: drmModeSubPixel,
    pub count_modes: libc::c_int,
    pub modes: drmModeModeInfoPtr,
    pub count_props: libc::c_int,
    pub props: *mut u32,
    pub prop_values: *mut u64,
    pub count_encoders: libc::c_int,
    pub encoders: *mut u32,
}
pub type drmModeConnector = _drmModeConnector;
pub type drmModeConnectorPtr = *mut _drmModeConnector;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
    pub surface: EGLSurface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub dev: *mut gbm_device,
    pub surface: *mut gbm_surface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub fd: libc::c_int,
    pub mode: *mut drmModeModeInfo,
    pub crtc_id: u32,
    pub connector_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drm_fb {
    pub bo: *mut gbm_bo,
    pub fb_id: u32,
}
static mut gl: C2RustUnnamed = C2RustUnnamed {
    display: 0 as *const libc::c_void as *mut libc::c_void,
    config: 0 as *const libc::c_void as *mut libc::c_void,
    context: 0 as *const libc::c_void as *mut libc::c_void,
    surface: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut gbm: C2RustUnnamed_0 = C2RustUnnamed_0 {
    dev: 0 as *const gbm_device as *mut gbm_device,
    surface: 0 as *const gbm_surface as *mut gbm_surface,
};
static mut drm: C2RustUnnamed_1 = C2RustUnnamed_1 {
    fd: 0,
    mode: 0 as *const drmModeModeInfo as *mut drmModeModeInfo,
    crtc_id: 0,
    connector_id: 0,
};
unsafe fn find_crtc_for_encoder(
    resources: *const drmModeRes,
    encoder: *const drmModeEncoder,
) -> Option<u32> {
    let mut i = 0i32;

    while i < (*resources).count_crtcs {
        /* possible_crtcs is a bitmask as described here:
         * https://dvdhrm.wordpress.com/2012/09/13/linux-drm-mode-setting-api
         */
        let crtc_mask: u32 = (1i32 << i) as u32;
        let crtc_id: u32 = *(*resources).crtcs.offset(i as isize);
        if (*encoder).possible_crtcs & crtc_mask != 0 {
            return Some(crtc_id);
        }
        i += 1
    }
    /* no match found */
    return None;
}
unsafe fn find_crtc_for_connector(
    resources: *const drmModeRes,
    connector: *const drmModeConnector,
) -> Option<u32> {
    for i in 0..(*connector).count_encoders {
        let encoder_id = *(*connector).encoders.offset(i as isize);
        let encoder: *mut drmModeEncoder = drmModeGetEncoder(drm.fd, encoder_id);
        if !encoder.is_null() {
            let crtc_id = find_crtc_for_encoder(resources, encoder);
            drmModeFreeEncoder(encoder);
            if crtc_id.is_some() {
                return crtc_id;
            }
        }
    }
    /* no match found */
    return None;
}

unsafe fn try_fd(device: &[u8]) -> Option<*mut drmModeRes> {
    drm.fd = open(device.as_ptr() as *const _, 0o2 as libc::c_int);
    if drm.fd < 0 as libc::c_int {
        println!("could not open drm device");
        return None;
    }
    let resources: *mut drmModeRes = drmModeGetResources(drm.fd);
    if resources.is_null() {
        libc::close(drm.fd);
        printf(
            b"drmModeGetResources failed: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return None;
    }

    return Some(resources);
}

unsafe fn init_drm() -> libc::c_int {
    let mut connector: *mut drmModeConnector = 0 as *mut drmModeConnector;
    let mut encoder: *mut drmModeEncoder = 0 as *mut drmModeEncoder;
    let mut i;
    let mut area;

    let mut resources = try_fd(b"/dev/dri/card0\x00");
    if resources.is_none() {
        resources = try_fd(b"/dev/dri/card1\x00");
    }
    if resources.is_none() {
        resources = try_fd(b"/dev/dri/card2\x00");
    }
    if resources.is_none() {
        panic!("KMS resources not found in either of /dev/dri/card0, /dev/dri/card1 and /dev/dri/card1");
    }
    let resources = resources.unwrap();

    /* find a connected connector: */
    i = 0 as libc::c_int;
    while i < (*resources).count_connectors {
        connector = drmModeGetConnector(drm.fd, *(*resources).connectors.offset(i as isize));
        if (*connector).connection as libc::c_uint
            == DRM_MODE_CONNECTED as libc::c_int as libc::c_uint
        {
            break;
        }
        drmModeFreeConnector(connector);
        connector = 0 as *mut drmModeConnector;
        i += 1
    }
    if connector.is_null() {
        /* we could be fancy and listen for hotplug events and wait for
         * a connector..
         */
        println!("no connected connector!");
        return -1;
    }
    /* find prefered mode or the highest resolution mode: */
    i = 0 as libc::c_int;
    area = 0 as libc::c_int;
    while i < (*connector).count_modes {
        let current_mode: *mut drmModeModeInfo =
            &mut *(*connector).modes.offset(i as isize) as *mut _drmModeModeInfo;
        if (*current_mode).type_0 & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            drm.mode = current_mode
        }
        let current_area: libc::c_int =
            (*current_mode).hdisplay as libc::c_int * (*current_mode).vdisplay as libc::c_int;
        if current_area > area {
            drm.mode = current_mode;
            area = current_area
        }
        i += 1
    }
    if drm.mode.is_null() {
        println!("could not find mode!");
        return -1;
    }
    /* find encoder: */
    i = 0 as libc::c_int;
    while i < (*resources).count_encoders {
        encoder = drmModeGetEncoder(drm.fd, *(*resources).encoders.offset(i as isize));
        if (*encoder).encoder_id == (*connector).encoder_id {
            break;
        }
        drmModeFreeEncoder(encoder);
        encoder = 0 as *mut drmModeEncoder;
        i += 1
    }
    if !encoder.is_null() {
        drm.crtc_id = (*encoder).crtc_id
    } else {
        let crtc_id = find_crtc_for_connector(resources, connector);
        match crtc_id {
            None => {
                println!("no crtc found!");
                return -1;
            }

            Some(crtc_id) if crtc_id == 0 => {
                println!("no crtc found!");
                return -1;
            }
            Some(crtc_id) => {
                drm.crtc_id = crtc_id;
            }
        }
    }
    drm.connector_id = (*connector).connector_id;
    return 0 as libc::c_int;
}

static mut configs: *mut EGLConfig = 0 as *const EGLConfig as *mut EGLConfig;
static mut config_index: libc::c_int = 0;
static mut num_config: EGLint = 0;
static mut count: EGLint = 0 as libc::c_int;

unsafe extern "C" fn init_gbm() -> libc::c_int {
    gbm.dev = gbm_create_device(drm.fd);
    gbm.surface = gbm_surface_create(
        gbm.dev,
        (*drm.mode).hdisplay as _,
        (*drm.mode).vdisplay as _,
        ('X' as i32
            | ('R' as i32) << 8 as libc::c_int
            | ('2' as i32) << 16 as libc::c_int
            | ('4' as i32) << 24 as libc::c_int) as _,
        (gbm_bo_flags_GBM_BO_USE_SCANOUT as libc::c_int
            | gbm_bo_flags_GBM_BO_USE_RENDERING as libc::c_int) as _,
    );
    if gbm.surface.is_null() {
        println!("failed to create gbm surface");
        return -1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_config_to_visual(
    egl_display: EGLDisplay,
    visual_id: EGLint,
    configs_0: *mut EGLConfig,
    count_0: libc::c_int,
) -> libc::c_int {
    let mut id: EGLint = 0;

    for i in 0..count_0 {
        if !(eglGetConfigAttrib(
            egl_display,
            *configs_0.offset(i as isize),
            0x302e as libc::c_int,
            &mut id,
        ) == 0)
        {
            if id == visual_id {
                return i;
            }
        }
    }
    return -1;
}
unsafe extern "C" fn init_gl() -> libc::c_int {
    let major: EGLint = 0;
    let minor: EGLint = 0;
    // let mut vertex_shader: GLuint = 0;
    // let mut fragment_shader: GLuint = 0;
    //let mut ret: GLint = 0;
    static mut context_attribs: [EGLint; 3] = [
        0x3098 as libc::c_int,
        2 as libc::c_int,
        0x3038 as libc::c_int,
    ];
    static mut config_attribs: [EGLint; 15] = [
        0x3033 as libc::c_int,
        0x4 as libc::c_int,
        0x3024 as libc::c_int,
        8 as libc::c_int,
        0x3023 as libc::c_int,
        8 as libc::c_int,
        0x3022 as libc::c_int,
        8 as libc::c_int,
        0x3021 as libc::c_int,
        0 as libc::c_int,
        0x3025 as libc::c_int,
        8 as libc::c_int,
        0x3040 as libc::c_int,
        0x4 as libc::c_int,
        0x3038 as libc::c_int,
    ];
    pub type PFNEGLGETPLATFORMDISPLAYEXTPROC = Option<
        unsafe extern "C" fn(_: EGLenum, _: *mut libc::c_void, _: *const EGLint) -> EGLDisplay,
    >;
    let get_platform_display: PFNEGLGETPLATFORMDISPLAYEXTPROC =
        ::std::mem::transmute::<*mut libc::c_void, PFNEGLGETPLATFORMDISPLAYEXTPROC>(
            ::std::mem::transmute::<__eglMustCastToProperFunctionPointerType, *mut libc::c_void>(
                eglGetProcAddress(b"eglGetPlatformDisplayEXT\x00" as *const u8 as *const _),
            ),
        );
    gl.display = get_platform_display.expect("non-null function pointer")(
        0x31d7 as libc::c_int as EGLenum,
        gbm.dev as *mut libc::c_void,
        0 as *const EGLint,
    );
    if eglInitialize(gl.display, 0 as *mut EGLint, 0 as *mut EGLint) == 0 {
        println!("failed to initialize");
        return -1;
    }
    printf(
        b"Using display %p with EGL version %d.%d\n\x00" as *const u8 as *const libc::c_char,
        gl.display,
        major,
        minor,
    );
    printf(
        b"EGL Version \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3054 as libc::c_int),
    );
    printf(
        b"EGL Vendor \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3053 as libc::c_int),
    );
    printf(
        b"EGL Extensions \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        eglQueryString(gl.display, 0x3055 as libc::c_int),
    );
    if eglBindAPI(0x30a0 as libc::c_int as EGLenum) == 0 {
        println!("failed to bind api EGL_OPENGL_ES_API");
        return -1;
    }

    eglGetConfigs(
        gl.display,
        0 as *mut EGLConfig,
        0 as libc::c_int,
        &mut count,
    );
    configs = libc::malloc(count as usize * ::std::mem::size_of::<EGLConfig>() as usize)
        as *mut EGLConfig;
    eglChooseConfig(
        gl.display,
        config_attribs.as_ptr(),
        configs,
        count,
        &mut num_config,
    );
    config_index = match_config_to_visual(
        gl.display,
        ('X' as i32
            | ('R' as i32) << 8 as libc::c_int
            | ('2' as i32) << 16 as libc::c_int
            | ('4' as i32) << 24 as libc::c_int) as EGLint,
        configs,
        num_config,
    );
    gl.context = eglCreateContext(
        gl.display,
        *configs.offset(config_index as isize),
        0 as EGLContext,
        context_attribs.as_ptr(),
    );
    if gl.context.is_null() {
        println!("failed to create context");
        return -1;
    }
    gl.surface = eglCreateWindowSurface(
        gl.display,
        *configs.offset(config_index as isize),
        gbm.surface as EGLNativeWindowType,
        0 as *const EGLint,
    );
    if gl.surface.is_null() {
        println!("failed to create egl surface");
        return -1;
    }
    /* connect the context to the surface */
    eglMakeCurrent(gl.display, gl.surface, gl.surface, gl.context);
    printf(
        b"GL Extensions: \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        glGetString(0x1f03 as libc::c_int as GLenum),
    );
    free(configs as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn drm_fb_destroy_callback(_bo_0: *mut gbm_bo, data: *mut libc::c_void) {
    let fb_0: *mut drm_fb = data as *mut drm_fb;
    if (*fb_0).fb_id != 0 {
        drmModeRmFB(drm.fd, (*fb_0).fb_id);
    }
    free(fb_0 as *mut libc::c_void);
}
unsafe extern "C" fn drm_fb_get_from_bo(bo_0: *mut gbm_bo) -> *mut drm_fb {
    let mut fb_0: *mut drm_fb = gbm_bo_get_user_data(bo_0) as *mut drm_fb;
    let width;
    let height;
    let stride;
    let handle;
    let ret;

    if !fb_0.is_null() {
        return fb_0;
    }
    fb_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<drm_fb>() as libc::c_ulong,
    ) as *mut drm_fb;
    (*fb_0).bo = bo_0;
    width = gbm_bo_get_width(bo_0);
    height = gbm_bo_get_height(bo_0);
    stride = gbm_bo_get_stride(bo_0);
    handle = gbm_bo_get_handle(bo_0).u32_;
    ret = drmModeAddFB(
        drm.fd,
        width,
        height,
        24,
        32,
        stride,
        handle,
        &mut (*fb_0).fb_id,
    );
    if ret != 0 {
        printf(
            b"failed to create fb: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        free(fb_0 as *mut libc::c_void);
        return 0 as *mut drm_fb;
    }
    gbm_bo_set_user_data(
        bo_0,
        fb_0 as *mut libc::c_void,
        Some(
            drm_fb_destroy_callback
                as unsafe extern "C" fn(_: *mut gbm_bo, _: *mut libc::c_void) -> (),
        ),
    );
    return fb_0;
}
unsafe extern "C" fn page_flip_handler(
    _fd: libc::c_int,
    _frame: libc::c_uint,
    _sec: libc::c_uint,
    _usec: libc::c_uint,
    data: *mut libc::c_void,
) {
    let waiting_for_flip: *mut libc::c_int = data as *mut libc::c_int;
    *waiting_for_flip = 0;
}

static mut bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
static mut fb: *mut drm_fb = 0 as *const drm_fb as *mut drm_fb;
static mut next_bo: *mut gbm_bo = 0 as *const gbm_bo as *mut gbm_bo;
static mut evctx: drmEventContext = _drmEventContext {
    version: 4 as libc::c_int,
    vblank_handler: None,
    page_flip_handler: Some(
        page_flip_handler
            as unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_uint,
                _: libc::c_uint,
                _: libc::c_uint,
                _: *mut libc::c_void,
            ) -> (),
    ),
    page_flip_handler2: None,
    sequence_handler: None,
};
#[no_mangle]
pub unsafe extern "C" fn swap_buffers() {
    let mut fds: fd_set = std::mem::zeroed();
    
    libc::FD_ZERO(&mut fds as *mut _);
    libc::FD_SET(0, &mut fds as *mut _);
    libc::FD_SET(drm.fd, &mut fds as *mut _);

    let mut ret;
    let mut waiting_for_flip: libc::c_int = 1 as libc::c_int;
    eglSwapBuffers(gl.display, gl.surface);
    next_bo = gbm_surface_lock_front_buffer(gbm.surface);
    fb = drm_fb_get_from_bo(next_bo);
    ret = drmModePageFlip(
        drm.fd,
        drm.crtc_id,
        (*fb).fb_id,
        0x1,
        &mut waiting_for_flip as *mut libc::c_int as *mut libc::c_void,
    );
    if ret != 0 {
        println!("drmModePageFlip failed");
    }
    while waiting_for_flip != 0 {
        ret = libc::select(
            drm.fd + 1 as libc::c_int,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if ret < 0 as libc::c_int {
            printf(
                b"select err: %s\n\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return;
        } else if ret == 0 as libc::c_int {
            println!("select timeout!");
            return;
        } else if libc::FD_ISSET(0, &mut fds as _) {
            println!("user interrupt");
            break;
        } else {
            drmHandleEvent(drm.fd, &mut evctx);
        }
    }
    /* release last buffer to render on again: */
    gbm_surface_release_buffer(gbm.surface, bo);
    bo = next_bo;
}

#[no_mangle]
pub unsafe extern "C" fn drm_screen_width() -> libc::c_int {
    return (*drm.mode).hdisplay as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn drm_screen_height() -> libc::c_int {
    return (*drm.mode).vdisplay as libc::c_int;
}

pub unsafe fn init() -> libc::c_int {
    let mut ret = init_drm();
    if ret != 0 {
        println!("failed to initialize DRM");
        return ret;
    }
    ret = init_gbm();
    if ret != 0 {
        println!("failed to initialize GBM");
        return ret;
    }
    ret = init_gl();
    if ret != 0 {
        println!("failed to initialize EGL");
        return ret;
    }
    /* clear the color buffer */
    glClearColor(
        0.5f64 as GLfloat,
        0.5f64 as GLfloat,
        0.5f64 as GLfloat,
        1.0f64 as GLfloat,
    );
    glClear(0x4000 as libc::c_int as GLbitfield);
    eglSwapBuffers(gl.display, gl.surface);
    bo = gbm_surface_lock_front_buffer(gbm.surface);
    fb = drm_fb_get_from_bo(bo);

    ret = drmModeSetCrtc(
        drm.fd,
        drm.crtc_id,
        (*fb).fb_id,
        0,
        0,
        &mut drm.connector_id,
        1 as libc::c_int,
        drm.mode,
    );
    if ret != 0 {
        printf(
            b"failed to set mode: %s\n\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return ret;
    }
    return ret;
}
