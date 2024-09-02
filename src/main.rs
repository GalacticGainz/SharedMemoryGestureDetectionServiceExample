#![allow(warnings)]
mod core;
mod build;

use crate::core::detection_system::{GestureDetectionDaemon, GestureDetectionIPCBuffer};

/// Main function demonstrating how to use the `GestureQueue`.
fn main() {
    let daemon = GestureDetectionDaemon::new();
    println!("buffer: {:?}", daemon);

    let result = daemon.detect(1000, 0, vec![core::point::Point::new(1.0, 2.0), core::point::Point::new(3.0, 4.0)]);
}



