use bcrypt::{hash, DEFAULT_COST};
use diesel::{Queryable, Insertable, prelude::*};
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::{schema::{users, secure_user_info}, PoolConn};

#[derive(Queryable, Serialize, Insertable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_date: NaiveDateTime,
    pub username: String,
}

#[derive(Queryable, Serialize, Insertable)]
#[table_name = "secure_user_info"]
pub struct SecureUserInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String
}

pub fn insert_new_user(conn: &PoolConn, user: CreateUser) -> anyhow::Result<User> {
    use self::users::dsl::*;
    use self::secure_user_info::dsl::*;

    let time = chrono::Utc::now();
    let hashed_pass = hash(user.password, DEFAULT_COST)
        .or_else(|e| Err(anyhow::anyhow!("Unable to compute hash value for new password.")))?;

    let new_user = User {
        id: Uuid::new_v4(),
        first_name: user.first_name,
        last_name: user.last_name,
        created_date: NaiveDateTime::from_timestamp(time.timestamp(), time.timestamp_subsec_nanos()),
        username: user.username,
    };

    let new_secure_info = SecureUserInfo {
        id: Uuid::new_v4(),
        user_id: new_user.id.to_owned(),
        password: hashed_pass
    };

    let commited_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .or_else(|e| Err(anyhow::anyhow!("Could not insert user.")))?;
    
    diesel::insert_into(secure_user_info)
        .values(&new_secure_info)
        .execute(conn)
        .or_else(|e| {
            diesel::delete(users.find(commited_user.id))
                .execute(conn)
                .or_else(|e| Err(anyhow::anyhow!("Unable to rollback user: {}", commited_user.username)))?;
            
            Err(anyhow::anyhow!("Unable to create secure user info for user: {}", commited_user.username))
        })?;
    
    
    Ok(commited_user)
}