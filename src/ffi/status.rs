#[repr(u8)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Status {
    #[value(name = "backlog")]
    backlog = b'b',
    #[value(name = "todo")]
    todo = b't',
    #[value(name = "doing")]
    doing = b'w',
    #[value(name = "done")]
    done = b'd',
}

impl From<u8> for Status {
    fn from(b: u8) -> Self {
        match b {
            b'b' => Status::backlog,
            b't' => Status::todo,
            b'w' => Status::doing,
            b'd' => Status::done,
            _ => Status::todo, // default
        }
    }
}

pub trait StatusVecExt {
    fn to_bytes(&self) -> Vec<u8>;
}

impl StatusVecExt for Vec<Status> {
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().map(|s| *s as u8).collect()
    }
}
