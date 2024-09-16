use crate::utils::torii_typegen::types::TypeHandler;

pub trait DojoEnumHandler {
    fn get_type_handler<'a>(&'a self) -> &'a dyn TypeHandler;
    
}