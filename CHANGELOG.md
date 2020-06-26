# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
 - `Shell::filter_names` now return `Iterator` instead of `Vec`.
 - `Shell::new_with_streams` is removed.
 - `cheats::io` is not private.

## [v0.3.1] - 2020-06-19
### Added
 - Added logging to functions and methods

## [v0.3.0] - 2020-06-18
### Added
 - `Shell::filter_names` method to filter codes.

## [v0.2.1] - 2020-06-18
### Changed
 - `Shell::run` now does not return any `ShellResult<()>`.
 - Changed visibility of `code::Code` from `pub` to `pub(crate)`.

## [v0.2.0] - 2020-06-18
### Changed
 - Instead of hardcoded parsing, [logos](https://crates.io/crates/logos) is used.

## [v0.1.0] - 2020-06-17

Initial release.