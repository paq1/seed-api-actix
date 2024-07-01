use async_trait::async_trait;
use crate::core::shared::command_handler::{CommandHandlerCreate, CommandHandler, CommandHandlerUpdate};
use crate::core::shared::context::Context;
use crate::core::todos::data::{TodoEvents, TodoStates, UpdatedEvent};
use crate::models::todos::commands::TodoCommands;

struct CreateTodoHandler;

#[async_trait]
impl CommandHandlerCreate<TodoStates, TodoCommands, TodoEvents> for CreateTodoHandler {
    fn name(&self) -> String {
        "create".to_string()
    }

    async fn on_command(&self, id: String, command: TodoCommands, context: Context) -> Result<TodoEvents, String> {
        println!("pouet");

        match command {
            TodoCommands::Create(c) => Ok(
                TodoEvents::Created { by: context.subject, at: context.now, name: c.name }
            ),
            _ => Err("bad request".to_string())
        }
    }
}

pub struct UpdateTodoHandler;
#[async_trait]
impl CommandHandlerUpdate<TodoStates, TodoCommands, TodoEvents> for UpdateTodoHandler {
    fn name(&self) -> String {
        "create".to_string()
    }

    async fn on_command(&self, id: String, state: TodoStates, command: TodoCommands, context: Context) -> Result<TodoEvents, String> {
        println!("pouet");

        match command {
            TodoCommands::Update(c) => Ok(
                TodoEvents::Updated (UpdatedEvent {by: context.subject, at: context.now, name: c.name})
            ),
            _ => Err("bad request".to_string())
        }
    }
}


fn wip() {
    let x: Vec<CommandHandler<TodoStates, TodoCommands, TodoEvents>> = vec!(
        CommandHandler::Create(Box::new(CreateTodoHandler {})),
        CommandHandler::Update(Box::new(UpdateTodoHandler {}))
    );
}
