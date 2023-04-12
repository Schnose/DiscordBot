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

1. Install [rustup](https://rustup.rs/). If you are on Linux or MacOS you will get an installation
   script to run in your shell. If you are on Windows you will download a `.exe`.
2. Clone this repo

```sh
git clone https://github.com/Schnose/DiscordBot.git
```

3. Create a PostgreSQL Database and run the `./migrations/schemas_up.sql` against it to initialize
   the relevant tables.
4. Create an account at https://www.shuttle.rs/
5. Copy the `Secrets.example.toml` to `Secrets.dev.toml` (for running locally) and to `Secrets.toml`
   (for deploying).
6. Modify the values according to your needs.
7. Run `./run.sh` to run the bot locally or `./deploy.sh` to deploy the bot to shuttle.
