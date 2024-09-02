use jni::objects::{JFloatArray, JObject};
use jni::sys::{jboolean, jfloat, jint, jlong, jsize};
use jni::JNIEnv;
use crate::core::detection_system::{GestureDetectionDaemon, GestureDetectionIPCBuffer, POINT_ARRAY_OFFSET};
use crate::core::point::Point;

#[no_mangle]
pub extern "C" fn Java_com_example_myapp_NativeService_start(env: JNIEnv, _: JObject, address: jint) -> jboolean {
    GestureDetectionDaemon::new();
    1
}

#[no_mangle]
extern "C" fn Java_com_example_myapp_NativeService_stop(env: JNIEnv, _: JObject, address: jint) {
    GestureDetectionDaemon::new().stop()
}

/// Offset to the touch points in the shared memory should be enough to accommodate all state bits
/// and args for the detect function, so the array will be the last thing in the shared memory.
#[no_mangle]
extern "C" fn Java_com_example_myapp_NativeService_detect(mut env: JNIEnv, _: JObject, address: *mut u8, fling_direction: jint, duration: jlong, size: usize) {
    let rust_slice = unsafe { std::slice::from_raw_parts_mut(address, size) }
        .iter()
        .map(|&byte| byte as f64)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| Point::new(chunk[0] as f64, chunk[1] as f64))
        .collect::<Vec<_>>();
    let mut gesture_detection_daemon = GestureDetectionDaemon::new();
    gesture_detection_daemon.detect(duration as i128, fling_direction as i64, rust_slice);
}