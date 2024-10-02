# The User API

Currently, we only have the `create_user` handler to help us create a `User`. Let's add some other common ways we can
interact with `User`. In addition to creating a `User`, we need to be able to do the following,

* `GET` a list of all `User`s
* `GET` a single `User` by its `ID`
* Update one/more fields of a `User`
* Update the whole `User` object
* `DELETE` a `User` by `ID`

## List Users

This is done via an HTTP `GET` method. There isn't much to it. We just ask Sea-ORM to retrieve all the items from our
`User` table. We want our Users list to be ordered by `ID`, so we ask that of Se-ORM as well. After that, we just return
what was given to us as JSON. In case, there is some error, we handle that as well. Here is what it looks like,

```rust
#[get("")]
pub async fn get_users(app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let result = User::find()
        .order_by_asc(entity::user::Column::Id)
        .all(&app_state.db)
        .await;

    match result {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

With Postman or other similar tool, send a `GET` request to <http://localhost:8080/users>. Here's a sample result,

```json
[
  {
    "id": 4,
    "username": "pod",
    "firstname": "beef",
    "lastname": "burger",
    "email": "beef@bur.ger",
    "password": "123456",
    "is_active": null,
    "last_login": null,
    "date_joined": null,
    "created_at": null,
    "updated_at": null,
    "is_admin": false,
    "is_superadmin": false
  },
  {
    "id": 14,
    "username": "loki",
    "firstname": "Loki",
    "lastname": "Laufeyson",
    "email": "loki@asgua.rd",
    "password": "lokiTheGreat",
    "is_active": true,
    "last_login": "1970-01-01T00:00:00",
    "date_joined": null,
    "created_at": null,
    "updated_at": "2024-10-02T14:51:17.314780",
    "is_admin": true,
    "is_superadmin": true
  }
]
```

If you get back an empty list, make sure to add some `User`s with the `create_user` endpoint first.

## Get User by Id

Now, we'll try to get a `User` by its `ID`. When we try this, there may be a case that a `User` with the requested `ID`
doesn't exist at all. We should handle that in our API. Here's the handler for retrieving a `User` by `ID`,

```rust
#[get("/{id}")]
pub async fn get_user(id: web::Path<i32>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id.clone())
        .one(&app_state.db)
        .await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id);
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user) => Ok(HttpResponse::Ok().json(user))
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

Let's test this. Hit <http://localhost:8080/users/14> with Postman (replace the `/14` part with what's available to
you). In my case, I get this,

```json
{
  "id": 14,
  "username": "loki",
  "firstname": "Loki",
  "lastname": "Laufeyson",
  "email": "loki@asgua.rd",
  "password": "lokiTheGreat",
  "is_active": true,
  "last_login": "1970-01-01T00:00:00",
  "date_joined": null,
  "created_at": null,
  "updated_at": "2024-10-02T14:54:42.744507",
  "is_admin": true,
  "is_superadmin": true
}
```

## Updating one/more fields

This is a partial update, and it is facilitated by the HTTP `PATCH` method. The `PATCH` method allows our end users to
send only the field/fields they want to update in their payload. So if they want to update only the `username` or
another field with that, they will be able to send only those fields. This reduces the payload size a bit. Though it may
not be a big deal, unless a lots of users were doing this simultaneously.

To allow users to pass in only one/more fields we need to change our `UserRequest` model as well `User` entity. We will
replace all `field_name: value_type` with `field_name: Option<value_type>` syntax. Previously, we had the following
fields that are not `Option`s,

```rust
pub struct UserRequest {
    // other fields
    pub username: String,
    pub email: String,
    pub password: String,
    // other fields
}
```

After changing them to `Option`s, the `UserRequest` struct looks like this,

```rust
pub struct UserRequest {
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub date_joined: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_admin: Option<bool>,
    pub is_superadmin: Option<bool>,
}
```

and the entity in `entity/src/user.rs`, looks like this,

```rust
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    #[sea_orm(unique)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
    pub last_login: Option<DateTime>,
    pub date_joined: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub is_admin: Option<bool>,
    pub is_superadmin: Option<bool>,
}
```

**NOTE:** Be warned though, with this change we just traded a safeguard that was available to us at the compile time for
a flexibility. This flexibility comes with a cost. Now, this change means we have to manually make sure to pass the
required fields when constructing a `User`, Rust won't help us with that anymore. The database will still throw an error
if we messed up. That means the error has now shifted towards the runtime rather than compile time, which is something
to be aware of.

Here's the `update_user` method,

```rust
#[patch("/{id}")]
pub async fn update_user(id: web::Path<i32>, payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id.clone()).one(&app_state.db).await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user_model) => {
                    let mut user = user_model.into_active_model();
                    user.username = Set(payload.username.clone().or(user.username.unwrap()));
                    user.firstname = Set(payload.firstname.clone().or(user.firstname.unwrap()));
                    user.lastname = Set(payload.lastname.clone().or(user.lastname.unwrap()));
                    user.email = Set(payload.email.clone().or(user.email.unwrap()));
                    user.password = Set(payload.password.clone().or(user.password.unwrap()));
                    user.is_active = Set(Option::from(payload.is_active).or(user.is_active.unwrap()));
                    user.is_admin = Set(Option::from(payload.is_admin).or(user.is_admin.unwrap()));
                    user.is_superadmin = Set(Option::from(payload.is_superadmin).or(user.is_superadmin.unwrap()));
                    user.updated_at = Set(Option::from(
                        NaiveDateTime::new(
                            NaiveDate::from(Utc::now().naive_utc()),
                            NaiveTime::from(Utc::now().time()),
                        )
                    ));

                    let result = user.update(&app_state.db).await;
                    match result {
                        Ok(response) => Ok(HttpResponse::Ok().json(response)),
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

Notice the syntax `user.username = Set(payload.username.clone().or(user.username.unwrap())`. The
`or(user.username.unwrap())` part is used to set a default value if `username` isn't supplied in the payload.

Let's change the `password` for our user `loki`. Send a `PATCH` request to <http://localhost:8080/users/14> with the
following payload,

```json
{
  "password": "mischief"
}
```

And, we get the following response back,

```json
{
  "id": 14,
  "username": "loki",
  "firstname": "Loki",
  "lastname": "Laufeyson",
  "email": "loki@asgua.rd",
  // see? the password changed
  "password": "mischief",
  "is_active": true,
  "last_login": "1970-01-01T00:00:00",
  "date_joined": null,
  "created_at": null,
  "updated_at": "2024-10-02T15:51:55.160264",
  "is_admin": true,
  "is_superadmin": true
}
```

If you noticed, the response is the same as before except for the password, since that is the only field we changed. All
other fields remain unchanged.

## A Complete Update

Complete updates are achieved via `PUT` method. Now, we have to all the required fields. We can still omit the fields
that are **not required** by the database. But omitting other fields will result in them being set to the default value
for the field's data type.

Here's the `PUT` method to achieve that,

```rust
#[put("/{id}")]
pub async fn update_user_full(id: web::Path<i32>, payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id.clone()).one(&app_state.db).await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user_model) => {
                    let mut user = user_model.into_active_model();
                    user.username = Set(payload.username.clone());
                    user.firstname = Set(payload.firstname.clone());
                    user.lastname = Set(payload.lastname.clone());
                    user.email = Set(payload.email.clone());
                    user.password = Set(payload.password.clone());
                    user.is_active = Set(payload.is_active.clone());
                    user.is_admin = Set(payload.is_admin.clone());
                    user.is_superadmin = Set(payload.is_superadmin.clone());
                    user.updated_at = Set(Option::from(
                        NaiveDateTime::new(
                            NaiveDate::from(Utc::now().date_naive()),
                            NaiveTime::from(Utc::now().time()),
                        )
                    ));

                    let update = user.update(&app_state.db).await;
                    match update {
                        Ok(response) => Ok(HttpResponse::Ok().json(response)),
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

Here is a sample `PUT` request to <http://localhost:8080/users/14> with the following payload,

```json
{
  "username": "loki",
  "firstname": "Loki",
  "lastname": "Laufeyson",
  "email": "loki@asgua.rd",
  "password": "lokiTheGreat",
  "is_active": true,
  "is_admin": true,
  "is_superadmin": true
}
```

and here is the response,

```json
{
  "id": 14,
  "username": "loki",
  "firstname": "Loki",
  "lastname": "Laufeyson",
  "email": "loki@asgua.rd",
  "password": "lokiTheGreat",
  "is_active": true,
  "last_login": "1970-01-01T00:00:00",
  "date_joined": null,
  "created_at": null,
  "updated_at": "2024-10-02T16:19:09.619818",
  "is_admin": true,
  "is_superadmin": true
}
```

## Deleting a User by Id

Again, there isn't much to it. We query the database to see if a `User` with the requested `ID` exists. If not we return
a friendly error message, otherwise we delete it.

```rust
#[delete("/{id}")]
async fn delete_user(id: web::Path<i32>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id).one(&app_state.db).await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user_model) => {
                    let user = user_model.into_active_model();
                    let res = user.delete(&app_state.db).await;

                    match res {
                        Ok(delete_result) => {
                            let message = format!("Deleted {} user with Id {}", delete_result.rows_affected, user_id.clone());
                            let response = ApiResponse { message };
                            Ok(HttpResponse::Ok().json(response))
                        }
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
```

To test this, we send a `DELETE` request to <http://localhost:8080/users/4>. The first time we send this request, we get
this response,

```json
{
  "message": "Deleted 1 user with Id 4"
}
```

Subsequent `DELETE` requests to the same URL gives us the following response,

```json
{
  "message": "User with ID `4`, does not exist"
}
```

So, there it is, the complete CRUD API for our `User`s.