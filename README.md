# VoidBot

An experimental Discord bot. Contains a proof of concept for streamlining the process of using any emotes the bot has access to, without having to be in the same servers as the bot.

## Setup Instructions

- Clone the repo
- `npm i`
- `npm run build` (runs the TypeScript compiler)
- `npm run pkg` (packages the codebase into an executable for easy deployment)
- Create an `.env` file containing `DISCORD_TOKEN` or set an environment variable
- `./voidbot register` (global registration)
- `./voidbot` (start running the bot)

## Dev Checklist

1. `npm start` (watches for changes in the codebase) or `npm run once` (builds and runs the program, optionally with command line arguments (i.e. `npm run once register 12345`))
2. Make changes
3. `npm test` (executes unit tests via Jest)
4. `npm run fmt` (formats the code via Prettier)
5. Update the `version` entry in `package.json` and do `npm i` to update `package-lock.json`
6. Update the readme and the changelog

# Files

- `voidbot`: The executable file of the bot. Possible parameters include:
	- `register (<guild>)`: Register command definitions, either globally or on a specific guild
	- `clean (<guild>)` Removes all existing slash commands, either globally or a specific guild
- `.env`: An optional file to set environment variables while running the bot. Used for read-only configuration data like tokens.
- `logs/yyyy-mm-dd_hh-mm-ss.log`: Log files

# Environment Variables

- `DISCORD_TOKEN`: The token for your bot. (_required_)

# Commands

- `/say`: Send a message with emotes the bot has access to
- `/list-emotes`: Either an alphabetical list or regex or distance-based filters

## Apps (Message)

- Create a react prompt to that specific message
- Edit or delete a proxy message via `/say`

# To-Do

- A refactor is absolutely necessary given the kludge of figuring things out
- Dev mode to suppress logs? Also restore `DEV_GUILD` for test bot, replaces functionality of `./voidbot register/clear`
- /list-emotes backup (sends JSON file for backup purposes)
- Create user-defined emote aliases? Users can also block out emotes they choose
- User Menu: Info (for the bot, it'll prompt you to choose between a user guide or info about the bot, the code, maybe the dev server, etc. (can define in .env maybe?))
- Matching strategy: Containing exact substrings (i.e. `ost` --> `p5_ost_41`)
- Revise matching strategy: Unit tests will consist of testing ordered lists of possible emote names and how they should be ordered (developer-defined heuristics)
- Fix emote selector (`~#`) to match distance query
