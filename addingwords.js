const readline = require("readline");

let rl = readline.createInterface({
	input: require("fs").createReadStream("input.txt"),
});

let env;

rl.on("line", line => {
	let words = line.split(' ');

	switch (words[0]) {
		case "def":
			env[words[1]] = parseInt(words[2]);
			break;
		case "calc":
			let args = words.slice(1, words.indexOf("="));
			evaluate(args, env);
			break;
		case "clear":
			env = {};
			break;
	}
});

function evaluate(args, env) {
	
}
