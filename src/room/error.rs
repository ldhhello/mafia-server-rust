use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    RoomIsFull,
    AlreadyLeft,
    PlayerNotEnough,
    AlreadyStarted,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoomIsFull => write!(f, "Room is full"),
            Self::AlreadyLeft => write!(f, "Already left session"),
            Self::PlayerNotEnough => write!(f, "Player not enough"),
            Self::AlreadyStarted => write!(f, "Game have already been started")
        }
    }
}

impl std::error::Error for Error {}