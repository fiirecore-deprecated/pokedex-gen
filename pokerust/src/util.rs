/// Helper macros and other junk for internal use only

#[doc(hidden)]
#[macro_export]
macro_rules! set_endpoint {
    { $A:ty, $B:tt, $C:literal } => {
impl crate::Endpoint for $A {
    type ResourceListKind = $B<$A>;

    const ENDPOINT: &'static str = $C;
}
};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_id {
    { $A:ty } => {
impl crate::Id for $A {
    fn id(&self) -> i16 {
        self.id
    }
}
};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_named {
    { $A:ty } => {
impl crate::Named for $A {
    fn name(&self) -> &String {
        &self.name
    }
}
};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_id_and_named {
    { $A:ty } => {
crate::impl_named!{$A}
crate::impl_id!{$A}
};
}
