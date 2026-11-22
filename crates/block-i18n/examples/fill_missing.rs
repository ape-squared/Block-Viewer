//! Copy English messages missing from each bundled locale file (community
//! catch-up). Skips `language-name`. Appends Fluent entries in English.

use fluent_syntax::{ast, parser, serializer};
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

fn read(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path).map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))
}

fn messages<'a>(source: &'a str, path: &Path) -> io::Result<BTreeMap<&'a str, ast::Message<&'a str>>> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let resource = parser::parse(source)
        .map_err(|(_, errors)| io::Error::new(io::ErrorKind::InvalidData, format!("{}: {errors:?}", path.display())))?;
    let mut messages = BTreeMap::new();
    for entry in resource.body {
        if let ast::Entry::Message(message) = entry {
            let key = message.id.name;
            if messages.insert(key, message).is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}: duplicate message {key}", path.display()),
                ));
            }
        }
    }
    Ok(messages)
}

fn main() -> io::Result<()> {
    let locales = Path::new(env!("CARGO_MANIFEST_DIR")).join("locales");
    let english_path = locales.join("en.ftl");
    let english_source = read(&english_path)?;
    let english = messages(&english_source, &english_path)?;

    for entry in std::fs::read_dir(&locales)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "ftl") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name == "en.ftl" {
            continue;
        }
        let mut target_source = read(&path)?;
        if !target_source.ends_with('\n') {
            target_source.push('\n');
        }
        let target = messages(&target_source, &path)?;
        let mut missing: Vec<_> = english
            .iter()
            .filter(|(key, _)| **key != "language-name" && !target.contains_key(*key))
            .map(|(_, message)| message.clone())
            .collect();
        let appended = missing.len();
        if appended > 0 {
            target_source.push_str("\n# English placeholders (synced from en.ftl)\n");
            for message in &missing {
                target_source.push_str(&serializer::serialize(&ast::Resource {
                    body: vec![ast::Entry::Message(message.clone())],
                }));
            }
        }
        if appended > 0 {
            std::fs::write(&path, target_source)?;
            eprintln!("{name}: +{appended} entries");
        }
    }
    Ok(())
}