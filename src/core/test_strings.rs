use crate::core::direction_result::DirectionResult;
use crate::core::generic_gesture_type::GenericGestureType;
use crate::core::point::Point;

pub(crate) enum TestStringPart {
    Answer{generic_gesture_type: GenericGestureType, direction_result: DirectionResult},
    TouchPointList{touch_points: Vec<Point>},
    Duration{duration: u64},
}