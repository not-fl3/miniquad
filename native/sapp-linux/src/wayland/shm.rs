use crate::wayland::{
    wayland_client::{
        wl_buffer, wl_buffer_interface, wl_proxy, wl_proxy_destroy, wl_proxy_marshal,
        wl_proxy_marshal_constructor, wl_shm, wl_shm_format_WL_SHM_FORMAT_ARGB8888, wl_shm_pool,
        wl_shm_pool_interface, WL_SHM_CREATE_POOL, WL_SHM_POOL_CREATE_BUFFER, WL_SHM_POOL_DESTROY,
    },
    SHM,
};

unsafe fn wl_shm_create_pool(wl_shm: *mut wl_shm, fd: i32, size: i32) -> *mut wl_shm_pool {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        wl_shm as _,
        WL_SHM_CREATE_POOL,
        &wl_shm_pool_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        fd,
        size,
    );

    id as *mut _
}

unsafe fn wl_shm_pool_destroy(wl_shm_pool: *mut wl_shm_pool) {
    wl_proxy_marshal(wl_shm_pool as _, WL_SHM_POOL_DESTROY);
    wl_proxy_destroy(wl_shm_pool as _);
}

unsafe fn wl_shm_pool_create_buffer(
    wl_shm_pool: *mut wl_shm_pool,
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    format: i32,
) -> *mut wl_buffer {
    wl_proxy_marshal_constructor(
        wl_shm_pool as _,
        WL_SHM_POOL_CREATE_BUFFER,
        &wl_buffer_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        offset,
        width,
        height,
        stride,
        format,
    ) as _
}

unsafe extern "C" fn create_tmpfile_cloexec(mut tmpname: *mut libc::c_char) -> libc::c_int {
    let mut fd = libc::mkostemp(tmpname, libc::O_CLOEXEC);
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
    mut width: i32,
    mut height: i32,
    mut pixels: &[u8],
) -> *mut wl_buffer {
    let mut pool: *mut wl_shm_pool = std::ptr::null_mut();
    let mut buffer: *mut wl_buffer = std::ptr::null_mut();
    let mut stride = width * 4;
    let mut length = width * height * 4;

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
    pool = wl_shm_create_pool(SHM, fd, length);
    libc::close(fd);

    let mut target = data as *mut u8;
    for i in 0..width * height * 4 {
        *target.offset(i as _) = pixels[i as usize];
    }

    buffer = wl_shm_pool_create_buffer(
        pool,
        0,
        width,
        height,
        stride,
        wl_shm_format_WL_SHM_FORMAT_ARGB8888 as _,
    );

    libc::munmap(data, length as _);

    wl_shm_pool_destroy(pool);
    return buffer;
}
