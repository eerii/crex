use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Attr {
    /// The name of the crate where the external block is.
    krate: syn::Ident,
    /// The macro will be called `export_{name}`.
    /// Optional, can be used to define macros for multiple extern blocks in the same crate.
    macro_name: syn::Ident,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let krate: syn::Ident = input.parse()?;
        let macro_name = input.parse().unwrap_or(krate.clone());
        Ok(Self { krate, macro_name })
    }
}

#[proc_macro_attribute]
pub fn export(attr: TokenStream, items: TokenStream) -> TokenStream {
    // Parse attributes to find the crate and exported module name
    let attr = parse_macro_input!(attr as Attr);
    let krate = &attr.krate;
    let macro_name = format_ident!("export_{}", &attr.macro_name);

    // Parse extern block
    let mut functions = vec![];
    let items = parse_macro_input!(items as syn::ItemForeignMod);
    for item in &items.items {
        let syn::ForeignItem::Fn(func) = item else {
            continue;
        };
        let fattrs = &func.attrs;
        let sig = &func.sig;
        let name = &sig.ident;
        let inputs = &sig.inputs;
        let output = &sig.output;

        // TODO: Check visibility

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

        // Variadic arguments are hard!!! They require the unstable feature `c_variadic`.
        // Moreover, when using ..., we can't directly pass the arguments to a C function.
        // To mitigate this, we need to have a C function that accepts a `va_list`.
        // For now, we just skip them.
        if sig.variadic.is_some() {
            continue;
        };

        functions.push(quote! {
            // #[no_mangle] (This doesn't seem to be needed and it doesn't work)
            #(#fattrs)* pub unsafe extern "C" fn #name(#inputs) #output {
                #krate::#name(#(#input_names),*)
            }
        });
    }

    // Create the exported macro
    let export_macro = quote! {
        #[macro_export]
        macro_rules! #macro_name {
            () => {
                #(#functions)*
            }
        }
    };

    // Put everything together
    let expanded = quote! {
      #items
      #export_macro
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    use test_lib::*;

    /// This test just verifies that everything links correctly.
    ///
    /// The static version works right away since rust already reexports symbols for this type of
    /// library. Use `cargo test` to check.
    ///
    /// Running `cargo test --features "dynamic"` will try to dynamically link `test_lib`. If you
    /// go to `target/debug` there should be a `libtest_dylib.so` file there. If you try to do the
    /// same without calling `export_symbols` in `test_dylib` you will see that there are missing
    /// symbols when linking.
    #[test]
    fn should_link() {
        unsafe {
            // Debug information
            info();

            // Reexported functions
            test();
            test_args(1, 2);
            assert_eq!(test_ret(), 4);

            // Symbols outside the extern block
            other_function();
            assert_eq!(OTHER_CONSTANT, 256);
        }
    }
}
