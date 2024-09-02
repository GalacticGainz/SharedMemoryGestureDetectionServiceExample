use crate::core::direction_result::DirectionResult;
use crate::core::generic_gesture_type::{GenericGestureType, ALL_GESTURE_TYPES_POSSIBLE};
use crate::core::gesture_detection::GestureDetection;
use crate::core::point::Point;

const DEFAULT_MINIMUM_HOLD_GESTURE_DURATION_THRESHOLD: i128 = 400;
const MINIMUM_CIRCLE_RADIUS: f64 = 30f64;
const MINIMUM_DRAG_LENGTH: f64 = 100f64;

pub fn determine_gesture_dummy(
    touch_points: &mut Vec<Point>,
    gesture_detection: &mut GestureDetection,
) -> (GenericGestureType, Option<DirectionResult>) {
    (GenericGestureType::Click, None)
}