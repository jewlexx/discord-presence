# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]


## [0.3.0] - 2018-12-06
### Changed
- Connection manager completely rewritten
- Allow cloning of clients


## [0.2.4] - 2018-12-04
### Changed
- No longer depends on `libc` for process id lookup


## [0.2.3] - 2018-04-08
### Added
- Connection manager with reconnection
- Method to clear the current Rich Presence state

### Changed
- Move rich presence code back into *models*
- Remove command payload and add generic one
- Timestamps are now 64 bit unsigned integers instead of 32 bit ([@Bond-009]) [6bbc9f8][c:6bbc9f8]


## [0.2.2] - 2018-04-03
### Changed
- Use a default socket connection for the current platform


## [0.2.1] - 2018-04-03
### Changed
- Move common connection methods into trait


## [0.2.0] - 2018-04-02
### Added
- Error type
- Windows support ([@Tenrys]) [620e9a6][c:620e9a6]

### Changed
- Convert OpCode with `try_from` instead of `try`
- Use Rust 1.25 style nested imports


## [0.1.5] - 2018-03-28
### Changed
- Opcode stored in Message is now an OpCode enum
- Rich Presence now lives in it's own submodule


## [0.1.4] - 2018-03-23
### Changed
- Opcodes are now represented as enum instead of integers


## [0.1.3] - 2018-03-23
### Added
- Contributing information

### Changed
- Use `libc::getpid` to allow builds with *stable* instead of *nightly*
- Make client struct fields private
- Make models private again and add prelude
- Connections are now using a shared Connection trait


## [0.1.2] - 2018-03-22
### Added
- Logging support


## [0.1.1] - 2018-03-22
### Changed
- Make models publicly accessible


## [0.1.0] - 2018-03-22
### Added
- Setting Rich Presence status
- Unix socket connection support


<!-- links -->

[Unreleased]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/develop
[0.2.4]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.2.4
[0.2.3]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.2.3
[0.2.2]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.2.2
[0.2.1]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.2.1
[0.2.0]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.2.0
[0.1.5]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.5
[0.1.4]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.4
[0.1.3]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.3
[0.1.2]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.2
[0.1.1]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.1
[0.1.0]: https://gitlab.com/valeth/discord-rpc-client.rs/tree/v0.1.0

[c:620e9a6]: https://github.com/valeth/discord-rpc-client.rs/commit/620e9a6b26650d825392cf0fbfd097a7ed1662aa
[c:6bbc9f8]: https://github.com/valeth/discord-rpc-client.rs/commit/6bbc9f85d77bc6792c36d9317e804fcf5a306fb2

[@Tenrys]: https://github.com/Tenrys
[@Bond-009]: https://github.com/Bond-009
