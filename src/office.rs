use webbrowser;

mod office_consts {
    pub const WORD_URL: &str = "https://office.live.com/start/Word.aspx";
    pub const POWERPOINT_URL: &str = "https://office.live.com/start/PowerPoint.aspx";
    pub const EXCEL_URL: &str = "https://office.live.com/start/Excel.aspx";
}

pub fn open_word() {
    if webbrowser::open(office_consts::WORD_URL).is_ok() {
        println!("Opening Word...");
    }
    else { panic!("Could not start Word"); }
}
pub fn open_powerpoint() {
    if webbrowser::open(office_consts::POWERPOINT_URL).is_ok() {
        println!("Opening PowerPoint...");
    }
    else { panic!("Could not start PowerPoint"); }
}
pub fn open_excel() {
    if webbrowser::open(office_consts::EXCEL_URL).is_ok() {
        println!("Opening Excel...");
    }
    else { panic!("Could not start Excel"); }
}