use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_sdk::{
    info,
    program_error::{PrintProgramError, ProgramError},
    program_utils::DecodeError,
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SolanaPokerError {
    #[error("deserialization failed")]
    DeserializationFailed,
    #[error("room is full")]
    RoomFull,
    #[error("invaid timestamp")]
    InvalidTimestamp,
    #[error("player not found")]
    PlayerNotFound,
    #[error("Token transfer failed")]
    TokenTransferFailed
}

impl From<SolanaPokerError> for ProgramError {
    fn from(e: SolanaPokerError) -> Self {
        ProgramError::CustomError(e as u32)
    }
}

impl<T> DecodeError<T> for SolanaPokerError {
    fn type_of() -> &'static str {
        "SolanaPokerError"
    }
}

impl PrintProgramError for SolanaPokerError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            SolanaPokerError::DeserializationFailed => info!("Error: deserialization failed"),
            SolanaPokerError::RoomFull => info!("Error: room is full"),
            SolanaPokerError::InvalidTimestamp => info!("Error: invalid timestamp"),
            SolanaPokerError::PlayerNotFound => info!("Error: player not found"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn return_tittactoe_error_as_program_error() -> ProgramError {
        SolanaPokerError::PlayerNotFound.into()
    }

    #[test]
    fn test_print_error() {
        let error = return_solanapoker_error_as_program_error();
        error.print::<SolanaPokerError>();
    }

    #[test]
    #[should_panic(expected = "CustomError(5)")]
    fn test_error_unwrap() {
        Err::<(), ProgramError>(return_tittactoe_error_as_program_error()).unwrap();
    }
}
