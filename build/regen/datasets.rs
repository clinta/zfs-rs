use case_style::CaseStyle;
use num_traits::cast::ToPrimitive;
use quote::{format_ident, quote};
use strum::IntoEnumIterator;

pub fn gen_zfs_datasets() -> String {
    let (dataset_names, dataset_ints): (Vec<_>, Vec<_>) = zfs_sys::zfs_type_t::iter()
        .map(|dataset_int| {
            let dataset_name = zfs_type_to_name(dataset_int.into());
            let dataset_int: u32 = dataset_int.to_u32().unwrap();
            (dataset_name, dataset_int)
        })
        .unzip();

    let all_dataset_names = dataset_names.join("|");

    let dataset_idents: Vec<_> = dataset_names
        .iter()
        .map(|name| format_ident!("{}", CaseStyle::from_snakecase(name).to_pascalcase()))
        .collect();

    let separators: Vec<_> = dataset_names
        .iter()
        .map(|name| match name.as_ref() {
            "bookmark" => '#',
            "snapshot" => '@',
            _ => '/',
        })
        .collect();

    let output = quote! {
        #[derive(AsRefStr, IntoStaticStr, PartialEq, Debug)]
        pub enum Datasets<'a> {
            #(
                #[strum(to_string=#dataset_names)]
                #dataset_idents(#dataset_idents<'a>),
            )*
        }

        impl ds::Dataset for Datasets<'_>{
            fn handle(&self) -> &Handle { match self { #( Self::#dataset_idents(v) => v.handle(), )* } }
            fn raw_path(&self) -> &Path { match self { #( Self::#dataset_idents(v) => v.raw_path(), )* } }
            fn raw_properties(&self) -> Result<&NvList> { match self { #( Self::#dataset_idents(v) => v.raw_properties(), )* } }
            fn reset_raw_properties(&mut self) { match self { #( Self::#dataset_idents(v) => v.reset_raw_properties(), )* } }
        }

        impl<'a> Dataset for Datasets<'a> {
            const NAME: &'static str = #all_dataset_names;
            const SEPARATOR: char = '/';
            const DATASET_TYPE: u32 = std::u32::MAX;

            fn path(&self) -> &Path { match self { #( Self::#dataset_idents(v) => v.path(), )* } }
            fn separator(&self) -> char { match self { #( Self::#dataset_idents(v) => v.separator(), )* } }
            fn dataset_type(&self) -> u32 { match self { #( Self::#dataset_idents(v) => v.dataset_type(), )* } }
            fn dataset_type_name(&self) -> &'static str { match self { #( Self::#dataset_idents(v) => v.dataset_type_name(), )* } }
        }

        impl<'a> TryFrom<RawDataset<'a>> for Datasets<'a> {
            type Error = Error;
            fn try_from(mut rds: RawDataset<'a>) -> Result<Self> {
                match rds.dataset_type()? {
                    #(
                        #dataset_idents::DATASET_TYPE => Self::#dataset_idents(#dataset_idents{raw: rds}).apply(Ok),
                    )*
                        v => Err(Error::UnknownDatasetType(v.into())),
                }
            }
        }

        #(
            #[derive(Debug, PartialEq)]
            pub struct #dataset_idents<'a>{
                pub(crate) raw: RawDataset<'a>,
            }

            impl ds::Dataset for #dataset_idents<'_>{
                fn handle(&self) -> &Handle {&self.raw.handle}
                fn raw_path(&self) -> &Path {&self.raw.path}
                fn raw_properties(&self) -> Result<&NvList> {self.raw.raw_properties()}
                fn reset_raw_properties(&mut self) { self.raw.reset_raw_properties() }
            }

            impl<'a> Dataset for #dataset_idents<'a> {
                const NAME: &'static str = #dataset_names;
                const SEPARATOR: char = #separators;
                const DATASET_TYPE: u32 = #dataset_ints;
            }

            impl<'a> From<#dataset_idents<'a>> for Datasets<'a> {
                fn from(ds: #dataset_idents<'a>) -> Self {
                    Self::#dataset_idents(ds)
                }
            }

            impl<'a> TryFrom<Datasets<'a>> for #dataset_idents<'a> {
                type Error = Error;
                fn try_from(ads: Datasets<'a>) -> Result<Self> {
                    match ads {
                        Datasets::#dataset_idents(ds) => Ok(ds),
                        _ => Err(Error::WrongDatasetType(ads.into())),
                    }
                }
            }

            impl<'a> TryFrom<RawDataset<'a>> for #dataset_idents<'a> {
                type Error = Error;
                fn try_from(rds: RawDataset<'a>) -> Result<Self> {
                    Datasets::try_from(rds)?.try_into()
                }
            }
        )*
    };

    output.to_string()
}

pub(crate) fn zfs_type_to_name(t: &str) -> String {
    t.strip_prefix("ZFS_TYPE_").unwrap().to_lowercase()
}
