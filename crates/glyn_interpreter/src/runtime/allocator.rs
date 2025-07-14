use safe_gc::{Gc, Heap, Root, Trace};

#[derive(Default)]
pub(crate) struct GlynAllocator {
    heap: Heap,
}

impl GlynAllocator {
    pub(crate) fn alloc<T: Trace>(&mut self, value: T) -> Root<T> {
        self.heap.alloc(value)
    }

    pub(crate) fn get<T: Trace>(&self, addr: Gc<T>) -> &T {
        self.heap.get(addr)
    }

    pub(crate) fn get_mut<T: Trace>(&mut self, addr: Gc<T>) -> &mut T {
        self.heap.get_mut(addr)
    }
}
