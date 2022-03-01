# ![](src/extract/static/favicon.png) mhrice

:warning: This project mainly targets the PC version. It might or might not work for the Switch version

## What?

To build http://mhrice.info, and to reverse engineer the game.

## Can I use the code and/or the output in my project?

Yes, you can. I open-sourced this because I want to share it.

Although I don't enforce anything beyond the license conditions, I kindly ask you to give attribution to the mhrice project if you use the code or the output from it in another places. Thank you!

## Obfuscation keys

Currently, running mhrice requires you to provide some keys to read data directly from pak.
You can provide them in base64 format via environment variables, or a file named `mhrice.config` in the current directory.

The file should look like this:
```
PAK_MAIN_KEY_MOD = fQv4wXwj...PPjUta+M=
PAK_SUB_KEY_EXP = wMJ3H1s0.../9CWTIAE=
PAK_SUB_KEY_MOD = E9eciYiR...Cr2XYdwM=
```

## Platform and dependency

 - Actively tested on linux and Windows. Might work on macOS. (If not please open an issue)
 - Rust, cargo
 - OpenGL 3.3 if running any model-related command

## Credits

 - praydog, for [REFramework](https://github.com/praydog/REFramework)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
