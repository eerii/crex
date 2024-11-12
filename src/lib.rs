#[cfg(test)]
mod tests {
    use test_lib::*;

    #[test]
    fn link_test() {
        unsafe {
            test();
            test_args(1, 40i8);
            assert_eq!(test_ret(), 4);
        }
    }
}

//use proc_macro::TokenStream;
//use quote::quote;
//use syn::parse_macro_input;

//#[proc_macro_attribute]
//pub fn export(attr: TokenStream, item: TokenStream) -> TokenStream {
//    let attr = parse_macro_input!(attr as ItemExtern);
//    let item = parse_macro_input!(item as ItemExtern);
//
//    let mut expanded = quote! {};
//
//    for item in item.items {
//        if let syn::ForeignItemFn(func) = item {
//            let name = &func.sig.ident;
//            let args = func
//                .sig
//                .inputs
//                .iter()
//                .map(|arg| {
//                    if let syn::FnArg::Typed(pat) = arg {
//                        let ty = &pat.ty;
//                        quote!(#ty)
//                    } else {
//                        panic!("Unsupported argument type");
//                    }
//                })
//                .collect::<Vec<_>>();
//            let ret_ty = &func.sig.output;
//
//            expanded.extend(quote! {
//                export_c_symbol!(fn #name(#(#args),*) #ret_ty);
//            });
//        }
//    }
//
//    quote! {
//        #item
//        #expanded
//    }
//}

// As a workaround for [rust-lang/rfcs#2771][2771], you can use this macro to
// make sure the gstreamer-sys symbols are correctly exported in the `dylib`.
// From https://github.com/Michael-F-Bryan/ffi_helpers.
//#[doc(hidden)]
//#[macro_export]
//macro_rules! export_c_symbol {
//    (fn $name:ident($( $arg:ident : $type:ty ),*) -> $ret:ty) => {
//        #[no_mangle]
//        pub unsafe extern "C" fn $name($( $arg : $type),*) -> $ret {
//            gstreamer_sys_internal::$name($( $arg ),*)
//        }
//    };
//    (fn $name:ident($( $arg:ident : $type:ty ),*)) => {
//        export_c_symbol!(fn $name($( $arg : $type),*) -> ());
//    }
//}

//export_c_symbol!(fn gst_buffer_remove_meta(buffer: *mut GstBuffer, meta: *mut GstMeta) -> gboolean);
//export_c_symbol!(fn gst_element_class_add_metadata(klass: *mut GstElementClass, key: *const c_char, value: *const c_char));
//export_c_symbol!(fn gst_element_class_set_metadata(klass: *mut GstElementClass, longname: *const c_char, classification: *const c_char, description: *const c_char, author: *const c_char));
