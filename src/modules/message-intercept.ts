import { Message } from "discord.js";
import { client } from "..";

export async function handlerMessageIntercept(message: Message<boolean>) {
	// Prevent looping by the bot's own messages
	if (message.author.id === client.user?.id) {
		return;
	}

	const text = message.content;
}
