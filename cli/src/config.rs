use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) db_file: PathBuf,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        let db_file = if let Some(mut home_dir) = std::env::home_dir() {
            home_dir.push(".mucuroso/mucuroso.db");
            home_dir
        } else {
            panic!("Eita")
        };
        Self { db_file }
    }
}

#[cfg(test)]
mod tests {
    use std::env::home_dir;

    use super::*;
    

    #[test]
    fn config_tester() {
        let config: Config = toml::from_str(
            r#"
                db_file = '/home/teste/teste.db'
            "#,
        )
        .unwrap();
        assert_eq!(config.db_file, PathBuf::from("/home/teste/teste.db"));

        let mut path = home_dir().unwrap();
        path.push(".mucuroso/mucuroso.db");

        let cfg: Config = confy::load("mucuroso", "config").unwrap();
        assert_eq!(cfg.db_file, path);
    }
}
