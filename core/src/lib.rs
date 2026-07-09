pub mod layout;
pub mod parse;
pub mod pdf;
pub mod style;

/// Runs the full pipeline: parse HTML, resolve styles, compute layout, render PDF bytes.
pub fn render_html_to_pdf(html: &str) -> Vec<u8> {
    let dom = parse::parse_html(html);
    let styled = style::apply_styles(&dom);
    let layout = layout::compute_layout(&styled);
    pdf::render_pdf(&layout)
}
