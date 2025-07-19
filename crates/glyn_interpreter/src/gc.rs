use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug)]
pub(crate) struct Gc<T>(Rc<RefCell<T>>);

impl<T> Gc<T> {
    pub(crate) fn new(value: T) -> Self {
        Gc(Rc::new(RefCell::new(value)))
    }
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Gc(self.0.clone())
    }
}

impl<T> PartialEq for Gc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl<T> Deref for Gc<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
