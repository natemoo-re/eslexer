# WIP: `eslexer`

A **WIP** lexical scanner for JavaScript and TypeScript code. Written in Rust, compiled to WASM.

## Why not RegExp?

`RegExp` is not a good solution for scanning JS/TS code! It cannot account for any syntactic nuance in your sourcecode.

For example, you may naively write something like the following to check if a file references `foo`
```js
const matches = /foo/g.test(code);
```

Unfortunately, this would return false positives for any of the following:
```js
// foo
/* foo */
const a = "foo";
const b = 'foo';
const c = `foo`;
```

## Why not a full parse?

Packages like `babel`, `acorn`, and `meriyah` are awesome! I personally use them all the time. But for a straightforward scan of code, they can be overkill. There are two main problems I see with full AST parsing:

- **Performance**. A full AST parse can be expensive on its own, but then you have to add a recursive `walk` pattern to actually traverse the code. This can easily become a bottleneck.
- **Context**. You might not know _what kind of JavaScript_ the sourcefile contains. Is it valid ES2015? Valid ES2020? TypeScript? Maybe the module isn't even valid! That can definitely happen if you're working with a Language Server and the file is still being edited. Parsers aren't great in this situation because they expect a fully valid module and will throw if they encounter some syntax they don't understand.

## Why Lexical Scanning?
Lexical scanning is exciting because it is:

- **Fast**. It's performed in a single linear pass of your file. In the case of `eslexer`, this is performed in WASM for an extra speed boost.
- **Flat**. Lexers output an array of tokens. This makes it straightforward to traverse and manipulate.
- **Forgiving**. Lexers (typically) won't throw on invalid syntax, they'll just output an `Invalid` token. This means your sourcefile is not required to be a fully valid module, which is perfect for _scan-as-users-type_ situations like Language Servers or for files that might contain non-standard syntax.

## Goals / TODO

- Build in support for common use cases like renaming an identifier.
- Build in construct-aware scanning, like variable or function declarations (in all the unique forms they can take). I _think_ tokens will be enough to handle these, but I could be wrong. This is treading pretty close to AST territory.
- Some type of query syntax (similar to `document.querySelector()`). I don't want to use SQL.
