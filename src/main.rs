use clap::{arg, App};
use colored::*;
use nixinfo::{distro, environment, hostname, kernel, memory, terminal, uptime};

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
    let terminal: String = terminal().unwrap();

    let args = args().get_matches();
    let decor: &str = args.value_of("decor").unwrap();
    let emoticon: &str = args.value_of("emoticon").unwrap();

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
    print_line!(cyan, "tm", terminal);
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

pub fn args() -> App<'static> {
    App::new("edenfetch")
        .version(env!("CARGO_PKG_VERSION"))
        .author("EdenQwQ <lsahlm1eden@gmail.com>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            arg!(-d --decor <COLOR_DECORATION>)
                .required(false)
                .takes_value(true)
                .help("Set a string to print as color decoration")
                .default_value(""),
        )
        .arg(
            arg!(-e --emoticon <EMOTICON>)
                .required(false)
                .takes_value(true)
                .help("Set a string to print as emoticon")
                .default_value("ʕ•́ᴥ•̀ʔっ♡"),
        )
}
