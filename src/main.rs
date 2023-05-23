#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Request;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/api/v1/answer")]
fn answer() -> &'static str {
    return get_answer();
}

fn get_answer() -> &'static str {
    return "42";
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/", routes![answer])
        .register(catchers![internal_error, not_found])
        .launch();
}