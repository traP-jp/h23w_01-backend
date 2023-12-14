use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Card::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Card::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Card::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Card::PublishDate).date_time().not_null())
                    .col(ColumnDef::new(Card::Message).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PublishChannel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PublishChannel::Id).uuid().not_null())
                    .col(ColumnDef::new(PublishChannel::CardId).uuid().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CardSvg::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CardSvg::CardId).uuid().not_null())
                    .col(ColumnDef::new(CardSvg::Content).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CardPng::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CardPng::CardId).uuid().not_null())
                    .col(ColumnDef::new(CardPng::Content).binary().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Image::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Image::MimeType).string().not_null())
                    .col(ColumnDef::new(Image::Content).binary().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Card::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PublishChannel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CardPng::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CardSvg::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Card {
    Table,
    Id,
    OwnerId,
    PublishDate,
    Message,
}

#[derive(DeriveIden)]
enum PublishChannel {
    Table,
    Id,
    CardId,
}

#[derive(DeriveIden)]
enum CardSvg {
    Table,
    CardId,
    Content,
}

#[derive(DeriveIden)]
enum CardPng {
    Table,
    CardId,
    Content,
}

#[derive(DeriveIden)]
enum Image {
    Table,
    Id,
    MimeType,
    Content,
}
