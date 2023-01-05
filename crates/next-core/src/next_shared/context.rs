use crate::{next_client::context::ClientContextType, next_server::context::ServerContextType};

#[turbo_tasks::value(serialization = "auto_for_input")]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum SharedContextType {
    Server(ServerContextType),
    Client(ClientContextType),
}
