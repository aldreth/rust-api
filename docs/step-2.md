## Add diesel
* Install diesel globally - `cargo install diesel_cli`
* Add `rocket_contrib`, `diesel` & `dotenv` to Cargo.toml
* Set `DATABASE_URL` in .env
* Run `diesel setup`

## Set up database on heroku
* Add migration release task to `Procfile`
* Install diesel using the buildpack by adding `RustConfig`
* Add pg database on heroku

## Add migrations
Going to use a `uuid` for primary key, so need to add a migration to ensure the pg `uuid-ossp` extension exists.
Add a second migration to create users table - `SELECT diesel_manage_updated_at('users');` allows diesel to manage the `updated_at` column. Whenever a row is changed, the value is automatically set using a trigger created in the initial migration.

