use daemonize::Daemonize;
use directories::BaseDirs;
use libc::{kill, SIGTERM};
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{Error, ErrorKind},
    path::PathBuf,
};

pub struct DaemonPaths {
    pub agents_dir: PathBuf,
    pub stdout_path: PathBuf,
    pub stderr_path: PathBuf,
    pub pid_file_path: PathBuf,
}

pub fn setup_daemon_dirs_and_files() -> Result<DaemonPaths, Error> {
    let base_dirs = BaseDirs::new().expect("Unable to find home directory");
    let home_dir = base_dirs.home_dir();

    let agents_dir = home_dir.join(".agents");
    let tmp_dir = agents_dir.join("tmp");

    if !agents_dir.exists() {
        create_dir_all(&agents_dir)?;
    }
    if !tmp_dir.exists() {
        create_dir_all(&tmp_dir)?;
    }

    let stdout_path = tmp_dir.join("daemon.out");
    let stderr_path = tmp_dir.join("daemon.err");
    let pid_file_path = tmp_dir.join("test.pid");

    Ok(DaemonPaths {
        agents_dir,
        stdout_path,
        stderr_path,
        pid_file_path,
    })
}
pub fn initialize_daemon() {
    let paths = setup_daemon_dirs_and_files().unwrap();
    let _stdout = File::create(&paths.stdout_path).unwrap();
    let stderr = File::create(&paths.stderr_path).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(&paths.pid_file_path)
        .working_directory(&paths.agents_dir)
        .stderr(stderr)
        .privileged_action(|| "Executed before drip privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => {
            println!("error starting daemon {}", e);
            //TODO: ignore for now since this occurs when is called and
            //server is already running; handled in bind err but should probably
            //make a better conditional and err msg so this is not expected
        }
    }
}

pub fn kill_daemon() {
    let paths = setup_daemon_dirs_and_files().unwrap();
    let pid_str = match read_to_string(&paths.pid_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading PID file: {}", e);
            return ();
        }
    };

    let pid = match pid_str.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            let error = Error::new(ErrorKind::InvalidInput, e);
            eprintln!("Error parsing PID: {}", error);
            0
        }
    };

    unsafe {
        match kill(pid, SIGTERM) != -1 {
            true => eprintln!("🦿 Agents stopped"),
            false => eprintln!("🦿 Agents wasn't running, did you `agents start`"),
        }
    }
}
