
use cmsis_os as c;

use std::ffi::CStr;

macro_rules! unimpl {
    () => (return Err(io::Error::new(io::ErrorKind::Other, "No such concept available in CMSIS RTOS."));)
}

/// CMSIS RTOS has no concept of errno
pub fn errno() -> i32 {
    0
}

pub fn error_string(_: i32) -> String {
    "CMSIS RTOS has no concept of errno".to_string()
}

pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

pub fn env() -> Env {
    Env
}

pub struct SplitPaths;

pub fn split_paths(unparsed: &OsStr) -> SplitPaths {
    SplitPaths
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "CMSIS RTOS has no concept of a path".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str {
        "CMSIS RTOS has no concept of a path"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    let tid = c::osThreadGetId();
    if tid.name.is_null() {
        Err(io::Error::new(io::ErrorKind::Other, "Thread has no name"))
    } else {
        Ok(PathBuf::from(
            CStr::from_ptr(c::osThreadGetId())
                .to_string_lossy()
                .into_owned(),
        ))
    }
}

pub fn getcwd() -> io::Result<PathBuf> {
    unimpl!()
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    unimpl!()
}

pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    unimpl!()
}

pub fn unsetenv(n: &OsStr) -> io::Result<()> {
    unimpl!()
}

pub fn temp_dir() -> PathBuf {
    unimplemented!()
}

pub fn home_dir() -> Option<PathBuf> {
    unimplemented!()
}

pub fn exit(code: i32) -> ! {
    let tid = c::osThreadGetId();
    let _ = c::osThreadTerminate(tid);
    unreachable!();
}
