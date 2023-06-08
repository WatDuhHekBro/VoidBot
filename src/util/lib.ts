/**
 * Splits a command by spaces while accounting for quotes which capture string arguments.
 * - `\"` = `"`
 * - `\\` = `\`
 */
export function parseArgs(line: string): string[] {
	let result = [];
	let selection = "";
	let inString = false;
	let isEscaped = false;

	for (let c of line) {
		if (isEscaped) {
			if (['"', "\\"].includes(c)) selection += c;
			else selection += "\\" + c;

			isEscaped = false;
		} else if (c === "\\") isEscaped = true;
		else if (c === '"') inString = !inString;
		else if (c === " " && !inString) {
			result.push(selection);
			selection = "";
		} else selection += c;
	}

	if (selection.length > 0) result.push(selection);

	return result;
}

/**
 * Allows you to store a template string with variable markers and parse it later.
 * - Use `%name%` for variables
 * - `%%` = `%`
 * - If the invalid token is null/undefined, nothing is changed.
 */
export function parseVars(
	line: string,
	definitions: { [key: string]: string },
	delimiter = "%",
	invalid: string | null = ""
): string {
	let result = "";
	let inVariable = false;
	let token = "";

	for (const c of line) {
		if (c === delimiter) {
			if (inVariable) {
				if (token === "") result += delimiter;
				else {
					if (token in definitions) result += definitions[token];
					else if (invalid === null) result += `%${token}%`;
					else result += invalid;

					token = "";
				}
			}

			inVariable = !inVariable;
		} else if (inVariable) token += c;
		else result += c;
	}

	return result;
}

export function parseVarsCallback(
	line: string,
	callback: (variable: string) => string,
	delimiter = "%"
): string {
	let result = "";
	let inVariable = false;
	let token = "";

	for (const c of line) {
		if (c === delimiter) {
			if (inVariable) {
				if (token === "") result += delimiter;
				else {
					result += callback(token);
					token = "";
				}
			}

			inVariable = !inVariable;
		} else if (inVariable) token += c;
		else result += c;
	}

	return result;
}

// A 50% chance would be "Math.random() < 0.5" because Math.random() can be [0, 1), so to make two equal ranges, you'd need [0, 0.5)U[0.5, 1).
// Similar logic would follow for any other percentage. Math.random() < 1 is always true (100% chance) and Math.random() < 0 is always false (0% chance).
export const Random = {
	num: (min: number, max: number) => Math.random() * (max - min) + min,
	int: (min: number, max: number) => Math.floor(Random.num(min, max)),
	chance: (decimal: number) => Math.random() < decimal,
	sign: (number = 1) => number * (Random.chance(0.5) ? -1 : 1),
	deviation: (base: number, deviation: number) =>
		Random.num(base - deviation, base + deviation),
};

/**
 * Pluralises a word and chooses a suffix attached to the root provided.
 * - pluralise("credit", "s") = credit/credits
 * - pluralise("part", "ies", "y") = party/parties
 * - pluralise("sheep") = sheep
 */
export function pluralise(
	value: number,
	word: string,
	plural = "",
	singular = "",
	excludeNumber = false
): string {
	let result = excludeNumber ? "" : `${value} `;

	if (value === 1) result += word + singular;
	else result += word + plural;

	return result;
}

/**
 * Pluralises a word for changes.
 * - (-1).pluraliseSigned() = '-1 credits'
 * - (0).pluraliseSigned() = '+0 credits'
 * - (1).pluraliseSigned() = '+1 credit'
 */
export function pluraliseSigned(
	value: number,
	word: string,
	plural = "",
	singular = "",
	excludeNumber = false
): string {
	const sign = value >= 0 ? "+" : "";
	return `${sign}${pluralise(value, word, plural, singular, excludeNumber)}`;
}

/** Returns a random element from this array. */
export function random<T>(array: T[]): T {
	return array[Math.floor(Math.random() * array.length)];
}

/**
 * Splits up this array into a specified length.
 * `$([1,2,3,4,5,6,7,8,9,10]).split(3)` = `[[1,2,3],[4,5,6],[7,8,9],[10]]`
 */
export function split<T>(array: T[], lengthOfEachSection: number): T[][] {
	const amountOfSections = Math.ceil(array.length / lengthOfEachSection);
	const sections = new Array<T[]>(amountOfSections);

	for (let index = 0; index < amountOfSections; index++)
		sections[index] = array.slice(
			index * lengthOfEachSection,
			(index + 1) * lengthOfEachSection
		);

	return sections;
}

/**
 * Utility function to require all possible cases to be handled at compile time.
 *
 * To use this function, place it in the "default" case of a switch statement or the "else" statement of an if-else branch.
 * If all cases are handled, the variable being tested for should be of type "never", and if it isn't, that means not all cases are handled yet.
 */
export function requireAllCasesHandledFor(variable: never): never {
	throw new Error(
		`This function should never be called but got the value: ${variable}`
	);
}

// Levenshtein distance coefficients for all transformation types.
// TODO: Investigate what values result in the most optimal matching strategy.
const directMatchWeight = 0.0;
const uppercaseWeight = 0.2;
const lowercaseWeight = 0.5;
const substitutionWeight = 1.0;
const deletionWeight = 1.5;
const insertionWeight = 1.5;

// Algorithm taken from https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_two_matrix_rows
// Modified for separate handling of uppercasing and lowercasing transformations.
export function levenshtein(s: string, t: string): number {
	const m = s.length;
	const n = t.length;

	let v0 = new Array(n + 1);
	let v1 = new Array(n + 1);

	let i, j;

	for (i = 0; i <= n; i++) v0[i] = i;

	for (i = 0; i < m; i++) {
		v1[0] = i + 1;

		for (j = 0; j < n; j++) {
			let r;

			if (s[i] === t[j]) r = directMatchWeight;
			else if (s[i] === t[j].toUpperCase()) r = uppercaseWeight;
			else if (s[i] === t[j].toLowerCase()) r = lowercaseWeight;
			else r = substitutionWeight;

			v1[j + 1] = Math.min(
				v0[j + 1] + deletionWeight,
				v1[j] + insertionWeight,
				v0[j] + r
			);
		}

		const tmp = v1;
		(v1 = v0), (v0 = tmp);
	}

	return v0[n];
}
