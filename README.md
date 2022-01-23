# lua-to-ts

This tool converts Lua code to TS automatically, including the conversion of common standards to their TS equivalents.

### Known issues
- swc_ecma_codegen, the tool used for outputting TypeScript, does not format much. The output may look ugly. Just run prettier.
- Lua truthiness is not converted! You should enable the roblox-ts eslint plugin's lua-truthiness rule and fix anything it points out.

### Unsupported features
- Lua multiple assignments (`a, b = 1, 2`)
