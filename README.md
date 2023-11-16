# tiny_url

This repo demonstrates how to use actix-web, sqlx, and postgres.  None of this is original work.  I have just assembled what I have found to be useful. But I think it's a useful "getting started" demo for setting up a simple web app with typical Rust crates.

## Setting up app configuration
This app uses [Figment] to managage configuration.  Currently [AppConfig] builds configuration from an `App.toml`, if one exists, and from env vars.  It's totally up to you which you choose to populate.  Or, just use the defaults in [AppConfig]!

Since we are using sqlx, the database_url must be defined in an env var.  The app uses [dotenv].  So you can either use a .env file, or add to the shell before running.

Here's all the possible config settings.  Again, only DATABASE_URL is required.

```bash
RUST_LOG=info
TINY_URL_HOST=127.0.0.1
TINY_URL_PORT=8080
# Required for sqlx
DATABASE_URL=postgresql://postgre:postgre@localhost:5432/tiny_url
```

## CoLima

I use colima on my Mac, rather than Docker Desktop.  Just use Homebrew to install.

## docker-compose

I have included a very, very basic [docker-compose.yml](./docker-compose.yml), to show how to launch Postgres.

## initialize.sh

This script checks to ensure colima is running, and then  does some Docker stuff.  NOTE:  this script will remove any Docker volume created by previous docker-compose runs.  So, don't run this if you love your data!

## Bruno
There is a [Bruno](https://usebruno.com) collection in this distro that demonstrates how to use the API.  I'm warming to Bruno over Postman.

## Postgres

There is a simple [create_database.sql](./postgres/create_database.sql) script that is passed to the Docker init script.  This just demonstrate how to set it up.

## Running the app

Run the initialize.sh script to get Postgres up and running.
Then just do `cargo run` to launch the web server.  It will listen on `localhost:8080` by default.

## Testing

I use [Bruno], but feel free to use Postman, curl, or whatever.

### `GET http://localhost:8080/api/healthcheck`

This API just checks to see if the service is running. It should return:

```json
{
    "status":"success",
    "message":"API Server is up and running"
}
```

## LICENSE
MIT
I copied most of this from other sources.  Feel free to pay it forward.

[Figment]: https://docs.rs/figment
[Bruno]: https://www.usebruno.com
[AppConfig]: src/app_config.rs
[sqlx]: https://docs.rs/sqlx
[dotenv]: https://docs.rs/dotenv