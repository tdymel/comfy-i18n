use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;

pub trait ToBasicTokenSreamVec {
    fn to_token_stream(&self) -> Vec<TokenStream>;
}

impl<T> ToBasicTokenSreamVec for Vec<T>
where
    T: ToTokens,
{
    fn to_token_stream(&self) -> Vec<TokenStream> {
        self.iter()
            .map(|it| it.to_token_stream())
            .collect::<Vec<_>>()
    }
}

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
