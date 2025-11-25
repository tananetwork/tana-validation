# tana-validation

shared validation logic for Tana smart contracts with WebAssembly support.

## overview

tana-validation provides error formatting and validation utilities that work across both Rust and TypeScript environments. by writing core logic once in Rust and compiling to WebAssembly, the library ensures consistent behavior everywhere.

## how it works

the library exports a single error formatting function that produces Rust/Gleam-style error messages with precise source locations. whether validation runs in the native Rust runtime or the browser-based playground, users see identical, helpful error output.

### error format

```
Validation Error
Invalid Import

  contract.ts:1:26
  
  1  import { console } from 'tana/invalid';
                              ^^^^^^^^^^^^ module 'tana/invalid' not found

  help: available modules: tana/core, tana/kv
```

### dual compilation

the same Rust code compiles to:

- **native binary** - used by tana-runtime and tana-edge
- **WebAssembly** - used by the playground and CLI tools

this eliminates the possibility of validation behavior diverging between development and production environments.

## architecture

written in Rust with wasm-pack for WebAssembly compilation. the library has zero dependencies beyond the standard library, keeping bundle size minimal (~21KB WASM).

### type definitions

TypeScript type definitions are generated automatically during the WASM build process, providing full type safety for JavaScript consumers.

## integration

tana-validation is used by:

- **tana-runtime** - validates contracts before on-chain execution
- **tana-edge** - validates contracts before HTTP handler execution
- **playground** - validates contracts in the browser before deployment
- **CLI** - validates contracts during local development

all validation errors look the same regardless of where they originate, providing a consistent developer experience.
