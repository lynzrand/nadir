# Nadir

Nadir is a TUI tool for receiving and showing notifications. This tool is mostly intended for personal use with a wide secondary screen on a separate machine, but you can use its interfaces for any use you like.

## Components

### `nadir-notify`

The notification displayer frontend. Exports interfaces in websocket ~~and unix domain socket~~, and allow other apps to connect and display notifications.

### Backends

Provides notification adapter for various programs. Resides in `backends/` folder and has crate names like `nadir-{}-backend`.

| Status | Backend    | Description                    |
| ------ | ---------- | ------------------------------ |
| WIP    | `maildir`  | Adapter for mail directories.  |
| WIP    | `telegram` | Adapter for telegram messages. |

## Protocol

A simple JSON Websocket protocol. See [docs/protocol.md](docs/protocol.md).

## License

Nadir is licensed under the MIT license.

Copyright (c) 2021 Rynco Maekawa.
