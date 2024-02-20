use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::prelude::*;
use cargo_test_support::str;
use cargo_test_support::Project;

#[cargo_test]
fn case() {
    cargo_test_support::registry::init();

    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = project_root.join("primary");

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("--path ../dependency --features your-face/nose")
        .current_dir(&cwd)
        .assert()
        .code(101)
        .stdout_matches(str![""])
        .stderr_matches(file!["stderr.log"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
