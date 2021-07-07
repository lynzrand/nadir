# Nadir

Nadir is a TUI tool for receiving and showing notifications. This tool is mostly intended for personal use with a wide secondary screen on a separate machine, but you can use its interfaces for any use you like.

## Components

### `nadir-notify`

(Root directory)

The notification displayer frontend. Exports interfaces in websocket ~~and unix domain socket~~, and allow other apps to connect and display notifications.

### Backends

(`/backends/*`)

Provides notification adapter for various programs. Resides in `backends/` folder and has crate names like `nadir-{}-backend`.

| Status | Backend    | Description                    |
| ------ | ---------- | ------------------------------ |
| WIP    | `maildir`  | Adapter for mail directories.  |
| WIP    | `telegram` | Adapter for telegram messages. |

### Other crates

(`crates/*`)

These crates are abstractions or reusable parts of various components in this project.

| Status | Folder                 | Description                        |
| ------ | ---------------------- | ---------------------------------- |
| WIP    | `nadir-backend-common` | Common parts for backend adaptors  |
| OK     | `nadir-types           | Message and model type definitions |

## Protocol

Nadir uses a simple JSON Websocket protocol to pass messages between frontend and backend. See [`docs/protocol.md`](docs/protocol.md) for more information.

## License

Nadir is licensed under the MIT license.

Copyright (c) 2021 Rynco Maekawa.
