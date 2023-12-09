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
use base64::{engine, alphabet, Engine as _};
use rocket::http::{Cookie, CookieJar};
use base64::engine::general_purpose;
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
          Response::build()
            .status(self.status)
            .header(ContentType::JSON)
	    .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct TopResponse {
   recipe: BakeResponse,
   pantry: BakeResponse,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct BakeResponse {
   flour: usize,
   sugar: usize,
   butter: usize,
   #[serde(rename = "baking powder")]
   baking_powder: usize,
   #[serde(rename = "chocolate chips")]
   chocolate_chips: usize,
}

#[derive(Debug)]
pub struct Pokemon {
   weight: usize,
}

#[get("/8/drop/<val>")]
async fn get_momentum(val: &str) -> String {
      let query_string = format!("https://pokeapi.co/api/v2/pokemon/{}",val.parse::<usize>().unwrap());
   let body = reqwest::get(query_string).await.expect("problem").text();
   let mut long_body: String = body.await.expect("issue");
   let mut split_body: Vec<_> = long_body.split(",").collect::<Vec<_>>();
   let re = Regex::new(r"(:|})").unwrap();
   let popped: Vec<_> = re.split(&split_body.last().unwrap()).collect();
   let mut answer: String = String::new();
   let mut momentum: f64 = 0.0;
   for splitted_str in popped.iter() {
      if let Ok(weight) = splitted_str.parse::<usize>() {
         //first p = m * v momentum, mass, velocity
	 //where Potential Energy = m *g*h mass, gravit accel and height
	 let PE = weight as f64/10.0 * 9.825 * 10.0;
	 println!("Potential Energy is {:?}", &PE);
	 let vel: f64 = ((2.0 as f64) * (9.825 as f64) * (10.0 as f64)).sqrt();
	 println!("velocity is {:?}", &vel);
	 momentum = weight as f64/10.0 * vel;
	 println!("momentum is {:?}", &momentum);
         // Mv=M√2gh g = 9.825 m/s², h = 10.0 * weight
         }
      }
   momentum.to_string()
}

#[get("/8/weight/<val>")]
async fn get_weight(val: &str) -> String {
   let query_string = format!("https://pokeapi.co/api/v2/pokemon/{}",val.parse::<usize>().unwrap());
   let body = reqwest::get(query_string).await.expect("problem").text();
   let mut long_body: String = body.await.expect("issue");
   let mut split_body: Vec<_> = long_body.split(",").collect::<Vec<_>>();
   let re = Regex::new(r"(:|})").unwrap();
   let popped: Vec<_> = re.split(&split_body.last().unwrap()).collect();
   let mut answer: String = String::new();
   for splitted_str in popped.iter() {
      if let Ok(weight) = splitted_str.parse::<usize>() {
          let ans = weight/10;
          answer = ans.to_string();
   }
   }
   answer
}

#[get("/7/bake")]
fn bake_cookies<'a>(bake: &'a CookieJar<'_>) -> Json<TopResponse> { 
   let mut bstring: HashMap<String, HashMap<String,usize>> = HashMap::new();
   let purs = "";
   let spur: TopResponse = TopResponse { recipe: BakeResponse {flour:0,sugar:0,butter:0,baking_powder:0,chocolate_chips:0}, pantry: BakeResponse { flour:0,sugar:0,butter:0,baking_powder:0,chocolate_chips:0} };
   for b in bake.iter() {
       if b.name() == "recipe" {
          let answer = b.value().as_bytes();
	  let answer_decoded: Vec<u8> = general_purpose::STANDARD_NO_PAD.decode(answer).unwrap();
	  let final_purpose = String::from_utf8(answer_decoded).unwrap();
	  let json_purpose = format!(r#"{}"#, &final_purpose);
	  let spur: Result<TopResponse> = serde_json::from_str(&final_purpose);
	  println!("spur {:?}", &spur);
	}
    }
    Json(spur)
}

#[get("/7/decode")]
fn decode_cookie<'a>(decode: &'a CookieJar<'_>) -> MyResponder {
   let mut cstring: HashMap<String, usize> = HashMap::new();
   for c in decode.iter() {
      if c.name() == "recipe" {
          let ans = c.value().replace(&['='][..],"");
          let answer = c.value().as_bytes();
	  let encoded: String = general_purpose::STANDARD_NO_PAD.encode(b"data");
	  //println!("all encoded {:?}", encoded);
	  let gen_purpose: Vec<u8>= general_purpose::STANDARD_NO_PAD.decode(answer).unwrap();
	  let final_purpose = String::from_utf8(gen_purpose.to_vec()).unwrap();
	  let s = final_purpose.replace(&['(', ')', '{', '}', '\"', '.', ';', '\''][..], "");
	  let re = Regex::new(r"(:|,)").unwrap();
	  let splitted_string: Vec<_> = re.split(&s).collect();
	  let mut ingredient = "";
	  let mut weight: usize = 0;
	  for splitted_str in splitted_string.iter() {
	     if let Err(_) = splitted_str.parse::<usize>() {
	         ingredient = splitted_str;
		 println!("ingredient {:?}", &ingredient);
		 }
	     else {
		 weight = splitted_str.parse::<usize>().unwrap();
		 }
	     cstring.insert(ingredient.to_string(), weight);
	  }	 
	  }
      }
   MyResponder { status: Status::Ok, data: cstring }
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
   let nore = Regex::new(r"elf on (a|that) (shelf.|shelf)").unwrap();
   let mut shelves: Vec<_> = nore.split(strelves).collect();
   shelves.retain(|&item| item!="");
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
   MyResponder { status: Status::Ok, data: jstring }
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
    let rocket = rocket::build().mount("/", routes![index, fake, integer_this, calc_strength, elf_count, decode_cookie, bake_cookies, get_weight, get_momentum]);

    Ok(rocket.into())
}