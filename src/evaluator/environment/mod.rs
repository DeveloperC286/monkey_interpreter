use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use crate::evaluator::Object;

/// A lexical scope, the variables bound within it and a handle on the scope which
/// lexically encloses it.
///
/// A `Scope` is a shared handle rather than a value, cloning one is a reference count
/// increment and never copies the variables bound within it. Sharing is what makes
/// closures possible, a function captures a handle on the scope it was defined within so
/// that scope outlives the call which created it, and what makes recursion possible, a
/// function bound after capturing its defining scope is still visible through the handle.
#[derive(Clone)]
pub struct Scope {
    scope: Rc<RefCell<ScopeData>>,
}

struct ScopeData {
    variables: BTreeMap<String, Object>,
    enclosing: Option<Scope>,
}

impl Scope {
    fn new(enclosing: Option<Scope>) -> Scope {
        Scope {
            scope: Rc::new(RefCell::new(ScopeData {
                variables: BTreeMap::new(),
                enclosing,
            })),
        }
    }

    fn set(&self, identifier: String, value: Object) {
        self.scope.borrow_mut().variables.insert(identifier, value);
    }

    fn get<T: AsRef<str>>(&self, identifier: T) -> Object {
        let scope = self.scope.borrow();

        match scope.variables.get(identifier.as_ref()) {
            Some(value) => value.clone(),
            None => match &scope.enclosing {
                Some(enclosing) => enclosing.get(identifier),
                None => Object::Null,
            },
        }
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scope = self.scope.borrow();

        f.debug_struct("Scope")
            .field("variables", &scope.variables)
            .field("enclosing", &scope.enclosing)
            .finish()
    }
}

impl PartialEq for Scope {
    /// Scopes are identities not values, two handles are equal when they are handles on the
    /// same scope. Comparing the variables bound within them would not terminate, a function
    /// bound within a scope holds a handle on that same scope.
    fn eq(&self, other: &Scope) -> bool {
        Rc::ptr_eq(&self.scope, &other.scope)
    }
}

/// The scope evaluation is currently occurring within.
#[derive(Debug)]
pub(super) struct Environment {
    current: Scope,
}

impl Environment {
    pub(super) fn new() -> Environment {
        Environment {
            current: Scope::new(None),
        }
    }

    /// A handle on the scope currently being evaluated within, to be captured by a function
    /// expression so the free variables of its body can later be resolved where the function
    /// was defined rather than where it is called.
    pub(super) fn current(&self) -> Scope {
        self.current.clone()
    }

    /// Enter a fresh scope enclosed by `enclosing`, returning the scope which was current so
    /// it can be restored by `pop`.
    pub(super) fn push(&mut self, enclosing: Scope) -> Scope {
        std::mem::replace(&mut self.current, Scope::new(Some(enclosing)))
    }

    /// Return to the scope `push` replaced.
    pub(super) fn pop(&mut self, previous: Scope) {
        self.current = previous;
    }

    pub(super) fn set(&mut self, identifier: String, value: Object) {
        self.current.set(identifier, value);
    }

    pub(super) fn get<T: AsRef<str>>(&self, identifier: T) -> Object {
        self.current.get(identifier)
    }
}
