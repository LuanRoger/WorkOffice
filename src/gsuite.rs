use webbrowser;

mod gsuite_consts {
    pub const DOC_URL: &str = "http://doc.new";
    pub const SHEETS_URL: &str = "http://sheet.new";
    pub const SLIDES_URL: &str = "http://slide.new";
    pub const FORMS_URL: &str = "http://form.new";
}
pub fn open_doc() {
    if webbrowser::open(gsuite_consts::DOC_URL).is_ok() {
        println!("Opening Docs...");
    }
    else { panic!("Could not start Doc"); }
}
pub fn open_sheets() {
    if webbrowser::open(gsuite_consts::SHEETS_URL).is_ok() {
        println!("Opening Sheets...");
    }
    else {
        println!("Error");
    }
}
pub fn open_slides() {
    if webbrowser::open(gsuite_consts::SLIDES_URL).is_ok() {
        println!("Opening Slides...");
    }
    else {
        println!("Error");
    }
}
pub fn open_forms() {
    if webbrowser::open(gsuite_consts::FORMS_URL).is_ok() {
        println!("Opening Forms...");
    }
    else {
        println!("Error");
    }
}