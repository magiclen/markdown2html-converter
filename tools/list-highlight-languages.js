// Ask the bundled highlight.js itself which languages it registers, so the list can never drift from the build.
//
// Usage: node tools/list-highlight-languages.js resources/highlight.min.js > resources/highlight-languages.txt

const fs = require('fs');
const vm = require('vm');

const bundle = process.argv[2];

if (!bundle) {
    throw new Error('usage: node tools/list-highlight-languages.js <highlight.js-bundle>');
}

// The bundle is a browser build, so it needs something which looks like a window to attach itself to.
const sandbox = {};

sandbox.window = sandbox;
sandbox.self = sandbox;
sandbox.globalThis = sandbox;

vm.createContext(sandbox);
vm.runInContext(fs.readFileSync(bundle, 'utf8'), sandbox);

const hljs = sandbox.hljs;

if (!hljs || typeof hljs.listLanguages !== 'function') {
    throw new Error('the bundle did not expose hljs');
}

const names = hljs.listLanguages().flatMap((name) => [name, ...(hljs.getLanguage(name).aliases || [])]);

process.stdout.write('# The languages which the bundled highlight.min.js registers, aliases included.\n');
process.stdout.write('# Regenerate with: node tools/list-highlight-languages.js resources/highlight.min.js > resources/highlight-languages.txt\n');
process.stdout.write([...new Set(names)].sort().join('\n'));
process.stdout.write('\n');
