use chrono::NaiveDate;
use diesel::{AsChangeset, ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: NaiveDate
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}


#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUserDb {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}


#[derive(Debug, Deserialize)]
pub struct UsersFilter {
    usernames: Option<Vec<String>>,
    username: Option<String>,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_user: NewUserDb,
) -> Result<UserModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(UserDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_db_to_user(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<UserModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::id.eq(id))
                .select(UserDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_db_to_user(res))
}

pub async fn find_by_username(
    pool: &deadpool_diesel::postgres::Pool,
    username: String,
) -> Result<Option<UserModel>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .select(UserDb::as_select())
                .first::<UserDb>(conn)
                .optional()
        })
        .await
        .map_err(adapt_infra_error)? // handle interact
        .map_err(adapt_infra_error)?; // handle diesel

    Ok(res.map(adapt_user_db_to_user))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: UsersFilter,
) -> Result<Vec<UserModel>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            let mut query = users::table.into_boxed::<diesel::pg::Pg>();

            if let Some(usernames) = filter.usernames {
                if !usernames.is_empty() {
                    print!("Querying multiple usernames");
                    query = query.filter(users::username.eq_any(usernames));
                }
            }

            if let Some(username) = filter.username {
                print!("Querying single username");
                query = query.filter(users::username.eq(username))
            }

            //println!("SQL Query: {}", debug_query::<Pg, _>(&query));

            query.select(UserDb::as_select()).load::<UserDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let users: Vec<UserModel> = res
        .into_iter()
        .map(|user_db| adapt_user_db_to_user(user_db))
        .collect();

    Ok(users)
}

pub async fn update(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: Uuid,
    changes: UpdateUserDb,
) -> Result<UserModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(changes)
                .returning(UserDb::as_returning())
                .get_result::<UserDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_db_to_user(res))
}


fn adapt_user_db_to_user(user_db: UserDb) -> UserModel {
    UserModel {
        id: user_db.id,
        email: user_db.email,
        username: user_db.username,
        is_admin: user_db.is_admin,
        password_hash: user_db.password_hash,
        created_at: user_db.created_at
    }
}

