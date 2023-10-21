#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use urlencoding::encode;
use serde_json::Value;
use serde_json::json;
use std::sync::Mutex;
use std::time::Instant;
use lazy_static::lazy_static;


lazy_static! {
    static ref LAST_REQUEST_MUTEX: Mutex<Option<Instant>> = Mutex::new(None);
    static ref REQUEST_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search")]
fn search() -> &'static str {
    "This is a search"
}

pub struct Video {
    video_id: String,
    video_length: String,
    video_channel_url: String,
    video_channel_name: String,
    video_age: String,
    video_viewcount: String,
    video_thumbnail: String,
    video_title: String,
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search])
        .launch();
    let full_url = get_youtube_search_query("TFT Set 9");
    let content = get_json_from_youtube(&full_url);
    let videos: Vec<Video> = get_videos_from_json(content);
    match videos.get(0) {
        Some(x) => println!("{}", x.video_title),
        None => todo!()
    }
    
}

pub fn get_youtube_search_query(search_query: &str) -> String {
    let youtube_base_url: String = "https://youtube.com".to_owned();
    let search_url: &str = "/results?search_query=";
    return youtube_base_url + search_url + &encode(search_query);
}

pub fn do_throttled_request(url: &str) -> Result<String, reqwest::Error> {
    let mut last_request_mutex = LAST_REQUEST_MUTEX.lock().unwrap();
    let last_request = last_request_mutex.take();
    let now = Instant::now();
    if let Some(last_request) = last_request {
        let duration = now.duration_since(last_request);
        if duration < *REQUEST_DELAY {
            std::thread::sleep(*REQUEST_DELAY - duration);
        }
    }
    let response = reqwest::blocking::get(url)?;
    last_request_mutex.replace(now);
    response.text()
}

pub fn get_json_from_youtube(full_url: &str) -> Value {
    let res = do_throttled_request(full_url).expect("failure");
    let content;
    match res.split("itemSectionRenderer\":").collect::<Vec<_>>().into_iter().nth(1) {
        Some(x) => content = x,
        None => content = "nada",
    }

    let mut content_vec: Vec<char> = Vec::new();
    let mut count_open_curly_brackets = 0;
    for c in content.chars(){
        if c == '{' {
            count_open_curly_brackets += 1;
        } else if c == '}' {
            count_open_curly_brackets -= 1;
        }
        content_vec.push(c);
        if count_open_curly_brackets == 0 {
            break;
        }
    }
    let content_str: String = content_vec.into_iter().collect();

    let content: Value;
    match serde_json::from_str(&content_str) {
        Ok(x) => content = x,
        Err(_) => todo!()
    }

    return content;
}

pub fn get_videos_from_json(content: Value) -> Vec<Video> {
    let serde_json::Value::Array(ref con) = content["contents"] else { todo!() };
    let mut videos: Vec<Video> = Vec::new();
    for c in con {
        if c["videoRenderer"] == json!(null) {
            continue;
        }
        videos.push(
            Video {
                video_id: c["videoRenderer"]["videoId"].to_string(),
                video_length: c["videoRenderer"]["lengthText"]["simpleText"].to_string(),
                video_channel_url: c["videoRenderer"]["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"].to_string(),
                video_channel_name: c["videoRenderer"]["longBylineText"]["runs"][0]["text"].to_string(),
                video_age: c["videoRenderer"]["publishedTimeText"]["simpleText"].to_string(),
                video_viewcount: c["videoRenderer"]["shortViewCountText"]["simpleText"].to_string(),
                video_thumbnail: c["videoRenderer"]["thumbnail"]["thumbnails"][0]["url"].to_string(),
                video_title: c["videoRenderer"]["title"]["runs"][0]["text"].to_string()
            }
        );
    }
    return videos;
}
