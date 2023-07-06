use std::{ffi::OsStr, path::Path};

use notify::{Config, RecursiveMode, Watcher};

use super::{FileWatchEventArgs, ISettingFileWatcher};

pub struct FileSystemSettingFileWatcher {
    is_close_requested: bool,
}

impl ISettingFileWatcher for FileSystemSettingFileWatcher {
    fn watch<T: AsRef<OsStr>, F: Fn(FileWatchEventArgs)>(&mut self, path: &T, func: F) {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::RecommendedWatcher::new(tx, Config::default()).unwrap();
        watcher
            .watch(Path::new(path), RecursiveMode::Recursive)
            .unwrap();

        for res in rx {
            match res {
                Ok(event) => {
                    let args = FileWatchEventArgs { paths: event.paths };
                    func(args);
                }
                Err(e) => println!("watch error: {:?}", e),
            }

            if self.is_close_requested {
                watcher.unwatch(Path::new(path)).unwrap();
                break;
            }
        }
    }

    fn unwatch(&mut self) {
        self.is_close_requested = true;
    }
}
