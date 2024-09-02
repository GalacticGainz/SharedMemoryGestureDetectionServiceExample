use crate::core::direction::Direction;
use crate::core::direction_result::DirectionResult;
use crate::core::generic_gesture_type::GenericGestureType;
use crate::core::point::Point;

/// Struct for encapsulating gesture detection requests and responses.
#[derive(Debug, Clone)]
pub struct GestureDetection {
    pub(crate) duration: i128,
    pub(crate) touch_points: Vec<Point>,
    pub(crate) direction_result: DirectionResult,
    pub(crate) possible_gestures: u8,  // Bitfield to represent the 5 possible generic gesture types being qualified/disqualified
    pub(crate) fling_direction: Option<Direction>,
}

impl GestureDetection {
    pub(crate) fn new(
        duration: i128,
        touch_points: Vec<Point>,
        fling_direction: Option<Direction>,
    ) -> Self {
        // Initialize possible gestures with all bits set to 1
        let possible_gestures = (1 << 4) - 1; // 4 gestures for 4 bits
        Self {
            duration,
            touch_points,
            direction_result: DirectionResult::None,
            possible_gestures,
            fling_direction,
        }
    }

    /// Remove gestures by clearing corresponding bits
    pub(crate) fn remove(&mut self,
                         reason: &str,
                         gestures: u8,
    ) {
        // print the reason
        println!("{}", reason);
        self.possible_gestures &= !gestures;
    }

    fn result(&self) -> (GenericGestureType, &DirectionResult) {
        // Find the gesture type with the highest bit set in possible_gestures
        let mut detected_gesture = GenericGestureType::Click; // Default to Click if no gesture is detected
        let mut highest_bit = 0;

        for gesture in GenericGestureType::values() {
            let bit_position = gesture.ordinal();
            let bit_mask = 1 << bit_position;

            if self.possible_gestures & bit_mask != 0 {
                // Update detected_gesture to the most significant bit set
                if bit_position > highest_bit {
                    detected_gesture = gesture;
                    highest_bit = bit_position;
                }
            }
        }


        // Return the detected gesture and a reference to direction_result
        (detected_gesture, &self.direction_result)
    }
}
