use astrolabiu_core::julian_day;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("astrolabiu {}", env!("CARGO_PKG_VERSION")),
        ["jd", y, m, d] => match (y.parse::<i32>(), m.parse::<u32>(), d.parse::<f64>()) {
            (Ok(y), Ok(m), Ok(d)) => println!("{:.5}", julian_day(y, m, d)),
            _ => usage(),
        },
        _ => usage(),
    }
}

fn usage() {
    eprintln!("usage: astrolabiu jd YEAR MONTH DAY | --version");
    std::process::exit(2);
}
