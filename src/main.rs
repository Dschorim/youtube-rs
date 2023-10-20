use urlencoding::encode;
use serde_json::Value;
use serde_json::json;

fn main() {
    let youtube_base_url: String = "https://youtube.com".to_owned();
    let search_url: &str = "/results?search_query=";
    let search_query: &str = "TFT Set 9";
    let full_url: String = youtube_base_url + search_url + &encode(search_query);
    
    let res = do_throttled_request(&full_url).expect("failure");
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

    let serde_json::Value::Array(ref con) = content["contents"] else { todo!() };
    for c in con {
        if c["videoRenderer"] == json!(null) {
            continue;
        }
        println!("{}", c["videoRenderer"]["title"]["runs"][0]["text"]);
        println!("{}", c["videoRenderer"]["videoId"]);
    }

    let video_id = &content["contents"][0]["videoRenderer"]["videoId"];
    let video_length = &content["contents"][0]["videoRenderer"]["lengthText"]["simpleText"];
    let video_channel_url = &content["contents"][0]["videoRenderer"]["longBylineText"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["canonicalBaseUrl"];
    let video_channel_name = &content["contents"][0]["videoRenderer"]["longBylineText"]["runs"][0]["text"];
    let video_age = &content["contents"][0]["videoRenderer"]["publishedTimeText"]["simpleText"];
    let video_viewcount = &content["contents"][0]["videoRenderer"]["shortViewCountText"]["simpleText"];
    let video_thumbnail = &content["contents"][0]["videoRenderer"]["thumbnail"]["thumbnails"][0]["url"];
    let video_title = &content["contents"][0]["videoRenderer"]["title"]["runs"][0]["text"];
}


pub fn do_throttled_request(url: &str) -> std::result::Result<std::string::String, reqwest::Error> {
    // See the real code for the throttling - it's omitted here for clarity
    let response = reqwest::blocking::get(url)?;
    response.text()
}
