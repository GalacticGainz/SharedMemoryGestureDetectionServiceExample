use crate::core::direction::Direction;
use crate::core::point::Point;

pub(crate) enum ClientCommand {
    Collect{ point: Point },
    Insert{ point: Point },
    Detect {
        duration: u64,
        count: u8,
        /* fits in thi u8 primitive because we have exactly 8 directions */
        fling_direction: Option<Direction>,
    },
    Reset,
}

impl ClientCommand {
    /// Retrieve the next command from the client by reading the first 64 bits (8 bytes) from the shared memory.
    pub(crate) fn retrieve_next_command() {
        // Read the first 64 bits from the shared memory
        // and interpret them as a ClientCommand
    }
}