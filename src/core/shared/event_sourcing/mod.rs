pub mod engine;

use async_trait::async_trait;
use crate::core::shared::context::Context;

pub enum CommandHandler<STATE, COMMAND, EVT> {
    Create(Box<dyn CommandHandlerCreate<STATE, COMMAND, EVT>>),
    Update(Box<dyn CommandHandlerUpdate<STATE, COMMAND, EVT>>),
}

#[async_trait]
pub trait CommandHandlerCreate<STATE, COMMAND, EVT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, id: String, command: COMMAND, context: Context) -> Result<EVT, String>;
}

#[async_trait]
pub trait CommandHandlerUpdate<STATE, COMMAND, EVT>: Send + Sync {
    fn name(&self) -> String;
    async fn on_command(&self, id: String, state: STATE, command: COMMAND, context: Context) -> Result<EVT, String>;
}
