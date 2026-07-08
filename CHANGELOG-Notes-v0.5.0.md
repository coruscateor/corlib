commit bbdee0733665cb3d36dbf606ef32a57ecae9f124 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jul 6 18:32:44 2026 +1200

    - Updated the unicode-ident dependency via the “cargo update” command.
    
    - Updated the package version to 0.5.0-beta.
    
    - Updated the corlib_macros dependency to version 0.1.1.
    
    - Updated the readme.
    
    - Renamed the any::any module to any::any_traits.
    
    - Added the type_id, is, downcast_ref, downcast_mut, as_any_ref and as_any_mut functions to the “any” module.
    
    - Updated documentation

commit bac10f71f9a6a6e119146db6d055a34cce647971 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 3 19:50:40 2026 +1200

    - Added a package authors field with "Paul Saunders" as an element.
    
    - The corlib_macros dependency now points to a local repository.
    
    - Cleaned up the Cargo.toml file.
    
    - Prepared the changelog.
    
    - Updated the readme.
    
    - Renamed the “convert” module to “any”.
    
    - Updated documentation
    
    - Added a “from” method to the Immut implementation.
    
    - Moved the “upgrading” module under the “rc” module.
    
    - Added a “from” method to the serde::Skip implementation.
    
    - Other minor changes.

commit a434603cd4c4252d2e0a7f7b4c720e32c0408795 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jul 2 22:18:51 2026 +1200

    - Updated the readme.
    
    - ControlFlow now conditionally derives the serde Serialize and Deserialize macros.
    
    - Updated documentation
    
    - Conditionally implemented the Default trait for the GapFillingCounter struct.
    
    - Immut now conditionally implements the Copy, PartialEq, Eq, PartialOrd, Ord and Hash traits.
    
    - Skip now conditionally implements the Copy, PartialEq, Eq, PartialOrd, Ord and Hash traits.
    
    - Other minor changes.

commit e993a9ce5e471ace8d35a74457ee946f7d835ed8 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 1 20:34:05 2026 +1200

    - Added the corlib_macros dependency.
    
    - Added a macros feature.
    
    - Conditionally exposed the corlib_macros::AsAnyRef and corlib_macros::AsAnyMut derive macros in the convert module.
    
    - Fixed a couple of errors with the Immut struct implementation.
    
    - Conditionally exposed the corlib_macros::WeakSelf derive macro.
    
    - Conditionally exposed the corlib_macros::RcDefault and corlib_macros::ArcDefault derive macros in the convert module.

commit d987e51dfc6aa6af63dc3888a31067b323457543 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jun 30 19:49:28 2026 +1200

    - Added the Cargo.lock file.
    
    - Updated documentation
    
    - Removed the derive attribute with the Serialize and Deserialize macro declarations from the Immut struct.
    
    - Implemented the Clone trait for the Immut struct.
    
    - Implemented the Display trait for the Immut struct.
    
    - Removed the serde::GenericVisitor struct.
    
    - Removed the serde::GenericVisitorExpected struct.
    
    - Implemented the Clone trait for the serde::Skip struct.
    
    - Implemented the Display trait for the serde::Skip struct.
    
    - Added a better_skip_test test function to the serde::skip_tests test module.
    
    - Other minor changes.

commit 499c2cb6c1a9924ef5a85709153abbe3c2775023 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Jun 27 20:08:24 2026 +1200

    - Added a GenericVisitor struct to the serde sub module.
    
    - Added a GenericVisitorExpected structto the serde sub module.

commit 59347d2790eeda5a4e06c50c99c824f002939b6f -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jun 26 18:47:07 2026 +1200

    - Removed the highly_sendable, env_var_helpers, inc_dec, capped_collections and accessorise optional dependencies.
    
    - Disabled the serde cfg_attr on the Immut struct.
    
    - Renamed the “item” field to “object” in the Immut struct definition.
    
    - Added a Display implementation of the Immut struct.
    
    - Added conditional serde Serialize and Deserialize implementations of the Immut struct.
    
    - Added the immut_tests test module with serialize_deserialize_test and serialize_deserialize_test_2 test functions.

commit bb6555730085e3654b5e13881d6823a0d7621941 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jun 23 20:29:05 2026 +1200

    - Updated the package edition to “2024”.
    
    - Added documentation
    
    - Decorated the impl_weak_self_trait macro with the deprecated attribute.
    
    - Other minor changes.

commit 52345a564ae53f5706d22b56559a8acf09a4f150 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jun 22 21:07:30 2026 +1200

    - Added serde_json as a dev-dependency.
    
    - Added documentation
    
    - Removed the impl_as_any_ref_method and impl_as_any_mut_method macros from the convert module.
    
    - Added a skip_tests test module with a skip_test test function to the serde sub-module.

commit a823ec46f712ea4d9586553a38afc735d35f525e -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jun 18 22:15:16 2026 +1200

    - Added RcRefCell<T> and WeakRefCell<T> type aliases to the cell module.
    
    - Deprecated the impl_as_any_ref macro.
    
    - Made the AsAnyMut trait, in the convert module, require the implementation of the AsAnyRef trait.
    
    - Deprecated the impl_as_any_mut macro.
    
    - Made the Self type aliases of the RcDefault and ArcDefault traits, of the rc module, require that the Default trait be implemented for them.

commit e7a758dea083950b98aad840f2bda2ba9a57dec7 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jun 12 16:54:07 2026 +1200

    - Removed the cargo_env module and its contents.
    
    - Removed ThisOrThat, ThisThatOther and DefaultOrValue enums.
    
    - Removed the getters_setters_callers module and its contents.
    
    - Removed the inc_dec module and its contents.
    
    - Changed “doc_auto_cfg” to “doc_cfg” in the docsrs package level cfg_attr statement.
    
    - Added the AsMutStr trait to the text module.
    
    - Removed the SendableText enum and the SendableTextLog and SendableTextLogWithBuffer structs from the text module.
    
    - Removed the WorkInProgressResult and IdedWorkInProgressResult structs.
    
    - Other minor changes

commit 16e822708e98d47520cfcdd2c44caa923a6c37dc -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jun 11 19:57:23 2026 +1200

    - Updated the serde dependency to version 1.0.228.
    
    - Updated the serde dependency to now use the “derive” feature.
    
    - Updated the cfg-if dependency to 1.0.4.
    
    - Added a From<ControlFlow> implementation for the bool type.
    
    - The Immut struct now conditionally derives the serde Serialize and Deserialize traits.
    
    - Added the serde public module.
    
    - Added the Skip struct to the serde module.

commit 68431609b8b811f542fc7af7dd07fd70edff56be -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Feb 24 14:44:49 2026 +1300

    - Added a ControlFlow struct.
    
    - Updated the package version string to "0.5.0-alpha".

commit d66a746fc283cc6f9d0e1024b7207bb608e37bd4 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Jul 19 16:53:23 2025 +1200

    Added may_retry_count and may_retry_count_return macros.

commit 90253766ff1b7af6e7d1791b5e7b60f752536dbc -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 18 18:22:21 2025 +1200

    - Added the may_retry and may_retry_return public macros as well as the once_twice and once_twice_return test functions for these macros.

commit 9635a41cfdc1199c65e87c9cf8ef43fb4e89ed63 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jun 30 17:39:01 2025 +1200

    - Added highly_sendable, env_var_helpers, inc_dec, capped_collections and accessorise crates as optional dependencies and exposed them as extern crates in the lib file.
    
    - Updated the documentation of the DropPanic struct.
    
    - Added the ThisOrThat, ThisThatOther and DefaultOrValue enums.
    
    - Disabled the cfg_if and get_some use statements in the lib file.
    
    - Disabled the getters_setters_callers, inc_dec, work_in_progress_result (WorkInProgressResult
    and IdedWorkInProgressResult structs) and cargo_env modules in the lib file.
    
    - Disabled the drop_panic cfg_if block and replaced it with conditional mod and use statements.
    
    - Updated the AsStr trait documentation in the text module.
    
    - Disabled the sendable_text, sendable_text_log and sendable_text_log_with_buffer sub-modules in the text module.
