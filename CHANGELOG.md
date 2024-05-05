# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Added
- Added `defmt-03` feature that derives `defmt::Format` for public data types.

## [1.1.0] - 2024-05-02

### Added
- Added types and support for setting accelerometer and gyroscope ranges and retrieving scale sensor values.

## [1.0.0] - 2024-01-17

### Changed
- Updated to `embedded-hal` 1.0.

## [0.1.2] - 2023-03-22

### Fixed
- Device communication when using SPI.

## [0.1.1] - 2023-03-17

### Changed
- Make `SlaveAddr::addr` public.
- Updated MSRV to version 1.62.0.

## [0.1.0] - 2020-02-08

Initial release to crates.io.

[Unreleased]: https://github.com/eldruin/bmi160-rs/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/eldruin/bmi160-rs/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/eldruin/bmi160-rs/compare/v0.1.2...v1.0.0
[0.1.2]: https://github.com/eldruin/bmi160-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/eldruin/bmi160-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/eldruin/bmi160-rs/releases/tag/v0.1.0