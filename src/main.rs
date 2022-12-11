use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

fn main() {
    let response = reqwest::blocking::get(
        "https://docs.google.com/document/d/e/2PACX-1vSYY2NUDdjuH-Ry8Vcuvsp50IV_Vi2E_3vE4oqfaKYVRbqJdf1jO_dd06jtOZPFp2tHs_v2v5TBhEzd/pub",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = Document::from(response.as_str());
    let found_content = document
        .find(Class("doc-content"))
        .next()
        .expect("not found doc-content");

    let result = found_content
        .children()
        .into_iter()
        .map(handle_node)
        .filter(|s| s.is_some())
        .map(|s| match s {
            Some(s) => s,
            None => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn handle_node(node: Node) -> Option<String> {
    let Some(name) = node.name() else { return None;};
    match name {
        "h1" => Some(format!("{}\n---", node.text())),
        "h2" => Some(format!("{}\n===", node.text())),
        "p" => Some(
            node.children()
                .into_iter()
                .map(handle_span)
                .collect::<Vec<String>>()
                .join(""),
        ),
        "span" => Some(handle_span(node)),
        "ul" => Some(
            node.children()
                .map(|c| format!("- {}", c.text()))
                .collect::<Vec<String>>()
                .join("\n"),
        ),
        "table" => Some(handle_table(node)),
        _ => None,
    }
}

fn handle_span(node: Node) -> String {
    node.children()
        .into_iter()
        .map(|inner| {
            let Some(inner_name) = inner.name() else { return node.text(); };
            match inner_name {
                "a" => format!("[{}]({})", inner.text(), inner.attr("href").unwrap()),
                "img" => format!("![]({})", inner.attr("src").unwrap()),
                _ => inner.text(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn handle_table(table: Node) -> String {
    let mut table_md = String::new();
    table.find(Name("tr")).enumerate().for_each(|(i, row)| {
        table_md.push('|');
        let mut child_len = 0;
        row.children().enumerate().for_each(|(j, col)| {
            let text = col
                .first_child()
                .and_then(handle_node)
                .map_or("".to_string(), |s| s + "|");
            table_md.push_str(&text);
            child_len = j + 1;
        });
        table_md.push('\n');
        if i == 0 {
            let header_sep = format!("|{}\n", "-|".repeat(child_len));
            table_md.push_str(&header_sep);
        }
    });
    table_md
}

