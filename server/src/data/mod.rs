use chrono::{DateTime, Utc};
use kuzu::{Connection, Database, Error as KuzuError, PreparedStatement, SystemConfig, Value};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

// Custom error type
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Database error: {0}")]
    Database(#[from] KuzuError),
    #[error("Fractal not found: {0}")]
    FractalNotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

// Fractal struct
#[derive(Debug, Clone)]
pub struct Fractal {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Description struct
#[derive(Debug, Clone)]
pub struct Description {
    pub id: Uuid,
    pub content: String,
    pub context_path: Vec<Uuid>,
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

pub fn create_fractal(
    db: &Database,
    name: &str,
    parent_id: Option<&Uuid>,
) -> Result<Fractal, DataError> {
    let conn = create_connection(db)?;
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

    if let Some(row) = result.into_iter().next() {
        let fractal = row_to_fractal(&row)?;

        // if let Some(parent_id) = parent_id {
        //     add_parent_relation(conn, &fractal.id, parent_id).await?;
        // }

        Ok(fractal)
    } else {
        Err(DataError::InvalidData(
            "Failed to create fractal".to_string(),
        ))
    }
}

pub fn get_fractal_by_name(db: &Database, name: &str) -> Result<Fractal, DataError> {
    let conn = create_connection(db)?;
    let query = "SELECT id, name, createdAt, updatedAt FROM Fractal WHERE name = $name";
    let mut stmt = conn.prepare(query)?;
    let mut result = conn.execute(&mut stmt, vec![("name", Value::String(name.to_string()))])?;

    if let Some(row) = result.next() {
        row_to_fractal(&row)
    } else {
        Err(DataError::FractalNotFound(name.to_string()))
    }
}

// pub async fn get_fractal_children(
//     conn: &DbConnection,
//     parent_id: &UUID,
// ) -> Result<Vec<Fractal>, DataError> {
//     let query = "
//         SELECT f.id, f.name, f.createdAt, f.updatedAt
//         FROM Fractal f
//         JOIN FractalParent fp ON f.id = fp.FROM
//         WHERE fp.TO = $1
//         ORDER BY fp.order
//     ";
//     let mut stmt = conn.prepare().await?;
//     stmt.bind_parameters(&[Value::UUID(*parent_id)])?;
//     let result = execute_query(conn, &mut stmt).await?;

//     result.into_iter().map(|row| row_to_fractal(&row)).collect()
// }

// pub async fn get_fractal_parents(
//     conn: &DbConnection,
//     child_id: &UUID,
// ) -> Result<Vec<Fractal>, DataError> {
//     let query = "
//         SELECT f.id, f.name, f.createdAt, f.updatedAt
//         FROM Fractal f
//         JOIN FractalParent fp ON f.id = fp.TO
//         WHERE fp.FROM = $1
//     ";
//     let mut stmt = prepare_statement(conn, query).await?;
//     stmt.bind_parameters(&[Value::UUID(*child_id)])?;
//     let result = execute_query(conn, &mut stmt).await?;

//     result.into_iter().map(|row| row_to_fractal(&row)).collect()
// }

// pub async fn get_description(
//     conn: &DbConnection,
//     fractal_id: &UUID,
//     context_pattern: &[String],
// ) -> Result<Description, DataError> {
//     let query = "
//         SELECT d.id, d.content, fd.contextPath
//         FROM Fractal f
//         JOIN FractalDescription fd ON f.id = fd.FROM
//         JOIN Description d ON fd.TO = d.id
//         WHERE f.id = $1 AND fd.contextPath[0:$2] = $3
//         LIMIT 1
//     ";
//     let mut stmt = prepare_statement(conn, query).await?;
//     stmt.bind_parameters(&[
//         Value::UUID(*fractal_id),
//         Value::Int64(context_pattern.len() as i64),
//         Value::List(
//             context_pattern
//                 .iter()
//                 .map(|s| Value::String(s.clone()))
//                 .collect(),
//         ),
//     ])?;
//     let result = execute_query(conn, &mut stmt).await?;

//     if let Some(row) = result.into_iter().next() {
//         row_to_description(&row)
//     } else {
//         Err(DataError::FractalNotFound(format!(
//             "Description for fractal {} with given context not found",
//             fractal_id
//         )))
//     }
// }

// async fn add_parent_relation(
//     conn: &DbConnection,
//     child_id: &UUID,
//     parent_id: &UUID,
// ) -> Result<(), DataError> {
//     let query = "
//         INSERT INTO FractalParent (FROM, TO, order)
//         VALUES ($1, $2, (SELECT COALESCE(MAX(order), 0) + 1 FROM FractalParent WHERE TO = $2))
//     ";
//     let mut stmt = prepare_statement(conn, query).await?;
//     stmt.bind_parameters(&[Value::UUID(*child_id), Value::UUID(*parent_id)])?;
//     execute_query(conn, &mut stmt).await?;
//     Ok(())
// }

// async fn prepare_statement(
//     conn: &DbConnection,
//     query: &str,
// ) -> Result<PreparedStatement, DataError> {
//     let conn = conn.lock().await;
//     Ok(conn.prepare(query)?)
// }

// async fn execute_query(
//     conn: &DbConnection,
//     stmt: &mut PreparedStatement,
// ) -> Result<Vec<HashMap<String, Value>>, DataError> {
//     let conn = conn.lock().await;
//     let result = conn.execute(stmt)?;
//     Ok(result.get_rows()?)
// }

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

// fn row_to_description(row: &HashMap<String, Value>) -> Result<Description, DataError> {
//     Ok(Description {
//         id: get_uuid(row, "id")?,
//         content: get_string(row, "content")?,
//         context_path: get_uuid_list(row, "contextPath")?,
//     })
// }

fn get_uuid(row: &HashMap<String, Value>, key: &str) -> Result<Uuid, DataError> {
    match row.get(key) {
        Some(Value::UUID(uuid)) => Ok(*uuid),
        _ => Err(DataError::InvalidData(format!(
            "Invalid UUID for key: {}",
            key
        ))),
    }
}

fn get_string(row: &HashMap<String, Value>, key: &str) -> Result<String, DataError> {
    match row.get(key) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(DataError::InvalidData(format!(
            "Invalid String for key: {}",
            key
        ))),
    }
}

fn get_timestamp(
    row: &HashMap<String, Value>,
    key: &str,
) -> Result<chrono::DateTime<chrono::Utc>, DataError> {
    match row.get(key) {
        Some(Value::Timestamp(ts)) => DateTime::<Utc>::from_timestamp(ts.unix_timestamp(), 0)
            .ok_or_else(|| DataError::InvalidData(format!("Invalid Timestamp for key: {}", key))),
        _ => Err(DataError::InvalidData(format!(
            "Invalid Timestamp for key: {}",
            key
        ))),
    }
}

// fn get_uuid_list(row: &HashMap<String, Value>, key: &str) -> Result<Vec<Uuid>, DataError> {
//     match row.get(key) {
//         Some(Value::List(list)) => list
//             .iter()
//             .map(|v| {
//                 if let Value::UUID(uuid) = v {
//                     Ok(*uuid)
//                 } else {
//                     Err(DataError::InvalidData("Invalid UUID in list".to_string()))
//                 }
//             })
//             .collect(),
//         _ => Err(DataError::InvalidData(format!(
//             "Invalid UUID list for key: {}",
//             key
//         ))),
//     }
// }
