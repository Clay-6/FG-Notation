#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    jumping: bool,
    motion: Motion,
    button: Button,
}

impl Move {
    pub fn new<S>(input: S) -> Result<Self, &'static str>
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Motion(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(String);

impl Button {
    pub fn from(b: &str) -> Result<Self, &'static str> {
        if !b.chars().all(|c| c.is_alphabetic()) {
            Err("Button inputs can only contain alphabetic characters")
        } else {
            Ok(Self(b.to_string()))
        }
    }
}
impl Motion {
    pub fn from(m: &str) -> Result<Self, &'static str> {
        if !m.chars().all(|c| c.is_numeric()) {
            Err("Motion inputs can only contain numeric characters")
        } else {
            Ok(Self(m.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j236h() {
        let attack = "j236H";
        let created = Move::new(attack).unwrap();

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
        let created = Move::new(attack).unwrap();

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
}
