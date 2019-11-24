#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello!")
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
