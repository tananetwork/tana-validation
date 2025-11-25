# tana-validation

Shared validation and error formatting logic for Tana smart contracts.

## Features

- ✅ **Single source of truth** - Write error formatting logic once in Rust
- ✅ **Works everywhere** - Compiles to native Rust and WebAssembly
- ✅ **Beautiful errors** - Rust/Gleam-style error messages with precise location info
- ✅ **Tiny bundle** - Only ~21KB WASM + 5KB JS wrapper
- ✅ **Zero dependencies** - Fully self-contained

## Usage

### TypeScript/JavaScript (via WASM)

```bash
npm install @tananetwork/tana-validation
# or
bun add @tananetwork/tana-validation
```

```typescript
import init, { format_validation_error } from '@tananetwork/tana-validation';

// Initialize WASM module
await init();

// Format an error
const error = format_validation_error(
  "import { console } from 'tana/invalid';",  // code
  "contract.ts",                               // file_path
  "Invalid Import",                            // error_kind
  1,                                          // line_num
  26,                                         // col_num
  "Module 'tana/invalid' not found",         // message
  "Available modules: tana/core, tana/kv",   // help
  12                                          // underline_length
);

console.log(error);
```

### Rust

```toml
[dependencies]
tana-validation = "0.1"
```

```rust
use tana_validation::format_validation_error;

let error = format_validation_error(
    "import { console } from 'tana/invalid';",
    "contract.ts",
    "Invalid Import",
    1,
    26,
    "Module 'tana/invalid' not found",
    "Available modules: tana/core, tana/kv",
    12,
);

println!("{}", error);
```

## Output Format

Both Rust and TypeScript/WASM produce identical output:

```
Validation Error
❌ Invalid Import

┌─ contract.ts:1:26
│
  1 │ import { console } from 'tana/invalid';
    │                          ^^^^^^^^^^^^ Module 'tana/invalid' not found
│
= help: Available modules: tana/core, tana/kv
│
```

## Why WASM?

By writing the error formatter once in Rust and compiling to WASM for TypeScript, we get:

1. **Guaranteed consistency** - Same code = same output
2. **No drift** - Impossible for implementations to diverge
3. **Native performance** - WASM is fast
4. **Small bundle** - Rust compiles to efficient WASM
5. **Type safety** - TypeScript definitions generated automatically

## Used By

- **tana-runtime** - On-chain contract execution (native Rust)
- **tana-edge** - HTTP contract server (native Rust)
- **playground** - Browser-based contract testing (WASM)
- **CLI tools** - Command-line validation (WASM via Bun)

## Development

```bash
# Test Rust code
cargo test

# Build WASM package
wasm-pack build --target bundler --scope tananetwork

# Test WASM in browser
cd pkg && npm link
cd your-project && npm link @tananetwork/tana-validation
```

## License

Dual-licensed under MIT OR Apache-2.0.
