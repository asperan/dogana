//! # Dogana
//! Dogana is a library to support test containers in integration tests for CLI tools.
//!
//! When a Dogana test is created and run, a container image with all the binaries of a package is
//! built and a container based on it is run.
//!
//! The usage of LazyLocks allows tests to run concurrently and to init/build images just the first
//! time they are referenced.
//!
//! ## Requirements
//! This library requires that a container manager runtime (such as `podman` or `docker`) is
//! installed in the system, and available in the system `PATH`.
//!
//! The supported container manager runtimes are:
//! - `podman`
//! - `docker`
//!
//! ## Configuration
//! Dogana images are configured mostly through metadata in Cargo.toml of the package, and they
//! uses:
//! * The package name.
//! * The package version.
//! * The rust-version, if any, else the version returned by the in-use rust compiler.
//! * The system packages to install in the images, categorized by image variants, in the metadata
//!   section:
//!     ```toml
//!     [package.metadata.dogana.<variant>]
//!     required_packages = []
//!     ```
//!     Each system package is a string that can be installed by the system package manager (e.g.
//!     apt-get for debian derivatives, or apk for alpine).
//!
//! Each supported image defines a metadata key. You can see all the supported images in
//! [dogana_images].
//!
//! ## Usage
//! Dogana can be used trasparently within a test method:
//! ```ignore
//! use dogana::{
//!     dogana_images::DEBIAN_IMAGE,
//!     dogana_test::{builder::DoganaTestBuilder, test_options::DoganaTestOptions, DoganaTestResult},
//! };
//!
//! #[test]
//! fn basic_integration_test() -> DoganaTestResult {
//!     DoganaTestBuilder::new()
//!         .set_test_options(DoganaTestOptions {
//!             keep_old_containers: true,
//!             ..Default::default()
//!         })
//!         .set_base_image(&DEBIAN_IMAGE)
//!         .set_run_commands(&["echo \"test\""])
//!         .set_expected_output("test")
//!         .build()
//!         .run()
//! }
//! ```
//!
//! As you can see, thanks to the possibility of expliciting a return value for test methods,
//! Dogana tests can be very concise.

mod container_manager;
pub mod dogana_images;
pub mod dogana_test;
mod image_builder;
mod image_builder_factory;
pub mod image_name;
mod metadata;
