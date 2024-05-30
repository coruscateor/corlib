<div align="center">

# Corlib

[![Crates.io](https://img.shields.io/crates/v/corlib)](https://crates.io/crates/corlib)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/corlib)](https://crates.io/crates/corlib)
[![Docs](https://docs.rs/corlib/badge.svg)](https://docs.rs/corlib/latest/corlib/)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

Corlib is a various ideas library.  

</div>

## Library Contents:    

### NonOption

Like an option but not optional.    

### RcByPtr and WeakByPtr
  
Compare and hash Rc and Rc::Weak objects by pointer instead of by value.  

### RcDefault and ArcDefault

Default traits for returning reference counted types.

### MovableText

For conveniently moving owned and static strings and string slices around.

### AsStr

A trait with an as_str method.

### Droppers

Structs for calling closures on drop.

### GapFillingCounter

A counter that can recycle "gaps".

### Getters Setters Callers

Macros for implementing getters, setters and calling methods on regular structs and RefCells.

### HasOne

Sometimes you just want the value 1 (or 1.0).

### Lazy

A struct for lazily initialising objects.

Does not contain its own initialising closure.

### As Any

A trait with a method (as_any) for returning a &dyn Any instance of &self (comes with a handy implementation macro).

### Get Some

Convert Options into certainty.

### Inc Dec Macros

Miss the ++ and -- operators?

This module can help sort you out.

### Collections

Collections and collection related objects.

### Events

Events and event related objects. Inspred by .NET events.

### Rfc

Macros and functions that help you work with RefCells.

### Upgrading

Functions that help you deal with upgrading reference counted objects.

## Todo:

- Add more documentation
- Add code examples
- Add more tests
- Add Hashmap and Hashset based event implementations
- Clean-up the code
- Solidify the API for 1.0

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

