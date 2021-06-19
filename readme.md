# Nadir

Nadir is a TUI tool for receiving and showing notifications. This tool is mostly intended for personal use with a wide secondary screen on a separate machine, but you can use its interfaces for any use you like.

## Components

### nadir-notify

The notification displayer frontend. Exports interfaces in websocket and unix domain socket, and allow other apps to connect and display notifications.

## Protocol

A simple JSON Websocket protocol. See [docs/protocol.md](docs/protocol.md).

## License

Nadir is licensed under the MIT license.

Copyright (c) 2021 Rynco Maekawa.
