use directories::BaseDirs;
use pickledb::{error, PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{
    fs::create_dir_all,
    io::Error,
    path::{Path, PathBuf},
};

pub fn initialize_db() -> Result<PickleDb, error::Error> {
    let db_path = setup_db_paths().unwrap();

    if Path::new(&db_path).exists() {
        let db = PickleDb::load(
            db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;
        Ok(db)
    } else {
        let mut db = PickleDb::new(
            db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        );
        db.lcreate("agents").unwrap();
        Ok(db)
    }
}

pub fn remove_db(mut db: PickleDb) {
    db.lrem_list("agents").unwrap();
    db.lcreate("agents").unwrap();
}

fn setup_db_paths() -> Result<PathBuf, Error> {
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

    let db_path = db_dir.join("agents.db");
    Ok(db_path)
}
