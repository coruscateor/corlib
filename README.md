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

Forces external immutability.

### WorkInProgressResult And IdedWorkInProgressResult

Containers used for communicating the results and statuses of ongoing operations.

### GapFillingCounter

A counter that can recycle "gaps".

### Getter Setter Caller Macros

Macros for implementing getters, setters and calling methods on regular structs and RefCells.

### Get Some

Convert Options into certainty.

## Sub-Modules:

### Cell

Macros, functions and the RefCellStore which help make working with RefCells easier.

### Collections

Collections and collection related objects.

### Convert

Convert stuff into &dyn Any and &mut dyn Any.

### Drop Panic

A fun way to crash your programme.

### Events

Events and event related objects.

Loosely inspred by .NET events.

### Has One

Get the value of one for each numeric type.

### Inc Dec

Miss the ++ and -\- operators?

This module can help sort you out.

### Rc

Reference counting related helper structs and traits.

### Text

Look at SendableText and AsStr.

### Upgrading

Functions that help you deal with upgrading reference counted objects.

### Value

For when you want to move values around.

## Compiler:

Build with the latest stable compiler.

## Todo:

- Add more documentation
- Add code examples
- Add more tests
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

