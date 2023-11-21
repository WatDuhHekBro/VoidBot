import {
	ApplicationCommandType,
	Events,
	ContextMenuCommandBuilder,
	SlashCommandBuilder,
} from "discord.js";
import { client } from "..";
import { MAX_ACCEPTED_DISTANCE } from "../modules/emote-registry";
import { SORT_BY_ALPHA_ASC, SORT_BY_ALPHA_DESC } from "./list-emotes";

// Command Names
export const CMD_LSEMOTES = "list-emotes";
export const CMD_LSEMOTES_ALL = "all";
export const CMD_LSEMOTES_QUERY = "query";
export const CMD_LSEMOTES_REGEX = "regex";
export const CMD_SAY = "say";
export const MENU_MSG_REACT = "[React]";
export const MENU_MSG_SAY_EDIT = "Proxy Message: Edit";
export const MENU_MSG_SAY_DELETE = "Proxy Message: Delete";
export const MENU_MSG_FILTER_LINKS = "[Filter Links]";
export const MENU_MSG_POKEMON = "Tackle";

// Command Definitions
export const commands = [
	// list-emotes all (<sort-by: string choices>) (<use-columns: boolean>)
	// list-emotes query <query: string> (<levenshtein-threshold>: number (>= 0)) (<use-columns: boolean>)
	// list-emotes regex <pattern: string> (<is-case-sensitive: boolean>) (<use-columns: boolean>)
	new SlashCommandBuilder()
		.setName(CMD_LSEMOTES)
		.setDescription(
			"Lists out all the emotes the bot currently has access to"
		)
		.addSubcommand((subcommand) =>
			subcommand
				.setName(CMD_LSEMOTES_ALL)
				.setDescription(
					"Displays a list of all available emotes without any filters"
				)
				.addStringOption((option) =>
					option
						.setName("sort-by")
						.setDescription(
							"Sorts the list by different metrics. (Default: Alphabetical (Ascending))"
						)
						.addChoices(
							{
								name: "Alphabetical (Ascending)",
								value: SORT_BY_ALPHA_ASC,
							},
							{
								name: "Alphabetical (Descending)",
								value: SORT_BY_ALPHA_DESC,
							}
						)
				)
				.addBooleanOption((option) =>
					option
						.setName("use-columns")
						.setDescription(
							"Displays the emote list in a table (recommended for desktop but not mobile)"
						)
				)
		)
		.addSubcommand((subcommand) =>
			subcommand
				.setName(CMD_LSEMOTES_QUERY)
				.setDescription(
					"Filter the emote list through the default emote resolver, based on Levenshtein distance"
				)
				.addStringOption((option) =>
					option
						.setName("query")
						.setDescription("The query to filter emotes by")
						.setRequired(true)
				)
				.addNumberOption((option) =>
					option
						.setName("levenshtein-threshold")
						.setDescription(
							`The likeness threshold of a query, with 0 being the strictest (Default: ${MAX_ACCEPTED_DISTANCE})`
						)
						.setMinValue(0)
				)
				.addBooleanOption((option) =>
					option
						.setName("disable-threshold")
						.setDescription(
							"Disables filtering emotes by a threshold (Default: false)"
						)
				)
				.addBooleanOption((option) =>
					option
						.setName("use-columns")
						.setDescription(
							"Displays the emote list in a table (recommended for desktop but not mobile)"
						)
				)
		)
		.addSubcommand((subcommand) =>
			subcommand
				.setName(CMD_LSEMOTES_REGEX)
				.setDescription("Filter the emote list through a regex pattern")
				.addStringOption((option) =>
					option
						.setName("pattern")
						.setDescription("The regex pattern to filter emotes by")
						.setRequired(true)
				)
				.addBooleanOption((option) =>
					option
						.setName("is-case-sensitive")
						.setDescription(
							"Whether or not to check the pattern for case-sensitivity (false by default)"
						)
				)
				.addBooleanOption((option) =>
					option
						.setName("use-columns")
						.setDescription(
							"Displays the emote list in a table (recommended for desktop but not mobile)"
						)
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
	// Filter Links - Examine each URL and replace if necessary
	new ContextMenuCommandBuilder()
		.setName(MENU_MSG_FILTER_LINKS)
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
import { executeMenu as executeMenuReact } from "./react";
import {
	execute as executeSay,
	executeMenuEdit,
	executeMenuDelete,
} from "./say";
import { executeMenu as executeMenuFilterLinks } from "./filter-links";

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
			case MENU_MSG_FILTER_LINKS:
				await executeMenuFilterLinks(interaction);
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
