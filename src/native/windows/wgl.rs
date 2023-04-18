#![allow(dead_code, non_snake_case)]
use winapi::{
    shared::{
        minwindef::{INT, UINT},
        windef::{HDC, HGLRC},
    },
    um::{errhandlingapi::GetLastError, wingdi::*},
};

use super::{LibOpengl32, WindowsDisplay};

pub const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
pub const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
pub const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
pub const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
pub const WGL_ACCELERATION_ARB: u32 = 0x2003;
pub const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
pub const WGL_RED_BITS_ARB: u32 = 0x2015;
pub const WGL_RED_SHIFT_ARB: u32 = 0x2016;
pub const WGL_GREEN_BITS_ARB: u32 = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: u32 = 0x2018;
pub const WGL_BLUE_BITS_ARB: u32 = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: u32 = 0x201a;
pub const WGL_ALPHA_BITS_ARB: u32 = 0x201b;
pub const WGL_ALPHA_SHIFT_ARB: u32 = 0x201c;
pub const WGL_ACCUM_BITS_ARB: u32 = 0x201d;
pub const WGL_ACCUM_RED_BITS_ARB: u32 = 0x201e;
pub const WGL_ACCUM_GREEN_BITS_ARB: u32 = 0x201f;
pub const WGL_ACCUM_BLUE_BITS_ARB: u32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: u32 = 0x2021;
pub const WGL_DEPTH_BITS_ARB: u32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB: u32 = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: u32 = 0x2024;
pub const WGL_STEREO_ARB: u32 = 0x2012;
pub const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
pub const WGL_SAMPLES_ARB: u32 = 0x2042;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: u32 = 0x20a9;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: u32 = 0x8252;
pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: u32 = 0x8256;
pub const WGL_NO_RESET_NOTIFICATION_ARB: u32 = 0x8261;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_ARB: u32 = 0x2097;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: u32 = 0;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: u32 = 0x2098;
pub const WGL_COLORSPACE_EXT: u32 = 0x309d;
pub const WGL_COLORSPACE_SRGB_EXT: u32 = 0x3089;
pub const ERROR_INVALID_VERSION_ARB: u32 = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: u32 = 0x2096;
pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: u32 = 0x2054;

type GetPixelFormatAttribivARB =
    extern "system" fn(_: HDC, _: INT, _: INT, _: UINT, _: *const INT, _: *mut INT) -> bool;
type GetExtensionsStringEXT = extern "system" fn() -> *const i8;
type GetExtensionsStringARB = extern "system" fn(_: HDC) -> *const i8;
type CreateContextAttribsARB = extern "system" fn(_: HDC, _: HGLRC, _: *const INT) -> HGLRC;
type SwapIntervalEXT = extern "system" fn(_: INT) -> bool;

#[derive(Copy, Clone)]
pub struct GlFbconfig {
    pub red_bits: i32,
    pub green_bits: i32,
    pub blue_bits: i32,
    pub alpha_bits: i32,
    pub depth_bits: i32,
    pub stencil_bits: i32,
    pub samples: i32,
    pub doublebuffer: bool,
    pub handle: u32,
}

impl Default for GlFbconfig {
    fn default() -> Self {
        // -1 means "don't care"
        GlFbconfig {
            red_bits: -1,
            green_bits: -1,
            blue_bits: -1,
            alpha_bits: -1,
            depth_bits: -1,
            stencil_bits: -1,
            samples: -1,
            doublebuffer: false,
            handle: 0,
        }
    }
}

pub unsafe fn gl_choose_fbconfig(
    desired: &mut GlFbconfig,
    alternatives: &[GlFbconfig],
) -> Option<usize> {
    let mut missing: i32;
    let mut least_missing: i32 = 1000000;
    let mut color_diff: i32;
    let mut least_color_diff: i32 = 10000000;
    let mut extra_diff: i32;
    let mut least_extra_diff: i32 = 10000000;
    let mut closest = None;

    for (i, current) in alternatives.iter().enumerate() {
        if desired.doublebuffer == current.doublebuffer {
            missing = 0;
            if desired.alpha_bits > 0 && current.alpha_bits == 0 {
                missing += 1;
            }
            if desired.depth_bits > 0 && current.depth_bits == 0 {
                missing += 1;
            }
            if desired.stencil_bits > 0 && current.stencil_bits == 0 {
                missing += 1;
            }
            if desired.samples > 0 && current.samples == 0 {
                // Technically, several multisampling buffers could be
                //  involved, but that's a lower level implentation detail and
                //  not important to us here, so we count them as one

                missing += 1;
            }

            // These polynomials make many small channel size differences matter
            //  less than one large channel size difference
            //  Calculate color channel size difference value

            color_diff = 0;
            if desired.red_bits != -1 {
                color_diff +=
                    (desired.red_bits - current.red_bits) * (desired.red_bits - current.red_bits);
            }
            if desired.green_bits != -1 {
                color_diff += (desired.green_bits - current.green_bits)
                    * (desired.green_bits - current.green_bits)
            }
            if desired.blue_bits != -1 {
                color_diff += (desired.blue_bits - current.blue_bits)
                    * (desired.blue_bits - current.blue_bits)
            }

            // Calculate non-color channel size difference value
            extra_diff = 0;
            if desired.alpha_bits != -1 {
                extra_diff += (desired.alpha_bits - current.alpha_bits)
                    * (desired.alpha_bits - current.alpha_bits)
            }
            if desired.depth_bits != -1 {
                extra_diff += (desired.depth_bits - current.depth_bits)
                    * (desired.depth_bits - current.depth_bits);
            }
            if desired.stencil_bits != -1 {
                extra_diff = (desired.stencil_bits - current.stencil_bits)
                    * (desired.stencil_bits - current.stencil_bits);
            }
            if desired.samples != -1 {
                extra_diff +=
                    (desired.samples - current.samples) * (desired.samples - current.samples);
            }
            if missing < least_missing {
                closest = Some(i);
            } else if missing == least_missing {
                if color_diff < least_color_diff
                    || color_diff == least_color_diff && extra_diff < least_extra_diff
                {
                    closest = Some(i);
                }
            }

            // Figure out if the current one is better than the best one found so far
            //  Least number of missing buffers is the most important heuristic,
            //  then color buffer size match and lastly size match for other buffers

            if closest.map_or(false, |closest| closest == i) {
                least_missing = missing;
                least_color_diff = color_diff;
                least_extra_diff = extra_diff
            }
        }
    }
    closest
}

pub struct Wgl {
    GetPixelFormatAttribivARB: Option<GetPixelFormatAttribivARB>,
    GetExtensionsStringEXT: Option<GetExtensionsStringEXT>,
    GetExtensionsStringARB: Option<GetExtensionsStringARB>,
    CreateContextAttribsARB: Option<CreateContextAttribsARB>,
    SwapIntervalEXT: Option<SwapIntervalEXT>,

    arb_multisample: bool,
    arb_create_context: bool,
    arb_create_context_profile: bool,
    ext_swap_control: bool,
    arb_pixel_format: bool,
}

unsafe fn get_wgl_proc_address<T>(libopengl32: &mut LibOpengl32, proc: &str) -> Option<T> {
    let proc = std::ffi::CString::new(proc).unwrap();
    let proc = (libopengl32.wglGetProcAddress)(proc.as_ptr() as *const _);

    if proc.is_null() {
        return None;
    }
    return Some(std::mem::transmute_copy(&proc));
}

impl Wgl {
    pub(crate) unsafe fn new(display: &mut WindowsDisplay) -> Wgl {
        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        pfd.nSize = std::mem::size_of_val(&pfd) as _;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 24;
        if SetPixelFormat(
            display.msg_dc,
            ChoosePixelFormat(display.msg_dc, &pfd),
            &pfd,
        ) == 0
        {
            panic!("WGL: failed to set pixel format for dummy context");
        }
        let rc = (display.libopengl32.wglCreateContext)(display.msg_dc);
        if rc.is_null() {
            panic!("WGL: Failed to create dummy context");
        }
        if (display.libopengl32.wglMakeCurrent)(display.msg_dc, rc) == false {
            panic!("WGL: Failed to make context current");
        }

        let GetExtensionsStringEXT: Option<GetExtensionsStringEXT> =
            get_wgl_proc_address(&mut display.libopengl32, "wglGetExtensionsStringEXT");
        let GetExtensionsStringARB: Option<GetExtensionsStringARB> =
            get_wgl_proc_address(&mut display.libopengl32, "wglGetExtensionsStringARB");
        let CreateContextAttribsARB: Option<CreateContextAttribsARB> =
            get_wgl_proc_address(&mut display.libopengl32, "wglCreateContextAttribsARB");
        let SwapIntervalEXT: Option<SwapIntervalEXT> =
            get_wgl_proc_address(&mut display.libopengl32, "wglSwapIntervalEXT");
        let GetPixelFormatAttribivARB: Option<GetPixelFormatAttribivARB> =
            get_wgl_proc_address(&mut display.libopengl32, "wglGetPixelFormatAttribivARB");

        let wgl_ext_supported = |ext: &str| -> bool {
            if let Some(getExtensionsStringEXT) = GetExtensionsStringEXT {
                let extensions = getExtensionsStringEXT();

                if extensions.is_null() == false {
                    let extensions_string = std::ffi::CStr::from_ptr(extensions).to_string_lossy();
                    if extensions_string.contains(ext) {
                        return true;
                    }
                }
            }

            if let Some(getExtensionsStringARB) = GetExtensionsStringARB {
                let extensions = getExtensionsStringARB((display.libopengl32.wglGetCurrentDC)());
                if extensions.is_null() == false {
                    let extensions_string = std::ffi::CStr::from_ptr(extensions).to_string_lossy();

                    if extensions_string.contains(ext) {
                        return true;
                    }
                }
            }
            return false;
        };

        let arb_multisample = wgl_ext_supported("WGL_ARB_multisample");
        let arb_create_context = wgl_ext_supported("WGL_ARB_create_context");
        let arb_create_context_profile = wgl_ext_supported("WGL_ARB_create_context_profile");
        let ext_swap_control = wgl_ext_supported("WGL_EXT_swap_control");
        let arb_pixel_format = wgl_ext_supported("WGL_ARB_pixel_format");
        assert!(arb_pixel_format, "WGL_ARB_pixel_format is required");

        (display.libopengl32.wglDeleteContext)(rc);

        Wgl {
            GetPixelFormatAttribivARB,
            GetExtensionsStringEXT,
            GetExtensionsStringARB,
            CreateContextAttribsARB,
            SwapIntervalEXT,

            arb_multisample,
            arb_create_context,
            arb_create_context_profile,
            ext_swap_control,
            arb_pixel_format,
        }
    }

    unsafe fn wgl_attrib(
        &self,
        display: &mut WindowsDisplay,
        pixel_format: i32,
        attrib: i32,
    ) -> i32 {
        let mut value = 0;
        if !(self.GetPixelFormatAttribivARB.unwrap())(
            display.dc,
            pixel_format,
            0,
            1,
            &attrib,
            &mut value as *mut _,
        ) {
            panic!("WGL: Failed to retrieve pixel format attribute");
        }
        return value;
    }

    unsafe fn wgl_find_pixel_format(&self, display: &mut WindowsDisplay, sample_count: i32) -> u32 {
        let native_count = self.wgl_attrib(display, 1, WGL_NUMBER_PIXEL_FORMATS_ARB as _);
        let mut usable_configs = vec![GlFbconfig::default(); native_count as usize];

        let mut usable_count = 0;
        for i in 0..native_count {
            let n = i + 1;
            let u = &mut usable_configs[usable_count];
            *u = Default::default();
            if self.wgl_attrib(display, n, WGL_SUPPORT_OPENGL_ARB as _) == 0
                || self.wgl_attrib(display, n, WGL_DRAW_TO_WINDOW_ARB as _) == 0
            {
                continue;
            }
            if self.wgl_attrib(display, n, WGL_PIXEL_TYPE_ARB as _) != WGL_TYPE_RGBA_ARB as _ {
                continue;
            }
            if self.wgl_attrib(display, n, WGL_ACCELERATION_ARB as _)
                == WGL_NO_ACCELERATION_ARB as _
            {
                continue;
            }
            u.red_bits = self.wgl_attrib(display, n, WGL_RED_BITS_ARB as _);
            u.green_bits = self.wgl_attrib(display, n, WGL_GREEN_BITS_ARB as _);
            u.blue_bits = self.wgl_attrib(display, n, WGL_BLUE_BITS_ARB as _);
            u.alpha_bits = self.wgl_attrib(display, n, WGL_ALPHA_BITS_ARB as _);
            u.depth_bits = self.wgl_attrib(display, n, WGL_DEPTH_BITS_ARB as _);
            u.stencil_bits = self.wgl_attrib(display, n, WGL_STENCIL_BITS_ARB as _);
            if self.wgl_attrib(display, n, WGL_DOUBLE_BUFFER_ARB as _) != 0 {
                u.doublebuffer = true;
            }
            if self.arb_multisample {
                u.samples = self.wgl_attrib(display, n, WGL_SAMPLES_ARB as _);
            }
            u.handle = n as _;
            usable_count += 1;
        }
        assert!(usable_count > 0);

        let mut desired = GlFbconfig::default();
        desired.red_bits = 8;
        desired.green_bits = 8;
        desired.blue_bits = 8;
        desired.alpha_bits = 8;
        desired.depth_bits = 24;
        desired.stencil_bits = 8;
        desired.doublebuffer = true;
        desired.samples = sample_count;
        let closest = gl_choose_fbconfig(&mut desired, &usable_configs[..]);
        let mut pixel_format = 0;
        if let Some(closest) = closest {
            pixel_format = usable_configs[closest].handle;
        }
        pixel_format
    }

    pub(crate) unsafe fn create_context(
        &mut self,
        display: &mut WindowsDisplay,
        sample_count: i32,
        swap_interval: i32,
    ) -> HGLRC {
        let pixel_format = self.wgl_find_pixel_format(display, sample_count);
        if 0 == pixel_format {
            panic!("WGL: Didn't find matching pixel format.");
        }
        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        if DescribePixelFormat(
            display.dc,
            pixel_format as _,
            std::mem::size_of_val(&pfd) as _,
            &mut pfd as *mut _ as _,
        ) == 0
        {
            panic!("WGL: Failed to retrieve PFD for selected pixel format!");
        }
        if SetPixelFormat(display.dc, pixel_format as _, &pfd) == 0 {
            panic!("WGL: Failed to set selected pixel format!");
        }
        if !self.arb_create_context {
            panic!("WGL: ARB_create_context required!\n");
        }
        if !self.arb_create_context_profile {
            panic!("WGL: ARB_create_context_profile required!");
        }

        // CreateContextAttribsARB is supposed to create the context with
        // the highest version version possible
        // but, somehow, sometimes, it creates 2.1 context when 3.2 is in fact available
        // so this is a workaround: try to create 3.2, and if it fails, go for 2.1
        let attrs = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            3,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            2,
            WGL_CONTEXT_FLAGS_ARB,
            WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            0,
            0,
        ];
        let mut gl_ctx = self.CreateContextAttribsARB.unwrap()(
            display.dc,
            std::ptr::null_mut(),
            attrs.as_ptr() as *const _,
        );

        if gl_ctx.is_null() {
            eprintln!("WGL: failed to create 3.2 context, trying 2.1");

            let attrs = [
                WGL_CONTEXT_MAJOR_VERSION_ARB,
                2,
                WGL_CONTEXT_MINOR_VERSION_ARB,
                1,
                WGL_CONTEXT_FLAGS_ARB,
                WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
                0,
                0,
            ];
            gl_ctx = self.CreateContextAttribsARB.unwrap()(
                display.dc,
                std::ptr::null_mut(),
                attrs.as_ptr() as *const _,
            );
        }
        if gl_ctx.is_null() {
            let err = GetLastError();
            if err == (0xc0070000 | ERROR_INVALID_VERSION_ARB) {
                panic!("WGL: Driver does not support OpenGL version 3.3");
            } else if err == (0xc0070000 | ERROR_INVALID_PROFILE_ARB) {
                panic!("WGL: Driver does not support the requested OpenGL profile");
            } else if err == (0xc0070000 | ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB) {
                panic!("WGL: The share context is not compatible with the requested context");
            } else {
                panic!("WGL: Failed to create OpenGL context");
            }
        }
        (display.libopengl32.wglMakeCurrent)(display.dc, gl_ctx);
        if self.ext_swap_control {
            /* FIXME: DwmIsCompositionEnabled() (see GLFW) */
            (self.SwapIntervalEXT.unwrap())(swap_interval);
        }

        gl_ctx
    }
}
