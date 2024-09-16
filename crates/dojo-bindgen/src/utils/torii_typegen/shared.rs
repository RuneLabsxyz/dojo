use cainome::parser::tokens::{Composite, Token};
use proc_macro2::TokenStream;
use quote::quote;
use crate::utils::torii_typegen::get_safe_ident;
use crate::utils::torii_typegen::types::TypeHandler;

pub trait BaseDojoHandler {

    fn get_type_handler<'a>(&'a self) -> &'a dyn TypeHandler;


    fn get_derive_types(&self, component: &Token) -> Vec<&'static str> {
        vec!["Debug", "Clone"]
    }


    fn get_attributes(&self, component: &Token) -> TokenStream {
        let derives = self.get_derive_types(component);

        if !derives.is_empty() {
            quote!(#[derive( #(#derives),* )])
        } else {
            quote!()
        }
    }

    fn get_generics(&self, composite: &Composite) -> Option<TokenStream> {
        if composite.is_generic() {
            let generics = composite.generic_args.iter()
                .map(|(name, _)| get_safe_ident(name));

            Some(quote!(#(#generics),*))
        } else {
            None
        }
    }
}