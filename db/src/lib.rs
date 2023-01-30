pub mod entities;
pub mod configuration;
use configuration::Configuration;
use sea_orm::*;

pub async fn set_up_db(config: &Configuration) -> Result<DatabaseConnection, DbErr> {
    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database,
    );

    let db = Database::connect(db_url).await?;
    Ok(db)
}
