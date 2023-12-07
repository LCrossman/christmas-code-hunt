use rocket::{get, routes};
use rocket::http::Status;
use std::path::PathBuf;
use rocket::serde::{Deserialize, Serialize};
use rocket::post;
use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response};
use rocket::http::ContentType;
use regex::Regex;
use unescape::unescape;
use serde_json::json;
use std::collections::HashMap;
use rocket::serde::json::Json;
use rocket::Data;
use rocket::form::{Form, FromForm};
use serde_json::Result;
use rocket::response::Responder;
use rocket::response::content;
use rocket::http::Method;

pub fn xor(vc: Vec<u32>) -> u32 {
   let mut item: u32 = 0;
   for v in vc {
      item ^= v;
      }
   item
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Santa {
   items: Vec<Reindeer>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Reindeer {
   name: String,
   strength: u32,
}


#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyResponder {
  pub status: Status,
  pub data: HashMap<String, usize>,
}

impl<'r> Responder<'r,'r> for MyResponder {
    fn respond_to(self, _: &rocket::Request<'_>) -> response::Result<'static> {
          let body = json!(self.data).to_string();
 
//	//println!("json string {:?}", &json_string);
  //      let json_body: response::Result<'r> = serde_json::from_str(unescape(&json_string).unwrap().as_str()).unwrap();
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
	    .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

#[post("/6", format="text/plain", data="<elves>")]
fn elf_count(elves: &str) -> MyResponder {
   let re = Regex::new(r"elf").unwrap();
   let strelves = elves.clone();
   let noelves = strelves.clone();
   let result = re.captures_iter(elves);
   let shre = Regex::new(r"shelf.");
   let mut noel: Vec<_> = shre.expect("issue splitting").split(noelves).collect();
   noel.retain(|&noe| noe!="");
   println!("noel is {:?}", &noel);
   let nore = Regex::new(r"elf on (a|that) (shelf.|shelf)").unwrap();
   let mut shelves: Vec<_> = nore.split(strelves).collect();
   shelves.retain(|&item| item!="");
   println!("shelves are {:?}", shelves);
   let mut shelf_count: usize = shelves.len() - 1;
   let mut noelf_count: usize = noel.len() - shelves.len();
   println!("noelves - shelves are {:?}", noelf_count);
   let mut elf_count: usize = 0;
   for res in result {
       elf_count+=1;
       }
   let mut jstring: HashMap<String, usize> = HashMap::new();
   if elf_count > 0 {
       jstring.insert("elf".to_string(), elf_count);
       }
   if shelf_count > 0 {
       jstring.insert("elf on a shelf".to_string(), shelf_count);
       }
   if noelf_count > 0 {
       jstring.insert("shelf with no elf on it".to_string(), noelf_count);
       }
   println!("jstring is {:?}", jstring);
   MyResponder { status: Status::Ok, data: jstring }
   //Json(MyResponder {elf: elf_count, shelf: shelf_count, noelf: noelf_count})
}

#[post("/4/strength", format = "application/json", data="<strength>")]
fn calc_strength(strength: &str) -> String {
    let r: Vec<Reindeer> = serde_json::from_str(strength).expect("none such");
    let mut reindeer_strength: Vec<u32> = Vec::new();
    for data in r.iter() {
        reindeer_strength.push(data.strength);
	}
    let answer: u32 = reindeer_strength.iter().sum();
    answer.to_string()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/<error>")]
fn fake(error: &str) -> Status {
    Status::InternalServerError
}

#[get("/1/<it..>")]
pub fn integer_this(it: PathBuf) -> String {
   let numbers: Vec<u32> = it.to_str().unwrap().split('/').filter_map(|s| s.parse().ok()).collect();
   let answer = xor(numbers).pow(3).to_string();
   answer
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, fake, integer_this, calc_strength, elf_count]);

    Ok(rocket.into())
}