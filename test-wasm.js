// Quick WASM test - Test the format_validation_error function
import init, { format_validation_error } from './pkg/tana_validation.js';

// Initialize WASM
await init();

console.log('\n🧪 Testing WASM Error Formatter\n');

// Test 1: Invalid import error
const test1 = format_validation_error(
  "import { console } from 'tana/invalid';\n\nexport async function contract() {\n  return { success: true };\n}",
  "contract.ts",
  "Invalid Import",
  1,
  26,
  "Module 'tana/invalid' not found",
  "Available modules: tana/core, tana/kv, tana/block, tana/tx",
  12
);

console.log('Test 1: Invalid Import');
console.log(test1);

// Test 2: Invalid export error
const test2 = format_validation_error(
  "import { kv } from 'tana/kv';\n\nexport async function notAllowed() {\n  await kv.put('key', 'value');\n}",
  "contract.ts",
  "Invalid Export",
  3,
  24,
  "Function 'notAllowed' is not allowed",
  "Allowed functions: init, contract, get, post",
  10
);

console.log('\nTest 2: Invalid Export');
console.log(test2);

// Test 3: Context error
const test3 = format_validation_error(
  "import { context } from 'tana/context';\n\nexport async function get() {\n  const caller = context.caller();\n  return { caller };\n}",
  "contract.ts",
  "Context Error",
  4,
  16,
  "context.caller() can only be used in init() or contract() functions",
  "HTTP handlers (get/post) cannot access execution context",
  15
);

console.log('\nTest 3: Context Error');
console.log(test3);

console.log('\n✅ All WASM tests completed!\n');
