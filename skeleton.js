const readline = require('readline');

let rl = readline.createInterface({
	input: require('fs').createReadStream('input.txt'),
});

rl.on('line', line => {


	rl.close();
});
