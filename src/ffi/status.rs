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
