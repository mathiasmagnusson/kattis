const readline = require("readline");

let rl = readline.createInterface({
	input: process.stdin,
});

let env = new Object();

rl.on("line", line => {
	let words = line.split(' ');

	switch (words[0]) {
		case "def":
			env[words[1]] = parseInt(words[2]);
			break;
		case "calc":
			let args = words.slice(1, words.indexOf("="));
			args.unshift("+");
			let word = evaluate(args, env);
			words.shift();
			console.log(`${words.join(' ')} ${word}`);
			break;
		case "clear":
			env = new Object();
			break;
	}
});

function evaluate(args, env) {
	let total = 0;
	
	for (let i = 0; i < args.length; i += 2) {
		let operator = args[i];
		let word = args[i + 1];

		if (!env.hasOwnProperty(word))
			return 'unknown';

		total += env[word] * (operator == "+" ? 1 : -1);
	}

	for (const [word, value] of Object.entries(env)) {
		if (total === value) {
			return word;
		}
	}

	return 'unknown';
}
