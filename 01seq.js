const readline = require('readline');
const fs = require('fs');

let rl = readline.createInterface({
	input: process.stdin,
});

rl.on('line', line => {
	let inversions = 0;
	forEveryPossibility(line, seq => {
		while (true) {
			let swapped = false;
			for (let i = 0; i < seq.length - 1; i++) {
				if (seq[i] === '1' && seq[i + 1] === '0') {
					seq[i    ] = '0';
					seq[i + 1] = '1';
					inversions = (inversions + 1) % 1000000007;
					swapped = true;
				}
			}
			if (!swapped) break;
		}
	});

	console.log(inversions);

	rl.close();
});

function forEveryPossibility(line, callback) {
	let qIndices = []; {
		let regex = /\?/g;
		let match;
		while (match = regex.exec(line)) {
			qIndices.push(match.index);
		}
	}

	for (let i = 0; i < 2 ** qIndices.length; i++) {
		let s = i.toString(2).padStart(qIndices.length, '0');
		let sequence = line.split('');
		for (let j = 0; j < s.length; j++) {
			sequence[qIndices[j]] = s[j];
		}
		
		callback(sequence);
	}
}
