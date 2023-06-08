import { Collection, GuildEmoji } from "discord.js";
import RE2 from "re2";
import { client } from "..";
import { levenshtein } from "../util/lib";

// {id, name}
type Emote = { id: string; name: string; ref: GuildEmoji };
// {id, name, levenshtein distance}
type DistanceEmote = {
	id: string;
	name: string;
	distance: number;
	ref: GuildEmoji;
};

// Maximum Levenshtein distance for an emote to be considered a suitable match candidate.
const MAX_ACCEPTED_DISTANCE = 3.0;

// Other patterns
const unicodeEmojiRegex =
	/^(?:[\u2700-\u27bf]|(?:\ud83c[\udde6-\uddff]){2}|[\ud800-\udbff][\udc00-\udfff]|[\u0023-\u0039]\ufe0f?\u20e3|\u3299|\u3297|\u303d|\u3030|\u24c2|\ud83c[\udd70-\udd71]|\ud83c[\udd7e-\udd7f]|\ud83c\udd8e|\ud83c[\udd91-\udd9a]|\ud83c[\udde6-\uddff]|\ud83c[\ude01-\ude02]|\ud83c\ude1a|\ud83c\ude2f|\ud83c[\ude32-\ude3a]|\ud83c[\ude50-\ude51]|\u203c|\u2049|[\u25aa-\u25ab]|\u25b6|\u25c0|[\u25fb-\u25fe]|\u00a9|\u00ae|\u2122|\u2139|\ud83c\udc04|[\u2600-\u26FF]|\u2b05|\u2b06|\u2b07|\u2b1b|\u2b1c|\u2b50|\u2b55|\u231a|\u231b|\u2328|\u23cf|[\u23e9-\u23f3]|[\u23f8-\u23fa]|\ud83c\udccf|\u2934|\u2935|[\u2190-\u21ff])[\ufe00-\ufe0f]?$/;
const discordEmoteMentionRegex = /^<a?:\w+:\d+>$/;
const emoteNameWithSelectorRegex = /^(.+)~(\d+)$/;

// Stateful wrapper over emote utils
class EmoteRegistry {
	// Map<id, GuildEmoji>
	private cache: Collection<string, GuildEmoji>;
	// {id, name}[]
	// Sorted alphabetically, for use with regex filter
	private alphaRegistry: Emote[];

	constructor() {
		this.cache = client.emojis.cache;
		this.alphaRegistry = [];
	}

	public update() {
		this.cache = client.emojis.cache;

		// Update alphabetical sort
		this.alphaRegistry = this.transformEmoteCache();
		this.alphaRegistry.sort((a, b) => {
			const first = a.name.toLowerCase();
			const second = b.name.toLowerCase();

			if (first > second) return 1;
			else if (first < second) return -1;
			else return 0;
		});
	}

	// Returns a list of entries ordered by Levenshtein distance
	// Forms the basis of finding nearest emote (just pick the first entry)
	public getDistanceSortedEmotes(
		query: string,
		threshold: number | null = MAX_ACCEPTED_DISTANCE
	): DistanceEmote[] {
		const emotes: DistanceEmote[] = [];

		// First gather distances and emotes based on distance
		for (const emote of this.cache.values()) {
			if (emote.name) {
				const distance = levenshtein(emote.name, query);

				// Ignore filter if threshold isn't defined
				if (threshold !== null && distance > threshold) {
					continue;
				}

				emotes.push({
					id: emote.id,
					name: emote.name,
					distance,
					ref: emote,
				});
			}
		}

		// Then sort them based on their distances
		emotes.sort((a, b) => a.distance - b.distance);

		return emotes;
	}

	// Should already be sorted via update()
	public getAlphaSortedEmotes(): Emote[] {
		return this.alphaRegistry;
	}

	// Picks the closest given emote based on the user's query
	// Also carries over the "emote~#" notation used to deduplicate emotes with identical names
	public getNearestEmote(query: string): string {
		// Selector number used for disambiguating multiple emotes with same name.
		let selector = 0;

		// If the query has emoteName~123 format, extract the actual name and the selector number.
		const queryWithSelector = query.match(emoteNameWithSelectorRegex);
		if (queryWithSelector) {
			query = queryWithSelector[1];
			selector = +queryWithSelector[2];
		}

		// Try to match an emote name directly if the selector is for the closest match.
		if (selector == 0) {
			const directMatchEmote = this.cache.find(
				(emote) => emote.name === query
			);
			if (directMatchEmote) return directMatchEmote.toString();
		}

		// Find all similar emote candidates within certain threshold and select Nth top one according to the selector.
		const similarEmotes = this.getDistanceSortedEmotes(query);

		if (similarEmotes.length > 0) {
			selector = Math.min(selector, similarEmotes.length - 1);
			return similarEmotes[selector].ref.toString();
		}

		return "❓";
	}

	// Returns alphabetical entries filtered by RE2 regex (linear time engine)
	// Returns null if the provided regex was invalid
	public getRegexFilteredEmotes(
		pattern: string,
		isCaseSensitive: boolean
	): Emote[] | null {
		try {
			const regex = isCaseSensitive
				? new RE2(pattern)
				: new RE2(pattern, "i");
			const emotes = this.getAlphaSortedEmotes();
			return emotes.filter((emote) => regex.test(emote.name));
		} catch {
			return null;
		}
	}

	// Whether or not the query directly matches a Unicode emoji or a Discord custom emote mention
	public static isNativeEmote(query: string): boolean {
		return (
			discordEmoteMentionRegex.test(query) ||
			unicodeEmojiRegex.test(query)
		);
	}

	// Transforms the refs registry into a reduced format
	private transformEmoteCache(): Emote[] {
		const refs: Emote[] = [];

		for (const emote of this.cache.values()) {
			if (emote.name) {
				refs.push({
					id: emote.id,
					name: emote.name,
					ref: emote,
				});
			}
		}

		return refs;
	}
}

export const emoteRegistry = new EmoteRegistry();
