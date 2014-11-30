![Travis-CI](https://travis-ci.org/gkbrk/RustOre-Classic.svg?branch=master)
#RustOre#
RustOre is a tiny Minecraft Classic server. It is a Work in Progress and still lacks a lot of functionality.

#Functionality#
* Can send heartbeats (Visible on the minecraft.net serverlist)
* Can authenticate players with Minecraft.net
* Spawns player on a tiny island in the middle of nowhere
* Can store the world temporarily between connections

#Planned Functionality#
* Block physics
* World saving/loading
* Multiplayer support
* Chat support
* World generation

#Compiling and Running#
To build RustOre, you need to get Cargo and Rust (you can use rustup.sh for this).

After getting Cargo and Rust, you can run

> cargo build

to build the executable in the _target_ folder or you can run

> cargo run

to build and run the executable in the same command.
