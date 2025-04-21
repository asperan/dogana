# Dogana

Dogana is a rust library for running integration tests for CLI tools in containers.

Dogana divides the test in three phases:
- Image build
- Test environment initialization
- CLI test run

## Installation
Install this crate (as a dev dependency) with cargo:

```
cargo add --dev dogana
```

## Configuration
This crate requires that all tested packages have a name and a version.

Optionally, you can specify the `rust-version` of your crate and Dogana will detect it.

Also, you can configure required packages for each image variant. To do so, refer to the crate documentation.

## Usage
For each test that requires a container, create a tesst method which builds a DoganaTest:
```rust
use dogana::{
    dogana_images::DEBIAN_IMAGE,
    dogana_test::{builder::DoganaTestBuilder, test_options::DoganaTestOptions, DoganaTestResult},
};

#[test]
fn basic_integration_test() -> DoganaTestResult {
    DoganaTestBuilder::new()
        .set_test_options(DoganaTestOptions {
            keep_old_containers: true,
            ..Default::default()
        })
        .set_base_image(&DEBIAN_IMAGE)
        .set_run_commands(&["echo \"test\""])
        .set_expected_output("test")
        .build()
        .run()
}
```
