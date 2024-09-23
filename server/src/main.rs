use server::data::{
    create_db, create_fractal_raw, init_database, setup_example_graph, DataError, FRACTAL_ROOT_ID,
};
use server::run;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    // Create and initialize the database
    let db =
        create_db("./demo_db").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Create a new scope for database initialization
    {
        let conn = server::data::create_connection(&db)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        init_database(&conn).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        match create_fractal_raw(&conn, "Root", None, None, Some(FRACTAL_ROOT_ID)) {
            Ok(_) => setup_example_graph(&conn),
            Err(DataError::FractalAlreadyExists(_)) => {
                println!("Root fractal already exists.");
                Ok(())
            }
            Err(e) => Err(e),
        }
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
    } // conn is dropped here

    run(listener, db)?.await
}
