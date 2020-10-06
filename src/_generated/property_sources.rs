#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug, AsRefStr, IntoStaticStr, Display, EnumString)]
pub enum Source {
    #[strum(serialize = "none")]
    None = 1u32,
    #[strum(serialize = "default")]
    Default = 2u32,
    #[strum(serialize = "temporary")]
    Temporary = 4u32,
    #[strum(serialize = "local")]
    Local = 8u32,
    #[strum(serialize = "inherited")]
    Inherited = 16u32,
    #[strum(serialize = "received")]
    Received = 32u32,
}
