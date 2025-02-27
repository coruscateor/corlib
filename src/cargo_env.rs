//!
//! This module mostly contains functions that return cargo environment variables.
//! 
//! <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
//! 
//! Excludes those environment variables only used during compilation.
//! 

//https://nrsk.dev/articles/capturing-doc-comments

//($(#[$attr:meta])* )

/*
///
/// Fetches the value associated with the $name environment variable.
/// 
 */

#[macro_export]
macro_rules! impl_cargo_env_accessor
{

    ($fn_name:ident, $name:ident) =>
    {

        #[doc = "Fetches the value associated with the "]
        #[doc = stringify!($name)]
        #[doc = " environment variable."]
        pub fn $fn_name() -> &'static str
        {

            env!(stringify!($name))

        }

    }

}

#[macro_export]
macro_rules! impl_cargo_env_accessor_pair
{

    ($fn_name:ident, $name:ident) =>
    {

        #[doc = "Fetches the value associated with the "]
        #[doc = stringify!($name)]
        #[doc = " environment variable. Returns a tuple with the name of the environment variable used in the first position and the value retrieved in the second position."]
        pub fn $fn_name() -> (&'static str, &'static str)
        {

            (stringify!($name), env!(stringify!($name)))

            //expected string literal

            /*
            let name = stringify!($name);

            (name, env!(name))
            */

        }

    }

}

//https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

impl_cargo_env_accessor!(cargo, CARGO);

impl_cargo_env_accessor_pair!(cargo_pair, CARGO);



impl_cargo_env_accessor!(cargo_manifest_dir, CARGO_MANIFEST_DIR);

impl_cargo_env_accessor_pair!(cargo_manifest_dir_pair, CARGO_MANIFEST_DIR);



impl_cargo_env_accessor!(cargo_manifest_path, CARGO_MANIFEST_PATH);

impl_cargo_env_accessor_pair!(cargo_manifest_path_pair, CARGO_MANIFEST_PATH);



impl_cargo_env_accessor!(cargo_pkg_version, CARGO_PKG_VERSION);

impl_cargo_env_accessor_pair!(cargo_pkg_version_pair, CARGO_PKG_VERSION);



impl_cargo_env_accessor!(cargo_pkg_version_major, CARGO_PKG_VERSION_MAJOR);

impl_cargo_env_accessor_pair!(cargo_pkg_version_major_pair, CARGO_PKG_VERSION_MAJOR);



impl_cargo_env_accessor!(cargo_pkg_version_minor, CARGO_PKG_VERSION_MINOR);

impl_cargo_env_accessor_pair!(cargo_pkg_version_minor_pair, CARGO_PKG_VERSION_MINOR);



impl_cargo_env_accessor!(cargo_pkg_version_patch, CARGO_PKG_VERSION_PATCH);

impl_cargo_env_accessor_pair!(cargo_pkg_version_patch_pair, CARGO_PKG_VERSION_PATCH);



impl_cargo_env_accessor!(cargo_pkg_version_pre, CARGO_PKG_VERSION_PRE);

impl_cargo_env_accessor_pair!(cargo_pkg_version_pre_pair, CARGO_PKG_VERSION_PRE);



impl_cargo_env_accessor!(cargo_pkg_authors, CARGO_PKG_AUTHORS);

impl_cargo_env_accessor_pair!(cargo_pkg_authors_pair, CARGO_PKG_AUTHORS);



impl_cargo_env_accessor!(cargo_pkg_name, CARGO_PKG_NAME);

impl_cargo_env_accessor_pair!(cargo_pkg_name_pair, CARGO_PKG_NAME);



impl_cargo_env_accessor!(cargo_pkg_description, CARGO_PKG_DESCRIPTION);

impl_cargo_env_accessor_pair!(cargo_pkg_description_pair, CARGO_PKG_DESCRIPTION);



impl_cargo_env_accessor!(cargo_pkg_homepage, CARGO_PKG_HOMEPAGE);

impl_cargo_env_accessor_pair!(cargo_pkg_homepage_pair, CARGO_PKG_HOMEPAGE);



impl_cargo_env_accessor!(cargo_pkg_repository, CARGO_PKG_REPOSITORY);

impl_cargo_env_accessor_pair!(cargo_pkg_repository_pair, CARGO_PKG_REPOSITORY);



impl_cargo_env_accessor!(cargo_pkg_license, CARGO_PKG_LICENSE);

impl_cargo_env_accessor_pair!(cargo_pkg_license_pair, CARGO_PKG_LICENSE);



impl_cargo_env_accessor!(cargo_pkg_license_file, CARGO_PKG_LICENSE_FILE);

impl_cargo_env_accessor_pair!(cargo_pkg_license_file_pair, CARGO_PKG_LICENSE_FILE);



impl_cargo_env_accessor!(cargo_pkg_rust_version, CARGO_PKG_RUST_VERSION);

impl_cargo_env_accessor_pair!(cargo_pkg_rust_version_pair, CARGO_PKG_RUST_VERSION);



impl_cargo_env_accessor!(cargo_pkg_readme, CARGO_PKG_README);

impl_cargo_env_accessor_pair!(cargo_pkg_readme_pair, CARGO_PKG_README);



//impl_cargo_env_accessor!(cargo_crate_name, CARGO_CRATE_NAME);

//impl_cargo_env_accessor!(cargo_bin_name, CARGO_BIN_NAME);

//impl_cargo_env_accessor!(out_dir, OUT_DIR);



/* 
impl_pub_env_accessor!
{

    ///
    /// Fetches the value associated with the CARGO envionement variable.
    /// 
    (cargo, CARGO)

}
*/
