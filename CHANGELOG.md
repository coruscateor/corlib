# Changelog

## Version 0.1.0 (20/07/2023)

- Inital release

## Version 0.2.0 (18/04/2024)

- Added a sync module
(remove) Added a Notifier object and associated objects as part of the sync module.

- Added Notifier and associated NotifierWaitResult and NotifierWaitTimeoutResult objects which are intended to make it easier to work with the standard Condvar object.

- Stopped the .vscode directory from being tracked in the repository.

(removed) Added a refcell_borrowing module which contains functions and macros that should make it a bit easier to work with Refcells. WIP

- Added the get_some module and macro.
- Added the inc_dec module which provides macros for convenient incrementation and decrementation of numeric values.

(removed) Implemented more rfc_ref and rfc_mut macro rules in refcell_borrowing and added owned_rfc_ref and owned_rfc_mut macros that are variations rfc_ref and rfc_mut for RefCells that are owned. Also the impl_owned_rfc_ref_method and impl_self_rfc_mut_method macros were added for method calls. I could not get impl_owned_rfc_ref_method to work in testing (probably due to hygiene rules). All of these macros will likely be removed.

(removed) Adjusted the naming of the call_* series of functions in refcell_borrowing. In each function name where applicable “rv” is now “_rv”.

(removed) Added a note to refcell_borrowing about moving rfc related macros from getters_setters_callers into that module.

(removed) Fixed the naming of some functions in  refcell_borrowing.

- Changed the name of the get_contents_ref methods on both RcByPtr and WeakByPtr to contents.
- Fixed RcByPtr module visibility.
- Added dyn_hashing_and_cmp traits under collections.
- Manually implemented Clone on RcByPtr and WeakByPtr.
- In RcByPtr and WeakByPtr their “new” methods now take references rather than values.
- “take” methods have been added to RcByPtr and WeakByPtr which drops self and returns the contained value.
- Added the as_any trait and its associated impl_as_any macro.

(renamed) Renamed dyn_traits to dyn_hashing_and_cmp and exposed it as a module.
(remove) (fixed) Tried to fix impl_rfc_borrow_call and impl_rfc_borrow_mut_call.

- Cleaned up RcByPtr and WeakByPtr, changed the naming of their upgrade and downgrade methods and implemented PartialOrd and Ord on both structs.

(remove) Re-enabled impl_as_any

- Added a MovableText enum for when you want to move Strings or a static String slices around.
- Cleaned up text_enums: removed ScopedText, ScopedStaticText and some old comments.

(remove) Fixed a spelling error in the notifier documentation.

- Added “#![doc = include_str!("../README.md")]” to the top of the lib.rs file.

(remove) Added documentation to inc_dec.rs.

(remove) Added documentation to has_one.rs.

(remove) deprecated a bunch of macros, fixed impl_rfc_borrow_call and removed a lot of comments from getters_setters_and_callers.rs.

- Removed a lot of comments from getters_setters_and_callers.rs
- deprecated all of get_ref_mut.rs and get_rc_or_weak_self.rs.

(remove) Amended documentation in gap_filling_counter.rs

- Updated the documentation for GapFillingCounter.
- Updated the project description in the Cargo.toml.
- Updated the README.md.
- Updated and added documentation for: AsAny, impl_as_any, DynHash, DynPartialEqOrEq, DynHashAdapter, DynPartialEqOrEqAdapter, KeyValuePair, Queue, ListArgsEvent, ListEvent, SkipOnce and Notifier.
- Deprecated HashsetItem struct and Invalid trait.
- Added module level documentation to collections.
- Updated the module level documentation for events.




