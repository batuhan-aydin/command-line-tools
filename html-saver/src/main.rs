use std::{path::Path, fs::OpenOptions, io::Write, time::{Duration, SystemTime, UNIX_EPOCH}};
use clap::{Command, Arg};

fn main() {
    let matches = cli().get_matches();
    let url = matches.value_of("url").expect("Url is not provided");
    let file_path = matches.value_of("path").unwrap_or("");
    let interval = matches.value_of("interval").unwrap_or("1").parse::<u64>().unwrap();
    let sleep_time = matches.value_of("sleep").unwrap_or("0").parse::<u64>().unwrap();

    println!("sleeping for {} minutes...", sleep_time);
    std::thread::sleep(Duration::from_secs(sleep_time * 60));
    loop {
        let response = reqwest::blocking::get(url).unwrap();
        let content = response.text().unwrap();

        let time = get_time();
        let path_str = format!("{}-{}.html", file_path, time);
        let path = Path::new(&path_str);

        write_to_file(&content, &path);
        println!("sleeping for {} minutes...", interval);
        std::thread::sleep(Duration::from_secs(interval * 60));
    }
}


fn cli() -> Command<'static> {
    Command::new("html-saver")
                       .about("Saving html content to file")
                       .arg_required_else_help(true)
                       .arg(
                           Arg::new("url")
                           .short('u')
                           .long("url")
                           .help("The url to get html content and save")
                           .takes_value(true)
                       )
                       .arg(
                           Arg::new("path")
                           .short('p')
                           .long("path")
                           .help("Path to save")
                           .takes_value(true)
                       )
                       .arg(
                           Arg::new("interval")
                           .short('i')
                           .long("interval")
                           .help("Sends request every interval * 1 minutes")
                           .takes_value(true)
                        )
                       .arg(
                           Arg::new("sleep")
                           .short('s')
                           .long("sleep")
                           .help("Sleeps before starting for slee * 1 minutes")
                           .takes_value(true)
                        )
}

fn write_to_file(content: &str, path: &Path) {
    let mut file = match  OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path) {
        Ok(f) => f,
        Err(e) => panic!("error while opening file: {}", e)
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("The file is written to the {:?}", &path),
        Err(e) => panic!("Error while writing to file: {}", e)
    }

}

fn get_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}