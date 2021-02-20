#![feature(proc_macro_hygiene, decl_macro)]

//rocket_carets
#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

mod increment;

#[get("/inc/<_n>")]
fn inc(_n: i64) -> String {
    format!("Average {}", increment::add_number(_n))
}

#[get("/all")]
fn all() -> String {
    format!("All {}", increment::number_string())
}

fn main() {
    rocket::ignite().mount("/", routes![hello, inc, all]).launch();
}