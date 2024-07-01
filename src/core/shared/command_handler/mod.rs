use async_trait::async_trait;


#[async_trait]
pub trait CommandHandler<STATE, COMMAND, EVT> {
    async fn handle(&self, cmd: COMMAND) -> EVT;
}

#[async_trait]
impl<STATE, COMMAND, EVT> CommandHandler<STATE, COMMAND, EVT> for dyn CommandHandlerCreate<STATE, COMMAND, EVT>
where
    EVT: Send + Sync + Clone,
    COMMAND: Send + Sync + Clone
{
    async fn handle(&self, cmd: COMMAND) -> EVT {
        self.on_command(cmd).await.clone()
    }
}

#[async_trait]
pub trait CommandHandlerCreate<STATE, COMMAND, EVT>: Send + Sync {

    async fn on_command(&self, command: COMMAND) -> EVT;

}

#[async_trait]
pub trait CommandHandlerUpdate<STATE, COMMAND, EVT> {

    async fn on_command(&self, command: COMMAND) -> EVT;

}

struct A;

#[async_trait]
impl CommandHandlerCreate<String, String, String> for A {
    async fn on_command(&self, command: String) -> String {
        "todo!()".to_string()
    }
}

struct B;
#[async_trait]
impl CommandHandlerUpdate<String, String, String> for B {
    async fn on_command(&self, command: String) -> String {
        "todo!()".to_string()
    }
}

