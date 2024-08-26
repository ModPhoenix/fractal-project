use server::data::{create_connection, create_db, create_fractal, DataError};
use server::run;

fn setup_database_and_query() -> Result<(), DataError> {
    // Create an empty on-disk database and connect to it
    // let db = Database::new("./demo_db", SystemConfig::default())?;
    // let conn = Connection::new(&db)?;

    let db = create_db("./demo_db")?;
    // let conn = create_connection(&db)?;

    // Create the tables
    //     conn.query(
    //         "CREATE NODE TABLE Fractal (
    //     id UUID,
    //     name STRING,
    //     createdAt TIMESTAMP,
    //     updatedAt TIMESTAMP,
    //     PRIMARY KEY (id)
    // );
    // ",
    //     )?;

    let fractal = create_fractal(&db, "Root", None)?;

    dbg!("{:?}", fractal);
    // conn.query("CREATE NODE TABLE User(name STRING, age INT64, PRIMARY KEY (name))")?;
    // conn.query("CREATE NODE TABLE City(name STRING, population INT64, PRIMARY KEY (name))")?;
    // conn.query("CREATE REL TABLE Follows(FROM User TO User, since INT64)")?;
    // conn.query("CREATE REL TABLE LivesIn(FROM User TO City)")?;

    // Load the data
    // conn.query("COPY User FROM './data/user.csv'")?;
    // conn.query("COPY City FROM './data/city.csv'")?;
    // conn.query("COPY Follows FROM './data/follows.csv'")?;
    // conn.query("COPY LivesIn FROM './data/lives-in.csv'")?;

    // let query_result =
    //     conn.query("MATCH (a:User)-[f:Follows]->(b:User) RETURN a.name, f.since, b.name;")?;

    // println!("{:?}", query_result.get_column_data_types());

    // // Print the rows
    // for row in query_result {
    //     println!("{}, {}, {}", row[0], row[1], row[2]);
    // }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    // Call the function to set up the database and run a query
    setup_database_and_query().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    run(listener)?.await
}
