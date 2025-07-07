use std::{collections::HashMap, rc::Rc};

use crate::{
    runtime::agent::JSAgent,
    runtime::realm::Realm,
    value::{object::JSObjAddr, string::JSString, JSValue},
};

pub(crate)type BehaviourFn = fn(&mut JSAgent, Vec<JSValue>) -> JSValue;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate)enum JSObjectSlotName {
    BehaviourFn,
    Extensible,
    InitialName,
    Prototype,
    Realm,
}

#[derive(Debug)]
pub(crate)enum JSObjectSlotValue {
    BehaviourFn(BehaviourFn),
    Realm(Rc<Realm>),
    Value(JSValue),
    NotSet,
}

impl From<JSValue> for JSObjectSlotValue {
    fn from(value: JSValue) -> Self {
        JSObjectSlotValue::Value(value)
    }
}

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/15.0/#sec-object-internal-methods-and-internal-slots
#[derive(Debug, Default)]
pub(crate)struct JSObjectInternalSlots(HashMap<JSObjectSlotName, JSObjectSlotValue>);

impl JSObjectInternalSlots {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, name: JSObjectSlotName, value: JSObjectSlotValue) {
        self.0.insert(name, value);
    }

    fn get(&self, name: &JSObjectSlotName) -> Option<&JSObjectSlotValue> {
        self.0.get(name)
    }

    pub(crate)fn prototype(&self) -> Option<JSObjAddr> {
        match self.get(&JSObjectSlotName::Prototype) {
            Some(JSObjectSlotValue::Value(JSValue::Object(addr))) => Some(*addr),
            _ => None,
        }
    }

    pub(crate)fn set_prototype(&mut self, prototype: Option<JSObjAddr>) {
        self.0.insert(
            JSObjectSlotName::Prototype,
            prototype.map_or(JSObjectSlotValue::NotSet, |p| JSValue::Object(p).into()),
        );
    }

    pub(crate)fn extensible(&self) -> bool {
        match self.get(&JSObjectSlotName::Extensible) {
            Some(JSObjectSlotValue::Value(JSValue::Bool(value))) => *value,
            _ => true,
        }
    }

    pub(crate)fn set_extensible(&mut self, extensible: bool) {
        self.0.insert(
            JSObjectSlotName::Extensible,
            JSValue::Bool(extensible).into(),
        );
    }

    pub(crate)fn realm(self) -> Option<Rc<Realm>> {
        match self.get(&JSObjectSlotName::Realm) {
            Some(JSObjectSlotValue::Realm(realm)) => Some(realm.clone()),
            _ => None,
        }
    }

    pub(crate)fn set_realm(&mut self, realm: Rc<Realm>) {
        self.0
            .insert(JSObjectSlotName::Realm, JSObjectSlotValue::Realm(realm));
    }

    pub(crate)fn initial_name(&self) -> Option<JSString> {
        match self.get(&JSObjectSlotName::InitialName) {
            Some(JSObjectSlotValue::Value(JSValue::String(name))) => Some(name.clone()),
            _ => None,
        }
    }

    pub(crate)fn set_initial_name(&mut self, name: JSString) {
        self.0
            .insert(JSObjectSlotName::InitialName, JSValue::String(name).into());
    }

    pub(crate)fn behaviour_fn(&self) -> Option<BehaviourFn> {
        match self.get(&JSObjectSlotName::BehaviourFn) {
            Some(JSObjectSlotValue::BehaviourFn(func)) => Some(*func),
            _ => None,
        }
    }

    pub(crate)fn set_behaviour_fn(&mut self, func: BehaviourFn) {
        self.0.insert(
            JSObjectSlotName::BehaviourFn,
            JSObjectSlotValue::BehaviourFn(func),
        );
    }
}

impl From<Vec<JSObjectSlotName>> for JSObjectInternalSlots {
    fn from(slots: Vec<JSObjectSlotName>) -> Self {
        let mut internal_slots = JSObjectInternalSlots::new();

        for slot in slots {
            internal_slots.insert(slot, JSObjectSlotValue::NotSet);
        }

        internal_slots
    }
}
