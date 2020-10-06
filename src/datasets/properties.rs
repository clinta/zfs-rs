use crate::*;
use num_traits::FromPrimitive;
use std::{
    ffi::{CStr, CString},
    fmt,
    fmt::Display,
    marker::PhantomData,
};
use strum_macros::*;

pub trait Property
where
    Self: Clone + PartialEq<Self> + std::fmt::Display + std::fmt::Debug,
{
    fn name(&self) -> &str;
    fn name_cstr(&self) -> &CStr;
}

pub trait NativeProperty: Property {
    const NAME: &'static str;
    const NAME_CSTR: &'static CStr;
    const COLUMN_NAME: &'static str;
    const RIGHT_ALIGN: bool;
    const HIDDEN: bool;
    const READ_ONLY: bool;
    const SET_ONCE: bool;
}

pub trait InheritableProperty<D: Dataset>: Property {}

pub trait SetableProperty<'a, D: Dataset>: Property {
    type Value: 'a;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()>;
}

pub trait SetableOnCreateProperty<'a, D: Dataset>: Property {
    type Value: 'a;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()>;
}

pub trait DatasetProperty<'a, D>
where
    D: Dataset,
    Self: Property,
{
    /// The type of value for this property
    type Value: Copy + PartialEq<Self::Value> + std::fmt::Debug + Display + 'a;

    /// Lookup this property on the dataset
    fn lookup(&self, dataset: &'a D) -> Result<Option<Value<'a, D, Self>>>;
}

fn lookup_string<'a, P, D>(property: &P, dataset: &'a D) -> Result<Option<Value<'a, D, P>>>
where
    P: DatasetProperty<'a, D, Value = &'a str>,
    D: Dataset,
{
    let RawValue { value, source } = match dataset.raw_property::<&CStr>(property.name_cstr())? {
        Some(v) => v,
        None => return Ok(None),
    };
    let value = value.to_str()?;
    Ok(Some(Value { value, source }))
}

fn lookup_number<'a, P, D>(property: &P, dataset: &D) -> Result<Option<Value<'a, D, P>>>
where
    P: DatasetProperty<'a, D, Value = u64>,
    D: Dataset,
{
    let RawValue { value, source } = match dataset.raw_property::<u64>(property.name_cstr())? {
        Some(v) => v,
        None => return Ok(None),
    };
    Ok(Some(Value { value, source }))
}

fn lookup_index<'a, P, V, D>(property: &P, dataset: &D) -> Result<Option<Value<'a, D, P>>>
where
    P: DatasetProperty<'a, D, Value = V>,
    V: 'a + FromPrimitive + PartialEq + Copy + std::fmt::Debug + std::fmt::Display,
    D: Dataset + 'a,
{
    let RawValue { value, source } = match dataset.raw_property::<u64>(property.name_cstr())? {
        Some(v) => v,
        None => return Ok(None),
    };
    let value = V::from_u64(value).ok_or(Error::BadIndexValue(value))?;
    Ok(Some(Value { value, source }))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawValue<T: NvPairGetValue> {
    value: T,
    source: Source,
}

impl<T: NvPairGetValue> RawValue<T> {
    pub fn new(value: T, source: Source) -> Self {
        Self { value, source }
    }
}

/// A returned value from getting a property
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value<'a, D: Dataset, P: DatasetProperty<'a, D>> {
    value: P::Value,
    source: Source,
}

impl<'a, D: Dataset, P: DatasetProperty<'a, D>> Value<'a, D, P> {
    pub fn new(value: P::Value, source: Source) -> Self {
        Self { value, source }
    }
    pub fn value(&self) -> P::Value {
        self.value
    }
    pub fn source(&self) -> Source {
        self.source
    }
}

impl<'a, D: Dataset, P: DatasetProperty<'a, D>> Display for Value<'a, D, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.value(), self.source())
    }
}

/// SetPropertyValues is a set of values that can be applied to a dataset or applied to new datasets on creation
///
/// May not contain properties that can only be set on create, to include those properties use SetOnCreatePropertyValues
#[derive(Debug)]
pub struct SetPropertyValues<D: Dataset>(pub(crate) NvList, PhantomData<D>);

impl<D: Dataset> SetPropertyValues<D> {
    pub fn add_value<'a, P: SetableProperty<'a, D>>(
        &mut self,
        property: &P,
        value: P::Value,
    ) -> Result<()> {
        property.add_value_to_list(value, self)
    }
}

// See rust #26925
impl<D: Dataset> Default for SetPropertyValues<D> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// SetOnCreatePropertyValues is a set of values that can be used when creating a dataset
///
/// This cannot be used for setting values on existing datasets, becasue it may contain properties
/// that can only be set on creation.
#[derive(Debug)]
pub struct SetOnCreatePropertyValues<D: Dataset>(pub(crate) NvList, PhantomData<D>);

impl<D: Dataset> SetOnCreatePropertyValues<D> {
    pub fn add_value<'a, P: SetableOnCreateProperty<'a, D>>(
        &mut self,
        property: P,
        value: P::Value,
    ) -> Result<()> {
        property.add_value_to_list(value, self)
    }
}

impl<D: Dataset> Default for SetOnCreatePropertyValues<D> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<D: Dataset> From<SetPropertyValues<D>> for SetOnCreatePropertyValues<D> {
    fn from(v: SetPropertyValues<D>) -> Self {
        Self(v.0, PhantomData)
    }
}

include!(concat!(env!("ZFSRS_INCLUDE_DIR"), "/property_sources.rs"));

pub mod native {
    use super::*;
    include!(concat!(env!("ZFSRS_INCLUDE_DIR"), "/native_properties.rs"));
}

/// A ZFS User Property
#[derive(Debug, Clone, PartialEq)]
pub struct UserProperty<'a> {
    name: &'a str,
    name_cstr: CString,
}

impl<'a> UserProperty<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        Ok(Self {
            name,
            name_cstr: CString::new(name)?,
        })
    }
}

impl<'a> Display for UserProperty<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.name, f)
    }
}

impl<'a> Property for UserProperty<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn name_cstr(&self) -> &CStr {
        self.name_cstr.as_ref()
    }
}

impl<'a> DatasetProperty<'a, Filesystem<'a>> for UserProperty<'a> {
    type Value = &'a str;

    fn lookup(&self, dataset: &'a Filesystem) -> Result<Option<Value<'a, Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}

impl<'a> DatasetProperty<'a, Volume<'a>> for UserProperty<'a> {
    type Value = &'a str;

    fn lookup(&self, dataset: &'a Volume) -> Result<Option<Value<'a, Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}

impl<'a> DatasetProperty<'a, Snapshot<'a>> for UserProperty<'a> {
    type Value = &'a str;

    fn lookup(&self, dataset: &'a Snapshot) -> Result<Option<Value<'a, Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}

impl<'a, D: Dataset> InheritableProperty<D> for UserProperty<'a> where Self: DatasetProperty<'a, D> {}

impl<'a, 'b, 'c, D: Dataset> SetableProperty<'a, D> for UserProperty<'b>
where
    Self: DatasetProperty<'c, D>,
{
    type Value = &'a str;

    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}

impl<'a, 'b, 'c, D: Dataset> SetableOnCreateProperty<'a, D> for UserProperty<'b>
where
    Self: DatasetProperty<'c, D>,
{
    type Value = &'a str;

    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
