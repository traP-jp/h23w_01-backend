use crate::error::RepositoryError;
use entity::prelude::*;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, TransactionTrait,
};
use sea_orm_migration::MigratorTrait;
use std::env::{var, VarError};
use uuid::Uuid;

use domain::repository::{CardModel, CardRepository, MigrationStrategy, SaveCardParams};

use migration::Migrator;

#[derive(Debug, Clone)]
pub struct CardRepositoryConfig {
    pub user: String,
    pub password: String,
    pub hostname: String,
    pub port: String,
    pub database: String,
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
    type Error = RepositoryError;

    async fn migrate(&self, strategy: MigrationStrategy) -> Result<(), RepositoryError> {
        match strategy {
            MigrationStrategy::Up => Migrator::up(&self.0, None).await?,
            MigrationStrategy::Down => Migrator::down(&self.0, None).await?,
            MigrationStrategy::Refresh => Migrator::refresh(&self.0).await?,
            MigrationStrategy::None => (),
        };
        Ok(())
    }

    async fn save_card(&self, params: &SaveCardParams) -> Result<(), RepositoryError> {
        // TODO: 画像の保存
        let db = &self.0;
        let tx = db.begin().await?;
        let card = CardActiveModel {
            id: ActiveValue::Set(params.id),
            owner_id: ActiveValue::Set(params.owner_id),
            publish_date: ActiveValue::Set(params.publish_date),
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
        let cards = Card::find()
            .all(db)
            .await?
            .into_iter()
            .map(CardModel::from)
            .collect();
        Ok(cards)
    }

    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, RepositoryError> {
        let db = &self.0;
        let cards = Card::find()
            .filter(CardColumn::Id.contains(user_id))
            .all(db)
            .await?
            .into_iter()
            .map(CardModel::from)
            .collect();
        Ok(cards)
    }
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, RepositoryError> {
        let db = &self.0;
        let card = Card::find_by_id(card_id)
            .one(db)
            .await?
            .map(CardModel::from);
        Ok(card)
    }
    async fn get_publish_channels_by_id(
        &self,
        card_id: Uuid,
    ) -> Result<Vec<Uuid>, RepositoryError> {
        let db = &self.0;
        let pub_chans = PublishChannel::find()
            .filter(PublishChannelColumn::CardId.contains(card_id))
            .all(db)
            .await?
            .into_iter()
            .map(|c| c.id)
            .collect();
        Ok(pub_chans)
    }
    async fn delete_card(&self, card_id: Uuid) -> Result<Option<()>, RepositoryError> {
        let db = &self.0;
        let result = Card::delete_by_id(card_id).exec(db).await;
        match result {
            Ok(_) => Ok(Some(())),
            Err(DbErr::RecordNotFound(_)) => Ok(None),
            Err(e) => Err(RepositoryError::DbErr(e)),
        }
    }
}
