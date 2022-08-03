use core::fmt;
use std::str::FromStr;

use crate::{abbreviated, CreationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    modifier: Modifier,
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
    None,
}

impl Move {
    pub fn new<S>(input: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let mut input = input.to_string().trim().to_string();
        let modifier = Self::get_modifier(&mut input)?;
        let motion = Motion::new(
            &input
                .chars()
                .take_while(|c| !c.is_ascii_alphabetic())
                .collect::<String>(),
        )?;
        let button = Button::new(
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

    pub fn button(&self) -> Button {
        self.button.clone()
    }

    pub fn motion(&self) -> Motion {
        self.motion.clone()
    }

    pub fn modifier(&self) -> Modifier {
        self.modifier
    }

    fn get_modifier(input: &mut String) -> Result<Modifier, CreationError> {
        if input.contains('.') {
            let prefix = input.chars().take_while(|c| *c != '.').collect::<String>();
            for _ in 0..prefix.len() {
                (*input).remove(0); // Remove characters
            }
            (*input).remove(0); // Remove '.'
            Ok(Modifier::new(prefix)?)
        } else {
            Ok(Modifier::None)
        }
    }
}

impl Modifier {
    pub fn new<S>(m: S) -> Result<Self, CreationError>
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

impl Button {
    pub fn new<S>(b: S) -> Result<Self, CreationError>
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

impl Motion {
    #![allow(clippy::len_without_is_empty)] // Not possible for `Motion` to be empty

    pub fn new<S>(m: S) -> Result<Self, CreationError>
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

impl From<abbreviated::Move> for Move {
    fn from(m: abbreviated::Move) -> Self {
        let button = Button::from(m.button());
        let a_mod = m.modifier();
        let motion = if a_mod == abbreviated::Modifier::Standing {
            Motion::new("5").unwrap()
        } else if a_mod == abbreviated::Modifier::Crouching {
            Motion::new("2").unwrap()
        } else {
            Motion::from(m.motion())
        };
        let modifier = Modifier::from(a_mod);

        Self {
            button,
            motion,
            modifier,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.modifier, self.motion.0, self.button.0)
    }
}

impl FromStr for Move {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<abbreviated::Button> for Button {
    fn from(b: abbreviated::Button) -> Self {
        Button(b.to_string())
    }
}

impl FromStr for Button {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<abbreviated::Motion> for Motion {
    fn from(m: abbreviated::Motion) -> Self {
        match m {
            abbreviated::Motion::N => Self("5".to_string()),
            abbreviated::Motion::U => Self("8".to_string()),
            abbreviated::Motion::D => Self("2".to_string()),
            abbreviated::Motion::B => Self("4".to_string()),
            abbreviated::Motion::F => Self("6".to_string()),
            abbreviated::Motion::DB => Self("1".to_string()),
            abbreviated::Motion::DF => Self("3".to_string()),
            abbreviated::Motion::UB => Self("7".to_string()),
            abbreviated::Motion::UF => Self("9".to_string()),
            abbreviated::Motion::QCF => Self("236".to_string()),
            abbreviated::Motion::QCB => Self("214".to_string()),
            abbreviated::Motion::HCF => Self("41236".to_string()),
            abbreviated::Motion::HCB => Self("63214".to_string()),
            abbreviated::Motion::DP => Self("623".to_string()),
            abbreviated::Motion::RDP => Self("421".to_string()),
            abbreviated::Motion::FullCircle => Self("41236987".to_string()),
            abbreviated::Motion::Double360 => Self("4123698741236987".to_string()),
            abbreviated::Motion::Other(o) => Self::new(o).unwrap(),
        }
    }
}

impl FromStr for Motion {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Motion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<abbreviated::Modifier> for Modifier {
    fn from(m: abbreviated::Modifier) -> Self {
        match m {
            abbreviated::Modifier::Close => Self::Close,
            abbreviated::Modifier::Far => Self::Far,
            abbreviated::Modifier::Standing => Self::None,
            abbreviated::Modifier::Crouching => Self::None,
            abbreviated::Modifier::Jump => Self::Jump,
            abbreviated::Modifier::SuperJump => Self::SuperJump,
            abbreviated::Modifier::JumpCancel => Self::JumpCancel,
            abbreviated::Modifier::TigerKnee => Self::TigerKnee,
            abbreviated::Modifier::None => Self::None,
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
            Modifier::None => "",
        };
        write!(f, "{}", prefix)
    }
}

impl FromStr for Modifier {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Modifier::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j236h() {
        let attack = "j.236H";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::Jump,
                motion: Motion("236".to_string()),
                button: Button("H".to_string())
            }
        )
    }

    #[test]
    fn heavy_dp() {
        let attack = "623Hp";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::None,
                motion: Motion("623".to_string()),
                button: Button("Hp".to_string())
            }
        )
    }

    #[test]
    fn jl() {
        let attack = "j.L";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::Jump,
                motion: Motion("5".to_string()),
                button: Button("L".to_string())
            }
        )
    }

    #[test]
    fn charge_move() {
        let attack = "[4]6A";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::None,
                motion: Motion("[4]6".to_string()),
                button: Button("A".to_string())
            }
        )
    }

    #[test]
    fn cs() {
        let attack = "c.S";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::Close,
                motion: Motion("5".to_string()),
                button: Button("S".to_string())
            }
        )
    }

    #[test]
    fn superjump() {
        let attack = "sj.236S";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                modifier: Modifier::SuperJump,
                motion: Motion("236".to_string()),
                button: Button("S".to_string())
            }
        )
    }

    #[test]
    fn move_tostring() {
        let m = Move::new("214L").unwrap();
        assert_eq!(m.to_string(), "214L".to_string());
    }

    #[test]
    fn button_creation() {
        let button = "HS";
        let created = Button::new(button).unwrap();

        assert_eq!(created, Button("HS".to_string()));
    }

    #[test]
    #[should_panic]
    fn invalid_button_fails() {
        let invalid = "69lol";

        Button::new(invalid).unwrap();
    }

    #[test]
    fn motion_creation() {
        let motion = "236";
        let created = Motion::new(motion).unwrap();

        assert_eq!(created, Motion("236".to_string()));
    }

    #[test]
    #[should_panic]
    fn invalid_motion_fails() {
        let invalid = "balls22";

        Motion::new(invalid).unwrap();
    }

    #[test]
    fn no_motion() {
        let m = Motion::new("").unwrap();

        println!("{m}")
    }
}
