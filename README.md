<p align="center">
    <a href="https://github.com/quaoz/sigma_bot/"><img alt="Build Status" src="https://img.shields.io/github/workflow/status/quaoz/sigma_bot/Lints?style=for-the-badge"></a>
    <a href="https://github.com/quaoz/sigma_bot/"><img alt="Code Size" src="https://img.shields.io/github/languages/code-size/quaoz/sigma_bot?style=for-the-badge"></a>
    <a href="https://github.com/quaoz/sigma_bot/"><img alt="Lines of Code" src="https://img.shields.io/tokei/lines/github/quaoz/sigma_bot?style=for-the-badge"></a>
    <a href="https://github.com/quaoz/sigma_bot/"><img alt="License" src="https://img.shields.io/github/license/quaoz/sigma_bot?style=for-the-badge"></a>
    <a href="https://github.com/quaoz/sigma_bot/"><img alt="Last commit" src="https://img.shields.io/github/last-commit/quaoz/sigma_bot?style=for-the-badge"></a>
</p>

# Sigma bot

A discord bot written in rust with [serenity](https://github.com/serenity-rs/serenity)

## Setup

- Requires [jq](https://stedolan.github.io/jq/download/) to be installed to update Lavalink
- Make a copy of `example.env` and rename it to `.env`

```bash
git clone https://github.com/quaoz/sigma_bot.git
cd sigma_bot
# Downloads and starts the latest Lavalink
./LavaLink.sh -u
# Compiles and runs the bot
./Bot.sh -s
```