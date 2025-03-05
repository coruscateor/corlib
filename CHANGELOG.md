# Changelog

## Version 0.4.1 (05/03/2025)

Added

- Added the necessary details to the Cargo.toml and the lib.rs files for docs.rs to build all the crates features and label their constituents.



Changed

- Updated the readme with a brief description of the Drop Panic module.



## Version 0.4.0 (28/02/2025)

Added

- Added pp_mut, checked_pp_mut, ppf_mut, mm_mut, mmf_mut, checked_mm_mut macros to the inc_dec module.

- Added the IncDecSelf trait to the inc_dec module.

- Added IncDecSelf trait implementations for the f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, and usize primitive objects.

- Added an ArcStr variant as well as is_arc_str and extract_string methods to MovableText and renamed it to SendableText.

- Added SendableTextLog and SendableTextLogWithBuffer to the text module.

- Added pp_mut, checked_pp_mut, ppf_mut, mm_mut, mmf_mut, checked_mm_mut macros to the inc_dec module.

- Added a drop_panic feature which enables the newly added DropPanic struct.

- Added the opp_mut and omm_mut macros.

- Added serde as an optional dependency.

- Added the Immut struct.

- Added the WorkInProgressResult and IdedWorkInProgressResult structs.

- Added the Cell module and moved the rfc module into it and publicly exposed its contents.

- Added the RefCellStore struct to the Cell module.

- Added the Convert module and moved the as_any module into it.

- Added impl_as_any_ref_method, AsAnyMut, impl_as_any_mut and impl_as_any_mut_method to the Convert module.

- Added SingleSubArgsEvent, PubSingleSubArgsEvent, SingleSubEvent and PubSingleSubEvent to the events module.

- Added macros impl_pub_single_sub_event_method and impl_pub_single_sub_args_event_method.

- Added the value module and its contents.

- Added the cargo_env public module.

- Added the impl_cargo_env_accessor macro to cargo_env.

- Added cargo, cargo_manifest_dir, cargo_manifest_path, cargo_pkg_version, cargo_pkg_version_major, cargo_pkg_version_patch, cargo_pkg_version_pre, cargo_pkg_authors, cargo_pkg_name, cargo_pkg_description, cargo_pkg_homepage, cargo_pkg_repository, cargo_pkg_license, cargo_pkg_license_file, cargo_pkg_rust_version and cargo_pkg_readme functions to the cargo_env module (via impl_cargo_env_accessor).

- Added RcRefCellStore and WeakRefCellStore to the cell module.

- Added impl_cargo_env_accessor_pair to cargo_env.

- Added cargo_pair, cargo_manifest_dir_pair, cargo_manifest_path_pair, cargo_pkg_version_pair, cargo_pkg_version_major_pair, cargo_pkg_version_minor_pair, cargo_pkg_version_patch_pair, cargo_pkg_version_pre_pair, cargo_pkg_authors_pair, cargo_pkg_name_pair, cargo_pkg_description_pair, cargo_pkg_homepage_pair and cargo_pkg_repository_pair, cargo_pkg_license_pair, cargo_pkg_license_file_pair, cargo_pkg_rust_version_pair and cargo_pkg_readme_pair to the cargo_env module via impl_cargo_env_accessor_pair.

- Added the weak_self public module containing the WeakSelf trait and the impl_weak_self_trait macro.

- Added the rc directory module and moved the RcByPtr and WeakByPtr structs and the RcDefault and ArcDefault traits into it.

- Added ArcByPtr and SyncWeakByPtr structs to the rc module.



Changed

- Renamed the “text_enums” module to “text” and made it public.

- For RcByPtr and WeakByPtr the “cmp” and “partial_cmp” implementations now cast all Rc pointers to unit to avoid ambiguous wide pointer comparison issues.

- “up_rc_pt” and “up_arc_pt” have been added to the “upgrading” module.

- The Dictionary, KeyValuePair, List, Queue and UniqueItemList collection objects now conditionally implement std::fmt::Debug.

- The GapFillingCounter, NonOption, RcByPtr and WeakByPtr objects now conditionally implement std::fmt::Debug.

- All HasOne object implementations now implement std::fmt::Debug.

- MovableText now implements Debug, Clone, PartialEq and Eq.

- Deprecated the trait_get, trait_get_2, trait_get_set, trait_get_2_set, impl_get, impl_get_2, impl_get_set and impl_get_2_set macros.

- Implemented Display, Deref (Target = str), From<String>, From<&String>, From<&'static str>, From<Arc<str>> and From<&Arc<str>> for SendableText and removed its ToString trait implementation.

- Made the inc_dec module public.

- Renamed overflowing_pp to opp in the IntIncDecSelf trait and implementers in the inc_dec module.

- Made the text module a directory oriented module with each object that was contained in the text module file getting its own module file in the new directory.

- SendableText now implements the Serialize trait when serde is specified as a dependency.

- Updated the delegate dependency to 0.13.1.

- Implemented serde::Deserialize on SendableText.

- All of the methods in the upgrading module that return bool have had “try_” prepended to their names. up_rc, up_rc_pt, up_arc and try_up_arc_pt have been updated to panic on error and not return bools.

- Implemented Default on SendableText.

- Renamed AsAny to AsAnyRef and impl_as_any to impl_as_any_ref and removed its second rule.

- In RcByPtr and WeakByPtr their contents methods now return clones of their respective contents fields. contents_ref methods have been added to each struct which returns a reference to the contents object of each struct. The relevant parts of the project have been updated accordingly.

- Made the users.rust-lang.org URL a hyper-link in the DynHash trait documentation.

- Updated documentation

- Renamed the DynPartialEqOrEq struct to DynPartialEq and updated the relevant parts of the project to reflect this.

- Renamed the object method of DynHashAdapter to object_ref and updated the relevant parts of the project to reflect this.

- Renamed the DynPartialEqOrEqAdapter struct to DynPartialEqAdapter and updated the relevant parts of the project to reflect this.

- Renamed the object method of DynPartialEqAdapter to object_ref and updated the relevant parts of the project to reflect this.

- Made inc_dec into a directory oriented module.

- Updated the readme.

- rc_default was made a private module after being moved into the rc directory module.

- Now using Keep a Changelog Version 1.1.0 (https://keepachangelog.com/en/1.1.0/) in producing the changelog documentation.



Fixed

- Fixed issues with integer incrementation and decrementation.



Removed

- Removed the Lazy struct.

- Removed the dropper module and its contents.

- Removed events::SkipOnce.

- Removed old code.

- Removed HashsetItem and HasStrId from the collections module.

- Removed the impl_get_weak_self_ref macro.



## Version 0.3.0 (30/05/2024)

- Added some module level documentation to as_any.
- Removed the Invalid trait.
- Added documentation to NonOption, added an “expect” method, removed the take and try_take methods, implemented Default with the default value being an invalid NonOption and cleaned it up a bit.
- Added a module called rfc which contains macros like rfc_borrow and rfc_borrow_mut which are intended to make it a bit easier to work with RefCells.
- Removed the sync module as well as the Notifier object contained within it (moved to LibSync).
- Removed the get_ref_mut module.
- Fixed the impl_rfc_borrow_mut_call macro.
- Moved the methods that were implementing traits from the get_ref_mut module into the impl<T: Default> Lazy<T> block.
- Updated Dictionary documentation
- Updated the List documentation and reimplemented a bunch of methods using the delegate macro.
- Updated Queue documentation
- Removed a bunch of old code files.
- Added and reimplemented a bunch of macros in getters_setters_callers.
- Added an “upgrading” module with functions that deal with upgrading reference counted objects.
- Updated the readme.
- Updated the module level documentation of the getters_setters_callers module.
- Updated dropper objects documentation.
- Removed the get_rc_or_weak_self module and its associated traits and macros.
- Cleaned up the lib file and made the getters_setters_callers and inc_dec modules private.
- Updated the NonOption struct level documentation.
- Updated the RcByPtr struct level documentation.
- Added documentation to the rc_default module.
- Updated the WeakByPtr struct level documentation.
- Removed the paste dependency.

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


