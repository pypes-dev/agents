use directories::BaseDirs;
use pickledb::{error, PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{
    fs::create_dir_all,
    io::Error,
    path::{Path, PathBuf},
};

pub struct DbConfig {
    pub agents_db: PickleDb,
    pub config_db: PickleDb,
}

struct DbPaths {
    agents_db_path: PathBuf,
    config_db_path: PathBuf,
}

pub fn initialize_db() -> Result<DbConfig, error::Error> {
    let db_paths = setup_db_paths().unwrap();
    let agents_db = if Path::new(&db_paths.agents_db_path).exists() {
        PickleDb::load(
            db_paths.agents_db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?
    } else {
        let mut agents_db = PickleDb::new(
            db_paths.agents_db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        );
        agents_db.lcreate("agents").unwrap();
        agents_db
    };

    let config_db = if Path::new(&db_paths.config_db_path).exists() {
        PickleDb::load(
            db_paths.config_db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?
    } else {
        PickleDb::new(
            db_paths.config_db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    };
    return Ok(DbConfig {
        agents_db,
        config_db,
    });
}

pub fn remove_db(mut db: PickleDb) {
    let keys = db.get_all();
    for key in keys.iter() {
        db.rem(key).unwrap();
    }
}

fn setup_db_paths() -> Result<DbPaths, Error> {
    let base_dirs = BaseDirs::new().expect("Unable to find home directory");
    let home_dir = base_dirs.home_dir();

    let agents_dir = home_dir.join(".agents");
    let db_dir = agents_dir.join("db");

    if !agents_dir.exists() {
        create_dir_all(&agents_dir)?;
    }
    if !db_dir.exists() {
        create_dir_all(&db_dir)?;
    }

    let agents_db_path = db_dir.join("agents.db");
    let config_db_path = db_dir.join("config.db");
    let db_paths = DbPaths {
        agents_db_path,
        config_db_path,
    };
    Ok(db_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let mut db = initialize_db().unwrap();
        db.agents_db.set("key", &"value").unwrap();
        let retrieved_value: Option<String> = db.agents_db.get("key");
        assert_eq!(retrieved_value, Some("value".to_string()));
    }
}
