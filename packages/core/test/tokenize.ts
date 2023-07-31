import { tokenize } from '../src/index.js';
import { describe, expect, it } from "vitest";

describe("tokenize", () => {
  it("console log", async () => {
    const input = `console.log("Hello <name>!")`;
    expect(tokenize(input)).toEqual(["console", ".", "log", "(", `"Hello <name>!"`, ")"]);
  });
  it("comment", async () => {
    const input = `// console.log("Hello <name>!")`;
    expect(tokenize(input)).toEqual(["// console.log(\"Hello <name>!\")"]);
  });
  it("trailing comment", async () => {
    const input = `foo.call(); // console.log("Hello <name>!")`;
    expect(tokenize(input)).toEqual(["foo", ".", "call", "(", ")", ";", " ", "// console.log(\"Hello <name>!\")"]);
  });
  it("multiline comment", async () => {
    const input = `/* foo.call();\nahhhhhhh*/`;
    expect(tokenize(input)).toEqual([input]);
  });
  it("JSON stringify", async () => {
    const input = `JSON.stringify({k:3.14**2}, null /*replacer*/, "\\t");\n// cool\n`;
    expect(tokenize(input)).toEqual(["JSON", ".", "stringify", "(", "{", "k", ":", "3.14", "**", "2", "}", ",", " ", "null", " ", "/*replacer*/", ",", " ", `"\\t"`, ")", ";", "\n", "// cool", "\n"]);
  });
  it("template", () => {
    const input = "{ value: `a\\`{{{${''}${a++}${ `${b\r}` + `${`c${5}`}` } d $${\n(x => { return x*2 })(4)} }}\\${}` }";
    expect(tokenize(input)).toEqual(["{", " ", "value", ":", " ", "`", "a\\`{{{", "${", "''", "}", "${", "a", "++", "}", "${", " ", "`", "${", "b", "\r", "}", "`", " + ", "`", "${", "`", "c", "${", "5", "}", "`", "}", "`", " } d $", "${", "\n", "(", "x", " ", "=>", " ", "{", " ", "return", " ", "x", "*", "2", " ", "}", ")", "(", "4", ")", "}", " }}\\${}", "`", " ", "}"]);
  })
  it("import meta", () => {
    const input = "import.meta.glob()";
    expect(tokenize(input)).toEqual(["import", ".", "meta", ".", "glob", "(", ")"]);
  })
  it('decimals', () => {
    const input = ".123";
    expect(tokenize(input)).toEqual([".123"]);
  })
  it('scientific notation', () => {
    const input = "1e0.123";
    expect(tokenize(input)).toEqual(["1e0.123"]);
  })
});
