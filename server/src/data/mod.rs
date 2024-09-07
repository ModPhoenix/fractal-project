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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub id: Uuid,
    pub content: String,
}

pub fn create_db(db_path: &str) -> Result<Database, DataError> {
    Database::new(db_path, SystemConfig::default()).map_err(DataError::from)
}

pub fn create_connection(db: &Database) -> Result<Connection, DataError> {
    Connection::new(db).map_err(DataError::from)
}

pub fn init_database(conn: &Connection) -> Result<(), DataError> {
    println!("Initializing database...");

    let create_tables = [
        "CREATE NODE TABLE IF NOT EXISTS Fractal (
            id UUID,
            name STRING,
            createdAt TIMESTAMP,
            updatedAt TIMESTAMP,
            PRIMARY KEY (id)
        )",
        "CREATE NODE TABLE IF NOT EXISTS Knowledge (
            id UUID,
            content STRING,
            createdAt TIMESTAMP,
            updatedAt TIMESTAMP,
            PRIMARY KEY (id)
        )",
        "CREATE REL TABLE IF NOT EXISTS HAS_CHILD(FROM Fractal TO Fractal)",
        "CREATE REL TABLE IF NOT EXISTS HAS_CONTEXT(FROM Fractal TO Fractal)",
        "CREATE REL TABLE IF NOT EXISTS HAS_KNOWLEDGE(FROM Fractal TO Knowledge)",
        "CREATE REL TABLE IF NOT EXISTS IN_CONTEXT(FROM Knowledge TO Fractal)",
    ];

    for query in create_tables.iter() {
        conn.query(query)?;
    }

    create_root_fractal(conn)?;

    println!("Database initialization completed.");
    Ok(())
}

fn create_root_fractal(conn: &Connection) -> Result<(), DataError> {
    match create_fractal(conn, "Root", &[], &[]) {
        Ok(root) => {
            let _ = create_fractal(conn, "Child1", &[root.id], &[root.id]);
            println!("Root fractal created.");
            Ok(())
        }
        Err(DataError::FractalAlreadyExists(_)) => {
            println!("Root fractal already exists.");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn create_fractal(
    conn: &Connection,
    name: &str,
    parent_ids: &[Uuid],
    context_ids: &[Uuid],
) -> Result<Fractal, DataError> {
    if get_fractal_by_name(conn, name).is_ok() {
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
    let result = conn.execute(
        &mut stmt,
        vec![
            ("uuid", Value::UUID(Uuid::new_v4())),
            ("name", Value::String(name.to_string())),
            ("datetime", Value::Timestamp(OffsetDateTime::now_utc())),
        ],
    )?;

    let fractal = result
        .into_iter()
        .next()
        .ok_or_else(|| DataError::InvalidData("Failed to create fractal".to_string()))
        .and_then(|row| row_to_fractal(&row))?;

    parent_ids
        .iter()
        .try_for_each(|parent_id| add_has_child_edge(conn, parent_id, &fractal.id))?;

    context_ids
        .iter()
        .try_for_each(|context_id| add_has_context_edge(conn, &fractal.id, context_id))?;

    Ok(fractal)
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

    result
        .next()
        .ok_or_else(|| DataError::FractalNotFound(name.to_string()))
        .and_then(|row| row_to_fractal(&row))
}

pub fn get_fractal_relations(
    conn: &Connection,
    id: &Uuid,
    relation: &str,
) -> Result<Vec<Fractal>, DataError> {
    let query = match relation {
        "parents" => "MATCH (parent:Fractal)-[:HAS_CHILD]->(Fractal {id: $id}) RETURN parent",
        "children" => "MATCH (Fractal {id: $id})-[:HAS_CHILD]->(child:Fractal) RETURN child",
        "contexts" => "MATCH (Fractal {id: $id})-[:HAS_CONTEXT]->(context:Fractal) RETURN context",
        _ => return Err(DataError::InvalidData("Invalid relation".to_string())),
    };

    let mut stmt = conn.prepare(&query)?;
    let result = conn.execute(&mut stmt, vec![("id", Value::UUID(*id))])?;

    dbg!(relation);

    result.into_iter().map(|row| row_to_fractal(&row)).collect()
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
                    context_ids.iter().map(|&id| Value::UUID(id)).collect(),
                ),
            ),
        ],
    )?;

    result
        .into_iter()
        .map(|row| row_to_knowledge(&row))
        .collect()
}

fn row_to_knowledge(row: &[Value]) -> Result<Knowledge, DataError> {
    Ok(Knowledge {
        id: extract_uuid(&row[0], "id")?,
        content: extract_string(&row[1], "content")?,
    })
}

fn row_to_fractal(row: &[Value]) -> Result<Fractal, DataError> {
    dbg!(row);

    if let Value::Node(node_val) = &row[0] {
        let properties = node_val.get_properties();

        let get_property = |name: &str| -> Result<&Value, DataError> {
            properties
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value)
                .ok_or_else(|| DataError::InvalidData(format!("Missing {}", name)))
        };

        Ok(Fractal {
            id: extract_uuid(get_property("id")?, "id")?,
            name: extract_string(get_property("name")?, "name")?,
            created_at: extract_datetime(get_property("createdAt")?, "createdAt")?,
            updated_at: extract_datetime(get_property("updatedAt")?, "updatedAt")?,
        })
    } else {
        let id = extract_uuid(&row[0], "id")?;
        let name = extract_string(&row[1], "name")?;
        let created_at = extract_datetime(&row[2], "createdAt")?;
        let updated_at = extract_datetime(&row[3], "updatedAt")?;

        Ok(Fractal {
            id,
            name,
            created_at,
            updated_at,
        })
    }
}

fn extract_uuid(value: &Value, field: &str) -> Result<Uuid, DataError> {
    match value {
        Value::UUID(uuid) => Ok(*uuid),
        _ => Err(DataError::InvalidData(format!(
            "Invalid UUID for {}",
            field
        ))),
    }
}

fn extract_string(value: &Value, field: &str) -> Result<String, DataError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => Err(DataError::InvalidData(format!(
            "Invalid String for {}",
            field
        ))),
    }
}

fn extract_datetime(value: &Value, field: &str) -> Result<DateTime<Utc>, DataError> {
    match value {
        Value::Timestamp(ts) => DateTime::<Utc>::from_timestamp(ts.unix_timestamp(), 0)
            .ok_or_else(|| DataError::InvalidData(format!("Invalid Timestamp for {}", field))),
        _ => Err(DataError::InvalidData(format!(
            "Invalid Timestamp for {}",
            field
        ))),
    }
}

pub fn get_root_fractal(conn: &Connection) -> Result<Fractal, DataError> {
    get_fractal_by_name(conn, "Root")
}

pub fn delete_fractal(conn: &Connection, id: &Uuid) -> Result<bool, DataError> {
    let query = "
        MATCH (f:Fractal {id: $id})
        DETACH DELETE f
        RETURN count(f) > 0 as deleted
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, vec![("id", Value::UUID(*id))])?;

    result
        .into_iter()
        .next()
        .and_then(|row| match &row[0] {
            Value::Bool(b) => Some(*b),
            _ => None,
        })
        .ok_or_else(|| DataError::InvalidData("Failed to delete fractal".to_string()))
}

pub fn add_knowledge(
    conn: &Connection,
    fractal_id: &Uuid,
    content: &str,
    context_ids: &[Uuid],
) -> Result<Knowledge, DataError> {
    let query = "
        MATCH (f:Fractal {id: $fractal_id})
        CREATE (k:Knowledge {id: $knowledge_id, content: $content, createdAt: $datetime, updatedAt: $datetime})
        CREATE (f)-[:HAS_KNOWLEDGE]->(k)
        WITH k
        UNWIND $context_ids as context_id
        MATCH (c:Fractal {id: context_id})
        CREATE (k)-[:IN_CONTEXT]->(c)
        RETURN k.id, k.content
    ";
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(
        &mut stmt,
        vec![
            ("fractal_id", Value::UUID(*fractal_id)),
            ("knowledge_id", Value::UUID(Uuid::new_v4())),
            ("content", Value::String(content.to_string())),
            ("datetime", Value::Timestamp(OffsetDateTime::now_utc())),
            (
                "context_ids",
                Value::List(
                    LogicalType::List {
                        child_type: Box::new(LogicalType::UUID),
                    },
                    context_ids.iter().map(|&id| Value::UUID(id)).collect(),
                ),
            ),
        ],
    )?;

    result
        .into_iter()
        .next()
        .ok_or_else(|| DataError::InvalidData("Failed to create knowledge".to_string()))
        .and_then(|row| row_to_knowledge(&row))
}
