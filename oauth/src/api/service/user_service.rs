use diesel::result::Error;
use thiserror::Error;

use crate::api::models::user::{NewUser, UpdateUser, User};
use crate::repository::{DbPool, UserRepository};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] Error),
    #[error("User not found")]
    NotFound,
}

pub struct UserService;

impl UserService {
    pub fn get_all_users(pool: &DbPool) -> Result<Vec<User>, UserServiceError> {
        UserRepository::find_all(pool).map_err(UserServiceError::DatabaseError)
    }

    pub fn get_user_by_id(id: i32, pool: &DbPool) -> Result<User, UserServiceError> {
        UserRepository::find_by_id(id, pool).map_err(|e| match e {
            Error::NotFound => UserServiceError::NotFound,
            _ => UserServiceError::DatabaseError(e),
        })
    }

    pub fn create_user(new_user: NewUser, pool: &DbPool) -> Result<User, UserServiceError> {
        UserRepository::create(new_user, pool).map_err(UserServiceError::DatabaseError)
    }

    pub fn update_user(
        id: i32,
        user: UpdateUser,
        pool: &DbPool,
    ) -> Result<User, UserServiceError> {
        UserRepository::update(id, user, pool).map_err(|e| match e {
            Error::NotFound => UserServiceError::NotFound,
            _ => UserServiceError::DatabaseError(e),
        })
    }

    pub fn delete_user(id: i32, pool: &DbPool) -> Result<(), UserServiceError> {
        let result = UserRepository::delete(id, pool).map_err(|e| match e {
            Error::NotFound => UserServiceError::NotFound,
            _ => UserServiceError::DatabaseError(e),
        })?;

        if result == 0 {
            return Err(UserServiceError::NotFound);
        }

        Ok(())
    }
}