use async_trait::async_trait;
use cainome::parser::tokens::{
    Composite, CompositeInner, CompositeType, CoreBasic, Function, FunctionOutputKind, Token,
};
use cainome::rs::{CairoEnum, CairoStruct};
use dojo_world::contracts::naming::{self, get_namespace_from_tag};
use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;
use std::ops::Index;
use std::path::{Path, PathBuf};

use crate::error::BindgenResult;
use crate::plugins::BuiltinPlugin;
use crate::{DojoContract, DojoData, DojoModel};

#[derive(Debug)]
pub struct BevyPlugin {}

impl BevyPlugin {
    pub fn new() -> Self {
        Self {}
    }

    fn generate_bindgen_warning() -> String {
        format!(
            "// Generated by dojo-bindgen on {}. Do not modify this file manually.\n",
            chrono::Utc::now().to_rfc2822()
        )
    }

    fn generate_bevy_imports() -> String {
        let mut code = String::new();
        code += "use bevy::prelude::*;\n";
        code += "use cainome::cairo_serde::ContractAddress;\n";
        code += "use dojo_types::schema::Struct as DojoStruct;\n";

        code
    }

    fn handle_components(
        &self,
        model: &DojoModel,
        handled_tokens: &mut HashMap<String, Composite>,
    ) -> String {
        let mut out = String::new();

        out += BevyPlugin::generate_bindgen_warning().as_str();
        out += BevyPlugin::generate_bevy_imports().as_str();
        // let tag = &model.tag;

        let custom_trait = "pub trait ToriiToBevy<T> {
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> T;
}\n";

        out += custom_trait;

        let mut token_stream: Vec<TokenStream2> = vec![];
        let derives = vec!["Component".to_string(), "Debug".to_string()];

        for token in &model.tokens.structs {
            let composite = token.to_composite().expect("composite expected");
            token_stream.push(CairoStruct::expand_decl(composite, &derives));
            token_stream.push(CairoStruct::expand_impl(composite));

            if token.type_name() == naming::get_name_from_tag(&model.tag) {
                let mut fields = String::from("");
                let composite_struct_name = composite.type_name();
                let mut all_fields: Vec<String> = vec![];

                for (index, struct_field) in composite.inners.iter().enumerate() {
                    match &struct_field.token {
                        Token::CoreBasic(_) => {
                            fields += format!(
                                "let {}: {};\n",
                                struct_field.name,
                                struct_field.token.type_name()
                            )
                            .as_str();
                            fields += BevyPlugin::gen_primitive_string(struct_field).as_str();

                            all_fields.push(struct_field.name.clone());
                        }
                        _ => {}
                    }
                }
                let full_string = all_fields.join(", ");
                fields += format!("{composite_struct_name} {{ {} }}", full_string).as_str();

                let custom_struct_impl = format!(
                    "impl ToriiToBevy<{}> for {} {{
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> {} {{
            {}\n
            }}
            }}\n",
                    composite.type_name(),
                    composite.type_name(),
                    composite.type_name(),
                    fields,
                );
                token_stream.push(custom_struct_impl.parse().unwrap());
            }
        }

        for token in &model.tokens.enums {
            let composite = token.to_composite().expect("composite expected");
            token_stream.push(CairoEnum::expand_decl(composite, &derives));
            token_stream.push(CairoEnum::expand_impl(composite));
        }

        token_stream.iter().for_each(|ts| {
            out += &format!("{}", ts);
        });

        out
    }

    fn gen_primitive_string(struct_field: &CompositeInner) -> String {
        let field_name = struct_field.name.clone();
        let index = struct_field.index;

        let primitive_string = match struct_field.token.type_name().as_str() {
            "ContractAddress" => {
                format!("{field_name} = ContractAddress::from(model.children[{index}].ty.as_primitive().unwrap().as_contract_address().unwrap());")
            }
            "u8" => {
                format!("{field_name} = model.children[{index}].ty.as_primitive().unwrap().as_u8().unwrap();")
            }
            "bool" => {
                format!("{field_name} = model.children[{index}].ty.as_primitive().unwrap().as_bool().unwrap();")
            }
            _ => "unknown type".to_string(),
        };

        primitive_string
    }
}

#[async_trait]
impl BuiltinPlugin for BevyPlugin {
    async fn generate_code(&self, data: &DojoData) -> BindgenResult<HashMap<PathBuf, Vec<u8>>> {
        let mut out: HashMap<PathBuf, Vec<u8>> = HashMap::new();
        let mut handled_tokens = HashMap::<String, Composite>::new();

        let imports_path = Path::new("components/mod.rs").to_owned();
        let mut mod_imports: Vec<String> = vec![];

        // Handle codegen for models
        for (name, model) in &data.models {
            let name = name.split('-').last().unwrap().to_lowercase();
            mod_imports.push(format!("pub mod {};", name));
            let models_path = Path::new(&format!("components/{}.rs", name)).to_owned();

            println!("Generating model: {}", name);
            let code = self.handle_components(model, &mut handled_tokens);

            out.insert(models_path, code.as_bytes().to_vec());
        }

        let import_code = format!("{}\n", mod_imports.join("\n"));
        out.insert(imports_path, import_code.as_bytes().to_vec());

        Ok(out)
    }
}
