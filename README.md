# ![](src/extract/static/favicon.png) mhrice

## What?

To build http://mhrice.info, and to reverse engineer the game.

## Can I use the code and/or the output in my project?

Yes, you can. I open-sourced this because I want to share it.

Although I don't enforce anything beyond the license conditions, I kindly ask you to give attribution to the mhrice project if you use the code or the output from it in another places. Thank you!

## Platform and dependency

 - Actively tested on linux. Might work on Windows or macOS. (If not please open an issue)
 - Rust, cargo
 - OpenGL 3.3 if running any model-related command
 - C++ for a BC7 decoding library. RIIR to remove this dependency is still WIP

*Note: this repo contain git submodule. Either clone with `git clone --recursive` or do `git submodule update --init --recursive` after cloning.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
