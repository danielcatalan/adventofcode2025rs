use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
};
use ureq;

#[derive(Deserialize)]
struct CookieSettings {
    content: String,
}

fn main() {
    const INPUT_FILE: &str = "input/input.txt";

    // Check if input already exists
    {
        let f = File::open(INPUT_FILE);
        if let Ok(_file) = f {
            return; // file already exists
        }
    }

    let year = 2025;
    let day = 1;
    let url_day = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    // Read Cookies
    let s = fs::read_to_string("../cookie.toml").unwrap();
    let cookie: CookieSettings = toml::from_str(&s).unwrap();

    // Get input though URL
    let header_value = format!("session={}; Domain=.adventofcode.com", cookie.content);

    let body = ureq::get(url_day.as_str())
        .header("Cookie", header_value.as_str())
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();

    let mut f = File::create(INPUT_FILE).unwrap();

    f.write_all(body.as_bytes()).unwrap();
}
