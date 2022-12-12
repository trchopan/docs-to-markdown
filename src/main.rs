use clap::Parser;
use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Google Docs published page link. Example: https://docs.google.com/document/d/e/<DOC_ID>/pub
    doc_publish_link: String,
}

fn main() {
    let args = Args::parse();

    let response = reqwest::blocking::get(args.doc_publish_link)
        .expect("Cannot get the docs. Please check docs link.")
        .text()
        .expect("Cannot extract the docs body text.");

    let document = Document::from(response.as_str());
    let found_content = document
        .find(Class("doc-content"))
        .next()
        .expect("Not found doc-content. Please check that the link is for a Google Docs published to the Web page.");

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
        "h3" => Some(format!("{} {}\n", "#".repeat(3), node.text())),
        "h4" => Some(format!("{} {}\n", "#".repeat(4), node.text())),
        "h5" => Some(format!("{} {}\n", "#".repeat(5), node.text())),
        "h6" => Some(format!("{} {}\n", "#".repeat(6), node.text())),
        "p" => Some(
            node.children()
                .into_iter()
                .map(handle_span)
                .collect::<Vec<String>>()
                .join(""),
        ),
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
                "a" => {
                    let Some(href) = inner.attr("href") else { return "".to_string();};
                    // Remove Google redirect link
                    let extract_link_regex =
                        Regex::new(r"(?m)https://www\.google\.com/url\?q=(.*?)&sa.*$").unwrap();
                    let href = extract_link_regex.replace_all(href, "$1");
                    return format!("[{}]({})", inner.text(), href);
                }
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
