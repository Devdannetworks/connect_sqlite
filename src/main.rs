use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

async fn create_schema(db_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            settings_id INTEGER PRIMARY KEY NOT NULL,
            description TEXT NOT NULL,
            created_on DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_on DATETIME DEFAULT (datetime('now', 'localtime')),
            done BOOLEAN NOT NULL DEFAULT 0
         )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS project (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            created_at DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_at DATETIME DEFAULT (datetime('now', 'localtime')),
            img_directory TEXT NOT NULL,
            out_directory TEXT NOT NULL,
            status TEXT NOT NULL,
            settings_id INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (settings_id) REFERENCES settings (settings_id) ON UPDATE NO ACTION ON DELETE SET NULL
         )"
    ).execute(&pool).await?;
    
    pool.close().await;
    Ok(())
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite:sqlite.db");

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(e) => eprintln!("Error creating the database, {:?}", e),
        }
    }

    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES (?)";
    let result = sqlx::query(qry).bind("testing").execute(&instances).await;

    instances.close().await;
    println!("Result: {:?}", result);
}
