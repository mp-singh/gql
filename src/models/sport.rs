use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
pub struct Sport {
    pub id: i32,
    pub name: String,
}

// #[graphql_object(context = Database)]
// impl Sport {
//     //  your resolvers

//     // To call the dataloader
//     async fn get_sport_by_ids(ctx: &Database, id: i32) -> Sport {
//         ctx.sport_loader.load(id).await
//     }
// }
