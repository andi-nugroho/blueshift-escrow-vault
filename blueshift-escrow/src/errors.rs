use {num_derive::FromPrimitive, pinocchio::program_error::ProgramError, thiserror::Error};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    #[error("Account is not a signer")]
    NotSigner,

    #[error("Invalid instruction data")]
    InvalidInstructionData,

    #[error("Account data is invalid")]
    InvalidAccountData,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Insufficient Address")]
    InsufficientAddress,

    #[error("Invalid Owner")]
    InvalidOwner,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// impl TryFrom<u32> for PinocchioError {
//     type Error = ProgramError;
//     fn try_from(error: u32) -> Result<Self, Self::Error> {
//         match error {
//             0 => Ok(PinocchioError::NotRentExempt),
//             _ => Err(ProgramError::InvalidArgument),
//         }
//     }
// }

// impl ToStr for PinocchioError {
//     fn to_str<E>(&self) -> &'static str {
//         match self {
//             PinocchioError::NotRentExempt => "Error: Lamport balance below rent-exempt threshold",
//         }
//     }
// }
