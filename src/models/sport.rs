#[derive(Debug, Clone)]
pub struct Sport {
    pub id: i32,
    pub name: String,
}

// #[juniper::graphql_object(Context = Context)]
// impl Sport {
//     //  your resolvers

//     // To call the dataloader
//     pub async fn get_sport_by_ids(ctx: &Context, id: i32) -> Sport {
//         ctx.sport_loader.load(id).await
//     }
// }
