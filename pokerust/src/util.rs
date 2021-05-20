/// Helper macros and other junk for internal use only

#[doc(hidden)]
#[macro_export]
macro_rules! set_endpoint {
    { $A:ty, $B:tt, $C:literal } => {
#[async_trait::async_trait]
impl crate::Endpoint for $A {
    type ResourceListKind = $B<$A>;

    const ENDPOINT: &'static str = $C;

    async fn list(client: &reqwest::Client, offset: usize, limit: usize) -> Result<Self::ResourceListKind, ::reqwest::Error> {
        crate::cache::get_resource(client, &format!("{}/?offset={}&limit={}", Self::ENDPOINT, offset, limit)).await?.json::<Self::ResourceListKind>().await
    }

    async fn full_list(client: &reqwest::Client) -> Result<Self::ResourceListKind, ::reqwest::Error> {
        crate::cache::get_resource(client, &format!("{}/?offset=0&limit=9999", Self::ENDPOINT)).await?.json::<Self::ResourceListKind>().await
    }
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
