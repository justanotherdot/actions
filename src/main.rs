#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello!", age, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}
