// Anything involving Discord.js imports
// Can even be domain-specific wrappers for other items in "util"
import { client } from "..";

export function transformEmoteCache() {
	console.log(client.emojis.cache);
}
