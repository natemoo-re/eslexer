import './style.css'
import init, { findRanges } from 'eslexer'

const app = document.querySelector<HTMLDivElement>('#app')!

const fixture = `
// import.meta.url()
import.meta.url();
import
  .meta
  .url();
import
  .meta
  // what?
  .url();
`;

async function run() {
  await init();
  const tokens = await test_wasm();
  console.log(tokens);
  tokens.forEach((range) => {
    const { start, end }= range;
    console.log(fixture.slice(start.offset, end.offset))
  })
}

async function test_wasm() {
  const start = performance.now()
  const tokens = findRanges(fixture, 'import.meta.url(*)')
  const end = performance.now()
  console.log(`WASM done in`, end - start);
  return JSON.parse(tokens);
}

run();

app.innerHTML = `
  <h1>Hello Vite!</h1>
  <a href="https://vitejs.dev/guide/features.html" target="_blank">Documentation</a>
`
