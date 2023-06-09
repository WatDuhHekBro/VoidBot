# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.3 - 2023-06-09

### Added

- Environment variable to suppress logs
- Environment variable to make `./voidbot register/clear` hassle-free in dev environments

### Fixed

- Unhelpful error messages in logs, attempts to log the entire call stack first

## 1.0.2 - 2023-06-08

### Fixed

- `/list-emotes query use-columns: true` loose equals for direct matches, where `distance = 0`

## 1.0.1 - 2023-06-08

### Added

- `/list-emotes` option to toggle a table for emote embeds that looks nice on desktop

### Changed

- `/list-emotes` now uses the mobile-friendly format by default (via the embed description)

## 1.0.0 - 2023-06-07

### Added

- `/say` and context menu modifiers
- `/list-emotes`
- React to message context menu action
- Old emote utils and library functions
