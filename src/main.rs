#![feature(decl_macro)] // Enable the experimental feature

use rocket::{get, routes, Rocket};

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
	"Hello, Rocket!"
}

#[get("/greet/<name>")]
fn greet(name: &str) -> String {
	format!("Hello, {}!", name)
}

fn main() {
	rocket::ignite()
		.mount("/", routes![index, hello, greet])
		.launch();
}