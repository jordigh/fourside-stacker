use std::env;
const DB_NAME: &str = "stackedfour";

pub mod entities;
mod migrator;

use sea_orm::*;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use sea_orm_migration::{MigratorTrait, SchemaManager};

use entities::{prelude::*, *};

use crate::{game::Square, GAME_SIZE};

#[derive(Debug, Clone)]
pub struct Db {
    url: String,
    conn: DatabaseConnection,
}

impl Db {
    pub async fn db_init() -> Result<Db, DbErr> {
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable should be set");

        let db = Database::connect(&database_url).await?;
        let db = match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
                ))
                .await?;

                let url = format!("{}/{}", database_url, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", DB_NAME),
                ))
                .await?;

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
            conn: db,
        };
        // Not sure, maybe get rid of this param
        assert!(db.url == database_url);
        Ok(db)
    }

    pub async fn get_player(&self, username: &String) -> player::Model {
        let player = Player::find()
            .filter(player::Column::Name.eq(username))
            .one(&self.conn)
            .await
            .unwrap();
        match player {
            Some(player) => player,
            // Create the player if it doesn't exist
            None => player::ActiveModel {
                name: ActiveValue::Set(username.to_owned()),
                ..Default::default()
            }
            .insert(&self.conn)
            .await
            .unwrap(),
        }
    }

    pub async fn get_game(&self, player_id: i32) -> game::Model {
        // Does a game exist where the player is player red or black? Then return it.
        let game = Game::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add(game::Column::PlayerRedId.eq(player_id))
                            .add(game::Column::PlayerBlackId.eq(player_id)),
                    )
                    .add(game::Column::Finished.eq(false)),
            )
            .one(&self.conn)
            .await
            .unwrap();
        if let Some(game) = game {
            return game;
        }

        // If not, is there a game waiting for a player? Add this player to player black and return that game.
        let game = Game::find()
            .filter(
                Condition::all()
                    .add(game::Column::PlayerRedId.is_null().not())
                    .add(game::Column::PlayerBlackId.is_null()),
            )
            .one(&self.conn)
            .await
            .unwrap();
        if let Some(game) = game {
            let mut game: game::ActiveModel = game.into();
            game.player_black_id = ActiveValue::Set(Some(player_id));
            let game: game::Model = game.update(&self.conn).await.unwrap();
            return game;
        }

        // Else, start a new game and assign the player to player red.
        let squares: Vec<Vec<Option<Square>>> = vec![vec![None; GAME_SIZE]; GAME_SIZE];
        game::ActiveModel {
            squares: ActiveValue::Set(serde_json::to_value(&squares).unwrap()),
            player_red_id: ActiveValue::Set(Some(player_id)),
            ..Default::default()
        }
        .insert(&self.conn)
        .await
        .unwrap()
    }

    pub async fn save_game(&self, game: game::Model) {
        let squares = game.squares.clone();
        let finished = game.finished;
        let mut game: game::ActiveModel = game.into();
        game.squares = ActiveValue::Set(squares);
        game.finished = ActiveValue::Set(finished);
        game.update(&self.conn).await.unwrap();
    }
}
