# Corlib

Corlib is a general purpose library.  

## Contents at a glance:    


### NonOption

Like an option but not optional.    
  
### RcByPtr
  
Compare and hash an Rc by pointer instead of value.  
  
### RcDefault and ArcDefault

Default trats for returning reference counted types.


### Text Enums

All String types unified into various enum types.


### WeakByPtr

RcByPtr, but weak.


### Droppers

Structs for caling closures on drop.


### GapFillingCounter

A counter that can recycle "gaps".


### Get Rc Or Weak Self

Traits and macros to support the Weak-Self pattern.


### Get Ref Mut

Traits for getting references.


### Getters Setters Callers

Macroes for implementing getters, setters and calling stuff.


### HasOne

Sometimes you just want the value 1 (or 1.0).


### Invalid

A trait for implenting a method that returns an invalid instance of Self.


### Lazy

A struct for lazily initalising objects. Dos not contain its own initalising closure.


### Collections

Collection and collection related objects.


### Events

Events and events related objects, inspred by .net events.  
  



## The Weak-Self Pattern

When a reference counted object holds a weak reference to itself, which can be accessed publicly, this can be called the Weak-Self pattern.  

This is useful when your types need to be used with Rc\<Any\> and you want to get a reference counted instance once the exact type of the Any instance has been established. 

If an object stores a weak (or strong) refernce to its reference counted instance, this makes it a reference type.

A referece type is a type that can only be refenced and not directly accessed or manipulated.


## Todo:

- Add more documentation
- Add code examples
- Add some tests
- Add Hashmap and Hashset based event implementations
- Code cleanup
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

