use crate::RdmaError;
use wiggle::GuestError;

impl From<GuestError> for RdmaError {
    fn from(value: GuestError) -> Self {
        // match value {
        //     GuestError::InvalidFlagValue(_) => {}
        //     GuestError::InvalidEnumValue(_) => {}
        //     GuestError::PtrOverflow => {}
        //     GuestError::PtrOutOfBounds(_) => {}
        //     GuestError::PtrNotAligned(_, _) => {}
        //     GuestError::PtrBorrowed(_) => {}
        //     GuestError::BorrowCheckerOutOfHandles => {}
        //     GuestError::SliceLengthsDiffer => {}
        //     GuestError::InFunc { .. } => {}
        //     GuestError::InvalidUtf8(_) => {}
        //     GuestError::TryFromIntError(_) => {}
        // }
        return Self::RuntimeError;
    }
}
