use webbrowser;

mod others_consts {
    pub const KEEP_URL: &str = "http://keep.new";
    pub const MS_TODO: &str = "ms-to-do://open";
}

pub fn open_mstodo() {
    if webbrowser::open(others_consts::MS_TODO).is_ok() {
        println!("Opening Microsoft To-Do...")
    }
    else {
        println!("Error")
    }
}
pub fn open_keep() {
    if webbrowser::open(others_consts::KEEP_URL).is_ok() {
        println!("Opening Keep...")
    }
    else {
        println!("Error")
    }
}