# VoidBot
An experimental Discord bot using Serenity.

# Features
- Emote Commands: `/say`, `/react`, `/emotes`, `/emote-registry`
- Stream Notification Embeds: `/stream`
- Timezone Info: `/time`
- Voice Channel Renaming: `/voice`
- Miscellaneous: `/config`

# Files
- `voidbot`: The executable file of the bot.
- `.env`: An optional file to set environment variables while running the bot. Used for read-only configuration data like tokens.
- `main.db`: A SQLite 3 database containing dynamic data. (Note: `test.db` only appears during a development instance.)

# Environment Variables
- `DISCORD_TOKEN`: The token for your bot. (*required*)
- `DISCORD_APPLICATION_ID`: The application ID for your bot. Automatically derived from your bot's token, but can be provided as a manual override in case your application ID doesn't match your bot's client ID. (*optional*)
- `DISCORD_BOT_OWNER`: The user ID of the bot's owner (*optional* (**required in dev mode**))

## Debug Environment Variables
- `DEV_GUILD`: The guild to test your slash commands on in dev mode. (*required*)
- `DEV_CLEAR`: A list of comma-separated guilds to clear slash commands from. (*optional*)
