use super::rocket;
use rocket::local::Client;

#[test]
fn hello_world() {
    let rocket = rocket::ignite().mount("/", routes![super::hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

#[test]
fn answer() {
    let rocket = rocket::ignite().mount("/", routes![super::answer]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/api/v1/answer").dispatch();
    assert_eq!(response.body_string(), Some("42".into()));
}

#[test]
fn not_found() {
    let rocket = rocket::ignite().mount("/", routes![super::hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/i_do_not_exist").dispatch();
    assert_eq!(response.body_string(), Some("I couldn't find '/i_do_not_exist'. Try something else?".into()));
}

// #[test]
// fn internal_error() {
//     let rocket = rocket::ignite().mount("/", routes![super::answer]);
//     let client = Client::new(rocket).unwrap();
//     let mut response = client.get("/api/v1/answer").dispatch();
//     assert_eq!(response.body_string(), Some("Whoops! Looks like we messed up.".into()));
// }