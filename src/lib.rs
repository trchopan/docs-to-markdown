use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use url::{form_urlencoded, Url};

pub fn parse(content: &str) -> Result<String, String> {
    let document = Document::from(content);
    let found_content = document
        .find(Class("doc-content"))
        .next()
        .ok_or("Not found `doc-content`")?;

    let result: Vec<String> = found_content
        .children()
        .into_iter()
        .map(handle_node)
        .map(|s| match s {
            Some(s) => s,
            None => "".to_string(),
        })
        .collect();

    let result = insert_newline(result);

    Ok(result)
}

fn handle_node(node: Node) -> Option<String> {
    let Some(name) = node.name() else { return None; };
    let header_format = |n: usize| Some(format!("{} {}", "#".repeat(n), node.text()));
    match name {
        "h1" => header_format(1),
        "h2" => header_format(2),
        "h3" => header_format(3),
        "h4" => header_format(4),
        "h5" => header_format(5),
        "h6" => header_format(6),
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
                    let href: Option<String> =
                        inner.attr("href").and_then(|f| match Url::parse(f) {
                            Ok(u) => {
                                form_urlencoded::parse(u.query().unwrap_or_default().as_bytes())
                                    .find_map(|(key, val)| {
                                        if key == "q" {
                                            Some(val.to_string())
                                        } else {
                                            None
                                        }
                                    })
                            }
                            Err(_) => None,
                        });
                    format!(
                        "[{}]({})",
                        inner.text(),
                        href.unwrap_or_else(|| "cannot parse google link".to_string())
                    )
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

fn insert_newline(s: Vec<String>) -> String {
    let mut in_code = false;
    let result: Vec<String> = s
        .iter()
        .map(|ss| -> String {
            if ss.contains("```") || ss.contains("+++") {
                in_code = !in_code;
            }
            if in_code {
                ss.to_owned()
            } else {
                ss.to_owned() + "\n"
            }
        })
        .collect();

    result.join("\n")
}
