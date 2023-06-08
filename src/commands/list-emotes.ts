import {
	ActionRowBuilder,
	ButtonBuilder,
	ButtonStyle,
	ChatInputCommandInteraction,
	ComponentType,
	EmbedBuilder,
	ModalActionRowComponentBuilder,
	ModalBuilder,
	TextInputBuilder,
	TextInputStyle,
} from "discord.js";
import { CMD_LSEMOTES_ALL, CMD_LSEMOTES_QUERY, CMD_LSEMOTES_REGEX } from ".";
import { DistanceEmote, emoteRegistry } from "../modules/emote-registry";
import { split } from "../util/lib";

const BTN_LEFT = "lsemotes-left";
const BTN_RIGHT = "lsemotes-right";
const BTN_DLEFT = "lsemotes-double-left";
const BTN_DRIGHT = "lsemotes-double-right";
const BTN_SEARCH = "lsemotes-search";
const ID_MENU = "lsemotes-search";
const ID_MENU_INPUT = "lsemotes-search-input";
export const SORT_BY_ALPHA_ASC = "sort-by-alpha";
export const SORT_BY_ALPHA_DESC = "sort-by-alpha-desc";

type EmbedEntry = {
	// Example: "<:mlep:123> mlep\n<:mlep:123> mlep2"
	emote: string;
	// Example: "Cloud Computing\nSome Other Server"
	guild: string;
	// Example: "1.5\n2.0"
	distance: number | null;
};

export async function execute(interaction: ChatInputCommandInteraction) {
	const subcommand = interaction.options.getSubcommand();
	const embedEntries: EmbedEntry[] = [];
	let hasDistance = false;

	// Generates different field text for the embed depending on the mode given
	// Then add those entries to the embed-related arrays, as that's all that'll be used in the end
	// No switch-case possible due to use of same variable names
	if (subcommand === CMD_LSEMOTES_ALL) {
		const sortMode =
			interaction.options.getString("sort-by") ?? SORT_BY_ALPHA_ASC;

		const emotes = emoteRegistry.getAlphaSortedEmotes();

		if (sortMode === SORT_BY_ALPHA_DESC) {
			emotes.reverse();
		}

		for (const emote of emotes) {
			embedEntries.push({
				emote: `${emote.ref} ${emote.ref.name}`,
				guild: emote.ref.guild.name,
				distance: null,
			});
		}
	} else if (subcommand === CMD_LSEMOTES_QUERY) {
		hasDistance = true;
		const query = interaction.options.getString("query")!;
		const threshold = interaction.options.getNumber(
			"levenshtein-threshold"
		);
		const disableThreshold =
			interaction.options.getBoolean("disable-threshold") ?? false;
		let emotes: DistanceEmote[];

		if (disableThreshold) {
			emotes = emoteRegistry.getDistanceSortedEmotes(query, null);
		} else if (threshold) {
			emotes = emoteRegistry.getDistanceSortedEmotes(query, threshold);
		} else {
			emotes = emoteRegistry.getDistanceSortedEmotes(query);
		}

		for (const emote of emotes) {
			embedEntries.push({
				emote: `${emote.ref} ${emote.ref.name}`,
				guild: emote.ref.guild.name,
				distance: emote.distance,
			});
		}
	} else if (subcommand === CMD_LSEMOTES_REGEX) {
		const pattern = interaction.options.getString("pattern")!;
		const isCaseSensitive =
			interaction.options.getBoolean("is-case-sensitive") ?? false;

		const emotes = emoteRegistry.getRegexFilteredEmotes(
			pattern,
			isCaseSensitive
		);

		if (!emotes) {
			return await interaction.reply({
				content: "**Error:** The regex pattern you entered is invalid.",
			});
		}

		for (const emote of emotes) {
			embedEntries.push({
				emote: `${emote.ref} ${emote.ref.name}`,
				guild: emote.ref.guild.name,
				distance: null,
			});
		}
	}

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

	const pages = split(embedEntries, 20);
	const totalPages = pages.length;
	let currentPage = 0;

	// Exit early if no entries
	if (embedEntries.length === 0) {
		return await interaction.reply("No valid emotes found by that query.");
	}

	// Exit early if single page
	if (totalPages === 1) {
		return await interaction.reply({
			embeds: [
				getEmoteEmbed(
					currentPage + 1,
					totalPages,
					pages[currentPage],
					hasDistance
				),
			],
		});
	}

	const output = await interaction.reply({
		embeds: [
			getEmoteEmbed(
				currentPage + 1,
				totalPages,
				pages[currentPage],
				hasDistance
			),
		],
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
					.setTitle("Which page to look at?")
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
					filter: (i) =>
						i.user.id === interaction.user.id &&
						i.customId === ID_MENU,
					time: 60000,
				});

				const input = response.fields.getTextInputValue(ID_MENU_INPUT);
				const num = +input - 1;

				if (!Number.isNaN(num)) {
					if (num < 0 || num > totalPages) {
						await response.reply({
							content:
								"**Error:** The page number you entered is out of bounds.",
							ephemeral: true,
						});
						return;
					} else {
						currentPage = num;
						currentPage = getLoopedPageNumber(
							currentPage,
							totalPages
						);
					}
				} else {
					await response.reply({
						content:
							"**Error:** You didn't enter a valid page number.",
						ephemeral: true,
					});
					return;
				}

				await response.deferUpdate();
				await i.editReply({
					embeds: [
						getEmoteEmbed(
							currentPage + 1,
							totalPages,
							pages[currentPage],
							hasDistance
						),
					],
					components: [row],
				});
			} catch (error) {
				console.error(error);
				// Because the collector timed out, there's no need for an error message
			}
		} else {
			switch (i.customId) {
				case BTN_LEFT:
					currentPage--;
					currentPage = getLoopedPageNumber(currentPage, totalPages);
					break;
				case BTN_RIGHT:
					currentPage++;
					currentPage = getLoopedPageNumber(currentPage, totalPages);
					break;
				case BTN_DLEFT:
					currentPage -= 3;
					currentPage = getLoopedPageNumber(currentPage, totalPages);
					break;
				case BTN_DRIGHT:
					currentPage += 3;
					currentPage = getLoopedPageNumber(currentPage, totalPages);
					break;
			}

			await i.update({
				embeds: [
					getEmoteEmbed(
						currentPage + 1,
						totalPages,
						pages[currentPage],
						hasDistance
					),
				],
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

// Tightly coupled abstractions
function getLoopedPageNumber(
	newPageNumber: number,
	upperBound: number
): number {
	if (newPageNumber >= upperBound) {
		return (newPageNumber %= upperBound);
	} else if (newPageNumber < 0) {
		// 0 --> 0
		// -1 --> 2 = 3 - (-(-1) % 3) = 3 - 1
		// -2 --> 1 = 3 - (-(-2) % 3) = 3 - 2
		// -3 --> 0 = 3 - (-(-3) % 3) = 3 - 0
		// -4 --> 2 = 3 - (-(-4) % 3) = 3 - 1
		// -5 --> 1 = 3 - (-(-5) % 3) = 3 - 2
		// -6 --> 0 = 3 - (-(-6) % 3) = 3 - 0
		newPageNumber = upperBound - (-newPageNumber % upperBound);
		return newPageNumber === upperBound ? 0 : newPageNumber;
	} else {
		return newPageNumber;
	}
}

// TODO: Create abstractions of pagination again
function getEmoteEmbed(
	currentPage: number,
	totalPages: number,
	page: EmbedEntry[],
	hasDistance: boolean
) {
	const embedEmotes: string[] = [];
	const embedGuilds: string[] = [];
	const embedDistance: number[] = [];

	for (const entry of page) {
		embedEmotes.push(entry.emote);
		embedGuilds.push(entry.guild);

		// Redundant as hasDistance, but oh well
		if (entry.distance) {
			embedDistance.push(entry.distance);
		}
	}

	const embed = new EmbedBuilder()
		.setTitle(
			totalPages === 1
				? "**Emotes**"
				: `**Emotes** (Page ${currentPage} of ${totalPages})`
		)
		.setColor("Aqua")
		.addFields([
			{
				name: "Emote",
				value: embedEmotes.join("\n"),
				inline: true,
			},
			{
				name: "Server",
				value: embedGuilds.join("\n"),
				inline: true,
			},
		]);

	if (hasDistance) {
		embed.addFields([
			{
				name: "Likeness",
				value: embedDistance.join("\n"),
				inline: true,
			},
		]);
	}

	return embed;
}
