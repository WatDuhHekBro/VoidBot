import { Events } from "discord.js";
import { client } from "..";
import { emoteRegistry } from "./emote-registry";

// It is critical to set a handler for this event, or any error will exit the program
// The error log should be viewable via the "screen" utility
client.on(Events.Error, console.error);

client.on(Events.ClientReady, () => {
	console.ready(`Logged in as ${client.user?.tag}.`);
	emoteRegistry.update();
});

// Although I could optimize these to make granular edits, I'm taking the lazy route
// Reason being is that making a sorted cache will probably avoid a lot of unnecessary computations already
client.on(Events.GuildEmojiCreate, emoteRegistry.update);
client.on(Events.GuildEmojiUpdate, emoteRegistry.update);
client.on(Events.GuildEmojiDelete, emoteRegistry.update);
