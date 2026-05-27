# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.3...v0.1.4) (2026-05-27)


### Features

* change welcoming sentence ([3f55fc8](https://github.com/openstreetlifting/openstreetlifting/commit/3f55fc8bce6464daa1a1a7bf6e59a6bedbb255aa))
* documentation, github workflow ([7800fc3](https://github.com/openstreetlifting/openstreetlifting/commit/7800fc31112eb475e5f7fb454315250eeb8a0d84))
* **frontend:** helm chart backend url values ([1e30f30](https://github.com/openstreetlifting/openstreetlifting/commit/1e30f309600de17bce28ae0a38e5f86e595ac586))
* merge continuous delivery pipeline ([47ece34](https://github.com/openstreetlifting/openstreetlifting/commit/47ece3423448e54d352b9a18c6433ccf0bc6835c))
* simplify backend and frontend continuous delivery ([a2576d3](https://github.com/openstreetlifting/openstreetlifting/commit/a2576d39180ec1aecded5065ab75fc27562051a2))


### Bug Fixes

* chart values backend intra url ([b32666c](https://github.com/openstreetlifting/openstreetlifting/commit/b32666cc46c22c1318c996dc233f67919ac64c32))
* typo in release please workflow ([bbecfdd](https://github.com/openstreetlifting/openstreetlifting/commit/bbecfdd609480ff142d1cc99b0e9e16ae948ae7d))

## [0.1.3](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.2...v0.1.3) (2026-05-25)


### Features

* add guard to avoid image push without ci ([9735129](https://github.com/openstreetlifting/openstreetlifting/commit/9735129f3d2d324b6405df67eefb4dff631d4a5a))
* **backend:** add ci rust cache directory ([1805488](https://github.com/openstreetlifting/openstreetlifting/commit/1805488a8def82975f5f8bdb98de6b1bd558a8d6))
* harmonize CI and CD Execution in context ([d582397](https://github.com/openstreetlifting/openstreetlifting/commit/d582397f2648289520f85669f775f0d09842f547))


### Bug Fixes

* launch cd after push ci ([4e833a9](https://github.com/openstreetlifting/openstreetlifting/commit/4e833a9069b2c4534458a5b8672765bb32087b18))
* make release please commit lock file to pr branhces ([8148477](https://github.com/openstreetlifting/openstreetlifting/commit/814847791579de1be35a9bdfc0905c3745977d94))
* release please update cargo toml ([6cc0c57](https://github.com/openstreetlifting/openstreetlifting/commit/6cc0c576a31d2bfa6da1ab8f5d88f15bef5f6066))

## [0.1.2](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.1...v0.1.2) (2026-05-25)


### Bug Fixes

* cargo lock update after toml update ([7bef4b7](https://github.com/openstreetlifting/openstreetlifting/commit/7bef4b7a9fb4fa92ac07720ed58d6eb03fc8e330))
* missing action checkout ([67a3b37](https://github.com/openstreetlifting/openstreetlifting/commit/67a3b377809628c53d27ee551bbe629bb96dd373))
* path in release please rust toolchain ([032ab5e](https://github.com/openstreetlifting/openstreetlifting/commit/032ab5e9a777a4a2fd22eb92f675418ac5bdba3c))

## [0.1.1](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.0...v0.1.1) (2026-05-25)


### Features

* add license and license data ([5749066](https://github.com/openstreetlifting/openstreetlifting/commit/574906631bb61825699a3cdda23eaee1c89c0d0b))
* add lock up to date ([93f2246](https://github.com/openstreetlifting/openstreetlifting/commit/93f22460e384ffe945ac47a6fc2c1b0f4c264a48))
* add release please ([79df401](https://github.com/openstreetlifting/openstreetlifting/commit/79df40111ed660d372f7a2feff1ad4a3da845149))
* add sealed secret inside chart definition ([232786a](https://github.com/openstreetlifting/openstreetlifting/commit/232786a59bbc2432e7d65518ef370bb2d323da0f))
* better renovate config ([5c00556](https://github.com/openstreetlifting/openstreetlifting/commit/5c0055635ee5fcf80f9fa453fcd7f2b2912fea2b))
* **book:** introduce directory ([b8d973c](https://github.com/openstreetlifting/openstreetlifting/commit/b8d973c5915bf7288427e9fea6c1786058eb85c3))
* **cd:** rework cd and add postrelease pipeline ([ecc453b](https://github.com/openstreetlifting/openstreetlifting/commit/ecc453bb1f7ac85fcd4c302d27d3cc3c6403b48d))
* **ci:** add image release pipeline ([90f78d4](https://github.com/openstreetlifting/openstreetlifting/commit/90f78d42d972cdca3aaac03be18a6dbf273f8fc7))
* **ci:** change ci ([7bd2fa9](https://github.com/openstreetlifting/openstreetlifting/commit/7bd2fa972132e66ae787d3718b10451dfa921428))
* **dev:** rethink the dev experience ([e3d65aa](https://github.com/openstreetlifting/openstreetlifting/commit/e3d65aab66977b8176ba90aff3d6d9e5fd5c0b6e))
* **doc:** update readme ([193ccef](https://github.com/openstreetlifting/openstreetlifting/commit/193ccefaa0b67c27244a54f04341171a8d4a5267))
* front new logo ([bba81b0](https://github.com/openstreetlifting/openstreetlifting/commit/bba81b0048e45ffb9a34725b8b92e1f4bf3d3643))
* Helm chart for frontend and backend as one helm application ([b0bec20](https://github.com/openstreetlifting/openstreetlifting/commit/b0bec207b29aa28d910532d68334f716417c3f69))
* helm upgrade after iac sealed secret creation ([932e5c6](https://github.com/openstreetlifting/openstreetlifting/commit/932e5c664f2c6fab0e9a5f2b4da65c052866eaf2))
* legal and contact cleanup ([8e4e55d](https://github.com/openstreetlifting/openstreetlifting/commit/8e4e55d4618184ff2ef0cd82cfc5e60e51e6e999))
* **mdbook:** add mdbook starter ([822883b](https://github.com/openstreetlifting/openstreetlifting/commit/822883b0b0fe4f0924e121f2563fb7b97e8be6fa))
* **mdbook:** add mdbook static pages workflow ([f654d32](https://github.com/openstreetlifting/openstreetlifting/commit/f654d3268d418bacca9bb390b168395a455b9abd))
* new ci pipeline for backend and frontend ([06a693d](https://github.com/openstreetlifting/openstreetlifting/commit/06a693d54fa20b63e0de95117ef1d3c35bc564aa))
* release please config for whole things ([4264b7e](https://github.com/openstreetlifting/openstreetlifting/commit/4264b7e1ea3f2892d3b5a2f1764a63bcf415293d))
* renovate ([632c264](https://github.com/openstreetlifting/openstreetlifting/commit/632c264a0a757bcac214a4500d9260847f45316d))
* rework documentation ([fa19793](https://github.com/openstreetlifting/openstreetlifting/commit/fa19793f9511fe0d570c6b6324fc8c995b5f39e3))
* **ris:** add RIS computation system ([c7495ad](https://github.com/openstreetlifting/openstreetlifting/commit/c7495adf571dc5f4d047fa238a1e699c0051a6ea))
* **sqlx:** schema migration change + workspace preparation ([a074f2a](https://github.com/openstreetlifting/openstreetlifting/commit/a074f2acd3320ec3460f81a9ec2b8f6efe40c771))
* **try:** try again ([e25a73a](https://github.com/openstreetlifting/openstreetlifting/commit/e25a73a0de80b4a822576d8858d790c8aa8d4037))
* **web:** add swagger ([a309d18](https://github.com/openstreetlifting/openstreetlifting/commit/a309d18aca8d542366a48a625b353acc1edbcc44))
* **web:** cors and Dockerfile optimization ([4aa4fb4](https://github.com/openstreetlifting/openstreetlifting/commit/4aa4fb4457adc089e45690684dca2d9eefceaf86))


### Bug Fixes

* cargo release process ([a10291e](https://github.com/openstreetlifting/openstreetlifting/commit/a10291e8edddfadbddabd46090f825f9f8effb1f))
* **deps:** update backend dependencies (cargo) ([#27](https://github.com/openstreetlifting/openstreetlifting/issues/27)) ([d228c3e](https://github.com/openstreetlifting/openstreetlifting/commit/d228c3ed33fcbe1ee3a94b18714fba20220e937d))
* fix cargo release by adding configuration toml ([e30d761](https://github.com/openstreetlifting/openstreetlifting/commit/e30d761236368d5d3bb924bafa7bbfa379eeff49))
* **migration:** fix sqlx migration path ([4460c2b](https://github.com/openstreetlifting/openstreetlifting/commit/4460c2b7241c7fbc12fddcc498625853ff192791))
* move residuals into backend crate ([2774804](https://github.com/openstreetlifting/openstreetlifting/commit/27748049238aebd4e5166ac2dc1f6b4c858d057a))
* **prod:** change readiness and liveness endpoint in chart ([4ef638a](https://github.com/openstreetlifting/openstreetlifting/commit/4ef638ab469a649d199e5dfdb0d0b6ead2f5a974))
* **release:** cargo release configuration ([48ff146](https://github.com/openstreetlifting/openstreetlifting/commit/48ff146d6847939dee825bafdbfedc4040cffa3a))
* run cd only after ci ([d32fe64](https://github.com/openstreetlifting/openstreetlifting/commit/d32fe64f96716efc9b1677033ee7fa280afacb3a))
* seperate cargo releaes from release please ([1ea60ef](https://github.com/openstreetlifting/openstreetlifting/commit/1ea60eff10a14f8ab5d751d8f6a6ad8b9afcbe9c))
* workspace publishable for crates ([85bdbc0](https://github.com/openstreetlifting/openstreetlifting/commit/85bdbc021ab4bac3952540f0f9494375bbca2ecd))

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
