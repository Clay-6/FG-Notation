use core::fmt;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    jumping: bool,
    motion: Motion,
    button: Button,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Motion(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(String);

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("Invalid motion input. Motions can only consist of ASCII digits")]
    InvalidMotion,
    #[error("Invalid button. Buttons can only consist of ASCII alphabetic characters")]
    InvalidButton,
}

impl Move {
    pub fn from<S>(input: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let mut input = input.to_string();
        let jumping = if input.starts_with('j') {
            input.remove(0); // Remove 'j'
            true
        } else if input.starts_with("j.") {
            input.remove(0); // Remove 'j'
            input.remove(0); // Remove '.'
            true
        } else {
            false
        };
        let motion = Motion::from(
            &input
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>(),
        )?;
        let button = Button::from(
            &input
                .chars()
                .skip_while(|c| c.is_numeric())
                .collect::<String>(),
        )?;

        Ok(Self {
            jumping,
            motion,
            button,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.jumping { "j" } else { "" },
            self.motion.0,
            self.button.0
        )
    }
}

impl FromStr for Move {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s)
    }
}

impl Button {
    pub fn from<S>(b: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let b = b.to_string();
        if !b.chars().all(|c| c.is_ascii_alphabetic()) {
            Err(CreationError::InvalidButton)
        } else {
            Ok(Self(b))
        }
    }
}

impl FromStr for Button {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s)
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Motion {
    pub fn from<S>(m: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let m = m.to_string();

        if m.is_empty() {
            return Ok(Self("5".to_string()));
        }

        if !m.chars().all(|c| c.is_ascii_digit()) {
            Err(CreationError::InvalidMotion)
        } else {
            Ok(Self(m))
        }
    }
}

impl FromStr for Motion {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s)
    }
}

impl fmt::Display for Motion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j236h() {
        let attack = "j236H";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                jumping: true,
                motion: Motion("236".to_string()),
                button: Button("H".to_string())
            }
        )
    }

    #[test]
    fn heavy_dp() {
        let attack = "623Hp";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                jumping: false,
                motion: Motion("623".to_string()),
                button: Button("Hp".to_string())
            }
        )
    }

    #[test]
    fn jl() {
        let attack = "jL";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                jumping: true,
                motion: Motion("5".to_string()),
                button: Button("L".to_string())
            }
        )
    }

    #[test]
    fn move_tostring() {
        let m = Move::from("214L").unwrap();
        assert_eq!(m.to_string(), "214L".to_string());
    }

    #[test]
    fn button_creation() {
        let button = "HS";
        let created = Button::from(button).unwrap();

        assert_eq!(created, Button("HS".to_string()));
    }

    #[test]
    #[should_panic]
    fn invalid_button_fails() {
        let invalid = "69lol";

        Button::from(invalid).unwrap();
    }

    #[test]
    fn motion_creation() {
        let motion = "236";
        let created = Motion::from(motion).unwrap();

        assert_eq!(created, Motion("236".to_string()));
    }

    #[test]
    #[should_panic]
    fn invalid_motion_fails() {
        let invalid = "balls22";

        Motion::from(invalid).unwrap();
    }

    #[test]
    fn no_motion() {
        let m = Motion::from("").unwrap();

        println!("{m}")
    }
}
