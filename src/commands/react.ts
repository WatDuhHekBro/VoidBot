import {
	ActionRowBuilder,
	ChatInputCommandInteraction,
	MessageContextMenuCommandInteraction,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";

// Have "confirm" boolean option to make message ephemeral and show emotes to react with before reacting with it?
// TODO: Also needs to check for permissions if react is off by default

export async function execute(interaction: ChatInputCommandInteraction) {
	await interaction.reply("react");
}

const ID_MENU = "react-query";
const ID_MENU_INPUT = "react-query-input";

export async function executeMenu(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;
	await message.react("1055589478594527345");

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

		const emotes = response.fields.getTextInputValue(ID_MENU_INPUT);
		console.log(emotes);

		await response.reply({
			content:
				Math.random() < 0.5
					? "Failed to react to the message."
					: "Reacting...",
			ephemeral: true,
		});
	} catch {
		// Because the collector times out, there's no need for an error message
	}
}
