# Exit on Error

[![Crates.io](https://img.shields.io/crates/v/eoe.svg)](https://crates.io/crates/eoe)
[![Documentation](https://docs.rs/eoe/badge.svg)](https://docs.rs/eoe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This crate helps you exit on error with underlying `anyhow` error handling.

## Install

```
cargo add eoe
```

## Examples

```rust
use anyhow::{anyhow, Context};
use eoe::ExitOnError;

Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
    .context("Doom-boom-ba-beh")
    .context("Doo-boo-boom-ba-beh-beh")
    .exit_on_error();
```

![](assets/01.png)

```rust
use anyhow::{anyhow, Context};
use eoe::QuitOnError;

Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
    .context("Dee-day-da, ee-day-da")
    .quit_on_error();
```

![](assets/02.png)
