use crate::error::BindgenResult;
use crate::plugins::BuiltinPlugin;
use crate::{DojoData, DojoModel};
use async_trait::async_trait;
use cainome::rs::ExecutionVersion;
use chrono::{DateTime, Utc};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::path::{Path, PathBuf};
use cainome::parser::tokens::Token;
use syn::File;
use crate::utils::torii_typegen::enums::{DefaultDojoEnumHandler, DojoEnumHandler};
use crate::utils::torii_typegen::structs::{DefaultDojoStructHandler, DojoStructHandler};
use crate::utils::torii_typegen::types::DefaultDojoTypeHandler;

#[derive(Debug)]
pub struct GodotPlugin {
    generated_time: DateTime<Utc>,
    struct_handler: DefaultDojoStructHandler<DefaultDojoTypeHandler>,
    enum_handler: DefaultDojoEnumHandler<DefaultDojoTypeHandler>,
}

impl GodotPlugin {
    pub fn new() -> Self {
        Self {
            generated_time: Utc::now(),
            struct_handler: DefaultDojoStructHandler::new(DefaultDojoTypeHandler),
            enum_handler: DefaultDojoEnumHandler::new(DefaultDojoTypeHandler),
        }
    }

    fn get_header(&self) -> String {
        // NOTE: Procedural macros cannot insert comments.
        //  As this is meant to be read by a user, we hard-code it in the output.
        format!(
            "// Generated by dojo-bindgen on {}. Do not modify this file manually.\n",
            self.generated_time.to_rfc2822()
        )
    }

    fn get_imports(&self) -> TokenStream {
        quote! {
            use godot::prelude::*;
            use cainome::cairo_serde::ContractAddress;
            use dojo_types::schema::Struct as DojoStruct;
        }
    }

    fn handle_struct(&self, token: &Token) -> TokenStream {
        // TODO: At some point, we need to have a way to combine the same declarations.
        match token {
            Token::Composite(c) => {
                self.struct_handler.get(c)
            },
            _ => panic!("Not a composite, cannot create a struct!")
        }
    }

    fn handle_enum(&self, token: &Token) -> TokenStream {
        match token {
            Token::Composite(c) => {
                self.enum_handler.get(c)
            },
            _ => panic!("Not a composite, cannot create an enum!")
        }
    }

    fn handle_model(&self, name: &str, model: &DojoModel) -> TokenStream {
        let imports = self.get_imports();

        let structs = model.tokens.structs.iter()
            .map(|e| self.handle_struct(e));

        let enums = model.tokens.enums.iter()
            .map(|e| self.handle_enum(e));

        quote! {
            #imports

            #(#structs)*

            #(#enums)*
        }
    }

    fn get_file(&self, stream: TokenStream) -> Vec<u8> {
        // TODO: This is not optimized, at all. We clone & resize vectors, but welp
        // TODO: Remove the unwrap

        crate::utils::write_file(&*self.get_header(), stream)
    }
}

#[async_trait]
impl BuiltinPlugin for GodotPlugin {
    async fn generate_code(&self, data: &DojoData) -> BindgenResult<HashMap<PathBuf, Vec<u8>>> {
        let mut imports = vec![];

        let mut files: HashMap<PathBuf, Vec<u8>> = HashMap::new();

        // Before everything, do a pass for every struct that is declared multiple times.

        for (name, model) in &data.models {
            let name = name.split('-').last().unwrap().to_lowercase();
            imports.push(Ident::new(&*name, Span::call_site()));

            println!("Adding file {name}...");

            let path = Path::new(&format!("{}.rs", name)).to_owned();
            files.insert(path, self.get_file(self.handle_model(&*name, model)));

            println!("Added file! {name}");
        }

        let lib_contents = quote! {
            #( pub mod #imports; )*
        };

        println!("Finishing!!! {}", lib_contents);

        files.insert(Path::new("lib.rs").to_owned(), self.get_file(lib_contents));

        // TODO: Contracts?

        Ok(files)
    }
}
