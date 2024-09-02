/// Enum representing possible gesture types.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GenericGestureType {
    Click,
    Hold,
    Swipe,
    Circle,
    Boomerang,
}

impl GenericGestureType {
    pub(crate) fn ordinal(&self) -> i64 {
        match self {
            GenericGestureType::Click => 0,
            GenericGestureType::Hold => 1,
            GenericGestureType::Swipe => 2,
            GenericGestureType::Circle => 3,
            GenericGestureType::Boomerang => 4,
        }
    }
}

pub(crate) const ALL_GESTURE_TYPES_POSSIBLE: u8 = 31u8;

impl GenericGestureType {
    pub(crate) fn name(self) -> String {
        String::from(
            match self {
                GenericGestureType::Click => "CLICK",
                GenericGestureType::Hold => "HOLD",
                GenericGestureType::Swipe => "SWIPE",
                GenericGestureType::Circle => "CIRCLE",
                GenericGestureType::Boomerang => "BOOMERANG",
            }
        )
    }
    pub(crate) fn values() -> Vec<Self> {
        vec![
            Self::Click,
            Self::Hold,
            Self::Swipe,
            Self::Circle,
            Self::Boomerang,
        ]
    }
    pub(crate) fn java_ordinal(&self) -> u8 {
        match self {
            GenericGestureType::Click => 1u8,
            GenericGestureType::Hold => 2u8,
            GenericGestureType::Swipe => 4u8,
            GenericGestureType::Circle => 8u8,
            GenericGestureType::Boomerang => 16u8,
        }
    }
    pub(crate) fn from_int_value(value: u8) -> Option<GenericGestureType> {
        match value {
            1 => Some(GenericGestureType::Click),
            2 => Some(GenericGestureType::Hold),
            4 => Some(GenericGestureType::Swipe),
            8 => Some(GenericGestureType::Circle),
            16 => Some(GenericGestureType::Boomerang),
            _ => None
        }
    }
    pub(crate) fn value_of(string: &str) -> Option<Self> {
        match string {
            "Click"|"CLICK" => Some(GenericGestureType::Click),
            "Hold"|"HOLD" => Some(GenericGestureType::Hold),
            "Swipe"|"SWIPE" => Some(GenericGestureType::Swipe),
            "Circle"|"CIRCLE" => Some(GenericGestureType::Circle),
            "Boomerang"|"BOOMERANG" => Some(GenericGestureType::Boomerang),
            _ => None,
        }
    }
    pub(crate)fn from_non_generic_gesture_int_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(GenericGestureType::Click),
            2 => Some(GenericGestureType::Hold),
            4..11 => Some(GenericGestureType::Swipe),
            8..15 => Some(GenericGestureType::Boomerang),
            16..18 => Some(GenericGestureType::Circle),
            _ => None
        }
    }
}