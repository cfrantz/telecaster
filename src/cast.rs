use once_cell::sync::OnceCell;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::GetTypeId;

pub struct TraitObject {
    data: *const (),
    vtable: *const (),
}

impl TraitObject {
    pub fn new<T: ?Sized>(dy: &T) -> TraitObject {
        let dy = dy as *const T;
        let (data, vtable) = dy.to_raw_parts();
        unsafe {
            TraitObject {
                data,
                vtable: std::mem::transmute_copy(&vtable),
            }
        }
    }

    pub unsafe fn cast<T: ?Sized>(&self) -> &T {
        let ptr = core::ptr::from_raw_parts::<T>(self.data, std::mem::transmute_copy(&self.vtable));
        ptr.as_ref().unwrap()
    }
}

type IdFn = fn() -> (TypeId, TypeId);
type CastFn = unsafe fn(*const ()) -> TraitObject;

pub struct Telecaster {
    pub id_: IdFn,
    pub cast_: CastFn,
}
inventory::collect!(Telecaster);

static CAST_MAP: OnceCell<Mutex<HashMap<(TypeId, TypeId), CastFn>>> = OnceCell::new();
static INST_MAP: OnceCell<Mutex<HashMap<usize, TypeId>>> = OnceCell::new();

pub(crate) enum Instance {
    Remember(*const (), TypeId),
    Forget(*const ()),
    Query(*const ()),
}

impl Telecaster {
    fn lookup(type_: TypeId, trait_: TypeId) -> Option<CastFn> {
        let castmap = CAST_MAP
            .get_or_init(|| {
                let mut casts = HashMap::new();
                for ty in inventory::iter::<Telecaster> {
                    let id = (ty.id_)();
                    casts.insert(id, ty.cast_);
                }
                Mutex::new(casts)
            })
            .lock()
            .unwrap();
        castmap.get(&(type_, trait_)).cloned()
    }

    pub(crate) fn instance(op: Instance) -> Option<TypeId> {
        let mut instmap = INST_MAP.get_or_init(|| Default::default()).lock().unwrap();
        match op {
            Instance::Remember(ptr, tid) => {
                let ptr = ptr as usize;
                instmap.insert(ptr, tid);
                None
            }
            Instance::Forget(ptr) => {
                let ptr = ptr as usize;
                instmap.remove(&ptr);
                None
            }
            Instance::Query(ptr) => {
                let ptr = ptr as usize;
                instmap.get(&ptr).cloned()
            }
        }
    }

    pub fn cast<'a, T, U>(object: &'a T) -> Option<&'a U>
    where
        T: ?Sized,
        U: ?Sized + 'static,
    {
        let object = object as *const T as *const ();
        let type_ = T::type_id().or_else(|| Self::instance(Instance::Query(object)))?;
        let trait_ = TypeId::of::<U>();
        Self::lookup(type_, trait_).map(|cast| unsafe {
            let trait_object = cast(object);
            // Cast the trait_object back into a dyn reference.
            // Use transmute to re-attach lifetime 'a to the result.
            std::mem::transmute(trait_object.cast::<U>())
        })
    }
}

#[macro_export]
macro_rules! telecaster {
    ($ty:ty, $($tr:ty),* $(,)?) => {
        $(
            const _:() = {
                use std::any::TypeId;
                use $crate::TraitObject;

                // This is a function so we don't have to require downstream
                // users to use const_type_id.
                fn idfn() -> (TypeId, TypeId) {
                    (TypeId::of::<$ty>(), TypeId::of::<$tr>())
                }
                unsafe fn cast(ptr: *const ()) -> TraitObject {
                    let ptr = ptr as *const $ty;
                    let dy: &$tr = ptr.as_ref().unwrap();
                    TraitObject::new(dy)
                }
                inventory::submit! {
                    Telecaster {
                        id_: idfn,
                        cast_: cast,
                    }
                }
            };
        )*
    }
}
