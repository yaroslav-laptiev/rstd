use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let migrations_dir = manifest_dir.join("migrations");

    let mut migrations = fs::read_dir(&migrations_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Failed to read migrations directory {}: {err}",
                migrations_dir.display()
            )
        })
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("sql")
        })
        .collect::<Vec<_>>();

    migrations.sort();

    let mut generated = String::from("const MIGRATIONS: &[(&str, &str)] = &[\n");

    for path in migrations {
        println!("cargo:rerun-if-changed={}", path.display());

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("Migration file name must be valid UTF-8");
        let path = path
            .to_str()
            .expect("Migration file path must be valid UTF-8");

        generated.push_str(&format!("    ({name:?}, include_str!({path:?})),\n"));
    }

    generated.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("migrations.rs"), generated).expect("Failed to write migrations.rs");
}
