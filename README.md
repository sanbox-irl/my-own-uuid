# my-own-uuid

[![](https://img.shields.io/crates/v/my_own_uuid.svg)](https://crates.io/crates/my_own_uuid)

This crate exports a single macro to create your own newtyped Uuids more conveniently.
It is not the most fully featured macro, and it comes baked in with a lot of assumptions,
such as Serde and using Uuid v4. These assumptions derive from my own workflow,
where I find this macro very convenient.

## Installation

In your Cargo.toml:

```toml
[dependencies]
my-own-uuid = 0.1.0
```

## When would you want to use a macro such as this

Whenver you are dealing with multiple types of Ids on a single object, having a variety
of Uuids can effectively make you feel like you're working in a dynamic language. This
can happen especially in game development, or any development which has lots of asset management.
Imagine you have Entities which use Uuids as some unique id (perhaps of their save files),
and you have Assets, which are also using Uuids. You also have Prefabs, which are a kind of entity,
which you address like as Asset, which also have their own Uuid. As you might imagine,
this is a lot of Uuids to keep around and make sure to not mess up.

This newtype wrapper greatly simplifies, but only by adding the small compiler error to warn
you against accidentally confusing your Uuids. Simply access the newtype directly with `.0`,
and you are back in full control over the raw Uuid data. Having our newtypes be an opaque
wrapper is a non-goal of this project.

## License

This crate is dual licensed under the Apache 2.0 and MIT licenses. Users may choose either license.