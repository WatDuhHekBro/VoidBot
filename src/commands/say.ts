import {
	ActionRowBuilder,
	ChatInputCommandInteraction,
	MessageContextMenuCommandInteraction,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";

// Is it possible to call a modal and have it redirect to your original prompt? Or is there no way to verify that there hasn't been tampering?

const ID_MENU = "say-query";
const ID_MENU_INPUT = "say-query-input";

export async function execute(interaction: ChatInputCommandInteraction) {
	//await interaction.reply("say");

	await interaction.showModal(
		new ModalBuilder()
			.setCustomId(ID_MENU)
			.setTitle("Enter the emotes to react with")
			.addComponents(
				new ActionRowBuilder<ModalActionRowComponentBuilder>().addComponents(
					new TextInputBuilder()
						.setCustomId(ID_MENU_INPUT)
						.setStyle(TextInputStyle.Paragraph)
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

		const input = response.fields.getTextInputValue(ID_MENU_INPUT);
		await response.reply(input + ", and thems the fax");
	} catch {
		// Because the collector timed out, there's no need for an error message
	}
}

export async function executeMenuEdit(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;
	// Check for if the author is the bot, no need to check for permissions
	await message.edit("now no one will ever know the context");
	await interaction.reply("say edit");
}

export async function executeMenuDelete(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;
	// Check for if the author is the bot, no need to check for permissions
	await message.delete();
	await interaction.reply("say delete");
}
