mod models;

use sqlx::{mysql::MySqlPoolOptions};
use models::Todo;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect("mysql://root:root@localhost/todos").await?;

    Todo::new("Poggers".to_string(), false).save(&pool).await?;

    let todos = Todo::all(&pool).await?;

    println!("{:?}", todos);

    Ok(())
}
