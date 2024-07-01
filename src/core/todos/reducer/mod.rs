use crate::core::shared::reducer::Reducer;
use crate::core::todos::data::{TodoEvents, TodoStates};
use crate::core::todos::data::TodoEvents::{Created, Updated};

pub struct TodoReducer {
    underlying: Reducer<TodoEvents, TodoStates>
}

impl TodoReducer {
    pub fn new() -> Self {

        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    if current.is_none() {
                        match event {
                            Created { by, at, name} => Some(TodoStates::Todo { name }),
                            _ => None
                        }
                    } else {
                        match event {
                            Updated (e) => Some(TodoStates::Todo {name: "test".to_string()}),
                            _ => None
                        }
                    }
                }
            }
        }
    }
}