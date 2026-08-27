#![allow(non_upper_case_globals, non_snake_case)]

use super::{
    libx11::{self, Display, Window, _XPrivDisplay},
    xi_input,
};
use std::collections::HashMap;

pub const XIAllDevices: libc::c_int = 0;
pub const XIAllMasterDevices: libc::c_int = 1;
pub const XI_Motion: libc::c_int = 6;
pub const XI_RawMotion: libc::c_int = 17;
pub const XI_MotionMask: libc::c_int = 1 << XI_Motion;
pub const XI_RawMotionMask: libc::c_int = 1 << XI_RawMotion;

pub const XIValuatorClass: libc::c_int = 2;
pub const XIScrollClass: libc::c_int = 3;
pub const XIScrollTypeVertical: libc::c_int = 1;
pub const XIScrollTypeHorizontal: libc::c_int = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIValuatorClassInfo {
    pub type_0: libc::c_int,
    pub sourceid: libc::c_int,
    pub number: libc::c_int,
    pub label: libx11::Atom,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub value: libc::c_double,
    pub resolution: libc::c_int,
    pub mode: libc::c_int,
}

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
pub struct XIAnyClassInfo {
    pub type_0: libc::c_int,
    pub sourceid: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIScrollClassInfo {
    pub type_0: libc::c_int,
    pub sourceid: libc::c_int,
    pub number: libc::c_int,
    pub scroll_type: libc::c_int,
    pub increment: libc::c_double,
    pub flags: libc::c_int,
}

#[repr(C)]
pub struct XIDeviceInfo {
    pub deviceid: libc::c_int,
    pub name: *mut libc::c_char,
    pub use_0: libc::c_int,
    pub attachment: libc::c_int,
    pub enabled: libc::c_int,
    pub num_classes: libc::c_int,
    pub classes: *mut *mut XIAnyClassInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIModifierState {
    pub base: libc::c_int,
    pub latched: libc::c_int,
    pub locked: libc::c_int,
    pub effective: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIGroupState {
    pub base: libc::c_int,
    pub latched: libc::c_int,
    pub locked: libc::c_int,
    pub effective: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIButtonState {
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
    pub values: *mut libc::c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIDeviceEvent {
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
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: libc::c_double,
    pub root_y: libc::c_double,
    pub event_x: libc::c_double,
    pub event_y: libc::c_double,
    pub flags: libc::c_int,
    pub buttons: XIButtonState,
    pub valuators: XIValuatorState,
    pub mods: XIModifierState,
    pub group: XIGroupState,
}

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

#[derive(Debug, Clone)]
pub struct ScrollValuator {
    pub number: i32,
    pub scroll_type: i32,
    pub increment: f64,
    pub last_value: Option<f64>,
}

use core::ffi::{c_char, c_int};
crate::declare_module!(
    LibXi,
    "libXi.so",
    "libXi.so.6",
    ...
    ...
    pub fn XQueryExtension(*mut Display, *const c_char, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    pub fn XIQueryVersion(*mut Display, *mut c_int, *mut c_int) -> c_int,
    pub fn XISelectEvents(*mut Display, Window, *mut XIEventMask, c_int),
    pub fn XGetEventData(*mut Display, *mut libx11::XGenericEventCookie) -> c_int,
    pub fn XFreeEventData(*mut Display, *mut libx11::XGenericEventCookie),
    pub fn XIQueryDevice(*mut Display, c_int, *mut c_int) -> *mut XIDeviceInfo,
    pub fn XIFreeDeviceInfo(*mut XIDeviceInfo),
    ...
    ...
    pub xi_extension_opcode: Option<i32>,
);

pub unsafe fn get_valuator_value(valuators: &XIValuatorState, valuator_num: i32) -> Option<f64> {
    let byte_idx = (valuator_num / 8) as usize;
    let bit_idx = (valuator_num % 8) as u8;
    if byte_idx >= valuators.mask_len as usize || valuators.mask.is_null() || valuators.values.is_null() {
        return None;
    }
    let mask_byte = *valuators.mask.add(byte_idx);
    if (mask_byte & (1 << bit_idx)) == 0 {
        return None;
    }
    let mut value_idx = 0;
    for b in 0..byte_idx {
        value_idx += (*valuators.mask.add(b)).count_ones() as usize;
    }
    for bit in 0..bit_idx {
        if (mask_byte & (1 << bit)) != 0 {
            value_idx += 1;
        }
    }
    Some(*valuators.values.add(value_idx))
}

impl LibXi {
    pub unsafe fn query_xi_extension(
        &mut self,
        libx11: &mut libx11::LibX11,
        display: *mut Display,
        window: Window,
        scroll_valuators: &mut HashMap<(i32, i32), ScrollValuator>,
    ) -> bool {
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
            eprintln!("XInput extension not found");
            return false;
        }

        // check the version of XInput
        let mut major = 2;
        let mut minor = 2;
        if (self.XIQueryVersion)(display, &mut major, &mut minor) != 0 {
            eprintln!("XIQueryVersion failed");
            return false;
        }

        if major < 2 || (major == 2 && minor < 1) {
            eprintln!(
                "XInput {}.{} does not support smooth scrolling",
                major, minor
            );
            return false;
        }

        let mut num_devices = 0;
        let devices = (self.XIQueryDevice)(display, XIAllDevices, &mut num_devices);
        if !devices.is_null() {
            for i in 0..num_devices {
                let dev = &*devices.add(i as usize);
                let mut initial_valuator_values: HashMap<i32, f64> = HashMap::new();

                for c in 0..dev.num_classes {
                    let class_ptr = *dev.classes.add(c as usize);
                    if !class_ptr.is_null() && (*class_ptr).type_0 == XIValuatorClass {
                        let val_info = &*(class_ptr as *const XIValuatorClassInfo);
                        initial_valuator_values.insert(val_info.number, val_info.value);
                    }
                }

                for c in 0..dev.num_classes {
                    let class_ptr = *dev.classes.add(c as usize);
                    if !class_ptr.is_null() && (*class_ptr).type_0 == XIScrollClass {
                        let scroll_info = &*(class_ptr as *const XIScrollClassInfo);
                        let increment = if scroll_info.increment != 0.0 {
                            scroll_info.increment
                        } else {
                            1.0
                        };
                        let init_val = initial_valuator_values.get(&scroll_info.number).copied();
                        scroll_valuators.insert(
                            (dev.deviceid, scroll_info.number),
                            ScrollValuator {
                                number: scroll_info.number,
                                scroll_type: scroll_info.scroll_type,
                                increment,
                                last_value: init_val,
                            },
                        );
                    }
                }
            }
            (self.XIFreeDeviceInfo)(devices);
        }

        if scroll_valuators.is_empty() {
            return false;
        }

        let root = (*(*(display as _XPrivDisplay))
            .screens
            .offset((*(display as _XPrivDisplay)).default_screen as isize))
        .root;

        let mut raw_mask = XI_RawMotionMask;
        let mut raw_masks = XIEventMask {
            deviceid: XIAllDevices,
            mask_len: ::std::mem::size_of::<libc::c_int>() as _,
            mask: &mut raw_mask as *mut _ as *mut _,
        };
        (self.XISelectEvents)(display, root, &mut raw_masks, 1);

        let mut motion_mask = XI_MotionMask;
        let mut motion_masks = [
            XIEventMask {
                deviceid: XIAllMasterDevices,
                mask_len: ::std::mem::size_of::<libc::c_int>() as _,
                mask: &mut motion_mask as *mut _ as *mut _,
            },
            XIEventMask {
                deviceid: XIAllDevices,
                mask_len: ::std::mem::size_of::<libc::c_int>() as _,
                mask: &mut motion_mask as *mut _ as *mut _,
            },
        ];
        (self.XISelectEvents)(display, window, motion_masks.as_mut_ptr(), 2);

        self.xi_extension_opcode = Some(xi_opcode);
        true
    }

    /// Get mouse delta from XI_RawMotion's event XGenericEventCookie data
    pub unsafe fn read_cookie(
        &mut self,
        xcookie: &mut libx11::XGenericEventCookie,
        display: *mut Display,
    ) -> (f64, f64) {
        assert!(xcookie.evtype == xi_input::XI_RawMotion);

        (self.XGetEventData)(display, xcookie);

        let raw_event = xcookie.data as *mut xi_input::XIRawEvent;

        let ptr = (*raw_event).raw_values as *const f64;
        let (dx, dy) = if !ptr.is_null() {
            (std::ptr::read_unaligned(ptr), std::ptr::read_unaligned(ptr.add(1)))
        } else {
            (0.0, 0.0)
        };

        (self.XFreeEventData)(display, &mut (*xcookie) as *mut _);

        (dx, dy)
    }

    /// Get scroll delta and cursor position from XI_Motion's event XGenericEventCookie data
    pub unsafe fn read_motion_cookie(
        &mut self,
        xcookie: &mut libx11::XGenericEventCookie,
        display: *mut Display,
        scroll_valuators: &mut HashMap<(i32, i32), ScrollValuator>,
    ) -> (f64, f64, f64, f64) {
        assert!(xcookie.evtype == xi_input::XI_Motion);

        if (self.XGetEventData)(display, xcookie) == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let event = xcookie.data as *mut xi_input::XIDeviceEvent;
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut pos_x = 0.0;
        let mut pos_y = 0.0;

        if !event.is_null() {
            let deviceid = (*event).deviceid;
            let sourceid = (*event).sourceid;
            pos_x = (*event).event_x;
            pos_y = (*event).event_y;

            let mut matched_vertical = false;
            let mut matched_horizontal = false;

            for (&(dev_id, val_num), val_info) in scroll_valuators.iter_mut() {
                if dev_id == deviceid {
                    if let Some(curr_val) = get_valuator_value(&(*event).valuators, val_num) {
                        if let Some(last_val) = val_info.last_value {
                            let delta = (curr_val - last_val) / val_info.increment;
                            if val_info.scroll_type == XIScrollTypeVertical && !matched_vertical {
                                dy -= delta;
                                matched_vertical = true;
                            } else if val_info.scroll_type == XIScrollTypeHorizontal && !matched_horizontal {
                                dx -= delta;
                                matched_horizontal = true;
                            }
                        }
                        val_info.last_value = Some(curr_val);
                    }
                }
            }

            if !matched_vertical || !matched_horizontal {
                for (&(dev_id, val_num), val_info) in scroll_valuators.iter_mut() {
                    if dev_id == sourceid {
                        if let Some(curr_val) = get_valuator_value(&(*event).valuators, val_num) {
                            if let Some(last_val) = val_info.last_value {
                                let delta = (curr_val - last_val) / val_info.increment;
                                if val_info.scroll_type == XIScrollTypeVertical && !matched_vertical {
                                    dy -= delta;
                                    matched_vertical = true;
                                } else if val_info.scroll_type == XIScrollTypeHorizontal && !matched_horizontal {
                                    dx -= delta;
                                    matched_horizontal = true;
                                }
                            }
                            val_info.last_value = Some(curr_val);
                        }
                    }
                }
            }
        }

        (self.XFreeEventData)(display, &mut (*xcookie) as *mut _);

        (dx, dy, pos_x, pos_y)
    }
}
