use std::env;
use std::collections::HashSet;
use std::time::Duration;
use std::path::PathBuf;
use cargo_tarpaulin::event_log::EventLog;
use cargo_tarpaulin::path_utils::*;
use cargo_tarpaulin::traces::TraceMap;
use cargo_tarpaulin::{
    args::TarpaulinCli,
    config::{Color, Config, ConfigWrapper, Mode, OutputFile, RunType, TraceEngine},
};
use cargo_tarpaulin::{launch_tarpaulin, run, setup_logging};
use clap::Parser;

fn get_test_path(test_dir_name: &str) -> PathBuf {
    let mut test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    test_dir.push("tests");
    test_dir.push("data");
    test_dir.push(test_dir_name);
    test_dir
}

pub fn check_percentage(project_name: &str, minimum_coverage: f64, has_lines: bool) -> TraceMap {
    let mut config = Config::default();
    config.set_include_tests(true);
    config.set_clean(false);
    check_percentage_with_config(project_name, minimum_coverage, has_lines, config)
}

pub fn check_percentage_with_config(
    project_name: &str,
    minimum_coverage: f64,
    has_lines: bool,
    mut config: Config,
) -> TraceMap {
    setup_logging(Color::Never, false, false);
    config.test_timeout = Duration::from_secs(60);
    let restore_dir = env::current_dir().unwrap();
    let test_dir = get_test_path(project_name);
    env::set_current_dir(&test_dir).unwrap();
    let mut manifest = test_dir;
    manifest.push("Cargo.toml");
    config.set_manifest(manifest);
    config.set_clean(false);

    // Note to contributors. If an integration test fails, uncomment this to be able to see the
    // tarpaulin logs
    //cargo_tarpaulin::setup_logging(true, true);
    let event_log = if config.dump_traces {
        let mut paths = HashSet::new();
        paths.insert(config.manifest());
        Some(EventLog::new(paths, &config))
    } else {
        None
    };

    let (res, ret) = launch_tarpaulin(&config, &event_log).unwrap();
    assert_eq!(ret, 0);

    env::set_current_dir(restore_dir).unwrap();
    if has_lines {
        assert!(res.total_coverable() > 0);
        assert!(
            res.coverage_percentage() >= minimum_coverage,
            "Assertion failed {} >= {}",
            res.coverage_percentage(),
            minimum_coverage
        );
    } else {
        assert_eq!(res.total_coverable(), 0);
    }
    res
}

#[test]
fn cargo_run_coverage() {
    let mut config = Config::default();
    config.command = Mode::Build;
    config.set_clean(false);
    check_percentage_with_config("sss-computing-strength", 1.0f64, true, config);
}