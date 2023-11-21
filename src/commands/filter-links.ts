import { MessageContextMenuCommandInteraction } from "discord.js";

// Regex to match any URL
// Group #1: "http://" or "https://"
// Group #2: Domain (e.g. "www.asdf.example.com")
// Group #3: Path (empty string or "/some/path")
const URL_PATTERN =
	/(https?:\/\/)([-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6})\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)/;

export async function executeMenu(
	interaction: MessageContextMenuCommandInteraction
) {
	const message = interaction.targetMessage;
	const updatedMessage = filterLinks(message.content);
	const isMessageFiltered = updatedMessage !== message.content;

	if (isMessageFiltered) {
		await replyPublicMessage(interaction, updatedMessage);
	} else {
		await replyNoMessageFound(interaction);
	}
}

function filterLinks(message: string): string {
	return regexCustomReplace(message, URL_PATTERN, transformLink);
}

// Transformer function with defined rules for URL
// Rule: "youtube.com" / "www.youtube.com" / "youtu.be" (filter out "si" and "pp" parameters)
// Rule: "twitter.com" --> "vxtwitter.com"
function transformLink(match: RegExpExecArray): string {
	const text = match[0];
	const url = new URL(text);

	if (url.host === "twitter.com" || url.host === "x.com") {
		return `${url.protocol}//vxtwitter.com${url.pathname}`;
	}

	if (
		(url.host === "youtube.com" || url.host === "www.youtube.com") &&
		url.pathname === "/watch"
	) {
		let parameters: string[] = [];

		for (const [key, value] of url.searchParams) {
			if (key !== "si" && key !== "pp") {
				parameters.push(`${key}=${value}`);
			}
		}

		// Add "?" if there are any parameters
		if (parameters.length > 0) {
			return `${url.origin}/watch?${parameters.join("&")}`;
		} else {
			return `${url.origin}/watch`;
		}
	}

	if (url.host === "youtu.be") {
		let parameters: string[] = [];

		for (const [key, value] of url.searchParams) {
			if (key !== "si" && key !== "pp") {
				parameters.push(`${key}=${value}`);
			}
		}

		// Add "?" if there are any parameters
		if (parameters.length > 0) {
			return `${url.origin}${url.pathname}?${parameters.join("&")}`;
		} else {
			return `${url.origin}${url.pathname}`;
		}
	}

	return text;
}

// Continually matches a string against a pattern, replacing each instance with a custom transformer function
// regexCustomReplace("this sentence is a sentence", /sentence/, (input) => "nope") --> "this nope is a nope"
//
// ISSUE: The regex checks the first instance only (stateless), will infinite loop if transformer gives same regex (e.g. URL), fix this via recursion (quick and dirty workaround).
// Meaning "this sentence is a sentence" makes a call to " is a sentence" with the same regex.
function regexCustomReplace(
	message: string,
	pattern: RegExp,
	transformer: (input: RegExpExecArray) => string
): string {
	let match = pattern.exec(message);

	if (match !== null) {
		const text = match[0];
		const index = match.index;
		const length = text.length;

		// Sandwich the new message with the new text at the center
		message =
			message.substring(0, index) +
			transformer(match) +
			regexCustomReplace(
				message.substring(index + length),
				pattern,
				transformer
			);

		// Next match
		match = pattern.exec(message);
	}

	return message;
}

async function replyPublicMessage(
	interaction: MessageContextMenuCommandInteraction,
	message: string
) {
	await interaction.reply(message);
}

async function replyNoMessageFound(
	interaction: MessageContextMenuCommandInteraction
) {
	await interaction.reply({
		content: "*No URLs needed to be filtered in this message.*",
		ephemeral: true,
	});
}
