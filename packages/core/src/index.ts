interface AcceptCondition {
	(c1: string): void | boolean;
}
function eof(): never {
	throw new Error('EOF');
}
type Balancer = { '{': number; '(': number; '[': number };
const balancer = () => ({ '{': 0, '(': 0, '[': 0 });
function isBalanced(balancer: Balancer) {
	return balancer['{'] === 0 && balancer['['] === 0 && balancer['('] === 0;
}
const STATE_DEFAULT = 0;
const STATE_TEMPLATE_LITERAL = 1;
export function tokenize(str: string, arr: string[] = []) {
	const len = str.length;
	// adjust length to ensure final token is always flushed
	str += '\n';
	let token: RegExpExecArray | null = null;
	let state: number = STATE_DEFAULT;
	let index = 0;
	let balancers: Balancer[] = [balancer()];
	const SPECIAL_CHARS_RE =
		/\$\{|=>|={2,3}|\.{1,3}|\d+|\?\.|[[\]{}()=:;<>'"`]|\/\/|\/\*|\*\/|\/|\\|,|[\+\-\*]{1,2}|\r\n?|\n|\s+/gi;

	function isEOL(str: string) {
		return /\r\n?|\n/.test(str);
	}
	function equal(c0: string | RegExp): AcceptCondition {
		if (c0 instanceof RegExp) return (c1: string) => c0.test(c1);
		return (c1: string) => c0 === c1;
	}
	function skip() {
		SPECIAL_CHARS_RE.lastIndex += 1;
	}
	function flush(adjustment = 0) {
		const end = Math.min((token?.index ?? 0) + adjustment, len);
		const chunk = str.slice(index, end);
		if (chunk) {
			arr.push(chunk);
			index = end;
		}
	}
	function track(char: string) {
		let brace: string = char;
		const balancer = balancers.at(-1)!;

		if (char === '${') {
			state = STATE_DEFAULT;
			brace = '{';
		}
		if (state !== STATE_DEFAULT) return;

		if (balancer[brace as keyof Balancer] !== undefined) {
			balancer[brace as keyof Balancer]++;
		} else if (brace === '}') {
			balancer['{']--;
			if (balancers.length > 1 && isBalanced(balancer)) {
				state = STATE_TEMPLATE_LITERAL;
			}
		} else if (brace === ')') {
			balancer['(']--;
		} else if (brace === ']') {
			balancer['[']--;
		}

		flush();
		if (char === '\n' && token!.index === len) return;
		arr.push(char);
		index = token!.index + char.length;
	}
	function match(...opts: AcceptCondition[]) {
		while ((token = SPECIAL_CHARS_RE.exec(str!))) {
			const c1 = token[0];
			if (c1 === '\\') {
				skip();
				continue;
			}
			for (const fn of opts) {
				if (fn === eof) continue;
				if (fn(c1)) return;
			}
			continue;
		}
		if (opts.at(-1) === eof) {
			eof();
		}
	}
	while ((token = SPECIAL_CHARS_RE.exec(str))) {
		try {
			const char = token[0];

			if (char === '\\') {
				skip();
			} else if (char === '//') {
				match(isEOL);
				flush();
			} else if (char === '/*') {
				match(equal('*/'), eof);
				flush(2);
			} else if (/^\d+$/.test(char)) {
				match(equal(/[^\d\.e]|\//));
				flush();
				SPECIAL_CHARS_RE.lastIndex--;
			} else if (char === '/') {
				const prev = arr
					.slice(0)
					.reverse()
					.find((v) => v.trim() && !v.startsWith('/'))!;
				// TODO: are these triggers comprehensive?
				// https://262.ecma-international.org/13.0/#sec-ecmascript-language-lexical-grammar
				if (/[{(\[;,]/.test(prev)) {
					match(equal('/'), isEOL);
				} else {
					arr.push(char);
					index = token!.index + char.length;
					continue;
				}
			} else if (char === '"' || char === "'") {
				match(equal(char), isEOL);
			} else if (char === '`') {
				if (balancers.length === 1) {
					balancers.push(balancer());
				} else {
					balancers.pop();
				}
				state = balancers.length === 1 ? STATE_DEFAULT : STATE_TEMPLATE_LITERAL;
				flush();
				arr.push(char);
				index = token!.index + char.length;
			} else {
				track(char);
			}
		} catch {}
	}

	return arr;
}
