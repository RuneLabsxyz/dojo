use crate::utils::torii_typegen;
/// In this file, the type handler will be implemented.
use cainome::parser::tokens::{Array, Composite, Token, Tuple};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse_quote, GenericArgument, Type};

fn type_from_str(ty: &str) -> Type {
    syn::parse_str(ty).unwrap_or_else(|_| panic!("Unable to convert {ty} to a syn::Type"))
}

macro_rules! ty {
    ($content: expr) => {
        torii_typegen::types::type_from_str(&*format!($content))
    };
    ($content: expr, $($args: expr),*) => {
            torii_typegen::types::type_from_str(&*format!($content, $($args),*))
    };
}

pub trait TypeHandler {
    fn get_type(&self, token: &Token) -> Type {
        // This implementation is close to what there is in cainome.
        // But unfortunately, due to some limitations on visibility, we are not able to use them.
        // For now, we will continue duplicating the code for a bit.
        match token {
            Token::CoreBasic(t) => self.get_basic_type(&*t.type_path),
            Token::Array(array) => self.get_array(array),
            Token::Tuple(tuple) => self.get_tuple(tuple),
            Token::Composite(composite) => self.get_composite(composite),
            Token::GenericArg(arg) => self.get_generic_arg(arg),
            t => panic!("Unsupported type {t:#?}!"),
        }
    }

    fn get_basic_type(&self, type_name: &str) -> Type {
        let ccsp = self.get_cairo_serde_path();
        let type_name = type_name.split("::").last().unwrap_or(type_name);

        match type_name {
            "ClassHash" => ty!("{ccsp}::ContractAddress"),
            "ContractAddress" => ty!("{ccsp}::ContractAddress"),
            "EthAddress" => ty!("{ccsp}::EthAddress"),
            "felt252" => ty!("starknet::core::types::Felt"),
            "felt" => ty!("starknet::core::types::Felt"),
            "bytes31" => ty!("{ccsp}::Bytes31"),
            "ByteArray" => ty!("{ccsp}::ByteArray"),
            "NonZero" => ty!("{ccsp}::NonZero"),
            "U256" => ty!("{ccsp}::U256"),
            _ => ty!("{type_name}"),
        }
    }

    fn get_builtin_composite(&self, composite: &Composite) -> Option<Type> {
        let ccsp = self.get_cairo_serde_path();
        match &*composite.type_path {
            "EthAddress" => Some(ty!("{ccsp}::EthAddress")),
            "ByteArray" => Some(ty!("{ccsp}::ByteArray")),
            "NonZero" => Some(ty!("{ccsp}::NonZero")),
            "U256" => Some(ty!("{ccsp}::U256")),
            _ => None,
        }
    }

    fn get_array(&self, array: &Array) -> Type {
        if array.is_legacy {
            let base = ty!("{}::CairoArrayLegacy", self.get_cairo_serde_path());
            let inner = self.get_type(&*array.inner);
            parse_quote!(#base<#inner>)
        } else {
            let inner = self.get_type(&*array.inner);
            parse_quote!(Vec<#inner>)
        }
    }

    fn get_tuple(&self, tuple: &Tuple) -> Type {
        let mapped_component: Vec<Type> = tuple.inners.iter().map(|e| self.get_type(e)).collect();

        parse_quote! {
            (#(#mapped_component),*)
        }
    }

    fn get_composite(&self, composite: &Composite) -> Type {
        let base_type;
        if let Some(ty) = self.get_builtin_composite(composite) {
            base_type = ty;
        } else {
            base_type = ty!("{}", composite.type_name_or_alias())
        }

        // Add genericity to the mix

        if composite.is_generic() {
            let parameters: Vec<Type> =
                composite.generic_args.iter().map(|(_, ty)| self.get_type(ty)).collect();

            parse_quote!(#base_type<#(#parameters),*>)
        } else {
            base_type
        }
    }

    fn get_generic_arg(&self, arg: &String) -> Type {
        // This is not the best thing you could do, but this allows us
        // to return a type that could be used somewhere else.
        Type::Verbatim(Ident::new(arg, Span::call_site()).into_token_stream())
    }

    fn get_cairo_serde_path(&self) -> &'static str {
        "cainome::cairo_serde"
    }
}

#[derive(Debug)]
pub struct DefaultDojoTypeHandler;

impl TypeHandler for DefaultDojoTypeHandler {}
