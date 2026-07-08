# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.4.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.5.0 (__/07/2026)

### Added

commit bbdee0733665cb3d36dbf606ef32a57ecae9f124

- Added the type_id, is, downcast_ref, downcast_mut, as_any_ref and as_any_mut functions to the “any” module.

commit bac10f71f9a6a6e119146db6d055a34cce647971

- Added a package authors field with "Paul Saunders" as an element.

- Added a “from” method to the Immut implementation.

-- Added a “from” method to the serde::Skip implementation.

Added in this version.

commit e993a9ce5e471ace8d35a74457ee946f7d835ed8

- Added the corlib_macros dependency.

- Added a macros feature.

commit d987e51dfc6aa6af63dc3888a31067b323457543

- Added the Cargo.lock file.

- Added a better_skip_test test function to the serde::skip_tests test module.

commit 499c2cb6c1a9924ef5a85709153abbe3c2775023

-- Added a GenericVisitor struct to the serde sub module.

Removed

-- Added a GenericVisitorExpected structto the serde sub module.

Removed

commit 59347d2790eeda5a4e06c50c99c824f002939b6f

- Added a Display implementation of the Immut struct.

- Added conditional serde Serialize and Deserialize implementations of the Immut struct.

- Added the immut_tests test module with serialize_deserialize_test and serialize_deserialize_test_2 test functions.

commit bb6555730085e3654b5e13881d6823a0d7621941

-- Added documentation

commit 52345a564ae53f5706d22b56559a8acf09a4f150

- Added serde_json as a dev-dependency.

-- Added documentation

- Added a skip_tests test module with a skip_test test function to the serde sub-module.

commit a823ec46f712ea4d9586553a38afc735d35f525e

-- Added RcRefCell<T> and WeakRefCell<T> type aliases to the cell module.

- Added RcRefCell and WeakRefCell type aliases to the cell module.

commit e7a758dea083950b98aad840f2bda2ba9a57dec7

- Added the AsMutStr trait to the text module.

commit 16e822708e98d47520cfcdd2c44caa923a6c37dc

- Added a From<ControlFlow> implementation for the bool type.

- Added the serde public module.

- Added the Skip struct to the serde module.

commit 68431609b8b811f542fc7af7dd07fd70edff56be

-- Added a ControlFlow struct.

- Added a ControlFlow enum.

commit d66a746fc283cc6f9d0e1024b7207bb608e37bd4

- Added may_retry_count and may_retry_count_return macros.

commit 90253766ff1b7af6e7d1791b5e7b60f752536dbc

- Added the may_retry and may_retry_return public macros as well as the once_twice and once_twice_return test functions for these macros.

commit 9635a41cfdc1199c65e87c9cf8ef43fb4e89ed63

-- Added highly_sendable, env_var_helpers, inc_dec, capped_collections and accessorise crates as optional dependencies and exposed them as extern crates in the lib file.

Removed

-- Added the ThisOrThat, ThisThatOther and DefaultOrValue enums.

Removed



### Changed

commit bbdee0733665cb3d36dbf606ef32a57ecae9f124

- Updated the unicode-ident dependency via the “cargo update” command.

-- Updated the package version to 0.5.0-beta.

- Updated the corlib_macros dependency to version 0.1.1.

- Updated the readme.

- Renamed the any::any module to any::any_traits.

- Updated documentation

commit bac10f71f9a6a6e119146db6d055a34cce647971

-- The corlib_macros dependency now points to a local repository.

- Cleaned up the Cargo.toml file.

-- Prepared the changelog.

-- Updated the readme.

- Renamed the “convert” module to “any”.

-- Updated documentation

- Moved the “upgrading” module under the “rc” module.

-> - Other minor changes.

Put Last

commit a434603cd4c4252d2e0a7f7b4c720e32c0408795

-- Updated the readme.

-- ControlFlow now conditionally derives the serde Serialize and Deserialize macros.

Added in this version.

-- Updated documentation

- Conditionally implemented the Default trait for the GapFillingCounter struct.

- Immut now conditionally implements the Copy, PartialEq, Eq, PartialOrd, Ord and Hash traits.

-- Skip now conditionally implements the Copy, PartialEq, Eq, PartialOrd, Ord and Hash traits.

Added in this version.

-- Other minor changes.

commit e993a9ce5e471ace8d35a74457ee946f7d835ed8

-- Conditionally exposed the corlib_macros::AsAnyRef and corlib_macros::AsAnyMut derive macros in the convert module.

- Conditionally exposed the corlib_macros::AsAnyRef and corlib_macros::AsAnyMut derive macros in the any module.

Renamed

- Conditionally exposed the corlib_macros::WeakSelf derive macro.

-- Conditionally exposed the corlib_macros::RcDefault and corlib_macros::ArcDefault derive macros in the convert module.

- Conditionally exposed the corlib_macros::RcDefault and corlib_macros::ArcDefault derive macros in the rc module.

Incorrect module name provided.

commit d987e51dfc6aa6af63dc3888a31067b323457543

-- Updated documentation

- Implemented the Clone trait for the Immut struct.

- Implemented the Display trait for the Immut struct.

-- Implemented the Clone trait for the serde::Skip struct.

Added in this version.

- Implemented the Display trait for the serde::Skip struct.

Added in this version.

-- Other minor changes.

commit 59347d2790eeda5a4e06c50c99c824f002939b6f

-- Disabled the serde cfg_attr on the Immut struct.

Added in this version.

-- Renamed the “item” field to “object” in the Immut struct definition.

- Renamed the “item” field to “object” in the Immut struct declaration.

commit a823ec46f712ea4d9586553a38afc735d35f525e

 -- Made the AsAnyMut trait, in the convert module, require the implementation of the AsAnyRef trait.

  - Made the AsAnyMut trait, in the any module, require the implementation of the AsAnyRef trait.

Renamed

- Made the Self type aliases of the RcDefault and ArcDefault traits, of the rc module, require that the Default trait be implemented for them.

commit e7a758dea083950b98aad840f2bda2ba9a57dec7

- Changed “doc_auto_cfg” to “doc_cfg” in the docsrs package level cfg_attr statement.

-- Other minor changes

commit 16e822708e98d47520cfcdd2c44caa923a6c37dc

- Updated the serde dependency to version 1.0.228.

- Updated the serde dependency to now use the “derive” feature.

-- Updated the cfg-if dependency to 1.0.4.

- Updated the cfg-if dependency to version 1.0.4.

-- The Immut struct now conditionally derives the serde Serialize and Deserialize traits.

Not anymore

commit 68431609b8b811f542fc7af7dd07fd70edff56be

-- Updated the package version string to "0.5.0-alpha".

commit 9635a41cfdc1199c65e87c9cf8ef43fb4e89ed63

-- Updated the documentation of the DropPanic struct.

-- Disabled the cfg_if and get_some use statements in the lib file.

-- Disabled the getters_setters_callers, inc_dec, work_in_progress_result (WorkInProgressResult and IdedWorkInProgressResult structs) and cargo_env modules in the lib file.

-- Disabled the drop_panic cfg_if block and replaced it with conditional mod and use statements.
    
-- Updated the AsStr trait documentation in the text module.

-- Disabled the sendable_text, sendable_text_log and sendable_text_log_with_buffer sub-modules in the text module.

Removed



### Deprecated

commit a823ec46f712ea4d9586553a38afc735d35f525e

-- Deprecated the impl_as_any_ref macro.

Removed

-- Deprecated the impl_as_any_mut macro.

Removed



### Removed

commit d987e51dfc6aa6af63dc3888a31067b323457543

-- Removed the derive attribute with the Serialize and Deserialize macro declarations from the Immut struct.

Added in this version.

-- Removed the serde::GenericVisitor struct.

Added in this version.

-- Removed the serde::GenericVisitorExpected struct.

Added in this version.

commit 59347d2790eeda5a4e06c50c99c824f002939b6f

-- Removed the highly_sendable, env_var_helpers, inc_dec, capped_collections and accessorise optional dependencies.

Added in this version.

commit bb6555730085e3654b5e13881d6823a0d7621941

- Updated the package edition to “2024”.

- Decorated the impl_weak_self_trait macro with the deprecated attribute.

REMOVE

-- Other minor changes.

commit 52345a564ae53f5706d22b56559a8acf09a4f150

-- Removed the impl_as_any_ref_method and impl_as_any_mut_method macros from the convert module.

- Removed the impl_as_any_ref_method and impl_as_any_mut_method macros from the any module.

Renamed

commit a823ec46f712ea4d9586553a38afc735d35f525e

- Removed the impl_as_any_ref macro.

Not Deprecated

- Removed the impl_as_any_mut macro.

Not Deprecated

commit e7a758dea083950b98aad840f2bda2ba9a57dec7

- Removed the cargo_env module and its contents.

-- Removed ThisOrThat, ThisThatOther and DefaultOrValue enums.

- Removed the ThisOrThat, ThisThatOther and DefaultOrValue enums.

- Removed the getters_setters_callers module and its contents.

- Removed the inc_dec module and its contents.

commit e7a758dea083950b98aad840f2bda2ba9a57dec7

- Removed the SendableText enum and the SendableTextLog and SendableTextLogWithBuffer structs from the text module.

- Removed the WorkInProgressResult and IdedWorkInProgressResult structs.





### Fixed

commit e993a9ce5e471ace8d35a74457ee946f7d835ed8

-- Fixed a couple of errors with the Immut struct implementation.

Relates to functionality added in this version.



### Security



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


