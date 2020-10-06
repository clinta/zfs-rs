use crate::errors::*;
use nvpair_sys as nv;
use std::{ffi::CStr, ffi::CString, ptr};

/// Owned data referenced by an nvlist
#[derive(Debug)]
pub enum NvListData {
    NvListData(Vec<NvListData>),
    NvLst(NvLst),
    CString(CString),
}

/// a wrapped nvlist that must be freed
#[derive(Debug)]
pub struct NvList {
    nvlist: *mut nv::nvlist_t,
    data: Vec<NvListData>,
}

impl Default for NvList {
    fn default() -> Self {
        Self {
            nvlist: unsafe { nv::fnvlist_alloc() },
            data: Vec::new(),
        }
    }
}

impl Drop for NvList {
    fn drop(&mut self) {
        unsafe { nv::fnvlist_free(self.nvlist) }
    }
}

impl From<&mut nv::nvlist_t> for NvList {
    fn from(nvlist: &mut nv::nvlist_t) -> Self {
        Self {
            nvlist,
            data: Vec::new(),
        }
    }
}

/// a wrapped nvlist that must not be freed
#[derive(Debug)]
pub struct NvLst {
    nvlist: *mut nv::nvlist_t,
    data: Vec<NvListData>,
}

impl Default for NvLst {
    fn default() -> Self {
        Self {
            nvlist: unsafe { nv::fnvlist_alloc() },
            data: Vec::new(),
        }
    }
}

impl NvListT for NvList {
    unsafe fn nvlist(&self) -> *mut nv::nvlist_t {
        self.nvlist
    }
    fn data(&self) -> &Vec<NvListData> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Vec<NvListData> {
        &mut self.data
    }
}

impl<'a> NvListT for NvLst {
    unsafe fn nvlist(&self) -> *mut nv::nvlist_t {
        self.nvlist
    }
    fn data(&self) -> &Vec<NvListData> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Vec<NvListData> {
        &mut self.data
    }
}

impl From<*mut nv::nvlist_t> for NvLst {
    fn from(nvlist: *mut nv::nvlist_t) -> Self {
        Self {
            nvlist,
            data: Vec::new(),
        }
    }
}

pub type RawNvList = *mut nv::nvlist_t;

pub fn nvlist_lookup<V: NvPairGetValue>(nvlist: RawNvList, name: &CStr) -> Result<V> {
    unsafe { V::nvlist_lookup(nvlist, &name) }
}

pub trait NvListT: Sized + Default {
    #![allow(dead_code)]

    unsafe fn nvlist(&self) -> *mut nv::nvlist_t;
    fn data(&self) -> &Vec<NvListData>;
    fn data_mut(&mut self) -> &mut Vec<NvListData>;

    fn lookup<V: NvPairGetValue>(&self, name: &CStr) -> Result<V> {
        nvlist_lookup(unsafe { self.nvlist() }, name)
    }

    fn iter(&self) -> NvListIter<Self> {
        NvListIter(self, ptr::null_mut())
    }

    fn add<T: NvPairAddValue>(&mut self, name: &CStr, value: T) -> Result<()> {
        unsafe { value.add_to_nvlist(self, name) }
    }

    fn _dump(&self) {
        fn do_dump<T: NvListT>(nvl: &T, n: usize) {
            for mut v in nvl.iter() {
                if let Ok(NvPairValue::NvLst(nvl2)) = v.value::<NvPairValue>() {
                    println!("{}{:?}:", " ".repeat(n), v.name());
                    do_dump(&nvl2, n + 2);
                } else {
                    println!(
                        "{}{:?}: {:?}",
                        " ".repeat(n),
                        v.name().map(str::to_string),
                        v.value::<NvPairValue>()
                    );
                }
            }
        }
        do_dump(self, 0);
    }

    fn _print_props(&self) {
        const SOURCE: &CStr = ::cstr::cstr!("source");
        const VALUE: &CStr = ::cstr::cstr!("value");
        for mut v in self.iter() {
            if let Ok(NvPairValue::NvLst(nvl)) = v.value::<NvPairValue>() {
                println!(
                    "name: {:?}, source: {:?}, value: {:?}",
                    v.name(),
                    nvl.lookup::<&str>(SOURCE),
                    nvl.lookup::<NvPairValue>(VALUE)
                );
            }
        }
    }
}

pub struct NvListIterator<T: NvListT>(Option<T>, *mut nv::nvpair_t);

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator<NvList>;
    fn into_iter(self) -> Self::IntoIter {
        Some(self).into()
    }
}

impl IntoIterator for NvLst {
    type Item = NvPair;
    type IntoIter = NvListIterator<NvLst>;
    fn into_iter(self) -> Self::IntoIter {
        Some(self).into()
    }
}

// I would prefer to use IntoIterator for this, but specialization is necessary
impl<T: NvListT> From<Option<T>> for NvListIterator<T> {
    fn from(o: Option<T>) -> Self {
        Self(o, ptr::null_mut())
    }
}

impl<T: NvListT> Iterator for NvListIterator<T> {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(nvlist) = self.0.as_mut() {
            self.1 = unsafe { nv::nvlist_next_nvpair(nvlist.nvlist(), self.1) };
            if !self.1.is_null() {
                Some(NvPair(self.1))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct NvListIter<'a, T: NvListT>(&'a T, *mut nv::nvpair_t);

impl<'a, T: NvListT> Iterator for NvListIter<'a, T> {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 = unsafe { nv::nvlist_next_nvpair((self.0).nvlist(), self.1) };
        if !self.1.is_null() {
            Some(NvPair(self.1))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct NvPair(*mut nv::nvpair_t);

impl NvPair {
    pub fn name(&mut self) -> Result<&str> {
        unsafe { CStr::from_ptr(nv::nvpair_name(self.0)) }
            .to_str()
            .map_err(Error::from)
    }

    pub fn value<'a, V: NvPairGetValue + 'a>(&'a mut self) -> Result<V> {
        unsafe { V::nvpair_value(self.0) }
    }
}

pub trait NvPairGetValue: Sized + std::fmt::Debug {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self>;
    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self>;
}

impl<'a> NvPairGetValue for &'a CStr {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        let mut c_str_ptr = ptr::null_mut();
        nv::nvpair_value_string(nvpair, &mut c_str_ptr).as_nvpair_error()?;
        Ok(CStr::from_ptr(c_str_ptr))
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut c_str_ptr = ptr::null_mut();
        nv::nvlist_lookup_string(nvlist, name.as_ptr(), &mut c_str_ptr).as_nvlist_error()?;
        Ok(CStr::from_ptr(c_str_ptr))
    }
}

impl<'a> NvPairGetValue for &'a str {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        let mut c_str_ptr = ptr::null_mut();
        nv::nvpair_value_string(nvpair, &mut c_str_ptr).as_nvpair_error()?;
        CStr::from_ptr(c_str_ptr).to_str().map_err(Error::from)
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut c_str_ptr = ptr::null_mut();
        nv::nvlist_lookup_string(nvlist, name.as_ptr(), &mut c_str_ptr).as_nvlist_error()?;
        CStr::from_ptr(c_str_ptr).to_str().map_err(Error::from)
    }
}

impl NvPairGetValue for u64 {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        let mut i: u64 = 0;
        nv::nvpair_value_uint64(nvpair, &mut i).as_nvpair_error()?;
        Ok(i)
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut i: u64 = 0;
        nv::nvlist_lookup_uint64(nvlist, name.as_ptr(), &mut i).as_nvlist_error()?;
        Ok(i)
    }
}

impl NvPairGetValue for i32 {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        let mut i: i32 = 0;
        nv::nvpair_value_int32(nvpair, &mut i).as_nvpair_error()?;
        Ok(i)
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut i: i32 = 0;
        nv::nvlist_lookup_int32(nvlist, name.as_ptr(), &mut i).as_nvlist_error()?;
        Ok(i)
    }
}

impl NvPairGetValue for NvLst {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        let mut nvlist = ptr::null_mut();
        nv::nvpair_value_nvlist(nvpair, &mut nvlist).as_nvpair_error()?;
        let nvlist = nvlist.as_mut().ok_or(Error::NvPairNullValue)?;
        Ok(NvLst::from(nvlist as *mut _))
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut dst_nvlist = ptr::null_mut();
        nv::nvlist_lookup_nvlist(nvlist, name.as_ptr(), &mut dst_nvlist).as_nvlist_error()?;
        let dst_nvlist = dst_nvlist.as_mut().ok_or(Error::NvPairNullValue)?;
        Ok(NvLst::from(dst_nvlist as *mut _))
    }
}

#[derive(Debug)]
pub enum NvPairValue<'a> {
    String(&'a str),
    Uint64(u64),
    NvLst(NvLst),
}

impl<'a> NvPairGetValue for NvPairValue<'a> {
    unsafe fn nvpair_value(nvpair: *mut nv::nvpair_t) -> Result<Self> {
        match nv::nvpair_type(nvpair) {
            nv::data_type_t::DATA_TYPE_STRING => <&'a str>::nvpair_value(nvpair).map(Self::String),
            nv::data_type_t::DATA_TYPE_UINT64 => <u64>::nvpair_value(nvpair).map(Self::Uint64),
            nv::data_type_t::DATA_TYPE_NVLIST => <NvLst>::nvpair_value(nvpair).map(Self::NvLst),
            t => Err(Error::NvPairUnsupportedType(t)),
        }
    }

    unsafe fn nvlist_lookup(nvlist: *mut nv::nvlist_t, name: &CStr) -> Result<Self> {
        let mut nvpair: *mut nv::nvpair_t = ptr::null_mut();
        nv::nvlist_lookup_nvpair(nvlist, name.as_ptr(), &mut nvpair).as_nvlist_error()?;
        Self::nvpair_value(nvpair)
    }
}

/// Add an value, with a provided name
pub trait NvPairAddValue: Sized {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()>;
}

pub trait NvPairAddAndGetValue: Sized {
    unsafe fn add_to_nvlist_and_get<'a, T: NvListT>(
        self,
        nvlist: &'a mut T,
        name: &CStr,
    ) -> Result<&'a mut Self>;
}

impl<'a> NvPairAddValue for &'a str {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        CString::new(self)?.add_to_nvlist(nvlist, name)
    }
}

impl<'a> NvPairAddValue for &'a CStr {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        nv::nvlist_add_string(nvlist.nvlist(), name.as_ptr(), self.as_ptr()).as_nvlist_error()
    }
}

impl NvPairAddValue for i32 {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        nv::nvlist_add_int32(nvlist.nvlist(), name.as_ptr(), self).as_nvlist_error()
    }
}

impl NvPairAddValue for u64 {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        nv::nvlist_add_uint64(nvlist.nvlist(), name.as_ptr(), self).as_nvlist_error()
    }
}

impl NvPairAddValue for bool {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        if self {
            nv::nvlist_add_boolean(nvlist.nvlist(), name.as_ptr()).as_nvlist_error()
        } else {
            Ok(())
        }
    }
}

impl NvPairAddValue for CString {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        let dst_nvlist = nvlist.nvlist();
        nvlist.data_mut().push(NvListData::CString(self));
        let c_str: &CStr = match nvlist.data_mut().last() {
            Some(NvListData::CString(v)) => v,
            _ => unreachable!("unable to get just pushed CString"),
        };
        nv::nvlist_add_string(dst_nvlist, name.as_ptr(), c_str.as_ptr()).as_nvlist_error()
    }
}

impl NvPairAddValue for &NvList {
    unsafe fn add_to_nvlist<'a, T: NvListT>(self, nvlist: &'a mut T, name: &CStr) -> Result<()> {
        nv::nvlist_add_nvlist(nvlist.nvlist(), name.as_ptr(), self.nvlist()).as_nvlist_error()
    }
}

impl NvPairAddAndGetValue for NvLst {
    unsafe fn add_to_nvlist_and_get<'a, T: NvListT>(
        self,
        nvlist: &'a mut T,
        name: &CStr,
    ) -> Result<&'a mut Self> {
        let dst_nvlist = nvlist.nvlist();
        nvlist.data_mut().push(NvListData::NvLst(self));
        let ret = match nvlist.data_mut().last_mut() {
            Some(NvListData::NvLst(v)) => v,
            _ => unreachable!("unable to get just pushed value"),
        };
        nv::nvlist_add_nvlist(dst_nvlist, name.as_ptr(), ret.nvlist()).as_nvlist_error()?;
        Ok(ret)
    }
}

impl NvPairAddValue for NvLst {
    unsafe fn add_to_nvlist<T: NvListT>(self, nvlist: &mut T, name: &CStr) -> Result<()> {
        self.add_to_nvlist_and_get(nvlist, name)?;
        Ok(())
    }
}

pub trait NvListAddPair {
    fn add_pair_to_nvlist<T: NvListT>(self, nvlist: &mut T) -> Result<()>;
}

// This should be From, but again, specialization is killing me
pub trait ToNvList {
    fn to_nvlist<T: NvListT>(self) -> Result<T>;
}

impl<I: Iterator<Item = V>, V: NvListAddPair> ToNvList for I {
    fn to_nvlist<T: NvListT>(self) -> Result<T> {
        let mut nvlist = T::default();
        for v in self {
            v.add_pair_to_nvlist(&mut nvlist)?;
        }
        Ok(nvlist)
    }
}
