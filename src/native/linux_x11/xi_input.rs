#![allow(non_upper_case_globals, non_snake_case)]

use super::{
    libx11::{self, Display, Window, _XPrivDisplay},
    xi_input,
};

pub const XIAllDevices: libc::c_int = 0 as libc::c_int;
pub const XI_RawMotion: libc::c_int = 17 as libc::c_int;
pub const XI_RawMotionMask: libc::c_int = (1 as libc::c_int) << XI_RawMotion;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIEventMask {
    pub deviceid: libc::c_int,
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIValuatorState {
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
    pub values: *mut libc::c_double,
}

pub type Time = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIRawEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub time: Time,
    pub deviceid: libc::c_int,
    pub sourceid: libc::c_int,
    pub detail: libc::c_int,
    pub flags: libc::c_int,
    pub valuators: XIValuatorState,
    pub raw_values: *mut libc::c_double,
}

type XQueryExtension = fn(
    _: *mut Display,
    _: *const libc::c_char,
    _: *mut libc::c_int,
    _: *mut libc::c_int,
    _: *mut libc::c_int,
) -> libc::c_int;
type XIQueryVersion = fn(
    dpy: *mut Display,
    major_version_inout: *mut libc::c_int,
    minor_version_inout: *mut libc::c_int,
) -> libc::c_int;
type XISelectEvents =
    fn(dpy: *mut Display, win: Window, masks: *mut XIEventMask, num_masks: libc::c_int);
type XGetEventData = fn(_: *mut Display, _: *mut libx11::XGenericEventCookie) -> libc::c_int;
type XFreeEventData = fn(_: *mut Display, _: *mut libx11::XGenericEventCookie);

#[derive(Clone)]
pub struct LibXi {
    _module: std::rc::Rc<crate::native::module::Module>,
    _XQueryExtension: XQueryExtension,
    XIQueryVersion: XIQueryVersion,
    XISelectEvents: XISelectEvents,
    XGetEventData: XGetEventData,
    XFreeEventData: XFreeEventData,
    pub xi_extension_opcode: Option<i32>,
}

impl LibXi {
    pub fn try_load() -> Option<LibXi> {
        crate::native::module::Module::load("libXi.so")
            .or_else(|_| crate::native::module::Module::load("libXi.so.6"))
            .map(|module| LibXi {
                _XQueryExtension: module.get_symbol("XQueryExtension").unwrap(),
                XIQueryVersion: module.get_symbol("XIQueryVersion").unwrap(),
                XISelectEvents: module.get_symbol("XISelectEvents").unwrap(),
                XGetEventData: module.get_symbol("XGetEventData").unwrap(),
                XFreeEventData: module.get_symbol("XFreeEventData").unwrap(),
                xi_extension_opcode: None,
                _module: std::rc::Rc::new(module),
            })
            .ok()
    }

    pub unsafe fn query_xi_extension(
        &mut self,
        libx11: &mut libx11::LibX11,
        display: *mut Display,
    ) {
        let mut ev = 0;
        let mut err = 0;
        let mut xi_opcode = 0;

        if (libx11.XQueryExtension)(
            display,
            b"XInputExtension\x00" as *const u8 as *const libc::c_char,
            &mut xi_opcode,
            &mut ev,
            &mut err,
        ) == 0
        {
            return;
        }

        // check the version of XInput
        let mut major = 2;
        let mut minor = 3;
        if (self.XIQueryVersion)(display, &mut major, &mut minor) != 0 {
            return;
        }

        // select events to listen
        let mut mask = XI_RawMotionMask;
        let mut masks = XIEventMask {
            deviceid: XIAllDevices,
            mask_len: ::std::mem::size_of::<libc::c_int>() as _,
            mask: &mut mask as *mut _ as *mut _,
        };

        (self.XISelectEvents)(
            display,
            // this weird pointers is macro expansion of DefaultRootWindow(display)
            (*(*(display as _XPrivDisplay))
                .screens
                .offset((*(display as _XPrivDisplay)).default_screen as isize))
            .root,
            &mut masks,
            1 as libc::c_int,
        );
        self.xi_extension_opcode = Some(xi_opcode);
    }

    /// Get mouse delta from XI_RawMotion's event XGenericEventCookie data
    pub unsafe fn read_cookie(
        &mut self,
        xcookie: &mut libx11::XGenericEventCookie,
        display: *mut Display,
    ) -> (f64, f64) {
        assert!(xcookie.evtype == xi_input::XI_RawMotion);

        (self.XGetEventData)(display, xcookie);

        let raw_event = (*xcookie).data as *mut xi_input::XIRawEvent;

        let dx = *(*raw_event).raw_values;
        let dy = *(*raw_event).raw_values.offset(1);

        (self.XFreeEventData)(display, &mut (*xcookie) as *mut _);

        (dx, dy)
    }
}
