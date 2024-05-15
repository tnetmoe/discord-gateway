# Discord Gateway

Inbound and outbound messages to and from Discord's [ws gateway](./gateway/ws), [http (interaction) webhook](./gateway/http), [voice nodes](./gateway/voice), [api](./gateway/api) and [webhooks](./gateway/webhook).

## About
This project is intended to keep the logic for any long and short lived connections separate from the actual Discord bot. Just let the autoscaler do the magic without having to worry about keeping connections and rate limits.

In the best case, only logic for interacting with the broker is necessary and everything else, such as keeping the connection, handling sharding and distributing the load is handled automagically.

Additionally, it allows easy splitting of the worker into microservices that only handle specific events (with the help of AMQP's [topic exchanges](https://www.rabbitmq.com/tutorials/amqp-concepts#exchange-topic)) without having to worry about which programming language should be used (with the help of AMQP & [Protobuf](https://protobuf.dev)).

## Disclaimer
- It is NOT a way to bypass rate limits. Although we will try to handle it as gracefully as possible, the software is still vulnerable to Discord's & Cloudflare's shenanigans, which may impact uptime.
- The project will only support the latest Discord API version. Currently, this is directly tied to [Serenity](https://github.com/serenity-rs/serenity).
- While we do use Protobuf to future-proof the broker api, the event data sent by Discord can change.
- Development is currently directly tied to my own Discord bot, so no guarantees for any updates.
- No support. I may investigate and fix bugs or merge pull requests, but I won't offer any support other than maintaining the existing documentation (or by writing new docs).
- Linux first. While it may work on e.g. Windows when the stars align, the only supported operating system is Linux.

## Images
We provide a rootless, immutable [Distroless](https://github.com/GoogleContainerTools/distroless) (based on Debian) & [Alpine](https://www.alpinelinux.org) Docker image for each binary crate.

Additionally, an [example docker compose file](./docker-compose.yml) is provided with [best practice security settings](https://cheatsheetseries.owasp.org/cheatsheets/Docker_Security_Cheat_Sheet.html) (such as dropping [capabilities](https://man7.org/linux/man-pages/man7/capabilities.7.html)).

## License
[AGPL-3.0](./LICENSES/AGPL-3.0) \
SPDX-License-Identifier: AGPL-3.0-only \
The license applies to all parts of the project, including the source code, its documentation and supplementary files unless otherwise directly indicated the file.