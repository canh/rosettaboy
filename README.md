RosettaBoy
==========
Trying to implement a gameboy emulator in a bunch of languages for my own
amusement and education; also giving people an opportunity to compare the
same code written in different languages, similar to
[Rosetta Code](https://www.rosettacode.org) but with a non-trivial codebase :)

The main goals are:

- Readability of the code
- Consistency across langauges
- Idiomatic use of language features
- Basic playability

Notably, 100% accuracy is not a goal - if Tetris works perfectly then I'm
happy, if other games require more obscure hardware features, then I'll
weigh up whether or not the feature is worth the complexity.

Also yes, "consistent across languages" and "idiomatic" can be at odds -
there are subjective compromises to be made, but for the most part that
doesn't seem to be a huge problem. Rust uses `Result`, Python uses
`Exception`, Go uses `error` - but so far it's always been pretty obvious
that eg `NewCart()` in go and `Cart.new()` in rust are doing fundamentally
the same thing in the same way.

So far all the implementations follow a fairly standard layout, with each
module teaching me how to do a new thing. In fact they're all so similar,
I wrote one copy of the documentation for all the implementations:

- [main](docs/main.md): argument parsing
- [cpu](docs/cpu.md): CPU emulation
- [gpu](docs/gpu.md): graphical processing
- [apu](docs/apu.md): audio processing
- [buttons](docs/buttons.md): user input
- [cart](docs/cart.md): binary file I/O and parsing
- [clock](docs/clock.md): timing / sleeping
- [consts](docs/consts.md): lists of constant values
- [ram](docs/ram.md): array access where some array values are special

Pull requests to translate into new languages, or fleshing out existing
languages, are very welcome :)

Completeness
------------
| Feature                       | Python  | C++     | Rust    | Go      |
| -------                       | ------- | ---     | ----    | --      |
| *CPU*                         |         |         |         |         |
| gblargh's test suite          | &check; | &check; | &check; | &check; |
| interrupts                    | &check; | &check; | &check; | &check; |
| logging                       | &check; | &check; | &check; | &check; |
| *Graphics*                    |         |         |         |         |
| scaled output                 | &check; | &check; | &check; | &check; |
| scanline rendering            | &cross; | &check; | &check; | &check; |
| GPU interrupts                | &cross; | &check; | &check; | &check; |
| *Audio*                       |         |         |         |         |
| audio                         | &cross; | off-key | glitchy | &cross; |
| *Inputs*                      |         |         |         |         |
| keyboard input                | &check; | &check; | &check; | &check; |
| gamepad input                 | &cross; | &cross; | &check; | &cross; |
| turbo button                  | &cross; | &check; | &cross; | &check; |
| *Memory*                      |         |         |         |         |
| memory mapping                | &check; | &check; | &check; | &check; |
| bank swapping                 | &cross; | &cross; | &cross; | &cross; |

Benchmarks
----------
**Warning**: These implementations aren't fully in-sync, so take numbers with
a large grain of salt. For example: the Python implementation uses native code
to blit whole 8x8 sprites in one go, while the other languages do one pixel at
a time (which is more correct, and necessary for things like parallax effects),
which means that the python version is unfairly fast.

Measurements are FPS, done with `--turbo --profile 3600 opus5.gb` (measuring
how much physical time it takes to emulate 60 seconds of game time), on a
Macbook Pro 2019 (2.4 GHz 8-Core Intel Core i9). If somebody knows how to
measure CPU instructions instead of clock time, that seems fairer; especially
if we can get the measurement included automatically via github actions. Pull
requests welcome :)

| Feature                            | Python | C++  | Rust | Go  |
| -------                            | ------ | ---  | ---- | --  |
| release, silent, headless          |  5     | 700  | 700  | 60  |
