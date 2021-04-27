use webbrowser;
pub fn open_forms() {
    let forms_url = "http://form.new";

    if webbrowser::open(forms_url).is_ok() {
        println!("Opening Forms...");
    }
    else {
        println!("Error");
    }
}