import { Bench } from 'tinybench';
import { readFile } from 'fs/promises';

import jsTokens from 'js-tokens';
import { parse as babel } from '@babel/parser';
import { tokenizer as acorn } from 'acorn'
import { tokenize } from '../src/index.ts';

const bench = new Bench({ });
let fixture = await readFile(new URL('../src/index.ts', import.meta.url), { encoding: 'utf8' });
fixture = fixture.repeat(3);

let totals = {
    eslexer: 0,
    jsTokens: 0,
    babel: 0,
    acorn: 0
}

bench
  .add('eslexer', () => {
    const tokens = tokenize(fixture);
    totals.eslexer = tokens.length;
  })
  .add('js-tokens', () => {
    const tokens = Array.from(jsTokens(fixture));
    totals.jsTokens = tokens.length;
  })
  .add('babel', () => {
    const { tokens } = babel(fixture, {
        tokens: true,
        errorRecovery: true,
        plugins: ["typescript"],
      });
    totals.babel = tokens!.length;
  })
  .add('acorn', () => {
    const tokens = Array.from(acorn(fixture, { ecmaVersion: 'latest' }));
    totals.acorn = tokens!.length;
  })


await bench.run();

console.table(bench.table());
console.table(totals);
