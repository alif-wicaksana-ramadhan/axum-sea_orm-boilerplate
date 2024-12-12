use sea_orm_migration::prelude::*;
use crate::m20241125_000001_create_users_table::Users;

#[derive(Iden)]
enum UserProfiles {
    Table,
    Id,
    FirstName,
    LastName,
    UserId
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241127_000001_create_user_profiles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserProfiles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfiles::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        ColumnDef::new(UserProfiles::FirstName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserProfiles::LastName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UserId)
                            .integer()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_profiles-user_id")
                            .from(UserProfiles::Table, UserProfiles::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserProfiles::Table).to_owned())
            .await
    }
}