# DiscordBot
This is a discord bot for CS:GO KZ using [serenity.rs](https://github.com/serenity-rs/serenity)
and [poise](https://github.com/serenity-rs/poise).

It talks to different APIs and its own Database to provide useful functionality that you would
otherwise only get ingame or from a website like [KZ:GO](https://kzgo.eu). This functionality mostly
revolves around `/` commands, including:
- `/pb`
- `/wr`
- `/maptop`
- `/recent`
- `/profile`

and many more! For a full list check out the [Wiki](https://github.com/Schnose/DiscordBot/wiki).
I am running a public instance that you can invite to your server via [this link](https://discord.com/oauth2/authorize?client_id=940308056451973120&permissions=327744&scope=bot%20applications.commands).

If you wish to run your own instance, read the following section.

## Setup

If you want to run your own instance of the bot, you can follow these steps:

1. Install dependencies:
  - [rustup](https://rustup.rs/)
  - [docker-compose](https://github.com/docker/compose)
  - [just](https://github.com/casey/just)

2. Clone this repo

```sh
git clone https://github.com/Schnose/DiscordBot.git
```

4. Create an account at https://www.shuttle.rs

5. Copy the `Secrets.example.toml` to `Secrets.dev.toml` (for running locally) and to `Secrets.toml`
   (for deploying) and modify the values according to your needs.

6. Run the project locally:

```sh
just dev
```
