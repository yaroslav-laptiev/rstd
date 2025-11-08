use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Migrator;

impl Migrator {
    pub fn new() -> Self {
        Self
    }

    fn app_root(&self) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        exe_path
            .parent() // bin/
            .expect("No parent for exe")
            .to_path_buf()
    }

    pub fn get_migrations(&self) -> String {
        let migrations_dir = self.app_root().join("migrations");
        let mut result = String::new();

        if !migrations_dir.exists() {
            panic!(
                "Migrations directory not found: {}",
                migrations_dir.display()
            );
        }

        let mut entries: Vec<_> = fs::read_dir(&migrations_dir)
            .expect("Failed to read migrations dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        // опціонально — сортуємо файли по імені (наприклад, 001_init.sql, 002_users.sql)
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                let contents = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read {:?}", path));
                result.push_str(&contents);
                result.push('\n');
            }
        }

        result
    }
}
