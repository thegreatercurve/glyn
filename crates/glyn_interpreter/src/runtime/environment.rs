use safe_gc::{Collector, Gc, Trace};

pub(crate) type EnvironmentAddr = Gc<Environment>;

#[derive(Clone, Debug)]
pub(crate) struct Environment;

impl Trace for Environment {
    fn trace(&self, _collector: &mut Collector) {}
}
