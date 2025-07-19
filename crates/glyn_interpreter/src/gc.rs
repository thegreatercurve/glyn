use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    runtime::{environment::Environment, realm::Realm},
    value::object::{JSObjAddr, JSObject},
};

type ID = u32;

pub(crate) enum Item {
    Object(Box<JSObject>),
    Environment(Box<Environment>),
    Realm(Box<Realm>),
}

impl Item {
    fn as_object(&self) -> Option<&JSObject> {
        match self {
            Item::Object(o) => Some(o),
            _ => None,
        }
    }

    fn as_object_mut(&mut self) -> Option<&mut JSObject> {
        match self {
            Item::Object(o) => Some(o),
            _ => None,
        }
    }

    fn as_environment(&self) -> Option<&Environment> {
        match self {
            Item::Environment(e) => Some(e),
            _ => None,
        }
    }

    fn as_environment_mut(&mut self) -> Option<&mut Environment> {
        match self {
            Item::Environment(e) => Some(e),
            _ => None,
        }
    }

    fn as_realm(&self) -> Option<&Realm> {
        match self {
            Item::Realm(r) => Some(r),
            _ => None,
        }
    }

    fn as_realm_mut(&mut self) -> Option<&mut Realm> {
        match self {
            Item::Realm(r) => Some(r),
            _ => None,
        }
    }
}

impl Trace for Item {
    fn trace(&self, tracer: &mut Tracer) {
        match self {
            Item::Object(o) => o.trace(tracer),
            Item::Environment(e) => e.trace(tracer),
            Item::Realm(r) => r.trace(tracer),
        }
    }
}

impl From<JSObject> for Item {
    fn from(o: JSObject) -> Self {
        Item::Object(Box::new(o))
    }
}

impl From<Environment> for Item {
    fn from(e: Environment) -> Self {
        Item::Environment(Box::new(e))
    }
}

impl From<Realm> for Item {
    fn from(r: Realm) -> Self {
        Item::Realm(Box::new(r))
    }
}

#[derive(Debug)]
pub(crate) struct Gc<T: Trace> {
    id: ID,
    _phantom: PhantomData<T>,
}

impl<T: Trace> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Trace> Copy for Gc<T> {}

impl<T: Trace> PartialEq for Gc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.id != 0
    }
}

impl<T: Trace> Gc<T> {
    fn new(id: ID) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
}

pub(crate) struct RootSet<T: Trace>(Vec<Gc<T>>);

#[derive(Default)]
pub(crate) struct Tracer {
    marked: HashSet<ID>,
}

impl Tracer {
    pub fn edge<T: Trace>(&mut self, to: Gc<T>) {
        if !self.is_marked(to.id) {
            self.marked.insert(to.id);
        }
    }

    fn is_marked(&self, id: ID) -> bool {
        self.marked.contains(&id)
    }
}

pub(crate) trait Trace {
    fn trace(&self, tracer: &mut Tracer);
}

#[derive(Default)]
pub(crate) struct Heap {
    items: HashMap<ID, Box<Item>>,
    next_id: ID,
}

impl Heap {
    pub(crate) fn alloc<T: Trace + Into<Item>>(&mut self, data: T) -> Gc<T> {
        let id = self.next_id;

        self.next_id += 1;

        self.items.insert(id, Box::new(data.into()));

        Gc::new(id)
    }

    pub(crate) fn collect<T: Trace>(&mut self, roots: &RootSet<T>) {
        let tracer = self.mark(roots);

        self.sweep(tracer);
    }

    fn mark<T: Trace>(&mut self, roots: &RootSet<T>) -> Tracer {
        let mut tracer = Tracer::default();

        for root in &roots.0 {
            self.get_mut(*root).trace(&mut tracer);
        }

        tracer
    }

    fn sweep(&mut self, tracer: Tracer) {
        self.items.retain(|id, _| !tracer.is_marked(*id));
    }

    fn get<T: Trace>(&self, ptr: Gc<T>) -> &Item {
        self.items.get(&ptr.id).unwrap()
    }

    fn get_mut<T: Trace>(&mut self, ptr: Gc<T>) -> &mut Item {
        self.items.get_mut(&ptr.id).unwrap()
    }

    pub(crate) fn obj(&self, ptr: &JSObjAddr) -> &JSObject {
        self.get(*ptr).as_object().unwrap()
    }

    pub(crate) fn obj_mut(&mut self, ptr: &JSObjAddr) -> &mut JSObject {
        self.get_mut(*ptr).as_object_mut().unwrap()
    }

    pub(crate) fn env(&self, ptr: Gc<Environment>) -> &Environment {
        self.get(ptr).as_environment().unwrap()
    }

    pub(crate) fn env_mut(&mut self, ptr: Gc<Environment>) -> &mut Environment {
        self.get_mut(ptr).as_environment_mut().unwrap()
    }

    pub(crate) fn realm(&self, ptr: Gc<Realm>) -> &Realm {
        self.get(ptr).as_realm().unwrap()
    }

    pub(crate) fn realm_mut(&mut self, ptr: Gc<Realm>) -> &mut Realm {
        self.get_mut(ptr).as_realm_mut().unwrap()
    }
}
