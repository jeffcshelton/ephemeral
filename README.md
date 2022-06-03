# ephemeral

`ephemeral` is a command line tool written in Rust that securely generates one or more random passwords to your specification. `evaporate` can be used if you don't trust closed-source mainstream password managers/generators to provide secure, random passwords or if you need a compact, terminal-friendly solution in a restricted environment.

## Install

`ephemeral` is not yet listed on major package repositories and so cannot yet be installed using `brew` or `apt`. As such, it must be compiled from source. This can be done by [installing Rust](https://www.rust-lang.org/tools/install) and then executing:

```
$ cargo install --git https://github.com/jeffreycshelton/ephemeral
```

Assuming ~/.cargo (or the equivalent on Windows) is on PATH, you should be able to execute `ephemeral`. If you ever want to uninstall `ephemeral`, just execute this command:

```
$ cargo uninstall ephemeral
```

## Usage

```
$ ephemeral gen [<count>] [--len <password-length>] [--no-caps] [--no-symbols] [--no-nums]
```

If not specified, `ephemeral` generates one password. By default, passwords are 16 characters long unless otherwise specified.

## License

`ephemeral` is licensed under the [MIT License](LICENSE).
