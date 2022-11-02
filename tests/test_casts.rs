#![feature(min_specialization)]
#![feature(ptr_metadata)]
use telecaster::{impl_type_id, telecaster, Telecaster};
use telecaster::{GetSelfId, GetTypeId};

trait Foo {
    fn foo(&self) -> u32 {
        0xF00
    }
}

trait Boo {
    fn boo(&self) -> u32 {
        0xB00
    }
}

struct Simple(u32);
impl Foo for Simple {}
impl Boo for Simple {}

impl_type_id!(Simple);
telecaster!(Simple, dyn Foo, dyn Boo);

fn call_boo<T: ?Sized>(t: &T) -> Option<u32> {
    println!("hello {:x?}", T::type_id());
    let b = Telecaster::cast::<T, dyn Boo>(t);
    b.map(|x| x.boo())
}

#[test]
fn test_boo() {
    let s = Simple(5);
    println!("S type_id = {:x?}", Simple::type_id());
    println!("s self_id = {:x?}", s.self_id());
    let result = call_boo(&s);
    assert_eq!(result, Some(0xB00));
}
