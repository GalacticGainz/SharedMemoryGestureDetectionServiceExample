# Shared Memory for Gesture Detection Sevices

## Introduction
The shared memory for this service works by designating specific i64 values at the beginning of the segment to be used
to track program state.

On the server side, the general program flow is to loop until the `is_alive` variable is set to `0`, and to check the `service_state`
variable to determine what action to take.
- Open shared memory.
  - Set `is_alive` = 1
  - Set `service_state` = `ServerState.WaitingForCommand`
  - Set `client_request` = `ClientRequest.NONE`
  - Set all coordinates to -1
- Loop
  - read the `service_state` variable
    - Follow the state transition table to determine what function to call. All functions will have access to the memory
    - and be able to update the values as needed for their procedure.

On the client side, the general program flow is more complex:
- Start the service
  - Open shared memory
    - Check that `is_alive` = 1. if not, stop trying to use this service or implement a restart
      strategy.
  - Set `client_state` = `ClientSate.UNINITIALIZED`
  - Set `client_request` = `PING`
  - Wait for the server to set `PingAcknowledged` to the `service_state` variable
    - If too much time elapses, choose a different gesture detection strategy and stop using this service.
  - Wait for the server to set `service_state` = `ServerState.WaitingForCommand`
    - If too much time elapses, choose a different gesture detection strategy and stop using this service.
- When a gesture is finished, write the `size`, `duration`, and `first_coordinate` variables to the shared memory
  - write any trailing points to the shared memory
- Set `client_request` = `DETECT`
- Wait for the server to set `service_state` = `DetectionComplete`
  - If too much time elapses, choose a different gesture detection strategy and stop using this service.
- Read the `result` variable to get the result of the gesture detection and convert it to a GestureType

Several values are stored at the beginning of the shared memory segment, followed by space to load points.
The shared memory holds these values in the following order:

| Variable name      | Rust Type | Java Type                            | Used to store                                                    |
|--------------------|-----------|--------------------------------------|------------------------------------------------------------------|
| `is_alive`         | i64       | JInt `->` `Boolean`                  | starts set to `1`, then changes to `0` before the program exits  |
| `service_state`    | i64       | JInt `->` `enum ServiceNotification` | state of the detection service itself                            |
| `client_request`   | i64       | JInt `->` `enum ClientRequest`       | a method call to `ping()`, `insert()`, `process()`, or `reset()` |
| `duration`         | l128      | JLong                                | duration of the gesture                                          |
| `size`             | i64       | JInt                                 | number of points in the gesture                                  |
| `result`           | i64       | JInt                                 | result of the computation                                        |
| `first_coordinate` | i64       | JFloat                               | address of the first point                                       |

### Point array size limitations
- The size of the memory allocated will be 1Kilobyte. The size of the points array is limited to (1024 - 8) / 2 points.
- This was calculated by summing the sizes of the `Integer Type` fields in the table above (minus one since for the `first_coordinate`'s
address).
- These coordinates are represented as consecutive floats spanning from the address of `first_coordinate` to the end of the memory segment.
- This is more than enough points to represent any reasonable gesture.
- Note that there must be an even number of coordinates in the array, so the actual number of points is half of the calculated
  value.

The detection service is always in exactly one of the following states, indicated by the `servers_state` variable:

| DetectionServiceState | Description                                                                                                        |
|-----------------------|--------------------------------------------------------------------------------------------------------------------|
| `WaitingForCommand` | The service app is waiting for a command from the client.                                                          |
| `BUSY`                | The service is busy running the detection algorithm.                                                               |
| `EXITING`             | The service is shutting down for some reason, and will write 255 to `is_alive` and `service_state` before exiting. |
| `PingAcknowledged`   | The detection service has finished processing the gesture and has stored the result in the `result` variable.      |
| `DetectionComplete`  | The detection service has finished processing the gesture and has stored the result in the `result` variable.      |

### ClientRequest
This is a list of commands that the clients may send by putting an appropriate `i64` in the `client_request` variable.
The service will read `client_request`, perform some action, and update its `service_notification` accordingly. Each
request also has an acknowledgment request to follow, so that the client and server stay in sync.

| Command       | Description                                                                                                                                                                          |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PING`        | Check that the detection service is running. Useful for the client to figure out if the service can be used (i.e., choose a different gesture detection if this app is not running). |
| `BEGIN`       | Start detecting a new gesture. This causes the server to fill all coordinates with -1                                                                                                |
| `DETECT`      | The detection service should start processing the gesture. The client app should have already populated the shared memory with the `points`, `size`, `duration` arguments.           |
| `CLEAR_ACKNOWLEDGEMENT` | Request that the service set its state back to `service_notification.EMPTY`.                                                                                                         |


- ## Server state transitions
The following transitions are allowed:

| ServiceState          | ClientRequest   |   New ServiceState             |
|-----------------------|-----------------|--------------------------------|
| `WaitingForCommand` | `PING`          | `PingAcknowledged`            |
| `PROVIDING_RESULT`    | `CLEAR_ACKNOWLEDGEMENT`   | `WaitingForCommand`          |
| `PingAcknowledged`   | `CLEAR_ACKNOWLEDGEMENT`   | `WaitingForCommand`          |
| `WaitingForCommand` | `DETECT`        | `BUSY`                         |

ignored state transitions:

| ServiceState          | ClientRequest   | New ServiceState (ignore)    |
|-----------------------|-----------------|------------------------------|
| `EXITING`             | `PING`          | `EXITING`                    |
| `PROVIDING_RESULT`    | `PING`          | `PROVIDING_RESULT`           |
| `PingAcknowledged`   | `PING`          | `PingAcknowledged`          |
| `WaitingForCommand` | `CLEAR_ACKNOWLEDGEMENT`   | `WaitingForCommand`        |
| `BUSY`                | `CLEAR_ACKNOWLEDGEMENT`   | `BUSY`                       |
| `EXITING`             | `CLEAR_ACKNOWLEDGEMENT`   | `EXITING`                    |
| `BUSY`                | `DETECT`        | `BUSY`                       |
| `EXITING`             | `DETECT`        | `EXITING`                    |
| `PROVIDING_RESULT`    | `DETECT`        | `PROVIDING_RESULT`           |
| `PingAcknowledged`   | `DETECT`        | `PingAcknowledged`          |

## NOTES
- If `is_alive` is ever set to `0`, the client should either attempt to restart the service, or choose another solution for
  gesture detection.

