use std::{env, path::Path, time::SystemTime};
use filetime::{FileTime, set_file_mtime};

fn main() -> anyhow::Result<()> {
    let filenames: Vec<String> = env::args().skip(1).collect();
    if filenames.len() == 0 {
        println!("we need 1 arguments of filename");
        return Ok(());
    }

    if let Ok(_) = std::fs::File::open(&filenames[0]) {
        let path = Path::new(&filenames[0]);
        let mtime = FileTime::from_system_time(SystemTime::now());
        set_file_mtime(path, mtime)?;
    }
    else {
        std::fs::write(&filenames[0], b"")?;
    }

    Ok(())
}
