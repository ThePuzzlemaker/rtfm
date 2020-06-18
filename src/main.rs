#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("rtfm")
        .about("Open the Arch Wiki search page for a string from the command line")
        .version(&crate_version!()[..])
        .arg(
            Arg::with_name("STRING")
                .help("The string to search for")
                .required(true),
        )
        .get_matches();
    
    let search_string = matches.value_of("STRING").unwrap();
    let string_encoded = urlencoding::encode(search_string);

    match opener::open(format!("https://wiki.archlinux.org/index.php?search={}", string_encoded)) {
        Err(e) => {
            eprintln!("error: failed to open link: {}", e);
            std::process::exit(-1);
        },
        _ => {}
    };
}
