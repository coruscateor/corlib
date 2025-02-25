# Changelog

## Version 0.4.0 (__/02/2025)

Added

- Added ArcByPtr and SyncWeakByPtr structs to rc.

- Added pp_mut, checked_pp_mut, ppf_mut, mm_mut, mmf_mut, checked_mm_mut macros to the inc_dec module.

- Added the IncDecSelf trait to the inc_dec module.

- Added IncDecSelf trait implementations for the f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, and usize primitive objects.

- Added an ArcStr variant as well as is_arc_str and extract_string methods to MovableText and renamed it to SendableText.

- Added SendableTextLog and SendableTextLogWithBuffer to the text module.

- Added pp_mut, checked_pp_mut, ppf_mut, mm_mut, mmf_mut, checked_mm_mut macros to the inc_dec module.

-- Added IncrementsSelf and DecrementsSelf traits to the inc_dec module.

-- Added IncrementsSelf and DecrementsSelf trait implementations for the f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, and usize primitive objects.

- Added a drop_panic feature which enables the newly added DropPanic struct.

- Added the opp_mut and omm_mut macros.

- Added the IntIncDecSelf trait and implementation blocks for integer primitives.

- Added wpp and wmm method definitions to the IntIncDecSelf trait and implemented these methods in the IntIncDecSelf implementers in the inc_dec module.

- Added serde as an optional dependency.

- Added Immut



Changed

- Renamed the “text_enums” module to “text” and made it public.

- For RcByPtr and WeakByPtr the “cmp” and “partial_cmp” implementations now cast all Rc pointers to unit to avoid ambiguous wide pointer comparison issues.

- “up_rc_pt” and “up_arc_pt” have been added to the “upgrading” module.

- The Dictionary, KeyValuePair, List, Queue and UniqueItemList collection objects now conditionally implement std::fmt::Debug.

- The GapFillingCounter, NonOption, RcByPtr and WeakByPtr objects now conditionally implement std::fmt::Debug.

- All HasOne object implementations now implement std::fmt::Debug.

- MovableText now implements Debug, Clone, PartialEq and Eq.

- Deprecated the trait_get, trait_get_2, trait_get_set, trait_get_2_set, i…
…mpl_get, impl_get_2, impl_get_set and impl_get_2_set macros.

- Implemented Display, Deref (Target = str), From<String>, From<&String>, From<&'static str>, From<Arc<str>> and From<&Arc<str>> for SendableText and removed its ToString trait implementation.

- Made the inc_dec module public.

- Renamed overflowing_pp to opp in the IntIncDecSelf trait and implementers in the inc_dec module.

- Made the text module a directory oriented module with each object that was contained in the text module file getting its own module file in the new directory.

- SendableText now implements the Serialize trait when serde is specified as a dependency.

- Updated the delegate dependency to 0.13.1.

- Implemented serde::Deserialize on SendableText.



Fixed

- Fixed issues with integer incrementation and decrementation.



Removed

- Removed the Lazy struct.


---

Added Then Removed

- Added BoxOrRc, BoxOrRcWeak, BoxOrWeak, BoxOrRcs, BoxOrRcsWeaks, Weakness, BoxOrRcCow, BoxOrRcsCow and ArcCow enums.

- Removed the box_or_ref module.

- Started work on StackedVec.

- Added a lot of great stuff to StackedVec

- StackedVec is now in a usable state.

- Added NonDefaultStackedVec


Changed Then Removed

- In the rfc module “borrow” and “borrow_mut” no longer take “param” parameters. All functions that do now have “_param” in their names. “borrow_only” and “borrow_mut_only” have been removed.

- “up_arc_pt_async” has been added to the “upgrading” module.

- BoxOrRc, BoxOrRcWeak, BoxOrWeak, BoxOrRcs, BoxOrRcsWeaks, Weakness, BoxOrRcCow, BoxOrRcsCow, ArcCow now conditionally implement std::fmt::Debug (new).

- Implemented serde::Serialize on StackedVec.


Added To/Changed On Something Added

- Added push_only, clear and overwite_buffer methods to SendableTextLogWithBuffer.

- When the append_to method of SendableTextLog is called its queued items are now reverse iterated.

- Renamed the overflowing_pp_mut macro to opp_mut and the overflowing_mm_mut macro to omm_mut in the inc_dec module.

- StackedVec now implements Copy if its item type does.

- Made the type of the elements of the private array of StackedVec be T instead of Option<T> and adjusted the implementation accordingly.


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


