# lua-to-ts

This tool converts Lua code to TS automatically, including the conversion of common standards to their TS equivalents.
Code that fails to be converted will be transformed into a call to `error` with some information regarding the reason it failed. Note: The text inside the error may currently be wrong due to a [full_moon bug](https://github.com/Kampfkarren/full-moon/issues/161).

### Known issues
- swc_ecma_codegen, the tool used for outputting TypeScript, does not format much. The output may look ugly. Just run prettier.
- Lua truthiness is not converted! You should enable the roblox-ts eslint plugin's lua-truthiness rule and fix anything it points out.

### Unsupported features
- Lua multiple assignments (`a, b = 1, 2`)
- For loops with multiple expressions (`for _ in a, b do`)
- Numeric for loops with a non-literal step argument (`for i = 1, 10, a do`)
