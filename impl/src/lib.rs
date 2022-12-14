#![allow(non_snake_case)]
mod utils;
extern crate proc_macro;

macro_rules! new_derive {
    ($i:ident, $(attributes($($a:ident),*))?) => {
        mod $i;
        #[proc_macro_derive($i, $(attributes($($a),*))?)]
        pub fn $i(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $i::main(syn::parse_macro_input!(input as syn::DeriveInput))
                .unwrap_or_else(syn::Error::into_compile_error)
                .into()
        }
    };
}

new_derive! {All, attributes(all_doc)}
new_derive! {Not, attributes(not)}
new_derive! {FromToStr, attributes(fromtostr)}
