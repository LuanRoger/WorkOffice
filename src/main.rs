use clap::{Arg, App};
mod gsuite;
mod office;
mod others;

fn main() {
    let matches = App::new("Work Office")
                          .version("0.3")
                          .author("LuanRoger <luan.roger.2003@gmail.com>")
                          .about("Shortcut to Office and Workspace tools in terminal")
                          .version_short("v")
                          .arg(Arg::with_name("TOOL")
                               .help("Choose the tool that will be open")
                               .long_help(
                                   "Choose the tool that will be open
                                   Use to open:
                                   [GSuite]
                                   docs | doc | document => Open Google Documents
                                   sheets | sheet | spreadsheet => Open Google Sheets
                                   slides | slide | presentation => Open Google Slides
                                   forms | form => Open Google Forms
                                   [Office]
                                   word => Open Office Word
                                   excel => Open Office Excel
                                   power | point | powerpoint | pp => Open Office PowerPoint
                                   [Others]
                                   keep | kp => Open Google Keep
                                   mstodo | todo => Open Microsoft To-Do"
                                )
                               .required(true))
                          .get_matches();

    match matches.value_of("TOOL").unwrap() {
        "docs" | "doc" | "document" => gsuite::open_doc(),
        "sheets" | "sheet" | "spreadsheet" => gsuite::open_sheets(),
        "slides" | "slide" | "presentation" => gsuite::open_slides(),
        "forms" | "form" => gsuite::open_forms(),
        "word" => office::open_word(),
        "excel" => office::open_excel(),
        "power" | "point" | "powerpoint" | "pp" => office::open_powerpoint(),
        "keep" | "kp" => others::open_keep(),
        "mstodo" | "todo" => others::open_mstodo(),
        _ => panic!("This argument is not valid"),
    }
}
