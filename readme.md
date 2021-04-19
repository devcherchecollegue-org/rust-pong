# Arcades

## Quick Start

Configure project: run `.bin/configure`  
This will install `cargo-cmd`, add a git hook for validation and add the `.env` file to the root of your project  
Now you can configure the `.env` file as you want

You can use `cargo cmd (build|run):(client|server|front|web)` if you have installed `cargo-cmd`  
Like: `cargo cmd run:client`

But if you prefer, you can still run the standard cargo command

> NB: you need to [install dependencies for bevy](https://bevyengine.org/learn/book/getting-started/setup/)

## Before each push:

- `cargo check` should not fail !!
- `cargo fmt` to format your source code.

> This wiil be run automatically before each commit if you have the pre-commit hook
