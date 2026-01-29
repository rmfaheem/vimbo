## vimbo

`vimbo` is a small terminal companion for Vim: a searchable cheatsheet you can keep open in a split or separate pane while you edit.

It is built with `ratatui` and `crossterm` and runs anywhere Rust does.

### Features

- **Searchable cheatsheet**: type to filter by command, category, or description (e.g. `copy`, `paste`, `window`, `macro`).
- **Curated Vim basics**: motions, insert/visual modes, search/replace, windows/tabs, buffers, registers, macros, and more.
- **Keyboard-friendly UI**: no mouse required; designed to sit next to your Vim session.

### Installation

You need a recent Rust toolchain and `cargo` installed.

```bash
cargo install --path .
```

This will install the `vimbo` binary into your Cargo bin directory (usually `~/.cargo/bin`), which should be on your `PATH`.

### Usage

From any terminal:

```bash
vimbo
```

You can also start with an initial query:

```bash
vimbo --query paste
```

Keep it open in a tmux / Vim split or another terminal window as a quick reference.

### Key bindings

- **Search**
  - **type**: append characters to the search query
  - **Backspace**: delete last character
  - **/**: clear the current query

- **Navigation**
  - **↑ / ↓**: move selection up/down
  - **PgUp / PgDn**: jump by a larger step
  - **g / G**: jump to top / bottom of the list

- **Misc**
  - **?**: toggle the help pane
  - **Esc**: quit `vimbo`

### Notes

`vimbo` is intentionally read-only and static: it does not attempt to interact with your Vim session, just to provide an always-available reference. Contributions of additional cheatsheet entries and refinements are welcome.

