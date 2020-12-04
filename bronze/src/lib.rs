use std::rc::Rc;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;


pub trait GcTrace {

}

/*
* I want the following kinds of references to Gc objects:
* 1. References IN the Gc heap (GcRef)
* 2. References in the roots collection (GcRoot)
* 3. References in the Rust heap (GcHandle)
*/

// References WITHIN the Gc heap.
// The compiler doesn't need to know how big a T is because it's just going to be a reference.
// pub struct GcRef<T: ?Sized> {
//     // No reference counting here; we'll trace instead.
//     obj_ref: *const GcUntypedRoot,
//     phantom: PhantomData<T>,
// }


// Based on rust-gc by Manishearth.
// These go on the Gc heap and hold the actual Gc data.
pub struct GcBox<T: GcTrace + ?Sized> { // TODO: make T 'static?
    data: T,
}

impl<T: GcTrace> GcBox<T> {
    pub fn new(data: T) -> NonNull<Self> {
        let bx = Box::new(GcBox {data});
        let bx_ptr = Box::into_raw(bx);

        unsafe {NonNull::new_unchecked(bx_ptr)}
    }

    pub fn new_ref(&mut self) -> GcRef<T> {
        let ptr: *mut Self = self;

        GcRef {obj_ref: unsafe {NonNull::new_unchecked(ptr)}}
    }

    pub fn ref_from_ptr(ptr: NonNull<Self>) -> GcRef<T> {
        GcRef {obj_ref: ptr}
    }
}

// GcRef represents a reference WITHIN the GC heap.
pub struct GcRef<T: ?Sized + GcTrace> {
    obj_ref: NonNull<GcBox<T>>,
}

impl<T: GcTrace + ?Sized> GcTrace for GcRef<T> {
    // TODO
}


impl<T: GcTrace + ?Sized + std::fmt::Display> std::fmt::Display for GcRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {(self.obj_ref).as_ref().data.fmt(f)}
    }
}

struct GcUntypedRoot {
    obj: NonNull<GcBox<dyn GcTrace>>,
}

// struct GcHandle<T> {
//     obj_ref: Rc<GcUntypedRoot>,
//     phantom: PhantomData<T>,
// }

// // Master list of all roots, for use in doing tracing.
// struct GcRoots {
//     roots_vec: Vec<dyn GcTrace>,
// }


// This is for prototyping only.
pub struct Gc<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<T: GcTrace> Gc<T> {
    pub fn new(b: T) -> GcRef<T> {
        let gc_box = GcBox::new(b);
        GcBox::ref_from_ptr(gc_box)
    }
}

// TODO
impl<T: GcTrace + ?Sized> GcTrace for Box<T> {}


#[cfg(test)]
mod tests {
    #[test]
    fn new_handle() {
        assert_eq!(2 + 2, 4);
    }
}
