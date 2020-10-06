use std::ptr;

#[test]
fn fnvlist_pack() {
    unsafe {
        zfs_rs_nvpair_sys::nvlist_pack(ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 0, 0)
    };
}
