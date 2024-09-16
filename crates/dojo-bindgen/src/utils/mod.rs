use proc_macro2::TokenStream;
use syn::File;

pub mod torii_typegen;


/// Writes a token stream into a formatted file using prettyplease.
pub fn write_file(header: &str, stream: TokenStream) -> Vec<u8> {

    let value: File = syn::parse2(stream).unwrap();
    let parsed_contents = prettyplease::unparse(&value);

    format!("{}\n{}", header, parsed_contents).into_bytes()
}
