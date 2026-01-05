---@meta

---@class Wezterm
local wezterm = {}

---@alias UserDataRef any
---@alias Value any
---@alias SplitPane any
---@alias SpawnTab any
---@alias SpawnWindow any

wezterm.url = {}
wezterm.mux = {}
wezterm.metrics = {}
wezterm.color = {}
wezterm.plugin = {}
wezterm.time = {}
wezterm.gui = {}
wezterm.procinfo = {}

---@class wezterm.Time
wezterm.Time = {}

---@param format string
---@return string
function wezterm.Time:format_utc(format) end

---@param lat number
---@param lon number
---@return any --[[ tbl --]]
function wezterm.Time:sun_times(lat, lon) end

---@param format string
---@return string
function wezterm.Time:format(format) end


---@class wezterm.MuxPane
wezterm.MuxPane = {}

---@return any --[[ mux .resolve_pane_id(this.0) .map(|(_domain_id, window_id, _tab_id)| MuxWindow(window_id)) --]]
function wezterm.MuxPane:window() end

---@return any --[[ mux .resolve_pane_id(this.0) .map(|(_domain_id, _window_id, tab_id)| MuxTab(tab_id)) --]]
function wezterm.MuxPane:tab() end

---@return any --[[ pane.has_unseen_output() --]]
function wezterm.MuxPane:has_unseen_output() end

---@return any --[[ pane.tty_name() --]]
function wezterm.MuxPane:get_tty_name() end

---@return any --[[ pane .get_current_working_dir(CachePolicy::FetchImmediate) .map(|url| Url { url }) --]]
function wezterm.MuxPane:get_current_working_dir() end

---@param nlines number?
---@return any --[[ text --]]
function wezterm.MuxPane:get_logical_lines_as_text(nlines) end

---@return any --[[ pane.is_alt_screen_active() --]]
function wezterm.MuxPane:is_alt_screen_active() end

---@param nlines number?
---@return any --[[ text --]]
function wezterm.MuxPane:get_lines_as_text(nlines) end

---@return any --[[ pane.get_foreground_process_name(CachePolicy::FetchImmediate) --]]
function wezterm.MuxPane:get_foreground_process_name() end

---@return any --[[ pane.get_cursor_position() --]]
function wezterm.MuxPane:get_cursor_position() end

---@return any --[[ pane.copy_user_vars() --]]
function wezterm.MuxPane:get_user_vars() end

---@param x number
---@param y isize
---@return any --[[ Match --]]
function wezterm.MuxPane:get_semantic_zone_at(x, y) end

---@param text string
function wezterm.MuxPane:paste(text) end

---@param text string
function wezterm.MuxPane:send_paste(text) end

---@param args SplitPane?
---@return any --[[ Await --]]
function wezterm.MuxPane:split(args) end

---@param nlines number?
---@return any --[[ text --]]
function wezterm.MuxPane:get_lines_as_escapes(nlines) end

---@return any --[[ Match --]]
function wezterm.MuxPane:get_domain_name() end

---@param text string
function wezterm.MuxPane:inject_output(text) end

---@return any --[[ pane.get_foreground_process_info(CachePolicy::AllowStale) --]]
function wezterm.MuxPane:get_foreground_process_info() end

---@param zone Value
---@return any --[[ this.get_text_from_semantic_zone(zone) --]]
function wezterm.MuxPane:get_text_from_semantic_zone(zone) end

---@param of_type Value
---@return any --[[ zones --]]
function wezterm.MuxPane:get_semantic_zones(of_type) end

---@return any --[[ pane.get_title() --]]
function wezterm.MuxPane:get_title() end

---@param workspace string?
---@return any
---@return any
function wezterm.MuxPane:move_to_new_window(workspace) end

function wezterm.MuxPane:activate() end

---@param start_x number
---@param start_y isize
---@param end_x number
---@param end_y isize
---@return any --[[ this.get_text_from_semantic_zone(zone) --]]
function wezterm.MuxPane:get_text_from_region(start_x, start_y, end_x, end_y) end

---@param text string
function wezterm.MuxPane:send_text(text) end

---@return any --[[ lua.to_value(&progress) --]]
function wezterm.MuxPane:get_progress() end

---@return any --[[ Ok(*this) --]]
function wezterm.MuxPane:mux_pane() end

---@return any
function wezterm.MuxPane:get_metadata() end

---@return any
---@return any
function wezterm.MuxPane:move_to_new_tab() end

---@return any --[[ Ok(this.0) --]]
function wezterm.MuxPane:pane_id() end

---@return any --[[ pane.get_dimensions() --]]
function wezterm.MuxPane:get_dimensions() end


---@class wezterm.GuiWin
wezterm.GuiWin = {}

---@return any --[[ Ok(this.mux_window_id) --]]
function wezterm.GuiWin:window_id() end

---@return any --[[ window.get_active().map(|tab| mux_lua::MuxTab(tab.tab_id())) --]]
function wezterm.GuiWin:active_tab() end

---@param text string
---@param clipboard ClipboardCopyDestination?
function wezterm.GuiWin:copy_to_clipboard(text, clipboard) end

---@return any --[[ result --]]
function wezterm.GuiWin:is_focused() end

---@return any --[[ result --]]
function wezterm.GuiWin:composition_status() end

---@return any
function wezterm.GuiWin:mux_window() end

---@param value Value
function wezterm.GuiWin:set_config_overrides(value) end

---@param width number
---@param height number
function wezterm.GuiWin:set_inner_size(width, height) end

---@param assignment KeyAssignment
---@param pane UserDataRef
---@return any --[[ result.map_err(mlua::Error::external) --]]
function wezterm.GuiWin:perform_action(assignment, pane) end

---@return any --[[ result --]]
function wezterm.GuiWin:active_key_table() end

---@param status string
function wezterm.GuiWin:set_left_status(status) end

---@return string
function wezterm.GuiWin:active_workspace() end

---@param title string
---@param message string
---@param url string?
---@param timeout number?
function wezterm.GuiWin:toast_notification(title, message, url, timeout) end

---@return any --[[ (*config).clone() --]]
function wezterm.GuiWin:effective_config() end

---@return string
---@return string
function wezterm.GuiWin:keyboard_modifiers() end

function wezterm.GuiWin:restore() end

---@return any --[[ result --]]
function wezterm.GuiWin:leader_is_active() end

---@return any
function wezterm.GuiWin:current_event() end

---@param x isize
---@param y isize
function wezterm.GuiWin:set_position(x, y) end

---@param pane UserDataRef
---@return any --[[ text --]]
function wezterm.GuiWin:get_selection_text_for_pane(pane) end

---@return string
function wezterm.GuiWin:get_appearance() end

---@return any
function wezterm.GuiWin:get_config_overrides() end

---@return any --[[ dims --]]
function wezterm.GuiWin:get_dimensions() end

---@param pane UserDataRef
---@return any --[[ result --]]
function wezterm.GuiWin:get_selection_escapes_for_pane(pane) end

---@param status string
function wezterm.GuiWin:set_right_status(status) end

function wezterm.GuiWin:toggle_fullscreen() end

function wezterm.GuiWin:focus() end

function wezterm.GuiWin:maximize() end

---@return any --[[ result --]]
function wezterm.GuiWin:active_pane() end


---@class wezterm.MuxTab
wezterm.MuxTab = {}

function wezterm.MuxTab:activate() end

---@return any --[[ result --]]
function wezterm.MuxTab:panes_with_info() end

---@return any --[[ Ok(this.0) --]]
function wezterm.MuxTab:tab_id() end

---@return any --[[ None --]]
function wezterm.MuxTab:window() end

---@return any
function wezterm.MuxTab:get_size() end

---@param title string
---@return any --[[ tab.set_title(&title) --]]
function wezterm.MuxTab:set_title(title) end

---@param direction Value
---@return any --[[ pane --]]
function wezterm.MuxTab:get_pane_direction(direction) end

---@param zoomed boolean
---@return any --[[ was_zoomed --]]
function wezterm.MuxTab:set_zoomed(zoomed) end

---@return any --[[ tab .iter_panes_ignoring_zoom() .into_iter() .map(|info| MuxPane(info.pane.pane_id())) .collect::<Vec<MuxPane>>() --]]
function wezterm.MuxTab:panes() end

---@return any --[[ tab.get_title() --]]
function wezterm.MuxTab:get_title() end

function wezterm.MuxTab:rotate_clockwise() end

---@return any --[[ tab.get_active_pane().map(|pane| MuxPane(pane.pane_id())) --]]
function wezterm.MuxTab:active_pane() end

function wezterm.MuxTab:rotate_counter_clockwise() end


---@class wezterm.MuxWindow
wezterm.MuxWindow = {}

---@return string
function wezterm.MuxWindow:get_title() end

---@return any --[[ Await --]]
function wezterm.MuxWindow:gui_window() end

---@return any --[[ window .iter() .map(|tab| MuxTab(tab.tab_id())) .collect::<Vec<MuxTab>>() --]]
function wezterm.MuxWindow:tabs() end

---@return string
function wezterm.MuxWindow:get_workspace() end

---@param new_name string
---@return any --[[ window.set_workspace(&new_name) --]]
function wezterm.MuxWindow:set_workspace(new_name) end

---@return any --[[ Ok(this.0) --]]
function wezterm.MuxWindow:window_id() end

---@return any --[[ result --]]
function wezterm.MuxWindow:tabs_with_info() end

---@param title string
---@return any --[[ window.set_title(&title) --]]
function wezterm.MuxWindow:set_title(title) end

---@return any --[[ window .get_active() .and_then(|tab| tab.get_active_pane().map(|pane| MuxPane(pane.pane_id()))) --]]
function wezterm.MuxWindow:active_pane() end

---@param spawn SpawnTab
---@return any --[[ Await --]]
function wezterm.MuxWindow:spawn_tab(spawn) end

---@return any --[[ window.get_active().map(|tab| MuxTab(tab.tab_id())) --]]
function wezterm.MuxWindow:active_tab() end


---@class wezterm.ColorWrap
wezterm.ColorWrap = {}

---@param amount number
---@return any --[[ this.saturate_fixed(amount) --]]
function wezterm.ColorWrap:saturate_fixed(amount) end

---@param factor number
---@return any --[[ Ok(this.saturate(factor)) --]]
function wezterm.ColorWrap:saturate(factor) end

---@param amount number
---@return any --[[ this.lighten_fixed(-amount) --]]
function wezterm.ColorWrap:darken_fixed(amount) end

---@return any --[[ Ok(this.0.to_laba()) --]]
function wezterm.ColorWrap:laba() end

---@param other UserDataRef
---@return any --[[ this.0.delta_e(&other.0) --]]
function wezterm.ColorWrap:delta_e(other) end

---@return any --[[ Ok(this.square()) --]]
function wezterm.ColorWrap:square() end

---@return any --[[ Ok(this.0.to_hsla()) --]]
function wezterm.ColorWrap:hsla() end

---@return any --[[ Ok(this.triad()) --]]
function wezterm.ColorWrap:triad() end

---@return any --[[ Ok(this.0.to_srgb_u8()) --]]
function wezterm.ColorWrap:srgba_u8() end

---@param amount number
---@return any --[[ this.adjust_hue_fixed(amount) --]]
function wezterm.ColorWrap:adjust_hue_fixed(amount) end

---@param factor number
---@return any --[[ this.saturate(-factor) --]]
function wezterm.ColorWrap:desaturate(factor) end

---@param factor number
---@return any --[[ Ok(this.lighten(-factor)) --]]
function wezterm.ColorWrap:darken(factor) end

---@param amount number
---@return any --[[ this.adjust_hue_fixed_ryb(amount) --]]
function wezterm.ColorWrap:adjust_hue_fixed_ryb(amount) end

---@return any --[[ Ok(this.complement()) --]]
function wezterm.ColorWrap:complement() end

---@param amount number
---@return any --[[ this.lighten_fixed(amount) --]]
function wezterm.ColorWrap:lighten_fixed(amount) end

---@return any --[[ Ok(this.complement_ryb()) --]]
function wezterm.ColorWrap:complement_ryb() end

---@return any
---@return any
---@return any
---@return any
function wezterm.ColorWrap:linear_rgba() end

---@param factor number
---@return any --[[ Ok(this.lighten(factor)) --]]
function wezterm.ColorWrap:lighten(factor) end

---@param other UserDataRef
---@return any --[[ Ok(this.0.contrast_ratio(&other.0)) --]]
function wezterm.ColorWrap:contrast_ratio(other) end

---@param amount number
---@return any --[[ this.saturate_fixed(-amount) --]]
function wezterm.ColorWrap:desaturate_fixed(amount) end


---@class wezterm.Value
wezterm.Value = {}

---@return any --[[ alias cycle: Result --]]
function wezterm.Value:__wezterm_to_dynamic() end


---@class wezterm.MuxDomain
wezterm.MuxDomain = {}

---@param window UserDataRef?
---@return any --[[ domain.attach(window.map(|w| w.0)).await.map_err(|err| { mlua::Error::external(format!( "failed to attach domain {}: {err:#}", domain.domain_name() )) }) --]]
function wezterm.MuxDomain:attach(window) end

---@return any --[[ have_panes_in_domain --]]
function wezterm.MuxDomain:has_any_panes() end

---@return any --[[ domain.detach().map_err(|err| { mlua::Error::external(format!( "failed to detach domain {}: {err:#}", domain.domain_name() )) }) --]]
function wezterm.MuxDomain:detach() end

---@return any --[[ Ok(this.0) --]]
function wezterm.MuxDomain:domain_id() end

---@return any --[[ Match --]]
function wezterm.MuxDomain:state() end

---@return any --[[ Await --]]
function wezterm.MuxDomain:label() end

---@return any --[[ domain.spawnable() --]]
function wezterm.MuxDomain:is_spawnable() end

---@return string
function wezterm.MuxDomain:name() end


---@param myself Table
---@param key Value
---@return Value
function wezterm.__index(myself, key) end

---@param text string
---@return any --[[ alias cycle: Result --]]
function wezterm.json_decode(text) end

---@param args Vec
---@return any --[[ alias cycle: Result --]]
function wezterm.run_child_process(args) end

---@return any
function wezterm.default_hyperlink_rules() end

---@param config_files ... string
---@return any --[[ alias cycle: Result --]]
function wezterm.enumerate_ssh_hosts(...) end

---@param args ... string
---@return nil
function wezterm.add_to_config_reload_watch_list(...) end

---@param mux_window_id MuxWindowId
---@return any --[[ win --]]
function wezterm.gui.gui_window_for_mux_window(mux_window_id) end

---@param old_workspace string
---@param new_workspace string
function wezterm.mux.rename_workspace(old_workspace, new_workspace) end

---@param format string
---@return any --[[ alias cycle: Result --]]
function wezterm.strftime_utc(format) end

---@param spawn SpawnWindow
---@return any --[[ Await --]]
function wezterm.mux.spawn_window(spawn) end

---@return any --[[ counters --]]
function wezterm.metrics.get_latency() end

---@param pid number
---@return any --[[ LocalProcessInfo::executable_path(pid) .and_then(|p| p.to_str().map(|s| s.to_string())) --]]
function wezterm.procinfo.executable_path_for_pid(pid) end

---@return any --[[ fe.gui_windows() --]]
function wezterm.gui.gui_windows() end

---@param name string
---@param fixup_command function
---@param label Value?
---@return ExecDomain
function wezterm.exec_domain(name, fixup_command, label) end

---@param attrs LuaFontAttributes
---@param map_defaults TextStyleAttributes?
---@return TextStyle
function wezterm.font(attrs, map_defaults) end

---@return any --[[ gpus --]]
function wezterm.gui.enumerate_gpus() end

---@param h number
---@param s number
---@param l number
---@param a number
---@return any
function wezterm.color.from_hsla(h, s, l, a) end

---@param gradient Gradient
---@param num_colors number
---@return any --[[ alias cycle: Result --]]
function wezterm.gradient(gradient, num_colors) end

---@param args ... Value
function wezterm.log_warn(...) end

---@param args ... Value
function wezterm.print(...) end

---@param pane_id number
---@return any --[[ pane --]]
function wezterm.mux.get_pane(pane_id) end

---@param name string
---@return any --[[ KeyAssignment::variants().contains(&name.as_str()) --]]
function wezterm.has_action(name) end

---@param file_name string
---@return any
---@return any
function wezterm.color.load_terminal_sexy_scheme(file_name) end

---@param tab_id number
---@return any --[[ tab --]]
function wezterm.mux.get_tab(tab_id) end

---@param pid number
---@return any --[[ Ok(LocalProcessInfo::with_root_pid(pid)) --]]
function wezterm.procinfo.get_info_for_pid(pid) end

---@param text string
---@return any --[[ alias cycle: Result --]]
function wezterm.yaml_decode(text) end

---@param value LuaValue
---@return any --[[ alias cycle: Result --]]
function wezterm.yaml_encode(value) end

---@param args Vec
---@return any --[[ alias cycle: Result --]]
function wezterm.background_child_process(args) end

---@param format string
---@return any --[[ alias cycle: Result --]]
function wezterm.strftime(format) end

---@param s string
---@return any --[[ Struct --]]
function wezterm.url.parse(s) end

function wezterm.plugin.update_all() end

---@param gradient Gradient
---@param num_colors number
---@return any --[[ alias cycle: Result --]]
function wezterm.gradient_colors(gradient, num_colors) end

---@param value LuaValue
---@return any --[[ alias cycle: Result --]]
function wezterm.json_encode(value) end

---@param value LuaValue
---@return any --[[ alias cycle: Result --]]
function wezterm.json_encode_pretty(value) end

---@param args ... Value
function wezterm.log_info(...) end

---@param line string
---@return Vec
function wezterm.shell_split(line) end

---@param pattern string
---@param path string?
---@return any --[[ alias cycle: Result --]]
function wezterm.glob(pattern, path) end

---@return any --[[ mux.iter_workspaces() --]]
function wezterm.mux.get_workspace_names() end

---@param spec string
---@return any --[[ alias cycle: Result --]]
function wezterm.parse(spec) end

---@param items Vec
---@return any --[[ alias cycle: Result --]]
function wezterm.format(items) end

---@param s string
---@param width number
---@return any --[[ Ok(pad_right(s, width)) --]]
function wezterm.pad_right(s, width) end

---@param interval_seconds number
---@param func function
function wezterm.time.call_after(interval_seconds, func) end

---@param text string
---@return any --[[ alias cycle: Result --]]
function wezterm.json_parse(text) end

---@param s string
---@return any --[[ Struct --]]
function wezterm.time.parse_rfc3339(s) end

---@param domain UserDataRef
function wezterm.mux.set_default_domain(domain) end

---@param _ nil
---@return string
function wezterm.hostname(_) end

---@return any --[[ mux .iter_windows() .into_iter() .map(MuxWindow) .collect::<Vec<MuxWindow>>() --]]
function wezterm.mux.all_windows() end

---@param callback function
---@return KeyAssignment
function wezterm.action_callback(callback) end

---@param workspace string
---@return any --[[ If --]]
function wezterm.mux.set_active_workspace(workspace) end

---@return any --[[ mux .iter_domains() .into_iter() .map(|dom| MuxDomain(dom.domain_id())) .collect::<Vec<MuxDomain>>() --]]
function wezterm.mux.all_domains() end

---@return any
function wezterm.plugin.list() end

---@param _ nil
---@return any --[[ alias cycle: Result --]]
function wezterm.battery_info(_) end

---@return any --[[ counters --]]
function wezterm.metrics.get_throughput() end

---@return any --[[ Ok(config::COLOR_SCHEMES.clone()) --]]
function wezterm.get_builtin_color_schemes() end

---@return any --[[ Ok(config::COLOR_SCHEMES.clone()) --]]
function wezterm.color.get_builtin_schemes() end

---@return any
function wezterm.gui.default_keys() end

---@param args ... Value
function wezterm.log_error(...) end

---@param s string
---@param width number
---@return any --[[ Ok(pad_left(s, width)) --]]
function wezterm.pad_left(s, width) end

---@param s string
---@param fmt string
---@return any --[[ Struct --]]
function wezterm.time.parse(s, fmt) end

---@param pid number
---@return any --[[ LocalProcessInfo::current_working_dir(pid) .and_then(|p| p.to_str().map(|s| s.to_string())) --]]
function wezterm.procinfo.current_working_dir_for_pid(pid) end

---@param window_id number
---@return any --[[ window --]]
function wezterm.mux.get_window(window_id) end

---@param item Table
---@return any --[[ alias cycle: Result --]]
function wezterm.permute_any_or_no_mods(item) end

---@param name string
---@param func function
---@return nil
function wezterm.on(name, func) end

---@return any --[[ Match --]]
function wezterm.gui.get_appearance() end

---@return any
function wezterm.gui.default_key_tables() end

---@param value LuaValue
---@return any --[[ alias cycle: Result --]]
function wezterm.toml_encode(value) end

---@param url string
---@param app string?
---@return any --[[ alias cycle: Result --]]
function wezterm.open_with(url, app) end

---@return any --[[ Ok(config::SshDomain::default_domains()) --]]
function wezterm.default_ssh_domains() end

---@param item Table
---@return any --[[ alias cycle: Result --]]
function wezterm.permute_any_mods(item) end

---@param args Vec
---@return string
function wezterm.shell_join_args(args) end

---@param arg Value
---@return string
function wezterm.to_string(arg) end

---@return any --[[ palette --]]
function wezterm.color.get_default_colors() end

---@return any --[[ Ok(unsafe { libc::getpid() }) --]]
function wezterm.procinfo.pid() end

---@param text string
---@return any --[[ alias cycle: Result --]]
function wezterm.toml_decode(text) end

---@param s string
---@param max_width number
---@return any
function wezterm.truncate_right(s, max_width) end

---@return any --[[ Ok(Time { utc: Utc::now() }) --]]
function wezterm.time.now() end

---@return any --[[ screens --]]
function wezterm.gui.screens() end

---@param file_name string
---@return any
---@return any
function wezterm.color.load_scheme(file_name) end

---@return any --[[ config --]]
function wezterm.config_builder() end

---@param value LuaValue
---@return any --[[ alias cycle: Result --]]
function wezterm.toml_encode_pretty(value) end

---@param text string
---@return string
function wezterm.utf16_to_utf8(text) end

---@param domain LuaValue
---@return any --[[ Match --]]
function wezterm.mux.get_domain(domain) end

---@return any --[[ Ok(crate::WslDomain::default_domains()) --]]
function wezterm.default_wsl_domains() end

---@param fallback Vec
---@param map_defaults TextStyleAttributes?
---@return TextStyle
function wezterm.font_with_fallback(fallback, map_defaults) end

function wezterm.reload_configuration() end

---@param repo_spec string
---@return any --[[ require_plugin(lua, repo_spec).map_err(|e| mlua::Error::external(format!("{e:#}"))) --]]
function wezterm.plugin.require(repo_spec) end

---@param file_name string
---@return any
---@return any
function wezterm.color.load_base16_scheme(file_name) end

---@return any --[[ counters --]]
function wezterm.metrics.get_sizes() end

---@return any --[[ Ok(crate::running_under_wsl()) --]]
function wezterm.running_under_wsl() end

---@return any --[[ counters --]]
function wezterm.metrics.get_counters() end

---@param text string
---@return Vec
function wezterm.split_by_newlines(text) end

---@param s string
---@param max_width number
---@return any --[[ Ok(truncate_left(&s, max_width)) --]]
function wezterm.truncate_left(s, max_width) end

---@param myself Table
---@param key string
---@param value Value
---@return nil
function wezterm.__newindex(myself, key, value) end

---@param file_name string
---@param params ExtractColorParams?
---@return any --[[ alias cycle: Result --]]
function wezterm.extract_colors_from_image(file_name, params) end

---@param colors Palette
---@param metadata ColorSchemeMetaData
---@param file_name string
---@return any --[[ scheme .save_to_file(file_name) .map_err(|err| mlua::Error::external(format!("{err:#}"))) --]]
function wezterm.color.save_scheme(colors, metadata, file_name) end

---@param arg string
---@return string
function wezterm.shell_quote_arg(arg) end

---@param s string
---@return any --[[ Ok(unicode_column_width(&s, None)) --]]
function wezterm.column_width(s) end

---@param name string
---@param ... any
---@return boolean
function wezterm.emit(name, ...) end

---@param milliseconds number
---@return any --[[ alias cycle: Result --]]
function wezterm.sleep_ms(milliseconds) end

---@param path string
---@return any --[[ alias cycle: Result --]]
function wezterm.read_dir(path) end

---@param myself Table
---@param strict boolean
---@return nil
function wezterm.set_strict_mode(myself, strict) end

---@param env string
---@return string?
function wezterm.getenv(env) end

---@return any --[[ mux.active_workspace() --]]
function wezterm.mux.get_active_workspace() end


return wezterm
