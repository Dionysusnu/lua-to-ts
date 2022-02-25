# lua-to-ts

This tool converts Lua code to TS automatically, including the conversion of common standards to their TS equivalents.
Code that fails to be converted will be transformed into a call to `error` with some information regarding the reason it failed.

### Known issues
- swc_ecma_codegen, the tool used for outputting TypeScript, does not format much. The output may look ugly. Just run prettier.
- Lua truthiness is not converted! You should run roblox-ts with the `--logTruthyChanges` option and fix anything it points out.
- Arrow functions (generated from anonymous functions in Lua) do not render their return types. This seems to be a bug with swc's emitter.
- The text inside "failed to convert" error calls may currently be wrong due to a [full_moon bug](https://github.com/Kampfkarren/full-moon/issues/161)
- All variable declarations are `let`, regardless of redeclarations. Use eslint's `prefer-const` rule.
- Type annotations from `for a: number in b do` are preserved, but disallowed by TS. They do parse correctly, but throw an error. Generally, these are unnecessary and can just be removed, but they are transformed for the sake of completeness.

### Unsupported features
- Lua multiple assignments (`a, b = 1, 2`)
- For loops with multiple expressions (`for _ in a, b do`)
- Numeric for loops with a non-literal step argument (`for i = 1, 10, a do`)
