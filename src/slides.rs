use webbrowser;
pub fn open_slides() {
    let slides_url = "http://slide.new";

    if webbrowser::open(slides_url).is_ok() {
        println!("Opening Slides...");
    }
    else {
        println!("Error");
    }
}