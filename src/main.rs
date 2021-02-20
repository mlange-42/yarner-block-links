use std::error::Error;
use yarner_lib::document::Node::Text;
use yarner_lib::document::{Node, Source, TextBlock};

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let (config, mut documents) = yarner_lib::parse_input(std::io::stdin())?;

    for (_path, doc) in documents.iter_mut() {
        let mut idx = 0;

        while idx < doc.nodes.len() {
            if let Node::Code(block) = &doc.nodes[idx] {
                let links: Vec<_> = block
                    .source
                    .iter()
                    .filter_map(|line| {
                        if let Source::Macro(name) = &line.source {
                            Some(name.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect();

                if let Some(name) = &block.name {
                    let insert_string = format!(
                        "<a id=\"block-{}\"></a>",
                        name.to_lowercase().replace(" ", "-")
                    );
                    doc.nodes.insert(
                        idx,
                        Text(TextBlock {
                            text: vec![insert_string],
                        }),
                    );
                    idx += 1;
                }

                if !links.is_empty() {
                    let insert_string = format!(
                        "> Refs: {}",
                        links
                            .iter()
                            .map(|l| format!(
                                "[`{}`](#block-{})",
                                l,
                                l.to_lowercase().replace(" ", "-")
                            ))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                    doc.nodes.insert(
                        idx + 1,
                        Text(TextBlock {
                            text: vec!["".to_string(), insert_string],
                        }),
                    )
                }
            }
            idx += 1;
        }
    }

    let out_json = yarner_lib::to_json(&config, &documents)?;
    println!("{}", out_json);
    Ok(())
}
