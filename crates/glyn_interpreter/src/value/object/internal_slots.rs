use std::collections::HashMap;

use crate::{
    runtime::{environment::EnvironmentAddr, realm::RealmAddr},
    value::{string::JSString, JSValue},
};

pub(crate) type BehaviourFn = fn(Vec<JSValue>) -> JSValue;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum InternalSlotName {
    BehaviourFn,
    InitialName,
    Realm,
    Environment,
}

#[derive(Debug)]
pub(crate) enum InternalSlotValue {
    BehaviourFn(BehaviourFn),
    Realm(RealmAddr),
    Environment(EnvironmentAddr),
    Value(JSValue),
    NotSet,
}

impl From<JSValue> for InternalSlotValue {
    fn from(value: JSValue) -> Self {
        InternalSlotValue::Value(value)
    }
}

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/16.0/#sec-object-internal-methods-and-internal-slots
#[derive(Debug, Default)]
pub(crate) struct InternalSlots(HashMap<InternalSlotName, InternalSlotValue>);

impl InternalSlots {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, name: InternalSlotName, value: InternalSlotValue) {
        self.0.insert(name, value);
    }

    fn get(&self, name: &InternalSlotName) -> Option<&InternalSlotValue> {
        self.0.get(name)
    }

    pub(crate) fn realm(&self) -> Option<&RealmAddr> {
        match self.get(&InternalSlotName::Realm) {
            Some(InternalSlotValue::Realm(realm_addr)) => Some(&realm_addr),
            _ => None,
        }
    }

    pub(crate) fn set_realm(&mut self, realm_addr: RealmAddr) {
        self.0.insert(
            InternalSlotName::Realm,
            InternalSlotValue::Realm(realm_addr),
        );
    }

    pub(crate) fn initial_name(&self) -> Option<JSString> {
        match self.get(&InternalSlotName::InitialName) {
            Some(InternalSlotValue::Value(JSValue::String(name))) => Some(name.clone()),
            _ => None,
        }
    }

    pub(crate) fn set_initial_name(&mut self, name: JSString) {
        self.0
            .insert(InternalSlotName::InitialName, JSValue::String(name).into());
    }

    pub(crate) fn behaviour_fn(&self) -> Option<BehaviourFn> {
        match self.get(&InternalSlotName::BehaviourFn) {
            Some(InternalSlotValue::BehaviourFn(func)) => Some(*func),
            _ => None,
        }
    }

    pub(crate) fn set_behaviour_fn(&mut self, func: BehaviourFn) {
        self.0.insert(
            InternalSlotName::BehaviourFn,
            InternalSlotValue::BehaviourFn(func),
        );
    }

    pub(crate) fn environment(&self) -> Option<EnvironmentAddr> {
        match self.get(&InternalSlotName::Environment) {
            Some(InternalSlotValue::Environment(env_addr)) => Some(env_addr.clone()),
            _ => None,
        }
    }

    pub(crate) fn set_environment(&mut self, env_addr: EnvironmentAddr) {
        self.0.insert(
            InternalSlotName::Environment,
            InternalSlotValue::Environment(env_addr),
        );
    }
}

impl From<Vec<InternalSlotName>> for InternalSlots {
    fn from(slots: Vec<InternalSlotName>) -> Self {
        let mut internal_slots = InternalSlots::new();

        for slot in slots {
            internal_slots.insert(slot, InternalSlotValue::NotSet);
        }

        internal_slots
    }
}
