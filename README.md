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

<details>
<summary> Show output </summary>
<div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
    <span style="color: #f14c4c; font-weight: bold">error</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Doo-boo-boom-ba-beh-beh</span><br>
    <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Doom-boom-ba-beh</span><br>
    <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Mm-noom-ba-deh</span><br>
</div>
</details>

Or if you prefer the word *quit*:

```rust
use anyhow::{Context, anyhow};
use eoe::QuitOnError;

Err::<(), _>(anyhow!("Mm-ba-ba-beh, mm-ba-ba-beh"))
    .context("Dee-day-da, ee-day-da")
    .quit_on_error();
```

<details>
<summary> Show output </summary>
<div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
    <span style="color: #f14c4c; font-weight: bold">error</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Dee-day-da, ee-day-da</span><br>
    <span style="color: #f14c4c; font-weight: bold">caused by</span><span style="color: #f14c4c; font-weight: bold">: </span><span style="color: #cccccc">Mm-ba-ba-beh, mm-ba-ba-beh</span><br>
</div>
</details>

Messages are customizable:

```rust
use eoe::{ExitOnError, Segment};
use owo_colors::Style;

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

<details>
<summary> Show output </summary>
<div style="background-color: #1e1e1e; font-family: monospace; padding: 10px; border-radius: 5px;">
    <span style="color: #3b8eea; font-weight: bold">Watchin' some good friends screamin'</span><span> 😱 </span><span style="color: #e5e510; font-style: italic">Let me out</span><br>
</div>
</details>
