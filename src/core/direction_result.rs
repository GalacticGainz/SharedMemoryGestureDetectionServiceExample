use crate::core::direction::Direction;
use crate::core::rotation_direction::RotationDirection;

#[derive(Debug, Copy, Clone)]
pub(crate) enum DirectionResult {
    Circular(RotationDirection),
    Drag(Direction),
    None,
}