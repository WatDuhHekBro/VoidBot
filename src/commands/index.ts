import {
	ApplicationCommandType,
	Events,
	ContextMenuCommandBuilder,
	SlashCommandBuilder,
} from "discord.js";
import { client } from "..";

// Command Names
export const CMD_LSEMOTES = "list-emotes";
export const CMD_REACT = "react";
export const CMD_SAY = "say";
export const MENU_MSG_REACT = "React with Emotes";
export const MENU_MSG_SAY_EDIT = "Edit Proxy Message";
export const MENU_MSG_SAY_DELETE = "Delete Proxy Message";
export const MENU_MSG_POKEMON = "Tackle";

// Command Definitions
export const commands = [
	// list-emotes (<regex: string>) (<is-case-sensitive: boolean>)
	new SlashCommandBuilder()
		.setName(CMD_LSEMOTES)
		.setDescription(
			"Lists out all the emotes the bot currently has access to"
		)
		.addStringOption((option) =>
			option
				.setName("regex")
				.setDescription("The regex pattern to filter emotes by")
		)
		.addBooleanOption((option) =>
			option
				.setName("is-case-sensitive")
				.setDescription(
					"Whether or not to check the pattern for case-sensitivity (false by default)"
				)
		)
		.toJSON(),
	// react <emotes: string> (<target: string>)
	new SlashCommandBuilder()
		.setName(CMD_REACT)
		.setDescription(
			"Reacts to the targeted message with any emotes the bot currently has access to"
		)
		.addStringOption((option) =>
			option
				.setName("emotes")
				.setDescription(
					"The list of space-separated emote names to react with"
				)
				.setRequired(true)
		)
		.addStringOption((option) =>
			option
				.setName("target")
				.setDescription(
					"The message to target (distance / message ID / channel-message ID pair / message link)"
				)
		)
		.toJSON(),
	// say
	new SlashCommandBuilder()
		.setName(CMD_SAY)
		.setDescription(
			"Sends a proxy message in your place with any emote you specify in /slashes/"
		)
		.toJSON(),
	// React to Message - Replaces the function of using ".react" via a message reply
	new ContextMenuCommandBuilder()
		.setName(MENU_MSG_REACT)
		.setType(ApplicationCommandType.Message)
		.toJSON(),
	// Edit Proxy Message
	new ContextMenuCommandBuilder()
		.setName(MENU_MSG_SAY_EDIT)
		.setType(ApplicationCommandType.Message)
		.toJSON(),
	// Delete Proxy Message
	new ContextMenuCommandBuilder()
		.setName(MENU_MSG_SAY_DELETE)
		.setType(ApplicationCommandType.Message)
		.toJSON(),
	// Pokemon
	new ContextMenuCommandBuilder()
		.setName(MENU_MSG_POKEMON)
		.setType(ApplicationCommandType.Message)
		.toJSON(),
];

// Renamed Handler Imports
import { execute as executeLsemotes } from "./list-emotes";
import {
	execute as executeReact,
	executeMenu as executeMenuReact,
} from "./react";
import {
	execute as executeSay,
	executeMenuEdit,
	executeMenuDelete,
} from "./say";

// Command Router
client.on(Events.InteractionCreate, async (interaction) => {
	// Maybe in the future, edit interactions with buttons to be disabled if it's out of scope?
	// Or an ephemeral message that lets the user know that the buttons they tried to use aren't monitored.
	if (interaction.isChatInputCommand()) {
		// Slash Commands
		switch (interaction.commandName) {
			case CMD_LSEMOTES:
				await executeLsemotes(interaction);
				break;
			case CMD_REACT:
				await executeReact(interaction);
				break;
			case CMD_SAY:
				await executeSay(interaction);
				break;
			default:
				await interaction.reply({
					content:
						"**Error:** Invalid command name! This probably means that the command definitions haven't been updated yet or there's a glaring oversight in the code.",
					ephemeral: true,
				});
		}
	} else if (interaction.isMessageContextMenuCommand()) {
		switch (interaction.commandName) {
			case MENU_MSG_REACT:
				await executeMenuReact(interaction);
				break;
			case MENU_MSG_SAY_EDIT:
				await executeMenuEdit(interaction);
				break;
			case MENU_MSG_SAY_DELETE:
				await executeMenuDelete(interaction);
				break;
			case MENU_MSG_POKEMON:
				await interaction.reply("It's super effective!");
				break;
			default:
				await interaction.reply({
					content:
						"**Error:** Invalid menu command! This probably means that the command definitions haven't been updated yet or there's a glaring oversight in the code.",
					ephemeral: true,
				});
		}
	}
});
