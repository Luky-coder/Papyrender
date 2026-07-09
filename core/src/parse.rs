use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub type Dom = RcDom;

pub fn parse_html(input: &str) -> Dom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .expect("HTML parsing is infallible for html5ever's tolerant parser")
}

/// Recursively finds the first `tag` element and returns its concatenated text content.
pub fn find_element_text(handle: &Handle, tag: &str) -> Option<String> {
    if let NodeData::Element { name, .. } = &handle.data {
        if &*name.local == tag {
            return Some(collect_text(handle));
        }
    }
    handle
        .children
        .borrow()
        .iter()
        .find_map(|child| find_element_text(child, tag))
}

fn collect_text(handle: &Handle) -> String {
    let mut text = String::new();
    for child in handle.children.borrow().iter() {
        match &child.data {
            NodeData::Text { contents } => text.push_str(&contents.borrow()),
            NodeData::Element { .. } => text.push_str(&collect_text(child)),
            _ => {}
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_h1() {
        let dom = parse_html("<h1>Hello</h1>");
        let text = find_element_text(&dom.document, "h1").expect("expected an <h1> element");
        assert_eq!(text, "Hello");
    }
}
