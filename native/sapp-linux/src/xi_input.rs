use crate::_sapp_x11_display;

use crate::x::{Display, Window, _XPrivDisplay};

pub const XIAllDevices: cty::c_int = 0 as cty::c_int;
pub const XI_RawMotion: cty::c_int = 17 as cty::c_int;
pub const XI_RawMotionMask: cty::c_int = (1 as cty::c_int) << XI_RawMotion;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIEventMask {
    pub deviceid: cty::c_int,
    pub mask_len: cty::c_int,
    pub mask: *mut cty::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIValuatorState {
    pub mask_len: cty::c_int,
    pub mask: *mut cty::c_uchar,
    pub values: *mut cty::c_double,
}

pub type Time = cty::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIRawEvent {
    pub type_0: cty::c_int,
    pub serial: cty::c_ulong,
    pub send_event: cty::c_int,
    pub display: *mut Display,
    pub extension: cty::c_int,
    pub evtype: cty::c_int,
    pub time: Time,
    pub deviceid: cty::c_int,
    pub sourceid: cty::c_int,
    pub detail: cty::c_int,
    pub flags: cty::c_int,
    pub valuators: XIValuatorState,
    pub raw_values: *mut cty::c_double,
}

extern "C" {
    pub fn XQueryExtension(
        _: *mut Display,
        _: *const cty::c_char,
        _: *mut cty::c_int,
        _: *mut cty::c_int,
        _: *mut cty::c_int,
    ) -> cty::c_int;

    pub fn XIQueryVersion(
        dpy: *mut Display,
        major_version_inout: *mut cty::c_int,
        minor_version_inout: *mut cty::c_int,
    ) -> cty::c_int;

    pub fn XISelectEvents(
        dpy: *mut Display,
        win: Window,
        masks: *mut XIEventMask,
        num_masks: cty::c_int,
    );

    pub fn XGetEventData(
        _: *mut Display,
        _: *mut crate::x::Xlib_h::XGenericEventCookie,
    ) -> cty::c_int;
    pub fn XFreeEventData(_: *mut Display, _: *mut crate::x::Xlib_h::XGenericEventCookie);

}

fn XISetMask(ptr: *mut (), event: usize) {}

pub unsafe fn query_xi_extension() -> Option<i32> {
    let mut ev = 0;
    let mut err = 0;
    let mut xi_opcode = 0;

    if XQueryExtension(
        _sapp_x11_display,
        b"XInputExtension\x00" as *const u8 as *const cty::c_char,
        &mut xi_opcode,
        &mut ev,
        &mut err,
    ) == 0
    {
        return None;
    }

    // check the version of XInput
    let mut rc = 0;
    let mut major = 2;
    let mut minor = 3;
    if XIQueryVersion(_sapp_x11_display, &mut major, &mut minor) != 0 {
        return None;
    }

    // select events to listen
    let mut mask = XI_RawMotionMask;
    let mut masks = XIEventMask {
        deviceid: XIAllDevices,
        mask_len: ::std::mem::size_of::<cty::c_int>() as _,
        mask: &mut mask as *mut _ as *mut _,
    };
    XISelectEvents(
        _sapp_x11_display,
        // this weird pointers is macro expansion of DefaultRootWindow(_sapp_x11_display)
        (*(*(_sapp_x11_display as _XPrivDisplay))
            .screens
            .offset((*(_sapp_x11_display as _XPrivDisplay)).default_screen as isize))
        .root,
        &mut masks,
        1 as cty::c_int,
    );
    return Some(xi_opcode);
}

/// Get mouse delta from XI_RawMotion's event XGenericEventCookie data
pub unsafe fn read_cookie(xcookie: &mut crate::x::Xlib_h::XGenericEventCookie) -> (f64, f64) {
    assert!(xcookie.evtype == crate::xi_input::XI_RawMotion);

    crate::xi_input::XGetEventData(_sapp_x11_display, xcookie);

    let raw_event = (*xcookie).data as *mut crate::xi_input::XIRawEvent;

    let dx = *(*raw_event).raw_values;
    let dy = *(*raw_event).raw_values.offset(1);

    crate::xi_input::XFreeEventData(_sapp_x11_display, &mut (*xcookie) as *mut _);

    (dx, dy)
}
