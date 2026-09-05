use std::collections::BTreeMap;

use crate::evaluator::Object;

#[derive(Debug, Clone)]
pub(super) struct Environment {
    variables: BTreeMap<String, Object>,
    sub_environment: Option<Box<Environment>>,
}

impl Environment {
    pub(super) fn new() -> Environment {
        Environment {
            variables: BTreeMap::new(),
            sub_environment: None,
        }
    }

    pub(super) fn push(&mut self) {
        self.sub_environment = Some(Box::new(self.clone()));
        self.variables = BTreeMap::new();
    }

    pub(super) fn pop(&mut self) {
        if let Some(sub_environment) = self.sub_environment.clone() {
            self.variables = sub_environment.variables;
            self.sub_environment = sub_environment.sub_environment;
        }
    }

    pub(super) fn set(&mut self, identifier: String, value: Object) {
        self.variables.insert(identifier, value);
    }

    pub(super) fn get<T: AsRef<str>>(&self, identifier: T) -> Object {
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
