#[cfg(test)]
use std::collections::BTreeMap;
#[cfg(not(test))]
use std::collections::HashMap;

use super::object::Object;

#[cfg(not(test))]
#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Object>,
    sub_environment: Option<Box<Environment>>,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Environment {
    variables: BTreeMap<String, Object>,
    sub_environment: Option<Box<Environment>>,
}

impl Environment {
    #[cfg(not(test))]
    pub fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
            sub_environment: None,
        }
    }

    #[cfg(test)]
    pub fn new() -> Environment {
        Environment {
            variables: BTreeMap::new(),
            sub_environment: None,
        }
    }

    #[cfg(not(test))]
    pub fn push(&mut self) {
        self.sub_environment = Some(Box::new(self.clone()));
        self.variables = HashMap::new();
    }

    #[cfg(test)]
    pub fn push(&mut self) {
        self.sub_environment = Some(Box::new(self.clone()));
        self.variables = BTreeMap::new();
    }

    pub fn pop(&mut self) {
        if let Some(sub_environment) = self.sub_environment.clone() {
            self.variables = sub_environment.variables;
            self.sub_environment = sub_environment.sub_environment;
        }
    }

    pub fn set(&mut self, identifier: String, value: Object) {
        self.variables.insert(identifier, value);
    }

    pub fn get<T: AsRef<str>>(&self, identifier: T) -> Object {
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
