use core::fmt;
use std::str::FromStr;

use crate::CreationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    modifier: Option<Modifier>,
    motion: Motion,
    button: Button,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Motion(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    Jump,
    SuperJump,
    JumpCancel,
    Close,
    Far,
    TigerKnee,
}

impl Move {
    pub fn from<S>(input: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let mut input = input.to_string().trim().to_string();
        let modifier = Self::get_modifier(&mut input)?;
        let motion = Motion::from(
            &input
                .chars()
                .take_while(|c| !c.is_ascii_alphabetic())
                .collect::<String>(),
        )?;
        let button = Button::from(
            &input
                .chars()
                .skip_while(|c| !c.is_ascii_alphabetic())
                .collect::<String>(),
        )?;

        Ok(Self {
            modifier,
            motion,
            button,
        })
    }

    fn get_modifier(input: &mut String) -> Result<Option<Modifier>, CreationError> {
        if input.contains('.') {
            let prefix = input.chars().take_while(|c| *c != '.').collect::<String>();
            for _ in 0..prefix.len() {
                (*input).remove(0); // Remove characters
            }
            (*input).remove(0); // Remove '.'
            Ok(Some(Modifier::from(prefix)?))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.modifier
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "".to_string()),
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
    #![allow(clippy::len_without_is_empty)] // Not possible for `Motion` to be empty

    pub fn from<S>(m: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let m = m.to_string();

        if m.is_empty() {
            return Ok(Self("5".to_string()));
        }

        if !m
            .chars()
            .all(|c| c.is_ascii_digit() || (c == '[' || c == ']'))
        {
            Err(CreationError::InvalidMotion)
        } else {
            Ok(Self(m))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_neutral(&self) -> bool {
        self.0 == "5"
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

impl Modifier {
    pub fn from<S>(m: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let m = m.to_string();

        match m.as_str() {
            "j." | "j" => Ok(Self::Jump),
            "sj." | "sj" => Ok(Self::SuperJump),
            "jc." | "jc" => Ok(Self::JumpCancel),
            "c." | "c" => Ok(Self::Close),
            "f." | "f" => Ok(Self::Far),
            "tk." | "tk" => Ok(Self::TigerKnee),
            _ => Err(CreationError::InvalidModifier),
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self {
            Modifier::Jump => "j.",
            Modifier::SuperJump => "sj.",
            Modifier::JumpCancel => "jc.",
            Modifier::Close => "c.",
            Modifier::Far => "f.",
            Modifier::TigerKnee => "tk.",
        };
        write!(f, "{}", prefix)
    }
}

impl FromStr for Modifier {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Modifier::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j236h() {
        let attack = "j.236H";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Some(Modifier::Jump),
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
                modifier: None,
                motion: Motion("623".to_string()),
                button: Button("Hp".to_string())
            }
        )
    }

    #[test]
    fn jl() {
        let attack = "j.L";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Some(Modifier::Jump),
                motion: Motion("5".to_string()),
                button: Button("L".to_string())
            }
        )
    }

    #[test]
    fn charge_move() {
        let attack = "[4]6A";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: None,
                motion: Motion("[4]6".to_string()),
                button: Button("A".to_string())
            }
        )
    }

    #[test]
    fn cs() {
        let attack = "c.S";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Some(Modifier::Close),
                motion: Motion("5".to_string()),
                button: Button("S".to_string())
            }
        )
    }

    #[test]
    fn superjump() {
        let attack = "sj.236S";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Some(Modifier::SuperJump),
                motion: Motion("236".to_string()),
                button: Button("S".to_string())
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
