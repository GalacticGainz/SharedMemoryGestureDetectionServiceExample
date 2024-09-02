use crate::core::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RotationDirection {
    Clockwise,
    AntiClockwise,
}

impl RotationDirection {
    pub fn name(self) -> String {
        String::from(
            match self {
                RotationDirection::Clockwise => "CLOCKWISE",
                RotationDirection::AntiClockwise => "ANTI_CLOCKWISE",
            }
        )
    }
    pub fn value_of(string: &str) -> Option<Self> {
        match string {
            "Clockwise"|"CLOCKWISE" => Some(RotationDirection::Clockwise),
            "AntiClockwise"|"ANTI_CLOCKWISE" => Some(RotationDirection::AntiClockwise),
            _ => None,
        }
    }
    pub(crate) fn java_ordinal(self) -> u32 {
        match self {
            RotationDirection::Clockwise => 0,
            RotationDirection::AntiClockwise => 1,
        }
    }
}