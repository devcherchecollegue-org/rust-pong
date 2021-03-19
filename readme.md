# Arcades

## Quick Start

Configure project: run `.configure/run`  
This will install `cargo-cmd`, add a git hook for validation and add the `.env` file to the root of your project  
Now you can configure the `.env` file as you want

You can use `cargo cmd (build|run):(client|server)` if you have installed `cargo-cmd`  
Like: `cargo cmd run:client`

But if you prefer, you can still run the standard cargo command

## Before each push:

- `cargo check` should not fail !!
- `cargo fmt` to format your source code.

> This wiil be run automatically before each commit if you have the pre-commit hook