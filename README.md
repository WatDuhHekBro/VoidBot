# VoidBot

An experimental Discord bot using Serenity. Streamlines the process of ways to use any emotes the bot has access to. Commands: `/say`, `/react`, `/emotes`

# Files

- `voidbot`: The executable file of the bot.
- `.env`: An optional file to set environment variables while running the bot. Used for read-only configuration data like tokens.

# Environment Variables

- `DISCORD_TOKEN`: The token for your bot. (_required_)
- `DISCORD_APPLICATION_ID`: The application ID for your bot. Automatically derived from your bot's token, but can be provided as a manual override in case your application ID doesn't match your bot's client ID. (_optional_)
- `DEV_GUILD`: The guild to test your slash commands on in dev mode. (_required_)
- `DEV_CLEAR`: A list of comma-separated guilds to clear slash commands from. Set to `*` to clear global slash commands. (_optional_)
