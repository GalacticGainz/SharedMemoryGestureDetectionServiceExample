use crate::core::client_command::ClientCommand;
use crate::core::server_state::ServerState;

pub(crate) enum GestureDetectorState {
    Client(ClientCommand),
    Server(ServerState),
}

impl GestureDetectorState {}