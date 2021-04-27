use webbrowser;

pub fn open_sheets() {
    let sheets_url = "http://sheet.new";

    if webbrowser::open(sheets_url).is_ok() {
        println!("Opening Sheets...");
    }
    else {
        println!("Error");
    }
}