# rustyban

Command line Kanban board

## Usage

To start the app using Cargo:

```sh
cargo run [-- path/to/file]
```

Without an argument, it will create a new empty board. With an argument it will open said file if it matches the expected structure.

Inside the app, use `<?>` to show the help and `<q>` to quit the application.
Use `<h/j/k/l>` or the arrow keys to select a card.

## Roadmap

Use **Rustyban** to see the roadmap:

```sh
cargo run -- res/roadmap.json
```
