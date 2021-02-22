use handlebars::{no_escape, Handlebars, RenderError};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use yarner_lib::{CodeBlock, Context, Document, Node, Source, TextBlock};

pub static CLEAN_LINK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());

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
    let (context, mut documents) = yarner_lib::parse_input()?;

    check_version(&context);

    let join = context
        .config
        .get("join")
        .and_then(|s| s.as_str())
        .unwrap_or(" ");

    let label = context
        .config
        .get("label")
        .and_then(|s| s.as_str())
        .unwrap_or("`{{label}}`");

    let template = context
        .config
        .get("template")
        .and_then(|s| s.as_str())
        .unwrap_or(
            "{{#if usage}}> Usage: {{usage}}  \n{{/if}}{{#if macros}}> Macros: {{macros}}{{/if}}",
        );

    let mut hb = Handlebars::new();
    hb.register_escape_fn(no_escape);
    hb.register_template_string("join", join)?;
    hb.register_template_string("label", label)?;
    hb.register_template_string("template", template)?;

    for (_path, doc) in documents.iter_mut() {
        let separator = hb.render(
            "join",
            &Helpers {
                newline: &doc.newline,
            },
        )?;
        let usage = block_usage(doc);

        let mut idx = 0;

        while idx < doc.nodes.len() {
            if let Node::Code(block) = &doc.nodes[idx] {
                let name = block
                    .name
                    .clone()
                    .unwrap_or_else(|| "unnamed-block".to_string());

                let used_by = usage.get(&name);
                let macros = macros(block);

                doc.nodes.insert(idx, Node::Text(format_anchor(&name)));
                idx += 1;

                if let Some(links) = format_links(&hb, &macros, used_by, &doc.newline, &separator)?
                {
                    doc.nodes.insert(idx + 1, Node::Text(links))
                }
            }
            idx += 1;
        }
    }

    yarner_lib::write_output(&documents)?;
    Ok(())
}

pub fn check_version(context: &Context) {
    if context.yarner_version != yarner_lib::YARNER_VERSION {
        eprintln!(
            "  Warning: The {} plugin was built against version {} of Yarner, \
                    but we're being called from version {}",
            context.name,
            yarner_lib::YARNER_VERSION,
            context.yarner_version
        )
    }
}

#[derive(Serialize)]
struct Block<'a> {
    macros: Option<String>,
    usage: Option<String>,
    newline: &'a str,
}
#[derive(Serialize)]
struct Label<'a> {
    label: &'a str,
}
#[derive(Serialize)]
struct Helpers<'a> {
    newline: &'a str,
}

fn format_links(
    hb: &Handlebars,
    macros: &[String],
    usage: Option<&Vec<String>>,
    newline: &str,
    separator: &str,
) -> Result<Option<TextBlock>, RenderError> {
    let block = Block {
        macros: if macros.is_empty() {
            None
        } else {
            Some(
                macros
                    .iter()
                    .map(|l| {
                        hb.render("label", &Label { label: l })
                            .map(|label| format!("[{}](#{})", label, block_link(l)))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .join(separator),
            )
        },
        usage: if let Some(usage) = usage {
            Some(
                usage
                    .iter()
                    .map(|l| {
                        hb.render("label", &Label { label: l })
                            .map(|label| format!("[{}](#{})", label, block_link(l)))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .join(separator),
            )
        } else {
            None
        },
        newline,
    };

    if block.macros.is_some() || block.usage.is_some() {
        let rendered = hb.render("template", &block)?;
        let mut text: Vec<_> = rendered.lines().map(|l| l.to_owned()).collect();
        text.insert(0, "".to_string());
        Ok(Some(TextBlock { text }))
    } else {
        Ok(None)
    }
}

fn block_link(name: &str) -> String {
    format!(
        "yarner-block-{}",
        &CLEAN_LINK_REGEX.replace_all(&name.to_lowercase(), "-")
    )
}

fn format_anchor(name: &str) -> TextBlock {
    let block_link = block_link(name);
    let insert_string = format!("<a name=\"{}\" id=\"{}\"></a>", block_link, block_link);
    TextBlock {
        text: vec![insert_string],
    }
}

fn macros(block: &CodeBlock) -> Vec<String> {
    block
        .source
        .iter()
        .filter_map(|line| {
            if let Source::Macro(name) = &line.source {
                Some(name.to_owned())
            } else {
                None
            }
        })
        .collect()
}

fn block_usage(document: &Document) -> HashMap<String, Vec<String>> {
    let mut usage: HashMap<String, Vec<String>> = HashMap::new();

    for node in &document.nodes {
        if let Node::Code(block) = node {
            for line in &block.source {
                if let Source::Macro(name) = &line.source {
                    let block_name = block
                        .name
                        .clone()
                        .unwrap_or_else(|| "unnamed-block".to_string());
                    match usage.entry(name.to_owned()) {
                        Entry::Occupied(mut entry) => entry.get_mut().push(block_name),
                        Entry::Vacant(entry) => {
                            entry.insert(vec![block_name]);
                        }
                    }
                }
            }
        }
    }

    usage
}
