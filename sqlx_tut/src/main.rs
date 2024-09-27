use sqlx::sqlite::SqlitePoolOptions;
use sqlx::FromRow;
use tokio;

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool to the SQLite database.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:my_database.db")
        .await?;

    // Insert a new user into the database.
    let user_name = "Alice";
    let user_email = "alice@example.com";
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(user_name)
        .bind(user_email)
        .execute(&pool)
        .await?;

    // Fetch all users from the database.
    let rows: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;

    for user in rows {
        println!("{:?}", user);
    }

    Ok(())
}
