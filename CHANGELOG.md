# Changelog

## Version 0.1.0 (20/07/2023)

- Inital release

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




