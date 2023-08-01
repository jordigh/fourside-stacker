use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_player_table::Player;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Squares).json_binary().not_null())
                    .col(ColumnDef::new(Game::PlayerRedId).integer())
                    .col(ColumnDef::new(Game::PlayerBlackId).integer())
                    .col(
                        ColumnDef::new(Game::Finished)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-player-red-id")
                            .from(Game::Table, Game::PlayerRedId)
                            .to(Player::Table, Player::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-player-two-id")
                            .from(Game::Table, Game::PlayerBlackId)
                            .to(Player::Table, Player::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Game {
    Table,
    Id,
    Squares,
    PlayerRedId,
    PlayerBlackId,
    Finished,
}
