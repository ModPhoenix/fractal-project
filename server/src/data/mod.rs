use chrono::{DateTime, Utc};
use kuzu::{Connection, Database, Error as KuzuError, LogicalType, SystemConfig, Value};
use std::time::SystemTime;
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

pub const FRACTAL_ROOT_ID: Uuid = Uuid::nil();

pub fn create_db(_db_path: &str) -> Result<Database, DataError> {
    Database::new("", SystemConfig::default()).map_err(DataError::from)
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
        "CREATE REL TABLE IF NOT EXISTS HAS_CHILD (
            FROM Fractal
            TO Fractal,
            context_id UUID
        )",
        "CREATE REL TABLE IF NOT EXISTS HAS_CONTEXT(FROM Fractal TO Fractal)",
        "CREATE REL TABLE IF NOT EXISTS HAS_KNOWLEDGE(FROM Fractal TO Knowledge)",
        "CREATE REL TABLE IF NOT EXISTS IN_CONTEXT(FROM Knowledge TO Fractal)",
    ];

    for query in create_tables.iter() {
        conn.query(query)?;
    }

    println!("Database tables created.");

    println!("Database initialization completed.");
    Ok(())
}

pub fn setup_example_graph(conn: &Connection) -> Result<(), DataError> {
    // Create Fractal nodes
    let programming = create_fractal(conn, "Programming", Some(&FRACTAL_ROOT_ID), None)?;
    let python = create_fractal(
        conn,
        "Python",
        Some(&programming.id),
        Some(&FRACTAL_ROOT_ID),
    )?;
    let _c_lang = create_fractal(conn, "C", Some(&programming.id), Some(&FRACTAL_ROOT_ID))?;
    let rust = create_fractal(conn, "Rust", Some(&programming.id), Some(&FRACTAL_ROOT_ID))?;
    let string = create_fractal(
        conn,
        "String",
        Some(&programming.id),
        Some(&FRACTAL_ROOT_ID),
    )?;

    add_has_child_edge(conn, &python.id, &string.id, Some(&programming.id))?;
    add_has_child_edge(conn, &rust.id, &string.id, Some(&programming.id))?;

    // Create specific child relationships with contexts
    // Python -> String -> .count()
    let _count_method = create_fractal(conn, ".count()", Some(&string.id), Some(&python.id))?;

    // Programming -> String -> String literal
    let _string_literal = create_fractal(
        conn,
        "String literal",
        Some(&string.id),
        Some(&programming.id),
    )?;

    // Rust -> String -> &str
    let _amp_str = create_fractal(conn, "&str", Some(&string.id), Some(&rust.id))?;

    // C -> String has no children or could have specific children if needed
    println!("Root fractal created.");
    Ok(())
}

pub fn create_fractal_raw(
    conn: &Connection,
    name: &str,
    parent_id: Option<&Uuid>,
    context_id: Option<&Uuid>,
    uuid: Option<Uuid>,
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

    let id = uuid.unwrap_or_else(Uuid::new_v4);
    let system_time = SystemTime::now();
    let datetime = OffsetDateTime::from(system_time);

    let params = vec![
        ("uuid", Value::UUID(id)),
        ("name", Value::String(name.to_string())),
        ("datetime", Value::Timestamp(datetime)),
    ];

    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

    let fractal = result
        .into_iter()
        .next()
        .ok_or_else(|| DataError::InvalidData("Failed to create fractal".to_string()))
        .and_then(|row| row_to_fractal(&row))?;

    if let Some(parent_id) = parent_id {
        add_has_child_edge(conn, parent_id, &fractal.id, context_id)?;
    }

    Ok(fractal)
}

pub fn create_fractal(
    conn: &Connection,
    name: &str,
    parent_id: Option<&Uuid>,
    context_id: Option<&Uuid>,
) -> Result<Fractal, DataError> {
    create_fractal_raw(conn, name, parent_id, context_id, None)
}

pub fn add_has_child_edge(
    conn: &Connection,
    parent_id: &Uuid,
    child_id: &Uuid,
    context_id: Option<&Uuid>,
) -> Result<(), DataError> {
    println!("Adding has_child edge");
    let query = "
        MATCH (parent:Fractal {id: $parent_id}), (child:Fractal {id: $child_id})
        CREATE (parent)-[:HAS_CHILD {
            context_id: $context_id
        }]->(child)
        ";
    let context_value = match context_id {
        Some(id) => Value::UUID(*id),
        None => Value::Null(LogicalType::UUID),
    };
    let params = vec![
        ("parent_id", Value::UUID(*parent_id)),
        ("child_id", Value::UUID(*child_id)),
        ("context_id", context_value),
    ];
    let mut stmt = conn.prepare(query)?;
    conn.execute(&mut stmt, params)?;

    Ok(())
}

pub fn add_has_context_edge(
    conn: &Connection,
    fractal_id: &Uuid,
    context_id: &Uuid,
) -> Result<(), DataError> {
    let query = "
        MATCH (f:Fractal {id: $fractal_id}), (c:Fractal {id: $context_id})
        CREATE (f)-[:HAS_CONTEXT]->(c)
    ";
    let params = vec![
        ("fractal_id", Value::UUID(*fractal_id)),
        ("context_id", Value::UUID(*context_id)),
    ];
    let mut stmt = conn.prepare(query)?;
    conn.execute(&mut stmt, params)?;
    Ok(())
}

pub fn get_fractal_by_name(conn: &Connection, name: &str) -> Result<Fractal, DataError> {
    let query = "
        MATCH (f:Fractal {name: $name})
        RETURN f.id, f.name, f.createdAt, f.updatedAt
    ";
    let params = vec![("name", Value::String(name.to_string()))];
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

    result
        .into_iter()
        .next()
        .ok_or_else(|| DataError::FractalNotFound(name.to_string()))
        .and_then(|row| row_to_fractal(&row))
}

pub fn get_children_of_fractal_with_context(
    conn: &Connection,
    fractal_id: &Uuid,
    context_id: Option<&Uuid>,
) -> Result<Vec<Fractal>, DataError> {
    let query = match context_id {
        Some(_) => {
            "
            MATCH (f:Fractal {id: $id})-[:HAS_CHILD {context_id: $context_id}]->(child:Fractal)
            RETURN child.id, child.name, child.createdAt, child.updatedAt
        "
        }
        None => {
            "
            MATCH (f:Fractal {id: $id})-[:HAS_CHILD]->(child:Fractal)
            RETURN child.id, child.name, child.createdAt, child.updatedAt
        "
        }
    };

    let params = match context_id {
        Some(id) => vec![
            ("id", Value::UUID(*fractal_id)),
            ("context_id", Value::UUID(*id)),
        ],
        None => vec![("id", Value::UUID(*fractal_id))],
    };

    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

    result.into_iter().map(|row| row_to_fractal(&row)).collect()
}

pub fn get_fractal_relations(
    conn: &Connection,
    id: &Uuid,
    relation: &str,
) -> Result<Vec<Fractal>, DataError> {
    let query = match relation {
        "parents" => "MATCH (parent:Fractal)-[:HAS_CHILD]->(f:Fractal {id: $id}) RETURN parent",
        "children" => "MATCH (f:Fractal {id: $id})-[:HAS_CHILD]->(child:Fractal) RETURN child",
        "contexts" => {
            "MATCH (f:Fractal {id: $id})-[:HAS_CONTEXT]->(context:Fractal) RETURN context"
        }
        _ => {
            return Err(DataError::InvalidData(format!(
                "Invalid relation '{}'",
                relation
            )))
        }
    };

    let params = vec![("id", Value::UUID(*id))];
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

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
    let context_ids_value = Value::List(
        LogicalType::List {
            child_type: Box::new(LogicalType::UUID),
        },
        context_ids.iter().map(|&id| Value::UUID(id)).collect(),
    );
    let params = vec![
        ("fractal_name", Value::String(fractal_name.to_string())),
        ("context_ids", context_ids_value),
    ];
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

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
    if let Value::Node(node_val) = &row[0] {
        let properties = node_val.get_properties();

        let get_property = |name: &str| -> Result<&Value, DataError> {
            properties
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value)
                .ok_or_else(|| DataError::InvalidData(format!("Missing '{}' property", name)))
        };

        Ok(Fractal {
            id: extract_uuid(get_property("id")?, "id")?,
            name: extract_string(get_property("name")?, "name")?,
            created_at: extract_datetime(get_property("createdAt")?, "createdAt")?,
            updated_at: extract_datetime(get_property("updatedAt")?, "updatedAt")?,
        })
    } else {
        // If the row is not a Node, we assume it contains the values directly
        Ok(Fractal {
            id: extract_uuid(&row[0], "id")?,
            name: extract_string(&row[1], "name")?,
            created_at: extract_datetime(&row[2], "createdAt")?,
            updated_at: extract_datetime(&row[3], "updatedAt")?,
        })
    }
}

fn extract_uuid(value: &Value, field: &str) -> Result<Uuid, DataError> {
    match value {
        Value::UUID(uuid) => Ok(*uuid),
        _ => Err(DataError::InvalidData(format!(
            "Expected UUID for '{}', found {:?}",
            field, value
        ))),
    }
}

fn extract_string(value: &Value, field: &str) -> Result<String, DataError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => Err(DataError::InvalidData(format!(
            "Expected String for '{}', found {:?}",
            field, value
        ))),
    }
}

fn extract_datetime(value: &Value, field: &str) -> Result<DateTime<Utc>, DataError> {
    match value {
        Value::Timestamp(ts) => {
            let system_time: SystemTime = (*ts).into();
            let datetime = DateTime::<Utc>::from(system_time);
            Ok(datetime)
        }
        _ => Err(DataError::InvalidData(format!(
            "Expected Timestamp for '{}', found {:?}",
            field, value
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
    let params = vec![("id", Value::UUID(*id))];
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

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
        CREATE (k:Knowledge {
            id: $knowledge_id,
            content: $content,
            createdAt: $datetime,
            updatedAt: $datetime
        })
        CREATE (f)-[:HAS_KNOWLEDGE]->(k)
        WITH k
        UNWIND $context_ids AS context_id
        MATCH (c:Fractal {id: context_id})
        CREATE (k)-[:IN_CONTEXT]->(c)
        RETURN k.id, k.content
    ";

    let system_time = std::time::SystemTime::now();
    let datetime = OffsetDateTime::from(system_time);

    let context_ids_value = Value::List(
        LogicalType::List {
            child_type: Box::new(LogicalType::UUID),
        },
        context_ids.iter().map(|&id| Value::UUID(id)).collect(),
    );

    let params = vec![
        ("fractal_id", Value::UUID(*fractal_id)),
        ("knowledge_id", Value::UUID(Uuid::new_v4())),
        ("content", Value::String(content.to_string())),
        ("datetime", Value::Timestamp(datetime)),
        ("context_ids", context_ids_value),
    ];
    let mut stmt = conn.prepare(query)?;
    let result = conn.execute(&mut stmt, params)?;

    result
        .into_iter()
        .next()
        .ok_or_else(|| DataError::InvalidData("Failed to create knowledge".to_string()))
        .and_then(|row| row_to_knowledge(&row))
}
