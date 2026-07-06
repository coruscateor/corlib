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

### Immut

Guarantees external immutability for its contained object.

### GapFillingCounter

Increments a number returning "gap" mumbers, numbers that are reported to be no longer "be in use", first.

### Getter Setter Caller Macros

Removed, look at [Accessorise](https://crates.io/crates/accessorise).

### ControlFlow

Like the standard ControlFlow enum, without the type requirements.

### GetSome

Convert Options into certainty with the get_some declarative macro.

### WeakSelf

Tait and derive macro used to implement the weak-self design pattern.

<br/>

## Sub-Modules:

### Any

Cast stuff into &dyn Any and &mut dyn Any.

### Cell

The items in this module help you work with RefCells.

### Collections

Collection related stucts and traits, some of which are inspired by .NET collections.

### DropPanic

Panicking made convenient.

### Events

Event structs loosely inspired by .NET events.

### HasOne

A Trait and trait implementations for helping you get the value of one for each standard numeric type.

### IncDec

Moved to: [IncDec](https://crates.io/crates/inc_dec)

### Rc

Reference counting related helpers.

### Text

Look at AsStr and AsMutStr, SendableText and related structs moved to [Highly Sendable](https://crates.io/crates/highly_sendable).

### Upgrading

Now in the Rc module.

### Value

For when you want to do stuff with values and generic type constraints.

<br/>

## Compiler:

Build with the latest stable compiler.

<br />

## Features

| Feature     | Description                              |
| ----------- | -----------------------------------------|
| serde       | Enable serde related features.           |
| drop_panic  | Enable the drop_panic module.            |
| macros      | Enable the declarative macros.           |

<br />

## Todo:

- Add more documentation
- Add code examples
- Add more tests
- Clean-up the code

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust

fn bar()
{
}

fn foo()
{

    bar();

}

```

Not this:

```rust

fn bar()
{
}

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

