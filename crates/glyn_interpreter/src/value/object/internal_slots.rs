use std::collections::HashMap;

use crate::value::{object::JSObjAddr, JSValue};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum JSObjectSlotName {
    /// [[Prototype]]
    Prototype,

    /// [[Extensible]]
    Extensible,
}

#[derive(Debug)]
pub enum JSObjectSlotValue {
    /// The value of the slot, if it exists.
    Value(JSValue),

    /// The slot is not set.
    NotSet,
}

impl From<JSValue> for JSObjectSlotValue {
    fn from(value: JSValue) -> Self {
        JSObjectSlotValue::Value(value)
    }
}

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/15.0/index.html#sec-object-internal-methods-and-internal-slots
#[derive(Debug, Default)]
pub(crate) struct JSObjectInternalSlots(HashMap<JSObjectSlotName, JSObjectSlotValue>);

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

    pub(crate) fn prototype(&self) -> Option<JSObjAddr> {
        match self.get(&JSObjectSlotName::Prototype) {
            Some(JSObjectSlotValue::Value(JSValue::Object(addr))) => Some(*addr),
            _ => None,
        }
    }

    pub(crate) fn set_prototype(&mut self, prototype: Option<JSObjAddr>) {
        self.0.insert(
            JSObjectSlotName::Prototype,
            prototype.map_or(JSObjectSlotValue::NotSet, |p| JSValue::Object(p).into()),
        );
    }

    pub(crate) fn extensible(&self) -> bool {
        match self.get(&JSObjectSlotName::Extensible) {
            Some(JSObjectSlotValue::Value(JSValue::Boolean(value))) => *value,
            _ => false,
        }
    }

    pub(crate) fn set_extensible(&mut self, extensible: bool) {
        self.0.insert(
            JSObjectSlotName::Extensible,
            JSValue::Boolean(extensible).into(),
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
