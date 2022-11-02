use core::marker::Unsize;
use core::ops::{CoerceUnsized, Deref};
use std::any::Any;

use crate::cast::Instance;
use crate::Telecaster;

pub struct TypeToken<'a, T: ?Sized> {
    reference: &'a T,
}

impl<T: ?Sized> Deref for TypeToken<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.reference
    }
}

impl<T: ?Sized> AsRef<T> for TypeToken<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized + Any> From<&'a T> for TypeToken<'a, T> {
    fn from(r: &'a T) -> Self {
        let ptr = r as *const T as *const ();
        let id = r.type_id();
        Telecaster::instance(Instance::Remember(ptr, id));
        TypeToken { reference: r }
    }
}

impl<T: ?Sized> Drop for TypeToken<'_, T> {
    fn drop(&mut self) {
        let ptr = self.as_ref() as *const T as *const ();
        Telecaster::instance(Instance::Forget(ptr));
    }
}

pub struct BoxToken<T: ?Sized> {
    boxed: Box<T>,
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<BoxToken<U>> for BoxToken<T> {}

impl<T: ?Sized> Deref for BoxToken<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.boxed.as_ref()
    }
}

impl<T: ?Sized> AsRef<T> for BoxToken<T> {
    fn as_ref(&self) -> &T {
        self.boxed.as_ref()
    }
}

impl<T: ?Sized + Any> From<Box<T>> for BoxToken<T> {
    fn from(b: Box<T>) -> Self {
        let ptr = b.as_ref() as *const T as *const ();
        let id = b.as_ref().type_id();
        Telecaster::instance(Instance::Remember(ptr, id));
        BoxToken { boxed: b }
    }
}

impl<T: ?Sized> BoxToken<T> {
    pub fn unwrap(mut self) -> Box<T> {
        let ptr = self.boxed.as_mut() as *mut T;
        std::mem::forget(self);
        Telecaster::instance(Instance::Forget(ptr as *const ()));
        unsafe { Box::from_raw(ptr) }
    }
}

impl<T: ?Sized> Drop for BoxToken<T> {
    fn drop(&mut self) {
        let ptr = self.as_ref() as *const T as *const ();
        Telecaster::instance(Instance::Forget(ptr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{impl_type_id, telecaster};
    use anyhow::Result;

    trait Foo {
        fn foo(&self) -> u32 {
            0xF00
        }
    }
    trait Bar {}
    struct Simple(u32);
    impl Foo for Simple {}
    impl Bar for Simple {}
    impl_type_id!(Simple);
    telecaster!(Simple, dyn Foo);

    #[test]
    fn test_ref_rememberance() -> Result<()> {
        let s = Simple(5);
        let bar: &dyn Bar = &s;
        {
            // We remember that instance `s` is really of type `Simple`.
            let _tt = TypeToken::from(&s);
            // But later, we have only type-erased reference `bar` and we
            // want to get a pointer to `dyn Foo`.
            let f: &dyn Foo = Telecaster::cast(bar).unwrap();
            assert_eq!(f.foo(), 0xF00);
        }

        {
            // Now that the previous scope has ended, the TypeToken no longer
            // exists and the cast should fail.
            let f: Option<&dyn Foo> = Telecaster::cast(bar);
            assert!(f.is_none());
        }
        Ok(())
    }

    #[test]
    fn test_box_rememberance() -> Result<()> {
        let at = BoxToken::from(Box::new(Simple(10)));
        let bt: BoxToken<dyn Bar> = at;
        let f: &dyn Foo = Telecaster::cast(bt.as_ref()).unwrap();
        assert_eq!(f.foo(), 0xF00);

        // Consumes `bt` and returns the box, thus forgetting the token.
        let ct = bt.unwrap();
        let f: Option<&dyn Foo> = Telecaster::cast(ct.as_ref());
        assert!(f.is_none());
        Ok(())
    }
}
