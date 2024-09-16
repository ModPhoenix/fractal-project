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
    parent_id: Uuid,
    context_ids: Option<Uuid>,
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

        let fractal = data::create_fractal(
            &conn,
            &input.name,
            Some(&input.parent_id),
            input.context_ids.as_ref(),
        )
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

    async fn add_relation(
        &self,
        ctx: &Context<'_>,
        parent_id: Uuid,
        child_id: Uuid,
        context_id: Option<Uuid>,
    ) -> Result<bool> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        data::add_has_child_edge(&conn, &parent_id, &child_id, context_id.as_ref())
            .map_err(GraphQLError::from)?;

        Ok(true)
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

        Ok(KnowledgeGraphQL::from_knowledge(knowledge)?)
    }
}

#[derive(MergedObject, Default)]
pub struct MutationRoot(FractalMutations);

#[derive(Default)]
pub struct FractalQueries;

#[Object]
impl FractalQueries {
    async fn fractal(&self, ctx: &Context<'_>, name: Option<String>) -> Result<FractalGraphQL> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let name = name.unwrap_or("Root".to_string());
        let fractal = data::get_fractal_by_name(&conn, &name).map_err(|e| match e {
            data::DataError::FractalNotFound(_) => {
                GraphQLError::NotFound(format!("Fractal '{}' not found", name))
            }
            _ => GraphQLError::from(e),
        })?;

        Ok(FractalGraphQL {
            id: fractal.id,
            name: fractal.name,
            created_at: fractal.created_at,
            updated_at: fractal.updated_at,
        })
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

        Ok(KnowledgeGraphQL {
            id: knowledge[0].id,
            content: knowledge[0].content.clone(),
        })
    }
}

struct FractalGraphQL {
    id: Uuid,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[Object]
impl FractalGraphQL {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<FractalGraphQL>> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let children =
            data::get_fractal_relations(&conn, &self.id, "children").map_err(GraphQLError::from)?;

        Ok(children.into_iter().map(FractalGraphQL::from).collect())
    }

    async fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    async fn parents(&self, ctx: &Context<'_>) -> Result<Vec<FractalGraphQL>> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let children =
            data::get_fractal_relations(&conn, &self.id, "parents").map_err(GraphQLError::from)?;

        Ok(children.into_iter().map(FractalGraphQL::from).collect())
    }

    async fn contexts(&self, ctx: &Context<'_>) -> Result<Vec<FractalGraphQL>> {
        let db = ctx.data::<Arc<Database>>()?;
        let conn = data::create_connection(&db).map_err(GraphQLError::from)?;

        let children =
            data::get_fractal_relations(&conn, &self.id, "contexts").map_err(GraphQLError::from)?;

        Ok(children.into_iter().map(FractalGraphQL::from).collect())
    }
}

struct KnowledgeGraphQL {
    id: Uuid,
    content: String,
}

#[Object]
impl KnowledgeGraphQL {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn content(&self) -> String {
        self.content.clone()
    }

    async fn fractal(&self, ctx: &Context<'_>) -> FractalGraphQL {
        FractalQueries
            .fractal(ctx, Some("Root".to_string()))
            .await
            .unwrap()
    }
}

impl From<Fractal> for FractalGraphQL {
    fn from(f: Fractal) -> Self {
        FractalGraphQL {
            id: f.id,
            name: f.name,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(FractalQueries);

pub type FractalSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

impl KnowledgeGraphQL {
    fn from_knowledge(k: data::Knowledge) -> Result<Self, GraphQLError> {
        Ok(KnowledgeGraphQL {
            id: k.id,
            content: k.content,
        })
    }
}
