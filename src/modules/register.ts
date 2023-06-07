import { REST, Routes } from "discord.js";
import { commands } from "../commands";

export async function registerCommands(token: string, clear: boolean) {
	const tokens = token.split(".");

	if (tokens.length !== 3) {
		return console.error(
			"Failed to register commands, the provided token was invalid."
		);
	}

	const clientID = Buffer.from(tokens[0], "base64").toString();
	const rest = new REST().setToken(token);
	const guildID: string | undefined = process.argv[3];

	if (guildID) {
		console.log(
			`${
				clear ? "Clearing" : "Registering"
			} guild commands for ${guildID}...`
		);

		await rest.put(Routes.applicationGuildCommands(clientID, guildID), {
			body: clear ? [] : commands,
		});

		console.log(
			`Successfully ${
				clear ? "cleared" : "registered"
			} guild commands for ${guildID}.`
		);
	} else {
		console.log(`${clear ? "Clearing" : "Registering"} global commands...`);

		await rest.put(Routes.applicationCommands(clientID), {
			body: clear ? [] : commands,
		});

		console.log(
			`Successfully ${clear ? "cleared" : "registered"} global commands.`
		);
	}
}
