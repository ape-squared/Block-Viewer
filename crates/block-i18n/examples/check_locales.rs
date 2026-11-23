//! Quick check: every bundled FTL parses and has unique keys in Fluent.

use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;

fn main() {
    for &(lang, source) in block_i18n::SOURCES {
        let source = source.strip_prefix('\u{feff}').unwrap_or(source);
        match FluentResource::try_new(source.to_owned()) {
            Ok(resource) => {
                let locale = lang.parse().expect("locale code");
                let mut bundle = FluentBundle::new_concurrent(vec![locale]);
                bundle.set_use_isolating(false);
                if let Err(errors) = bundle.add_resource(resource) {
                    eprintln!("{lang}: duplicate or invalid entries: {errors:?}");
                    std::process::exit(1);
                }
                println!("{lang}: ok");
            }
            Err((_partial, errors)) => {
                eprintln!("{lang}: parse {errors:?}");
                std::process::exit(1);
            }
        }
    }
}
