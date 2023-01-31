use html_parser::{Dom, Element, Node};

pub fn parse_html(html: &str) -> Vec<String> {
    let dom = Dom::parse(html);

    match dom {
        Ok(dom) => parse_dom(dom),
        Err(_) => Vec::new(),
    }
}

fn parse_dom(dom: Dom) -> Vec<String> {
    let mut texts = Vec::new();

    for node in &dom.children {
        parse_node(node, &mut texts);
    }

    texts
}

fn parse_element(element: &Element, texts: &mut Vec<String>) {
    for node in &element.children {
        parse_node(node, texts);
    }
}

fn parse_node(node: &Node, texts: &mut Vec<String>) {
    match &node {
        Node::Element(element) => parse_element(element, texts),
        Node::Text(text) => texts.push(text.clone()),
        _ => {}
    }
}
