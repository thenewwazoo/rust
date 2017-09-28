
use io;
use sys::fd::FileDesc;

macro_rules! unimpl {
    () => (return Err(io::Error::new(io::ErrorKind::Other, "No pipes available CMSIS RTOS."));)
}

pub struct AnonPipe(FileDesc);

pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    unimpl!()
}

impl AnonPipe {
    pub fn from_fd(fd: FileDesc) -> io::Result<AnonPipe> {
        unimpl!()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimpl!()
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unimpl!()
    }

    pub fn fd(&self) -> &FileDesc {
        self.0
    }
    pub fn into_fd(self) -> FileDesc {
        self.0
    }
}

pub fn read2(p1: AnonPipe, v1: &mut Vec<u8>, p2: AnonPipe, v2: &mut Vec<u8>) -> io::Result<()> {
    unimpl!()
}
