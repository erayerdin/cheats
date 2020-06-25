# Cheats

[![Version][version_badge]][crate_url]
[![License][license_badge]](LICENSE.txt)
[![Total Downloads][total_downloads_badge]][crate_url]
[![Recent Downloads][recent_downloads_badge]][crate_url]

| | Build | Coverage
|-|-|-|
| **master** | [![Build on Master][master_build_badge]][actions_url] | [![Coverage on Master][master_coverage_badge]][codecov_url] |
| **development** | [![Build on Development][development_build_badge]][actions_url] | [![Coverage on Development][development_coverage_badge]][codecov_url] |

[version_badge]: https://img.shields.io/crates/v/cheats?label=version&style=flat-square&logo=rust
[license_badge]: https://img.shields.io/crates/l/cheats?label=license&style=flat-square
[total_downloads_badge]: https://img.shields.io/crates/d/cheats?label=downloads%20%28total%29&style=flat-square
[recent_downloads_badge]: https://img.shields.io/crates/dr/cheats?label=downloads%20%28recent%29&style=flat-square
[master_build_badge]: https://img.shields.io/github/workflow/status/erayerdin/cheats/CI/master?logo=github&style=flat-square
[development_build_badge]: https://img.shields.io/github/workflow/status/erayerdin/cheats/CI/development?logo=github&style=flat-square
[master_coverage_badge]: https://img.shields.io/codecov/c/gh/erayerdin/cheats/master?style=flat-square
[development_coverage_badge]: https://img.shields.io/codecov/c/gh/erayerdin/cheats/development?style=flat-square

[crate_url]: https://crates.io/crates/cheats
[actions_url]: https://github.com/erayerdin/cheats/actions
[codecov_url]: https://codecov.io/gh/erayerdin/cheats

cheats is a shell backend for games. Basically, it helps you 
invoke functionality with a Valve-game-like shell grammar.

The library is not yet production-ready. It has a very 
simple implementation of developer console and might lack 
some features you might desire.

# Shell Grammar

There are many conventions about how to cheat. Grand Theft 
Auto series receive sequential keypresses and invokes 
functionality. Age of Empires II has a simple
textbox to invoke a cheat but its cheats do not accept any
arguments.

In this library, cheats, the developer console of Valve 
games such as Half-Life, Counter-Strike, Portal, Left 4 
Dead etc. has been an inspiration  and it is implemented in such a way. The grammar is similar to below.

    // this is a comment
    # this is a comment as well
    cl_hello // without args
    cl_hello Eray # with args

# Documentation

See [the documentation](https://docs.rs/cheats) to learn how
to use this library.