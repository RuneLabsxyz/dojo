use std::env::var;
use cainome::parser::tokens::{Composite, CompositeInner, CompositeInnerKind, Token};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;
use crate::utils::torii_typegen::{get_ident, get_safe_ident};
use crate::utils::torii_typegen::shared::BaseDojoHandler;
use crate::utils::torii_typegen::types::TypeHandler;


pub trait DojoEnumHandler: BaseDojoHandler {
    fn handle_variant_type(&self, variant: &CompositeInner, parent: &Composite) -> Option<Type> {
        // If we have a unit type, return none
        if variant.token.type_name() == "()" {
            None
        } else {
            Some(self.get_type_handler().get_type(&variant.token))
        }
    }

    fn handle_variant(&self, variant: &CompositeInner, parent: &Composite) -> TokenStream {
        let name = get_ident(&*variant.name);
        let value = self.handle_variant_type(variant, parent)
            .map(|e|
                quote! { (#e) }
            );

        quote! {
            #name #value
        }
    }

    fn get_declaration(&self, entity: &Composite) -> TokenStream {
        let token = Token::Composite(entity.clone());

        let name = get_safe_ident(&*entity.type_name_or_alias());
        let generics = self.get_generics(entity);


        let annotations = self.get_attributes(&token);

        let variants: Vec<TokenStream> = entity.inners.iter()
            .map(|e| self.handle_variant(e, entity))
            .collect();

        quote! {
            #annotations
            pub enum #name #generics {
                #( #variants ),*
            }
        }
    }

    fn get(&self, entity: &Composite) -> TokenStream {
        if entity.is_builtin() {
            return quote!();
        }

        let declaration = self.get_declaration(entity);

        quote! {
            #declaration
        }
    }
}


#[derive(Debug)]
pub struct DefaultDojoEnumHandler<T: TypeHandler>(T);

impl<T: TypeHandler> DefaultDojoEnumHandler<T> {
    pub fn new(type_handler: T) -> Self {
        Self(type_handler)
    }
}

impl<T: TypeHandler> BaseDojoHandler for DefaultDojoEnumHandler<T> {
    fn get_type_handler<'a>(&'a self) -> &'a dyn TypeHandler {
        &self.0
    }
}

impl<T: TypeHandler> DojoEnumHandler for DefaultDojoEnumHandler<T> {}