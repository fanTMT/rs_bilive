use clap::{Command, Parser, arg, command};
use dotenv::dotenv;
use serde_json::json;
use std::env;

//%7B%22albumName%22:%22%22,%22interval%22:%22%22,%22name%22:%22%E7%A8%BB%E9%A6%99%22,%22playLater%22:false,%22singer%22:%22%22%7D

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Specify your name
    #[arg(short, long)]
    code: Option<String>,
}
fn main() {
    dotenv().ok();

    let cli = Command::new("myapp")
        .version("1.0.0")
        .author("番茄是水果")
        .about("this is the short about")
        .long_about("this is the long about")
        .arg(
            arg!(-c --code <CODE>)
                .value_parser(clap::value_parser!(String))
                .help("Specify your name"),
        )
        .get_matches();
    let a = cli.get_one::<String>("code").unwrap();
    println!("code;{}", a);
    let cli = Cli::parse();
    let a = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let b = match env::var("CODE") {
        Ok(env) => env,
        Err(_) => match cli.code {
            Some(s) => s,
            None => {
                eprintln!("请输入 身份码");
                return;
            }
        },
    };

    println!("a{},b{}", a, b);
    let json = json!({"albumName":"","interval":"","name":"稻香","playLater":false,"singer":""});
    let data = serde_json::to_string(&json).unwrap();
    let msg = urlencoding::encode(&data);
    // let msg = urlencoding::encode(&data);
    println!("{}------{}", json, msg);

    let re = regex::Regex::new(r"点歌\s+(\S+)(?:\s+(\S+))?").unwrap();
    let a = re.captures("点歌 夜曲周杰伦").unwrap();
    println!("{:?}", a.len());
    let musicname = a.get(1).unwrap().as_str().trim().to_string();
    // let singer = a.get(2).or_else("f".to_string());
    let singer = a.get(2).map_or("", |m| m.as_str());

    println!("{:?}===={:?}", musicname, singer);
}
