[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSAvoidUsingCmdletAliases', '', Justification='overriding pushd and popd in a dev script')] # using Scope didn't work
param( #cspell:ignore wezterm
	[switch]$noPrevent,
	[switch]$noPushMain,
	[switch]$amendAndForce,
	[string]$message,
	[switch]$onlyRecreateSubtreeInWeztermPluginRepo,
	[string[]]$recreateSubtreeInWeztermPluginRepoWithPaths
)

#set-strictMode -version Latest

$ErrorActionPreference = 'Stop'

function arrgs		{	if ($args.length -le 0)	{@(								 )} # verbatim from ~vike's profile
					elseif ($args.length -le 1)	{@(	$args[0						])}
					else						{	$args[0..($args.length-1)	] }} #cspell:words arrgs

$dir			= "out"
$subtreePath	= "$dir"
$subtreeBranch	= "subtree/$dir"

$messageArgs	= @(if($message.length){arrgs -m "$message"	})
$amend			= @(if($amendAndForce ){arrgs --amend		})
$force			= @(if($amendAndForce ){arrgs -f			})

# custom simplified variants compared to original at ~vike's profile:
function confirmable() {if ($noConfirm -or		$(read-host "<# confirm by regex '^y(es)?$' #> $(			$args -join ' ') # confirm") -match		'^y(es)?$'		) {$A=$args[1..($args.length-1)];&$args[0] @A}}
function preventable() {if ($noPrevent -or -not($(read-host "<# prevent by not regex '^(y(es)?)?$' #> $(	$args -join ' ') # confirm") -notmatch	'^(y(es)?)?$')	) {$A=$args[1..($args.length-1)];&$args[0] @A}}

remove-item -ea 0 alias:pushd	;function pushd{push-location $args[0]
	write-host "get-location after push-location $(get-location)"}
remove-item -ea 0 alias:popd	;function  popd{ pop-location
	write-host "get-location after pop-location $(get-location)"}

if (-not $onlyRecreateSubtreeInWeztermPluginRepo) {
	pushd $PSScriptRoot				;				git status
	preventable										git add .
	preventable										git commit @messageArgs @amend
	if (-not $noPushMain)			{preventable	git push @force												}
	if ( $amendAndForce )			{preventable	git branch -D								$subtreeBranch	}
	preventable										git subtree split --prefix=$subtreePath -b	$subtreeBranch
	preventable										git push @force origin						$subtreeBranch
	popd
}
if ($recreateSubtreeInWeztermPluginRepoWithPaths) {
	if ($recreateSubtreeInWeztermPluginRepoWithPaths.length -lt 2) {write-error "-recreateSubtreeInWeztermPluginRepoWithPaths should be an array of at least length 2 like @(`$weztermPluginRepoPath,`$weztermPluginSubtreePrefix,`$weztermPluginResetArgs…) wh/ like e.g. HEAD^^ or `$commitHash"}
	$weztermPluginRepoPath = $recreateSubtreeInWeztermPluginRepoWithPaths[0]
	$weztermPluginSubtreePrefix = $recreateSubtreeInWeztermPluginRepoWithPaths[1]
	$weztermPluginResetArgs = $recreateSubtreeInWeztermPluginRepoWithPaths[2..($recreateSubtreeInWeztermPluginRepoWithPaths.length-1)]
	pushd $weztermPluginRepoPath	;				git status
	if ($weztermPluginResetArgs)	{preventable	git reset @weztermPluginResetArgs}
	preventable										remove-item -r $weztermPluginSubtreePrefix
	preventable										git subtree add --prefix=$weztermPluginSubtreePrefix https://github.com/vike2000/wezterm-lua-defs.git $subtreeBranch --squash
	popd
}

# ~vike's dev command w/ sourced $PROFILE.CurrentUserAllHosts: > &{ $weztermLuaDefsRepoPath=$(vpdr ~/file/develop/rust/wezterm-lua-defs) ; $weztermPluginRepoPath=$(vpdr ~/.wezterm/local-plugin/resurrect.wezterm) ; &$weztermLuaDefsRepoPath/dev -amend -message "initial commit" -recreateSubtree "$weztermPluginRepoPath",plugin/types/wezterm,HEAD^^ } #cspell:ignore vpdr
# ~vike's dev command w/ sourced $PROFILE.CurrentUserAllHosts: > &{ if ($thenDev=1) { $devNoPrevent=1 } ; if ($firstOut=1) { $clean=0 ; $debug=0 ; $print=0 ; $git=0 ; $paths=2 } ; $weztermPluginRepoPath=$(vpdr ~/.wezterm/local-plugin/resurrect.wezterm) ; $weztermLuaDefsRepoPath=$(vpdr ~/file/develop/rust/wezterm-lua-defs) ; $weztermRepoPath=$(vpdr ~/file/develop/git/wezterm) ; $devNoPrevent=@(if($devNoPrevent){arrgs -noPrevent}) ; $doubleDash=@(if($debug -or -not $subtreeOnly){arrgs --}) ; $debug=@(if($debug){arrgs --debug $debug}) ; $clean=@(if($clean){arrgs -clean}) ; $print=@(if($print){arrgs -print}) ; $git=@(if($git -le 0){arrgs -noGit}elseif($git -eq 1){arrgs -subtreeOnly}else{arrgs -gitCommitMessage "update out/wezterm.d.lua"}) ; $paths=@(if(-not$subtreeOnly){if($paths -eq 2){arrgs $weztermRepoPath/*/src $weztermRepoPath/lua-api-crates}elseif($paths -eq 1){arrgs $weztermRepoPath/config/src/lua.rs}else{write-error "bad paths: $paths"}}) ; $subtreeOnly=@(if($subtreeOnly){arrgs -subtreeOnly}) ; if($firstOut){_vp $weztermLuaDefsRepoPath/out @clean @print @git @doubleDash @debug @paths} ; if($thenDev){_vp $weztermLuaDefsRepoPath/dev -amend -message "initial commit" -recreateSubtree "$weztermPluginRepoPath",plugin/types/wezterm,7af0ad6 @devNoPrevent} }
# ~vike's dev command w/ sourced $PROFILE.CurrentUserAllHosts: > &{ if ($thenDev=1) { $devNoPrevent=0 ; $recreateSubtree=$null ; $amend=0 ; $message="refactor ``_…_`` into ``_`` in ``extract_…_signature`` using new symbols: using ``fn lua_params_from_sources`` to convert ``enum ParamSource`` by ``fn extract_param_sources_from_typed_pat`` " } ; if ($firstOut=1) { $clean=0 ; $debug=0 ; $print=0 ; $git=0 ; $paths=2 } ; $weztermPluginRepoPath=$(vpdr ~/.wezterm/local-plugin/resurrect.wezterm) ; $weztermLuaDefsRepoPath=$(vpdr ~/file/develop/rust/wezterm-lua-defs) ; $weztermRepoPath=$(vpdr ~/file/develop/git/wezterm) ; if($thenDev){ $devNoPrevent=@(if($devNoPrevent){arrgs -noPrevent}) ; $recreateSubtree=@(if($recreateSubtree){arrgs -recreateSubtree $recreateSubtree}) ; $amend=@(if($amend){arrgs -amend}) ; $message=@(if($message){arrgs -message "$message"}) ;  } ; if($firstOut){ $doubleDash=@(if($debug -or -not $subtreeOnly){arrgs --}) ; $debug=@(if($debug){arrgs --debug $debug}) ; $clean=@(if($clean){arrgs -clean}) ; $print=@(if($print){arrgs -print}) ; $git=@(if($git -le 0){arrgs -noGit}elseif($git -eq 1){arrgs -subtreeOnly}else{arrgs -gitCommitMessage "update out/wezterm.d.lua"}) ; $paths=@(if(-not$subtreeOnly){if($paths -eq 2){arrgs $weztermRepoPath/*/src $weztermRepoPath/lua-api-crates}elseif($paths -eq 1){arrgs $weztermRepoPath/config/src/lua.rs}else{write-error "bad paths: $paths"}}) ; $subtreeOnly=@(if($subtreeOnly){arrgs -subtreeOnly}) } ; if($firstOut){_vp $weztermLuaDefsRepoPath/out @clean @print @git @doubleDash @debug @paths} ; if($thenDev){_vp $weztermLuaDefsRepoPath/dev @amend @message @recreateSubtree @devNoPrevent} }