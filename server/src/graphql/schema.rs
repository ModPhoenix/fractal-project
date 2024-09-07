use super::errors::GraphQLError;
use std::sync::Arc;

use crate::data::{self, Fractal};
use async_graphql::{
    Context, EmptySubscription, InputObject, MergedObject, Object, Result, Schema, SimpleObject,
};
use kuzu::Database;
use uuid::Uuid;

#[derive(Default)]
pub struct FractalMutations;

#[derive(InputObject)]
struct CreateFractalInput {
    name: String,
    parent_id: Uuid,
    context_ids: Vec<Uuid>,
}

#[derive(InputObject)]
struct AddKnowledgeInput {
    fractal_id: Uuid,
    content: String,
    context: Vec<Uuid>,
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

        let parent_ids = vec![input.parent_id];
        let fractal = data::create_fractal(&conn, &input.name, &parent_ids, &input.context_ids)
            .map_err(|e| match e {
                data::DataError::FractalAlreadyExists(_) => {
                    GraphQLError::InvalidInput(format!("Fractal '{}' already exists", input.name))
                }
                _ => GraphQLError::from(e),
            })?;

        Ok(FractalGraphQL::from(fractal))
    }

    async fn delete_fractal(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        data::delete_fractal(&conn, &id)
            .map_err(GraphQLError::from)
            .map_err(Into::into)
    }

    async fn add_knowledge(
        &self,
        ctx: &Context<'_>,
        input: AddKnowledgeInput,
    ) -> Result<KnowledgeGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let knowledge =
            data::add_knowledge(&conn, &input.fractal_id, &input.content, &input.context)
                .map_err(GraphQLError::from)?;

        Ok(KnowledgeGraphQL::from_knowledge(&conn, knowledge)?)
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

        let children = data::get_fractal_relations(&conn, &fractal.id, "children")
            .map_err(GraphQLError::from)?;
        let parents = data::get_fractal_relations(&conn, &fractal.id, "parents")
            .map_err(GraphQLError::from)?;
        let contexts = data::get_fractal_relations(&conn, &fractal.id, "contexts")
            .map_err(GraphQLError::from)?;

        Ok(FractalGraphQL {
            id: fractal.id,
            name: fractal.name,
            children: children.into_iter().map(FractalGraphQL::from).collect(),
            created_at: fractal.created_at,
            updated_at: fractal.updated_at,
            parents: parents.into_iter().map(FractalGraphQL::from).collect(),
            contexts: contexts.into_iter().map(FractalGraphQL::from).collect(),
        })
    }

    async fn root(&self, ctx: &Context<'_>) -> Result<FractalGraphQL> {
        self.fractal(ctx, "Root".to_string()).await
    }

    async fn knowledge(
        &self,
        ctx: &Context<'_>,
        fractal_name: String,
        context: Vec<Uuid>,
    ) -> Result<KnowledgeGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;
        let knowledge = data::get_fractal_knowledge_with_context(&conn, &fractal_name, &context)
            .map_err(GraphQLError::from)?;

        if knowledge.is_empty() {
            return Err(GraphQLError::NotFound("Knowledge not found".to_string()).into());
        }

        let fractal = self.fractal(ctx, fractal_name).await?;

        Ok(KnowledgeGraphQL {
            id: knowledge[0].id,
            content: knowledge[0].content.clone(),
            fractal,
        })
    }
}

#[derive(SimpleObject)]
struct FractalGraphQL {
    id: Uuid,
    name: String,
    children: Vec<FractalGraphQL>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    parents: Vec<FractalGraphQL>,
    contexts: Vec<FractalGraphQL>,
}
#[derive(SimpleObject)]
struct KnowledgeGraphQL {
    id: Uuid,
    content: String,
    fractal: FractalGraphQL,
}

impl From<Fractal> for FractalGraphQL {
    fn from(f: Fractal) -> Self {
        FractalGraphQL {
            id: f.id,
            name: f.name,
            children: Vec::new(),
            created_at: f.created_at,
            updated_at: f.updated_at,
            parents: Vec::new(),
            contexts: Vec::new(),
        }
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(FractalQueries);

pub type FractalSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
impl FractalGraphQL {
    fn from_fractal(conn: &kuzu::Connection, f: Fractal) -> Result<Self, GraphQLError> {
        let children =
            data::get_fractal_relations(conn, &f.id, "children").map_err(GraphQLError::from)?;
        let parents =
            data::get_fractal_relations(conn, &f.id, "parents").map_err(GraphQLError::from)?;
        let contexts =
            data::get_fractal_relations(conn, &f.id, "contexts").map_err(GraphQLError::from)?;

        Ok(FractalGraphQL {
            id: f.id,
            name: f.name,
            children: children.into_iter().map(FractalGraphQL::from).collect(),
            created_at: f.created_at,
            updated_at: f.updated_at,
            parents: parents.into_iter().map(FractalGraphQL::from).collect(),
            contexts: contexts.into_iter().map(FractalGraphQL::from).collect(),
        })
    }
}

impl KnowledgeGraphQL {
    fn from_knowledge(conn: &kuzu::Connection, k: data::Knowledge) -> Result<Self, GraphQLError> {
        let fractal = data::get_fractal_by_name(conn, &k.content).map_err(GraphQLError::from)?;
        let fractal_graphql = FractalGraphQL::from_fractal(conn, fractal)?;

        Ok(KnowledgeGraphQL {
            id: k.id,
            content: k.content,
            fractal: fractal_graphql,
        })
    }
}
