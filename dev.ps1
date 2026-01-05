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