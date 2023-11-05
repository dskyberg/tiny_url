# tiny_url

This repo demonstrates how to use actix-web, sqlx, and postgres.  None of this is original work.  I have just assembled what I have found to be useful.

## Setting up a .env

Here's an example .env that you can copy:

```bash
RUST_LOG=info
TINY_URL_HOST=127.0.0.1
TINY_URL_PORT=8080
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=tiny_url
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}?schema=public
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

I use [Bruno](https://www.usebruno.com/), but feel free to use Postman, curl, or whatever.

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
