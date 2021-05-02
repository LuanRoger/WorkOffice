use clap::{Arg, App};
mod gsuite;

fn main() {
    let matches = App::new("GsuiteCLI")
                          .version("0.2")
                          .author("LuanRoger <luan.roger.2003@gmail.com>")
                          .about("Shortcut to GSuite tools in terminal")
                          .arg(Arg::with_name("TOOL")
                               .help("Chose the tools than will be open")
                               .required(true))
                          .get_matches();

    match matches.value_of("TOOL").unwrap() {
        "docs" | "doc" => gsuite::open_doc(),
        "sheets" | "sheet" => gsuite::open_sheets(),
        "slides" | "slide" => gsuite::open_slides(),
        "forms" | "form" => gsuite::open_forms(),
        _ => panic!("This argument is not valid"),
    }
}
