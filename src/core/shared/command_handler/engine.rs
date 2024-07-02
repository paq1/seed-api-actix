use crate::core::shared::command_handler::CommandHandler;
use crate::core::shared::context::Context;
use crate::core::shared::daos::{ReadOnlyEntityRepo, WriteOnlyEntityRepo, WriteOnlyEventRepo};
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::reducer::Reducer;

pub struct Engine<
    STATE,
    COMMAND,
    EVENT,
    STORE,
    JOURNAL
>
where
    STORE: WriteOnlyEntityRepo<STATE, String> + ReadOnlyEntityRepo<STATE, String>,
    JOURNAL: WriteOnlyEventRepo<EVENT, String>,
{
    handlers: Vec<CommandHandler<STATE, COMMAND, EVENT>>,
    reducer: Reducer<EVENT, STATE>,
    store: STORE,
    journal: JOURNAL
}

impl<STATE, COMMAND, EVENT, STORE, JOURNAL> Engine<STATE, COMMAND, EVENT, STORE, JOURNAL>
where
    STATE: Clone,
    EVENT: Clone,
    STORE: WriteOnlyEntityRepo<STATE, String> + ReadOnlyEntityRepo<STATE, String>,
    JOURNAL: WriteOnlyEventRepo<EVENT, String>,
{
    pub async fn compute(self, command: COMMAND, entity_id: String, name: String, context: Context) -> Result<String, String> {
        let command_handler_found = self
            .handlers
            .iter().find(|handler| {
            match handler {
                CommandHandler::Create(created) => created.clone().name() == name,
                CommandHandler::Update(updated) => updated.clone().name() == name
            }
        })
            .ok_or("pas de gestionnaire pour cette commande".to_string())?; // fixme changer l'erreur

        let maybe_entity = self.store.fetch_one(entity_id.clone()).await?;
        let maybe_state = maybe_entity.clone().map(|entity| entity.data);

        let event = match command_handler_found {
            CommandHandler::Create(x) => x.on_command(entity_id.clone(), command, context).await,
            CommandHandler::Update(x) => {
                let state = maybe_state.clone().ok_or("resource not found".to_string())?;

                x.on_command(entity_id.clone(), state, command, context).await
            }
        }?;

        let new_state = (self.reducer.compute_new_state)(maybe_state, event.clone()).ok_or("transition etat impossible".to_string())?;
        let version = maybe_entity
            .map(|x| x.version.unwrap_or(0));

        let new_entity = Entity {
            entity_id: entity_id.clone(),
            data: new_state,
            version
        };

        self.store.insert(new_entity).await?;
        let event_entity = EntityEvent {
            entity_id: entity_id.clone(),
            event_id: "todo genenerate".to_string(), // todo generate event id
            data: event
        };

        self.journal.insert(event_entity).await
    }
}
