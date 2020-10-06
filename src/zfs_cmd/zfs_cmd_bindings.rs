#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(dead_code)]

include!(concat!(env!("ZFSRS_INCLUDE_DIR"), "/zfs_cmd.rs"));

impl From<bool> for boolean_t {
    fn from(b: bool) -> Self {
        if b {
            Self::B_TRUE
        } else {
            Self::B_FALSE
        }
    }
}

impl From<boolean_t> for bool {
    fn from(b: boolean_t) -> Self {
        b == boolean_t::B_TRUE
    }
}
