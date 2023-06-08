# VoidBot

An experimental Discord bot. Contains a proof of concept for streamlining the process of using any emotes the bot has access to, without having to be in the same servers as the bot.

## Setup Instructions

- Clone the repo
- `npm i`
- `npm run build`
- `npm run pkg`
- Create an `.env` file containing `DISCORD_TOKEN` or set an environment variable
- `./voidbot register` (global registration)
- `./voidbot` (start running the bot)

## Package Commands

- `npm start`: Watches for changes in the codebase
- `npm run build`: Runs the TypeScript compiler
- `npm run pkg`: Packages the codebase into an executable for easy deployment
- `npm test`: Executes unit tests via Jest
- `npm run fmt`: Formats the code via Prettier
- `npm run once`: Builds and runs the program, optionally with command line arguments (i.e. `npm run once register 12345`)

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
