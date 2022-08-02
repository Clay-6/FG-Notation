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
