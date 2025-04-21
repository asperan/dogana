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
