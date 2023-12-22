use rocket::{get, routes};
use rocket::http::Status;
use std::path::PathBuf;
//use rocket::serde::{Deserialize, Serialize};
use rocket::post;
use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response};
use rocket::http::ContentType;
use rocket::response::status;
use rocket::fs::TempFile;
use chrono::{DateTime, Utc};
use rocket::State;
use std::fs::read;
use sha256::{digest, try_digest};
use itertools::Itertools;
use rocket::serde::json::Value;
use rocket::fs;
use handlebars::Handlebars;
use password_rules_parser::{parse_password_rules, CharacterClass};
use std::sync::Mutex;
use rocket::futures::StreamExt;
use std::time::SystemTime;
use sqlx::Row;
use regex::Regex;
use base64::{engine, alphabet, Engine as _};
use rocket::http::{Cookie, CookieJar};
use base64::engine::general_purpose;
use serde_json::json;
use sqlx::Column;
use sqlx::ValueRef;
use rocket::form::prelude::ErrorKind::Int;
use std::collections::HashMap;
use std::collections::BTreeMap;
use rocket::serde::json::Json;
use rocket::Data;
use rocket::fs::relative;
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::form::{Form, FromForm};
use serde_json::Result;
use rocket::response::Responder;
use rocket::response::status::BadRequest;
use rocket::response::content;
use rocket::http::Method;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, PgPool};
use sqlx::query;
use num_traits::cast::AsPrimitive;
use image::GenericImageView;
use image::Rgba;
use sqlx::Postgres;
use shuttle_persist::PersistInstance;
use shuttle_runtime::CustomError;

pub fn xor(vc: Vec<i32>) -> i32 {
   let mut item: i32 = 0;
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
  pub data: HashMap<String, i32>,
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


#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyBakeResponder {
  pub status: Status,
  pub data: HashMap<(String, u32), HashMap<String, i64>>,
}

impl<'r> Responder<'r,'r> for MyBakeResponder {
    fn respond_to(self, _: &rocket::Request<'_>) -> response::Result<'static> {
          println!("data is in bake responder {:?}", &self.data);
	  let body = serde_json::to_string_pretty(&self.data).unwrap();
	  println!("body is {:?}", &body);
          let body = json!(self.data).to_string();
          Response::build()
            .status(self.status)
            .header(ContentType::JSON)
	    .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}


#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct TopResponse {
   recipe: BakeResponse,
   pantry: BakeResponse,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct EndResponse {
   cookies: i64,
   pantry: BakeResponse,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct BakeResponse {
   flour: i64,
   sugar: i64,
   butter: i64,
   #[serde(rename = "baking powder")]
   baking_powder: i64,
   #[serde(rename = "chocolate chips")]
   chocolate_chips: i64,
}

pub fn get_regexchar_matches(string_search: &str, reg_char: &Regex, search_char: &str, match_len: usize, expected_string: &str) -> i32 {
    let mut counter: i32 = 0;
    for (i,_) in string_search.char_indices() {
        if let Some(cap) = reg_char.captures(&string_search[i..]) {
            if cap[1].to_string() == search_char {
                if i+match_len <= string_search.len() {
                    if &string_search[i..i+match_len] == expected_string {
                        counter+=1;
                        }
                }
            }
        }
    }
    counter
}

#[derive(Debug)]
pub struct Pokemon {
   weight: usize,
}

#[derive(Debug)]
#[derive(FromForm)]
struct Image<'f> {
    image: TempFile<'f>,
}

struct MyState {
   pool: PgPool,
}

struct Memory {
   data: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct MyString {
  string_val: String,
  datetime: SystemTime,
}

#[derive(Serialize, FromRow, Debug)]
struct Todo {
    pub id: i32,
    pub note: String,
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

type OrdersNew = Vec<Orders>;

#[derive(sqlx::Type)]
#[derive(FromForm)]
#[derive(Serialize, FromRow, Debug, Clone, Deserialize)]
struct Orders {
   pub id: i32,
   pub region_id: i32,
   pub gift_name: String,
   pub quantity: i64,
}

type RegionsNew = Vec<Regions>;

#[derive(sqlx::Type)]
#[derive(FromForm)]
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
struct Regions {
   pub id: i32,
   pub name: String,
}


#[derive(Deserialize)]
struct NiceElf {
    input: String,
}

fn is_nice_(chr: u8) -> bool {
    matches!(chr as char, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

/// Generic moving window iterator over sequences to return k-mers
///
/// Iterator returns slices to the original data.
 #[derive(Debug)]
pub struct Kmers<'a> {
    k: u8,
    start_pos: usize,
    buffer: &'a [u8],
}

impl<'a> Kmers<'a> {
    pub fn new(buffer: &'a [u8], k: u8) -> Self {
        Kmers {
            k,
            start_pos: 0,
            buffer,
        }
    }
}

impl<'a> Iterator for Kmers<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_pos + self.k as usize > self.buffer.len() {
            return None;
        }
        let pos = self.start_pos;
        self.start_pos += 1;
        Some(&self.buffer[pos..pos + self.k as usize])
    }
}

#[derive(Debug, Serialize)]
struct Output {
    region: String,
    top_gifts: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
struct TopList {
    region: String,
    top_gifts: Vec<String>,
}

pub fn count_element_function<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
           *result.entry(item).or_insert(0) += 1;
    }

    result
}

#[rocket::post("/22/integers", data="<integer_load>")]
pub fn presents(integer_load: String) -> String {
    //println!("integer_load is {:?}", integer_load);
    let details: Vec<_> = integer_load.split('\n').collect();
    let mut keep: Vec<_> = Vec::new();
    for det in details {
      keep.push(det);
      }
    let counted: HashMap<_,_> = count_element_function(keep);
    let mut actual_result = 0;
    for (k,v) in counted {
       if k != "" {
           //println!("k {:?} v is {:?}", &k, &v);
	   if v == 1 {
	       actual_result = k.parse::<usize>().unwrap();
	       }
	   }
	   
       }
    "🎁".to_string().repeat(actual_result)
}


#[get("/18/regions/top_list/<number>")]
async fn fetch_gifttotals(number: i32, state: &State<MyState>) -> Json<Vec<Output>> {
     let num = if number == 0 { 1 } else { number };
     let result = sqlx::query_as("WITH GiftRank AS (SELECT regions.name AS region, COALESCE(orders.gift_name::text, '') AS top_gift, SUM(orders.quantity) AS totalorders, RANK() OVER (PARTITION BY regions.name ORDER BY SUM(orders.quantity) DESC, orders.gift_name::text) AS rnk FROM regions LEFT JOIN orders ON orders.region_id = regions.id GROUP BY regions.name, orders.gift_name) SELECT region, ARRAY_AGG(top_gift) AS top_gifts FROM GiftRank WHERE rnk <= $1 GROUP BY region;")
     .bind(num);
     let answer: Vec<TopList> = result.fetch_all(&state.pool).await.expect("no thats not working");
     let mut output_result: Vec<Output> = answer
        .into_iter()
        .map(|toplist| Output {
            region: toplist.region,
            top_gifts: toplist.top_gifts,
        })
        .collect();
     for mut outres in output_result.iter_mut() {
	if outres.top_gifts == vec![""] {
	    outres.top_gifts.clear();
	}
	if number == 0 {
	    outres.top_gifts.clear();
	    }
	}
     output_result.sort_by(|a, b| a.region.cmp(&b.region));
     Json(output_result)
}


#[derive(Debug, Serialize, FromRow)]
struct Regional {
    region: String,
    total: i64,
}

#[get("/18/regions/total")]
async fn fetch_regiontotals(state: &State<MyState>) -> Json<Vec<Regional>> {
     let result = sqlx::query_as("SELECT name AS region, SUM(quantity) AS total FROM regions INNER JOIN orders ON orders.region_id = regions.id GROUP BY regions.name;");
     let mut answer: Vec<Regional> = result.fetch_all(&state.pool).await.expect("no thats not working");
     answer.sort_by(|a, b| a.region.cmp(&b.region));
     Json(answer)
}

#[post("/18/orders", format="application/json", data="<data>")]
async fn add_orders18(data: Json<OrdersNew>, state: &State<MyState>) {
    println!("orders data {:?}", &data);
    for ord in data.0.iter() {
        println!("ord.id {:?} ord.region_id {:?}, ord.gift_name {:?}, ord.quantity {:?}", &ord.id, &ord.region_id, &ord.gift_name, &ord.quantity);
        let answer = sqlx::query("INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)")
            .bind(ord.id)
	    .bind(ord.region_id)
	    .bind(ord.gift_name.clone())
	    .bind(ord.quantity)
	    .execute(&state.pool)
	    .await
	    .map_err(|e| BadRequest(e.to_string())).expect("inserting into orders error");
	println!("answer is {:?}", answer);
	}
}


#[post("/18/regions", data="<data>")]
async fn add_regions(data: Json<RegionsNew>, state: &State<MyState>) {
    println!("regions data {:?}", &data);
    for regi in data.0.iter() {
       let answer = sqlx::query("INSERT INTO regions (id, name) VALUES ($1, $2) ")
           .bind(regi.id)
	   .bind(regi.name.clone())
	   .execute(&state.pool)
	   .await
	   .map_err(|e| BadRequest(e.to_string())).expect("inserting into regions error");
       }
}


#[post("/18/reset")]
async fn reset18(state: &State<MyState>) -> Status {
    let _res = sqlx::query("DROP TABLE IF EXISTS regions;")
    .execute(&state.pool)
    .await.unwrap();
    let _res2 = sqlx::query("DROP TABLE IF EXISTS orders;")
    .execute(&state.pool)
    .await.unwrap();
    let _res3 = sqlx::query("CREATE TABLE regions ( id INT PRIMARY KEY, name VARCHAR(50));")
    .execute(&state.pool)
    .await.unwrap();
    let _res4 = sqlx::query("CREATE TABLE orders ( id INT PRIMARY KEY, region_id INT, gift_name VARCHAR(50), quantity INT);")
    .execute(&state.pool)
    .await.unwrap();
    Status::Ok
}

#[derive(Responder)]
enum GameElfResponse {
    #[response(status = 200, content_type = "json")]
    A(String),
    #[response(status = 400, content_type = "json")]
    B(String),
    #[response(status = 406, content_type = "json")]
    F(String),
    #[response(status = 451, content_type = "json")]
    G(String),
    #[response(status = 416, content_type = "json")]
    H(String),
    #[response(status = 426, content_type = "json")]
    I(String),
    #[response(status = 418, content_type = "json")]
    J(String),
}

#[derive(Responder)]
enum NiceElfResponse {
    #[response(status = 200, content_type = "json")]
    A(String),
    #[response(status = 400, content_type = "json")]
    B(String),
}

#[post("/15/game", format="application/json", data="<data>")]
async fn game_nice(data: Json<NiceElf>) -> GameElfResponse {
    let mut nice = 0;
    let re = vec!['0','1','2','3','4','5','6','7','8','9'];
    //let naughty_list = vec!["ab".to_string(),"cd".to_string(),"pq".to_string(),"xy".to_string()];
    let elfstring = &data.input;
    println!("elfstring is {:?}", &elfstring);
    //if elfstring == "2000.23.A joy joy" {
    //    return GameElfResponse::F(format!("{{\"result\":\"naughty\",\"reason\":\"not joyful enough\"}}"));
//	}
    let elfsha = digest(elfstring);
    if elfstring.len() < 8 { return GameElfResponse::B(format!("{{\"result\":\"naughty\",\"reason\":\"8 chars\"}}")); };
    
     // [U+2980..U+2BFF], [1F600..1F64F]";
    let elfchars: Vec<_> = elfstring.chars().collect();
    let has_lowercase = elfstring.chars().any(char::is_lowercase);
    let has_uppercase = elfstring.chars().any(char::is_uppercase);
    if has_lowercase == false {
       return GameElfResponse::B(format!("{{\"result\":\"naughty\",\"reason\":\"more types of chars\"}}"));
       }
    if has_uppercase == false {
       return GameElfResponse::B(format!("{{\"result\":\"naughty\",\"reason\":\"more types of chars\"}}"));
       }
    let mut digcount = 0;
    let mut sumdig = 0;
    let mut is_emojiflag = 0;
    let mut joyflag = 0;
    let mut sandwichflag = 0;
    let mut rangeflag = 0;
    let mut is_emoji = false;
    let mut is_range = false;
    for elf in &elfchars {
       if re.contains(&elf) {
           digcount +=1;
	   }
       is_emoji = match elf {
              '\u{1F600}'..='\u{1F64F}' | // Emoticons
              '\u{1F300}'..='\u{1F5FF}' | // Misc Symbols and Pictographs
              '\u{1F680}'..='\u{1F6FF}' | // Transport and Map
              '\u{1F1E0}'..='\u{1F1FF}' => true, // Flags
              _ => false,
	      };
	if is_emoji == true {
	    is_emojiflag+=1;
	    }
	is_range = match elf {
	    '\u{2980}'..='\u{2BFF}' => true,
	    _ => false
	    };
	//println!("range true {:?}", is_range);
	if is_range == true {
	    rangeflag+=1;
	    }
       }
    let digits: Vec<&str> = elfstring.split(|c: char| !c.is_digit(10)).filter(|&s| !s.is_empty()).collect();
    for dig in digits {
        sumdig+= dig.parse::<u32>().unwrap();
	}
    //println!("digcount {:?}", &digcount);
    //println!("sumdig is {:?}", &sumdig);
    let mut joyflag1 = 1;
    let mut joyflag2 = 1;
    let mut joyflag3 = 1;
    if digcount >= 5 {
       if sumdig == 2023 {
          for elf in elfchars.clone() {
             if elf == 'j' {
	        joyflag1 -= 1;
		}
             if elf == 'o' || elf == '0' && joyflag1 == 0 {
	        joyflag2 -=1;
		}
	     if elf == 'y' && joyflag1 < 1 && joyflag2 == 0 {
	        joyflag3 -=1;
	        println!("allowed in joy");
		}
             //println!("elf is {:?} joyflag is {:?}", &elf, &joyflag);
	     }
	  }
       else {
        return GameElfResponse::B(format!("{{\"result\":\"naughty\",\"reason\":\"math is hard\"}}"));
	}
       }
     else {
        return GameElfResponse::B(format!("{{\"result\":\"naughty\",\"reason\":\"55555\"}}"));
	}
     for elf in elfchars.clone() {
	     if joyflag1 == 0 && joyflag2 == 0 && joyflag3 == 0 {
                     let k_iter = Kmers::new(elfstring.as_bytes(), 3);
                     for kit in k_iter {
		         //println!("kit is {:?}", &String::from_utf8_lossy(kit));
	                 if String::from_utf8_lossy(kit).chars().nth(0) == String::from_utf8_lossy(kit).chars().nth(2) {
	                    //println!("identikit {:?}", kit);
			    if String::from_utf8_lossy(kit).chars().nth(0).expect("problem with nth statement").is_alphabetic() { //|| String::from_utf8(kit.to_vec()).unwrap().chars().nth(0).unwrap() == '0'  {
			        sandwichflag+=1;
			    }     
	               }
		      }
		      //println!("sandwich flag is {:?}", sandwichflag);
		      if sandwichflag == 0 {
		           return GameElfResponse::G(format!("{{\"result\":\"naughty\",\"reason\":\"illegal: no sandwich\"}}"));
			   }
		      if rangeflag < 1 {
		          return GameElfResponse::H(format!("{{\"result\":\"naughty\",\"reason\":\"outranged\"}}"));
			  }
		      if is_emojiflag < 1 {
		          return GameElfResponse::I(format!("{{\"result\":\"naughty\",\"reason\":\"😳\"}}"));
			  }
	              if !elfsha.ends_with("a") {
		          nice+=1;
		          return GameElfResponse::J(format!("{{\"result\":\"naughty\",\"reason\":\"not a coffee brewer\"}}"));
		          }
		  }
		 else {
	              return GameElfResponse::F(format!("{{\"result\":\"naughty\",\"reason\":\"not joyful enough\"}}"));
	              }
	     }
    if nice > 0 {
        return GameElfResponse::B(format!("{{\"result\":\"naughty\"}}"))
	
	}
    else {
        return GameElfResponse::A(format!("{{\"result\":\"nice\",\"reason\":\"that's a nice password\"}}"))
	}
}

#[post("/15/nice", format="application/json", data="<data>")]
async fn validate_nice(data: Json<NiceElf>) -> NiceElfResponse {
    let mut nice = 0;
    let re = vec!['a','e','i','o','u','y'];
    let naughty_list = vec!["ab".to_string(),"cd".to_string(),"pq".to_string(),"xy".to_string()];
    let elfstring = &data.input;
    let mut flag = 0;
    println!("elfstring is {:?}", &elfstring);
    let elfchars: Vec<_> = elfstring.chars().collect();
    let mut vowelcount = 0;
    for elf in elfchars {
       if re.contains(&elf) {
           vowelcount+=1;
	   }
       }
    if vowelcount >= 3 {
        let k_iter = Kmers::new(elfstring.as_bytes(), 2);
        for kit in k_iter {
	     for naught in &naughty_list {
	                    if &String::from_utf8(kit.to_vec()).unwrap() == naught {
	                        nice+=1;
		                }
	                    }
	     if String::from_utf8(kit.to_vec()).unwrap().chars().nth(0) == String::from_utf8(kit.to_vec()).unwrap().chars().nth(1) {
	                println!("identikit {:?}", &String::from_utf8(kit.to_vec()).unwrap().chars().nth(0));
			flag +=1;
			if !String::from_utf8(kit.to_vec()).unwrap().chars().nth(0).expect("problem with nth statement").is_alphabetic() {
			    nice+=1;
			    }
	                
	               }
	     }
        }
    if nice > 0 || flag < 1  {
        NiceElfResponse::B(format!("{{\"result\":\"naughty\"}}"))
	}
    else {
        NiceElfResponse::A(format!("{{\"result\":\"nice\"}}"))
	}
}


#[get("/13/orders/total")]
async fn fetch_ordertotal(state: &State<MyState>) -> String {
  //   let result = sqlx::query_scalar::<_, Orders>("SELECT SUM(quantity) FROM orders")
     let result = sqlx::query("SELECT SUM(quantity) FROM orders;")
        .fetch_one(&state.pool)
	.await
	.map_err(|e| BadRequest(e.to_string())).expect("sum total error");
     let answer = result.try_get::<i64, _>(0).unwrap();
     format!("{{\"total\":{:?}}}", answer)
}

#[post("/13/orders", data="<data>")]
async fn add_orders(data: Json<OrdersNew>, state: &State<MyState>) {
    for ord in data.0.iter() {
        let answer = sqlx::query("INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)")
            .bind(ord.id)
	    .bind(ord.region_id)
	    .bind(ord.gift_name.clone())
	    .bind(ord.quantity)
	    .execute(&state.pool)
	    .await
	    .map_err(|e| BadRequest(e.to_string())).expect("inserting into orders error");
	}
}
	
    

#[post("/13/reset")]
async fn reset(state: &State<MyState>) -> Status {
    let _res = sqlx::query("DROP TABLE IF EXISTS orders;")
    .execute(&state.pool)
    .await.unwrap();
    let _res2 = sqlx::query("CREATE TABLE orders ( id INT PRIMARY KEY, region_id INT, gift_name VARCHAR(50), quantity INT);")
    .execute(&state.pool)
    .await.unwrap();
    Status::Ok
}

#[derive(Deserialize)]
struct Task14 {
   content: String,
}

//#[post("/14/unsafe", type="application/json", data="<content>")]
//async fn unsafe_provide(data: Json<Task14>) -> &'static str {
 //   let mut handlebars = Handlebars::new();
  //  let source = "{{ content }}";
 //   println!(
   //    "{}",
 //      handlebars.render_template("{{content}}", &json!({"content":data.content}))?);
    //let mut datahash = HashMap::new();
    //datahash.insert("content", data.content);
    //println!("{}", handlebars.render("content", &datahash).unwrap());
//}
    

#[get("/13/sql")]
async fn get_todo(state: &State<MyState>) -> String {
    //let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = 5")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(e.to_string())).expect("serious error");
    todo.note 
}

#[post("/13/add", data="<data>")]
async fn add_todo(data: Json<TodoNew>,state: &State<MyState>,) {
    let id = 5;
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (id, note) VALUES (5, $2) returning id, note;")
        .bind(id)
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(e.to_string())).expect("cant add to db");
}

#[rocket::post("/12/save/<query>")]
async fn store_data(query: &str, memory: &State<Memory>) -> String {
   let current_time = SystemTime::now();
   let datetime: DateTime<Utc> = current_time.into();
   let datestring = datetime.to_rfc2822();
   let mut data = memory.data.lock().unwrap();
   data.insert(query.to_string(), datestring);
   "data stored".to_string()
}

#[rocket::get("/12/load/<key>")]
fn retrieve_data(key: String, memory: &State<Memory>) -> String {
   let current_time = SystemTime::now();
   let datetime: DateTime<Utc> = current_time.into();
   let datestring = datetime.to_rfc2822();
   let nowdate = DateTime::parse_from_rfc2822(&datestring).map(|dt| dt.with_timezone(&Utc)).unwrap();
   let data = memory.data.lock().unwrap();
   match data.get(&key) {
      Some(value) => {
           let orig_datetime = value;
	   let origdate = DateTime::parse_from_rfc2822(&orig_datetime).map(|dt| dt.with_timezone(&Utc)).unwrap();
	   let result = nowdate - origdate;
           result.num_seconds().to_string()   
	   },
	   None => "no data found for that string".to_string(),
	   }
}

#[rocket::post("/11/red_pixels", data = "<image>")]
async fn red_pixels(mut image: Form<Image<'_>>) -> String {
   image.image.persist_to("assets/image.png").await.expect("no image");
   let raw_image_data = image::open(Path::new("assets/image.png")).unwrap();
   let (width, height) = raw_image_data.dimensions();
   let mut red_pixel_count = 0;
   let mut checked_overflow = 0;
   for pixel in raw_image_data.pixels() {
        // println!("pixel 2 is {:?} {:?}", pixel.2.0[0] as u32, pixel.2.0[1] as u32 + pixel.2.0[2]) as u32;
	 if let Some(mut checked_overflow) = pixel.2.0[1].checked_add(pixel.2.0[2]) {
	      checked_overflow = pixel.2.0[1] + pixel.2.0[2];
	      if pixel.2.0[0] > checked_overflow { red_pixel_count +=1 };
	 } else {
	    checked_overflow = 255;
	    if pixel.2.0[0] > checked_overflow {
	       red_pixel_count += 1;
	       }
	    }
      }
   red_pixel_count.to_string()
}

#[rocket::get("/11/assets/<path>")]
pub async fn serve(mut path: PathBuf) -> Option<NamedFile> {
    path.set_extension("png");
    let mut path = Path::new(relative!("assets")).join(path);
    if path.is_dir() {
        path.push("decoration.png");
    }
    NamedFile::open(path).await.ok()
}

#[get("/8/drop/<val>")]
async fn get_momentum(val: &str) -> String {
   let query_string = format!("https://pokeapi.co/api/v2/pokemon/{}",val.parse::<usize>().unwrap());
   let body = reqwest::get(query_string).await.expect("problem with the reqwest call to API").text();
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
   let query_string = format!("https://pokeapi.co/api/v2/pokemon/{}",val.parse::<f64>().unwrap());
   let body = reqwest::get(query_string).await.expect("problem").text();
   let mut long_body: String = body.await.expect("issue");
   let mut split_body: Vec<_> = long_body.split(",").collect::<Vec<_>>();
   let re = Regex::new(r"(:|})").unwrap();
   let popped: Vec<_> = re.split(&split_body.last().unwrap()).collect();
   let mut answer: String = String::new();
   for splitted_str in popped.iter() {
      if let Ok(weight) = splitted_str.parse::<f64>() {
          let ans = weight/10.0;
          answer = ans.to_string();
   }
   }
   answer
}

#[get("/7/bake")]
fn bake_cookies<'a>(bake: &'a CookieJar<'_>) -> Json<EndResponse> { 
   let pantry_items: Vec<&str> = vec!["flour", "sugar", "butter", "baking_powder", "chocolate_chips"];
   let mut flour_val: i64 = 0;
   let mut flour_val_temp: i64 = 0;
   let mut sugar_val: i64 = 0;
   let mut sugar_val_temp: i64 = 0;
   let mut butter_val: i64 = 0;
   let mut butter_val_temp: i64 = 0;
   let mut baking_powder_val: i64 = 0;
   let mut baking_powder_temp: i64 = 0;
   let mut chocolate_chips_val: i64 = 0;
   let mut chocolate_chips_temp: i64 = 0;
   let mut cookies = 0;
   for b in bake.iter() {
       if b.name() == "recipe" {
          let answer = b.value().as_bytes();
	  let answer_decoded: Vec<u8> = general_purpose::STANDARD.decode(answer).unwrap();
	  let final_purpose = String::from_utf8(answer_decoded).unwrap();
	  let json_purpose = format!(r#"{}"#, &final_purpose);;
	  let spur: Result<TopResponse> = serde_json::from_str(&final_purpose);
	  let pantry = &spur.as_ref().expect("problem with pantry").pantry;
	  let recipe = &spur.as_ref().expect("issue with recipe").recipe;
	  flour_val = pantry.flour as i64;
	  sugar_val = pantry.sugar as i64;
	  butter_val = pantry.butter as i64;
	  baking_powder_val = pantry.baking_powder as i64;
	  chocolate_chips_val = pantry.chocolate_chips as i64;
	  println!("recipe {:?}", &recipe);
	  let mut cnt = 0;
	  'outer: loop {
	     for item in pantry_items.iter() {
	        match item {
	            &"flour" => { if flour_val - recipe.flour as i64 >= 0 { flour_val_temp = flour_val - recipe.flour as i64; } else { break 'outer; }},
		    &"sugar" => { if sugar_val - recipe.sugar as i64 >= 0 { sugar_val_temp = sugar_val - recipe.sugar as i64; } else {break 'outer; }},
		    &"butter" => { if butter_val - recipe.butter as i64 >= 0 { butter_val_temp = butter_val - recipe.butter as i64; } else {break 'outer;}},
		    &"baking_powder" => { if baking_powder_val - recipe.baking_powder as i64 >= 0 { baking_powder_temp = baking_powder_val - recipe.baking_powder as i64; } else { break 'outer; }},
		    &"chocolate_chips" => { if chocolate_chips_val - recipe.chocolate_chips as i64 >= 0 { chocolate_chips_temp = chocolate_chips_val - recipe.chocolate_chips as i64; } else { break 'outer; }},
		    _ => break 'outer,
	        }
	     }
	     flour_val = flour_val_temp;
	     sugar_val = sugar_val_temp;
	     butter_val = butter_val_temp;
	     baking_powder_val = baking_powder_temp;
	     chocolate_chips_val = chocolate_chips_temp;
	     cnt+=1;     
          };

	  cookies = cnt;
    }
    }
    println!("cookies {:?}", &cookies);
    println!("flour_val {:?} sugar_val {:?}, butter_val {:?} baking {:?} choc {:?}", &flour_val, &sugar_val, &butter_val, &baking_powder_val, &chocolate_chips_val);
    Json(EndResponse { cookies: cookies, pantry: BakeResponse { flour: flour_val, sugar: sugar_val, butter: butter_val, baking_powder: baking_powder_val, chocolate_chips: chocolate_chips_val } })
}


#[get("/7/decode")]
fn decode_cookie<'a>(decode: &'a CookieJar<'_>) -> String {
   let mut final_purpose = String::new();
   for c in decode.iter() {
      if c.name() == "recipe" {
          println!("c value is {:?}", &c.value());
          let answer = c.value().as_bytes();
	  let gen_purpose: Vec<u8> = general_purpose::STANDARD.decode(answer).unwrap();
	  final_purpose = String::from_utf8(gen_purpose).unwrap();
	  }
	  }
  final_purpose
}


#[post("/6", data="<elves>")]
pub fn elf_count(elves: &str) -> MyResponder {
   let re = Regex::new(r"elf").unwrap();
   println!("so elves are {:?}", &elves);
   let strelves = elves.clone();
   let noelves = strelves.clone();
   let result = re.captures_iter(&strelves);
   let re = Regex::new("^(e)").unwrap();
   let shre = Regex::new("^(s)").unwrap();
   let mut shelf_count = get_regexchar_matches(elves, &re, "e", 14, "elf on a shelf");
   let mut noelf_count = get_regexchar_matches(elves, &shre, "s", 5, "shelf");
   println!("shelf count {:?}", shelf_count);
   println!("noelf count {:?}", noelf_count - shelf_count);
   //let mut shelves: Vec<_> = nore.matches(&strelves).into_iter().collect();
   //shelves.retain(|&item| item!="");
   //let is_punctuation = |c: char| c.is_ascii_punctuation();
   //if !elves.trim_end_matches(is_punctuation).ends_with("elf on a shelf") {
   //      println!("shelf match");
   //      noelf_count -=1;
   //	 }
   let mut jstring: HashMap<String, i32> = HashMap::new(); 
   let mut elf_count: i32 = 0;
   for res in result {
       elf_count+=1;
       }
   jstring.insert("elf".to_string(), elf_count);
   jstring.insert("elf on a shelf".to_string(), shelf_count);
   jstring.insert("shelf with no elf on it".to_string(), noelf_count - shelf_count);
  // if elf_count > 0 {
    //           jstring.insert("elf".to_string(), elf_count);
      //         }
        //  if shelf_count > 0 {
          //     jstring.insert("elf on a shelf".to_string(), shelf_count);
            //   }
       //   if noelf_count > 0 {
         //      jstring.insert("shelf with no elf on it".to_string(), noelf_count);
           //    }
   //let answer = serde_json::from_str(jstring).expect("none such");
   
   MyResponder { status: Status::Ok, data: jstring }
}

#[derive(Debug)]
#[derive(FromForm)]
#[derive(Deserialize)]
struct params {
   #[field(default = 0)]
   offset: usize,
   #[field(default = 1000)]
   limits: usize,
   #[field(default = 1000)]
   limit: usize,
   #[field(default = 0)]
   split: usize,
}

#[derive(Debug)]
#[derive(FromForm)]
#[derive(Deserialize)]
struct grinchnames {
    fullnames: Vec<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum MyReturn {
    Single(Vec<String>),
    Double(Vec<Vec<String>>),
}

#[post("/5?<params..>", format = "application/json", data="<grinchnames>")]
pub fn grinch(params: params, grinchnames: Json<Vec<String>>) -> status::Custom<Json<MyReturn>> {
    let offsety = params.offset;
    let limity = params.limits;
    let limitsy = params.limit;
    let splity = params.split;
    println!("actual params {:?}", &params);
    //let offsetlimits: Vec<_> = details.split('?').collect();
    println!("details is {:?}", grinchnames);
    println!("offset {:?} limits {:?} split {:?}", &offsety, &limity, &splity);
    let fullnames = grinchnames;
    if splity == 0 {
        let mut results: Vec<_> = Vec::new();
        if limitsy >= 0 && limitsy < fullnames.len() {
	    println!("in top");
            results = fullnames[offsety..offsety+limitsy].to_vec();
	    }
        else {
	    println!("in sec");
            results = fullnames[offsety..fullnames.len()].to_vec();
	     }
	return status::Custom(Status::Ok, Json(MyReturn::Single(results.into())));
   }
   else {
      let dst: &Vec<Vec<_>> = &fullnames[offsety..fullnames.len()].chunks(splity).map(|s| s.to_vec()).collect();
      let mut answer = Vec::new();
      for (idx, d) in dst.iter().enumerate() {
         println!("idx {:?} d {:?}", &idx, &d);
	 }
      println!("limity is {:?} offset minus {:?}", limitsy, limitsy - offsety);
      if limity > 0 && limitsy < dst.len() {
         answer = dst[0..limitsy].to_vec();
	 println!("answer is {:?}", answer);
      }
      else {
         answer = dst.to_vec();
	 println!("last bit");
	 }
      return status::Custom(Status::Ok, Json(MyReturn::Double(answer.clone().into())));

      }
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
   let numbers: Vec<i32> = it.to_str().unwrap().split('/').filter_map(|s| s.parse().ok()).collect();
   let answer = xor(numbers).pow(3).to_string();
   answer
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
       .await.map_err(CustomError::new)?;

    let state = MyState { pool };
    let rocket = rocket::build()
     //Memory { data:Mutex::new(HashMap::new()) }, state)
    .mount("/", routes![presents, index, validate_nice, fetch_gifttotals, fetch_ordertotal, fetch_regiontotals, add_orders18, reset18, add_regions, fake, add_todo, get_todo, reset, add_orders, serve, store_data, retrieve_data, integer_this, calc_strength, elf_count, game_nice, decode_cookie, bake_cookies, grinch, get_weight, get_momentum, red_pixels])
    .manage(state)
    .manage( Memory { data:Mutex::new(HashMap::new()) });
    

    Ok(rocket.into())
}