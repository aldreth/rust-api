#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}
fn rocket() -> Rocket {
  rocket::ignite().mount("/", routes![index])
}

fn main() {
  rocket().launch();
}
