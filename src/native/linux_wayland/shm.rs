use super::libwayland_client::{
    wl_buffer, wl_proxy, wl_shm, wl_shm_format_WL_SHM_FORMAT_ARGB8888, wl_shm_pool,
    LibWaylandClient, WL_SHM_CREATE_POOL, WL_SHM_POOL_CREATE_BUFFER, WL_SHM_POOL_DESTROY,
};
use crate::wl_request_constructor;

unsafe fn wl_shm_pool_destroy(libwayland: &mut LibWaylandClient, wl_shm_pool: *mut wl_shm_pool) {
    (libwayland.wl_proxy_marshal)(wl_shm_pool as _, WL_SHM_POOL_DESTROY);
    (libwayland.wl_proxy_destroy)(wl_shm_pool as _);
}

unsafe extern "C" fn create_tmpfile_cloexec(tmpname: *mut libc::c_char) -> libc::c_int {
    let fd = libc::mkostemp(tmpname, libc::O_CLOEXEC);
    if fd >= 0 {
        libc::unlink(tmpname);
    }
    return fd;
}

unsafe extern "C" fn create_anonymous_file(size: usize) -> libc::c_int {
    let xdg_folder_path = std::env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
    let filepath = format!("{}/miniquad-shared-XXXXXX", xdg_folder_path);
    let c_filepath = std::ffi::CString::new(filepath).unwrap();
    let fd = create_tmpfile_cloexec(c_filepath.as_ptr() as _);

    if fd < 0 {
        panic!("Cant create temp file");
    }

    let ret = libc::posix_fallocate(fd, 0, size as _);

    if ret != 0 {
        libc::close(fd);
        panic!("Cant create temp file")
    }
    return fd;
}

pub unsafe fn create_shm_buffer(
    libwayland: &mut LibWaylandClient,
    shm: *mut wl_shm,
    width: i32,
    height: i32,
    pixels: &[u8],
) -> *mut wl_buffer {
    let stride = width * 4;
    let length = width * height * 4;

    let fd = create_anonymous_file(length as _);
    if fd < 0 {
        panic!("Failed to create temporary file");
    }
    let data = libc::mmap(
        std::ptr::null_mut(),
        length as _,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0,
    );

    if data == -1 as _ {
        libc::close(fd);
        panic!("Failed to mmap temporary file");
    }

    let pool = wl_request_constructor!(
        libwayland,
        shm,
        WL_SHM_CREATE_POOL,
        libwayland.wl_shm_pool_interface,
        fd,
        length
    );
    libc::close(fd);

    let target = data as *mut u8;
    for i in 0..width * height * 4 {
        *target.offset(i as _) = pixels[i as usize];
    }

    let buffer = wl_request_constructor!(
        libwayland,
        pool,
        WL_SHM_POOL_CREATE_BUFFER,
        libwayland.wl_buffer_interface,
        0,
        width,
        height,
        stride,
        wl_shm_format_WL_SHM_FORMAT_ARGB8888
    );

    libc::munmap(data, length as _);

    wl_shm_pool_destroy(libwayland, pool);
    return buffer;
}
