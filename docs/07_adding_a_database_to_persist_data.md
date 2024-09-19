# Adding a Database to Persist Data

To persist data we need to set up our Database. In this app, we will use [PostgreSQL](https://www.postgresql.org/) as
our database and [Sea-ORM](https://www.sea-ql.org/SeaORM/) to manage database migrations. We'll now update our`.env`
file with the database connection string. My`.env` file looks like this after adding the connection string,

```toml
HOST = 127.0.0.1
PORT = 8080

# just added
DATABASE_URL = postgres://user:password@localhost/database-name
```

With that let's update our `config.rs` file with the database setup. Add the following to the existing file,

```rust
use std::env;
use std::env::VarError;
use lazy_static::lazy_static;

lazy_static! {
    // previous declarations omitted
    pub static ref DATABASE_URL: String = set_db();
}

fn set_db() -> String {
    get_env("DATABASE_URL").unwrap()
}

pub fn get_db() -> String {
    (*DATABASE_URL).clone()
}
```

Before we can add the database connection to our main, we need to create a struct that has a member of type . First,
let's install sea-orm with the features we need,

```bash
cargo add sea-orm -F sea-orm/sqlx-postgres,sea-orm/runtime-tokio-rustls,sea-orm/macros
```

Now, we'll define a `AppState` struct with our database connection as its member in `src/utils/app_state.rs`,

```rust
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
}
```

In `main.rs` we add the database connection from `AppState` as an application
state (<https://actix.rs/docs/application/#state>). Add the databasee connection as an `app_data()` to the application
instance like this, `.app_data(web::Data::new(AppState { db: db.clone() }))`. Our `main.rs` should now look like this,

```rust
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use utils::config::{get_address, get_db};
use utils::app_state::AppState;

// database setup
async fn setup_db() -> DatabaseConnection {
    let db_url = get_db();
    Database::connect(db_url).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // database setup
    let db = setup_db().await;
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            // database setup
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(home::urls::routes)
    })
        .bind((host, port))?
        .run()
        .await
}
```

Our database setup is complete. We should be able to save any data in our database if we want to. To do that, let's
create some models, and we will use Sea-ORM instead of typing raw SQL to query the database.
