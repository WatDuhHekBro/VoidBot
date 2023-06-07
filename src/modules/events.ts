import { Events } from "discord.js";
import { client } from "..";

client.on(Events.ClientReady, () => {
	console.log(`Logged in as ${client.user?.tag}.`);
});

// It is critical to set a handler for this event, or any error will exit the program
// The error log should be viewable via the "screen" utility
client.on(Events.Error, console.error);
