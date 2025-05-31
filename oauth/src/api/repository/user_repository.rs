use diesel::prelude::*;
use diesel::result::Error;

use crate::api::models::schema::users;
use crate::api::models::user::{NewUser, UpdateUser, User};
use crate::repository::DbPool;

pub struct UserRepository;

impl UserRepository {
    pub fn find_all(pool: &DbPool) -> Result<Vec<User>, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        users::table.select(User::as_select()).load(&mut conn)
    }

    pub fn find_by_id(id: i32, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
    }

    pub fn create(new_user: NewUser, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    pub fn update(id: i32, user: UpdateUser, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::update(users::table.find(id))
            .set(user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    pub fn delete(id: i32, pool: &DbPool) -> Result<usize, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(users::table.find(id)).execute(&mut conn)
    }
}