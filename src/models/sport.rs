use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct Sport {
    pub id: i32,
    pub name: String,
}
