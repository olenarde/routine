use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Simple user table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().primary_key())
                    .to_owned()
            )
            .await?;

        // One user can have as many sessions as needed
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    // FK uuid NN not unique
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .col(ColumnDef::new(Session::Token).string().not_null())
                    .to_owned()
            )
            .await?;

        // One user can only have one credentials record
        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Credentials::UserId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Credentials::Login).string_len(32).not_null().unique_key())
                    .col(ColumnDef::new(Credentials::Password).string_len(100).not_null())
                    .to_owned()
            )
            .await?;


        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user")
                    .from(Session::Table, Session::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user")
                    .from(Credentials::Table, Credentials::UserId)
                    .to(User::Table, User::Id)
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_user")
                    .table(Session::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_user")
                    .table(Credentials::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Credentials::Table)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Session::Table)
                    .if_exists()
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(User::Table)
                    .if_exists()
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id
}

#[derive(DeriveIden)]
enum Session {
    Table,
    UserId,
    Token,
}

#[derive(DeriveIden)]
enum Credentials {
    Table,
    UserId,
    Login,
    Password,
}