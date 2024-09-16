use torii_grpc::types::schema::Entity;

pub trait FromTorii: Sized {
    fn from(entity: Entity) -> Self;
}

impl <T: From<Entity>> FromTorii for T {
    fn from(entity: Entity) -> T {
        entity.into()
    }
}