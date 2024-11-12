use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Attr {
    name: syn::Ident,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn export(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attributes to find the exported module name
    let attr = parse_macro_input!(attr as Attr);
    let attr_name = &attr.name;
    let macro_name = format_ident!("export_{}", attr_name);
    let mod_name = format_ident!("{}_exported", attr_name);

    // Parse extern block
    let mut functions = vec![];
    let item = parse_macro_input!(item as syn::ItemForeignMod);
    for item in &item.items {
        if let syn::ForeignItem::Fn(func) = item {
            let sig = &func.sig;
            let name = &sig.ident;
            let inputs = &sig.inputs;
            // This only handles the simple cases
            let input_names = inputs
                .iter()
                .filter_map(|arg| {
                    let syn::FnArg::Typed(syn::PatType { pat, .. }) = arg else {
                        return None;
                    };
                    let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref() else {
                        return None;
                    };
                    Some(ident)
                })
                .collect::<Vec<_>>();
            let output = &sig.output;
            functions.push(quote! {
                // #[no_mangle]
                pub unsafe fn #name(#inputs) #output {
                    ::#attr_name::#name(#(#input_names),*)
                }
            });
        }
    }

    // Create the exported macro
    let export_macro = quote! {
        #[macro_export]
        macro_rules! #macro_name {
            () => {
                pub mod #mod_name {
                    use core::ffi::{c_char, c_int, c_ulong};
                    #(#functions)*
                }
            }
        }
    };

    // Put everything together
    let expanded = quote! {
      #item
      #export_macro
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    use test_lib::*;

    #[test]
    fn should_link() {
        unsafe {
            test();
            test_args(1, 50i8);
            assert_eq!(test_ret(), 4);
        }
    }
}
