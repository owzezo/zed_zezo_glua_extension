# GLua (Garry's Mod) — Zed extension

A Zed-native equivalent of the VS Code *GLua Enhanced* extension. It brings the
Garry's Mod Lua development experience to Zed by pairing a Lua Tree-sitter
grammar with the **Lua Language Server** and the community **GMod API
definitions** (scraped from the wiki by
[luttje/glua-api-snippets](https://github.com/luttje/glua-api-snippets)).

## Features

- **Syntax highlighting** for `.lua` (Tree-sitter Lua grammar).
- **Autocomplete / IntelliSense** for the whole GMod API via LuaLS + wiki
  definitions.
- **Hover docs** — function args, descriptions, notes/warnings/bugs from the
  wiki.
- **Enum argument autocomplete**, **go-to-definition**, **find references**.
- **Diagnostics** from LuaLS (LuaJIT runtime, GMod globals pre-declared).
- **Snippets** (`hook`, `netstart`, `concommand`, `timer`, `hudpaint`, ...).

The extension auto-downloads both `lua-language-server` and the GMod
definitions on first use (or reuses a `lua-language-server` already on your
`PATH`).

## Install (dev extension)

Requires a Rust toolchain with the wasm target (Zed compiles the extension):

```
rustup target add wasm32-wasip1
```

Then in Zed: `Ctrl-Shift-P` → **zed: install dev extension** → pick this
`zed_zezo_glua_extension/` folder. Zed builds it and loads it. Open a `.lua` file; the status
bar should show **GLua** and, after the one-time download, LuaLS completions.

## Notes & limitations

Zed extensions are sandboxed WASM + LSP, so the VS Code-only UI features do not
port: **no** bytecode heatmap, `Color()` picker, asset browser, or image/material
previews. NetworkVar / net-message workspace scanning is also not carried over —
LuaLS covers general symbols instead.

- `.lua` files are claimed by **GLua** globally. To limit it to GMod projects,
  edit `languages/glua/config.toml` and remove `path_suffixes`, then set the
  language manually per-file, or change the suffix.
- `//` line comments and `!`/`&&`/`||` are declared as non-standard symbols for
  LuaLS, but the Tree-sitter grammar is vanilla Lua — files that use those
  heavily may show minor highlighting glitches around those tokens.
- If GMod completions don't appear, point LuaLS at the definitions manually in
  Zed `settings.json`:

  ```json
  {
    "lsp": {
      "glua-luals": {
        "settings": {
          "Lua": { "workspace": { "library": ["/absolute/path/to/gmod-defs"] } }
        }
      }
    }
  }
  ```
