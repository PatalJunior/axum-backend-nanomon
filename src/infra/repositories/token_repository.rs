use chrono::{DateTime, Utc};
use diesel::{AsChangeset, ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::token::{TokenError, TokenModel};
use crate::infra::db::schema::{tokens};
use crate::infra::errors::{adapt_infra_error, InfraError};
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub ip_address: String,
    pub user_agent: String,
    pub replaced_by: Option<Uuid>,
    pub previous_token_id: Option<Uuid>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = tokens)]
pub struct NewTokenDb {
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
}


#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = tokens)]
pub struct UpdateTokenDb {
    pub expires_at: DateTime<Utc>,
    pub revoked_at: DateTime<Utc>,
    pub replaced_by: Option<Uuid>,
}


#[derive(Debug, Deserialize)]
pub struct TokensFilter {
    user_id: Option<Uuid>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    get_chained_tokens: Option<bool>,
}


pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_token: crate::infra::repositories::token_repository::NewTokenDb,
) -> Result<TokenModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(tokens::table)
                .values(new_token)
                .returning(TokenDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_token_db_to_token(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<TokenModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            tokens::table
                .filter(tokens::id.eq(id))
                .select(TokenDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_token_db_to_token(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: TokensFilter,
) -> Result<Vec<TokenModel>, InfraError> {
    // Parse user_id UUID early if present and non-empty

    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            let mut query = tokens::table.into_boxed::<diesel::pg::Pg>();

            if let Some(user_uuid) = filter.user_id {
                query = query.filter(tokens::user_id.eq(user_uuid));
            }

            if let Some(ip_address) = filter.ip_address {
                if !ip_address.is_empty() {
                    query = query.filter(tokens::ip_address.eq(ip_address));
                }
            }

            query.select(TokenDb::as_select()).load::<TokenDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let tokens: Vec<TokenModel> = res
        .into_iter()
        .map(|token_db| adapt_token_db_to_token(token_db))
        .collect();

    Ok(tokens)
}


pub async fn update(
    pool: &deadpool_diesel::postgres::Pool,
    token_id: Uuid,
    changes: UpdateTokenDb,
) -> Result<TokenModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(tokens::table.filter(tokens::id.eq(token_id)))
                .set(changes)
                .returning(TokenDb::as_returning())
                .get_result::<TokenDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_token_db_to_token(res))
}


fn adapt_token_db_to_token(token_db: TokenDb) -> TokenModel {
    TokenModel {
        id: token_db.id,
        user_id: token_db.user_id,
        token_hash: token_db.token_hash,
        created_at: token_db.created_at,
        expires_at: token_db.expires_at,
        revoked_at: token_db.revoked_at,
        ip_address: token_db.ip_address,
        user_agent: token_db.user_agent,
        replaced_by: token_db.replaced_by,
        previous_token_id: token_db.previous_token_id,

    }
}