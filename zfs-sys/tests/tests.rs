#[test]
fn prop_init() {
    unsafe { zfs_rs_zfs_sys::zfs_prop_init() };
}
