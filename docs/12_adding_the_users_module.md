# Adding the users module

Since we are taking the modular approach to developing our application, it seems befitting that we put all code related
to `User` in a separate module. Let's call it `users`.

We start by creating a `users` directory and adding the following files,

```bash
users
├── handlers.rs
├── mod.rs
├── models.rs
├── serializers.rs
└── urls.rs
```

Let's first look at the `mod.rs`,

```rust
// private modules
mod models;
mod serializers;

// public modules
pub mod handlers;
pub mod urls;
```

Then, the `models.rs`,

```rust
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub date_joined: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
```

We use `ApiResponse` to return custom responses. The `UserRequest` corresponds to our `User` model and is used to
serialize JSON data coming from the API endpoint.

Then, the `serializers.rs`,

```rust
use crate::users::models::UserRequest;
use actix_web::web::Json;
use entity::user::ActiveModel;
use sea_orm::ActiveValue;

pub struct UserSerializer {
    pub data: Json<UserRequest>,
}

impl UserSerializer {
    pub fn serialize(&self) -> ActiveModel {
        let is_active = self.is_active();
        let user = ActiveModel {
            username: ActiveValue::Set(self.data.username.clone()),
            firstname: ActiveValue::Set(self.data.firstname.clone()),
            lastname: ActiveValue::Set(self.data.lastname.clone()),
            email: ActiveValue::Set(String::from(self.data.email.clone())),
            password: ActiveValue::Set(self.data.password.clone()),
            is_active: ActiveValue::Set(Option::from(is_active)),
            last_login: ActiveValue::Set(Option::from(self.data.last_login)),
            date_joined: ActiveValue::Set(Option::from(self.data.date_joined)),
            created_at: ActiveValue::Set(Option::from(self.data.created_at)),
            updated_at: ActiveValue::Set(Option::from(self.data.updated_at)),
            ..Default::default()
        };

        user
    }

    fn is_active(&self) -> bool {
        match self.data.is_active {
            None => false,
            Some(_) => true
        }
    }
}
```

The `UserSerializer` offloads the task of serializing JSON data into `User` model from our `create_user` endpoint. We
could do this in the endpoint itself, but again, I decided to take the modular approach. It helps keep the actual
endpoint clean. Additionally, if you want, you can add much more customization here without cluttering the handler
itself. Like, I did here by adding the `is_active()` function, which will convert `null` (`None` for Sea-ORM) type
values or, absence of values to `false` and `true` otherwise. I can add more utility functions like this to
`UserSerializer` without having to update the `create_user` endpoint. This is not necessary by any means. The goal is to
keep the endpoint as minimal as possible. This way, you can just take a look at the endpoint and tell exactly what it's
doing. If you want to know the details about any function/definition, most modern code editors/IDEs will allow you to do
that. I have seen people write everything in one giant handler/controller that spans thousands of lines of code. I love
writing modular code, when it adds to clarity, whenever possible. Be aware though, too much can add to complexity. So be
modular if you want, but do it in moderation.

Then, the `handlers.rs`,

```rust
use actix_web::web::Json;
use actix_web::{post, web, Error, HttpResponse, Responder};
use sea_orm::ActiveModelTrait;

use crate::users::models::{ApiResponse, UserRequest};
use crate::users::serializers::UserSerializer;
use crate::utils::app_state::AppState;


#[post("/create")]
pub async fn create_user(payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let serializer = UserSerializer { data: payload };
    let user = serializer.serialize();

    let result = user.insert(&app_state.db).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

Nothing special here. All the work is done by `UserSerializer`. We just retrieve the `User` object from the serializer
and make an attempt to insert it to the database. If the operation succeds, we serialize the inserted `User` object into
JSON and return that. Otherwise, we encounter an error and we return that to the end user.

and, finally, the `urls.rs`, not much to explain here,

```rust
use actix_web::web;
use crate::users::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(handlers::create_user)
    );
}
```

To make `users` available to the application, we update our `App()` instance. We update our `main.rs` as follows,

```rust
mod home;   // we just added this
mod users;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use utils::config::{get_address, get_db_connection};
use utils::log::set_logger;

use crate::utils::app_state::AppState;

fn init() {
    set_logger();
    dotenv::dotenv().ok();
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    init();

    let db = get_db_connection().await;
    Migrator::up(&db, None).await.unwrap();

    let (host, port) = get_address();
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(home::urls::routes)
            .configure(users::urls::routes)     // we just added this
    })
        .bind((host, port))?
        .run()
        .await
}
```

At this point we are almost done with `users`. Except, we want some of our users to have administrative privileges. This
means we need to add some fields to our `User` model again. This time though, we will do it through a `alter_table`
migration.

But this souldn't stop us from testing our endpoint. Head over to Postman, and send a `POST` request
to <http://localhost:8080/users/create> with the following payload,

```json
{
  "username": "jon",
  "firstname": "Jon",
  "lastname": "Snow",
  "email": "jon@sn.ow",
  "password": "123456",
  "last_login": "2024-12-13T00:00:00",
  "date_joined": "2024-11-15T00:00:00",
  "created_at": "2024-10-17T00:00:00",
  "updated_at": "2024-09-19T00:00:00"
}
```

You should get a response like this,

```json
{
  "id": 28,
  "username": "jon",
  "firstname": "Jon",
  "lastname": "Snow",
  "email": "jon@sn.ow",
  "password": "123456",
  "is_active": false,
  "last_login": "2024-12-13T00:00:00",
  "date_joined": "2024-11-15T00:00:00",
  "created_at": "2024-10-17T00:00:00",
  "updated_at": "2024-09-19T00:00:00"
}
```
