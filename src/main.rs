use std::process;
use clap::Parser;

mod writer;
use writer::Writer;

mod parser;
use parser::ApiParser;

#[derive(Parser, Debug)]
#[clap(name = "java_api", version = "0.1.0", author = "Marco Balossini")]
#[clap(about = "Parse API endpoints from java code.", long_about = None)]
struct CliArgs {
    /// Specify the target file
    #[clap(short('f'), long("file"), value_name = "file")]
    file: Option<String>,
    /// Expect a text from stdin
    #[clap(short('t'), long("text"))]
    text: bool,
    /// Specify the target directory
    #[clap(short('d'), long("dir"), value_name = "dir")]
    dir: Option<String>,
    /// Specify the output file
    #[clap(short('o'), long("output"), value_name = "output")]
    output: Option<String>,
}

fn main() {
    // Parse command line arguments
    let args = CliArgs::parse();

    let writer: Writer = match args.output {
        Some(name) => Writer::new(Some(name)),
        None => Writer::new(None),
    };
    let mut parser: ApiParser = ApiParser::new(writer);

    let mut apis;
    if args.text {
        let mut text = String::new();
        let lines = std::io::stdin().lines();
        // Lines to text
        lines.for_each(|l| {
            text.push_str(l.unwrap().as_str());
            text.push('\n');
        });
        apis = parser.parse_text(text, None);
    } else if let Some(dir) = args.dir {
        apis = parser.parse_dir(dir);
    } else if let Some(file) = args.file {
        apis = parser.parse_file(file);
    } else {
        eprintln!("No input specified");
        process::exit(1);
    }

    apis.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));
    apis.into_iter().for_each(|a| println!("{}", a.to_string()));

}
