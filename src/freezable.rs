use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;

pub trait Freezable: Serialize + for<'a> Deserialize<'a> {
    fn write_to_file(&self, path: PathBuf);
    fn write_to_file_str(&self, path: &str);
    fn from_file(path: PathBuf) -> Self;
    fn from_file_str(path: &str) -> Self;
    fn freeze_to_string(&self) -> String;
    fn unfreeze_from_string(from: String) -> Self;
}

impl<T: Serialize + for<'a> Deserialize<'a>> Freezable for T {
    fn write_to_file(&self, path: PathBuf) {
        let file_exists = fs::metadata(&path).is_ok();
        if !file_exists {
            touch(&path).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
        let write_str = serde_json::to_string(&self).expect("Could not serialize object");
        fs::write(path, write_str).expect("Unable to write config file.")
    }

    fn write_to_file_str(&self, path_str: &str) {
        let path_expanded = shellexpand::tilde(path_str).into_owned().to_string();
        let mut path = PathBuf::new();
        path.push(path_expanded);
        self.write_to_file(path);
    }

    fn from_file(path: PathBuf) -> Self {
        let file_exists = fs::metadata(&path).is_ok();
        if !file_exists {
            return serde_json::from_str("{}").expect("Could not initialize default value for self");
        }
        let contents = fs::read_to_string(path).expect("Could not read provided file");
        serde_json::from_str(&contents).expect("Could not deserialize object")
    }

    fn from_file_str(path: &str) -> Self {
        let path_expanded = shellexpand::tilde(path).into_owned().to_string();
        let mut path = PathBuf::new();
        path.push(path_expanded);
        Self::from_file(path)
    }

    fn freeze_to_string(&self) -> String {
        serde_json::to_string(&self).expect("Could not serialize object")
    }

    fn unfreeze_from_string(from: String) -> Self {
        serde_json::from_str(&from).expect("Could not deserialize object")
    }
}

fn touch(path: &PathBuf) -> io::Result<()> {
    match OpenOptions::new().create(true).truncate(false).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
