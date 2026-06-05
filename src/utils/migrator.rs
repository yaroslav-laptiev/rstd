include!(concat!(env!("OUT_DIR"), "/migrations.rs"));

pub struct Migrator;

impl Migrator {
    pub fn new() -> Self {
        Self
    }

    pub fn get_migrations(&self) -> String {
        let mut result = String::new();

        if MIGRATIONS.is_empty() {
            panic!("No SQL migrations were embedded at build time");
        }

        for (_, contents) in MIGRATIONS {
            result.push_str(contents);
            result.push('\n');
        }

        result
    }
}
