## Add diesel
* Install diesel globally - `cargo install diesel_cli`
* Add rocket_contrib, diesel & dotenv to Cargo.toml
* Set DATABASE_URL in .env
* Run `diesel setup`

## Set up database on heroku
* Add migration release task to `Procfile`
* Install diesel using the buildpack by adding `RustConfig`
* Add pg database on heroku
