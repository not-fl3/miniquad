use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE};
use winapi::um::winuser::CF_UNICODETEXT;
use winapi::um::winuser::{
    CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
};

use std::ptr;

struct ClipboardGuard;
impl ClipboardGuard {
    unsafe fn open() -> Option<Self> {
        let result = OpenClipboard(ptr::null_mut());
        if result == false as _ {
            return None;
        }
        Some(ClipboardGuard)
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
}

unsafe fn get_raw_clipboard() -> Option<Vec<u16>> {
    // https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-the-clipboard

    let guard = ClipboardGuard::open();

    if guard.is_none() {
        println!("Failed to open clipboard");
        return None;
    }

    // Returns a handle to a clipboard object
    let clipboard_data = GetClipboardData(CF_UNICODETEXT);
    if clipboard_data.is_null() {
        return None;
    }

    let data_ptr = GlobalLock(clipboard_data) as *const u16;
    if data_ptr.is_null() {
        return None;
    }
    let data_size = GlobalSize(clipboard_data) as usize;

    let slice = std::slice::from_raw_parts(data_ptr, data_size);
    let len = slice.iter().position(|b| *b == 0).unwrap_or(data_size);

    // search for the first null byte to see where the string ends.

    let mut res = vec![0; len];
    ptr::copy_nonoverlapping(data_ptr, res.as_mut_ptr(), len);

    Some(res)
}

unsafe fn set_raw_clipboard(data: *const u8, len: usize) {
    let guard = ClipboardGuard::open();

    if guard.is_none() {
        println!("Failed to open clipboard");
        return;
    }

    let alloc_handle = GlobalAlloc(GMEM_MOVEABLE, len);

    if alloc_handle.is_null() {
        println!("Failed to set clipboard: memory not allocated");
        return;
    }

    let lock = GlobalLock(alloc_handle) as *mut u8;
    ptr::copy_nonoverlapping(data, lock, len);

    GlobalUnlock(lock as _);
    EmptyClipboard();

    SetClipboardData(CF_UNICODETEXT, alloc_handle);
}

pub unsafe fn set_clipboard_text(text: &str) {
    let text_w = format!("{}\0", text).encode_utf16().collect::<Vec<u16>>();
    set_raw_clipboard(text_w.as_ptr() as _, text_w.len() * 2);
}

pub unsafe fn get_clipboard_text() -> Option<String> {
    get_raw_clipboard().map(|data| String::from_utf16_lossy(&data))
}
