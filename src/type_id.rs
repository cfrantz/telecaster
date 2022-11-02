use std::any::TypeId;

pub trait GetTypeId {
    fn type_id() -> Option<TypeId>;
}

pub trait GetSelfId {
    fn self_id(&self) -> Option<TypeId>;
}

impl<T: ?Sized> GetTypeId for T {
    default fn type_id() -> Option<TypeId> {
        None
    }
}

impl<T: GetTypeId> GetSelfId for T {
    fn self_id(&self) -> Option<TypeId> {
        Self::type_id()
    }
}

#[macro_export]
macro_rules! impl_type_id {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::GetTypeId for $ty {
                fn type_id() -> Option<std::any::TypeId> {
                    Some(std::any::TypeId::of::<$ty>())
                }
            }
        )*
    }
}

impl_type_id!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_type_id!(f32, f64);
impl_type_id!(str, &str, String);

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_primitive_types() -> Result<()> {
        assert_eq!(u8::type_id(), Some(TypeId::of::<u8>()));
        assert_eq!(u16::type_id(), Some(TypeId::of::<u16>()));
        assert_eq!(u32::type_id(), Some(TypeId::of::<u32>()));
        assert_eq!(u64::type_id(), Some(TypeId::of::<u64>()));
        assert_eq!(i8::type_id(), Some(TypeId::of::<i8>()));
        assert_eq!(i16::type_id(), Some(TypeId::of::<i16>()));
        assert_eq!(i32::type_id(), Some(TypeId::of::<i32>()));
        assert_eq!(i64::type_id(), Some(TypeId::of::<i64>()));
        assert_eq!(str::type_id(), Some(TypeId::of::<str>()));
        Ok(())
    }

    #[test]
    fn test_primitive_instances() -> Result<()> {
        assert_eq!(5u8.self_id(), Some(TypeId::of::<u8>()));
        assert_eq!(5u16.self_id(), Some(TypeId::of::<u16>()));
        assert_eq!(5u32.self_id(), Some(TypeId::of::<u32>()));
        assert_eq!(5u64.self_id(), Some(TypeId::of::<u64>()));
        assert_eq!(5i8.self_id(), Some(TypeId::of::<i8>()));
        assert_eq!(5i16.self_id(), Some(TypeId::of::<i16>()));
        assert_eq!(5i32.self_id(), Some(TypeId::of::<i32>()));
        assert_eq!(5i64.self_id(), Some(TypeId::of::<i64>()));
        Ok(())
    }

    #[test]
    fn test_primitive_refs() -> Result<()> {
        let five = &5u8;
        let foo = "foo";
        let bar = String::from("bar");
        let bref = &bar;
        assert_eq!(five.self_id(), Some(TypeId::of::<u8>()));
        assert_eq!(foo.self_id(), Some(TypeId::of::<&str>()));
        assert_eq!(bref.self_id(), Some(TypeId::of::<String>()));
        Ok(())
    }
}
