use webbrowser;

pub fn open_doc() {
    let doc_url = "http://doc.new";

    if webbrowser::open(doc_url).is_ok() {
        println!("Opening Docs...");
    }
    else {
        println!("Error");
    }
}