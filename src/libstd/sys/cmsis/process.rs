use sys::pipe::{self, AnonPipe};

pub struct Command;

pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

pub enum Stdio {
    Unimplemented,
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        unimplemented!()
    }

    pub fn arg(&mut self, arg: &OsStr) {
        unimplemented!()
    }
    pub fn env(&mut self, key: &OsStr, val: &OsStr) {
        unimplemented!()
    }
    pub fn env_remove(&mut self, key: &OsStr) {
        unimplemented!()
    }
    pub fn env_clear(&mut self) {
        unimplemented!()
    }
    pub fn cwd(&mut self, dir: &OsStr) {
        unimplemented!()
    }
    pub fn stdin(&mut self, stdin: Stdio) {
        unimplemented!()
    }
    pub fn stdout(&mut self, stdout: Stdio) {
        unimplemented!()
    }
    pub fn stderr(&mut self, stderr: Stdio) {
        unimplemented!()
    }
    pub fn spawn(
        &mut self,
        default: Stdio,
        needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No processes available CMSIS RTOS.",
        ))
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No processes available CMSIS RTOS.",
        ))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatus;

impl ExitStatus {
    pub fn success(&self) -> bool {
        false
    }

    pub fn code(&self) -> Option<i32> {
        None
    }

    pub fn signal(&self) -> Option<i32> {
        None
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "none")
    }
}

pub struct Process;

impl Process {
    pub fn id(&self) -> u32 {
        0xFFFFFFFF
    }

    pub fn kill(&mut self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No processes available CMSIS RTOS.",
        ))
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No processes available CMSIS RTOS.",
        ))
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "No processes available CMSIS RTOS.",
        ))
    }
}
