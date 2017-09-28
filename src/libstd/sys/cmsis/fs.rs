
use ffi::OsString;
use io;
use path::{Path, PathBuf};
use sys::time::SystemTime;

// CMSIS RTOS has no concept of a filesystem
macro_rules! unimpl {
    () => (return Err(io::Error::new(io::ErrorKind::Other, "CMSIS RTOS has no concept of a file"));)
}

pub struct File;

pub struct FileAttr;

impl FileAttr {
    pub fn size(&self) -> u64 {
        0
    }
    pub fn perm(&self) -> FilePermissions {
        FilePermissions
    }

    pub fn file_type(&self) -> FileType {
        FileType
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unimpl!()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unimpl!()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unimpl!()
    }
}

pub struct FilePermissions;

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        true
    }
    pub fn set_readonly(&mut self, readonly: bool) {
        unimplemented!();
    }
}

pub struct FileType;

impl FileType {
    pub fn is_dir(&self) -> bool {
        false
    }
    pub fn is_file(&self) -> bool {
        false
    }
    pub fn is_symlink(&self) -> bool {
        false
    }
}

pub struct ReadDir;

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        None
    }
}

pub struct DirEntry;

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!();
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!();
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unimpl!()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unimpl!()
    }
}

pub struct OpenOptions;

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions
    }
    pub fn read(&mut self, read: bool) {
        unimplemented!();
    }
    pub fn write(&mut self, write: bool) {
        unimplemented!();
    }
    pub fn append(&mut self, append: bool) {
        unimplemented!();
    }
    pub fn truncate(&mut self, truncate: bool) {
        unimplemented!();
    }
    pub fn create(&mut self, create: bool) {
        unimplemented!();
    }
    pub fn create_new(&mut self, create_new: bool) {
        unimplemented!();
    }
}

pub struct DirBuilder;

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "CMSIS RTOS has no concept of a file",
        ))
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    unimpl!()
}

pub fn unlink(p: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    unimpl!()
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    unimpl!()
}

pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unimpl!()
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    unimpl!()
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    unimpl!()
}

pub fn lstat(path: &Path) -> io::Result<FileAttr> {
    unimpl!()
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    unimpl!()
}
