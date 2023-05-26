// fn main() {
//     println!("Hello, world!");
// }

use std::env;
use std::io::{Error, Write};
use std::process::{Command, Stdio};

fn download_latest_backup(app_name: &str, api_key: &str) -> Result<(), Error> {
    env::set_var("HEROKU_API_KEY", api_key);

    // Herokuのpg:backups:urlコマンドを実行してダウンロードURLを取得
    let output = match Command::new("heroku")
        .arg("pg:backups:url")
        .arg("--app")
        .arg(app_name)
        .output()
    {
        Ok(o) => o,
        Err(e) => panic!("failed to execute process: {}", e),
    };
    println!("output: {:?}", output);
    //
    // // コマンドの出力をダウンロードURLとして取得
    // let download_url = match String::from_utf8(output.stdout) {
    //     Ok(s) => s,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // .unwrap().trim().to_string();
    // println!("download_url: {}", download_url);

    // // curlコマンドを使用してダウンロード
    // Command::new("curl")
    //     .arg("-o")
    //     .arg("latest.dump")
    //     .arg(download_url)
    //     .spawn()?;

    Ok(())
}

fn main() {
    let api_key = env::var("HEROKU_API_KEY").unwrap_or_else(|_| panic!("HEROKU_API_KEY not found"));
    let app_name =
        env::var("HEROKU_APP_NAME").unwrap_or_else(|_| panic!("HEROKU_APP_NAME not found"));

    match download_latest_backup(&app_name, &api_key) {
        Ok(()) => println!("Download complete!"),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
