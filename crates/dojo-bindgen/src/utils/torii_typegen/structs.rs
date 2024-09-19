use crate::utils::torii_typegen::shared::BaseDojoHandler;
use crate::utils::torii_typegen::types::TypeHandler;
use crate::utils::torii_typegen::{get_ident, get_safe_ident};
use cainome::parser::tokens::{Composite, CompositeInner, Token};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Token, Type};

// We use a trait to make it relatively easy to override the components
pub trait DojoStructHandler: BaseDojoHandler {
    // This function must not provide the commas. This will be handled by the struct generator
    fn get_field(&self, field: &CompositeInner, owner: &Composite) -> TokenStream {
        let ident = get_safe_ident(&*field.name);
        let ty = self.get_type_handler().get_type(&field.token);

        quote!(#ident: #ty)
    }

    /// IMPORTANT: This function needs to be idempotent.
    /// For a given composite, it always gives out the same value.
    fn get_name(&self, composite: &Composite) -> Ident {
        get_safe_ident(&*composite.type_name_or_alias())
    }

    fn get_declaration(&self, composite: &Composite) -> TokenStream {
        let token = Token::Composite(composite.clone());

        let attributes = self.get_attributes(&token);
        let name = self.get_name(composite);

        let generics = self.get_generics(composite);
        let generics = quote!(<#generics>);

        let fields = composite.inners.iter().map(|e| self.get_field(e, composite));

        quote! {
            #attributes
            struct #name #generics {
                #(#fields),*
            }
        }
    }

    fn get_field_name(&self, field: &CompositeInner, owner: &Composite) -> TokenStream {
        let ident = get_safe_ident(&*field.name);
        // let ty = self.get_type_handler().get_type(&field.token);

        quote!(#ident)
    }

    fn get_field_conversion(&self, field: &CompositeInner, owner: &Composite) -> TokenStream {
        let field_index = syn::Index::from(field.index);

        let conversion = self.get_type_handler().get_type_conversion(&field.token);

        quote!(models[0].children[#field_index].ty.#conversion)
    }

    fn get_from_impl(&self, composite: &Composite) -> TokenStream {
        let dojo_entity = quote!(torii_grpc::types::schema::Entity);
        let name = self.get_name(composite);

        // Generics adds a lot of difficulty to something that could be simple.
        // Fortunately, quote supports optionals, and will not output anything if the variant is
        // null. This is why we have the generics without the brackets, and others with.

        let generics = self.get_generics(composite).as_ref().map(|e| quote!(<#e>));
        let field_names = composite.inners.iter().map(|e| self.get_field_name(e, composite));
        let conversions = composite.inners.iter().map(|e| self.get_field_conversion(e, composite));

        quote! {
            impl #generics From<#dojo_entity> for #name #generics {
                fn from(from: #dojo_entity) -> Self {
                    // todo!()
                    #name {
                        #(#field_names: from.#conversions),*
                    }
                }
            }
        }
    }

    fn get_implementation(&self, composite: &Composite) -> TokenStream {
        let name = self.get_name(composite);

        let from_impl = self.get_from_impl(composite);

        quote! {
            #from_impl
        }
    }

    fn get(&self, composite: &Composite) -> TokenStream {
        // In debug mode, to validate the basic assertions, we check that the name
        // function is deterministic for that object
        #[cfg(debug_assertions)]
        {
            let first_name = self.get_name(composite);
            let second_name = self.get_name(composite);

            assert_eq!(first_name, second_name);
        }

        let declaration = self.get_declaration(composite);
        let implementation = self.get_implementation(composite);

        quote! {
            #declaration

            #implementation
        }
    }
}

#[derive(Debug)]
pub struct DefaultDojoStructHandler<T: TypeHandler>(T);

impl<T: TypeHandler> DefaultDojoStructHandler<T> {
    pub fn new(type_handler: T) -> Self {
        Self(type_handler)
    }
}

impl<T: TypeHandler> BaseDojoHandler for DefaultDojoStructHandler<T> {
    fn get_type_handler<'a>(&'a self) -> &'a dyn TypeHandler {
        &self.0
    }
}

impl<T: TypeHandler> DojoStructHandler for DefaultDojoStructHandler<T> {}
