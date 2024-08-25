use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IncorrectPeopleCount
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectPeopleCount => write!(f, "People count is too little or too much")
        }
    }
}

impl std::error::Error for Error {}