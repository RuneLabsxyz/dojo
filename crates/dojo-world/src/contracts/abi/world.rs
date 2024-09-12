// AUTOGENERATED FILE, DO NOT EDIT.
// To generate the bindings, please run `cargo run --bin dojo-world-abigen` instead.
use cainome::rs::abigen;

abigen!(
    WorldContract,
    r#"[
  {
    "type": "impl",
    "name": "World",
    "interface_name": "dojo::world::world_contract::IWorld"
  },
  {
    "type": "struct",
    "name": "core::byte_array::ByteArray",
    "members": [
      {
        "name": "data",
        "type": "core::array::Array::<core::bytes_31::bytes31>"
      },
      {
        "name": "pending_word",
        "type": "core::felt252"
      },
      {
        "name": "pending_word_len",
        "type": "core::integer::u32"
      }
    ]
  },
  {
    "type": "struct",
    "name": "dojo::model::metadata::ResourceMetadata",
    "members": [
      {
        "name": "resource_id",
        "type": "core::felt252"
      },
      {
        "name": "metadata_uri",
        "type": "core::byte_array::ByteArray"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::array::Span::<core::felt252>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<core::felt252>"
      }
    ]
  },
  {
    "type": "enum",
    "name": "dojo::model::model::ModelIndex",
    "variants": [
      {
        "name": "Keys",
        "type": "core::array::Span::<core::felt252>"
      },
      {
        "name": "Id",
        "type": "core::felt252"
      },
      {
        "name": "MemberId",
        "type": "(core::felt252, core::felt252)"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::array::Span::<core::integer::u8>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<core::integer::u8>"
      }
    ]
  },
  {
    "type": "struct",
    "name": "dojo::model::layout::FieldLayout",
    "members": [
      {
        "name": "selector",
        "type": "core::felt252"
      },
      {
        "name": "layout",
        "type": "dojo::model::layout::Layout"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::array::Span::<dojo::model::layout::FieldLayout>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<dojo::model::layout::FieldLayout>"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::array::Span::<dojo::model::layout::Layout>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<dojo::model::layout::Layout>"
      }
    ]
  },
  {
    "type": "enum",
    "name": "dojo::model::layout::Layout",
    "variants": [
      {
        "name": "Fixed",
        "type": "core::array::Span::<core::integer::u8>"
      },
      {
        "name": "Struct",
        "type": "core::array::Span::<dojo::model::layout::FieldLayout>"
      },
      {
        "name": "Tuple",
        "type": "core::array::Span::<dojo::model::layout::Layout>"
      },
      {
        "name": "Array",
        "type": "core::array::Span::<dojo::model::layout::Layout>"
      },
      {
        "name": "ByteArray",
        "type": "()"
      },
      {
        "name": "Enum",
        "type": "core::array::Span::<dojo::model::layout::FieldLayout>"
      }
    ]
  },
  {
    "type": "enum",
    "name": "dojo::world::world_contract::Resource",
    "variants": [
      {
        "name": "Model",
        "type": "(core::starknet::class_hash::ClassHash, core::starknet::contract_address::ContractAddress)"
      },
      {
        "name": "Contract",
        "type": "(core::starknet::class_hash::ClassHash, core::starknet::contract_address::ContractAddress)"
      },
      {
        "name": "Namespace",
        "type": "()"
      },
      {
        "name": "World",
        "type": "()"
      },
      {
        "name": "Unregistered",
        "type": "()"
      }
    ]
  },
  {
    "type": "enum",
    "name": "core::bool",
    "variants": [
      {
        "name": "False",
        "type": "()"
      },
      {
        "name": "True",
        "type": "()"
      }
    ]
  },
  {
    "type": "interface",
    "name": "dojo::world::world_contract::IWorld",
    "items": [
      {
        "type": "function",
        "name": "metadata",
        "inputs": [
          {
            "name": "resource_selector",
            "type": "core::felt252"
          }
        ],
        "outputs": [
          {
            "type": "dojo::model::metadata::ResourceMetadata"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "set_metadata",
        "inputs": [
          {
            "name": "metadata",
            "type": "dojo::model::metadata::ResourceMetadata"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "register_namespace",
        "inputs": [
          {
            "name": "namespace",
            "type": "core::byte_array::ByteArray"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "register_model",
        "inputs": [
          {
            "name": "class_hash",
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "upgrade_model",
        "inputs": [
          {
            "name": "class_hash",
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "deploy_contract",
        "inputs": [
          {
            "name": "salt",
            "type": "core::felt252"
          },
          {
            "name": "class_hash",
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "outputs": [
          {
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "upgrade_contract",
        "inputs": [
          {
            "name": "selector",
            "type": "core::felt252"
          },
          {
            "name": "class_hash",
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "outputs": [
          {
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "init_contract",
        "inputs": [
          {
            "name": "selector",
            "type": "core::felt252"
          },
          {
            "name": "init_calldata",
            "type": "core::array::Span::<core::felt252>"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "uuid",
        "inputs": [],
        "outputs": [
          {
            "type": "core::integer::u32"
          }
        ],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "emit",
        "inputs": [
          {
            "name": "keys",
            "type": "core::array::Array::<core::felt252>"
          },
          {
            "name": "values",
            "type": "core::array::Span::<core::felt252>"
          }
        ],
        "outputs": [],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "entity",
        "inputs": [
          {
            "name": "model_selector",
            "type": "core::felt252"
          },
          {
            "name": "index",
            "type": "dojo::model::model::ModelIndex"
          },
          {
            "name": "layout",
            "type": "dojo::model::layout::Layout"
          }
        ],
        "outputs": [
          {
            "type": "core::array::Span::<core::felt252>"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "set_entity",
        "inputs": [
          {
            "name": "model_selector",
            "type": "core::felt252"
          },
          {
            "name": "index",
            "type": "dojo::model::model::ModelIndex"
          },
          {
            "name": "values",
            "type": "core::array::Span::<core::felt252>"
          },
          {
            "name": "layout",
            "type": "dojo::model::layout::Layout"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "delete_entity",
        "inputs": [
          {
            "name": "model_selector",
            "type": "core::felt252"
          },
          {
            "name": "index",
            "type": "dojo::model::model::ModelIndex"
          },
          {
            "name": "layout",
            "type": "dojo::model::layout::Layout"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "base",
        "inputs": [],
        "outputs": [
          {
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "resource",
        "inputs": [
          {
            "name": "selector",
            "type": "core::felt252"
          }
        ],
        "outputs": [
          {
            "type": "dojo::world::world_contract::Resource"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "is_owner",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "address",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [
          {
            "type": "core::bool"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "grant_owner",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "address",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "revoke_owner",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "address",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "is_writer",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "contract",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [
          {
            "type": "core::bool"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "grant_writer",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "contract",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "revoke_writer",
        "inputs": [
          {
            "name": "resource",
            "type": "core::felt252"
          },
          {
            "name": "contract",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      }
    ]
  },
  {
    "type": "impl",
    "name": "UpgradeableWorld",
    "interface_name": "dojo::world::world_contract::IUpgradeableWorld"
  },
  {
    "type": "interface",
    "name": "dojo::world::world_contract::IUpgradeableWorld",
    "items": [
      {
        "type": "function",
        "name": "upgrade",
        "inputs": [
          {
            "name": "new_class_hash",
            "type": "core::starknet::class_hash::ClassHash"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      }
    ]
  },
  {
    "type": "impl",
    "name": "UpgradeableState",
    "interface_name": "dojo::world::update::IUpgradeableState"
  },
  {
    "type": "struct",
    "name": "dojo::world::update::StorageUpdate",
    "members": [
      {
        "name": "key",
        "type": "core::felt252"
      },
      {
        "name": "value",
        "type": "core::felt252"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::array::Span::<dojo::world::update::StorageUpdate>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<dojo::world::update::StorageUpdate>"
      }
    ]
  },
  {
    "type": "struct",
    "name": "dojo::world::update::ProgramOutput",
    "members": [
      {
        "name": "prev_state_root",
        "type": "core::felt252"
      },
      {
        "name": "new_state_root",
        "type": "core::felt252"
      },
      {
        "name": "block_number",
        "type": "core::felt252"
      },
      {
        "name": "block_hash",
        "type": "core::felt252"
      },
      {
        "name": "config_hash",
        "type": "core::felt252"
      },
      {
        "name": "world_da_hash",
        "type": "core::felt252"
      },
      {
        "name": "message_to_starknet_segment",
        "type": "core::array::Span::<core::felt252>"
      },
      {
        "name": "message_to_appchain_segment",
        "type": "core::array::Span::<core::felt252>"
      }
    ]
  },
  {
    "type": "interface",
    "name": "dojo::world::update::IUpgradeableState",
    "items": [
      {
        "type": "function",
        "name": "upgrade_state",
        "inputs": [
          {
            "name": "new_state",
            "type": "core::array::Span::<dojo::world::update::StorageUpdate>"
          },
          {
            "name": "program_output",
            "type": "dojo::world::update::ProgramOutput"
          },
          {
            "name": "program_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      }
    ]
  },
  {
    "type": "impl",
    "name": "ConfigImpl",
    "interface_name": "dojo::world::config::IConfig"
  },
  {
    "type": "interface",
    "name": "dojo::world::config::IConfig",
    "items": [
      {
        "type": "function",
        "name": "set_differ_program_hash",
        "inputs": [
          {
            "name": "program_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "set_merger_program_hash",
        "inputs": [
          {
            "name": "program_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "get_differ_program_hash",
        "inputs": [],
        "outputs": [
          {
            "type": "core::felt252"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "get_merger_program_hash",
        "inputs": [],
        "outputs": [
          {
            "type": "core::felt252"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "set_facts_registry",
        "inputs": [
          {
            "name": "address",
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "get_facts_registry",
        "inputs": [],
        "outputs": [
          {
            "type": "core::starknet::contract_address::ContractAddress"
          }
        ],
        "state_mutability": "view"
      }
    ]
  },
  {
    "type": "constructor",
    "name": "constructor",
    "inputs": [
      {
        "name": "contract_base",
        "type": "core::starknet::class_hash::ClassHash"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::WorldSpawned",
    "kind": "struct",
    "members": [
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "creator",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::ContractDeployed",
    "kind": "struct",
    "members": [
      {
        "name": "salt",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      },
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "namespace",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "name",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::ContractUpgraded",
    "kind": "struct",
    "members": [
      {
        "name": "class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      },
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::ContractInitialized",
    "kind": "struct",
    "members": [
      {
        "name": "selector",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "init_calldata",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::WorldUpgraded",
    "kind": "struct",
    "members": [
      {
        "name": "class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::MetadataUpdate",
    "kind": "struct",
    "members": [
      {
        "name": "resource",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "uri",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::NamespaceRegistered",
    "kind": "struct",
    "members": [
      {
        "name": "namespace",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "hash",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::ModelRegistered",
    "kind": "struct",
    "members": [
      {
        "name": "name",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "namespace",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      },
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::ModelUpgraded",
    "kind": "struct",
    "members": [
      {
        "name": "name",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "namespace",
        "type": "core::byte_array::ByteArray",
        "kind": "data"
      },
      {
        "name": "class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      },
      {
        "name": "prev_class_hash",
        "type": "core::starknet::class_hash::ClassHash",
        "kind": "data"
      },
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "prev_address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::StoreSetRecord",
    "kind": "struct",
    "members": [
      {
        "name": "table",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "entity_id",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "keys",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      },
      {
        "name": "values",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::StoreUpdateRecord",
    "kind": "struct",
    "members": [
      {
        "name": "table",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "entity_id",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "values",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::StoreUpdateMember",
    "kind": "struct",
    "members": [
      {
        "name": "table",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "entity_id",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "member_selector",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "values",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::StoreDelRecord",
    "kind": "struct",
    "members": [
      {
        "name": "table",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "entity_id",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::WriterUpdated",
    "kind": "struct",
    "members": [
      {
        "name": "resource",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "contract",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "value",
        "type": "core::bool",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::OwnerUpdated",
    "kind": "struct",
    "members": [
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "resource",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "value",
        "type": "core::bool",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::config::Config::DifferProgramHashUpdate",
    "kind": "struct",
    "members": [
      {
        "name": "program_hash",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::config::Config::MergerProgramHashUpdate",
    "kind": "struct",
    "members": [
      {
        "name": "program_hash",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::config::Config::FactsRegistryUpdate",
    "kind": "struct",
    "members": [
      {
        "name": "address",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::config::Config::Event",
    "kind": "enum",
    "variants": [
      {
        "name": "DifferProgramHashUpdate",
        "type": "dojo::world::config::Config::DifferProgramHashUpdate",
        "kind": "nested"
      },
      {
        "name": "MergerProgramHashUpdate",
        "type": "dojo::world::config::Config::MergerProgramHashUpdate",
        "kind": "nested"
      },
      {
        "name": "FactsRegistryUpdate",
        "type": "dojo::world::config::Config::FactsRegistryUpdate",
        "kind": "nested"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::StateUpdated",
    "kind": "struct",
    "members": [
      {
        "name": "da_hash",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "dojo::world::world_contract::world::Event",
    "kind": "enum",
    "variants": [
      {
        "name": "WorldSpawned",
        "type": "dojo::world::world_contract::world::WorldSpawned",
        "kind": "nested"
      },
      {
        "name": "ContractDeployed",
        "type": "dojo::world::world_contract::world::ContractDeployed",
        "kind": "nested"
      },
      {
        "name": "ContractUpgraded",
        "type": "dojo::world::world_contract::world::ContractUpgraded",
        "kind": "nested"
      },
      {
        "name": "ContractInitialized",
        "type": "dojo::world::world_contract::world::ContractInitialized",
        "kind": "nested"
      },
      {
        "name": "WorldUpgraded",
        "type": "dojo::world::world_contract::world::WorldUpgraded",
        "kind": "nested"
      },
      {
        "name": "MetadataUpdate",
        "type": "dojo::world::world_contract::world::MetadataUpdate",
        "kind": "nested"
      },
      {
        "name": "NamespaceRegistered",
        "type": "dojo::world::world_contract::world::NamespaceRegistered",
        "kind": "nested"
      },
      {
        "name": "ModelRegistered",
        "type": "dojo::world::world_contract::world::ModelRegistered",
        "kind": "nested"
      },
      {
        "name": "ModelUpgraded",
        "type": "dojo::world::world_contract::world::ModelUpgraded",
        "kind": "nested"
      },
      {
        "name": "StoreSetRecord",
        "type": "dojo::world::world_contract::world::StoreSetRecord",
        "kind": "nested"
      },
      {
        "name": "StoreUpdateRecord",
        "type": "dojo::world::world_contract::world::StoreUpdateRecord",
        "kind": "nested"
      },
      {
        "name": "StoreUpdateMember",
        "type": "dojo::world::world_contract::world::StoreUpdateMember",
        "kind": "nested"
      },
      {
        "name": "StoreDelRecord",
        "type": "dojo::world::world_contract::world::StoreDelRecord",
        "kind": "nested"
      },
      {
        "name": "WriterUpdated",
        "type": "dojo::world::world_contract::world::WriterUpdated",
        "kind": "nested"
      },
      {
        "name": "OwnerUpdated",
        "type": "dojo::world::world_contract::world::OwnerUpdated",
        "kind": "nested"
      },
      {
        "name": "ConfigEvent",
        "type": "dojo::world::config::Config::Event",
        "kind": "nested"
      },
      {
        "name": "StateUpdated",
        "type": "dojo::world::world_contract::world::StateUpdated",
        "kind": "nested"
      }
    ]
  }
]"#,
type_aliases {
dojo::world::config::Config::Event as DojoConfigEvent;
},
derives(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize),
);
