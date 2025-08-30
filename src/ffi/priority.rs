#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum Priority {
    #[value(name = "a", help = "High priority")]
    a = b'a',
    #[value(name = "b", help = "Medium priority")]
    b = b'b',
    #[value(name = "c", help = "Low priority")]
    c = b'c',
    #[value(name = "z", help = "Default priority")]
    z = b'z',

    #[value(name = "none", help = "Use default priority")]
    None = 0,
}
impl From<u8> for Priority {
    fn from(b: u8) -> Self {
        match b {
            b'a' => Priority::a,
            b'b' => Priority::b,
            b'c' => Priority::c,
            b'z' => Priority::z,
            _ => Priority::z, // default
        }
    }
}

pub trait PriorityVecExt {
    fn to_bytes(&self) -> Vec<u8>;
}

impl PriorityVecExt for Vec<Priority> {
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().map(|p| *p as u8).collect()
    }
}
