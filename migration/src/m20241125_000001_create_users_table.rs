use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    Password,
    Role
}

#[derive(Iden)]
enum UserRole {
    Table,
    User,
    Admin,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241125_000001_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Table)
                    .values([UserRole::User, UserRole::Admin])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Users::Role)
                            .custom(UserRole::Table)
                            .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_table(Users::Table.to_string()).await? {
            manager
                .drop_table(Table::drop().table(Users::Table).to_owned())
                .await?;
        }
        manager.drop_type(Type::drop().name(UserRole::Table).to_owned()).await?;
        Ok(())
    }
}

