#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Null,
    Night,
    Day,
    Vote,
    FinalObjection,
    YesnoVote,
    End
}

pub const TIME_LENGTH: [i32; 7] = [
    0,
    25,
    60 /* todo */,
    15,
    15,
    10,
    -1
];