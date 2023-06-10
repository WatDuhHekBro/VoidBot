import {
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
	ChatInputCommandInteraction,
	MessageContextMenuCommandInteraction,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";
import { parseVarsCallback } from "../util/lib";
import { emoteRegistry } from "../modules/emote-registry";
import { CMD_SAY } from ".";

const ID_MENU = "say-query";
const ID_MENU_INPUT = "say-query-input";

export async function execute(interaction: ChatInputCommandInteraction) {
	await interaction.showModal(
		new ModalBuilder()
			.setCustomId(ID_MENU)
			.setTitle("Enter your message")
			.addComponents(
				new ActionRowBuilder<ModalActionRowComponentBuilder>().addComponents(
					new TextInputBuilder()
						.setCustomId(ID_MENU_INPUT)
						.setStyle(TextInputStyle.Paragraph)
						.setLabel('Enter emotes in /slashes/, "//" = slash')
						.setPlaceholder(
							"This is some sample text with /emote1/ and /emote2/, you know? Okay, but does he//she know?"
						)
						.setRequired(true)
				)
			)
	);

	try {
		const response = await interaction.awaitModalSubmit({
			filter: (interaction) => interaction.customId === ID_MENU,
			time: 60000,
		});

		const input = response.fields.getTextInputValue(ID_MENU_INPUT);

		await response.reply(
			parseVarsCallback(
				input,
				(emoteQuery) => emoteRegistry.getNearestEmote(emoteQuery),
				"/"
			)
		);
	} catch {
		// Because the collector timed out, there's no need for an error message
	}
}

const ID_MENU_EDIT = "say-query-edit";
const ID_MENU_EDIT_INPUT = "say-query-edit-input";

export async function executeMenuEdit(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;

	// First check if the author is the bot (so there's no need to check for permissions)
	if (message.author.id !== interaction.client.user.id) {
		return await interaction.reply({
			content: "The message you tried to edit wasn't made by me.",
			ephemeral: true,
		});
	}
	// Then check if there is an interaction to began with
	if (!message.interaction) {
		return await interaction.reply({
			content:
				"The message you tried to edit wasn't made through an interaction.",
			ephemeral: true,
		});
	}
	// Then verify that the user of the target message is modifying their own message and not someone else's
	if (message.interaction.user.id !== interaction.user.id) {
		return await interaction.reply({
			content: "The message you tried to edit isn't yours.",
			ephemeral: true,
		});
	}
	// Then verify that the interaction was from /say and not something else
	if (message.interaction.commandName !== CMD_SAY) {
		return await interaction.reply({
			content: "The message you tried to edit isn't a proxy message.",
			ephemeral: true,
		});
	}

	await interaction.showModal(
		new ModalBuilder()
			.setCustomId(ID_MENU_EDIT)
			.setTitle("Enter your message")
			.addComponents(
				new ActionRowBuilder<ModalActionRowComponentBuilder>().addComponents(
					new TextInputBuilder()
						.setCustomId(ID_MENU_EDIT_INPUT)
						.setStyle(TextInputStyle.Paragraph)
						.setLabel('Enter emotes in /slashes/, "//" = slash')
						.setPlaceholder(
							"This is some sample text with /emote1/ and /emote2/, you know? Okay, but does he//she know?"
						)
						.setValue(message.content)
						.setRequired(true)
				)
			)
	);

	try {
		const response = await interaction.awaitModalSubmit({
			filter: (interaction) => interaction.customId === ID_MENU_EDIT,
			time: 60000,
		});

		const input = response.fields.getTextInputValue(ID_MENU_EDIT_INPUT);

		await response.deferUpdate();
		await message.edit(
			parseVarsCallback(
				input,
				(emoteQuery) => emoteRegistry.getNearestEmote(emoteQuery),
				"/"
			)
		);
	} catch {
		// Because the collector timed out, there's no need for an error message
	}
}

const BTN_CONFIRM = "say-delete-confirm";
const BTN_CANCEL = "say-delete-cancel";

export async function executeMenuDelete(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;

	// First check if the author is the bot (so there's no need to check for permissions)
	if (message.author.id !== interaction.client.user.id) {
		return await interaction.reply({
			content: "The message you tried to edit wasn't made by me.",
			ephemeral: true,
		});
	}
	// Then check if there is an interaction to began with
	if (!message.interaction) {
		return await interaction.reply({
			content:
				"The message you tried to edit wasn't made through an interaction.",
			ephemeral: true,
		});
	}
	// Then verify that the user of the target message is modifying their own message and not someone else's
	if (message.interaction.user.id !== interaction.user.id) {
		return await interaction.reply({
			content: "The message you tried to edit isn't yours.",
			ephemeral: true,
		});
	}
	// Then verify that the interaction was from /say and not something else
	if (message.interaction.commandName !== CMD_SAY) {
		return await interaction.reply({
			content: "The message you tried to edit isn't a proxy message.",
			ephemeral: true,
		});
	}

	const row = new ActionRowBuilder<ButtonBuilder>().addComponents(
		new ButtonBuilder()
			.setCustomId(BTN_CONFIRM)
			.setEmoji("✅")
			.setStyle(ButtonStyle.Secondary),
		new ButtonBuilder()
			.setCustomId(BTN_CANCEL)
			.setEmoji("❌")
			.setStyle(ButtonStyle.Secondary)
	);

	const output = await interaction.reply({
		content: "Are you sure you want to delete this proxy message?",
		ephemeral: true,
		components: [row],
	});

	try {
		const confirmation = await output.awaitMessageComponent({
			filter: (i) => i.user.id === interaction.user.id,
			time: 10_000,
		});

		if (confirmation.customId === BTN_CONFIRM) {
			await message.delete();
		}

		await interaction.deleteReply();
	} catch (e) {
		await interaction.deleteReply();
	}
}
