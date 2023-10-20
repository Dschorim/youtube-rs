use urlencoding::encode;
use serde_json::Value;

fn main() {
    let youtube_base_url: String = "https://youtube.com".to_owned();
    let search_url: &str = "/results?search_query=";
    let search_query: &str = "TFT Set 9";
    let full_url: String = youtube_base_url + search_url + &encode(search_query);


    // println!("{full_url}");
    
    let res = do_throttled_request(&full_url).expect("failure");
    let content;
    match res.split("itemSectionRenderer\":").collect::<Vec<_>>().into_iter().nth(1) {
        Some(x) => content = x,
        None => content = "nada",
    }
    // println!("{content}");
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
    println!("{}", content["contents"][1]);

}


pub fn do_throttled_request(url: &str) -> std::result::Result<std::string::String, reqwest::Error> {
    // See the real code for the throttling - it's omitted here for clarity
    let response = reqwest::blocking::get(url)?;
    response.text()
}
