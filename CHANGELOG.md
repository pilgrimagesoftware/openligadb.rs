
## [1.0.0] - 2026-07-21


### Added

- Replace Box<dyn Error> with typed OpenLigaError enum **[BREAKING]**


### Changed

- Rename league param to league_id in team match lookups

- Use util::get/util::list instead of inline reqwest


### Documentation

- Regenerate CHANGELOG.md from full tag history via git-cliff

- Use idiomatic module doc comment


### Fixed

- Add explicit permissions to CI/release caller workflows

- Pass Url directly, drop redundant 'static bound, tighten visibility

- Remove redundant serde rename attributes


## [Unreleased]

## [0.0.13] - 2026-07-20

### Added

- Gate reqwest/async-trait/url behind an optional http-client feature **[BREAKING]**

### Documentation

- Propose gate-wasm-networking change

### Fixed

- Update workflows

## [0.0.12] - 2026-05-10

### Documentation

- Add specs from robot assessment

### Fixed

- Update package dependencies and configuration

## [0.0.9] - 2026-05-10

### Fixed

- License reference

- Comment out test that requires current league

## [0.0.7] - 2025-02-12
