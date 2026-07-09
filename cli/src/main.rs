use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("usage: papyrender-cli <input.html>");
    let html = fs::read_to_string(&path).expect("failed to read input file");
    let pdf_bytes = papyrender_core::render_html_to_pdf(&html);
    println!("rendered {} bytes of PDF output", pdf_bytes.len());
}
