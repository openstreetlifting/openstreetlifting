# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.16.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.15.0...v0.16.0) (2026-08-27)


### Features

* **competitions:** add nineteen USA Streetlifting competitions ([#488](https://github.com/openstreetlifting/openstreetlifting/issues/488)) ([30776f9](https://github.com/openstreetlifting/openstreetlifting/commit/30776f97717463dcca6adfecd80b92c59dcd696a))
* **competitions:** add the first ever USA Streetlifting competition ([#474](https://github.com/openstreetlifting/openstreetlifting/issues/474)) ([886d31e](https://github.com/openstreetlifting/openstreetlifting/commit/886d31ee3ac67345c245d4cf1644eb4506ba24ff))


### Bug Fixes

* **frontend:** drop the sex from a disqualified row's weight class ([#485](https://github.com/openstreetlifting/openstreetlifting/issues/485)) ([e8c26cd](https://github.com/openstreetlifting/openstreetlifting/commit/e8c26cd78bbfd51215105a8419caae967dee655e))
* **frontend:** open a Classic competition on a column it can sort ([#479](https://github.com/openstreetlifting/openstreetlifting/issues/479)) ([eec8a15](https://github.com/openstreetlifting/openstreetlifting/commit/eec8a1558d1281e2d4e2fad56a62b2c81fb00df5))
* **importer:** let a re-import correct a competition's federation and dates ([#477](https://github.com/openstreetlifting/openstreetlifting/issues/477)) ([c663453](https://github.com/openstreetlifting/openstreetlifting/commit/c6634539dc285e296fa07f9a225cbdbd9e5a08ec))
* **importer:** let an athlete with only one name be imported ([#471](https://github.com/openstreetlifting/openstreetlifting/issues/471)) ([3b239d5](https://github.com/openstreetlifting/openstreetlifting/commit/3b239d5a41ddbf779c94e072c33dc4e05b2e1dbb))
* **importer:** reject a bombed movement left as a competed result ([#483](https://github.com/openstreetlifting/openstreetlifting/issues/483)) ([ec0b92c](https://github.com/openstreetlifting/openstreetlifting/commit/ec0b92c1b75d7193174d1b9b18263c7c6f7d2a1c))
* **importer:** reject a weight class bound of zero or less ([#478](https://github.com/openstreetlifting/openstreetlifting/issues/478)) ([6e2b299](https://github.com/openstreetlifting/openstreetlifting/commit/6e2b299dbb6acfcc7993b8a7e7a006838671e7fa))

## [0.15.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.14.0...v0.15.0) (2026-08-27)


### Features

* **frontend:** say when a RIS score was reported rather than computed ([#469](https://github.com/openstreetlifting/openstreetlifting/issues/469)) ([3ae73ad](https://github.com/openstreetlifting/openstreetlifting/commit/3ae73adf7712a5e0c5cb565a52f685cc1db085c1))


### Bug Fixes

* **frontend:** make the results tables readable on a phone ([#465](https://github.com/openstreetlifting/openstreetlifting/issues/465)) ([edc7e79](https://github.com/openstreetlifting/openstreetlifting/commit/edc7e797482226d3613cdf794fec0213b5c52a20))
* **frontend:** paint the page background on the root, not on a div ([#468](https://github.com/openstreetlifting/openstreetlifting/issues/468)) ([f2c5cba](https://github.com/openstreetlifting/openstreetlifting/commit/f2c5cbaaece991f942e96678437d887999acf9e2))
* **rankings:** give an athlete one place, at their best competition ([#463](https://github.com/openstreetlifting/openstreetlifting/issues/463)) ([2f8fe7b](https://github.com/openstreetlifting/openstreetlifting/commit/2f8fe7b8f66a86f70d0391ad26b19ae9d3cd6772))

## [0.14.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.13.0...v0.14.0) (2026-08-27)


### ⚠ BREAKING CHANGES

* **api:** tell a no-show apart from a disqualification, and show both ([#454](https://github.com/openstreetlifting/openstreetlifting/issues/454))
* **importer:** rename meet.toml to competition.toml ([#452](https://github.com/openstreetlifting/openstreetlifting/issues/452))

### Features

* **api:** tell a no-show apart from a disqualification, and show both ([#454](https://github.com/openstreetlifting/openstreetlifting/issues/454)) ([b59487b](https://github.com/openstreetlifting/openstreetlifting/commit/b59487b14e120ec8c805a4133070190443c8ae2e))
* **competitions:** add DCSV German Championship 2022 ([#455](https://github.com/openstreetlifting/openstreetlifting/issues/455)) ([5b3d7b5](https://github.com/openstreetlifting/openstreetlifting/commit/5b3d7b57570c00e0bae24a1e878de009318aad5d))
* **competitions:** add DCSV German Championship 2023 ([#456](https://github.com/openstreetlifting/openstreetlifting/issues/456)) ([cbea6ff](https://github.com/openstreetlifting/openstreetlifting/commit/cbea6ff84cbc252b7a6ff4f9e036db585b089aad))
* **competitions:** add DCSV German Championship 2024 ([#458](https://github.com/openstreetlifting/openstreetlifting/issues/458)) ([45ab344](https://github.com/openstreetlifting/openstreetlifting/commit/45ab3445ac7eb361a5b5b84f4272c52522025368))
* **frontend:** show the three attempts behind every lift ([#447](https://github.com/openstreetlifting/openstreetlifting/issues/447)) ([a1243d3](https://github.com/openstreetlifting/openstreetlifting/commit/a1243d3c82f9cde8375bd72a2c6b0b40342485ca))
* **importer:** rename meet.toml to competition.toml ([#452](https://github.com/openstreetlifting/openstreetlifting/issues/452)) ([32c7b62](https://github.com/openstreetlifting/openstreetlifting/commit/32c7b626685b8606ddd944f80e3bab54a13a0d37))


### Bug Fixes

* **frontend:** put the competition history on the shared table ([#459](https://github.com/openstreetlifting/openstreetlifting/issues/459)) ([5d3bf2f](https://github.com/openstreetlifting/openstreetlifting/commit/5d3bf2f01dd8ca9cf5701b9833fbd2b71e547b52))
* **importer:** score every meet with the current RIS formula ([#444](https://github.com/openstreetlifting/openstreetlifting/issues/444)) ([7afadd8](https://github.com/openstreetlifting/openstreetlifting/commit/7afadd8242a7f5dac004f18f10daff1e61aefa56))


### Refactoring

* **frontend:** give colour, type and wording one meaning ([#450](https://github.com/openstreetlifting/openstreetlifting/issues/450)) ([4405d78](https://github.com/openstreetlifting/openstreetlifting/commit/4405d787133ebb8843f4f28d432621fa1eaa66a3))

## [0.13.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.12.0...v0.13.0) (2026-08-24)


### Features

* **competitions:** filter and search the competition list ([#432](https://github.com/openstreetlifting/openstreetlifting/issues/432)) ([b74b908](https://github.com/openstreetlifting/openstreetlifting/commit/b74b908d463f8ca25a6cff09ed1054fae13f5e7f))

## [0.12.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.11.0...v0.12.0) (2026-08-24)


### ⚠ BREAKING CHANGES

* **importer:** drop format_version and reject unknown meet.toml keys ([#430](https://github.com/openstreetlifting/openstreetlifting/issues/430))
* **competitions:** announce a meet before it is lifted ([#428](https://github.com/openstreetlifting/openstreetlifting/issues/428))
* **api:** key results by weight class and division instead of category ([#427](https://github.com/openstreetlifting/openstreetlifting/issues/427))
* **importer:** replace canonical JSON with meet.toml and entries.csv ([#420](https://github.com/openstreetlifting/openstreetlifting/issues/420))

### Features

* **api:** key results by weight class and division instead of category ([#427](https://github.com/openstreetlifting/openstreetlifting/issues/427)) ([d40d746](https://github.com/openstreetlifting/openstreetlifting/commit/d40d746a25bca87ea8c1a11a150a771fed872bcf))
* **competitions:** announce a meet before it is lifted ([#428](https://github.com/openstreetlifting/openstreetlifting/issues/428)) ([d07e6d9](https://github.com/openstreetlifting/openstreetlifting/commit/d07e6d9331bb1b4bbc6683bffe512bbe863a9c16))
* **importer:** drop format_version and reject unknown meet.toml keys ([#430](https://github.com/openstreetlifting/openstreetlifting/issues/430)) ([fd3f12e](https://github.com/openstreetlifting/openstreetlifting/commit/fd3f12eb8ce6c20f3945532f4448b6cda99c8d59))
* **importer:** replace canonical JSON with meet.toml and entries.csv ([#420](https://github.com/openstreetlifting/openstreetlifting/issues/420)) ([2768098](https://github.com/openstreetlifting/openstreetlifting/commit/2768098385fe2fe586c21c27d71f857a2fd193e0))


### Bug Fixes

* **chart:** keep staging out of the analytics counts ([659c62a](https://github.com/openstreetlifting/openstreetlifting/commit/659c62a938b6eb6f671435875e8912e17083a1bb))
* **deploy:** copy the canonical data tree from its current path ([#429](https://github.com/openstreetlifting/openstreetlifting/issues/429)) ([ab7c259](https://github.com/openstreetlifting/openstreetlifting/commit/ab7c259298c26cd1245b069c3c60faa19de8cc02))
* **frontend:** load the umami script from the cluster instance ([#431](https://github.com/openstreetlifting/openstreetlifting/issues/431)) ([9c5a2ee](https://github.com/openstreetlifting/openstreetlifting/commit/9c5a2ee77e12d0b1d948e94e60a9a6f88c765d7a))

## [0.11.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.10.0...v0.11.0) (2026-08-21)


### ⚠ BREAKING CHANGES

* **canonical:** replace venue and judge count with region ([#402](https://github.com/openstreetlifting/openstreetlifting/issues/402))

### Features

* **athletes:** link an athlete's Instagram from a checked in file ([#412](https://github.com/openstreetlifting/openstreetlifting/issues/412)) ([03a9de0](https://github.com/openstreetlifting/openstreetlifting/commit/03a9de0d993e6307cbf080d7f50c287393cf6b88))
* **canonical:** replace venue and judge count with region ([#402](https://github.com/openstreetlifting/openstreetlifting/issues/402)) ([fc47f11](https://github.com/openstreetlifting/openstreetlifting/commit/fc47f1181c94e0936267b11d9f69a2cc6f106f3b))
* **competitions:** link the canonical file from the competition page ([#409](https://github.com/openstreetlifting/openstreetlifting/issues/409)) ([4ef0e9b](https://github.com/openstreetlifting/openstreetlifting/commit/4ef0e9b2c1e7f79db5debeace990b9f189d2075f))
* **rankings:** make the instagram icon white and use [#732734](https://github.com/openstreetlifting/openstreetlifting/issues/732734) for table headers ([80967f8](https://github.com/openstreetlifting/openstreetlifting/commit/80967f8145d1b8c4498eddb3c92fb134d2f4724a))
* **rankings:** search by athlete name ([#398](https://github.com/openstreetlifting/openstreetlifting/issues/398)) ([8a8bbc5](https://github.com/openstreetlifting/openstreetlifting/commit/8a8bbc5d1ba25be8cdf7dc6c584ac38c365c69ce))
* **rankings:** show the instagram icon next to an athlete's name ([2c3caaa](https://github.com/openstreetlifting/openstreetlifting/commit/2c3caaa5bfa821ff132d6eff25b198e150d0eefe))
* **rankings:** sort with a dropdown and drop the double fetch ([#406](https://github.com/openstreetlifting/openstreetlifting/issues/406)) ([fbcbc71](https://github.com/openstreetlifting/openstreetlifting/commit/fbcbc716dc4e0c1df4c92ffa24799346e109371a))


### Bug Fixes

* **frontend:** adapt the layout to phone widths ([#407](https://github.com/openstreetlifting/openstreetlifting/issues/407)) ([646f037](https://github.com/openstreetlifting/openstreetlifting/commit/646f037d0cfe54f0441365e97c21ff9d9f198d42))
* **frontend:** render in Inter instead of the system font ([#400](https://github.com/openstreetlifting/openstreetlifting/issues/400)) ([1aa6987](https://github.com/openstreetlifting/openstreetlifting/commit/1aa69870859b2f8bc5406dcea3c51dabab5b3e9d))

## [0.10.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.9.0...v0.10.0) (2026-08-15)


### Features

* deploy main to staging, release to production ([#394](https://github.com/openstreetlifting/openstreetlifting/issues/394)) ([0aae4bb](https://github.com/openstreetlifting/openstreetlifting/commit/0aae4bbd05cf7bdad9a3f5bbafb0978f76c84728))

## [0.9.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.8.0...v0.9.0) (2026-08-14)


### Features

* **frontend:** rank the home page by RIS, not a splash screen ([#386](https://github.com/openstreetlifting/openstreetlifting/issues/386)) ([7be7be5](https://github.com/openstreetlifting/openstreetlifting/commit/7be7be5511c8fd551bc95b97d1c1c37bcfa5bedd))
* **frontend:** render country flags as self-hosted SVGs ([#390](https://github.com/openstreetlifting/openstreetlifting/issues/390)) ([f59ec28](https://github.com/openstreetlifting/openstreetlifting/commit/f59ec281f90024857af40b1719a7fceb3a9849a2))


### Bug Fixes

* **canonical:** correct Lylia Ammour's first name in FNSL Elite 2026 ([#391](https://github.com/openstreetlifting/openstreetlifting/issues/391)) ([cf7864d](https://github.com/openstreetlifting/openstreetlifting/commit/cf7864d1a8019d50ec17dff16719da7099ca5deb))

## [0.8.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.7.0...v0.8.0) (2026-08-14)


### Features

* **canonical:** accept a stated best lift without attempts ([#385](https://github.com/openstreetlifting/openstreetlifting/issues/385)) ([686b581](https://github.com/openstreetlifting/openstreetlifting/commit/686b581c30ec220550877a9b76619147bf4701d8))
* **chart:** import the canonical files on every deploy ([#243](https://github.com/openstreetlifting/openstreetlifting/issues/243)) ([8c2e75c](https://github.com/openstreetlifting/openstreetlifting/commit/8c2e75c68abc4ce5d37f6679970f4a18f4f9be17))

## [0.7.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.6.0...v0.7.0) (2026-08-12)


### ⚠ BREAKING CHANGES

* **rankings:** stop giving disqualified lifters a place ([#236](https://github.com/openstreetlifting/openstreetlifting/issues/236))
* **rankings:** rank totals within one event ([#228](https://github.com/openstreetlifting/openstreetlifting/issues/228))

### Features

* **importer:** delete the competitions no file claims ([#241](https://github.com/openstreetlifting/openstreetlifting/issues/241)) ([4f6ca6c](https://github.com/openstreetlifting/openstreetlifting/commit/4f6ca6cc3e8db1c03a6af677e01aa54c1b0078cb))
* **rankings:** rank totals within one event ([#228](https://github.com/openstreetlifting/openstreetlifting/issues/228)) ([d7f3344](https://github.com/openstreetlifting/openstreetlifting/commit/d7f33449b269b6745ef1e99b0dcf8cf9bb7e56d3))


### Bug Fixes

* **db:** store a bodyweight lift and a bombed movement ([#231](https://github.com/openstreetlifting/openstreetlifting/issues/231)) ([bf16c65](https://github.com/openstreetlifting/openstreetlifting/commit/bf16c654b4f46942872643b4fd0b84fa1d2e44bd))
* **rankings:** stop giving disqualified lifters a place ([#236](https://github.com/openstreetlifting/openstreetlifting/issues/236)) ([9df75f3](https://github.com/openstreetlifting/openstreetlifting/commit/9df75f30a8754034938f73fefe718c2d5a6314bd))

## [0.6.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.5.0...v0.6.0) (2026-08-12)


### ⚠ BREAKING CHANGES

* **importer:** match athletes on a folded name ([#225](https://github.com/openstreetlifting/openstreetlifting/issues/225))
* **api:** remove the write endpoints ([#176](https://github.com/openstreetlifting/openstreetlifting/issues/176))

### Features

* **api:** remove the write endpoints ([#176](https://github.com/openstreetlifting/openstreetlifting/issues/176)) ([de05842](https://github.com/openstreetlifting/openstreetlifting/commit/de058427b0769f4b8dc81c4b5a7945b9a333caa1))
* **importer:** delete rows a canonical file no longer lists ([#167](https://github.com/openstreetlifting/openstreetlifting/issues/167)) ([e34bdba](https://github.com/openstreetlifting/openstreetlifting/commit/e34bdbad6a281bee44dc6bac149ddcc4809b415b))
* **importer:** match athletes on a folded name ([#225](https://github.com/openstreetlifting/openstreetlifting/issues/225)) ([ab5def2](https://github.com/openstreetlifting/openstreetlifting/commit/ab5def24c3490ad1db74ce0bf5bd6c1795d34ead))

## [0.5.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.4.0...v0.5.0) (2026-08-11)


### ⚠ BREAKING CHANGES

* **domain:** record whether a RIS score is computed or reported ([#161](https://github.com/openstreetlifting/openstreetlifting/issues/161))

### Features

* **domain:** record whether a RIS score is computed or reported ([#161](https://github.com/openstreetlifting/openstreetlifting/issues/161)) ([05c7233](https://github.com/openstreetlifting/openstreetlifting/commit/05c723302cf03c0187a5ff55ec9891903a6f9589))

## [0.4.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.3.2...v0.4.0) (2026-08-11)


### ⚠ BREAKING CHANGES

* **chart:** read DATABASE_URL from the postgres operator secret ([#156](https://github.com/openstreetlifting/openstreetlifting/issues/156))
* **db:** categories reference weight_classes ([#148](https://github.com/openstreetlifting/openstreetlifting/issues/148))
* **importer:** canonical format 1.3.0 with explicit weight class bounds ([#146](https://github.com/openstreetlifting/openstreetlifting/issues/146))

### Features

* **db:** add weight_classes table keyed on bounds ([#147](https://github.com/openstreetlifting/openstreetlifting/issues/147)) ([1a3e6bf](https://github.com/openstreetlifting/openstreetlifting/commit/1a3e6bf436daddf45ef3e0049fc14be327014a33))
* **db:** categories reference weight_classes ([#148](https://github.com/openstreetlifting/openstreetlifting/issues/148)) ([cc09480](https://github.com/openstreetlifting/openstreetlifting/commit/cc094806ef670c03c9f46c33d94a81a480b2560d))
* **importer:** canonical format 1.3.0 with explicit weight class bounds ([#146](https://github.com/openstreetlifting/openstreetlifting/issues/146)) ([17ba977](https://github.com/openstreetlifting/openstreetlifting/commit/17ba9770c761644f0a335f1f417d1dc72e155b10))


### Bug Fixes

* **importer:** apply corrections on re-import ([#141](https://github.com/openstreetlifting/openstreetlifting/issues/141)) ([44b0d05](https://github.com/openstreetlifting/openstreetlifting/commit/44b0d0598ec0cf53631b370150ad190d0df50f18))
* **importer:** import competitions as completed when the file omits status ([#149](https://github.com/openstreetlifting/openstreetlifting/issues/149)) ([30ebc06](https://github.com/openstreetlifting/openstreetlifting/commit/30ebc066cc8627bc05d7f920454273a4af93c35c))


### Chores

* **chart:** read DATABASE_URL from the postgres operator secret ([#156](https://github.com/openstreetlifting/openstreetlifting/issues/156)) ([2339d54](https://github.com/openstreetlifting/openstreetlifting/commit/2339d545155e683c864610e24953dce9568805ce))

## [0.3.2](https://github.com/openstreetlifting/openstreetlifting/compare/v0.3.1...v0.3.2) (2026-08-10)


### Bug Fixes

* **ci:** use github app token in cd and release deploy ([#134](https://github.com/openstreetlifting/openstreetlifting/issues/134)) ([3f5fce1](https://github.com/openstreetlifting/openstreetlifting/commit/3f5fce1a0cda6bf7039d58ed503d811ba35b846f))

## [0.3.1](https://github.com/openstreetlifting/openstreetlifting/compare/v0.3.0...v0.3.1) (2026-08-10)


### Bug Fixes

* **ci:** use github app token in cd and release deploy ([#132](https://github.com/openstreetlifting/openstreetlifting/issues/132)) ([69a479f](https://github.com/openstreetlifting/openstreetlifting/commit/69a479f7518bec7acdfcc684ecf79ba4df4977e0))

## [0.3.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.2.0...v0.3.0) (2026-08-10)


### ⚠ BREAKING CHANGES

* **importer:** canonical format 1.2.0 ([#131](https://github.com/openstreetlifting/openstreetlifting/issues/131))
* **importer:** canonical format 1.1.0 ([#124](https://github.com/openstreetlifting/openstreetlifting/issues/124))
* **importer:** canonical json is the only ingest path ([#123](https://github.com/openstreetlifting/openstreetlifting/issues/123))
* **api:** restful v1 surface ([#119](https://github.com/openstreetlifting/openstreetlifting/issues/119))

### Features

* **importer:** add import fmt ([#127](https://github.com/openstreetlifting/openstreetlifting/issues/127)) ([ac3e6ba](https://github.com/openstreetlifting/openstreetlifting/commit/ac3e6ba6255ef2fcba6f6908aac50c9c59b689ec))
* **importer:** canonical format 1.1.0 ([#124](https://github.com/openstreetlifting/openstreetlifting/issues/124)) ([837a69a](https://github.com/openstreetlifting/openstreetlifting/commit/837a69aabdff67acbb3c9b831b5e35f7f1fff05b))
* **importer:** canonical format 1.2.0 ([#131](https://github.com/openstreetlifting/openstreetlifting/issues/131)) ([f28703a](https://github.com/openstreetlifting/openstreetlifting/commit/f28703a21790161c76dc856bc68e88dc1e0779e3))
* **skill:** add extract-competition skill ([#128](https://github.com/openstreetlifting/openstreetlifting/issues/128)) ([189b324](https://github.com/openstreetlifting/openstreetlifting/commit/189b3249ef3e45fa60848065ea611eb70d996ab9))


### Bug Fixes

* **deps:** update backend dependencies (cargo) ([#115](https://github.com/openstreetlifting/openstreetlifting/issues/115)) ([704e5a1](https://github.com/openstreetlifting/openstreetlifting/commit/704e5a1b7573b9199a4a059041d0add04844b603))
* **deps:** update backend dependencies (cargo) to 0.7.0 ([#98](https://github.com/openstreetlifting/openstreetlifting/issues/98)) ([900f5fe](https://github.com/openstreetlifting/openstreetlifting/commit/900f5fe7d7ca353cf08ec01eed3e29350a81f181))


### Refactoring

* **api:** restful v1 surface ([#119](https://github.com/openstreetlifting/openstreetlifting/issues/119)) ([02f09be](https://github.com/openstreetlifting/openstreetlifting/commit/02f09be521e50025247a9772e159336df37e497e))
* **backend:** enforce api / domain / storage boundaries ([#116](https://github.com/openstreetlifting/openstreetlifting/issues/116)) ([d85e004](https://github.com/openstreetlifting/openstreetlifting/commit/d85e0046fd0deafef39811b645fee765a5dbccb1))
* **importer:** canonical json is the only ingest path ([#123](https://github.com/openstreetlifting/openstreetlifting/issues/123)) ([d6de6d2](https://github.com/openstreetlifting/openstreetlifting/commit/d6de6d213790c0d509f977ea718c542f68507286))


### Documentation

* **backend:** trim comments, fix stale test assertions ([#122](https://github.com/openstreetlifting/openstreetlifting/issues/122)) ([576bb40](https://github.com/openstreetlifting/openstreetlifting/commit/576bb4096f4ed4dcd26774104cc46329b2a535e7))

## [0.2.0](https://github.com/openstreetlifting/openstreetlifting/compare/v0.1.4...v0.2.0) (2026-05-27)


### Features

* **backend:** ci improvement ([0b4539a](https://github.com/openstreetlifting/openstreetlifting/commit/0b4539ad6dbc95ae29dc2312aa77fdf79e1ab82f))
* dockerfile ([01eeac1](https://github.com/openstreetlifting/openstreetlifting/commit/01eeac181d2331953a506fbd4c20cb9f0adbea26))
* **frontend:** new api and client architecture and fix backend var name ([729fbac](https://github.com/openstreetlifting/openstreetlifting/commit/729fbac5ee22f73a56d385bdd0fefa082fae6850))


### Bug Fixes

* change google release please behavior with patch ([9e0346c](https://github.com/openstreetlifting/openstreetlifting/commit/9e0346c01910fcacf535797fc6179d26964bd30b))
* **frontend:** broken import ([b2b727d](https://github.com/openstreetlifting/openstreetlifting/commit/b2b727d8b6f8997a41f8f920b03a62118c4c56ab))
* **frontend:** more forggoten import ([47331a3](https://github.com/openstreetlifting/openstreetlifting/commit/47331a33d6ee4050ecf4c8d0eb7c5e1b18d4c426))
* **frontend:** wrong import declaration ([54adfcb](https://github.com/openstreetlifting/openstreetlifting/commit/54adfcb074f4765ddca9f56758e7969b76c7ec9d))
* pull rebase before pushing to cd ([9b1634b](https://github.com/openstreetlifting/openstreetlifting/commit/9b1634bf0c11f6aded47a4714b3077b75bd13cee))
* reactive params ([640e073](https://github.com/openstreetlifting/openstreetlifting/commit/640e0737971c0369467b6c69140ec209d5b5ec35))
* target amd64 only ([2c2eca5](https://github.com/openstreetlifting/openstreetlifting/commit/2c2eca51532945d44d03f294d1ef0778655c1c3e))

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
