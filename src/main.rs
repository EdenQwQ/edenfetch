use colored::*;
use nixinfo::{distro, environment, hostname, kernel, memory, uptime};

fn main() {
    let username: String = std::env::var("USER").unwrap();
    let hostname: String = hostname().unwrap();
    let uptime: String = uptime().unwrap();
    let distro: String = distro().unwrap().replace("\"", "");
    let kernel: String = kernel().unwrap();
    let memory: String = memory().unwrap();
    let environment: String = environment()
        .unwrap()
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let decor: &str = "";
    let emoticon: &str = "ʕ•́ᴥ•̀ʔっ♡";
    // let emoticon: &str = "ʕ•ᴥ•ʔ";

    macro_rules! print_line {
        ($color:ident, $info:expr, $value:ident) => {
            println!("  {} ~ {}", $info.$color(), $value.white());
        };
    }

    println!();
    print_line!(cyan, "tm", uptime);
    print_line!(cyan, "os", distro);
    print_line!(cyan, "kr", kernel);
    print_line!(cyan, "mm", memory);
    print_line!(cyan, "wm", environment);
    println!();
    println!(
        "  {} {} {} {} {} {} {}",
        decor.red(),
        decor.yellow(),
        decor.green(),
        decor.blue(),
        decor.magenta(),
        decor.cyan(),
        decor.white()
    );
    println!();
    println!(
        "  {}{}{}{} {}",
        emoticon.white(),
        " welcome to ".white(),
        hostname.magenta(),
        ",".white(),
        username.magenta()
    );
}
