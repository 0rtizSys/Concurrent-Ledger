# Changelog

## [0.0.4] - 2026-08-04

### Added
- Added an interactive terminal menu for account management.
- Added account creation from the CLI.
- Added a formatted account list view.
- Added terminal cleanup utilities through the new utils module.

### Changed
- Improved menu UX with clearer options, colors, pauses, and feedback messages.
- Replaced raw debug account output with a readable table.
- Allowed exiting the app with `0` in addition to `salir`.
- Made Ledger account insertion internal to the ledger module.

### Fixed
- Flushed the input prompt so it appears reliably before reading user input.
- Prevented empty account names from creating new accounts.

## [0.0.3] - 2026-08-03

### Added
- Aurum can now search accounts
- Added constructors for Ledger and Historial.

### Changes & refactor
- Ledger and Cuenta are now modularized

## [0.0.2] - 2026-08-03

### Added
- Added Ledger structure.
- Added account storage using Vec.
- Added account creation through Ledger.
- Added initial account management logic.

## [0.0.1] - 2026-08-02

### Added 
- Created initial Rust project.
- Added first Account structure.
- Uploaded to github, Horaay! 
