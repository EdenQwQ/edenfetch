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
    let color: &str = args.value_of("color").unwrap();

    macro_rules! print_line {
        ($color:expr, $info:expr, $value:ident) => {
            println!("  {} ~ {}", $info.color($color), $value.white());
        };
    }

    println!();
    print_line!(color, "tm", uptime);
    print_line!(color, "os", distro);
    print_line!(color, "kr", kernel);
    print_line!(color, "mm", memory);
    print_line!(color, "wm", environment);
    print_line!(color, "tm", terminal);
    println!();
    println!(
        "  {} {} {} {} {} {} {}",
        decor.red(),
        decor.yellow(),
        decor.green(),
        decor.cyan(),
        decor.blue(),
        decor.magenta(),
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
        .arg(
            arg!(-c --color <COLOR>)
                .required(false)
                .takes_value(true)
                .help("Set a color to print the info names")
                .default_value("cyan"),
        )
}
