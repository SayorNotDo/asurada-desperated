use core::fmt::{Display, Formatter};

#[must_use]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(pub usize);

impl Status {
    pub const SUCCESS: Status = Self(0);

    pub fn is_success(&self) -> bool {
        self == &Self::SUCCESS
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "success"),
            _ => write!(f, "{:#X}", self.0),
        }
    }
}

pub type Result<T> = core::result::Result<T, Status>;

impl From<Status> for Result<()> {
    fn from(status: Status) -> Self {
        match status {
            Status::SUCCESS => Ok(()),
            e => Err(e),
        }
    }
}

impl<T> From<Result<T>> for Status {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(_) => Self::SUCCESS,
            Err(e) => e,
        }
    }
}
