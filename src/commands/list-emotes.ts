import {
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
	ChatInputCommandInteraction,
	ComponentType,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";

const BTN_LEFT = "lsemotes-left";
const BTN_RIGHT = "lsemotes-right";
const BTN_DLEFT = "lsemotes-double-left";
const BTN_DRIGHT = "lsemotes-double-right";
const BTN_SEARCH = "lsemotes-search";
const ID_MENU = "lsemotes-search";
const ID_MENU_INPUT = "lsemotes-search-input";

export async function execute(interaction: ChatInputCommandInteraction) {
	let number = 0;

	// TODO: Add custom IDs based off of message IDs to differentiate between different messages
	// Also specify the user who sent the command, unless you can mitigate that via an ephemeral option
	const row = new ActionRowBuilder<ButtonBuilder>().addComponents(
		new ButtonBuilder()
			.setCustomId(BTN_DLEFT)
			.setEmoji("⏪")
			.setStyle(ButtonStyle.Secondary),
		new ButtonBuilder()
			.setCustomId(BTN_LEFT)
			.setEmoji("⬅️")
			.setStyle(ButtonStyle.Secondary),
		new ButtonBuilder()
			.setCustomId(BTN_SEARCH)
			.setEmoji("🔎")
			.setStyle(ButtonStyle.Secondary),
		new ButtonBuilder()
			.setCustomId(BTN_RIGHT)
			.setEmoji("➡️")
			.setStyle(ButtonStyle.Secondary),
		new ButtonBuilder()
			.setCustomId(BTN_DRIGHT)
			.setEmoji("⏩")
			.setStyle(ButtonStyle.Secondary)
	);

	const output = await interaction.reply({
		content: `Number: \`${number}\``,
		components: [row],
	});

	const collector = output.createMessageComponentCollector({
		componentType: ComponentType.Button,
		filter: (i) => i.user.id === interaction.user.id,
		time: 3_600_000,
	});

	collector.on("collect", async (i) => {
		if (i.customId === BTN_SEARCH) {
			await i.showModal(
				new ModalBuilder()
					.setCustomId(ID_MENU)
					.setTitle("Enter the page number to jump to")
					.addComponents(
						new ActionRowBuilder<ModalActionRowComponentBuilder>().addComponents(
							new TextInputBuilder()
								.setCustomId(ID_MENU_INPUT)
								.setStyle(TextInputStyle.Short)
								.setLabel("Page Number")
								.setRequired(true)
						)
					)
			);

			try {
				const response = await i.awaitModalSubmit({
					filter: (i) => i.customId === ID_MENU,
					time: 60000,
				});

				const input = response.fields.getTextInputValue(ID_MENU_INPUT);
				const num = +input;

				if (!Number.isNaN(num)) {
					number += num;
				}

				await response.deferUpdate();
				await i.editReply({
					content: `Number: \`${number}\``,
					components: [row],
				});
			} catch (error) {
				console.error(error);
				// Because the collector timed out, there's no need for an error message
			}
		} else {
			switch (i.customId) {
				case BTN_LEFT:
					number--;
					break;
				case BTN_RIGHT:
					number++;
					break;
				case BTN_DLEFT:
					number -= 3;
					break;
				case BTN_DRIGHT:
					number += 3;
					break;
			}

			await i.update({
				content: `Number: \`${number}\``,
				components: [row],
			});
		}
	});

	/*catch (error) {
		console.error(error);

		await interaction.editReply({
			content: "Closing menu",
			components: [],
		});
	}*/
}
