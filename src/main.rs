use clap::Parser;
use docs_to_markdown::parse;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Google Docs published page link. Example: https://docs.google.com/document/d/e/<DOC_ID>/pub
    doc_publish_link: String,
}

fn main() {
    let args = Args::parse();

    let content = reqwest::blocking::get(args.doc_publish_link)
        .expect("Cannot get the docs. Please check docs link.")
        .text()
        .expect("Cannot extract the docs body text.");

    let result = parse(&content).expect("Cannot parse content.");

    println!("{}", result);
}
