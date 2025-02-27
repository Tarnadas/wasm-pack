use assert_cmd::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use utils::{self, fixture};
use wasm_pack::command::build::Target;
use wasm_pack::command::utils::get_crate_path;
use wasm_pack::{self, license, manifest};

#[test]
fn it_gets_the_crate_name_default_path() {
    let path = &PathBuf::from(".");
    let crate_data = manifest::CrateData::new(&path, None).unwrap();
    let name = crate_data.crate_name();
    assert_eq!(name, "wasm_pack");
}

#[test]
fn it_gets_the_crate_name_provided_path() {
    let fixture = fixture::js_hello_world();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    assert_eq!(crate_data.crate_name(), "js_hello_world");
}

#[test]
fn it_gets_the_default_name_prefix() {
    let path = &PathBuf::from(".");
    let crate_data = manifest::CrateData::new(&path, None).unwrap();
    let name = crate_data.name_prefix();
    assert_eq!(name, "wasm_pack");
}

#[test]
fn it_gets_the_name_prefix_passed_from_cli() {
    let path = &PathBuf::from(".");
    let crate_data = manifest::CrateData::new(&path, Some("index".to_owned())).unwrap();
    let name = crate_data.name_prefix();
    assert_eq!(name, "index");
}

#[test]
fn it_checks_has_cdylib_default_path() {
    let fixture = fixture::no_cdylib();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    assert!(crate_data.check_crate_config().is_err());
}

#[test]
fn it_checks_has_cdylib_provided_path() {
    let fixture = fixture::js_hello_world();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    crate_data.check_crate_config().unwrap();
}

#[test]
fn it_checks_has_cdylib_wrong_crate_type() {
    let fixture = fixture::bad_cargo_toml();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    assert!(crate_data.check_crate_config().is_err());
}

#[test]
fn it_recognizes_a_map_during_depcheck() {
    let fixture = fixture::serde_feature();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    crate_data.check_crate_config().unwrap();
}

#[test]
fn it_creates_a_package_json_default_path() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Bundler, false)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");
    assert_eq!(pkg.side_effects, false);

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world.d.ts",
        "js_hello_world_bg.js",
        "js_hello_world_bg.wasm",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Bundler, false)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world.d.ts",
        "js_hello_world_bg.js",
        "js_hello_world_bg.wasm",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path_with_scope() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(
            &out_dir,
            &Some("test".to_string()),
            false,
            Target::Bundler,
            false
        )
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "@test/js-hello-world");
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world.d.ts",
        "js_hello_world_bg.js",
        "js_hello_world_bg.wasm",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_with_correct_files_on_node() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Nodejs, false)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.main, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world.d.ts",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_with_correct_files_on_nomodules() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::NoModules, false)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.browser, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world.d.ts",
        "js_hello_world_bg.wasm",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_with_correct_files_when_out_name_is_provided() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, Some("index".to_owned())).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Bundler, false)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "index.js");
    assert_eq!(pkg.types, "index.d.ts");
    assert_eq!(pkg.side_effects, false);

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> =
        ["index_bg.wasm", "index_bg.js", "index.d.ts", "index.js"]
            .iter()
            .map(|&s| String::from(s))
            .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_in_out_dir() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("./custom/out");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Bundler, false)
        .is_ok());

    let package_json_path = &fixture.path.join(&out_dir).join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
}

#[test]
fn it_creates_a_package_json_with_correct_keys_when_types_are_skipped() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, true, Target::Bundler, false)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world_bg.js",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_with_correct_files_when_is_child_is_provided() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, Some("index".to_owned())).unwrap();
    let crate_data_child =
        manifest::CrateData::new(&fixture.path, Some("child".to_owned())).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Web, false)
        .is_ok());
    assert!(crate_data_child
        .write_package_json(&out_dir, &None, false, Target::Web, true)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "index.js");
    assert_eq!(pkg.types, "index.d.ts");
    assert_eq!(pkg.side_effects, false);

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "index_bg.wasm",
        "index.d.ts",
        "index.js",
        "child_bg.wasm",
        "child.d.ts",
        "child.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_errors_when_wasm_bindgen_is_not_declared() {
    let fixture = fixture::bad_cargo_toml();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    assert!(crate_data.check_crate_config().is_err());
}

#[test]
fn it_errors_when_out_dir_of_child_does_not_exist() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let out_dir_child = fixture.path.join("pkg-child");
    let crate_data = manifest::CrateData::new(&fixture.path, Some("index".to_owned())).unwrap();
    let crate_data_child =
        manifest::CrateData::new(&fixture.path, Some("child".to_owned())).unwrap();
    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, Target::Web, false)
        .is_ok());
    assert!(crate_data_child
        .write_package_json(&out_dir_child, &None, false, Target::Web, true)
        .is_err());
}

#[test]
fn it_sets_homepage_field_if_available_in_cargo_toml() {
    // When 'homepage' is available
    let fixture = utils::fixture::Fixture::new();
    fixture.hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "homepage-field-test"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"
            homepage = "https://rustwasm.github.io/wasm-pack/"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "=0.2"

            [dev-dependencies]
            wasm-bindgen-test = "=0.2"
        "#,
    );

    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();

    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, true, Target::Bundler, false)
        .unwrap();

    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(
        pkg.homepage,
        Some("https://rustwasm.github.io/wasm-pack/".to_string()),
    );

    // When 'homepage' is unavailable
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();

    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, true, Target::Bundler, false)
        .unwrap();

    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.homepage, None);
}

#[test]
fn it_sets_keywords_field_if_available_in_cargo_toml() {
    // When 'homepage' is available
    let fixture = utils::fixture::Fixture::new();
    fixture.hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "homepage-field-test"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"
            keywords = ["wasm"]

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "=0.2"

            [dev-dependencies]
            wasm-bindgen-test = "=0.2"
        "#,
    );

    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();

    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, true, Target::Bundler, false)
        .unwrap();

    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let keywords = pkg.keywords.clone().unwrap();
    assert!(
        keywords.contains(&"wasm".to_string()),
        "keywords is not in files: {:?}",
        keywords,
    );

    // When 'keywords' is unavailable
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();

    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, true, Target::Bundler, false)
        .unwrap();

    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.keywords, None);
}

#[test]
fn it_does_not_error_when_wasm_bindgen_is_declared() {
    let fixture = fixture::js_hello_world();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();
    crate_data.check_crate_config().unwrap();
}

#[test]
fn configure_wasm_bindgen_debug_incorrectly_is_error() {
    let fixture = utils::fixture::Fixture::new();
    fixture.readme().hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "whatever"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            [package.metadata.wasm-pack.profile.dev.wasm-bindgen]
            debug-js-glue = "not a boolean"
            "#,
    );
    fixture
        .wasm_pack()
        .arg("build")
        .arg("--dev")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "package.metadata.wasm-pack.profile.dev.wasm-bindgen.debug",
        ));
}

#[test]
fn parse_crate_data_returns_unused_keys_in_cargo_toml() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .file(
            "Cargo.toml",
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "whatever"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            # Note: production is not valid.
            [package.metadata.wasm-pack.profile.production.wasm-bindgen]
            debug-js-glue = true
            "#,
        )
        .hello_world_src_lib()
        .install_local_wasm_bindgen();
    fixture
        .wasm_pack()
        .arg("build")
        .assert()
        .success()
        .stderr(predicates::str::contains(
        "[WARN]: :-) \"package.metadata.wasm-pack.profile.production\" is an unknown key and will \
         be ignored. Please check your Cargo.toml.",
    ));
}

#[test]
fn it_lists_license_files_in_files_field_of_package_json() {
    let fixture = fixture::dual_license();
    let out_dir = fixture.path.join("pkg");

    let crate_data = manifest::CrateData::new(&fixture.path, None).unwrap();

    wasm_pack::command::utils::create_pkg_dir(&out_dir).unwrap();
    license::copy_from_crate(&crate_data, &fixture.path, &out_dir).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, false, Target::Bundler, false)
        .unwrap();

    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();

    assert!(
        pkg.files.contains(&"LICENSE-WTFPL".to_string()),
        "LICENSE-WTFPL is not in files: {:?}",
        pkg.files,
    );

    assert!(
        pkg.files.contains(&"LICENSE-MIT".to_string()),
        "LICENSE-MIT is not in files: {:?}",
        pkg.files,
    );
}

#[test]
fn it_recurses_up_the_path_to_find_cargo_toml() {
    let fixture = utils::fixture::Fixture::new();
    fixture.hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "recurse-for-manifest-test"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"
            homepage = "https://rustwasm.github.io/wasm-pack/"
        "#,
    );
    let path = get_crate_path(None).unwrap();
    let crate_data = manifest::CrateData::new(&path, None).unwrap();
    let name = crate_data.crate_name();
    assert_eq!(name, "wasm_pack");
}

#[test]
fn it_doesnt_recurse_up_the_path_to_find_cargo_toml_when_default() {
    let fixture = utils::fixture::Fixture::new();
    fixture.hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "recurse-for-manifest-test"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"
            homepage = "https://rustwasm.github.io/wasm-pack/"
        "#,
    );
    let path = get_crate_path(Some(PathBuf::from("src"))).unwrap();
    let crate_data = manifest::CrateData::new(&path, None);
    assert!(crate_data.is_err());
}
