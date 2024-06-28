use crate::api::shared::daos::dbos::EventDBO;
use crate::api::todos::todo_dbo::TodoDboEvent;
use crate::core::shared::data::EntityEvent;
use crate::core::todos::data::TodoEvents;

impl From<TodoDboEvent> for TodoEvents {
    fn from(value: TodoDboEvent) -> Self {
        match value {
            TodoDboEvent::TodoCreatedDbo { by, at } => TodoEvents::Created { by, at }
        }
    }
}

impl From<EventDBO<TodoDboEvent, String>> for EntityEvent<TodoEvents, String> {
    fn from(value: EventDBO<TodoDboEvent, String>) -> Self {
        EntityEvent {
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone()
        }
    }
}


impl From<EntityEvent<TodoEvents, String>> for EventDBO<TodoDboEvent, String> {
    fn from(value: EntityEvent<TodoEvents, String>) -> Self {
        EventDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
            event_id: value.event_id.clone()
        }
    }
}

impl From<TodoEvents> for TodoDboEvent {
    fn from(value: TodoEvents) -> Self {
        match value {
            TodoEvents::Created { by, at } => TodoDboEvent::TodoCreatedDbo { by, at }
        }
    }
}

