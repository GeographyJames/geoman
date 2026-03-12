macro_rules! typed_id {
    ($name:ident) => {
        #[derive(
            Debug,
            serde::Serialize,
            serde::Deserialize,
            sqlx::Type,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Clone,
            Default,
            Copy,
            Hash,
        )]
        #[sqlx(transparent)]
        pub struct $name(pub i32);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<i32> for $name {
            fn as_ref(&self) -> &i32 {
                &self.0
            }
        }

        impl From<i32> for $name {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
    };
}

typed_id!(FigureId);
typed_id!(FigureLayerId);
typed_id!(BaseMapId);

typed_id!(DataProviderId);
typed_id!(LayerStyleId);
