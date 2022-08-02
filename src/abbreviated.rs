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
    U,
    D,
    F,
    B,
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
