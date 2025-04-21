use std::sync::Arc;

use uuid::Uuid;

use crate::image_name::ImageName;

use super::{test_options::DoganaTestOptions, DoganaTest};

/// Convenience struct for creating [DoganaTest]s.
///
/// This struct can also be used to prepare tests with similar values, as it is not invalidated
/// after building the test.
///
/// Building a test requires:
/// - the image name
/// - the run commands
/// - the expected output
///
/// The other fields are optional.
#[derive(Debug)]
pub struct DoganaTestBuilder {
    test_name: String,
    test_options: DoganaTestOptions,
    base_image: Option<Arc<ImageName>>,
    init_commands: Vec<String>,
    run_commands: Option<Vec<String>>,
    expected_output: Option<String>,
}

impl DoganaTestBuilder {
    /// Initialize a builder with default [DoganaTestOptions] and a randomly-generated test name.
    pub fn new() -> Self {
        DoganaTestBuilder {
            test_name: Uuid::new_v4().to_string(),
            test_options: Default::default(),
            base_image: None,
            init_commands: vec![],
            run_commands: None,
            expected_output: None,
        }
    }

    pub fn set_test_options(&mut self, test_options: DoganaTestOptions) -> &mut Self {
        self.test_options = test_options;
        self
    }

    pub fn set_base_image(&mut self, base_image: &Arc<ImageName>) -> &mut Self {
        self.base_image = Some(base_image.clone());
        self
    }

    pub fn set_init_commands(&mut self, init_commands: &[&str]) -> &mut Self {
        self.init_commands = init_commands.iter().map(|it| it.to_string()).collect();
        self
    }

    pub fn set_run_commands(&mut self, run_commands: &[&str]) -> &mut Self {
        self.run_commands = Some(run_commands.iter().map(|it| it.to_string()).collect());
        self
    }

    pub fn set_expected_output(&mut self, expected_output: &str) -> &mut Self {
        self.expected_output = Some(expected_output.to_string());
        self
    }

    pub fn build(&self) -> DoganaTest {
        let mut uninitialized_required_values = vec![];
        if self.base_image.is_none() {
            uninitialized_required_values.push("base_image");
        }
        if self.run_commands.is_none() {
            uninitialized_required_values.push("run_commands");
        }
        if self.expected_output.is_none() {
            uninitialized_required_values.push("expected_output");
        }
        if !uninitialized_required_values.is_empty() {
            panic!(
                "failed to initialize test: the fields {} are not initialized",
                uninitialized_required_values.join(", ")
            );
        }
        DoganaTest {
            test_name: self.test_name.clone(),
            base_image: self
                .base_image
                .as_ref()
                .expect("should have panicked if empty")
                .clone(),
            init_commands: self.init_commands.clone(),
            run_commands: self
                .run_commands
                .as_ref()
                .expect("should have panicked if empty")
                .clone(),
            expected_output: self
                .expected_output
                .as_ref()
                .expect("should have panicked if empty")
                .clone(),
            test_options: self.test_options.clone(),
        }
    }
}

impl Default for DoganaTestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use crate::dogana_images::DEBIAN_IMAGE;

    use super::*;

    #[test]
    #[should_panic]
    fn missing_a_required_field_panics() {
        DoganaTestBuilder::new().build();
    }

    #[test]
    fn setting_all_required_fields_allows_to_build_test() {
        let t = DoganaTestBuilder::new()
            .set_run_commands(&["true"])
            .set_expected_output("")
            .set_base_image(&DEBIAN_IMAGE)
            .build();
        assert!(t.type_id() == TypeId::of::<DoganaTest>());
    }
}
