use crate::error::RepositoryError;
use entity::prelude::*;
use migration::Migrator;
use sea_orm::{
    prelude::DateTimeUtc, ActiveValue, ColumnTrait, ConnectOptions, Database, DatabaseConnection,
    DbErr, EntityTrait, QueryFilter, TransactionTrait,
};
use sea_orm_migration::MigratorTrait;
use std::env::{var, VarError};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait CardRepository {
    async fn migrate(&self, strategy: MigrationStrategy) -> Result<(), DbErr>;
    async fn save_card(&self, params: &SaveCardParams) -> Result<(), RepositoryError>;
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, RepositoryError>;
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, RepositoryError>;
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, RepositoryError>;
    async fn delete_card(&self, card_id: Uuid) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct CardRepositoryConfig {
    pub user: String,
    pub password: String,
    pub hostname: String,
    pub port: String,
    pub database: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MigrationStrategy {
    Up,
    Down,
    Refresh,
    #[default]
    None,
}

impl std::str::FromStr for MigrationStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "refresh" => Ok(Self::Refresh),
            "none" => Ok(Self::None),
            s => Err(format!("unknown strategy `{}`", s)),
        }
    }
}

impl CardRepositoryConfig {
    pub fn load_env_with_prefix(prefix: &str) -> Result<Self, VarError> {
        let var_suff = |suffix: &'static str| var(format!("{}{}", prefix, suffix));
        Ok(Self {
            user: var_suff("USER")?,
            password: var_suff("PASSWORD")?,
            hostname: var_suff("HOSTNAME")?,
            port: var_suff("PORT")?,
            database: var_suff("DATABASE")?,
        })
    }

    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.hostname, self.port, self.database
        )
    }
}

pub struct CardRepositoryImpl(DatabaseConnection);
impl CardRepositoryImpl {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self(db.clone())
    }

    pub async fn connect(opt: impl Into<ConnectOptions>) -> Result<Self, RepositoryError> {
        let db = Database::connect(opt).await?;
        Ok(Self(db))
    }

    pub async fn connect_with_config(
        config: CardRepositoryConfig,
    ) -> Result<Self, RepositoryError> {
        let url = config.database_url();
        Self::connect(url).await
    }
}

#[async_trait::async_trait]
impl CardRepository for CardRepositoryImpl {
    async fn migrate(&self, strategy: MigrationStrategy) -> Result<(), DbErr> {
        match strategy {
            MigrationStrategy::Up => Migrator::up(&self.0, None).await,
            MigrationStrategy::Down => Migrator::down(&self.0, None).await,
            MigrationStrategy::Refresh => Migrator::refresh(&self.0).await,
            MigrationStrategy::None => Ok(()),
        }
    }

    async fn save_card(&self, params: &SaveCardParams) -> Result<(), RepositoryError> {
        // TODO: 画像の保存
        let db = &self.0;
        let tx = db.begin().await?;
        let card = CardActiveModel {
            id: ActiveValue::Set(params.id),
            owner_id: ActiveValue::Set(params.owner_id),
            publish_date: ActiveValue::Set(params.publish_date.naive_utc()),
            message: ActiveValue::Set(params.message.clone()),
        };
        let channels = params
            .channels
            .iter()
            .map(|channel_id| PublishChannelActiveModel {
                id: ActiveValue::Set(*channel_id),
                card_id: ActiveValue::Set(card.id.clone().unwrap()),
            })
            .collect::<Vec<_>>();
        Card::insert(card).exec(&tx).await?;
        PublishChannel::insert_many(channels).exec(&tx).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn get_all_cards(&self) -> Result<Vec<CardModel>, RepositoryError> {
        let db = &self.0;
        let cards = Card::find().all(db).await?;
        Ok(cards)
    }

    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, RepositoryError> {
        let db = &self.0;
        let cards = Card::find()
            .filter(CardColumn::Id.contains(user_id))
            .all(db)
            .await?;
        Ok(cards)
    }
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, RepositoryError> {
        let db = &self.0;
        let card = Card::find_by_id(card_id).one(db).await?;
        Ok(card)
    }
    async fn delete_card(&self, card_id: Uuid) -> Result<(), RepositoryError> {
        let db = &self.0;
        Card::delete_by_id(card_id).exec(db).await?;
        Ok(())
    }
}

pub struct SaveCardParams {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub message: Option<String>,
    pub channels: Vec<Uuid>,
}
