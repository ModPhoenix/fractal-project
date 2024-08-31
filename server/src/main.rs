use server::data::{
    create_connection, create_db, create_fractal, get_fractal_by_name, get_fractal_children,
    init_database, DataError,
};
use server::run;

fn setup_database_and_query() -> Result<(), DataError> {
    let db = create_db("./demo_db")?;
    let conn = create_connection(&db)?;
    init_database(&conn)?;

    let root = create_fractal(&conn, "Root", None)?;

    dbg!("{:?}", &root);

    let child1 = create_fractal(&conn, "Child1", Some(&root.id))?;

    dbg!("{:?}", &child1);

    let query = get_fractal_by_name(&conn, "Root")?;

    dbg!("{:?}", &query);

    let fractal_children = get_fractal_children(&db, &query.id)?;

    dbg!("{:?}", &fractal_children);
    let child_fractals = fractal_children;
    let children = child_fractals.len();
    dbg!("Number of child fractals: {}", children);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    // Call the function to set up the database and run a query
    setup_database_and_query().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    run(listener)?.await
}
