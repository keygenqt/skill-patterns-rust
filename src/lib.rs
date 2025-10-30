use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

////////////////////////////
/// Chain of Responsibility
////////////////////////////
#[proc_macro_derive(ComponentHelp)]
pub fn deriv_print_help(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let text = format!("Help: simple component '{}'.", struct_name);
    quote! {
        impl ComponentHelp for #struct_name {
            fn print_help(&self) {
                println!(#text);
            }
        }
    }.into()
}