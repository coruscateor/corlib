# Changelog

## Version 0.3.0 (_/05/2024)

6ff2d7bbf4aa1e5d78048a34cb6224268a688e83

- Added some module level documentation to as_any.
- Removed the Invalid trait.

- Added documentation to NonOption, added an “expect” method, removed the take and try_take methods, implemented Default with the default value being an invalid NonOption and cleaned it up a bit.

- Added an rfc module containing macros like rfc_borrow and rfc_borrow_mut which are intended to make it a bit easier to work with RefCells.

- Removed the sync module as well as the Notifier object contained within it (moved to LibSync).



f69c890dd415fc2455c516addc27f03b15ca1dfa



- Prepared List for a re-write.
- Removed the get_ref_mut module.

- Fixed impl_rfc_borrow_mut_call

- Moved the methods that were implementing traits from the get_ref_mut module into the impl<T: Default> Lazy<T> block.

- Cleaned up lib.rs

- Added documentation to the the macros in the rfc module.



7fefea78af264c3baf7493251e48bddbdd890d7a



- Updated Dictionary documentation
- Updated List documentation and reimplemented a bunch of methods using the delegate macro.

- Updated Queue documentation

- Removed a bunch of old code.

- Added and reimplemented a bunch of macros in getters_setters_callers.



c6b0eba1a2c740d376860e74ccde403be26ecb95



- Added functions that handle RefCell borrowing to the rfc module.
- Added an “upgrading” module with functions that deal with upgrading reference counted objects.



c0470282bff813ebf97e251b703a6e5c7d59160f


- Updated the rfc borrow and borrow_mut functions so that the “param” parameter now gets passed to the provided func by reference.

- Added async variants of the up_arc functions in the “upgrading” module as well as more up_rc and up_arc function variants that deal with Optional return values.



8276848a969eef60241c6b53143a0b179d38a813



Added documentation to the rfc borrow and borrow_mut functions as well as the “upgrading” up_rc and up_arc functions.





## Version 0.2.0 (18/04/2024)

- Added a sync module.
- Added Notifier and associated NotifierWaitResult and NotifierWaitTimeoutResult objects which are intended to make it easier to work with the standard Condvar object.
- Stopped the .vscode directory from being tracked in the repository.
- Added the get_some module and macro.
- Added the inc_dec module which provides macros for convenient incrementation and decrementation of numeric values.
- Changed the name of the get_contents_ref methods on both RcByPtr and WeakByPtr to contents.
- Fixed RcByPtr module visibility.
- Added dyn_hashing_and_cmp traits under collections.
- Manually implemented Clone on RcByPtr and WeakByPtr.
- In RcByPtr and WeakByPtr their “new” methods now take references rather than values.
- “take” methods have been added to RcByPtr and WeakByPtr which drops self and returns the contained value.
- Added the as_any trait and its associated impl_as_any macro.
- Cleaned up RcByPtr and WeakByPtr, changed the naming of their upgrade and downgrade methods and implemented PartialOrd and Ord on both structs.
- Added a MovableText enum for when you want to move Strings or a static String slices around.
- Cleaned up text_enums: removed ScopedText, ScopedStaticText and some old comments.
- Added “#![doc = include_str!("../README.md")]” to the top of the lib.rs file.
- Removed a lot of comments from getters_setters_and_callers.rs
- deprecated all of get_ref_mut.rs and get_rc_or_weak_self.rs.
- Updated the documentation for GapFillingCounter.
- Updated the project description in the Cargo.toml.
- Updated the README.md.
- Updated and added documentation for: AsAny, impl_as_any, DynHash, DynPartialEqOrEq, DynHashAdapter, DynPartialEqOrEqAdapter, KeyValuePair, Queue, ListArgsEvent, ListEvent, SkipOnce and Notifier.
- Deprecated HashsetItem struct and Invalid trait.
- Added module level documentation to collections.
- Updated the module level documentation for events.

## Version 0.1.0 (20/07/2023)

- Inital release


