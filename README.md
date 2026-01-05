### A quick and dirty generator of a [Lua Language Server definition-file for Wezterm plugins using git subtree](out/README.md).<!-- cspell:ignore Wezterm -->

It is my first Rust program, partly written using a personal style (I know it's weird), and partly leaning heavily on copy-paste iterating with ChatGPT.

## Building and running the generator

Currently there's only a Powershell script to automate this, and it's only tested on Windows 11.

Example usage:

* Build and run without git but printing as well as over-writing `out/wezterm.d.lua` by

  ``` pwsh
  cd "$weztermRepoPath"
  &$weztermLuaDefsRepo/out -noGit -print -path out/wezterm.d.lua -- --debug 4 */src lua-api-crates
  ```

  or

  ``` pwsh
  ./out -noGit -print -path out/wezterm.d.lua -- --debug 4 $weztermRepoPath/*/src $weztermRepoPath/lua-api-crates
  ```

* Build and run over-writing `out/wezterm.d.lua`; git add and commit `out/wezterm.d.lua` to main branch; git subtree split, and push to the subtree branch by

  ``` pwsh
  cd "$weztermRepoPath"
  $weztermLuaDefsRepo/out -gitCommitMessage "update out/wezterm.d.lua" */src lua-api-crates
  ```

  or

  ``` pwsh
  ./out $weztermRepoPath/*/src $weztermRepoPath/lua-api-crates
  ```

* Only git add and commit `out/wezterm.d.lua` to main branch; git subtree split, and push to the subtree branch by

  ``` pwsh
  cd "$weztermRepoPath"
  &$weztermLuaDefsRepo/out -subtree -gitCommitMessage "update out/wezterm.d.lua"
  ```

  or

  ``` pwsh
  ./out -subtree
  ```

