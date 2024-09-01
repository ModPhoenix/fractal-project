use std::sync::Arc;

use crate::data::{self, Fractal};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, MergedObject, Object, Result, Schema,
};
use kuzu::Database;
use uuid::Uuid;

#[derive(Default)]
pub struct FractalQueries;

#[Object]
impl FractalQueries {
    async fn fractal(&self, ctx: &Context<'_>, name: String) -> Result<FractalGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db)?;
        let fractal = data::get_fractal_by_name(&conn, &name)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let children = data::get_fractal_children(&db, &fractal.id)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(FractalGraphQL {
            id: fractal.id,
            name: fractal.name,
            children: children.into_iter().map(FractalGraphQL::from).collect(),
        })
    }
}

#[derive(async_graphql::SimpleObject)]
struct FractalGraphQL {
    id: Uuid,
    name: String,
    children: Vec<FractalGraphQL>,
}

impl From<Fractal> for FractalGraphQL {
    fn from(f: Fractal) -> Self {
        FractalGraphQL {
            id: f.id,
            name: f.name,
            children: Vec::new(),
        }
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(FractalQueries);

pub type FractalSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
