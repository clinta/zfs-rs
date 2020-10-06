#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Creation;
impl NativeProperty for Creation {
    const NAME: &'static str = "creation";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("creation");
    const COLUMN_NAME: &'static str = "CREATION";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Creation {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Creation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Creation>
where
    D: crate::datasets::Dataset,
    Creation: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Creation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Creation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Creation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Bookmark<'a>> for Creation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Bookmark,
    ) -> Result<Option<Value<'a, crate::datasets::Bookmark<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Used;
impl NativeProperty for Used {
    const NAME: &'static str = "used";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("used");
    const COLUMN_NAME: &'static str = "USED";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Used {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Used {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Used>
where
    D: crate::datasets::Dataset,
    Used: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Used {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Used {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Used {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Available;
impl NativeProperty for Available {
    const NAME: &'static str = "available";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("available");
    const COLUMN_NAME: &'static str = "AVAIL";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Available {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Available {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Available>
where
    D: crate::datasets::Dataset,
    Available: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Available {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Available {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Referenced;
impl NativeProperty for Referenced {
    const NAME: &'static str = "referenced";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("referenced");
    const COLUMN_NAME: &'static str = "REFER";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Referenced {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Referenced {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Referenced>
where
    D: crate::datasets::Dataset,
    Referenced: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Referenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Referenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Referenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Compressratio;
impl NativeProperty for Compressratio {
    const NAME: &'static str = "compressratio";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("compressratio");
    const COLUMN_NAME: &'static str = "RATIO";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Compressratio {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Compressratio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Compressratio>
where
    D: crate::datasets::Dataset,
    Compressratio: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Compressratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Compressratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Compressratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mounted;
impl NativeProperty for Mounted {
    const NAME: &'static str = "mounted";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("mounted");
    const COLUMN_NAME: &'static str = "MOUNTED";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Mounted {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Mounted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Mounted>
where
    D: crate::datasets::Dataset,
    Mounted: DatasetProperty<'a, D, Value = values::Mounted>,
{
    fn default() -> Self {
        Self {
            value: values::Mounted::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Mounted {
    type Value = values::Mounted;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Origin;
impl NativeProperty for Origin {
    const NAME: &'static str = "origin";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("origin");
    const COLUMN_NAME: &'static str = "ORIGIN";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Origin {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Origin {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Origin {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quota;
impl NativeProperty for Quota {
    const NAME: &'static str = "quota";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("quota");
    const COLUMN_NAME: &'static str = "QUOTA";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Quota {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Quota {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Quota>
where
    D: crate::datasets::Dataset,
    Quota: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Quota {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Quota
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Quota
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Reservation;
impl NativeProperty for Reservation {
    const NAME: &'static str = "reservation";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("reservation");
    const COLUMN_NAME: &'static str = "RESERV";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Reservation {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Reservation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Reservation>
where
    D: crate::datasets::Dataset,
    Reservation: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Reservation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Reservation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Reservation
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Reservation
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Volsize;
impl NativeProperty for Volsize {
    const NAME: &'static str = "volsize";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("volsize");
    const COLUMN_NAME: &'static str = "VOLSIZE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Volsize {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Volsize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Volsize>
where
    D: crate::datasets::Dataset,
    Volsize: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Volsize {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Volsize {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Volsize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Volsize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Volblocksize;
impl NativeProperty for Volblocksize {
    const NAME: &'static str = "volblocksize";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("volblocksize");
    const COLUMN_NAME: &'static str = "VOLBLOCK";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Volblocksize {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Volblocksize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Volblocksize>
where
    D: crate::datasets::Dataset,
    Volblocksize: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 8192u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Volblocksize {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Volblocksize where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Volblocksize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Recordsize;
impl NativeProperty for Recordsize {
    const NAME: &'static str = "recordsize";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("recordsize");
    const COLUMN_NAME: &'static str = "RECSIZE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Recordsize {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Recordsize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Recordsize>
where
    D: crate::datasets::Dataset,
    Recordsize: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 131072u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Recordsize {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Recordsize where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Recordsize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Recordsize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mountpoint;
impl NativeProperty for Mountpoint {
    const NAME: &'static str = "mountpoint";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("mountpoint");
    const COLUMN_NAME: &'static str = "MOUNTPOINT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Mountpoint {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Mountpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Mountpoint>
where
    D: crate::datasets::Dataset,
    Mountpoint: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "/",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Mountpoint {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Mountpoint where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Mountpoint
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Mountpoint
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sharenfs;
impl NativeProperty for Sharenfs {
    const NAME: &'static str = "sharenfs";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("sharenfs");
    const COLUMN_NAME: &'static str = "SHARENFS";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Sharenfs {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Sharenfs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Sharenfs>
where
    D: crate::datasets::Dataset,
    Sharenfs: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "off",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Sharenfs {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Sharenfs where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Sharenfs
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Sharenfs
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Checksum;
impl NativeProperty for Checksum {
    const NAME: &'static str = "checksum";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("checksum");
    const COLUMN_NAME: &'static str = "CHECKSUM";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Checksum {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Checksum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Checksum>
where
    D: crate::datasets::Dataset,
    Checksum: DatasetProperty<'a, D, Value = values::Checksum>,
{
    fn default() -> Self {
        Self {
            value: values::Checksum::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Checksum {
    type Value = values::Checksum;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Checksum {
    type Value = values::Checksum;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Checksum where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Checksum
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Checksum;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Checksum
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Checksum;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Compression;
impl NativeProperty for Compression {
    const NAME: &'static str = "compression";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("compression");
    const COLUMN_NAME: &'static str = "COMPRESS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Compression {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Compression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Compression>
where
    D: crate::datasets::Dataset,
    Compression: DatasetProperty<'a, D, Value = values::Compression>,
{
    fn default() -> Self {
        Self {
            value: values::Compression::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Compression {
    type Value = values::Compression;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Compression {
    type Value = values::Compression;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Compression where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Compression
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Compression;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Compression
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Compression;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Atime;
impl NativeProperty for Atime {
    const NAME: &'static str = "atime";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("atime");
    const COLUMN_NAME: &'static str = "ATIME";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Atime {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Atime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Atime>
where
    D: crate::datasets::Dataset,
    Atime: DatasetProperty<'a, D, Value = values::Atime>,
{
    fn default() -> Self {
        Self {
            value: values::Atime::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Atime {
    type Value = values::Atime;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Atime where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Atime
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Atime;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Atime
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Atime;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Devices;
impl NativeProperty for Devices {
    const NAME: &'static str = "devices";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("devices");
    const COLUMN_NAME: &'static str = "DEVICES";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Devices {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Devices {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Devices>
where
    D: crate::datasets::Dataset,
    Devices: DatasetProperty<'a, D, Value = values::Devices>,
{
    fn default() -> Self {
        Self {
            value: values::Devices::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Devices {
    type Value = values::Devices;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Devices {
    type Value = values::Devices;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Devices where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Devices
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Devices;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Devices
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Devices;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Exec;
impl NativeProperty for Exec {
    const NAME: &'static str = "exec";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("exec");
    const COLUMN_NAME: &'static str = "EXEC";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Exec {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Exec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Exec>
where
    D: crate::datasets::Dataset,
    Exec: DatasetProperty<'a, D, Value = values::Exec>,
{
    fn default() -> Self {
        Self {
            value: values::Exec::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Exec {
    type Value = values::Exec;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Exec {
    type Value = values::Exec;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Exec where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Exec
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Exec;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Exec
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Exec;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Setuid;
impl NativeProperty for Setuid {
    const NAME: &'static str = "setuid";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("setuid");
    const COLUMN_NAME: &'static str = "SETUID";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Setuid {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Setuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Setuid>
where
    D: crate::datasets::Dataset,
    Setuid: DatasetProperty<'a, D, Value = values::Setuid>,
{
    fn default() -> Self {
        Self {
            value: values::Setuid::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Setuid {
    type Value = values::Setuid;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Setuid {
    type Value = values::Setuid;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Setuid where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Setuid
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Setuid;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Setuid
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Setuid;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Readonly;
impl NativeProperty for Readonly {
    const NAME: &'static str = "readonly";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("readonly");
    const COLUMN_NAME: &'static str = "RDONLY";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Readonly {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Readonly {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Readonly>
where
    D: crate::datasets::Dataset,
    Readonly: DatasetProperty<'a, D, Value = values::Readonly>,
{
    fn default() -> Self {
        Self {
            value: values::Readonly::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Readonly {
    type Value = values::Readonly;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Readonly {
    type Value = values::Readonly;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Readonly where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Readonly
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Readonly;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Readonly
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Readonly;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Zoned;
impl NativeProperty for Zoned {
    const NAME: &'static str = "zoned";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("zoned");
    const COLUMN_NAME: &'static str = "ZONED";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Zoned {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Zoned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Zoned>
where
    D: crate::datasets::Dataset,
    Zoned: DatasetProperty<'a, D, Value = values::Zoned>,
{
    fn default() -> Self {
        Self {
            value: values::Zoned::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Zoned {
    type Value = values::Zoned;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Zoned where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Zoned
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Zoned;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Zoned
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Zoned;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Snapdir;
impl NativeProperty for Snapdir {
    const NAME: &'static str = "snapdir";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("snapdir");
    const COLUMN_NAME: &'static str = "SNAPDIR";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Snapdir {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Snapdir {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Snapdir>
where
    D: crate::datasets::Dataset,
    Snapdir: DatasetProperty<'a, D, Value = values::Snapdir>,
{
    fn default() -> Self {
        Self {
            value: values::Snapdir::Hidden,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Snapdir {
    type Value = values::Snapdir;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Snapdir where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Snapdir
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Snapdir;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Snapdir
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Snapdir;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Private;
impl NativeProperty for Private {
    const NAME: &'static str = "priv_prop";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("priv_prop");
    const COLUMN_NAME: &'static str = "PRIV_PROP";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Private {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Private {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Private>
where
    D: crate::datasets::Dataset,
    Private: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Private {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Aclinherit;
impl NativeProperty for Aclinherit {
    const NAME: &'static str = "aclinherit";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("aclinherit");
    const COLUMN_NAME: &'static str = "ACLINHERIT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Aclinherit {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Aclinherit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Aclinherit>
where
    D: crate::datasets::Dataset,
    Aclinherit: DatasetProperty<'a, D, Value = values::Aclinherit>,
{
    fn default() -> Self {
        Self {
            value: values::Aclinherit::Restricted,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Aclinherit {
    type Value = values::Aclinherit;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Aclinherit where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Aclinherit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Aclinherit;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Aclinherit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Aclinherit;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Createtxg;
impl NativeProperty for Createtxg {
    const NAME: &'static str = "createtxg";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("createtxg");
    const COLUMN_NAME: &'static str = "CREATETXG";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Createtxg {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Createtxg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Createtxg>
where
    D: crate::datasets::Dataset,
    Createtxg: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Createtxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Createtxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Createtxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Bookmark<'a>> for Createtxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Bookmark,
    ) -> Result<Option<Value<'a, crate::datasets::Bookmark<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Name;
impl NativeProperty for Name {
    const NAME: &'static str = "name";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("name");
    const COLUMN_NAME: &'static str = "NAME";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Name {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Name {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Name {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Name {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Name {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Bookmark<'a>> for Name {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Bookmark,
    ) -> Result<Option<Value<'a, crate::datasets::Bookmark<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Canmount;
impl NativeProperty for Canmount {
    const NAME: &'static str = "canmount";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("canmount");
    const COLUMN_NAME: &'static str = "CANMOUNT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Canmount {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Canmount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Canmount>
where
    D: crate::datasets::Dataset,
    Canmount: DatasetProperty<'a, D, Value = values::Canmount>,
{
    fn default() -> Self {
        Self {
            value: values::Canmount::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Canmount {
    type Value = values::Canmount;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Canmount
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Canmount;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Canmount
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Canmount;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Iscsioptions;
impl NativeProperty for Iscsioptions {
    const NAME: &'static str = "iscsioptions";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("iscsioptions");
    const COLUMN_NAME: &'static str = "ISCSIOPTIONS";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Iscsioptions {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Iscsioptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Iscsioptions {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Iscsioptions where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Iscsioptions
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Iscsioptions
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Xattr;
impl NativeProperty for Xattr {
    const NAME: &'static str = "xattr";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("xattr");
    const COLUMN_NAME: &'static str = "XATTR";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Xattr {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Xattr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Xattr>
where
    D: crate::datasets::Dataset,
    Xattr: DatasetProperty<'a, D, Value = values::Xattr>,
{
    fn default() -> Self {
        Self {
            value: values::Xattr::On,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Xattr {
    type Value = values::Xattr;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Xattr {
    type Value = values::Xattr;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Xattr where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Xattr
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Xattr;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Xattr
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Xattr;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Numclones;
impl NativeProperty for Numclones {
    const NAME: &'static str = "numclones";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("numclones");
    const COLUMN_NAME: &'static str = "NUMCLONES";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Numclones {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Numclones {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Numclones>
where
    D: crate::datasets::Dataset,
    Numclones: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Numclones {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Copies;
impl NativeProperty for Copies {
    const NAME: &'static str = "copies";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("copies");
    const COLUMN_NAME: &'static str = "COPIES";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Copies {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Copies {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Copies>
where
    D: crate::datasets::Dataset,
    Copies: DatasetProperty<'a, D, Value = values::Copies>,
{
    fn default() -> Self {
        Self {
            value: values::Copies::N1,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Copies {
    type Value = values::Copies;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Copies {
    type Value = values::Copies;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Copies where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Copies
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Copies;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Copies
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Copies;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Version;
impl NativeProperty for Version {
    const NAME: &'static str = "version";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("version");
    const COLUMN_NAME: &'static str = "VERSION";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Version {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Version {
    type Value = values::Version;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Version {
    type Value = values::Version;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Version
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Version;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Version
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Version;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Utf8only;
impl NativeProperty for Utf8only {
    const NAME: &'static str = "utf8only";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("utf8only");
    const COLUMN_NAME: &'static str = "UTF8ONLY";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Utf8only {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Utf8only {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Utf8only>
where
    D: crate::datasets::Dataset,
    Utf8only: DatasetProperty<'a, D, Value = values::Utf8only>,
{
    fn default() -> Self {
        Self {
            value: values::Utf8only::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Utf8only {
    type Value = values::Utf8only;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Utf8only {
    type Value = values::Utf8only;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Utf8only where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Utf8only
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Utf8only;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Normalize;
impl NativeProperty for Normalize {
    const NAME: &'static str = "normalization";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("normalization");
    const COLUMN_NAME: &'static str = "NORMALIZATION";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Normalize {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Normalize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Normalize>
where
    D: crate::datasets::Dataset,
    Normalize: DatasetProperty<'a, D, Value = values::Normalize>,
{
    fn default() -> Self {
        Self {
            value: values::Normalize::None,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Normalize {
    type Value = values::Normalize;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Normalize {
    type Value = values::Normalize;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Normalize where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Normalize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Normalize;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Case;
impl NativeProperty for Case {
    const NAME: &'static str = "casesensitivity";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("casesensitivity");
    const COLUMN_NAME: &'static str = "CASE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Case {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Case {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Case>
where
    D: crate::datasets::Dataset,
    Case: DatasetProperty<'a, D, Value = values::Case>,
{
    fn default() -> Self {
        Self {
            value: values::Case::Sensitive,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Case {
    type Value = values::Case;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Case {
    type Value = values::Case;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Case where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Case
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Case;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vscan;
impl NativeProperty for Vscan {
    const NAME: &'static str = "vscan";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("vscan");
    const COLUMN_NAME: &'static str = "VSCAN";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Vscan {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Vscan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Vscan>
where
    D: crate::datasets::Dataset,
    Vscan: DatasetProperty<'a, D, Value = values::Vscan>,
{
    fn default() -> Self {
        Self {
            value: values::Vscan::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Vscan {
    type Value = values::Vscan;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Vscan where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Vscan
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Vscan;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Vscan
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Vscan;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Nbmand;
impl NativeProperty for Nbmand {
    const NAME: &'static str = "nbmand";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("nbmand");
    const COLUMN_NAME: &'static str = "NBMAND";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Nbmand {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Nbmand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Nbmand>
where
    D: crate::datasets::Dataset,
    Nbmand: DatasetProperty<'a, D, Value = values::Nbmand>,
{
    fn default() -> Self {
        Self {
            value: values::Nbmand::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Nbmand {
    type Value = values::Nbmand;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Nbmand {
    type Value = values::Nbmand;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Nbmand where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Nbmand
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Nbmand;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Nbmand
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Nbmand;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sharesmb;
impl NativeProperty for Sharesmb {
    const NAME: &'static str = "sharesmb";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("sharesmb");
    const COLUMN_NAME: &'static str = "SHARESMB";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Sharesmb {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Sharesmb {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Sharesmb>
where
    D: crate::datasets::Dataset,
    Sharesmb: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "off",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Sharesmb {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Sharesmb where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Sharesmb
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Sharesmb
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Refquota;
impl NativeProperty for Refquota {
    const NAME: &'static str = "refquota";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("refquota");
    const COLUMN_NAME: &'static str = "REFQUOTA";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Refquota {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Refquota {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Refquota>
where
    D: crate::datasets::Dataset,
    Refquota: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Refquota {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Refquota
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Refquota
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Refreservation;
impl NativeProperty for Refreservation {
    const NAME: &'static str = "refreservation";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("refreservation");
    const COLUMN_NAME: &'static str = "REFRESERV";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Refreservation {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Refreservation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Refreservation>
where
    D: crate::datasets::Dataset,
    Refreservation: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Refreservation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Refreservation {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Refreservation
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Refreservation
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Guid;
impl NativeProperty for Guid {
    const NAME: &'static str = "guid";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("guid");
    const COLUMN_NAME: &'static str = "GUID";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Guid {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Guid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Guid>
where
    D: crate::datasets::Dataset,
    Guid: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Guid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Guid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Guid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Bookmark<'a>> for Guid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Bookmark,
    ) -> Result<Option<Value<'a, crate::datasets::Bookmark<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Primarycache;
impl NativeProperty for Primarycache {
    const NAME: &'static str = "primarycache";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("primarycache");
    const COLUMN_NAME: &'static str = "PRIMARYCACHE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Primarycache {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Primarycache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Primarycache>
where
    D: crate::datasets::Dataset,
    Primarycache: DatasetProperty<'a, D, Value = values::Primarycache>,
{
    fn default() -> Self {
        Self {
            value: values::Primarycache::All,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Primarycache {
    type Value = values::Primarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Primarycache {
    type Value = values::Primarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Primarycache {
    type Value = values::Primarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Primarycache where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Primarycache
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Primarycache;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Primarycache
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Primarycache;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Secondarycache;
impl NativeProperty for Secondarycache {
    const NAME: &'static str = "secondarycache";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("secondarycache");
    const COLUMN_NAME: &'static str = "SECONDARYCACHE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Secondarycache {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Secondarycache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Secondarycache>
where
    D: crate::datasets::Dataset,
    Secondarycache: DatasetProperty<'a, D, Value = values::Secondarycache>,
{
    fn default() -> Self {
        Self {
            value: values::Secondarycache::All,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Secondarycache {
    type Value = values::Secondarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Secondarycache {
    type Value = values::Secondarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Secondarycache {
    type Value = values::Secondarycache;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Secondarycache where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Secondarycache
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Secondarycache;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Secondarycache
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Secondarycache;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Usedsnap;
impl NativeProperty for Usedsnap {
    const NAME: &'static str = "usedbysnapshots";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("usedbysnapshots");
    const COLUMN_NAME: &'static str = "USEDSNAP";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Usedsnap {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Usedsnap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Usedsnap>
where
    D: crate::datasets::Dataset,
    Usedsnap: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Usedsnap {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Usedsnap {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Usedds;
impl NativeProperty for Usedds {
    const NAME: &'static str = "usedbydataset";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("usedbydataset");
    const COLUMN_NAME: &'static str = "USEDDS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Usedds {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Usedds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Usedds>
where
    D: crate::datasets::Dataset,
    Usedds: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Usedds {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Usedds {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Usedchild;
impl NativeProperty for Usedchild {
    const NAME: &'static str = "usedbychildren";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("usedbychildren");
    const COLUMN_NAME: &'static str = "USEDCHILD";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Usedchild {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Usedchild {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Usedchild>
where
    D: crate::datasets::Dataset,
    Usedchild: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Usedchild {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Usedchild {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Usedrefreserv;
impl NativeProperty for Usedrefreserv {
    const NAME: &'static str = "usedbyrefreservation";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("usedbyrefreservation");
    const COLUMN_NAME: &'static str = "USEDREFRESERV";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Usedrefreserv {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Usedrefreserv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Usedrefreserv>
where
    D: crate::datasets::Dataset,
    Usedrefreserv: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Usedrefreserv {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Usedrefreserv {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Useraccounting;
impl NativeProperty for Useraccounting {
    const NAME: &'static str = "useraccounting";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("useraccounting");
    const COLUMN_NAME: &'static str = "USERACCOUNTING";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Useraccounting {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Useraccounting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Useraccounting>
where
    D: crate::datasets::Dataset,
    Useraccounting: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Useraccounting {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Useraccounting {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Useraccounting {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StmfShareinfo;
impl NativeProperty for StmfShareinfo {
    const NAME: &'static str = "stmf_sbd_lu";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("stmf_sbd_lu");
    const COLUMN_NAME: &'static str = "STMF_SBD_LU";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for StmfShareinfo {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for StmfShareinfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for StmfShareinfo {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for StmfShareinfo where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for StmfShareinfo
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for StmfShareinfo
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DeferDestroy;
impl NativeProperty for DeferDestroy {
    const NAME: &'static str = "defer_destroy";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("defer_destroy");
    const COLUMN_NAME: &'static str = "DEFER_DESTROY";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for DeferDestroy {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for DeferDestroy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, DeferDestroy>
where
    D: crate::datasets::Dataset,
    DeferDestroy: DatasetProperty<'a, D, Value = values::DeferDestroy>,
{
    fn default() -> Self {
        Self {
            value: values::DeferDestroy::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for DeferDestroy {
    type Value = values::DeferDestroy;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Userrefs;
impl NativeProperty for Userrefs {
    const NAME: &'static str = "userrefs";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("userrefs");
    const COLUMN_NAME: &'static str = "USERREFS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Userrefs {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Userrefs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Userrefs>
where
    D: crate::datasets::Dataset,
    Userrefs: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Userrefs {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Logbias;
impl NativeProperty for Logbias {
    const NAME: &'static str = "logbias";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("logbias");
    const COLUMN_NAME: &'static str = "LOGBIAS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Logbias {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Logbias {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Logbias>
where
    D: crate::datasets::Dataset,
    Logbias: DatasetProperty<'a, D, Value = values::Logbias>,
{
    fn default() -> Self {
        Self {
            value: values::Logbias::Latency,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Logbias {
    type Value = values::Logbias;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Logbias {
    type Value = values::Logbias;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Logbias where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Logbias
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Logbias;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Logbias
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Logbias;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Unique;
impl NativeProperty for Unique {
    const NAME: &'static str = "unique";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("unique");
    const COLUMN_NAME: &'static str = "UNIQUE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Unique {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Unique {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Unique>
where
    D: crate::datasets::Dataset,
    Unique: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Unique {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Unique {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Unique {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Objsetid;
impl NativeProperty for Objsetid {
    const NAME: &'static str = "objsetid";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("objsetid");
    const COLUMN_NAME: &'static str = "OBJSETID";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Objsetid {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Objsetid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Objsetid>
where
    D: crate::datasets::Dataset,
    Objsetid: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Objsetid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Objsetid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Objsetid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dedup;
impl NativeProperty for Dedup {
    const NAME: &'static str = "dedup";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("dedup");
    const COLUMN_NAME: &'static str = "DEDUP";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Dedup {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Dedup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Dedup>
where
    D: crate::datasets::Dataset,
    Dedup: DatasetProperty<'a, D, Value = values::Dedup>,
{
    fn default() -> Self {
        Self {
            value: values::Dedup::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Dedup {
    type Value = values::Dedup;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Dedup {
    type Value = values::Dedup;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Dedup where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Dedup
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Dedup;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Dedup
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Dedup;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mlslabel;
impl NativeProperty for Mlslabel {
    const NAME: &'static str = "mlslabel";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("mlslabel");
    const COLUMN_NAME: &'static str = "MLSLABEL";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Mlslabel {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Mlslabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Mlslabel>
where
    D: crate::datasets::Dataset,
    Mlslabel: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Mlslabel {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Mlslabel {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Mlslabel {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Mlslabel where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Mlslabel
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Mlslabel
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sync;
impl NativeProperty for Sync {
    const NAME: &'static str = "sync";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("sync");
    const COLUMN_NAME: &'static str = "SYNC";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Sync {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Sync {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Sync>
where
    D: crate::datasets::Dataset,
    Sync: DatasetProperty<'a, D, Value = values::Sync>,
{
    fn default() -> Self {
        Self {
            value: values::Sync::Standard,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Sync {
    type Value = values::Sync;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Sync {
    type Value = values::Sync;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Sync where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Sync
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Sync;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Sync
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Sync;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dnodesize;
impl NativeProperty for Dnodesize {
    const NAME: &'static str = "dnodesize";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("dnodesize");
    const COLUMN_NAME: &'static str = "DNSIZE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Dnodesize {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Dnodesize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Dnodesize>
where
    D: crate::datasets::Dataset,
    Dnodesize: DatasetProperty<'a, D, Value = values::Dnodesize>,
{
    fn default() -> Self {
        Self {
            value: values::Dnodesize::Legacy,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Dnodesize {
    type Value = values::Dnodesize;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Dnodesize where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Dnodesize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Dnodesize;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Dnodesize
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Dnodesize;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Refratio;
impl NativeProperty for Refratio {
    const NAME: &'static str = "refcompressratio";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("refcompressratio");
    const COLUMN_NAME: &'static str = "REFRATIO";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Refratio {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Refratio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Refratio>
where
    D: crate::datasets::Dataset,
    Refratio: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Refratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Refratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Refratio {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Written;
impl NativeProperty for Written {
    const NAME: &'static str = "written";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("written");
    const COLUMN_NAME: &'static str = "WRITTEN";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Written {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Written {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Written>
where
    D: crate::datasets::Dataset,
    Written: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Written {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Written {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Written {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Clones;
impl NativeProperty for Clones {
    const NAME: &'static str = "clones";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("clones");
    const COLUMN_NAME: &'static str = "CLONES";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Clones {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Clones {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Clones {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Logicalused;
impl NativeProperty for Logicalused {
    const NAME: &'static str = "logicalused";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("logicalused");
    const COLUMN_NAME: &'static str = "LUSED";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Logicalused {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Logicalused {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Logicalused>
where
    D: crate::datasets::Dataset,
    Logicalused: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Logicalused {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Logicalused {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Logicalreferenced;
impl NativeProperty for Logicalreferenced {
    const NAME: &'static str = "logicalreferenced";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("logicalreferenced");
    const COLUMN_NAME: &'static str = "LREFER";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Logicalreferenced {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Logicalreferenced {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Logicalreferenced>
where
    D: crate::datasets::Dataset,
    Logicalreferenced: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Logicalreferenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Logicalreferenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Logicalreferenced {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Inconsistent;
impl NativeProperty for Inconsistent {
    const NAME: &'static str = "inconsistent";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("inconsistent");
    const COLUMN_NAME: &'static str = "INCONSISTENT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Inconsistent {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Inconsistent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Inconsistent>
where
    D: crate::datasets::Dataset,
    Inconsistent: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Inconsistent {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Inconsistent {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Inconsistent {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Volmode;
impl NativeProperty for Volmode {
    const NAME: &'static str = "volmode";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("volmode");
    const COLUMN_NAME: &'static str = "VOLMODE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Volmode {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Volmode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Volmode>
where
    D: crate::datasets::Dataset,
    Volmode: DatasetProperty<'a, D, Value = values::Volmode>,
{
    fn default() -> Self {
        Self {
            value: values::Volmode::Default,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Volmode {
    type Value = values::Volmode;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Volmode {
    type Value = values::Volmode;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Volmode where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Volmode
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Volmode;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Volmode
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Volmode;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FilesystemLimit;
impl NativeProperty for FilesystemLimit {
    const NAME: &'static str = "filesystem_limit";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("filesystem_limit");
    const COLUMN_NAME: &'static str = "FSLIMIT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for FilesystemLimit {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for FilesystemLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, FilesystemLimit>
where
    D: crate::datasets::Dataset,
    FilesystemLimit: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 18446744073709551615u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for FilesystemLimit {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for FilesystemLimit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for FilesystemLimit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SnapshotLimit;
impl NativeProperty for SnapshotLimit {
    const NAME: &'static str = "snapshot_limit";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("snapshot_limit");
    const COLUMN_NAME: &'static str = "SSLIMIT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SnapshotLimit {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SnapshotLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SnapshotLimit>
where
    D: crate::datasets::Dataset,
    SnapshotLimit: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 18446744073709551615u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SnapshotLimit {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SnapshotLimit {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SnapshotLimit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SnapshotLimit
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FilesystemCount;
impl NativeProperty for FilesystemCount {
    const NAME: &'static str = "filesystem_count";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("filesystem_count");
    const COLUMN_NAME: &'static str = "FSCOUNT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for FilesystemCount {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for FilesystemCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, FilesystemCount>
where
    D: crate::datasets::Dataset,
    FilesystemCount: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 18446744073709551615u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for FilesystemCount {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SnapshotCount;
impl NativeProperty for SnapshotCount {
    const NAME: &'static str = "snapshot_count";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("snapshot_count");
    const COLUMN_NAME: &'static str = "SSCOUNT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for SnapshotCount {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SnapshotCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SnapshotCount>
where
    D: crate::datasets::Dataset,
    SnapshotCount: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 18446744073709551615u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SnapshotCount {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SnapshotCount {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Snapdev;
impl NativeProperty for Snapdev {
    const NAME: &'static str = "snapdev";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("snapdev");
    const COLUMN_NAME: &'static str = "SNAPDEV";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Snapdev {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Snapdev {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Snapdev>
where
    D: crate::datasets::Dataset,
    Snapdev: DatasetProperty<'a, D, Value = values::Snapdev>,
{
    fn default() -> Self {
        Self {
            value: values::Snapdev::Hidden,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Snapdev {
    type Value = values::Snapdev;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Snapdev {
    type Value = values::Snapdev;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Snapdev where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Snapdev
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Snapdev;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Snapdev
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Snapdev;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Acltype;
impl NativeProperty for Acltype {
    const NAME: &'static str = "acltype";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("acltype");
    const COLUMN_NAME: &'static str = "ACLTYPE";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Acltype {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Acltype {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Acltype>
where
    D: crate::datasets::Dataset,
    Acltype: DatasetProperty<'a, D, Value = values::Acltype>,
{
    fn default() -> Self {
        Self {
            value: values::Acltype::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Acltype {
    type Value = values::Acltype;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Acltype {
    type Value = values::Acltype;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Acltype where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Acltype
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Acltype;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Acltype
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Acltype;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SelinuxContext;
impl NativeProperty for SelinuxContext {
    const NAME: &'static str = "context";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("context");
    const COLUMN_NAME: &'static str = "CONTEXT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SelinuxContext {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SelinuxContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SelinuxContext>
where
    D: crate::datasets::Dataset,
    SelinuxContext: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SelinuxContext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for SelinuxContext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SelinuxContext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SelinuxContext
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SelinuxContext
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SelinuxFscontext;
impl NativeProperty for SelinuxFscontext {
    const NAME: &'static str = "fscontext";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("fscontext");
    const COLUMN_NAME: &'static str = "FSCONTEXT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SelinuxFscontext {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SelinuxFscontext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SelinuxFscontext>
where
    D: crate::datasets::Dataset,
    SelinuxFscontext: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SelinuxFscontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for SelinuxFscontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SelinuxFscontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SelinuxFscontext
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SelinuxFscontext
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SelinuxDefcontext;
impl NativeProperty for SelinuxDefcontext {
    const NAME: &'static str = "defcontext";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("defcontext");
    const COLUMN_NAME: &'static str = "DEFCONTEXT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SelinuxDefcontext {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SelinuxDefcontext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SelinuxDefcontext>
where
    D: crate::datasets::Dataset,
    SelinuxDefcontext: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SelinuxDefcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for SelinuxDefcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SelinuxDefcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SelinuxDefcontext
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SelinuxDefcontext
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SelinuxRootcontext;
impl NativeProperty for SelinuxRootcontext {
    const NAME: &'static str = "rootcontext";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("rootcontext");
    const COLUMN_NAME: &'static str = "ROOTCONTEXT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SelinuxRootcontext {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SelinuxRootcontext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SelinuxRootcontext>
where
    D: crate::datasets::Dataset,
    SelinuxRootcontext: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SelinuxRootcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for SelinuxRootcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for SelinuxRootcontext {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SelinuxRootcontext
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SelinuxRootcontext
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Relatime;
impl NativeProperty for Relatime {
    const NAME: &'static str = "relatime";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("relatime");
    const COLUMN_NAME: &'static str = "RELATIME";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Relatime {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Relatime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Relatime>
where
    D: crate::datasets::Dataset,
    Relatime: DatasetProperty<'a, D, Value = values::Relatime>,
{
    fn default() -> Self {
        Self {
            value: values::Relatime::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Relatime {
    type Value = values::Relatime;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Relatime where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Relatime
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Relatime;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Relatime
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Relatime;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RedundantMetadata;
impl NativeProperty for RedundantMetadata {
    const NAME: &'static str = "redundant_metadata";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("redundant_metadata");
    const COLUMN_NAME: &'static str = "REDUND_MD";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for RedundantMetadata {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for RedundantMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, RedundantMetadata>
where
    D: crate::datasets::Dataset,
    RedundantMetadata: DatasetProperty<'a, D, Value = values::RedundantMetadata>,
{
    fn default() -> Self {
        Self {
            value: values::RedundantMetadata::All,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for RedundantMetadata {
    type Value = values::RedundantMetadata;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for RedundantMetadata {
    type Value = values::RedundantMetadata;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for RedundantMetadata where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for RedundantMetadata
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::RedundantMetadata;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for RedundantMetadata
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::RedundantMetadata;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Overlay;
impl NativeProperty for Overlay {
    const NAME: &'static str = "overlay";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("overlay");
    const COLUMN_NAME: &'static str = "OVERLAY";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Overlay {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Overlay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Overlay>
where
    D: crate::datasets::Dataset,
    Overlay: DatasetProperty<'a, D, Value = values::Overlay>,
{
    fn default() -> Self {
        Self {
            value: values::Overlay::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Overlay {
    type Value = values::Overlay;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Overlay where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Overlay
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Overlay;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Overlay
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Overlay;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PrevSnap;
impl NativeProperty for PrevSnap {
    const NAME: &'static str = "prevsnap";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("prevsnap");
    const COLUMN_NAME: &'static str = "PREVSNAP";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for PrevSnap {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for PrevSnap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for PrevSnap {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for PrevSnap {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ReceiveResumeToken;
impl NativeProperty for ReceiveResumeToken {
    const NAME: &'static str = "receive_resume_token";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("receive_resume_token");
    const COLUMN_NAME: &'static str = "RESUMETOK";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for ReceiveResumeToken {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for ReceiveResumeToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for ReceiveResumeToken {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for ReceiveResumeToken {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Encryption;
impl NativeProperty for Encryption {
    const NAME: &'static str = "encryption";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("encryption");
    const COLUMN_NAME: &'static str = "ENCRYPTION";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Encryption {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Encryption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Encryption>
where
    D: crate::datasets::Dataset,
    Encryption: DatasetProperty<'a, D, Value = values::Encryption>,
{
    fn default() -> Self {
        Self {
            value: values::Encryption::Off,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Encryption {
    type Value = values::Encryption;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Encryption {
    type Value = values::Encryption;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Encryption {
    type Value = values::Encryption;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for Encryption where Self: DatasetProperty<'a, D> {}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Encryption
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Encryption;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Keylocation;
impl NativeProperty for Keylocation {
    const NAME: &'static str = "keylocation";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("keylocation");
    const COLUMN_NAME: &'static str = "KEYLOCATION";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for Keylocation {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Keylocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Keylocation>
where
    D: crate::datasets::Dataset,
    Keylocation: DatasetProperty<'a, D, Value = &'a str>,
{
    fn default() -> Self {
        Self {
            value: "none",
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Keylocation {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Keylocation {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for Keylocation
where
    Self: DatasetProperty<'b, D>,
{
    type Value = &'a str;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Keylocation
where
    Self: DatasetProperty<'b, D>,
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Keyformat;
impl NativeProperty for Keyformat {
    const NAME: &'static str = "keyformat";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("keyformat");
    const COLUMN_NAME: &'static str = "KEYFORMAT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Keyformat {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Keyformat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Keyformat>
where
    D: crate::datasets::Dataset,
    Keyformat: DatasetProperty<'a, D, Value = values::Keyformat>,
{
    fn default() -> Self {
        Self {
            value: values::Keyformat::None,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Keyformat {
    type Value = values::Keyformat;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Keyformat {
    type Value = values::Keyformat;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Keyformat
where
    Self: DatasetProperty<'b, D>,
{
    type Value = values::Keyformat;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pbkdf2Salt;
impl NativeProperty for Pbkdf2Salt {
    const NAME: &'static str = "pbkdf2salt";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("pbkdf2salt");
    const COLUMN_NAME: &'static str = "PBKDF2SALT";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Pbkdf2Salt {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Pbkdf2Salt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Pbkdf2Salt>
where
    D: crate::datasets::Dataset,
    Pbkdf2Salt: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Pbkdf2Salt {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Pbkdf2Salt {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Pbkdf2Salt
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pbkdf2Iters;
impl NativeProperty for Pbkdf2Iters {
    const NAME: &'static str = "pbkdf2iters";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("pbkdf2iters");
    const COLUMN_NAME: &'static str = "PBKDF2ITERS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = true;
}
impl Property for Pbkdf2Iters {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Pbkdf2Iters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Pbkdf2Iters>
where
    D: crate::datasets::Dataset,
    Pbkdf2Iters: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Pbkdf2Iters {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Pbkdf2Iters {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for Pbkdf2Iters
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EncryptionRoot;
impl NativeProperty for EncryptionRoot {
    const NAME: &'static str = "encryptionroot";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("encryptionroot");
    const COLUMN_NAME: &'static str = "ENCROOT";
    const RIGHT_ALIGN: bool = false;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for EncryptionRoot {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for EncryptionRoot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for EncryptionRoot {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for EncryptionRoot {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for EncryptionRoot {
    type Value = &'a str;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_string(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct KeyGuid;
impl NativeProperty for KeyGuid {
    const NAME: &'static str = "keyguid";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("keyguid");
    const COLUMN_NAME: &'static str = "KEYGUID";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for KeyGuid {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for KeyGuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, KeyGuid>
where
    D: crate::datasets::Dataset,
    KeyGuid: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for KeyGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for KeyGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for KeyGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Keystatus;
impl NativeProperty for Keystatus {
    const NAME: &'static str = "keystatus";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("keystatus");
    const COLUMN_NAME: &'static str = "KEYSTATUS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Keystatus {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Keystatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Keystatus>
where
    D: crate::datasets::Dataset,
    Keystatus: DatasetProperty<'a, D, Value = values::Keystatus>,
{
    fn default() -> Self {
        Self {
            value: values::Keystatus::None,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Keystatus {
    type Value = values::Keystatus;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Keystatus {
    type Value = values::Keystatus;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Keystatus {
    type Value = values::Keystatus;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_index(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Remaptxg;
impl NativeProperty for Remaptxg {
    const NAME: &'static str = "remaptxg";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("remaptxg");
    const COLUMN_NAME: &'static str = "REMAPTXG";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for Remaptxg {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for Remaptxg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, Remaptxg>
where
    D: crate::datasets::Dataset,
    Remaptxg: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for Remaptxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for Remaptxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for Remaptxg {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpecialSmallBlocks;
impl NativeProperty for SpecialSmallBlocks {
    const NAME: &'static str = "special_small_blocks";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("special_small_blocks");
    const COLUMN_NAME: &'static str = "SPECIAL_SMALL_BLOCKS";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = false;
    const READ_ONLY: bool = false;
    const SET_ONCE: bool = false;
}
impl Property for SpecialSmallBlocks {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for SpecialSmallBlocks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, SpecialSmallBlocks>
where
    D: crate::datasets::Dataset,
    SpecialSmallBlocks: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for SpecialSmallBlocks {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a, D: Dataset> InheritableProperty<D> for SpecialSmallBlocks where Self: DatasetProperty<'a, D>
{}
impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for SpecialSmallBlocks
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(&self, value: Self::Value, list: &mut SetPropertyValues<D>) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for SpecialSmallBlocks
where
    Self: DatasetProperty<'b, D>,
{
    type Value = u64;
    fn add_value_to_list(
        &self,
        value: Self::Value,
        list: &mut SetOnCreatePropertyValues<D>,
    ) -> Result<()> {
        unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct IvsetGuid;
impl NativeProperty for IvsetGuid {
    const NAME: &'static str = "ivsetguid";
    const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!("ivsetguid");
    const COLUMN_NAME: &'static str = "IVSETGUID";
    const RIGHT_ALIGN: bool = true;
    const HIDDEN: bool = true;
    const READ_ONLY: bool = true;
    const SET_ONCE: bool = false;
}
impl Property for IvsetGuid {
    fn name(&self) -> &str {
        <Self as NativeProperty>::NAME
    }
    fn name_cstr(&self) -> &::std::ffi::CStr {
        <Self as NativeProperty>::NAME_CSTR
    }
}
impl<'a> ::std::fmt::Display for IvsetGuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(Self::NAME, f)
    }
}
impl<'a, D> Default for Value<'a, D, IvsetGuid>
where
    D: crate::datasets::Dataset,
    IvsetGuid: DatasetProperty<'a, D, Value = u64>,
{
    fn default() -> Self {
        Self {
            value: 0u64,
            source: Source::Default,
        }
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Filesystem<'a>> for IvsetGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Filesystem,
    ) -> Result<Option<Value<'a, crate::datasets::Filesystem<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Snapshot<'a>> for IvsetGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Snapshot,
    ) -> Result<Option<Value<'a, crate::datasets::Snapshot<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Volume<'a>> for IvsetGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Volume,
    ) -> Result<Option<Value<'a, crate::datasets::Volume<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
impl<'a> DatasetProperty<'a, crate::datasets::Bookmark<'a>> for IvsetGuid {
    type Value = u64;
    fn lookup(
        &self,
        dataset: &'a crate::datasets::Bookmark,
    ) -> Result<Option<Value<'a, crate::datasets::Bookmark<'a>, Self>>> {
        lookup_number(self, dataset)
    }
}
pub mod values {
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Mounted {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Mounted {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Checksum {
        #[strum(serialize = "on")]
        On = 1u64,
        #[strum(serialize = "off")]
        Off = 2u64,
        #[strum(serialize = "fletcher2")]
        Fletcher2 = 6u64,
        #[strum(serialize = "fletcher4")]
        Fletcher4 = 7u64,
        #[strum(serialize = "sha256")]
        Sha256 = 8u64,
        #[strum(serialize = "noparity")]
        Noparity = 10u64,
        #[strum(serialize = "sha512")]
        Sha512 = 11u64,
        #[strum(serialize = "skein")]
        Skein = 12u64,
        #[strum(serialize = "edonr")]
        Edonr = 13u64,
    }
    impl crate::nvpair::NvPairAddValue for Checksum {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Compression {
        #[strum(serialize = "on")]
        On = 1u64,
        #[strum(serialize = "off")]
        Off = 2u64,
        #[strum(serialize = "lzjb")]
        Lzjb = 3u64,
        #[strum(serialize = "gzip", serialize = "gzip-6")]
        Gzip = 10u64,
        #[strum(serialize = "gzip-1")]
        Gzip1 = 5u64,
        #[strum(serialize = "gzip-2")]
        Gzip2 = 6u64,
        #[strum(serialize = "gzip-3")]
        Gzip3 = 7u64,
        #[strum(serialize = "gzip-4")]
        Gzip4 = 8u64,
        #[strum(serialize = "gzip-5")]
        Gzip5 = 9u64,
        #[strum(serialize = "gzip-7")]
        Gzip7 = 11u64,
        #[strum(serialize = "gzip-8")]
        Gzip8 = 12u64,
        #[strum(serialize = "gzip-9")]
        Gzip9 = 13u64,
        #[strum(serialize = "zle")]
        Zle = 14u64,
        #[strum(serialize = "lz4")]
        Lz4 = 15u64,
    }
    impl crate::nvpair::NvPairAddValue for Compression {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Atime {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Atime {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Devices {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Devices {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Exec {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Exec {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Setuid {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Setuid {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Readonly {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Readonly {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Zoned {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Zoned {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Snapdir {
        #[strum(serialize = "hidden")]
        Hidden = 0u64,
        #[strum(serialize = "visible")]
        Visible = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Snapdir {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Aclinherit {
        #[strum(serialize = "discard")]
        Discard = 0u64,
        #[strum(serialize = "noallow")]
        Noallow = 1u64,
        #[strum(serialize = "restricted", serialize = "secure")]
        Restricted = 4u64,
        #[strum(serialize = "passthrough")]
        Passthrough = 3u64,
        #[strum(serialize = "passthrough-x")]
        PassthroughX = 5u64,
    }
    impl crate::nvpair::NvPairAddValue for Aclinherit {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Canmount {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
        #[strum(serialize = "noauto")]
        Noauto = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Canmount {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Xattr {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on", serialize = "dir")]
        On = 1u64,
        #[strum(serialize = "sa")]
        Sa = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Xattr {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Copies {
        #[strum(serialize = "1")]
        N1 = 1u64,
        #[strum(serialize = "2")]
        N2 = 2u64,
        #[strum(serialize = "3")]
        N3 = 3u64,
    }
    impl crate::nvpair::NvPairAddValue for Copies {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Version {
        #[strum(serialize = "1")]
        N1 = 1u64,
        #[strum(serialize = "2")]
        N2 = 2u64,
        #[strum(serialize = "3")]
        N3 = 3u64,
        #[strum(serialize = "4")]
        N4 = 4u64,
        #[strum(serialize = "5", serialize = "current")]
        N5 = 5u64,
    }
    impl crate::nvpair::NvPairAddValue for Version {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Utf8only {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Utf8only {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Normalize {
        #[strum(serialize = "none")]
        None = 0u64,
        #[strum(serialize = "formD")]
        Formd = 16u64,
        #[strum(serialize = "formKC")]
        Formkc = 96u64,
        #[strum(serialize = "formC")]
        Formc = 80u64,
        #[strum(serialize = "formKD")]
        Formkd = 32u64,
    }
    impl crate::nvpair::NvPairAddValue for Normalize {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Case {
        #[strum(serialize = "sensitive")]
        Sensitive = 0u64,
        #[strum(serialize = "insensitive")]
        Insensitive = 1u64,
        #[strum(serialize = "mixed")]
        Mixed = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Case {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Vscan {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Vscan {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Nbmand {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Nbmand {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Primarycache {
        #[strum(serialize = "none")]
        None = 0u64,
        #[strum(serialize = "metadata")]
        Metadata = 1u64,
        #[strum(serialize = "all")]
        All = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Primarycache {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Secondarycache {
        #[strum(serialize = "none")]
        None = 0u64,
        #[strum(serialize = "metadata")]
        Metadata = 1u64,
        #[strum(serialize = "all")]
        All = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Secondarycache {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum DeferDestroy {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for DeferDestroy {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Logbias {
        #[strum(serialize = "latency")]
        Latency = 0u64,
        #[strum(serialize = "throughput")]
        Throughput = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Logbias {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Dedup {
        #[strum(serialize = "on")]
        On = 1u64,
        #[strum(serialize = "off")]
        Off = 2u64,
        #[strum(serialize = "verify")]
        Verify = 257u64,
        #[strum(serialize = "sha256")]
        Sha256 = 8u64,
        #[strum(serialize = "sha256,verify")]
        Sha256Verify = 264u64,
        #[strum(serialize = "sha512")]
        Sha512 = 11u64,
        #[strum(serialize = "sha512,verify")]
        Sha512Verify = 267u64,
        #[strum(serialize = "skein")]
        Skein = 12u64,
        #[strum(serialize = "skein,verify")]
        SkeinVerify = 268u64,
        #[strum(serialize = "edonr,verify")]
        EdonrVerify = 269u64,
    }
    impl crate::nvpair::NvPairAddValue for Dedup {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Sync {
        #[strum(serialize = "standard")]
        Standard = 0u64,
        #[strum(serialize = "always")]
        Always = 1u64,
        #[strum(serialize = "disabled")]
        Disabled = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Sync {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Dnodesize {
        #[strum(serialize = "legacy")]
        Legacy = 0u64,
        #[strum(serialize = "auto")]
        Auto = 1u64,
        #[strum(serialize = "1k")]
        N1k = 1024u64,
        #[strum(serialize = "2k")]
        N2k = 2048u64,
        #[strum(serialize = "4k")]
        N4k = 4096u64,
        #[strum(serialize = "8k")]
        N8k = 8192u64,
        #[strum(serialize = "16k")]
        N16k = 16384u64,
    }
    impl crate::nvpair::NvPairAddValue for Dnodesize {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Volmode {
        #[strum(serialize = "default")]
        Default = 0u64,
        #[strum(serialize = "full", serialize = "geom")]
        Full = 1u64,
        #[strum(serialize = "dev")]
        Dev = 2u64,
        #[strum(serialize = "none")]
        None = 3u64,
    }
    impl crate::nvpair::NvPairAddValue for Volmode {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Snapdev {
        #[strum(serialize = "hidden")]
        Hidden = 0u64,
        #[strum(serialize = "visible")]
        Visible = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Snapdev {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Acltype {
        #[strum(serialize = "off", serialize = "disabled", serialize = "noacl")]
        Off = 0u64,
        #[strum(serialize = "posixacl")]
        Posixacl = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Acltype {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Relatime {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Relatime {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum RedundantMetadata {
        #[strum(serialize = "all")]
        All = 0u64,
        #[strum(serialize = "most")]
        Most = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for RedundantMetadata {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Overlay {
        #[strum(serialize = "off")]
        Off = 0u64,
        #[strum(serialize = "on")]
        On = 1u64,
    }
    impl crate::nvpair::NvPairAddValue for Overlay {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Encryption {
        #[strum(serialize = "on")]
        On = 1u64,
        #[strum(serialize = "off")]
        Off = 2u64,
        #[strum(serialize = "aes-128-ccm")]
        Aes128Ccm = 3u64,
        #[strum(serialize = "aes-192-ccm")]
        Aes192Ccm = 4u64,
        #[strum(serialize = "aes-256-ccm")]
        Aes256Ccm = 5u64,
        #[strum(serialize = "aes-128-gcm")]
        Aes128Gcm = 6u64,
        #[strum(serialize = "aes-192-gcm")]
        Aes192Gcm = 7u64,
        #[strum(serialize = "aes-256-gcm")]
        Aes256Gcm = 8u64,
    }
    impl crate::nvpair::NvPairAddValue for Encryption {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Keyformat {
        #[strum(serialize = "none")]
        None = 0u64,
        #[strum(serialize = "raw")]
        Raw = 1u64,
        #[strum(serialize = "hex")]
        Hex = 2u64,
        #[strum(serialize = "passphrase")]
        Passphrase = 3u64,
    }
    impl crate::nvpair::NvPairAddValue for Keyformat {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
    #[repr(u64)]
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Debug,
        num_derive :: FromPrimitive,
        num_derive :: ToPrimitive,
        strum_macros :: AsRefStr,
        strum_macros :: IntoStaticStr,
        strum_macros :: Display,
        strum_macros :: EnumString,
        strum_macros :: EnumIter,
    )]
    pub enum Keystatus {
        #[strum(serialize = "none")]
        None = 0u64,
        #[strum(serialize = "unavailable")]
        Unavailable = 1u64,
        #[strum(serialize = "available")]
        Available = 2u64,
    }
    impl crate::nvpair::NvPairAddValue for Keystatus {
        unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(
            self,
            nvlist: &mut T,
            name: &::std::ffi::CStr,
        ) -> crate::Result<()> {
            crate::nvpair::NvPairAddValue::add_to_nvlist(
                num_traits::ToPrimitive::to_u64(&self)
                    .expect("failed to convert index value to u64"),
                nvlist,
                name,
            )
        }
    }
}
