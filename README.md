# Cheats

[![Version][version_badge]][crate_url]
[![License][license_badge]](LICENSE.txt)
[![Total Downloads][total_downloads_badge]][crate_url]
[![Recent Downloads][recent_downloads_badge]][crate_url]
[![Build on Master][master_build_badge]][actions_url]
[![Build on Development][development_build_badge]][actions_url]

[version_badge]: https://img.shields.io/crates/v/cheats?label=version&style=flat-square&logo=rust
[license_badge]: https://img.shields.io/crates/l/cheats?label=license&style=flat-square
[total_downloads_badge]: https://img.shields.io/crates/d/cheats?label=downloads%20%28total%29&style=flat-square
[recent_downloads_badge]: https://img.shields.io/crates/dr/cheats?label=downloads%20%28recent%29&style=flat-square
[master_build_badge]: https://img.shields.io/github/workflow/status/erayerdin/cheats/CI/master?label=build%20%28master%29&logo=github&style=flat-square
[development_build_badge]: https://img.shields.io/github/workflow/status/erayerdin/cheats/CI/development?label=build%20%28development%29&logo=github&style=flat-square

[crate_url]: https://crates.io/crates/cheats
[actions_url]: https://github.com/erayerdin/cheats/actions

cheats is a shell backend for games. Basically, it helps you 
invoke code with a provided string line.

The library is not yet production-ready. It has a very 
simple implementation of developer console and might lack 
some features you might desire.

# Inspiration

There are many conventions about how to cheat. Grand Theft 
Auto series receive sequential keypresses and invokes 
functionality. Age of Empires II has a simple
textbox to invoke a cheat but its cheats do not have any 
arguments.

In this library, cheats, the developer console of Valve 
games such as Half-Life,
Counter-Strike, Portal, Left 4 Dead, has been an inspiration 
and it is implemented in such a way.

    <COMMAND>
    <COMMAND> <ARGS>

    cl_hello
    cl_hello Eray

# Documentation

See [the documentation](https://docs.rs/cheats) to learn how
to use this library.