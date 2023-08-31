use juniper::GraphQLObject;

#[derive(Clone, GraphQLObject)]
///a color
pub struct Color {
    ///the id
    pub id: i32,
    ///the name
    pub name: String,
}
