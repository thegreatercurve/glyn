use crate::JSObjAddr;

/// 6.1.7.2 Object Internal Methods and Internal Slots
/// https://262.ecma-international.org/15.0/index.html#sec-object-internal-methods-and-internal-slots
#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct ObjectInternalSlots {
    /// [[Prototype]]
    pub(crate) prototype: Option<JSObjAddr>,

    /// [[Extensible]]
    pub(crate) extensible: Option<bool>,
}
