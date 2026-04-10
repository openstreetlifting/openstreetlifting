# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.1...v0.1.2) (2026-04-10)


### Bug Fixes

* run cd only after ci ([d32fe64](https://github.com/openstreetlifting/openstreetlifting/commit/d32fe64f96716efc9b1677033ee7fa280afacb3a))

## [0.1.1](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.0...v0.1.1) (2026-04-10)


### Features

* add release please ([79df401](https://github.com/openstreetlifting/openstreetlifting/commit/79df40111ed660d372f7a2feff1ad4a3da845149))
* **book:** introduce directory ([b8d973c](https://github.com/openstreetlifting/openstreetlifting/commit/b8d973c5915bf7288427e9fea6c1786058eb85c3))
* **ci:** add image release pipeline ([90f78d4](https://github.com/openstreetlifting/openstreetlifting/commit/90f78d42d972cdca3aaac03be18a6dbf273f8fc7))
* **ci:** change ci ([7bd2fa9](https://github.com/openstreetlifting/openstreetlifting/commit/7bd2fa972132e66ae787d3718b10451dfa921428))
* **dev:** rethink the dev experience ([e3d65aa](https://github.com/openstreetlifting/openstreetlifting/commit/e3d65aab66977b8176ba90aff3d6d9e5fd5c0b6e))
* **doc:** update readme ([193ccef](https://github.com/openstreetlifting/openstreetlifting/commit/193ccefaa0b67c27244a54f04341171a8d4a5267))
* **mdbook:** add mdbook starter ([822883b](https://github.com/openstreetlifting/openstreetlifting/commit/822883b0b0fe4f0924e121f2563fb7b97e8be6fa))
* **mdbook:** add mdbook static pages workflow ([f654d32](https://github.com/openstreetlifting/openstreetlifting/commit/f654d3268d418bacca9bb390b168395a455b9abd))
* new ci pipeline for backend and frontend ([06a693d](https://github.com/openstreetlifting/openstreetlifting/commit/06a693d54fa20b63e0de95117ef1d3c35bc564aa))
* rework documentation ([fa19793](https://github.com/openstreetlifting/openstreetlifting/commit/fa19793f9511fe0d570c6b6324fc8c995b5f39e3))
* **ris:** add RIS computation system ([c7495ad](https://github.com/openstreetlifting/openstreetlifting/commit/c7495adf571dc5f4d047fa238a1e699c0051a6ea))
* **sqlx:** schema migration change + workspace preparation ([a074f2a](https://github.com/openstreetlifting/openstreetlifting/commit/a074f2acd3320ec3460f81a9ec2b8f6efe40c771))
* **web:** add swagger ([a309d18](https://github.com/openstreetlifting/openstreetlifting/commit/a309d18aca8d542366a48a625b353acc1edbcc44))
* **web:** cors and Dockerfile optimization ([4aa4fb4](https://github.com/openstreetlifting/openstreetlifting/commit/4aa4fb4457adc089e45690684dca2d9eefceaf86))


### Bug Fixes

* **migration:** fix sqlx migration path ([4460c2b](https://github.com/openstreetlifting/openstreetlifting/commit/4460c2b7241c7fbc12fddcc498625853ff192791))
* move residuals into backend crate ([2774804](https://github.com/openstreetlifting/openstreetlifting/commit/27748049238aebd4e5166ac2dc1f6b4c858d057a))

## [Unreleased]

## [0.0.2] - 2025-11-17

### Added
- New docker compose configuration
- Default environment variables and workflows

## [0.0.1] - 2025-11-14

### Changed
- Updated release workflow and deployment process

## [0.0.0] - 2025-11-14

### Added
- Initial Docker image release
- Rust backend application with web and import binaries
- Multi-platform Docker support (linux/amd64, linux/arm64)
- Database migrations support

[Unreleased]: https://github.com/adrienpelfresne/openstreetlifting_backend/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/adrienpelfresne/openstreetlifting_backend/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/adrienpelfresne/openstreetlifting_backend/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/adrienpelfresne/openstreetlifting_backend/releases/tag/v0.0.0
