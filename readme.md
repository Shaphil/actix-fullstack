# Fullstack web application with Actix-Web

We are going to create a fullstack web application with Rust. We are going to use the following tools/frameworks,

- **Backend:** Actix-Web - <https://actix.rs/>
- **Migrations:** Sea-ORM - <https://www.sea-ql.org/SeaORM/>
- **Database:** PostgreSQL - <https://www.postgresql.org/>
- **Templates:** Tera - <https://keats.github.io/tera/docs/>
- **Frontend:** Svelte - <https://svelte.dev/>

We will also
use [redis](https://redis.io/), [Docker](https://www.docker.com/)/[Docker Compose](https://docs.docker.com/compose/) and
also try to incorporate [RabbitMQ](https://www.rabbitmq.com/).

Let's get started.

## Table of contents

1. [Initial Project Setup](docs/01_initial_project_setup.md)
2. [Logging](docs/02_logging.md)
3. [Reading application defaults from .env](docs/03_reading_application_defaults_from_.env.md)
4. [Making the application Modular](docs/04_making_the_application_modular.md)
5. [Returning JSON response](docs/05_returning_json_response.md)
6. [Updating the logger](docs/06_updating_the_logger.md)
7. [Adding a Database to Persist Data](docs/07_adding_a_database_to_persist_data.md)
8. [Adding Sea-ORM](docs/08_adding_sea_orm.md)
9. [Sea-ORM: Creating Models and Entities](docs/09_sea_orm_creating_models_and_entities.md)
10. [Adding missing field to a Model](docs/10_adding_missing_field_to_a_model.md)
11. [Updating the entity](docs/11_updating_entity.md)
12. [Adding the users module](docs/12_adding_the_users_module.md)
13. [Adding Admin fields to User](docs/13_adding_admin_fields_to_user.md)
14. [The User API](docs/14_the_user_api.md)

## Running the server

To run the server, rename the example `dot.env` to `.env` and set the expected values to your suitable default. Here is
an example for running locally,

```dotenv
# application
HOST=127.0.0.1
PORT=8080

# database
DATABASE_URL=postgres://postgres:password@localhost/actix-fullstack
```

From the terminal, navigate to application directory. From inside the directory, hit `cargo run`. Your server should be
up and running and should be available to you at <http://localhost:8080> or <http://127.0.0.1>