use core::fmt;
use std::str::FromStr;

use crate::{numpad, CreationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    button: Button,
    motion: Motion,
    modifier: Modifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Motion {
    N,
    U,
    D,
    B,
    F,
    DB,
    DF,
    UB,
    UF,
    QCF,
    QCB,
    HCF,
    HCB,
    DP,
    RDP,
    FullCircle,
    Double360,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    Close,
    Far,
    Standing,
    Crouching,
    Jump,
    SuperJump,
    JumpCancel,
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
        let input = input.split_whitespace().collect::<Vec<&str>>();
        let motion = if input.len() > 1 {
            Motion::new(input[0])
        } else {
            Motion::N
        };
        let button = Button::new(input.last().unwrap())?;

        Ok(Self {
            button,
            motion,
            modifier,
        })
    }

    fn get_modifier(input: &mut String) -> Result<Modifier, CreationError> {
        if input.contains('.') {
            let prefix = input.chars().take_while(|c| *c != '.').collect::<String>();
            for _ in 0..prefix.len() {
                (*input).remove(0);
            }
            (*input).remove(0);
            Ok(Modifier::new(prefix)?)
        } else {
            Ok(Modifier::None)
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
            "cl." | "cl" => Ok(Self::Close),
            "f." | "f" => Ok(Self::Far),
            "tk." | "tk" => Ok(Self::TigerKnee),
            "cr." | "cr" => Ok(Self::Crouching),
            "st." | "st" => Ok(Self::Standing),
            _ => Err(CreationError::InvalidModifier),
        }
    }
}

impl Motion {
    pub fn new<S>(m: S) -> Self
    where
        S: ToString,
    {
        let m = m.to_string();
        match m.to_lowercase().as_str() {
            "n" => Self::N,
            "u" => Self::U,
            "d" => Self::D,
            "b" => Self::B,
            "f" => Self::F,
            "ub" | "u/b" => Self::UB,
            "uf" | "u/f" => Self::UF,
            "db" | "d/b" => Self::DB,
            "df" | "d/f" => Self::DF,
            "qcf" => Self::QCF,
            "qcb" => Self::QCB,
            "hcf" => Self::HCF,
            "hcb" => Self::HCB,
            "360" => Self::FullCircle,
            "720" => Self::Double360,
            other => Self::Other(other.to_string()),
        }
    }
}

impl From<numpad::Motion> for Motion {
    fn from(m: numpad::Motion) -> Self {
        match m.to_string().as_str() {
            "5" | "" => Self::N,
            "8" => Self::U,
            "2" => Self::D,
            "4" => Self::B,
            "6" => Self::F,
            "7" => Self::UB,
            "9" => Self::UF,
            "1" => Self::DB,
            "3" => Self::DF,
            "236" => Self::QCF,
            "214" => Self::QCB,
            "41236" => Self::HCF,
            "63214" => Self::HCB,
            "41236987" => Self::FullCircle,
            "4123698741236987" => Self::Double360,
            other => Self::Other(other.to_string()),
        }
    }
}

impl FromStr for Motion {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl fmt::Display for Motion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Motion::N => write!(f, "N"),
            Motion::U => write!(f, "U"),
            Motion::D => write!(f, "D"),
            Motion::B => write!(f, "B"),
            Motion::F => write!(f, "F"),
            Motion::DB => write!(f, "DB"),
            Motion::DF => write!(f, "DF"),
            Motion::UB => write!(f, "UB"),
            Motion::UF => write!(f, "UF"),
            Motion::QCF => write!(f, "QCF"),
            Motion::QCB => write!(f, "QCB"),
            Motion::HCF => write!(f, "HCF"),
            Motion::HCB => write!(f, "HCB"),
            Motion::DP => write!(f, "DP"),
            Motion::RDP => write!(f, "RDP"),
            Motion::FullCircle => write!(f, "360"),
            Motion::Double360 => write!(f, "720"),
            Motion::Other(o) => write!(f, "'{}'", o),
        }
    }
}

impl From<numpad::Modifier> for Modifier {
    fn from(m: numpad::Modifier) -> Self {
        match m {
            numpad::Modifier::Jump => Self::Jump,
            numpad::Modifier::SuperJump => Self::SuperJump,
            numpad::Modifier::JumpCancel => Self::JumpCancel,
            numpad::Modifier::Close => Self::Close,
            numpad::Modifier::Far => Self::Far,
            numpad::Modifier::TigerKnee => Self::TigerKnee,
            numpad::Modifier::None => Self::None,
        }
    }
}

impl FromStr for Modifier {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self {
            Modifier::Close => "cl.",
            Modifier::Far => "f.",
            Modifier::Standing => "st.",
            Modifier::Crouching => "cr.",
            Modifier::Jump => "j.",
            Modifier::SuperJump => "sj.",
            Modifier::JumpCancel => "jc.",
            Modifier::TigerKnee => "tk.",
            Modifier::None => "",
        };
        write!(f, "{}", prefix)
    }
}

impl From<numpad::Button> for Button {
    fn from(b: numpad::Button) -> Self {
        Self(b.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qcf_hp() {
        let attack = "qcf HP";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("HP".to_string()),
                motion: Motion::QCF,
                modifier: Modifier::None
            }
        )
    }

    #[test]
    fn cr_mk() {
        let attack = "cr.mk";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("mk".to_string()),
                motion: Motion::N,
                modifier: Modifier::Crouching
            },
        )
    }

    #[test]
    fn tk_qcf_hk() {
        let attack = "tk.qcf HK";
        let created = Move::new(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("HK".to_string()),
                motion: Motion::QCF,
                modifier: Modifier::TigerKnee
            }
        )
    }
}
