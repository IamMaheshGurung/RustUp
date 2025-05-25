use std::env;
use sqlx::PgPool;
use dotenvy::dotenv;


#[tokio::main]
async fn main() -> Result <(), sqlx::Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _pool = PgPool::connect(&database_url).await?;

    
    println!("Connection successful");
    
    Ok(())
    
}
