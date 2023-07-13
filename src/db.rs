use futures::executor::block_on;

const DATABASE_URL: &str = "postgresql://oi:ecometrica@localhost";
const DB_NAME: &str = "rustgame";
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
                .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
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

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    Ok(())
}

pub fn db_init () {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
 
