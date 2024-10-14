#[macro_use] extern crate rocket;

mod controller;
use controller::*;

mod jsonfn;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, filter, map, fold])
}
