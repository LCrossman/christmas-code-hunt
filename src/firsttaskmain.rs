use rocket::{get, routes};
use rocket::http::Status;
use rocket::FromForm;
use rocket::form::Form;
use rocket::post;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/<error>")]
fn fake(error: &str) -> Status {
    Status::InternalServerError
}

#[get("/1/<num1>/<num2>")]
pub fn integer_this(num1: u32, num2: u32) -> String {
   let result = (num1 ^ num2).pow(3).to_string();
   result
}


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, fake, integer_this]);

    Ok(rocket.into())
}
