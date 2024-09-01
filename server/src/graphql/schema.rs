use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Object, Result, Schema};

#[derive(Default)]
pub struct FractalQueries;

#[Object]
impl FractalQueries {
    async fn fractal(&self) -> Result<String> {
        return Ok("Fractal".to_string());
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(FractalQueries);

pub type FractalSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
