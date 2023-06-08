import { strict as assert } from "assert";
import {
	pluralise,
	pluraliseSigned,
	split,
	parseVars,
	parseVarsCallback,
} from "./lib";

// I can't figure out a way to run the test suite while running the bot.
describe("Wrappers", () => {
	describe("NumberWrapper", () => {
		describe("#pluralise()", () => {
			it('should return "5 credits"', () => {
				assert.strictEqual(pluralise(5, "credit", "s"), "5 credits");
			});

			it('should return "1 credit"', () => {
				assert.strictEqual(pluralise(1, "credit", "s"), "1 credit");
			});

			it('should return "-1 credits"', () => {
				assert.strictEqual(pluralise(-1, "credit", "s"), "-1 credits");
			});

			it("should be able to work with a plural suffix", () => {
				assert.strictEqual(
					pluralise(2, "part", "ies", "y"),
					"2 parties"
				);
			});

			it("should be able to work with a singular suffix", () => {
				assert.strictEqual(pluralise(1, "part", "ies", "y"), "1 party");
			});

			it("should be able to exclude the number", () => {
				assert.strictEqual(
					pluralise(1, "credit", "s", "", true),
					"credit"
				);
			});
		});

		describe("#pluraliseSigned()", () => {
			it('should return "-1 credits"', () => {
				assert.strictEqual(
					pluraliseSigned(-1, "credit", "s"),
					"-1 credits"
				);
			});

			it('should return "+0 credits"', () => {
				assert.strictEqual(
					pluraliseSigned(0, "credit", "s"),
					"+0 credits"
				);
			});

			it('should return "+1 credit"', () => {
				assert.strictEqual(
					pluraliseSigned(1, "credit", "s"),
					"+1 credit"
				);
			});
		});
	});

	describe("StringWrapper", () => {
		describe("#parseVars()", () => {
			it('should replace %test% with "yeet"', () => {
				assert.strictEqual(
					parseVars("ya %test%", { test: "yeet" }),
					"ya yeet"
				);
			});
		});

		describe("#parseVarsCallback()", () => {
			it('should replace %test% with "yeet"', () => {
				assert.strictEqual(
					parseVarsCallback("ya %test% the %pear%", (variable) =>
						variable === "test" ? "yeet" : "null"
					),
					"ya yeet the null"
				);
			});
		});
	});

	describe("ArrayWrapper", () => {
		describe("#split()", () => {
			it("should split [1,2,3,4,5,6,7,8,9,10] into [[1,2,3],[4,5,6],[7,8,9],[10]]", () => {
				assert.deepStrictEqual(
					split([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
					[[1, 2, 3], [4, 5, 6], [7, 8, 9], [10]]
				);
			});

			it("should split [5] into [[5]]", () => {
				assert.deepStrictEqual(split([5], 3), [[5]]);
			});

			it("should split [] into []", () => {
				assert.deepStrictEqual(split([], 3), []);
			});
		});
	});
});
