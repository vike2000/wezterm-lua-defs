param(
#	[Parameter(ParameterSetName='noConfirm')][switch]$noConfirm,
	[Parameter(ParameterSetName='noPrevent')][switch]$noPrevent,
	[Parameter()][switch]$subtreeOnly,
	[Parameter()][switch]$recreateSubtree,
	[Parameter()][switch]$clean,
	[Parameter()][switch]$print,
	[Parameter()][switch]$noGit,
	[Parameter()][string]$gitCommitMessage,
	[Parameter()][string]$path,
	[Parameter(ValueFromRemainingArguments,Position=0)]$arguments
)

$ErrorActionPreference = 'Stop'

$bin = 'generator'
$scheme = 'debug' # $profile is reserved in pwsh

$dir = "out"
if (!$path) {$path = "$PSScriptRoot/$dir/wezterm.d.lua"}

if ($subtreeOnly) {
	$lastExitCode = 0
} else {
	# won't implement cause-of too implicit, hidden and unexpected feature: if ($PSCmdlet.ParameterSetName -ne 'noConfirm') {$noConfirm=$gitCommitMessage.length -gt 0}
	# won't implement cause-of too implicit, hidden and unexpected feature: if ($PSCmdlet.ParameterSetName -ne 'noPrevent') {$noPrevent=$gitCommitMessage.length -gt 0}
	
	push-location $PSScriptRoot
	if ($clean) {
		cargo clean
	}
	cargo build --bin $bin
	pop-location

	$content = env RUST_BACKTRACE=1 "$PSScriptRoot/target/$scheme/$bin" @arguments # $wezterm\*\src\ $wezterm\lua-api-crates\ #cspell:ignore wezterm
}
if ($lastExitCode) {
	[Console]::Error.WriteLine($content -join "`n"); exit($lastExitCode)
} else {
	if (!$subtreeOnly) {
		if ($print) {$content}
		
		$content > $path
	}
	
	if (-not $noGit) {
		# custom simplified variants compared to original at ~vike's profile:
		function confirmable() {if ($noConfirm -or		$(read-host "<# confirm by regex '^y(es)?$' #> $(			$args -join ' ') # confirm") -match		'^y(es)?$'		) {$A=$args[1..($args.length-1)];&$args[0] @A}}
		function preventable() {if ($noPrevent -or -not($(read-host "<# prevent by not regex '^(y(es)?)?$' #> $(	$args -join ' ') # confirm") -notmatch	'^(y(es)?)?$')	) {$A=$args[1..($args.length-1)];&$args[0] @A}}
		
		remove-item -ea 0 alias:pushd	;function pushd{push-location $args[0]
			write-output "get-location after push-location $(get-location)"}
		remove-item -ea 0 alias:popd	;function  popd{ pop-location
			write-output "get-location after pop-location $(get-location)"}
		
		$subtreePath = "$dir"
		$subtreeBranch = "subtree/$dir"
		
		push-location $PSScriptRoot
		if ($recreateSubtree) {
			preventable git branch -D "$subtreeBranch"
		} else {
			preventable git add "$path"
			if ($gitCommitMessage.length) {
				preventable git commit -m "$gitCommitMessage"
			} else {
				preventable git commit # will open editor (like env $EDITOR on *nix/bash)
			}
		}
		preventable git subtree split --prefix="$subtreePath" -b "$subtreeBranch"
		preventable git push -f origin "$subtreeBranch"
		pop-location
	}
}

# ~vike's dev command w/ sourced $PROFILE.CurrentUserAllHosts after out.ps1: &{ $clean=0 ; $debug=0 ; $print=0 ; $git=0 ; $paths=2 ; $weztermLuaDefsRepoPath=$(vpdr ~/file/develop/rust/wezterm-lua-defs) ; $weztermRepoPath=$(vpdr ~/file/develop/git/wezterm) ; $doubleDash=@(if($debug -or -not $subtreeOnly){arrgs --}) ; $debug=@(if($debug){arrgs --debug $debug}) ; $clean=@(if($clean){arrgs -clean}) ; $print=@(if($print){arrgs -print}) ; $git=@(if($git -le 0){arrgs -noGit}elseif($git -eq 1){arrgs -subtreeOnly}else{arrgs -gitCommitMessage "update out/wezterm.d.lua"}) ; $paths=@(if(-not$subtreeOnly){if($paths -eq 2){arrgs $weztermRepoPath/*/src $weztermRepoPath/lua-api-crates}elseif($paths -eq 1){arrgs lua-api-crates\time-funcs\src\lib.rs}else{write-error "bad paths: $paths"}}) ; $subtreeOnly=@(if($subtreeOnly){arrgs -subtreeOnly}) ; _vp $weztermLuaDefsRepoPath/out @clean @print @git @doubleDash @debug @paths }
# ~vike's dev command w/ sourced $PROFILE.CurrentUserAllHosts before out.ps1: &{ $debug=0 ; $saving=1 ; $grep='emit' ; $paths=2 ; $diff=1 ; $suffixes=0 ; $debug=@(if($debug){arrgs --debug 1}) ; $suffixes=@(if($suffixes -eq 2){arrgs -old ''}elseif($suffixes -le 1){arrgs ''}else{write-error "bad suffixes: $suffixes"}) ; $paths=@(if($paths -eq 2){arrgs C:\Users\vike\file\develop\git\wezterm\*\src\ C:\Users\vike\file\develop\git\wezterm\lua-api-crates\}elseif($paths -eq 1){arrgs C:\Users\vike\file\develop\git\wezterm\config\src\lua.rs}else{write-error "bad paths: $paths"}) ; eval(d('$suffixes')) ; echo '--' ; eval(d('$paths')) ; echo '---' ; $diffables=@() ; foreach($suf in $suffixes){$saveable="C:\Users\vike\OneDrive\shell\library\lua\definition\wezterm$suf.d.lua" ; cd1 C:\Users\vike\file\develop\rust\wezterm-lua-defs env RUST_BACKTRACE=1 _v cargo run --bin main$suf -- -- -- @debug -- @paths |&{ if(-not$saving){$input}else{$input|tee $saveable} } |&{ if(-not$grep){$input}else{$input|grep_ps -context 8 $grep} } ; $saveables+=$saveable ; if($diff){$diffables=$diffables[-1..-1]+@($saveable) ; if($diffables.length -gt 1){_vp diff -u @diffables}}} } #cspell:ignore arrgs diffables saveables