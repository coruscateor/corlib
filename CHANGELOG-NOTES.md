aead00030c7c5da58c5d8c28ee87a34c2409ea72 -

- Added ArcByPtr and SyncWeakByPtr structs.

- Added BoxOrRc, BoxOrRcWeak, BoxOrWeak, BoxOrRcs, BoxOrRcsWeaks, Weakness, BoxOrRcCow, BoxOrRcsCow and ArcCow enums.

- Renamed the “text_enums” module to “text” and made it public.

- For RcByPtr and WeakByPtr the “cmp” and “partial_cmp” implementations now cast all Arc pointers to unit to avoid ambiguous wide pointer comparison issues.

- In the rfc module “borrow” and “borrow_mut” no longer take “param” parameters. All functions that do now have “_param” in their names. “borrow_only” and “borrow_mut_only” have been removed.

- “up_rc_pt”, “up_arc_pt” and “up_arc_pt_async” have been added to the “upgrading” module.

b905019d0070c43b09298b0136927aed9b0942ff -

- ArcByPtr, BoxOrRc, BoxOrRcWeak, BoxOrWeak, BoxOrRcs, BoxOrRcsWeaks, Weakness, BoxOrRcCow, BoxOrRcsCow, ArcCow, SyncWeakByPtr now conditionally implement std::fmt::Debug (new).

- The Dictionary, KeyValuePair, List, Queue and UniqueItemList collection objects now conditionally implement std::fmt::Debug.

- The GapFillingCounter, NonOption, RcByPtr and WeakByPtr objects now conditionally implement std::fmt::Debug.

- All HasOne object implementations now implement std::fmt::Debug.

- MovableText now implements Debug, Clone, PartialEq and Eq.

ea91b552a77057a69fab4cf26b3bb593cff43140 -

Deprecated the trait_get, trait_get_2, trait_get_set, trait_get_2_set, impl_get, impl_get_2, impl_get_set and impl_get_2_set macros.

5f8fed123566f479ddf9fa3e20030d8992ab3c7e -

Added an ArcStr variant as well as is_arc_str and extract_string methods to MovableText and renamed it to SendableText.

5072548997e9aa68458cb8e903cc7e6f517365a7 -

Implemented Display, Deref (Target = str), From<String>, From<&String>, From<&'static str>, From<Arc<str>> and From<&Arc<str>> for SendableText and removed its ToString trait implementation.

ab39324a51e4b260822a36f429a77a259982dee2 -

Added SendableTextLog and SendableTextLogWithBuffer to the text module.

87f7671c14775468a996e9c7845ad63e786532f2 -

- Removed the box_or_ref module.
- Added push_only, clear and overwite_buffer methods to SendableTextLogWithBuffer.

9983f2c276ba18a3120e4d32acaaa91ffe715ab9 -

When the append_to method of SendableTextLog is called its queued items are now reverse iterated.

fd7996f05a8d23e91418b40e4e9915270268eaea -

- Updated the module level documentation of the inc_dec module. - ?

- Added pp_mut, checked_pp_mut, ppf_mut, mm_mut, mmf_mut, checked_mm_mut macros to the inc_dec module.

- Added IncrementsSelf and DecrementsSelf traits to the inc_dec module.

- Added IncrementsSelf and DecrementsSelf trait implementations for the f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, and usize primitive objects.

e49b47a59e667caaface3aff1902894fcc9ff31f -

Added a drop_panic feature which enables the DropPanic struct.

717e6cb143d87a42fe44c9936c35301f59d92c23 -

Made the inc_dec module public.

115774fb40a2f1916aef3280929f91250b53e569 -

- Combined the IncrementsSelf and DecrementsSelf traits into a single IncDecSelf trait and made all relevant primitive types implement this new trait.

- Added the IntIncDecSelf trait and implementation blocks for integer primitives.

- Added the overflowing_pp_mut and overflowing_mm_mut macros.

- Fixed issues with integer incrementation and decrementation.

2f16f6f83c472a48617af9b5f0a0a7643613658e

- Renamed overflowing_pp to opp in the IntIncDecSelf trait and implementers in the inc_dec module.

- Added wpp and wmm method definitions to the IntIncDecSelf trait and implemented these methods in the IntIncDecSelf implementers in the inc_dec module.

- Renamed the overflowing_pp_mut macro to opp_mut and the overflowing_mm_mut macro to omm_mut in the inc_dec module.

6b4270337d3244dedd067b332bd54b08f4d125eb -

Started work on StackedVec.

0b489b166e47ba4656df17fa36aba784ce11ac55 -

Added a lot of great stuff to StackedVec

3520a2bbd27a31843f7bf4c6463f46feb2afcc06 -

StackedVec is now in a usable state.

102a1d6416366e37071636715d486047c72587df -

StackedVec now implements Copy if its item type does.

329096852f7a31875518f5e9009186863b5ed686 -

- Made the type of the elements of the private array of StackedVec be T instead of Option<T> and adjusted the implementation accordingly.

- Added NonDefaultStackedVec

e5d7eef850f0d332ed1a58f91284160c44b5beac -

- Made the text module a directory oriented module with each object that was contained in the text module file getting its own module file in the new directory.

- Added serde as an optional dependency.

- SendableText now implements the Serialize trait when serde is specified as a dependency.

- Updated the delegate dependency to 0.13.1.

e5cd63b6060b06771727d23e30d40c039534fffc -

- Implemented serde::Serialize on StackedVec.

- Added Immut

- Removed Lazy

91ee7cb791ef79fa626764235424fb7ee17380b8 -

Implemented serde::Deserialize on SendableText.

f70b716f73ab0cba5e76dac660af483955289f83 -

Some code re-arrangement.

575044b9a69fbe1fadba0f51f11c28412c34e270

- Added ResultFrame and IdedResultFrame.
- All of the methods in the upgrading module that return bool have had “try_” prepended to their names. up_rc, up_rc_pt, up_arc and try_up_arc_pt have been updated to panic on error and not return bools.

- Other minor changes.

02073e70e343318f821f2bf5e61ee4abf7cf1073

- Replaced ResultFrame and IdedResultFrame with WorkInProgressResult and IdedWorkInProgressResult.

- Implemented Default on SendableText.

7a293a2e21644ad4db716d4d32655af07686e34b

- Removed impl_get_weak_self_ref

- Added MutState

fd4d632aced5115e9bee6498ec0255ca5b7128ff

- Added the Cell module and moved the rfc module into it and publicly exposed its contents.

- Renamed MutState to RefCellStore and moved it into the Cell module.

- Added the Convert module and moved the as_any module into it.

- Renamed AsAny to AsAnyRef and impl_as_any to impl_as_any_ref and removed its second rule.

- Added impl_as_any_ref_method, AsAnyMut, impl_as_any_mut and impl_as_any_mut_method to the Convert module.

8c957d7048d584056678a9bfbfc379f0b9a54099

Added constrained Display and Debug implementations to WorkInProgressResult and IdedWorkInProgressResult.

984888bcecc571bac73490c5ba93f62c544e0dae

- Added from_refcell to RefCellStore and renamed ref_cell_ref to refcell_ref.

- Added SingleSubArgsEvent, SubUnSubArgs, SingleSubEvent and SubUnSub to the events module.

- Other minor changes.

fcea22831f05649d26bc7cb684c8d199718ac773

- Made SingleArgsEvent and SingleSubArgsEvent a bit more user friendly.

- Renamed SubUnSub to PubSingleSubEvent and SubUnSubArgs to PubSingleSubArgsEvent.

- Added macros impl_pub_single_sub_event_method and impl_pub_single_sub_args_event_method.

361cf255d3e4b316f68282d39ca65eac78678cc8

Added get, set, clone_set, try_get, try_set, try_clone_set, take and take_refcell to RefCellStore.

b73bb126137d4059d1bca55d4f5a677c7da508d4

Tried to implement AsCopy.

f627955651a8366b059b26320e106a65776c481c

- Added TakeItCell to the Cell sub-module.

07fe211e5151a6d5edccb19978a9148ff5db3b24

- Added the value module and its contents.

77b43753eb9b287a479d2bf654b560a3fa70f22f

- Updated all the traits in the value module to use associated types instead of generics

a0fffb563ff6cbd5ad3af2de1c6b2b693ecbb32a

- Removed AsCopy.

- collections::StackedVec and NonDefaultStackedVec.

- Removed the dropper module and its contents.

- Removed events::SkipOnce.

- Removed some other old code.

cb6b33ab14b150e00a68b79174df765bc1c5e93a

- Removed cell::TakeItCell

e7c8dcca2f67ab758f35867517491a0e26e9ba0f

- Added RcRefCellStore and WeakRefCellStore to the cell module.

- Renamed clone_set to set_clone and try_clone_set to try_set_clone in RefCellStore.

c9ae90ba4b527902709fc9225edb0e54149c7233

- Conditionally implemented std::fmt::Debug on RefCellStore, SingleSubEvent and related structs, SingleSubArgsEvent and related structs and Immut.

5d4215aba8127f9ef2139f31ce49ff2ecf341c56

- Added the cargo_env public module.

- Added the impl_pub_env_accessor macro to cargo_env.

- Added the cargo function to cargo_env (via impl_pub_env_accessor).

87a4fb39ceee8299888146dd3a05a5d060564f86

- In ArcByPtr, RcByPtr, SyncWeakByPtr and WeakByPtr their contents methods now return clones of their respective contents fields. contents_ref methods have been added to each struct which returns a reference to the contents object of each struct. The relevant parts of the project have been updated accordingly.

- impl_pub_env_accessor now documents the function it generates.

- Added cargo_manifest_dir, cargo_manifest_path, cargo_pkg_version, cargo_pkg_version_major, cargo_pkg_version_patch, cargo_pkg_version_pre, cargo_pkg_authors, cargo_pkg_name, cargo_pkg_description, cargo_pkg_homepage, cargo_pkg_repository, cargo_pkg_license, cargo_pkg_license_file, cargo_pkg_rust_version and cargo_pkg_readme functions to the cargo_env module.

c1f68f4ad3c1a0572063321a3a9ccc08240666cf

- Documented cargo_env at the module level.

- Renamed impl_pub_env_accessor to impl_cargo_env_accessor.

- Added impl_cargo_env_accessor_pair to cargo_env.

- Added cargo_pair, cargo_manifest_dir_pair, cargo_manifest_path_pair, cargo_pkg_version_pair, cargo_pkg_version_major_pair, cargo_pkg_version_minor_pair, cargo_pkg_version_patch_pair, cargo_pkg_version_pre_pair, cargo_pkg_authors_pair, cargo_pkg_name_pair, cargo_pkg_description_pair, cargo_pkg_homepage_pair and cargo_pkg_repository_pair, cargo_pkg_license_pair, cargo_pkg_license_file_pair, cargo_pkg_rust_version_pair and cargo_pkg_readme_pair to the cargo_env module via impl_cargo_env_accessor_pair.

- Made the users.rust-lang.org URL a hyper-link in the DynHash trait documentation.

8e88673b336424b0fa5cf84c32a5f1e376eacb7e

- Added the weak_self public module, the WeakSelf trait and the impl_weak_self_trait macro both defined within this module.

faa135eec2ae6c0ac3a59faa5532c985865c2a01

- Updated documentation

- Renamed the DynPartialEqOrEq struct to DynPartialEq and updated the relevant parts of the project to reflect this.

- Renamed the object method of DynHashAdapter to object_ref and updated the relevant parts of the project to reflect this.

- Renamed the DynPartialEqOrEqAdapter struct to DynPartialEqAdapter and updated the relevant parts of the project to reflect this.

- Renamed the object method of DynPartialEqAdapter to object_ref and updated the relevant parts of the project to reflect this.

- Removed HashsetItem and HasStrId from the collections module.

0f010768d5e006c54528a00f42ce81131adc982a

- Updated the version string to 0.4.0-beta.

- Started updating the readme.

- Added more documentation to the library contents.

- Made inc_dec into a directory oriented module.

- Renamed the buffer method to buffer_ref in SendableTextLogWithBuffer.

74578ff4d8c44eb6620665f3ca4e419d9e5a8ca9

- Updated the readme.

- Added the rc directory module and moved the ArcByPtr, RcByPtr, SyncWeakByPtr, WeakByPtr structs and the RcDefault and ArcDefault traits into it.

- rc_default was made a private module after being moved.

- Updated the documentation of various objects.