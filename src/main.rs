use clap::{Arg, App};
mod doc;
mod sheets;
mod slides;
mod forms;

fn main() {
    let matches = App::new("GsuiteCLI")
                          .version("0.1")
                          .author("LuanRoger <luan.roger.2003@gmail.com>")
                          .about("Shortcut to GSuite tools in terminal")
                          .arg(Arg::with_name("TOOL")
                               .help("Chose the tools than will be open")
                               .required(true))
                          .get_matches();

    match matches.value_of("TOOL").unwrap() {
        "docs" | "doc" => doc::open_doc(),
        "sheets" | "sheet" => sheets::open_sheets(),
        "slides" | "slide" => slides::open_slides(),
        "forms" | "form" => forms::open_forms(),
        _ => panic!("This argument is not valid"),
    }
}
