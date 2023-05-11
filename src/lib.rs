extern crate syn;
use syn::{__private::quote, parse_macro_input, Block};
use proc_macro::TokenStream;
use quote::__private::TokenStream as TT2;

#[proc_macro]
pub fn cotime(inp: TokenStream) -> TokenStream {
    let i = parse_macro_input!(inp as Block);
    let v: TT2  = "#i".parse().unwrap();
    return TokenStream::from(quote::quote! {
        #[proc_macro]
        pub fn cotime_out(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let i = {
                #i
            };
            return proc_macro::TokenStream::from(quote::quote! {
                {
                    #v
                }
            })
        }
    })
}