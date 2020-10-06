use std::os::raw::c_int;
pub use std::result::Result as StdResult;

pub use anyhow::Error as AnyError;

/// an error returned by the libzfs-rs
/// wraps many c conversion errors as well as errors from libzfs as LibZfsSysError
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    TryFromNumError(std::num::TryFromIntError),

    #[error(transparent)]
    IntoStringError(#[from] std::ffi::IntoStringError),

    #[error(transparent)]
    NixError(#[from] nix::Error),

    #[error("nvpair error: {0}")]
    NvPairError(i32),

    #[error("unsupported nvpair data type: {0:#?}")]
    NvPairUnsupportedType(nvpair_sys::data_type_t),

    #[error("null nvpair value")]
    NvPairNullValue,

    #[error("nvlist error: {0}")]
    NvListError(i32),

    #[error("unknown dataset type: {0}")]
    UnknownDatasetType(u64),

    #[error("wrong dataset type: {0}")]
    WrongDatasetType(&'static str),

    #[error("no properties for dataset")]
    NoProperties,

    #[error["error converting raw value to property"]]
    PropertyConversionError(AnyError),

    #[error["{0} is not a valid index value for this property"]]
    BadIndexValue(u64),

    #[error["not a boolean value"]]
    NotBoolValue,

    #[error["empty snapshot list"]]
    EmptySnapshotList,

    #[error["empty snapshot value"]]
    EmptySnapshotValue,

    #[error["empty bookmark list"]]
    EmptyBookmarkList,

    #[error["empty bookmark value"]]
    EmptyBookmarkValue,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait FromCInt: Copy {
    fn check(self) -> StdResult<(), Self>;
    fn as_nvpair_error(self) -> Result<()>;
    fn as_nvlist_error(self) -> Result<()>;
}

impl FromCInt for c_int {
    fn check(self) -> StdResult<(), Self> {
        match self {
            0 => Ok(()),
            n => Err(n),
        }
    }
    fn as_nvpair_error(self) -> Result<()> {
        self.check().map_err(Error::NvPairError)
    }

    fn as_nvlist_error(self) -> Result<()> {
        self.check().map_err(Error::NvListError)
    }
}
