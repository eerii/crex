use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
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
pub fn export(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attributes to find the crate and exported module name
    let attr = parse_macro_input!(attr as Attr);
    let krate = &attr.krate;
    let macro_name = format_ident!("export_{}", &attr.macro_name);

    // Parse extern block
    let mut functions = vec![];
    let mut item = parse_macro_input!(item as syn::ItemForeignMod);
    for item in item.items.iter_mut() {
        let syn::ForeignItem::Fn(func) = item else {
            continue;
        };
        let fattrs = &func.attrs;
        let sig = &func.sig;
        let name = &sig.ident;
        let inputs = &sig.inputs;
        let output = &sig.output;

        // TODO: Check visibility

        // Check if the functions attributes have some conditional compilation logic
        // to exclude them from processing.
        for attr in fattrs {
            if attr.path().is_ident("cfg") {
                let Ok(cfg) = attr.parse_args::<syn::Expr>() else {
                    continue;
                };
                println!("cfg {}", cfg.into_token_stream());
            }
        }

        // This only handles the simple cases
        let mut input_names = inputs
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

        // Variadic arguments are hard!!! They require the unstable feature `c_varadic`.
        // Moreover, when using ..., we can't directly pass the arguments to a C function.
        // To mitigate this, we need to have a C function that accepts a `va_list`.
        // We use the convention `fn_name_valist` to denote this function. See `funcs.c`
        // for an example on how to set up the duo of functions.
        if sig.variadic.is_some() {
            let name_valist = format_ident!("{}_valist", name);
            let args = format_ident!("args");
            input_names.push(&args);

            functions.push(quote! {
                extern "C" {
                    #(#fattrs)* pub fn #name_valist(#inputs var_args: std::ffi::VaList) #output;
                }
                #(#fattrs)* pub unsafe extern "C" fn #name(#inputs mut args: ...) #output {
                    let args = args.as_va_list();
                    self::#name_valist(#(#input_names),*)
                }
            });
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
      #item
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
            info();
            test();
            test_args(1, 2);
            assert_eq!(test_ret(), 4);
            other_function();
            assert_eq!(OTHER_CONSTANT, 256);
            assert_eq!(test_variadic(5, 1, 2, 3, 4, 5), 15);
        }
    }
}
