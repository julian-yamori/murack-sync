use sqlx::{PgPool, Row};

pub async fn connect_db_pool(database_url: &str) -> sqlx::Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;

    // 接続に問題があれば早期終了する
    test_connection(&pool).await?;

    Ok(pool)
}

/// アプリ起動時の疎通確認
async fn test_connection(pool: &PgPool) -> sqlx::Result<()> {
    let row = sqlx::query("SELECT 1 as test").fetch_one(pool).await?;

    let test_value: i32 = row.get("test");
    println!("Database connection test: {test_value}");

    Ok(())
}
