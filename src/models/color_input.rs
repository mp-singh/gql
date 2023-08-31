use juniper::GraphQLInputObject;

#[derive(Clone, GraphQLInputObject)]
///a color
pub struct ColorInput {
    ///the name
    pub name: String,
}
