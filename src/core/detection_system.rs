use crate::core::detect_gesture_dummy::determine_gesture_dummy;
use crate::core::direction::Direction;
use crate::core::direction_result::DirectionResult;
use crate::core::generic_gesture_type::GenericGestureType;
use crate::core::gesture_detection::GestureDetection;
use crate::core::point::Point;
use crate::core::rotation_direction::RotationDirection;
use crate::core::detection_system::ClientRequest::{Begin, ClearAcknowledgement, Detect, Halt, Ping, Uninitialized};
use crate::core::detection_system::ServerState::{DetectionComplete, DetectingGesture, WaitingForArgs, PingAcknowledged, Exited, WaitingForCommand};
use std::ffi::c_void;
use std::ptr;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::thread::{JoinHandle, Thread};
use std::time::{Duration, Instant};
use nix::libc::{exit, servent};

/// Define constants for states and requests
pub(crate) const IS_ALIVE: i64 = 1;
/// Define constants for states and requests
pub(crate) const NOT_ALIVE: i64 = 0;

/// The number of points that can be stored in the shared memory
const POINT_ARRAY_SIZE: usize = ((1024 - 8) / 2) as usize;


/// Bytewise offset to load the duration argument
const DURATION_OFFSET: u32 = 24;

/// Bytewise offset to load the fling direction argument
const FLING_DIRECTION_OFFSET: u32 = 40;

/// Bytewise offset to load the point array size argument
const POINT_ARRAY_SIZE_OFFSET: u32 = 48;

/// Bytewise offset to load the result argument
const RESULT_OFFSET: u32 = 56;

/// Bytewise offset to load the point array start argument
/// The last field in the shared memory space is the point array size, which is a 64-bit integer.
pub const POINT_ARRAY_OFFSET: u8 = 64;

/// Bitwise offset to bit-shift for a click result
const CLICK_INDEX_BITWISE_OFFSET: u32 = 0;


/// Bitwise offset to bit-shift for a boomerang result
const BOOMERANG_BITWISE_OFFSET: u32 = 2;


/// Bitwise offset to bit-shift for a swipe result
const SWIPE_BITWISE_OFFSET: u32 = 10;


/// Bitwise offset to bit-shift for a circle result
const CIRCLE_BITWISE_OFFSET: u32 = 18;


/// Bitwise offset to bit-shift for an error result
const ERROR_BITWISE_OFFSET: u32 = 20;


#[derive(Debug, Clone, Copy)]
enum ServerState {
    WaitingForCommand,
    WaitingForArgs,
    DetectingGesture,
    PingAcknowledged,
    DetectionComplete,
    Exited,
}

impl ServerState {
    pub(crate) fn java_ordinal(&self) -> i64 {
        match self {
            ServerState::WaitingForCommand => 0,
            WaitingForArgs => 1,
            ServerState::DetectingGesture => 2,
            ServerState::PingAcknowledged => 3,
            ServerState::DetectionComplete => 4,
            ServerState::Exited => 5,
        }
    }
    pub(crate) fn from_java_ordinal(ordinal: i64) -> Self {
        match ordinal {
            0 => ServerState::WaitingForCommand,
            1 => WaitingForArgs,
            2 => ServerState::DetectingGesture,
            3 => ServerState::PingAcknowledged,
            4 => ServerState::DetectionComplete,
            5 => ServerState::Exited,
            _ => ServerState::WaitingForCommand,
        }
    }
    pub(crate) fn from_string(string: &str) -> Self {
        match string {
            "WAITING_FOR_COMMAND" => ServerState::WaitingForCommand,
            "WAITING_FOR_ARGS" => WaitingForArgs,
            "DETECTING_GESTURE" => ServerState::DetectingGesture,
            "PING_ACKNOWLEDGED" => ServerState::PingAcknowledged,
            "DETECTION_COMPLETE" => ServerState::DetectionComplete,
            "EXITED" => ServerState::Exited,
            _ => ServerState::WaitingForCommand,
        }
    }
    pub(crate) fn name(&self) -> String {
        String::from(match self {
            ServerState::WaitingForCommand => "WAITING_FOR_COMMAND",
            WaitingForArgs => "WAITING_FOR_ARGS",
            ServerState::DetectingGesture => "DETECTING_GESTURE",
            ServerState::PingAcknowledged => "PING_ACKNOWLEDGED",
            ServerState::DetectionComplete => "DETECTION_COMPLETE",
            ServerState::Exited => "EXITED",
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum ClientRequest {
    Uninitialized,
    Ping,
    Begin,
    Detect,
    Halt,
    ClearAcknowledgement,
}

impl ClientRequest {
    pub fn java_ordinal(&self) -> i64 {
        match self {
            ClientRequest::Uninitialized => 0,
            ClientRequest::Ping => 1,
            ClientRequest::Begin => 2,
            ClientRequest::Detect => 3,
            ClientRequest::Halt => 4,
            ClientRequest::ClearAcknowledgement => 5,
        }
    }
    pub fn java_value_of(ordinal: i64) -> Self {
        match ordinal {
            0 => ClientRequest::Uninitialized,
            1 => ClientRequest::Ping,
            2 => ClientRequest::Begin,
            3 => ClientRequest::Detect,
            4 => ClientRequest::ClearAcknowledgement,
            5 => ClientRequest::Halt,
            _ => ClientRequest::Uninitialized,
        }
    }
    pub fn from_string(string: &str) -> Self {
        match string {
            "EMPTY" => ClientRequest::Uninitialized,
            "PING" => ClientRequest::Ping,
            "BEGIN" => ClientRequest::Begin,
            "DETECT" => ClientRequest::Detect,
            "CLEAR_ACKNOWLEDGEMENT" => ClientRequest::ClearAcknowledgement,
            "HALT" => ClientRequest::Halt,
            _ => ClientRequest::Uninitialized,
        }
    }
    pub fn name(&self) -> String {
        String::from(match self {
            Empty => "EMPTY",
            Ping => "PING",
            Begin => "BEGIN",
            Detect => "DETECT",
            ClearAcknowledgement => "CLEAR_ACKNOWLEDGEMENT",
            Halt => "HALT",
        })
    }
    pub fn from_java_ordinal(ordinal: i64) -> Self {
        match ordinal {
            0 => Uninitialized,
            1 => Ping,
            2 => Begin,
            3 => Detect,
            4 => ClearAcknowledgement,
            5 => Halt,
            _ => Uninitialized,
        }
    }
}

/// This enum represents the offsets for each field in the shared memory space.
enum SharedMemoryOffset {
    IsAlive = 0,
    ServiceState = 8,
    ClientRequest = 16,
    Duration = 24,
    FlingDirection = 40,
    PointArraySize = 48,
    Result = 56,
    PointArrayStart = 64,
}


/// This struct represents the memory space used for the entire communication layer between this
/// detection service and the client.
#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct GestureDetectionIPCBuffer {
    pub(crate) is_alive: i64, // offset=0, +8
    pub(crate) server_state: i64, // offset=8, +8
    client_request: i64, // offset=16, +8
    duration: i128, // offset=24, +16

    /// The implied direction of the gesture at its last known position.
    fling_direction: i64, // offset=40, +8
    point_array_size: i64, // offset=48, +8
    result: i64, // offset=56, +8
    point_array_start: i64, // offset=64, +8
}

impl GestureDetectionIPCBuffer {
    pub fn new() -> Self {
        GestureDetectionIPCBuffer {
            is_alive: IS_ALIVE,
            server_state: WaitingForCommand.java_ordinal(),
            client_request: Uninitialized.java_ordinal(),
            duration: 0,
            fling_direction: -1,
            point_array_size: 0,
            result: 0,
            point_array_start: POINT_ARRAY_OFFSET as i64,
        }
    }
    /// When a client has provided the starting address of the shared memory space, this function
    /// will create a new instance by casting the address to a pointer to this struct.
    pub unsafe fn from_raw(address: *mut c_void) -> Self {
        (&mut *(address as *mut GestureDetectionIPCBuffer)).clone()
    }
}


/// Simple wrapper to put the memory struct into
#[derive(Clone, Debug)]
struct GestureDetectionIPC {
    memory: Arc<Mutex<GestureDetectionIPCBuffer>>,
}
impl GestureDetectionIPC {
    pub fn new(memory: Arc<Mutex<GestureDetectionIPCBuffer>>) -> Self {
        GestureDetectionIPC { memory }
    }

    /// Internal helper method to compute the bitwise offset for a result that requires a Direction.
    pub(crate) fn int_pow2(exponent: u32) -> u32 {
        let mut result = 1;
        for _ in 0..exponent {
            result *= 2;
        }
        result
    }

    /// Internal helper function so that procedures can be defied as closures and executed on the
    /// shared memory space.
    fn with_buffer(&self, function: fn(MutexGuard<GestureDetectionIPCBuffer>) -> i64) -> i64 {
        function(self.memory.lock().unwrap())
    }
}

#[derive(Clone, Debug)]
struct GestureDetectionServer {
    ipc: GestureDetectionIPC,
}

impl GestureDetectionServer {
    pub(crate) fn stop(&self) {
        todo!()
    }
}

impl GestureDetectionServer {
    /// Used by the server, callable from JNI, to update the server's state.
    pub(crate) fn set_server_state(&self, new_server_state: ServerState) {
        self.ipc.memory.lock().unwrap().server_state = new_server_state.java_ordinal();
    }

    /// Internal helper method to compute the bitwise offset for a result that requires a Direction.
    fn int_pow2(exponent: u32) -> u32 {
        let mut result = 1;
        for _ in 0..exponent {
            result *= 2;
        }
        result
    }

    /// Internal helper method to compute the bitwise offset for a result that requires a Direction.
    fn directional_offset_for(maybe_direction: Option<DirectionResult>, i: u32) -> u32 {
        match maybe_direction {
            None => { Self::int_pow2(ERROR_BITWISE_OFFSET) }
            Some(direction_result) => {
                match direction_result {
                    DirectionResult::Drag(direction) => Self::int_pow2(Direction::java_ordinal(direction) + i),
                    DirectionResult::Circular(rotation) => Self::int_pow2(RotationDirection::java_ordinal(rotation)),
                    _ => Self::int_pow2(ERROR_BITWISE_OFFSET),
                }
            }
        }
    }


    /// Used by the server, callable from JNI, to run the gesture detection algorithm.
    fn detect_gesture(&self) -> i64 {
        let mut service_buffer = self.ipc.memory.lock().unwrap();

        service_buffer.fling_direction = DetectingGesture.java_ordinal();
        let duration = service_buffer.duration;
        let size = service_buffer.point_array_size;
        let fling_direction = service_buffer.fling_direction;
        let first_coordinate = service_buffer.point_array_start;
        let mut coordinates: [f64; POINT_ARRAY_SIZE] = [-1.0; POINT_ARRAY_SIZE];

        for i in 0..size as usize {
            coordinates[i] = unsafe {
                ptr::read(coordinates.as_ptr()
                    .wrapping_offset((first_coordinate + i as i64) as isize))
            };
        }

        println!("Detecting gesture with {} points.", size);

        let mut detection = GestureDetection::new(
            duration,
            coordinates.chunks(2).map(|chunk| Point::new(chunk[0], chunk[1])).collect::<Vec<_>>(),
            if fling_direction == -1 { Option::None } else { Some(Direction::from_index(fling_direction as usize)) },
        );

        let result = match determine_gesture_dummy(
            &mut detection.touch_points.clone(),
            &mut detection,
        ) {
            (generic_gesture_type, maybe_direction) => {
                match generic_gesture_type {
                    GenericGestureType::Click => Self::int_pow2(CLICK_INDEX_BITWISE_OFFSET),
                    GenericGestureType::Hold => Self::int_pow2(1),
                    GenericGestureType::Boomerang => Self::directional_offset_for(maybe_direction, BOOMERANG_BITWISE_OFFSET),
                    GenericGestureType::Swipe => Self::directional_offset_for(maybe_direction, SWIPE_BITWISE_OFFSET),
                    GenericGestureType::Circle => {
                        match maybe_direction {
                            Some(dr) => match dr {
                                DirectionResult::Circular(rotation_direction) => {
                                    match rotation_direction {
                                        RotationDirection::Clockwise => { Self::int_pow2(CIRCLE_BITWISE_OFFSET) }
                                        RotationDirection::AntiClockwise => { Self::int_pow2(CIRCLE_BITWISE_OFFSET + 1) }
                                    }
                                }
                                DirectionResult::Drag(_) => Self::int_pow2(ERROR_BITWISE_OFFSET),
                                DirectionResult::None => Self::int_pow2(ERROR_BITWISE_OFFSET),
                            },
                            _ => Self::int_pow2(ERROR_BITWISE_OFFSET)
                        }
                    }
                }
            }
        };

        // Store the result
        service_buffer.result = result as i64;

        // Notify the client that the detection is complete
        service_buffer.server_state = DetectionComplete.java_ordinal();
        service_buffer.result
    }

    /// Used by the server, callable from JNI, to check if the client has made a request, process it,
    /// and update its state and the result if applicable.
    pub(crate) fn process_server_actions(&self) {
        let mut service_buffer = self.ipc.memory.lock().unwrap();
        loop {
            if service_buffer.is_alive == NOT_ALIVE {
                panic!("Server is shutting down.");
            }

            let request = service_buffer.client_request;

            match ClientRequest::from_java_ordinal(request) {
                Ping => {
                    if(service_buffer.server_state != WaitingForCommand.java_ordinal()) {
                        return;
                    }
                    println!("PING received. Acknowledging.");
                    service_buffer.server_state = PingAcknowledged.java_ordinal();
                }
                Begin => {
                    if(service_buffer.server_state != WaitingForCommand.java_ordinal()) {
                        return;
                    }
                    println!("BEGIN received. Initializing gesture detection.");
                    service_buffer.point_array_size = 0;
                    self.set_server_state(WaitingForCommand);
                }
                Detect => {
                    if(service_buffer.server_state != WaitingForArgs.java_ordinal()) {
                        return;
                    }
                    println!("DETECT received. Starting gesture detection.");
                    self.set_server_state(DetectingGesture);
                    self.detect_gesture();
                    self.set_server_state(DetectionComplete);
                }
                Halt => {
                    println!("HALT received. Exiting.");
                    let last_server_state = service_buffer.server_state as i32;
                    self.set_server_state(Exited);
                    service_buffer.is_alive = NOT_ALIVE;
                    unsafe { exit(last_server_state); }
                }
                ClearAcknowledgement => {
                    if(service_buffer.server_state != DetectionComplete.java_ordinal()
                      && service_buffer.server_state != PingAcknowledged.java_ordinal()) {
                        return;
                    }
                    println!("CLEAR_ACKNOWLEDGEMENT received. Acknowledging.");
                    self.set_server_state(WaitingForCommand);
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Debug)]
struct CompositeGestureDetectionClient {
    ipc: GestureDetectionIPC,
}

#[derive(Debug)]
pub struct GestureDetectionDaemon {
    client: CompositeGestureDetectionClient,
    server: GestureDetectionServer,
    thread: Option<JoinHandle<()>>,
    stop_signal: Arc<Mutex<bool>>, // Signal to stop the thread
}

impl GestureDetectionDaemon {
    pub fn new() -> Self {
        let buffer = GestureDetectionIPCBuffer::new();
        let stop_signal = Arc::new(Mutex::new(false));

        let ipc = GestureDetectionIPC::new(Arc::new(Mutex::new(buffer.clone())));
        let client = CompositeGestureDetectionClient { ipc: ipc.clone() };
        let server = GestureDetectionServer { ipc: ipc.clone() };

        let stop_signal_clone = Arc::clone(&stop_signal);

        let thread = Some(thread::spawn(move || {
            let server = GestureDetectionServer { ipc: GestureDetectionIPC::new(Arc::new(Mutex::new(buffer))).clone() };
            while !*stop_signal_clone.lock().unwrap() {
                server.process_server_actions();
                // Add a small sleep to prevent busy waiting
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }));

        GestureDetectionDaemon {
            client,
            server,
            thread,
            stop_signal,
        }
    }
    pub fn stop(&mut self) {
        if let Some(thread) = self.thread.take() {
            // Set the stop signal to true
            *self.stop_signal.lock().unwrap() = true;
            // Wait for the thread to finish
            thread.join().expect("Thread panicked during stop");
        }
    }
    pub fn detect(&self, duration: i128, fling_direction: i64, points: Vec<Point>) -> i64 {
        self.client.detect(duration, fling_direction, points)
    }
}

impl CompositeGestureDetectionClient {
    fn new(ipc: GestureDetectionIPC) -> Self {
        CompositeGestureDetectionClient { ipc }
    }

    /// Used by the client, callable from JNI, to start the server.
    /// This function will start the server in a new thread.
    pub fn start(address: i64) {
        let server = GestureDetectionServer {
            ipc: GestureDetectionIPC::new(
                Arc::new(
                    Mutex::new(
                        unsafe { GestureDetectionIPCBuffer::from_raw(address as *mut c_void) }
                    )
                )
            )
        };

        thread::spawn(move || { server.process_server_actions(); });
    }



    fn get_new_thread(server: GestureDetectionServer) -> JoinHandle<()> {
        thread::spawn(move || {
            server.process_server_actions();
        })
    }

    /// Used by the client, callable from JNI, to check if the server is alive.
    pub(crate) fn is_alive(&self) -> bool {
        self.ping();
        self.ipc.memory.lock().unwrap().server_state == IS_ALIVE
    }

    /// Used by the client, callable from JNI, to detect a gesture.
    /// Given a fling direction, duration, and a list of points, this function will send the
    /// detection request to the server and wait for the result.
    pub fn detect(&self, duration: i128, fling_direction: i64, points: Vec<Point>) -> i64 {
        println!("Detecting gesture with {} points.", points.len());
        let mut service_buffer = self.ipc.memory.lock().unwrap();
        println!("a");
        service_buffer.point_array_size = points.len() as i64;
        println!("b");
        service_buffer.point_array_start = SharedMemoryOffset::PointArrayStart as i64;
        println!("c");
        for (i, point) in points.iter().enumerate() {
            unsafe {
                ptr::write(
                    &mut ((service_buffer.point_array_start as f64).clone() + (i * 2) as f64),
                    point.x,
                );
                ptr::write(
                    &mut ((service_buffer.point_array_start as f64).clone() + (i * 2 + 1) as f64),
                    point.y,
                );
            }
        }
        println!("d");
        service_buffer.duration = duration;
        service_buffer.fling_direction = fling_direction;
        println!("e");
        let fling_direction = if points.len() > 1 {
            let last_point = points[points.len() - 1];
            let second_to_last_point = points[points.len() - 2];
            let fling_direction = Direction::angle_from(last_point, second_to_last_point);

            Direction::direction_from_alpha(fling_direction)
        } else {
            Direction::North
        };
        println!("f");
        service_buffer.fling_direction = fling_direction.java_ordinal() as i64;
        println!("g");
        self.block_until(Detect, DetectionComplete)
    }

    fn ping(&self) {
        self.block_until(Ping, PingAcknowledged);
    }

    fn acknowledge_begin(&self) {
        self.block_until(Begin, WaitingForArgs);
    }

    fn set_client_request(&self, request: ClientRequest) {
        print!("set_client_request");
        let mut service_buffer = self.ipc.memory.lock().unwrap();
        print!("got the buffer");
        service_buffer.client_request = request.java_ordinal();
        print!("set the request");
    }

    fn block_until(&self, request: ClientRequest, desired_server_state: ServerState) -> i64 {
        // println!("Blocking until {:?} is acknowledged.", desired_server_state);
        self.set_client_request(request);
        print!("f");
        let start_time = Instant::now();
        print!("g");
        let mut did_timeout = false;
        println!("server_state: {:?}, desired_server_state: {:?}", self.ipc.memory.lock().unwrap().server_state, desired_server_state.java_ordinal());
        while self.ipc.memory.lock().unwrap().server_state != desired_server_state.java_ordinal() {
            print!("f");
            thread::sleep(Duration::from_micros(100));
            print!("g");
            if start_time.elapsed().as_millis() > 40 {
                print!("TIMEOUT TIMEOUT TIMEOUT TIMEOUT TIMEOUT TIMEOUT TIMEOUT TIMEOUT.");
                did_timeout = true;
                break;
            }
        }
        print!("h");
        if did_timeout {
            println!("Detection timed out.");
            -1
        } else {
            println!(
                "Detection complete: {:?}",
                GenericGestureType::from_non_generic_gesture_int_value(self.ipc.memory.lock().unwrap().result as u8)
                    .unwrap()
                    .name()
            );
            match desired_server_state {
                    Exited => -1,
                    DetectionComplete => self.ipc.memory.lock().unwrap().result,
                _ => desired_server_state.java_ordinal()
            }
        }
    }
}