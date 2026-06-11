# rsfetch

A small neofetch-style system information tool written in Rust. It prints host,
OS, kernel, shell, uptime, CPU, memory/swap, and disk usage next to an
auto-detected ASCII logo.

```
  /\_/\         Host  : my-pc
 ( o.o )  WSL   OS    : Fedora Linux 44
  > ^ <         Kernel: 6.6.114.1-microsoft-standard-WSL2
 /     \        Shell : /bin/bash
( Linux )       Uptime: 1h 55m
 \_____/        CPU   : 13th Gen Intel(R) Core(TM) i9-13900K (32 cores)
                Memory: 3.3 GiB / 31.2 GiB (10%)
```

## Build & run

```sh
cargo run                # build and run
cargo build --release    # optimized binary at target/release/rsfetch
cargo test               # run the test suite
```

## Usage

```
rsfetch [OPTIONS]

  --fields <LIST>     Comma-separated fields to show, in order
                      (host, os, kernel, shell, uptime, cpu, memory, swap, disks)
  --separator <STR>   String between each label and value (default ":")
  --all-disks         Show every mount instead of only the root filesystem
  --color <WHEN>      auto (default), always, or never
  --theme <NAME>      default or mono
  --logo <NAME>       auto (default), none, or arch/debian/fedora/ubuntu/wsl/generic
  --config <PATH>     Use a specific config file
  -h, --help          Print help
  -V, --version       Print version
```

Examples:

```sh
rsfetch --fields host,os,cpu --color always
rsfetch --logo arch --theme mono
rsfetch --all-disks
```

## Configuration

rsfetch reads `$XDG_CONFIG_HOME/rsfetch/config.toml` (falling back to
`~/.config/rsfetch/config.toml`) if present. Settings are layered, highest
precedence first: **command-line flags → config file → built-in defaults**.

See [`config.example.toml`](config.example.toml) for all options.
