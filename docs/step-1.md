# Step 1

## Initial set up

```
cargo new rust-users
cd rust-users
cargo run
git add .
git commit -m 'Initial project setup'
```

## Add rocket

* https://rocket.rs/v0.4/guide/getting-started/
  * Add rocket crate
  * Update main.rs
  * Add test

## Run tests in github actions
  * Add ./workflows/rust.yml

## Deploy to heroku
  * Add Procfile
  * Create heroku app
  * Add rust buildpack - https://github.com/emk/heroku-buildpack-rust
  * Create SECRET_KEY environment config var - `openssl rand -base64 32`
