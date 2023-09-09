# Bootstrapping Nix and Linux from TinyCC

* [Talk info](https://talks.nixcon.org/nixcon-2023/talk/DDQRWQ/)
* [Recording](https://media.ccc.de/v/nixcon-2023-34402-bootstrapping-nix-and-linux-from-tinycc)

Building every operating system starts from something. I'm not as hardcore as the stage0 project folks going from assembly to Guix, but a shorter path from a ~320 KB TinyCC binary to Nix to a proof-of-concept Linux distribution also proved to be a story worth telling.

We'll begin our journey by executing a -nostdlib C program with TinyCC, compile our way out towards a libc and a shell, work through older compilers to modern compilers and build Nix. Then we'll walk the same path again once over, but this time, we'll use Nix. Finally, in a different flake, we'll reimplement some of the founding nixpkgs patterns, work our way from toolchain to Linux and arrive at a bootable .iso. Corners will be cut, hacks will run amock, Nix will be both praised and complained about, reproducibility will be earned hard, and some patches will get upstreamed.

Links: https://github.com/ZilchOS/bootstrap-from-tcc, https://github.com/ZilchOS/core, http://bootstrappable.org
