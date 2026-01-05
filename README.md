This directory is added to a downstream repo by the following:

``` sh
git subtree add --prefix=plugin/types/wezterm https://github.com/vike2000/wezterm-lua-defs.git subtree/out --squash
```

and with a clean working directory anyone should be able to pull upstream changes by the following:

``` sh
git subtree pull --prefix=plugin/types/wezterm https://github.com/vike2000/wezterm-lua-defs.git subtree/out --squash
```

`wezterm.d.lua` is a [Lua Language Server definition-file](https://luals.github.io/wiki/definition-files/) file meant to be added to the workspace scope by [workspace.library](https://luals.github.io/wiki/settings/#workspacelibrary) in a [`.luarc.json`](https://luals.github.io/wiki/configuration/#luarcjson-file) or an IDE equivalence e.g. Visual Studio Code extension [@id:sumneko.lua](https://marketplace.visualstudio.com/items?itemName=sumneko.lua) in [Workspace](https://code.visualstudio.com/docs/configure/settings) or [Folder](https://code.visualstudio.com/docs/editing/workspaces/multi-root-workspaces#_settings) settings (`.vscode/settings.json`) key `Lua.workspace.library`.

If the subtree 'connection' seems broken it should be fine to (after any desired `git reset`) then run `rm -r plugin\types\wezterm` and then `git subtree add` as above again.