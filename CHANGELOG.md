# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.4](https://github.com/SichangHe/fmtm/compare/v0.0.3...v0.0.4) - 2026-04-08

### Fixed

- *(ci)* don't use local patch
- *(ci)* bump sccache
- *(test)* patch markdown-fmt source and cover link-separator panic

### Other

- bump fmtm_ytmimi_markdown_fmt to v0.0.4
- add yaml-header HTML spacing snapshots
- increased whitespace after yaml header before HTML
- add panic regression snapshots for link before separator
- test of panicking link
- bump versions;fix cache
- format w/ new fmtm

## [0.0.3](https://github.com/SichangHe/fmtm/compare/v0.0.2...v0.0.3) - 2024-09-03

### Added
- [**breaking**] update to fmtt 0.8.0

### Fixed
- prevent checkboxes starts from being split off to their own lines

### Other
- fix incorrect mention of FMTT
- further describe

## [0.0.2](https://github.com/SichangHe/fmtm/compare/v0.0.1...v0.0.2) - 2024-05-29

### Added
- *(display math)* bump markdown-fmt;math test
- handle hard breaks

## [0.0.1](https://github.com/SichangHe/fmtm/compare/v0.0.0...v0.0.1) - 2024-05-28

### Fixed
- expose ways to set `None` arguments
- fix CI

### Other
- do git dependency instead of workspace&path
- depend on `markdown-fmt` fork;attempt to re-trigger release
- release
- good enough README
- emphasis&strong marker options
