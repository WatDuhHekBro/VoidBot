import "dotenv/config";
import { initFileLogger } from "./modules/globals";
import { Client, GatewayIntentBits } from "discord.js";

// First declare accessible constants
export const client = new Client({
	intents: [
		// According to the Discord.js Guide: "The "Guilds" intents option is necessary for the discord.js client to work as you expect it to, as it ensures that the caches for guilds, channels, and roles are populated and available for internal use."
		// Attempting to react to a message won't work without it since there are no emotes in the cache
		GatewayIntentBits.Guilds,
		// This is needed in order to have the cache update whenever someone adds/edits/deletes an emote
		GatewayIntentBits.GuildEmojisAndStickers,
	],
});

// Then import other modules that might rely on those constants
import { registerCommands } from "./modules/register";
import "./modules/events";

// Don't proceed any further if the token isn't defined
const token = process.env.DISCORD_TOKEN;

if (token) {
	// Then decide what to launch depending on the command line argument
	const choice: string | undefined = process.argv[2];

	// Start the bot normally
	if (choice === undefined) {
		// Check if suppress logs is set to any string, otherwise enable logs
		if (!process.env.SUPPRESS_LOGS) {
			initFileLogger();
		}

		client.login(token).catch(console.error);
	}
	// Register command definitions
	else if (choice === "register") {
		registerCommands(token, false);
	}
	// Clear command definitions
	else if (choice === "clear") {
		registerCommands(token, true);
	}
	// Unexpected choice
	else {
		console.error(`The provided argument (${choice}) isn't valid.`);
	}
} else {
	console.error(
		'No token was provided as the environment variable "DISCORD_TOKEN" either through the system or .env.'
	);
}
