use proc_macro2::{Ident, Span};

/// TODO: Write a bit more documentation about this generation
/// Go a bit more about:
/// - The torii deserialization, that breaks the cainome system (we have to create a custom one)
/// - The debug assertion
/// - A bit more explaination of why we use the following traits
/// - Explain a bit about the cross reference system.
///
pub mod structs;
pub mod types;
pub mod enums;

pub mod shared;

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

pub fn get_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}