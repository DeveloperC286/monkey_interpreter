use std::collections::HashMap;

use crate::evaluator::model::object::Object;

#[derive(Clone)]
pub(crate) struct Environment {
    variables: HashMap<String, Object>,
    sub_environment: Option<Box<Environment>>,
}

impl Environment {
    pub(crate) fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
            sub_environment: None,
        }
    }

    pub(crate) fn push(&mut self) {
        self.sub_environment = Some(Box::new(self.clone()));
        self.variables = HashMap::new();
    }

    pub(crate) fn pop(&mut self) {
        if let Some(sub_environment) = self.sub_environment.clone() {
            self.variables = sub_environment.variables;
            self.sub_environment = sub_environment.sub_environment;
        }
    }

    pub(crate) fn set(&mut self, identifier: String, value: Object) {
        self.variables.insert(identifier, value);
    }

    pub(crate) fn get<T: AsRef<str>>(&self, identifier: T) -> Object {
        match self.variables.get(identifier.as_ref()) {
            Some(value) => value.clone(),
            None => {
                if let Some(sub_environment) = &self.sub_environment {
                    return sub_environment.get(identifier);
                }

                Object::Null
            }
        }
    }
}
