extern crate cbindgen;

use cbindgen::{Config, EnumConfig, ItemType, Language, Style};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = Config {
        language: Language::C,
        header: Some(
            "/*\nCopyright (C) 2024 Signal Messenger, LLC.\nSPDX-License-Identifier: AGPL-3.0-only\n*/"
                .into(),
        ),
        style: Style::Type,
        cpp_compat: true,
        enumeration: EnumConfig {
            prefix_with_name: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ext_config = config.clone();
    ext_config.include_guard = Some("SIGNAL_SQLCIPHER_EXTENSION_H".into());
    ext_config.export.exclude = vec![
        "sqlcipher_malloc".into(),
        "sqlcipher_register_provider".into(),
        "SqlCipherProvider".into(),
    ];
    ext_config.export.item_types = vec![ItemType::Functions];

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_config(ext_config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/signal-sqlcipher-ext.h");

    let mut fts5_config = config.clone();
    fts5_config.include_guard = Some("SIGNAL_FTS5_TOKENIZER_H_".into());
    fts5_config.export.item_types = vec![
        ItemType::Functions,
        ItemType::OpaqueItems,
        ItemType::Structs,
        ItemType::Typedefs,
    ];

    fts5_config
        .export
        .rename
        .insert("Sqlite3".into(), "sqlite3".into());
    fts5_config
        .export
        .rename
        .insert("SqliteAPIRoutines3".into(), "sqlite3_api_routines".into());
    fts5_config
        .export
        .rename
        .insert("TokenFunction".into(), "sqlite3__fts5_token_fn".into());
    fts5_config.defines.insert(
        "feature = extension".into(),
        "SIGNAL_FTS5_TOKENIZER_EXTENSION_H_".into(),
    );

    cbindgen::Builder::new()
        .with_crate_and_name(crate_dir, "signal-tokenizer")
        .with_config(fts5_config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/signal-tokenizer.h");
}
