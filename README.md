# VoidBot
An experimental Discord bot using Serenity.

# Files
- `voidbot`: The executable file of the bot.
- `.env`: An optional file to set environment variables while running the bot. Used for read-only configuration data like tokens.
- `main.db`: A SQLite 3 database containing dynamic data. (Note: `test.db` only appears during a development instance.)

# Environment Variables
- `DISCORD_TOKEN`: The token for your bot. (*required*)
- `APPLICATION_ID`: The client ID for your bot. (*required*)

## Debug Environment Variables
- `DEV_GUILD`: The guild to test your slash commands on in dev mode. (*required*)
