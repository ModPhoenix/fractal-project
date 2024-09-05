use chrono::{DateTime, Utc};
use kuzu::{Connection, Database, Error as KuzuError, LogicalType, SystemConfig, Value};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Database error: {0}")]
    Database(#[from] KuzuError),
    #[error("Fractal with name '{0}' already exists")]
    FractalAlreadyExists(String),
    #[error("Fractal not found: {0}")]
    FractalNotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

#[derive(Debug, Clone)]
pub struct Fractal {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub fractal_id: Uuid,
}

pub fn create_db(db_path: &str) -> Result<Database, DataError> {
    let db = Database::new(db_path, SystemConfig::default())?;
    Ok(db)
}

// Function to create a new database connection
pub fn create_connection(db: &Database) -> Result<Connection, DataError> {
    let conn = Connection::new(db)?;

    Ok(conn)
}

pub fn init_database(conn: &Connection) -> Result<(), DataError> {
    println!("Initializing database...");

    // Create node tables
    conn.query(
        "CREATE NODE TABLE IF NOT EXISTS Fractal (
            id UUID,
            name STRING,
            createdAt TIMESTAMP,
            updatedAt TIMESTAMP,
            PRIMARY KEY (id)
        )",
    )?;

    conn.query(
        "CREATE NODE TABLE IF NOT EXISTS Knowledge (
            id UUID,
            content STRING,
            createdAt TIMESTAMP,
            updatedAt TIMESTAMP,
            PRIMARY KEY (id)
        )",
    )?;

    // Create relationship tables
    conn.query("CREATE REL TABLE IF NOT EXISTS HAS_CHILD(FROM Fractal TO Fractal)")?;
    conn.query("CREATE REL TABLE IF NOT EXISTS HAS_CONTEXT(FROM Fractal TO Fractal)")?;
    conn.query("CREATE REL TABLE IF NOT EXISTS HAS_KNOWLEDGE(FROM Fractal TO Knowledge)")?;
    conn.query("CREATE REL TABLE IF NOT EXISTS IN_CONTEXT(FROM Knowledge TO Fractal)")?;

    // Create constraints
    // conn.query("CREATE CONSTRAINT ON (f:Fractal) ASSERT f.id IS UNIQUE")?;
    // conn.query("CREATE CONSTRAINT ON (f:Fractal) ASSERT f.name IS UNIQUE")?;
    // conn.query("CREATE CONSTRAINT ON (k:Knowledge) ASSERT k.id IS UNIQUE")?;

    // Create indexes
    // conn.query("CREATE INDEX ON :Fractal(name)")?;
    // conn.query("CREATE INDEX ON :Knowledge(id)")?;

    let root_fractal = create_fractal(&conn, "Root", &[], &[]);

    match root_fractal {
        Ok(root) => {
            let _ = create_fractal(&conn, "Child1", &[root.id], &[root.id]);
            println!("Root fractal created.");
        }
        Err(_) => {
            println!("Root fractal already exists.");
        }
    }

    println!("Database initialization completed.");
    Ok(())
}

pub fn create_fractal(
    conn: &Connection,
    name: &str,
    parent_ids: &[Uuid],
    context_ids: &[Uuid],
) -> Result<Fractal, DataError> {
    if let Ok(_) = get_fractal_by_name(conn, name) {
        return Err(DataError::FractalAlreadyExists(name.to_string()));
    }

    let query = "
        CREATE (f:Fractal {
            id: $uuid,
            name: $name,
            createdAt: $datetime,
            updatedAt: $datetime
        })
        RETURN f.id, f.name, f.createdAt, f.updatedAt
    ";
    let mut stmt = conn.prepare(query)?;
    let result: kuzu::QueryResult = conn.execute(
        &mut stmt,
        vec![
            ("uuid", Value::UUID(Uuid::new_v4())),
            ("name", Value::String(name.to_string())),
            ("datetime", Value::Timestamp(OffsetDateTime::now_utc())),
        ],
    )?;

    dbg!("{:?}", result.get_compiling_time());
    dbg!("{:?}", result.get_execution_time());

    if let Some(row) = result.into_iter().next() {
        let fractal = row_to_fractal(&row)?;

        for parent_id in parent_ids {
            add_has_child_edge(conn, parent_id, &fractal.id)?;
        }

        for context_id in context_ids {
            add_has_context_edge(conn, &fractal.id, context_id)?;
        }

        Ok(fractal)
    } else {
        Err(DataError::InvalidData(
            "Failed to create fractal".to_string(),
        ))
    }
}

fn add_has_child_edge(
    conn: &Connection,
    parent_id: &Uuid,
    child_id: &Uuid,
) -> Result<(), DataError> {
    let query = "
        MATCH (parent:Fractal {id: $parent_id}), (child:Fractal {id: $child_id})
        CREATE (parent)-[:HAS_CHILD]->(child)
    ";
    let mut stmt = conn.prepare(query)?;
    conn.execute(
        &mut stmt,
        vec![
            ("parent_id", Value::UUID(*parent_id)),
            ("child_id", Value::UUID(*child_id)),
        ],
    )?;
    Ok(())
}

fn add_has_context_edge(
    conn: &Connection,
    fractal_id: &Uuid,
    context_id: &Uuid,
) -> Result<(), DataError> {
    let query = "
        MATCH (f:Fractal {id: $fractal_id}), (c:Fractal {id: $context_id})
        CREATE (f)-[:HAS_CONTEXT]->(c)
    ";
    let mut stmt = conn.prepare(query)?;
    conn.execute(
        &mut stmt,
        vec![
            ("fractal_id", Value::UUID(*fractal_id)),
            ("context_id", Value::UUID(*context_id)),
        ],
    )?;
    Ok(())
}

pub fn get_fractal_by_name(conn: &Connection, name: &str) -> Result<Fractal, DataError> {
    let query = "
        MATCH (f:Fractal {name: $name})
        RETURN f.id, f.name, f.createdAt, f.updatedAt
    ";
    let mut stmt = conn.prepare(query)?;
    let params = vec![("name", Value::String(name.to_string()))];
    let mut result = conn.execute(&mut stmt, params)?;

    dbg!("get fractal by name {:?}", result.get_compiling_time());
    dbg!("get fractal by name {:?}", result.get_execution_time());

    if let Some(row) = result.next() {
        row_to_fractal(&row)
    } else {
        Err(DataError::FractalNotFound(name.to_string()))
    }
}

pub fn get_fractal_parents(conn: &Connection, id: &Uuid) -> Result<Vec<Fractal>, DataError> {
    let query = "
        MATCH (parent:Fractal)-[:HAS_CHILD]->(f:Fractal {id: $id})
        RETURN parent.id, parent.name, parent.createdAt, parent.updatedAt
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, vec![("id", Value::UUID(*id))])?;

    let mut parents = Vec::new();
    for row in result {
        parents.push(row_to_fractal(&row)?);
    }

    Ok(parents)
}

pub fn get_fractal_children(conn: &Connection, id: &Uuid) -> Result<Vec<Fractal>, DataError> {
    let query = "
        MATCH (f:Fractal {id: $id})-[:HAS_CHILD]->(child:Fractal)
        RETURN child.id, child.name, child.createdAt, child.updatedAt
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, vec![("id", Value::UUID(*id))])?;

    let mut children = Vec::new();
    for row in result {
        children.push(row_to_fractal(&row)?);
    }

    Ok(children)
}

pub fn get_fractal_contexts(conn: &Connection, id: &Uuid) -> Result<Vec<Fractal>, DataError> {
    let query = "
        MATCH (f:Fractal {id: $id})-[:HAS_CONTEXT]->(context:Fractal)
        RETURN context.id, context.name, context.createdAt, context.updatedAt
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, vec![("id", Value::UUID(*id))])?;

    let mut contexts = Vec::new();
    for row in result {
        contexts.push(row_to_fractal(&row)?);
    }

    Ok(contexts)
}

pub fn get_fractal_knowledge_with_context(
    conn: &Connection,
    fractal_name: &str,
    context_ids: &[Uuid],
) -> Result<Vec<Knowledge>, DataError> {
    let query = "
        MATCH (f:Fractal {name: $fractal_name})-[:HAS_KNOWLEDGE]->(k:Knowledge)
        WHERE ALL(contextId IN $context_ids WHERE (k)-[:IN_CONTEXT]->(:Fractal {id: contextId}))
        RETURN k.id, k.content
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(
        &mut stmt,
        vec![
            ("fractal_name", Value::String(fractal_name.to_string())),
            (
                "context_ids",
                Value::List(
                    LogicalType::List {
                        child_type: Box::new(LogicalType::UUID),
                    },
                    context_ids
                        .iter()
                        .map(|&id| Value::UUID(id))
                        .collect::<Vec<_>>(),
                ),
            ),
        ],
    )?;

    let mut knowledge = Vec::new();
    for row in result {
        knowledge.push(row_to_knowledge(&row)?);
    }

    Ok(knowledge)
}

fn row_to_knowledge(row: &[Value]) -> Result<Knowledge, DataError> {
    Ok(Knowledge {
        id: match &row[0] {
            Value::UUID(uuid) => *uuid,
            _ => return Err(DataError::InvalidData("Invalid UUID for id".to_string())),
        },
        content: match &row[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(DataError::InvalidData(
                    "Invalid String for content".to_string(),
                ))
            }
        },
    })
}

fn row_to_fractal(row: &[Value]) -> Result<Fractal, DataError> {
    Ok(Fractal {
        id: match &row[0] {
            Value::UUID(uuid) => *uuid,
            _ => return Err(DataError::InvalidData("Invalid UUID for id".to_string())),
        },
        name: match &row[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(DataError::InvalidData(
                    "Invalid String for name".to_string(),
                ))
            }
        },
        created_at: match &row[2] {
            Value::Timestamp(ts) => DateTime::<Utc>::from_timestamp(ts.unix_timestamp(), 0)
                .ok_or_else(|| {
                    DataError::InvalidData("Invalid Timestamp for createdAt".to_string())
                })?,
            _ => {
                return Err(DataError::InvalidData(
                    "Invalid Timestamp for createdAt".to_string(),
                ))
            }
        },
        updated_at: match &row[3] {
            Value::Timestamp(ts) => DateTime::<Utc>::from_timestamp(ts.unix_timestamp(), 0)
                .ok_or_else(|| {
                    DataError::InvalidData("Invalid Timestamp for updatedAt".to_string())
                })?,
            _ => {
                return Err(DataError::InvalidData(
                    "Invalid Timestamp for updatedAt".to_string(),
                ))
            }
        },
    })
}
