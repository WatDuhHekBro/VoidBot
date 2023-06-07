import {
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
	ChatInputCommandInteraction,
} from "discord.js";

export async function execute(interaction: ChatInputCommandInteraction) {
	// TODO: Add custom IDs based off of message IDs to differentiate between different messages
	// Also specify the user who sent the command, unless you can mitigate that via an ephemeral option
	const row = new ActionRowBuilder<ButtonBuilder>().addComponents(
		new ButtonBuilder()
			.setCustomId("asdf")
			.setLabel("⬅️")
			.setStyle(ButtonStyle.Danger),
		new ButtonBuilder()
			.setCustomId("zxcv")
			.setLabel("zxcv")
			.setStyle(ButtonStyle.Success)
	);

	await interaction.reply({
		content: "lsemotes (insert embed here)",
		components: [row],
	});
}
