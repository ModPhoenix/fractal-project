use super::errors::GraphQLError;
use std::sync::Arc;

use crate::data::{self, Fractal};
use async_graphql::{
    Context, EmptySubscription, InputObject, MergedObject, Object, Result, Schema,
};
use kuzu::Database;
use uuid::Uuid;

#[derive(Default)]
pub struct FractalMutations;

#[derive(InputObject)]
struct CreateFractalInput {
    name: String,
    parent_id: Option<Uuid>,
}

#[Object]
impl FractalMutations {
    async fn create_fractal(
        &self,
        ctx: &Context<'_>,
        input: CreateFractalInput,
    ) -> Result<FractalGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let fractal = data::create_fractal(&conn, &input.name, input.parent_id.as_ref()).map_err(
            |e| match e {
                data::DataError::FractalAlreadyExists(_) => {
                    GraphQLError::InvalidInput(format!("Fractal '{}' already exists", input.name))
                }
                _ => GraphQLError::from(e),
            },
        )?;

        Ok(FractalGraphQL::from(fractal))
    }
}

#[derive(MergedObject, Default)]
pub struct MutationRoot(FractalMutations);

#[derive(Default)]
pub struct FractalQueries;

#[Object]
impl FractalQueries {
    async fn fractal(&self, ctx: &Context<'_>, name: String) -> Result<FractalGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;
        let fractal = data::get_fractal_by_name(&conn, &name).map_err(|e| match e {
            data::DataError::FractalNotFound(_) => {
                GraphQLError::NotFound(format!("Fractal '{}' not found", name))
            }
            _ => GraphQLError::from(e),
        })?;

        let children = data::get_fractal_children(&db, &fractal.id).map_err(GraphQLError::from)?;

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

pub type FractalSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
