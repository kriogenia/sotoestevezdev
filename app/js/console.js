const commands = {
	about: {
		action: () => history.appendChild(wrapNode("p", "Hi, I'm Kriogenia")),
	},
	help: {
		action: () =>
			history.appendChild(
				wrapNode(
					"p",
					`List of commands: ${commandList
						.map((cmd) => "<a>" + cmd + "</a>")
						.join(", ")}`
				)
			),
	},
	stack: {
		action: () => history.appendChild(wrapNode("p", "This is my tech stack")),
	},
	theme: {
		action: () => changeTheme(),
	},
};
const commandList = Object.keys(commands);

const prompt = "sesh > ";

const history = document.getElementById("history");
const input = document.getElementById("input");
const cursor = document.getElementById("cursor");

const controlInput = (_) => {
	input.focus();

	const range = document.createRange();
	const selection = window.getSelection();
	const { childNodes } = input;
	const lastChildNode = childNodes && childNodes.length - 1;

	range.selectNodeContents(
		lastChildNode === -1 ? input : childNodes[lastChildNode]
	);
	range.collapse(false);

	selection.removeAllRanges();
	selection.addRange(range);
};

const handleCommand = (command) => {
	const line = document.createElement("div");
	line.classList.add("line");

	const prefix = wrapNode("span", prompt);
	prefix.classList.add("prompt");

	line.appendChild(prefix);
	line.appendChild(document.createTextNode(command));
	history.appendChild(line);

	if (commandList.includes(command)) {
		commands[command].action();
	}

	history.parentElement.scrollTo(0, history.scrollHeight);
};

const wrapNode = (tag, content) => {
	const element = document.createElement(tag);
	element.innerHTML = content;
	return element;
};

const changeTheme = () => {
	history.appendChild(wrapNode("p", "Changing theme"));
};

document.addEventListener("selectionchange", () => {
	if (document.activeElement.id !== "input") return;

	const range = window.getSelection().getRangeAt(0);
	if (range.endOffset < input.textContent.length) {
		input.classList.add("noCaret");
	} else {
		input.classList.remove("noCaret");
	}
});

input.addEventListener("input", () => {
	// If we paste HTML, format it as plain text and break it up
	// input individual lines/commands:
	if (input.childElementCount > 0) {
		const lines = input.innerText.replace(/\n$/, "").split("\n");
		const lastLine = lines[lines.length - 1];

		for (let i = 0; i <= lines.length - 2; ++i) {
			handleCommand(lines[i]);
		}

		input.textContent = lastLine;

		controlInput();
	}

	// If we delete everything, display the square caret again:
	if (input.innerText.length === 0) {
		input.classList.remove("noCaret");
	}
});

document.addEventListener("keydown", ({ target }) => {
	if (target !== input) {
		controlInput();
	}
});

input.addEventListener("keydown", (e) => {
	if (e.key === "Enter") {
		e.preventDefault();

		handleCommand(input.textContent);
		input.textContent = "";
		controlInput();
	}
});

// Set the focus to the input so that you can start typing straigh away:
input.focus();
