use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;

#[proc_macro]
pub fn key(_input: TokenStream) -> TokenStream {
    let mut rng = rand::thread_rng();

    let key: u8 = rng.gen_range(5..50);

    quote! {#key}.into()
}
