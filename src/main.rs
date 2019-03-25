#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;
#[get("/json")]
fn json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "Hello new, world!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello my, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, world, json])
                    .launch();
    
}