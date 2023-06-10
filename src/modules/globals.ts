import chalk from "chalk";
import fs from "fs";
import path from "path";

declare global {
	interface Console {
		ready: (...data: any[]) => void;
	}
}

const oldConsole = console;
let writer: fs.WriteStream | undefined;

// The custom console. In order of verbosity, error, warn, log, and debug. Ready is a variation of log.
console = {
	...oldConsole,
	// General Purpose Logger
	log(...args: any[]) {
		const timestamp = getFormattedTimestamp();
		oldConsole.log(
			chalk.white.bgGray(timestamp),
			chalk.black.bgWhite("INFO"),
			...args
		);
		writer?.write(`[${timestamp}] [INFO] ${args.join(" ")}\n`);
	},
	// "It'll still work, but you should really check up on this."
	warn(...args: any[]) {
		const timestamp = getFormattedTimestamp();
		oldConsole.warn(
			chalk.white.bgGray(timestamp),
			chalk.black.bgYellow("WARN"),
			...args
		);
		writer?.write(`[${timestamp}] [WARN] ${args.join(" ")}\n`);
	},
	// Used for anything which prevents the program from actually running.
	error(...args: any[]) {
		const timestamp = getFormattedTimestamp();
		oldConsole.error(
			chalk.white.bgGray(timestamp),
			chalk.white.bgRed("ERROR"),
			...args
		);
		writer?.write(`[${timestamp}] [ERROR] ${args.join(" ")}\n`);
	},
	// Be as verbose as possible. If anything might help when debugging an error, then include it.
	// Format: <path>/::(<object>.)<function>(<args>) = <value>
	// Example: console.debug(`core/lib::parseArgs("testing \"in progress\"") = ["testing", "in progress"]`)
	// Would probably be more suited for debugging program logic rather than function logic, which can be checked using unit tests.
	debug(...args: any[]) {
		const timestamp = getFormattedTimestamp();
		oldConsole.debug(
			chalk.white.bgGray(timestamp),
			chalk.white.bgBlue("DEBUG"),
			...args
		);
		writer?.write(`[${timestamp}] [DEBUG] ${args.join(" ")}\n`);
	},
	// Used once at the start of the program when the bot loads.
	ready(...args: any[]) {
		const timestamp = getFormattedTimestamp();
		oldConsole.log(
			chalk.white.bgGray(timestamp),
			chalk.black.bgGreen("READY"),
			...args
		);
		writer?.write(`[${timestamp}] [READY] ${args.join(" ")}\n`);
	},
};

// Enable conditionally:
// - Only when running the bot, not on register/clear
// - Not when SUPPRESS_LOGS is enabled
export function initFileLogger() {
	// Create write stream to log files
	if (!fs.existsSync("logs")) fs.mkdirSync("logs");

	writer = fs.createWriteStream(
		path.join("logs", `${getFormattedTimestamp(true)}.log`)
	);
}

function getFormattedTimestamp(filename = false) {
	const now = new Date();
	const year = now.getFullYear();
	const month = (now.getMonth() + 1).toString().padStart(2, "0");
	const day = now.getDate().toString().padStart(2, "0");
	const hour = now.getHours().toString().padStart(2, "0");
	const minute = now.getMinutes().toString().padStart(2, "0");
	const second = now.getSeconds().toString().padStart(2, "0");

	if (filename) {
		return `${year}-${month}-${day}_${hour}-${minute}-${second}`;
	} else {
		return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
	}
}
