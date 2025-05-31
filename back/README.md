# EthGlobal Backend

A Rust backend API using Axum, Diesel, and PostgreSQL.

## Project Structure

```
back/
├── src/
│   ├── api/
│   │   ├── models/
│   │   │   ├── user.rs
│   │   │   ├── schema.rs
│   │   │   └── mod.rs
│   │   ├── repository/
│   │   │   ├── db.rs
│   │   │   ├── user_repository.rs
│   │   │   └── mod.rs
│   │   ├── service/
│   │   │   ├── user_service.rs
│   │   │   └── mod.rs
│   │   ├── routes/
│   │   │   ├── user_routes.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── main.rs
├── migrations/
│   └── 2024-05-01-000000_create_users/
│       ├── up.sql
│       └── down.sql
├── .env
├── diesel.toml
└── Cargo.toml
```

## Setup

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install

2. Set up PostgreSQL (choose one option):

   **Option A: Install PostgreSQL locally**
   - Install PostgreSQL: https://www.postgresql.org/download/
   - Create a PostgreSQL database:
     ```
     createdb ethglobal
     ```

   **Option B: Use Docker Compose**
   - Install Docker and Docker Compose: https://docs.docker.com/get-docker/
   - Start PostgreSQL using Docker Compose:
     ```
     cd back
     docker-compose up -d
     ```

3. Install Diesel CLI:
   ```
   cargo install diesel_cli --no-default-features --features postgres
   ```

4. Update the `.env` file with your database credentials and OAuth configuration if needed.
   - Default configuration works with both local PostgreSQL and Docker Compose setup.
   - For OAuth functionality, add the following environment variables:
     ```
     GITHUB_CLIENT_ID=your_github_client_id
     GITHUB_CLIENT_SECRET=your_github_client_secret
     OAUTH_REDIRECT_URL=http://localhost:3000/api/auth/github/callback
     ```
   - You can obtain GitHub OAuth credentials by creating a new OAuth App at https://github.com/settings/developers

5. Run the migrations:
   ```
   cd back
   diesel migration run
   ```

6. Build and run the application:
   ```
   cargo run
   ```

## Using Diesel ORM

Diesel is a Safe, Extensible ORM and Query Builder for Rust. This project uses Diesel for database operations with PostgreSQL.

### Setup and Configuration

1. **Add Diesel to your project**:
   ```toml
   # Cargo.toml
   [dependencies]
   diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
   dotenvy = "0.15.7"
   r2d2 = "0.8.10"
   ```

2. **Configure Diesel**:
   Create a `diesel.toml` file in your project root:
   ```toml
   [print_schema]
   file = "src/api/models/schema.rs"
   ```

3. **Set up environment variables**:
   Create a `.env` file with your database connection string:
   ```
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

### Database Connection

Set up a database connection pool using r2d2:

```rust
// src/api/repository/db.rs
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
```

### Migrations

Diesel uses migrations to manage database schema changes:

1. **Create a new migration**:
   ```bash
   diesel migration generate create_users
   ```

2. **Write migration SQL**:
   - `up.sql`: Contains SQL to apply the migration
     ```sql
     CREATE TABLE users (
         id SERIAL PRIMARY KEY,
         username VARCHAR NOT NULL,
         email VARCHAR NOT NULL,
         created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
         updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
     );
     ```
   - `down.sql`: Contains SQL to revert the migration
     ```sql
     DROP TABLE users;
     ```

   Example of a migration to add columns:
   - `up.sql`:
     ```sql
     ALTER TABLE users
     ADD COLUMN github_id VARCHAR,
     ADD COLUMN github_username VARCHAR,
     ADD COLUMN avatar_url VARCHAR,
     ADD COLUMN access_token VARCHAR;
     ```
   - `down.sql`:
     ```sql
     ALTER TABLE users
     DROP COLUMN github_id,
     DROP COLUMN github_username,
     DROP COLUMN avatar_url,
     DROP COLUMN access_token;
     ```

3. **Run migrations**:
   ```bash
   diesel migration run
   ```

4. **Generate schema**:
   ```bash
   diesel print-schema > src/api/models/schema.rs
   ```
   Or let Diesel CLI handle it automatically when running migrations.

### Defining Models

Define Rust structs that map to your database tables:

```rust
// src/api/models/user.rs
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::models::schema::users;

// Model for querying users
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Model for inserting new users
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
}

// Model for updating users
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub github_id: Option<String>,
    pub github_username: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
}
```

### CRUD Operations

Implement repository methods for database operations:

```rust
// src/api/repository/user_repository.rs
use diesel::prelude::*;
use diesel::result::Error;

use crate::api::models::schema::users;
use crate::api::models::user::{NewUser, UpdateUser, User};
use crate::api::repository::DbPool;

pub struct UserRepository;

impl UserRepository {
    // Find all users
    pub fn find_all(pool: &DbPool) -> Result<Vec<User>, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        users::table.select(User::as_select()).load(&mut conn)
    }

    // Find user by ID
    pub fn find_by_id(id: i32, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
    }

    // Create a new user
    pub fn create(new_user: NewUser, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    // Update a user
    pub fn update(id: i32, user: UpdateUser, pool: &DbPool) -> Result<User, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::update(users::table.find(id))
            .set(user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    // Delete a user
    pub fn delete(id: i32, pool: &DbPool) -> Result<usize, Error> {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::delete(users::table.find(id)).execute(&mut conn)
    }
}
```

### Advanced Queries

Diesel supports complex queries with its query builder:

```rust
// Example: Find users with a specific username
pub fn find_by_username(username: &str, pool: &DbPool) -> Result<Vec<User>, Error> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    users::table
        .filter(users::username.eq(username))
        .select(User::as_select())
        .load(&mut conn)
}

// Example: Find users with pagination
pub fn find_with_pagination(page: i64, per_page: i64, pool: &DbPool) -> Result<Vec<User>, Error> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    users::table
        .select(User::as_select())
        .limit(per_page)
        .offset((page - 1) * per_page)
        .load(&mut conn)
}

// Example: Find users with ordering
pub fn find_ordered_by_username(pool: &DbPool) -> Result<Vec<User>, Error> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    users::table
        .select(User::as_select())
        .order(users::username.asc())
        .load(&mut conn)
}
```

## API Endpoints

### Users

- `GET /api/users` - Get all users
- `GET /api/users/:id` - Get a user by ID
- `POST /api/users` - Create a new user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

### OAuth

- `GET /api/auth/github` - Initiate GitHub OAuth flow
- `GET /api/auth/github/callback` - Handle GitHub OAuth callback
- `GET /api/auth/me` - Get the currently authenticated user

## Example Requests

### Create a User

```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username": "john_doe", "email": "john@example.com", "github_id": "12345", "github_username": "johndoe", "avatar_url": "https://github.com/avatars/johndoe.png", "access_token": "gho_token123"}'
```

### Get All Users

```bash
curl http://localhost:3000/api/users
```

### Get a User by ID

```bash
curl http://localhost:3000/api/users/1
```

### Update a User

```bash
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"username": "jane_doe", "email": "jane@example.com", "github_username": "janedoe", "avatar_url": "https://github.com/avatars/janedoe.png"}'
```

### Delete a User

```bash
curl -X DELETE http://localhost:3000/api/users/1
```

### OAuth Flow

1. **Initiate GitHub OAuth**:

   Navigate to:
   ```
   http://localhost:3000/api/auth/github
   ```
   This will redirect to GitHub for authentication.

2. **After successful authentication**:

   GitHub will redirect to your callback URL with a code parameter.
   The server will:
   - Exchange the code for an access token
   - Retrieve user information from GitHub
   - Create or update the user in the database
   - Set an authentication cookie
   - Redirect to `/auth/success`

3. **Get Current User**:

   ```bash
   curl -b "auth_token=YOUR_AUTH_TOKEN" http://localhost:3000/api/auth/me
   ```
   This will return the currently authenticated user's information.
