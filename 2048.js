const readline = require('readline');

let rl = readline.createInterface({
	input: require('fs').createReadStream('input.txt'),
});

let lines = [];
let dir;

rl.on('line', line => {
	if (lines.length === 4) {
		dir = parseInt(line);
		return rl.close();
	}
	lines.push(line.split(' ').map(s => parseInt(s)));
});

rl.once('close', () => {
	switch (dir) {
		case 0:
			left();
			break;
		case 1:
			up();
			break;
		case 2:
			right();
			break;
		case 3:
			down();
			break;
	}

	console.log(lines.map(l => l.join(' ')).join('\n'));
});

function left() {
	for (let i = 0; i < 4; i++) {
		lines[i] = lines[i].filter(cell => cell != 0);

		for (let j = 0; j < lines[i].length - 1; j++) {
			if (lines[i][j] === lines[i][j + 1]) {
				lines[i][j] *= 2;
				lines[i][j + 1] = 0;
			}
		}

		lines[i] = lines[i].filter(cell => cell != 0);

		while (lines[i].length < 4) {
			lines[i].push(0)
		}
	}
}

function up() {
	let flippedLines = [[], [], [], []];

	for (let i = 0; i < 4; i++) {
		for (let j = 0; j < 4; j++) {
			flippedLines[3 - j][i] = lines[i][j];
		}
	}

	for (let i = 0; i < 4; i++) {
		flippedLines[i] = flippedLines[i].filter(cell => cell != 0);

		for (let j = 0; j < flippedLines[i].length - 1; j++) {
			if (flippedLines[i][j] === flippedLines[i][j + 1]) {
				flippedLines[i][j] *= 2;
				flippedLines[i][j + 1] = 0;
			}
		}

		flippedLines[i] = flippedLines[i].filter(cell => cell != 0);

		while (flippedLines[i].length < 4) {
			flippedLines[i].push(0)
		}
	}

	for (let i = 0; i < 4; i++) {
		for (let j = 0; j < 4; j++) {
			lines[j][3 - i] = flippedLines[i][j];
		}
	}
}

function right() {
	for (let i = 0; i < 4; i++) {
		lines[i] = lines[i].filter(cell => cell != 0);

		for (let j = lines[i].length - 1; j > 0; j--) {
			if (lines[i][j] === lines[i][j - 1]) {
				lines[i][j] *= 2;
				lines[i][j - 1] = 0;
			}
		}

		lines[i] = lines[i].filter(cell => cell != 0);

		while (lines[i].length < 4) {
			lines[i].splice(0, 0, 0);
		}
	}
}

function down() {
	let flippedLines = [[], [], [], []];

	for (let i = 0; i < 4; i++) {
		for (let j = 0; j < 4; j++) {
			flippedLines[3 - j][i] = lines[i][j];
		}
	}

	for (let i = 0; i < 4; i++) {
		flippedLines[i] = flippedLines[i].filter(cell => cell != 0);

		for (let j = 0; j < flippedLines[i].length - 1; j++) {
			if (flippedLines[i][j] === flippedLines[i][j + 1]) {
				flippedLines[i][j] *= 2;
				flippedLines[i][j + 1] = 0;
			}
		}

		flippedLines[i] = flippedLines[i].filter(cell => cell != 0);

		while (flippedLines[i].length < 4) {
			flippedLines[i].splice(0, 0, 0);
		}
	}

	for (let i = 0; i < 4; i++) {
		for (let j = 0; j < 4; j++) {
			lines[j][3 - i] = flippedLines[i][j];
		}
	}
}
