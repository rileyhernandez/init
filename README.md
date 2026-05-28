# init

This is my personal project setup tool. I'm usually on a Fedora Silverblue host amchine and do almost everything inside [Distrobox](https://distrobox.it/) containers. 

I got tired of manually setting up my environment every time I make a start a new project, particularly with getting my LSPs and linting working in [Zed](https://zed.dev/) and remembering what to run inside a container and what to run on host.

## How I use it

I run `init`, and it handles all the tedious setup:
1.  **Sets up a `justfile`**: I use `just` as my main command runner. The generated file is configured to proxy everything through `distrobox-enter`, so I can run `just build` or `just test` from my host terminal and have it execute inside the right container.
2.  **Configures Zed**: It drops a `.zed/settings.json` into the project that wraps my LSPs (like `rust-analyzer` or `ruff`) in `distrobox-enter`. This way, Zed on my host system can talk to the language servers inside the container easily.

## Support

Right now, it works for:
- **Rust**: Initializes via `cargo new` and sets up `rust-analyzer`.
- **Python**: (Work in progress) I'm having some issues still with how the Zed config should be set up, and I need to decide on the `uv` setup I like still.

## Usage

Can be run interactively:
```bash
init
```

Or with these flags:
```bash
init -l rust -d my-container -p my-new-project
```

## Installation

```bash
cargo install --path .
```

## To-do list

- [x] Rust
- [ ] Python (almost there)
- [ ] Clean up the template logic
- [ ] Add `/.zed` and whatever else to `.gitignore` in the generated project
- [ ] Include my AI config files