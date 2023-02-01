//! A crate/binary to convert between forms of fighting game notation,
//! currently [numpad](https://glossary.infil.net/?t=Numpad%20Notation)
//! & [abbreviated](https://glossary.infil.net/?t=Notation) notation,
//! using the corresponding modules.
//!
//! The modules provide types for full moves & their components
//! which have [`From`] impls for their counterparts in the other module.
//!
//! # Example
//!
//! ```
//! # use fg_notation::{abbreviated, numpad, CreationError};
//!
//! let numpad_move = numpad::Move::new("236H")?;
//! let abbreviated_move = abbreviated::Move::new("qcf H")?;
//!
//! assert_eq!(numpad::Move::from(abbreviated_move.clone()), numpad_move);
//! assert_eq!(abbreviated::Move::from(numpad_move), abbreviated_move);
//!
//! # Result::<(), CreationError>::Ok(())
//! ```

pub mod abbreviated;
pub mod numpad;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("Invalid motion input.")]
    InvalidMotion,
    #[error("Invalid button.")]
    InvalidButton,
    #[error("Invalid modifier.")]
    InvalidModifier,
}
