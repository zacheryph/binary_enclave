#![feature(proc_macro_diagnostic)]

//! Macro crate for `binary_enclave`.

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, GenericArgument, ItemStatic, PathArguments, Type};

/// setup required linker options and trait impls
///
/// this puts in to place whats required for our enclave. Without
/// it, it will not be locatable for writing to after compilation.
/// Some systems may limit the length of section names. It is
/// recommended to keep the section name short and simple.
///
/// ```
/// #[enclave(binary_section_name)]
/// pub static OUR_STATIC: Enclave<ConfStruct, 128> = Enclave::new()
/// ```
#[proc_macro_attribute]
pub fn enclave(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemStatic = parse_macro_input!(item as ItemStatic);

    let (link_section, section) = if cfg!(target_os = "linux") {
        (format!(".{}", attr), format!(".{}", attr))
    } else if cfg!(target_os = "macos") {
        (format!("__DATA,__{}", attr), format!("__{}", attr))
    } else {
        ("".to_string(), "".to_string())
    };

    let segment = match item.ty.as_ref() {
        Type::Path(path) => {
            let seg = &path.path.segments;
            seg.first().unwrap().clone()
        }
        _ => {
            item.ty
                .span()
                .unwrap()
                .error("not sure how to handle this type")
                .emit();
            return TokenStream::new();
        }
    };

    if segment.ident != Ident::new("Enclave", Span::call_site()) {
        item.ty
            .span()
            .unwrap()
            .error("enclave must be of type Enclave");
        return TokenStream::new();
    }

    let generics = match segment.arguments {
        PathArguments::AngleBracketed(gens) => gens,
        _ => return TokenStream::new(),
    };

    let ty = match generics.args.first().unwrap() {
        GenericArgument::Type(ty) => ty,
        _ => return TokenStream::new(),
    };

    let output = quote! {
        #[no_mangle]
        #[link_section = #link_section]
        #item

        impl binary_enclave::EnclaveLocator for #ty {
            const SECTION: &'static str = #section;
        }
    };

    TokenStream::from(output)
}
