use proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn authenticated(input: TokenStream) -> TokenStream {

    println!("input: \"{}\"", input.to_string());
    input
}