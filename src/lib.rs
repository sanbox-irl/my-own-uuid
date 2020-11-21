#![deny(missing_docs, missing_crate_level_docs)]

//! This crate exports a single macro to create your own newtyped Uuids more conveniently.
//! It is not the most fully featured macro, and it comes baked in with a lot of assumptions,
//! such as Serde and using Uuid v4. These assumptions derive from my own workflow,
//! where I find this macro very convenient.
//!
//! ## When would you want to use a macro such as this?
//!
//! Whenver you are dealing with multiple types of Ids on a single object, having a variety
//! of Uuids can effectively make you feel like you're working in a dynamic language. This
//! can happen especially in game development, or any development which has lots of asset management.
//! Imagine you have Entities which use Uuids as some unique id (perhaps of their save files),
//! and you have Assets, which are also using Uuids. You also have Prefabs, which are a kind of entity,
//! which you address like as Asset, which also have their own Uuid. As you might imagine,
//! this is a lot of Uuids to keep around and make sure to not mess up.
//!
//! This newtype wrapper greatly simplifies, but only by adding the small compiler error to warn
//! you against accidentally confusing your Uuids. Simply access the newtype directly with `.0`,
//! and you are back in full control over the raw Uuid data. Having our newtypes be an opaque
//! wrapper is a non-goal of this project
/// Make your own Uuid.
///
/// We might use this to differentiate between two different identifiers:
/// ```
/// # #[macro_use] extern crate mouuid;
/// # fn main() {
/// // we have an entity id...
/// my_own_uuid!(EntityId);
///
/// // and we have an asset id...
/// my_own_uuid!(AssetId);
///
/// // an example struct...
/// struct Entity {
///     id: EntityId,
///     sprite: AssetId,
/// }
///
/// let my_new_entity = Entity {
///     id: EntityId::new(),
///     sprite: AssetId::new()
/// };
///
/// // you'd get this from some sort of database in real code.
/// let my_other_asset = AssetId::new();
///
/// // this line won't compile because of the newtype wrapper!
/// // if my_new_entity.id == my_other_asset {
///
/// // but this line WILL compile!
/// if my_new_entity.sprite == my_other_asset {
///     // explode, we had a Uuid collision, contact the UN, the impossible happened.
///     // but actually you probably got this from a database, rather than
///     // constructed them inline.
/// }
/// # }
/// ```
/// Important note here: you can document your own Uuid like so:
///
/// ```
/// # #[macro_use] extern crate mouuid;
/// # fn main() {
/// my_own_uuid!(
///     /// An EntityId is a very good id. It's honestly
///     /// my *favorite* id. Don't tell the other ids.
///     EntityId
/// );
/// # }
/// ```
/// The above will show up in docs and in most IDEs when you hover over an EntityId!
#[macro_export]
macro_rules! my_own_uuid {
    ($(#[$meta:meta])* $this_val:ident) => {
        #[derive(
            PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Copy, Clone, Default, Debug,
        )]
        $(#[$meta])*
        pub struct $this_val(pub uuid::Uuid);

        impl $this_val {
            /// Creates a new Id using `Uuid::new_v4` which is randomly generated.
            #[inline]
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            /// Creates a new Id with the provided Uuid.
            #[inline]
            #[allow(dead_code)]
            pub fn with_id(id: uuid::Uuid) -> Self {
                Self(id)
            }

            /// Creates a new Id with the provided String, attempting to parse it into a Uuid.
            #[inline]
            #[allow(dead_code)]
            pub fn with_string(input: &str) -> Result<Self, uuid::Error> {
                Ok(Self(uuid::Uuid::parse_str(input)?))
            }
        }

        impl std::fmt::Display for $this_val {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($this_val), self.0)
            }
        }
    };
}
