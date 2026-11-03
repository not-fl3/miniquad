use winapi::shared::minwindef::HGLOBAL;
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

struct GlobalUnlockGuard(HGLOBAL);

impl Drop for GlobalUnlockGuard {
    fn drop(&mut self) {
        unsafe {
            GlobalUnlock(self.0);
        }
    }
}

unsafe fn get_raw_clipboard() -> Option<Vec<u16>> {
    // https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-the-clipboard

    let guard = ClipboardGuard::open();

    if guard.is_none() {
        eprintln!("Failed to open clipboard");
        return None;
    }

    // Returns a handle to a clipboard object
    let clipboard_data = GetClipboardData(CF_UNICODETEXT);
    if clipboard_data.is_null() {
        return None;
    }

    read_clipboard_data(clipboard_data)
}

unsafe fn read_clipboard_data(clipboard_data: HGLOBAL) -> Option<Vec<u16>> {
    let data_ptr = GlobalLock(clipboard_data) as *const u16;
    if data_ptr.is_null() {
        return None;
    }
    let _lock_guard = GlobalUnlockGuard(clipboard_data);
    let data_size = GlobalSize(clipboard_data) / std::mem::size_of::<u16>();

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
        eprintln!("Failed to open clipboard");
        return;
    }

    let alloc_handle = GlobalAlloc(GMEM_MOVEABLE, len);

    if alloc_handle.is_null() {
        eprintln!("Failed to set clipboard: memory not allocated");
        return;
    }

    let lock = GlobalLock(alloc_handle) as *mut u8;
    ptr::copy_nonoverlapping(data, lock, len);

    GlobalUnlock(lock as _);
    EmptyClipboard();

    SetClipboardData(CF_UNICODETEXT, alloc_handle);
}

pub struct WindowsClipboard {}
impl WindowsClipboard {
    pub fn new() -> WindowsClipboard {
        WindowsClipboard {}
    }
}
impl crate::native::Clipboard for WindowsClipboard {
    fn get(&mut self) -> Option<String> {
        unsafe { get_raw_clipboard().map(|data| String::from_utf16_lossy(&data)) }
    }

    fn set(&mut self, data: &str) {
        unsafe {
            let text_w = format!("{}\0", data).encode_utf16().collect::<Vec<u16>>();
            set_raw_clipboard(text_w.as_ptr() as _, text_w.len() * 2);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use winapi::um::winbase::{GlobalFlags, GlobalFree, GMEM_LOCKCOUNT};

    struct Allocation(HGLOBAL);

    impl Drop for Allocation {
        fn drop(&mut self) {
            unsafe {
                GlobalFree(self.0);
            }
        }
    }

    #[test]
    fn reading_clipboard_data_releases_the_memory_lock() {
        unsafe {
            let text = "hello 🦀\0ignored".encode_utf16().collect::<Vec<_>>();
            let allocation = Allocation(GlobalAlloc(GMEM_MOVEABLE, text.len() * 2));
            assert!(!allocation.0.is_null());
            let data = GlobalLock(allocation.0) as *mut u16;
            assert!(!data.is_null());
            ptr::copy_nonoverlapping(text.as_ptr(), data, text.len());
            GlobalUnlock(allocation.0);
            for _ in 0..3 {
                assert_eq!(
                    read_clipboard_data(allocation.0).unwrap(),
                    "hello 🦀".encode_utf16().collect::<Vec<_>>()
                );
                assert_eq!(GlobalFlags(allocation.0) & GMEM_LOCKCOUNT, 0);
            }
        }
    }

    #[test]
    fn unterminated_clipboard_data_stays_within_the_allocation() {
        unsafe {
            let allocation = Allocation(GlobalAlloc(GMEM_MOVEABLE, 16));
            assert!(!allocation.0.is_null());
            let data = GlobalLock(allocation.0) as *mut u8;
            assert!(!data.is_null());
            let size = GlobalSize(allocation.0);
            ptr::write_bytes(data, 0x41, size);
            GlobalUnlock(allocation.0);
            assert_eq!(
                read_clipboard_data(allocation.0).unwrap(),
                vec![0x4141; size / 2]
            );
            assert_eq!(GlobalFlags(allocation.0) & GMEM_LOCKCOUNT, 0);
        }
    }

    #[test]
    fn discarded_clipboard_data_does_not_get_locked() {
        unsafe {
            let allocation = Allocation(GlobalAlloc(GMEM_MOVEABLE, 0));
            assert!(!allocation.0.is_null());
            assert!(read_clipboard_data(allocation.0).is_none());
            assert_eq!(GlobalFlags(allocation.0) & GMEM_LOCKCOUNT, 0);
        }
    }
}
