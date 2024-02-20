use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::str;
use cargo_test_support::CargoCommand;
use cargo_test_support::Project;

#[cargo_test]
fn case() {
    cargo_test_support::registry::init();
    cargo_test_support::registry::Package::new("clippy", "0.4.1+my-package").publish();
    cargo_test_support::registry::Package::new("docopt", "0.6.2+my-package").publish();
    cargo_test_support::registry::Package::new("regex", "0.1.1+my-package").publish();
    cargo_test_support::registry::Package::new("rustc-serialize", "0.4.0+my-package").publish();
    cargo_test_support::registry::Package::new("toml", "0.1.1+my-package").publish();
    cargo_test_support::registry::Package::new("semver", "0.1.1")
        .feature("std", &[])
        .publish();
    cargo_test_support::registry::Package::new("serde", "1.0.90")
        .feature("std", &[])
        .publish();

    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("remove")
        .args(["rustc-serialize"])
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches(str![""])
        .stderr_matches(file!["stderr.log"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
