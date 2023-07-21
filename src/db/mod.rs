use std::env;
const DB_NAME: &str = "stackedfour";

mod migrator;
pub mod entities;

use sea_orm::{ConnectionTrait, Database, DbBackend, DatabaseConnection, DbErr, Statement};
use sea_orm_migration::{SchemaManager, MigratorTrait};
use sea_orm::*;

use entities::{prelude::*, *};

#[derive(Debug, Clone)]
pub struct Db {
    url: String,
    conn: DatabaseConnection,
}

impl Db {
    pub async fn db_init() -> Result<Db, DbErr> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable should be set");

        let db = Database::connect(&database_url).await?;
        let db = match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
                )).await?;

                let url = format!("{}/{}", database_url, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
                )).await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", DB_NAME),
                )).await?;

                let url = format!("{}/{}", database_url, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        };

        let schema_manager = SchemaManager::new(&db);

        migrator::Migrator::refresh(&db).await?;
        // Let's make sure the db is ready
        assert!(schema_manager.has_table("player").await?);
        assert!(schema_manager.has_table("game").await?);

        let db = Db { 
            url: database_url.clone(),
            conn: db
        };
        // Not sure, maybe get rid of this param
        assert!(db.url == database_url);
        Ok(db)
    }

    pub async fn get_player(&self, username: String) -> player::Model {
        let player = Player::find()
            .filter(player::Column::Name.eq(&username))
            .one(&self.conn)
            .await
            .unwrap();
        match player {
            Some(player) => player,
            // Create the player if it doesn't exist
            None => player::ActiveModel {
                name: ActiveValue::Set(username.to_owned()),
                ..Default::default()
            }.insert(&self.conn).await.unwrap()
        }
    }
}
