use std::path::PathBuf;
use tokio::*;

fn check_dir() -> io::Result<PathBuf> {
    let base = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "could not resolve data dir"))?;
    let root = base.join(".ukmcl");
    let instances = root.join("instances");

    if !root.exists() {
        std::fs::create_dir_all(&instances)?;
    } else if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
                format!("expected directory not found: {}", root.display()),
        ))
    }

    Ok(instances)
}

fn get_inst_dir(instance_name: &str) -> io::Result<PathBuf> {
    let instances_dir = check_dir()?;
    let instance_dir = instances_dir.join(instance_name);
    std::fs::create_dir_all(&instance_dir)?;

    Ok(instance_dir)
}

pub fn setup_instance(instance_name: &str) -> io::Result<PathBuf> {
    let instance_dir = get_inst_dir(instance_name)?;

    for sub in ["minecraft"] {
        std::fs::create_dir_all(&instance_dir.join(sub))?;
    }

    Ok(instance_dir)
}