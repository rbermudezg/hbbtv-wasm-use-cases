// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { parse } from "./pkg/subtitles.js";

const runWasm = async () => {
  await init();
  const res = await fetch("./subtitles.xml");
  const text = await res.text();
  parse(text);

  // Call the Add function export from wasm, save the result
  //const addResult = helloWasm.add(24, 24);

  // Set the result onto the body
  //document.body.textContent = `Hello!: ${addResult}`;
};
runWasm();
