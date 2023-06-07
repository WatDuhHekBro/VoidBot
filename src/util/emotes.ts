// For the sake of decoupling/testing, these functions will only work with general JS structures/types
// Since Discord.js functions like message.react() can interpret stringified IDs, that's all you need, not a GuildEmoji object
// You can then get emote info by using the ID for client.emoji.cache
// Map<id: string/u64, name: string>

export type EmoteRegistry = Map<string, string>;

// Proposed cached sorted list
export let registry = {};

export function generateUpdatedEmoteRegistry() {
	//
}

// Returns a list of entries ordered by Levenshtein distance
// Maybe this sorted list should be kept in memory until an emote update occurs?
// Forms the basis of finding nearest emote (just pick first entry)
export function sortEmotesByQuery() {
	//
}

// Picks the closest given emote based on the current cached registry
// Also carries over the "emote~#" notation used to deduplicate emotes with identical names
export function getNearestEmote() {
	//
}

// Meant for an emote list, displays entries ordered by Levenshtein distance filtered by a given threshold
export function filterEmotesByQuery() {
	// Call sortEmotesByQuery(), then filter via user-given threshold
}

// Meant for an emote list, displays alphabetical entries filtered by RE2 regex (linear time engine)
export function filterEmotesByRegex() {
	//
}
