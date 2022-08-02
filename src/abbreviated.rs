use core::fmt;
use std::str::FromStr;

use crate::CreationError;

pub struct Move {
    button: Button,
    motion: Motion,
    modifier: Modifier,
}

pub struct Button(String);

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
