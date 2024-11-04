use onenote_parser::Parser;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "logging")]
    env_logger::builder()
        .format_timestamp(None)
        .init();

    let path = env::args().nth(1).expect("usage: parse <file> [--silent]");
    let opt_silent = Some(String::from("--silent")) == env::args().nth(2);

    let path = PathBuf::from(path);

    let mut parser = Parser::new();
    if path.extension() == Some(&OsString::from("onetoc2".to_string())) {
        let notebook = parser.parse_notebook(&path).unwrap();
        if !opt_silent {
            println!("{:#?}", notebook);
        }
    } else {
        let section = parser.parse_section(&path).unwrap();
        if !opt_silent {
            println!("{:#?}", section);
        }
    }
}
