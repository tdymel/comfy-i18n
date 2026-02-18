use std::str::FromStr;

use proc_macro2::TokenStream;

pub trait ToBasicTokenStream {
    fn to_basic_token_stream(&self) -> TokenStream;
}

impl ToBasicTokenStream for &str {
    fn to_basic_token_stream(&self) -> TokenStream {
        TokenStream::from_str(self).unwrap()
    }
}

impl ToBasicTokenStream for String {
    fn to_basic_token_stream(&self) -> TokenStream {
        self.as_str().to_basic_token_stream()
    }
}
