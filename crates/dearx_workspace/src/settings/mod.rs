mod file_system_setting_files_watcher;
pub use file_system_setting_files_watcher::FileSystemSettingFileWatcher;

use std::{ffi::OsStr, path::PathBuf};

pub struct FileWatchEventArgs {
    paths: Vec<PathBuf>,
}

impl FileWatchEventArgs {
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
}

pub trait ISettingFileWatcher {
    fn watch<T: AsRef<OsStr>, F: Fn(FileWatchEventArgs)>(&mut self, path: &T, func: F);

    fn unwatch(&mut self);
}
