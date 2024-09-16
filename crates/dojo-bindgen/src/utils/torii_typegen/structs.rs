use cainome::parser::tokens::{Composite, CompositeInner, Token};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Token, Type};
use crate::utils::torii_typegen::types::TypeHandler;

// We use a trait to make it relatively easy to override the components
pub trait DojoStructHandler {
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

    // This function must not provide the commas. This will be handled by the struct generator
    fn get_field(&self, field: &CompositeInner, owner: &Composite) -> TokenStream {
        let ident = get_safe_ident(&*field.name);
        let ty = self.get_type_handler()
            .get_type(&field.token);

        quote!(#ident: #ty)
    }

    /// IMPORTANT: This function needs to be idempotent.
    /// For a given composite, it always gives out the same value.
    fn get_name(&self, composite: &Composite) -> Ident {
        get_safe_ident(&*composite.type_name_or_alias())
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

    fn get_declaration(&self, composite: &Composite) -> TokenStream {
        let token = Token::Composite(composite.clone());

        let attributes = self.get_attributes(&token);
        let name = self.get_name(composite);

        let generics = self.get_generics(composite);
        let generics = quote!(<#generics>);

        let fields = composite.inners.iter()
            .map(|e| self.get_field(e, composite));

        quote! {
            #attributes
            struct #name #generics {
                #(#fields),*
            }
        }
    }

    fn get_from_impl(&self, composite: &Composite) -> TokenStream {
        let dojo_entity = get_ident("torii_grpc::types::schema::Entity");
        let name = self.get(composite);

        // Generics adds a lot of difficulty to something that could be simple.
        // Fortunately, quote supports optionals, and will not output anything if the variant is
        // null. This is why we have the generics without the brackets, and others with.

        let generics = self.get_generics(composite).as_ref()
            .map(|e| quote!(<#e>));


        quote! {
            impl #generics From<#dojo_entity> for #name #generics {
                fn from(from: #dojo_entity) -> Self {
                    todo!()
                }
            }

            impl FromTorii #generics for #name #generics {
            }
        }
    }

    fn get_implementation(&self, composite: &Composite) -> TokenStream {
        let name = self.get_name(composite);

        quote! {
            impl #name {

            }
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

static RESERVED_KEYWORDS: [&'static str; 3] = [
    "type",
    "move",
    "final"
];

/// Utility function that allows to create a safe ident. In the case where the identifier is a reserved keyword,
/// this function call will return a raw ident (prefixed with r#) to work around the rust limitation.
pub fn get_safe_ident(ident_str: &str) -> Ident {
    if RESERVED_KEYWORDS.contains(&ident_str) {
        Ident::new_raw(ident_str, Span::call_site())
    } else {
        Ident::new(ident_str, Span::call_site())
    }
}

fn get_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}


#[derive(Debug)]
pub struct DefaultDojoStructHandler<T: TypeHandler>(T);

impl<T: TypeHandler> DefaultDojoStructHandler<T> {
    pub fn new(type_handler: T) -> Self {
        Self(type_handler)
    }
}

impl <T: TypeHandler> DojoStructHandler for DefaultDojoStructHandler<T> {
    fn get_type_handler<'a>(&'a self) -> &'a dyn TypeHandler {
        &self.0
    }
}