use std::io::Error;
use std::process::Command;
use std::{env, panic};

use std::fs::File;
use std::io::copy;

use reqwest::blocking::get;

fn fetch_latest_s3_backup_url(app_name: &str) -> Result<String, Error> {
    let output = match Command::new("heroku")
        .arg("pg:backups:url")
        .arg("--app")
        .arg(app_name)
        .output()
    {
        Ok(o) => o,
        Err(e) => panic!("failed to execute process: {}", e),
    };
    let download_url = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => panic!("failed to execute process: {}", e),
    };
    let trimed_url = download_url.trim().to_string();
    println!("trimed_url: {}", trimed_url);

    Ok(trimed_url)
}

fn download_backup_file(url: &str) -> Result<(), Error> {
    let mut file = File::create("backup.dump").expect("Failed to create file");
    let mut response = get(url).expect("request failed");
    copy(&mut response, &mut file).expect("copy failed");
    Ok(())
}

fn main() {
    let api_key = env::var("HEROKU_API_KEY").unwrap_or_else(|_| panic!("HEROKU_API_KEY not found"));
    env::set_var("HEROKU_API_KEY", api_key);

    let app_name =
        env::var("HEROKU_APP_NAME").unwrap_or_else(|_| panic!("HEROKU_APP_NAME not found"));

    let download_url = match fetch_latest_s3_backup_url(&app_name) {
        Ok(url) => url,
        Err(e) => panic!("failed to fetch latest backup url: {}", e),
    };
    download_backup_file(&download_url).unwrap();
}
