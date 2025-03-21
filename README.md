# Exit on Error

[![Crates.io](https://img.shields.io/crates/v/eoe.svg)](https://crates.io/crates/eoe)
[![Documentation](https://docs.rs/eoe/badge.svg)](https://docs.rs/eoe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This crate provides utilities for exiting processes on errors gracefully, leveraging `anyhow` to display detailed error context and chained messages.

## Examples

Exiting on error:

```rust
use anyhow::{Context, anyhow};
use eoe::ExitOnError;

Err::<(), _>(anyhow!("Mm-noom-ba-deh"))
    .context("Doom-boom-ba-beh")
    .context("Doo-boo-boom-ba-beh-beh")
    .exit_on_error();
```

![exit on error](assets/exit_on_error.png)

Or if you prefer the word *quit*:

```rust
use anyhow::{Context, anyhow};
use eoe::QuitOnError;

Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
    .context("Dee-day-da, ee-day-da")
    .quit_on_error();
```

![quit on error](assets/quit_on_error.png)

Messages are customizable:

```rust
use eoe::{ExitOnError, Segment, Style};

let _ = eoe::ERROR.set(Segment {
    style: Style::new().bold().blue(),
    value: "Watchin' some good friends screamin'",
});
let _ = eoe::SEP.set(Segment {
    style: Style::new(),
    value: " 😱 ",
});
let _ = eoe::MESSAGE_STYLE.set(Style::new().italic().yellow());
let _ = eoe::MESSAGE_ON_NONE.set("Let me out");

None::<()>.exit_on_error();
```

![customized](assets/customized.png)
