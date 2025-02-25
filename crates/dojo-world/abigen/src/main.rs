/// Script that generates the bindings for World and Executor contracts.
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use cairo_lang_starknet_classes::contract_class::ContractClass;
use camino::Utf8PathBuf;
use scarb::core::{Config, TargetKind};
use scarb::ops::{CompileOpts, FeaturesOpts, FeaturesSelector};

const SCARB_MANIFEST: &str = "crates/dojo-core/Scarb.toml";
const SCARB_MANIFEST_BACKUP: &str = "crates/dojo-core/bak.Scarb.toml";
const SCARB_LOCK: &str = "crates/dojo-core/Scarb.lock";
const SCARB_LOCK_BACKUP: &str = "crates/dojo-core/bak.Scarb.lock";
const WORLD_ARTIFACT: &str = "crates/dojo-core/target/dev/dojo_world.contract_class.json";
const MODEL_ARTIFACT: &str =
    "crates/dojo-core/target/dev/dojo_resource_metadata.contract_class.json";
const OUT_DIR: &str = "crates/dojo-world/src/contracts/abi";

fn define_check_only() -> bool {
    let args: Vec<_> = std::env::args().collect();

    args.len() > 1 && args[1] == "--check"
}

fn main() {
    let is_check_only = define_check_only();

    compile_dojo_core();

    generate_bindings("WorldContract", WORLD_ARTIFACT, "world.rs", is_check_only);
    generate_bindings("ModelContract", MODEL_ARTIFACT, "model.rs", is_check_only);
}

/// Generates the bindings for the given contracts, or verifies
/// if the bindings are up to date.
fn generate_bindings(
    contract_name: &str,
    contract_artifact_path: &str,
    bindings_filename: &str,
    is_check_only: bool,
) {
    let contract =
        serde_json::from_reader::<_, ContractClass>(File::open(contract_artifact_path).unwrap())
            .expect("Could not read World Contract Class file");

    let bindings = get_bindings_file_content(contract_name, contract);
    let out_path = format!("{OUT_DIR}/{bindings_filename}");

    if is_check_only {
        if Path::new(&out_path).exists() {
            let existing_bindings = fs::read_to_string(out_path).expect("Could not read file");

            // Trim to remove the last empty line of the file.
            if existing_bindings.trim() != bindings {
                panic!(
                    "{contract_name} ABI bindings are not up to date. Consider generating them \
                     running `cargo run -p dojo-world-abigen`"
                );
            }
        } else {
            println!("No bindings found for {contract_name}, check skipped");
        }
    } else {
        write_file(&out_path, &bindings);
    }
}

fn rename_file(old_path: &str, new_path: &str) {
    let o = Path::new(old_path);
    let n = Path::new(new_path);
    fs::rename(o, n)
        .unwrap_or_else(|e| panic!("Could not rename file {old_path} into {new_path}: {e}"));
}

fn write_file(file_path: &str, content: &str) {
    let mut file = File::create(file_path).expect("Could not create file");
    writeln!(file, "{}", content).expect("Could not write Scarb.toml file");
}

/// Writes a binding file using cainome inlined ABI for the given contract.
fn get_bindings_file_content(contract_name: &str, contract_class: ContractClass) -> String {
    format!(
        "// AUTOGENERATED FILE, DO NOT EDIT.\n// To generate the bindings, please run `cargo run \
         --bin dojo-world-abigen` instead.\nuse cainome::rs::abigen;\n\nabigen!(\n    {},\n    \
         r#\"{}\"#,\ntype_aliases {{\ndojo::world::config::Config::Event as \
         DojoConfigEvent;\n}},\nderives(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize),\n);",
        contract_name,
        serde_json::to_string_pretty(&contract_class.abi).unwrap()
    )
}

/// Compiles dojo-core contracts targetting starknet contract without using dojo-plugin.
fn compile_dojo_core() {
    rename_file(SCARB_MANIFEST, SCARB_MANIFEST_BACKUP);

    if Path::new(SCARB_LOCK).exists() {
        rename_file(SCARB_LOCK, SCARB_LOCK_BACKUP);
    }

    // Write new Scarb.toml file with starknet contract target.
    let mut file = File::create(SCARB_MANIFEST).expect("Could not create file");
    writeln!(
        file,
        r#"
[package]
cairo-version = "2.7.0"
edition = "2024_07"
name = "dojo"
version = "1.0.0-alpha.4"

[dependencies]
starknet = "2.7.0"

[[target.starknet-contract]]
sierra = true
"#,
    )
    .expect("Could not write Scarb.toml file");

    let path = Utf8PathBuf::from(SCARB_MANIFEST);
    let config = Config::builder(path.canonicalize_utf8().unwrap()).build().unwrap();
    let ws = scarb::ops::read_workspace(config.manifest_path(), &config)
        .expect("Could not read Scarb workspace");
    let packages = ws.members().map(|p| p.id).collect();

    let features_opts =
        FeaturesOpts { features: FeaturesSelector::AllFeatures, no_default_features: false };

    scarb::ops::compile(
        packages,
        CompileOpts {
            include_target_names: vec![],
            include_target_kinds: vec![],
            exclude_target_kinds: vec![TargetKind::TEST],
            features: features_opts,
        },
        &ws,
    )
    .expect("Could not run Scarb compile");

    rename_file(SCARB_MANIFEST_BACKUP, SCARB_MANIFEST);
    rename_file(SCARB_LOCK_BACKUP, SCARB_LOCK);
}
