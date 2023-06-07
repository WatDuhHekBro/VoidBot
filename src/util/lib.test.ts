import { yeet } from "./lib";

test("asdf", () => {
	expect(yeet(3, 5)).toBe(8);
});

test("parseVars", () => {
	//expect(yeet(3, 5)).toBe(8);
	// parse_message_with_emotes
	// "Hello //world" --> "Hello /world"
	// "Hello \world" --> "Hello \nworld"
	// "Hello \\world" --> "Hello \world"
});
