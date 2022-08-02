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
