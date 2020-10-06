use case_style::CaseStyle;
use itertools::Itertools;
use num_traits::cast::ToPrimitive;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{ffi::CStr, os::raw::c_int};
use strum::*;
use zfs_sys::*;

type ZpropDescsTable = [zprop_desc_t; zfs_prop_t::ZFS_NUM_PROPS as usize];

pub(crate) fn gen_native_properties() -> String {
    unsafe { zfs_prop_init() };

    let prop_descs = unsafe { zfs_prop_get_table().cast::<ZpropDescsTable>().as_ref() }.unwrap();

    const NUM_PROPS: i32 = zfs_prop_t::ZFS_NUM_PROPS as i32;
    let (mut props_mod, mut values_mod) = (TokenStream::new(), TokenStream::new());

    let props: Vec<_> = zfs_prop_t::iter()
        .filter(|p| p.to_i32().unwrap() > 0 && p.to_i32().unwrap() < NUM_PROPS)
        .collect();

    for prop in props.iter() {
        let prop_desc = prop_descs[prop.to_i32().unwrap() as usize];
        gen_native_prop(*prop, prop_desc, &mut props_mod, &mut values_mod)
    }

    (quote! {
        #props_mod
        pub mod values {
            #values_mod
        }
    })
    .to_string()
}

fn gen_native_prop(
    prop: zfs_prop_t,
    prop_desc: zprop_desc_t,
    props_mod: &mut TokenStream,
    values_mod: &mut TokenStream,
) {
    let prop_ident = <&str>::from(prop).strip_prefix("ZFS_PROP_").unwrap();
    let prop_ident = CaseStyle::from_snakecase(prop_ident).to_pascalcase();
    let prop_ident = format_ident!("{}", prop_ident);

    let prop_name = unsafe { CStr::from_ptr(prop_desc.pd_name) }
        .to_str()
        .expect("unable to convert prop name to str");

    let col_name = unsafe { CStr::from_ptr(prop_desc.pd_colname) }
        .to_str()
        .unwrap()
        .to_string();
    let right_align = <bool>::from(prop_desc.pd_rightalign);
    let hidden = !<bool>::from(prop_desc.pd_visible);
    let read_only = <bool>::from(unsafe { zfs_prop_readonly(prop) });
    let set_once = <bool>::from(unsafe { zfs_prop_setonce(prop) });
    let inheritable = <bool>::from(unsafe { zfs_prop_inheritable(prop) });

    props_mod.extend(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct #prop_ident;
        impl NativeProperty for #prop_ident {
            const NAME: &'static str = #prop_name;
            const NAME_CSTR: &'static ::std::ffi::CStr = ::cstr::cstr!(#prop_name);
            const COLUMN_NAME: &'static str = #col_name;
            const RIGHT_ALIGN: bool = #right_align;
            const HIDDEN: bool = #hidden;
            const READ_ONLY: bool = #read_only;
            const SET_ONCE: bool = #set_once;
        }
        impl Property for #prop_ident {
            fn name(&self) -> &str { <Self as NativeProperty>::NAME }
            fn name_cstr(&self) -> &::std::ffi::CStr { <Self as NativeProperty>::NAME_CSTR }
        }
        impl<'a> ::std::fmt::Display for #prop_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(Self::NAME, f)
            }
        }
    });

    let value: syn::Type;
    let lookup: syn::Path;
    let default: Option<syn::Expr>;
    match prop_desc.pd_proptype {
        zprop_type_t::PROP_TYPE_STRING => {
            value = syn::parse2(quote! {&'a str}).expect("failed to parse string type");
            lookup = syn::parse2(quote! {lookup_string}).expect("failed to parse lookup_string");
            let default_str = if prop_desc.pd_strdefault.is_null() {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(prop_desc.pd_strdefault) }
                        .to_str()
                        .expect("unable to convert default string CStr to str"),
                )
            };
            default = default_str
                .map(|s| syn::parse2(quote! {#s}).expect("unable to parse default string as expr"));
        }
        zprop_type_t::PROP_TYPE_NUMBER => {
            value = syn::parse2(quote! {u64}).expect("failed to parse int type");
            lookup = syn::parse2(quote! {lookup_number}).expect("failed to parse lookup_number");
            let default_num = prop_desc.pd_numdefault;
            default =
                Some(syn::parse2(quote! {#default_num}).expect("unable to parse default number"));
        }
        zprop_type_t::PROP_TYPE_INDEX => {
            value = syn::parse2(quote! {values::#prop_ident}).expect("failed to parse index type");
            let (vals, idx_default) = gen_index_value(&prop_ident, &prop_desc);
            values_mod.extend(vals);
            lookup = syn::parse2(quote! {lookup_index}).expect("failed to parse lookup_index");
            default = idx_default.map(|d| {
                syn::parse2(quote! {values::#prop_ident::#d})
                    .expect("failed to parse index default")
            });
        }
    }
    if let Some(default) = default {
        props_mod.extend(quote! {
            impl<'a, D> Default for Value<'a, D, #prop_ident>
            where
                D: crate::datasets::Dataset,
                #prop_ident: DatasetProperty<'a, D, Value=#value>,
            {
                fn default() -> Self {
                    Self {
                        value: #default,
                        source: Source::Default,
                    }
                }
            }
        });
    }

    for dataset in zfs_type_t::iter().filter(|ds_type| {
        <bool>::from(unsafe { zfs_prop_valid_for_type(prop as c_int, *ds_type, false.into()) })
    }) {
        let dataset: syn::TypePath = syn::parse_str(&format!(
            "crate::datasets::{}",
            CaseStyle::from_snakecase(super::datasets::zfs_type_to_name(dataset.into()))
                .to_pascalcase()
        ))
        .expect("failed to parse datasets type");

        props_mod.extend(quote! {
            impl<'a> DatasetProperty<'a, #dataset<'a>> for #prop_ident {
                type Value = #value;

                fn lookup(&self, dataset: &'a #dataset) -> Result<Option<Value<'a, #dataset<'a>, Self>>> {
                    #lookup(self, dataset)
                }
            }
        });
    }

    if inheritable {
        props_mod.extend(quote! {
            impl<'a, D: Dataset> InheritableProperty<D> for #prop_ident
            where
                Self: DatasetProperty<'a, D>
            {}
        });
    }

    if !read_only {
        props_mod.extend(quote! {
            impl<'a, 'b, D: Dataset> SetableProperty<'a, D> for #prop_ident
            where
                Self: DatasetProperty<'b, D>,
            {
                type Value = #value;
                fn add_value_to_list(
                    &self,
                    value: Self::Value,
                    list: &mut SetPropertyValues<D>,
                ) -> Result<()> {
                    unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
                }
            }
        });
    }

    if !read_only || set_once {
        props_mod.extend(quote! {
            impl<'a, 'b, D: Dataset> SetableOnCreateProperty<'a, D> for #prop_ident
            where
                Self: DatasetProperty<'b, D>,
            {
                type Value = #value;
                fn add_value_to_list(
                    &self,
                    value: Self::Value,
                    list: &mut SetOnCreatePropertyValues<D>,
                ) -> Result<()> {
                    unsafe { value.add_to_nvlist(&mut list.0, self.name_cstr()) }
                }
            }
        });
    }
}

fn gen_index_value(
    prop_ident: &syn::Ident,
    prop_desc: &zprop_desc_t,
) -> (TokenStream, Option<syn::Ident>) {
    let data = unsafe { prop_desc.pd_table.as_ref() }.unwrap();
    let len = prop_desc.pd_table_size as usize;
    let prop_index_vals: &'static [zfs_sys::zprop_index_t] =
        unsafe { std::slice::from_raw_parts(data, len) };

    let (val_ident, val_index): (Vec<_>, Vec<_>) = prop_index_vals
        .iter()
        .unique_by(|v| v.pi_value)
        .map(|v| {
            let mut ident = unsafe { CStr::from_ptr(v.pi_name) }
                .to_str()
                .unwrap()
                .replace('-', "_")
                .replace(',', "_");
            if ident.chars().next().unwrap().is_numeric() {
                ident = format! {"N{}", ident};
            }
            let ident = format_ident!("{}", CaseStyle::from_snakecase(ident).to_pascalcase());
            (ident, v.pi_value as u64)
        })
        .unzip();

    let val_names: Vec<Vec<String>> = val_index
        .iter()
        .map(|idx| {
            prop_index_vals
                .iter()
                .filter(|v| v.pi_value == *idx)
                .map(|v| {
                    unsafe { CStr::from_ptr(v.pi_name) }
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect()
        })
        .collect();

    let idx_vals = quote! {
        #[repr(u64)]
        #[derive(Copy, Clone, PartialEq, Debug, num_derive::FromPrimitive, num_derive::ToPrimitive,
            strum_macros::AsRefStr, strum_macros::IntoStaticStr, strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
        pub enum #prop_ident {
            #(
                #[strum(#(serialize=#val_names),*)]
                #val_ident = #val_index,
            )*
        }
        impl crate::nvpair::NvPairAddValue for #prop_ident {
            unsafe fn add_to_nvlist<T: crate::nvpair::NvListT>(self, nvlist: &mut T, name: &::std::ffi::CStr) -> crate::Result<()> {
                crate::nvpair::NvPairAddValue::add_to_nvlist(num_traits::ToPrimitive::to_u64(&self).expect("failed to convert index value to u64"), nvlist, name)
            }
        }
    };

    let default = val_index
        .iter()
        .position(|idx| *idx == prop_desc.pd_numdefault)
        .map(|p| &val_ident[p])
        .cloned();

    (idx_vals, default)
}
