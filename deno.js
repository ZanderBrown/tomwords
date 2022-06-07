const decoder = new TextDecoder("utf-8");
const data = Deno.readFileSync("words/words.txt");
var words = decoder.decode(data);
words = words.split("\n");

var badLetters = /[gkmqvwxzio]/;
var longestAcceptableWord = "";

for (var testWord of words) {
	if (testWord.length <= longestAcceptableWord.length) {
		continue;
	}
	
	if (testWord.match(badLetters)) {
		continue;
	}

	longestAcceptableWord = testWord;
}

console.log(longestAcceptableWord);

