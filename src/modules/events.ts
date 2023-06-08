import { Events } from "discord.js";
import { client } from "..";
import { emoteRegistry } from "./emote-registry";
import { version } from "../version";

// It is critical to set a handler for this event, or any error will exit the program
// The error log should be viewable via the "screen" utility
client.on(Events.Error, console.error);

client.on(Events.ClientReady, () => {
	// Reference: https://docs.npmjs.com/cli/v9/using-npm/scripts#packagejson-vars
	// Unfortunately, this doesn't work when using vercel/pkg, so genversion is included in the build scripts as a workaround
	// https://github.com/nodejs/help/issues/2354
	console.ready(
		`Logged in as ${client.user?.tag} running VoidBot v${version}.`
	);
	console.ready(
		`Ready to serve ${client.users.cache.size} users in ${client.guilds.cache.size} servers.`
	);
	emoteRegistry.update();
});

// Although I could optimize these to make granular edits, I'm taking the lazy route
// Reason being is that making a sorted cache will probably avoid a lot of unnecessary computations already
// Note: Make sure not to send in "emoteRegistry.update" because it loses "this" binding
client.on(Events.GuildEmojiCreate, (emote) => {
	emoteRegistry.update();
});
client.on(Events.GuildEmojiUpdate, (emote) => {
	emoteRegistry.update();
});
client.on(Events.GuildEmojiDelete, (emote) => {
	emoteRegistry.update();
});
client.on(Events.GuildCreate, (emote) => {
	emoteRegistry.update();
});
client.on(Events.GuildDelete, (emote) => {
	emoteRegistry.update();
});
