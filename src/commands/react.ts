import {
	ActionRowBuilder,
	ChatInputCommandInteraction,
	MessageContextMenuCommandInteraction,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	PermissionsBitField,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";
import { emoteRegistry } from "../modules/emote-registry";

// Have "confirm" boolean option to make message ephemeral and show emotes to react with before reacting with it?

export async function execute(interaction: ChatInputCommandInteraction) {
	// You should check if the server owner has the reaction permission off by default first
	if (
		!interaction.guild?.members.me?.permissions.has(
			PermissionsBitField.Flags.AddReactions
		)
	) {
		return await interaction.reply({
			content:
				"**Error:** I don't have permissions to add reactions in the channel or server you tried to use this in!",
			ephemeral: true,
		});
	}

	await interaction.reply("react");
}

const ID_MENU = "react-query";
const ID_MENU_INPUT = "react-query-input";

export async function executeMenu(
	interaction: MessageContextMenuCommandInteraction
) {
	// You should check if the server owner has the reaction permission off by default first
	if (
		!interaction.guild?.members.me?.permissions.has(
			PermissionsBitField.Flags.AddReactions
		)
	) {
		return await interaction.reply({
			content:
				"**Error:** I don't have permissions to add reactions in the channel or server you tried to use this in!",
			ephemeral: true,
		});
	}

	const message = interaction.targetMessage;

	await interaction.showModal(
		new ModalBuilder()
			.setCustomId(ID_MENU)
			.setTitle("Enter the emotes to react with")
			.addComponents(
				new ActionRowBuilder<ModalActionRowComponentBuilder>().addComponents(
					new TextInputBuilder()
						.setCustomId(ID_MENU_INPUT)
						.setStyle(TextInputStyle.Short)
						.setLabel("Emote Names")
						.setPlaceholder("emote1 emote2 ...")
						.setRequired(true)
				)
			)
	);

	try {
		const response = await interaction.awaitModalSubmit({
			filter: (interaction) => interaction.customId === ID_MENU,
			time: 60000,
		});

		const emotesInput = response.fields.getTextInputValue(ID_MENU_INPUT);
		const emotesList = emotesInput.split(/ +/);
		const emotesOutput: string[] = [];

		for (const query of emotesList) {
			emotesOutput.push(emoteRegistry.getNearestEmote(query));
		}

		await response.deferUpdate();

		// React asynchronously in a loop
		for (const emote of emotesOutput) {
			try {
				const reaction = await message.react(emote);

				setTimeout(() => {
					reaction.users.remove(reaction.client.user.id);
				}, 5000);
			} catch (error) {
				return console.error(error);
			}
		}
	} catch {
		// Because the collector times out, there's no need for an error message
	}
}
