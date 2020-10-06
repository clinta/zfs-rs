use case_style::CaseStyle;
use num_traits::cast::ToPrimitive;
use quote::{format_ident, quote};
use strum::*;
use zfs_sys::*;

pub fn gen_property_sources() -> String {
    let (source_name, source_index): (Vec<_>, Vec<_>) = zprop_source_t::iter()
        .map(|s| {
            let name = <&'static str>::from(s)
                .strip_prefix("ZPROP_SRC_")
                .unwrap()
                .to_string()
                .to_lowercase();
            let index = s.to_u32().unwrap();
            (name, index)
        })
        .unzip();
    let source_ident: Vec<_> = source_name
        .iter()
        .map(|n| format_ident!("{}", CaseStyle::from_snakecase(n).to_pascalcase()))
        .collect();
    (quote! {
        #[repr(u32)]
        #[derive(Copy, Clone, PartialEq, Debug, AsRefStr, IntoStaticStr, Display, EnumString)]
        pub enum Source {
            #(
                #[strum(serialize=#source_name)]
                #source_ident = #source_index,
            )*
        }
    })
    .to_string()
}
