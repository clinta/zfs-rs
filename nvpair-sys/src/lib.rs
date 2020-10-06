#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]

use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
