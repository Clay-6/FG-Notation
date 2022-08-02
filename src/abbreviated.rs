use core::fmt;
use std::str::FromStr;

use crate::CreationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    button: Button,
    motion: Motion,
    modifier: Option<Modifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Move {
    pub fn from<S>(input: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let mut input = input.to_string().trim().to_string();
        let modifier = Self::get_modifier(&mut input)?;
        let input = input.split_whitespace().collect::<Vec<&str>>();
        let motion = if input.len() > 1 {
            Motion::from(input[0])?
        } else {
            Motion::N
        };
        let button = Button::from(input.last().unwrap())?;

        Ok(Self {
            button,
            motion,
            modifier,
        })
    }

    fn get_modifier(input: &mut String) -> Result<Option<Modifier>, CreationError> {
        if input.contains('.') {
            let prefix = input.chars().take_while(|c| *c != '.').collect::<String>();
            for _ in 0..prefix.len() {
                (*input).remove(0);
            }
            (*input).remove(0);
            Ok(Some(Modifier::from(prefix)?))
        } else {
            Ok(None)
        }
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
    pub fn from<S>(m: S) -> Result<Self, CreationError>
    where
        S: ToString,
    {
        let m = m.to_string();
        match m.to_lowercase().as_str() {
            "n" => Ok(Self::N),
            "u" => Ok(Self::U),
            "d" => Ok(Self::D),
            "b" => Ok(Self::B),
            "f" => Ok(Self::F),
            "ub" | "u/b" => Ok(Self::UB),
            "uf" | "u/f" => Ok(Self::UF),
            "db" | "d/b" => Ok(Self::DB),
            "df" | "d/f" => Ok(Self::DF),
            "qcf" => Ok(Self::QCF),
            "qcb" => Ok(Self::QCB),
            "hcf" => Ok(Self::HCF),
            "hcb" => Ok(Self::HCB),
            "360" => Ok(Self::FullCircle),
            "720" => Ok(Self::Double360),
            _ => Err(CreationError::InvalidMotion),
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
        }
    }
}

impl FromStr for Modifier {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s)
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
        };
        write!(f, "{}", prefix)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qcf_hp() {
        let attack = "qcf HP";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("HP".to_string()),
                motion: Motion::QCF,
                modifier: None
            }
        )
    }

    #[test]
    fn cr_mk() {
        let attack = "cr.mk";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("mk".to_string()),
                motion: Motion::N,
                modifier: Some(Modifier::Crouching)
            },
        )
    }

    #[test]
    fn tk_qcf_hk() {
        let attack = "tk.qcf HK";
        let created = Move::from(attack).unwrap();

        assert_eq!(
            created,
            Move {
                button: Button("HK".to_string()),
                motion: Motion::QCF,
                modifier: Some(Modifier::TigerKnee)
            }
        )
    }
}
