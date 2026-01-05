	 use anyhow::Result
	;use std::collections::{HashMap, HashSet}
	;use clap::{Parser, CommandFactory, ArgAction}

	;use regex::Regex

	;use std::fs
	;use std::path::{Path, PathBuf}
	;use walkdir::WalkDir // cspell:ignore walkdir

	;use syn::visit::Visit
	;use syn::Expr
	;use syn::spanned::Spanned
	;

	trait			TypeStr {
		fn			type_str			(&self								) -> &'static str;}
	trait			ToSrc {
		#[allow(dead_code)]
		fn			to_src				(&self,				source: &String	) -> String {
			self.	to_src_debug		(		0,			source)}
		fn			to_src_debug		(&self, debug: i8,	source: &String	) -> String;}

	#[derive(Default)]
	struct HelpOpt
		{short		: String
		,long		: String
		,key		: String
		,alts		: String
		,optional	: bool
		,default	: String
		,help		: String}
	#[derive(Default)]
	struct HelpArg
		{serial		: usize // no practical example (yet)
		,last		: bool
		,optional	: bool
		,key		: String
		,help		: String}
	trait HelpArgs
		{fn to_string(&self) -> String;}
	trait HelpValueByOpt
		{fn value_by_opt(&self, id: &HelpOpt) -> String;}
	trait HelpArgsByArg
		{fn args_by_arg(&self, id: &HelpArg) -> &dyn HelpArgs;}

	fn help<T>(cmd: clap::Command, parsed: T, code: i32) -> i32 where T: HelpValueByOpt + HelpArgsByArg
		{let name		= cmd.get_bin_name	().unwrap_or_else(|| cmd.get_name())
		;let version	= cmd.get_version	().map		(|s| format!("version {}\n", s)).unwrap_or_default()
		;let options	= cmd.get_arguments	().filter	(|a| !a.is_hide_set() && (a.get_short().is_some() || a.get_long().is_some()))
		;let mut opts: Vec<HelpOpt> = Vec::new()
		;for arg in options
			{let mut opt = HelpOpt::default()
			;if let Some(short	) = arg.get_short()
				{opt.short = short.to_string()}
			;if let Some(long	) = arg.get_long()
				{opt.long = long.to_string()}
			;if let Some(help	) = arg.get_help()
				{opt.help = help.to_string()}
			;if arg.get_action().takes_values()
				{if let Some(name) = arg.get_value_names().and_then(|arr| arr.first())
					{opt.key = name.to_string()}}
			;let possible_values = arg.get_possible_values()
			;if !possible_values.is_empty()
				{opt.alts = possible_values
					.iter().map(|v| v.get_name().to_string())
					.collect::<Vec<String>>().join("|")}
			;match arg.get_action() {
				ArgAction::SetTrue | ArgAction::Set | ArgAction::Append => {opt.optional = true}
				_ => {}}
			;if let Some(default) = arg.get_default_values().first()
				{opt.default = default.to_string_lossy().to_string()}
			;opts.push(opt)}
	
		;let mut args: Vec<HelpArg> = Vec::new()
		;let mut serial: usize = 0
		;for positional in cmd.get_positionals()
			{let mut arg = HelpArg::default()
			;let Some(key) = positional.get_value_names().and_then(|arr| arr.first()) else {continue}
			;arg.serial = serial; serial += 1
			;arg.key = key.to_string()
			;arg.optional = !positional.is_required_set()
			;arg.last = positional.is_last_set()
			;arg.help = positional.get_help().unwrap_or_default().ansi().to_string()
			;args.push(arg)}
	
		;let summary	= opts.iter().map(|o| {format!(
			"{opt_open}{short}{short_long_pipe}{long}{key}{opt_close}{alts}{value}{default}"
			,opt_open			= if!	o.optional							{ ""	.to_owned()	} else {				"["							.to_owned	()}
			,short				= if	o.short		.is_empty()				{ ""	.to_owned()	} else {format!(	   "-{}" ,	o.short							 )}
			,short_long_pipe	= if	o.short		.is_empty()
				||						o.long		.is_empty()				{ ""	.to_owned()	} else {				"|"							.to_owned	()}
			,long				= if	o.short		.is_empty()				{ ""	.to_owned()	} else {format!(	  "--{}" ,	o.long							 )}
			,key				= if	o.key		.is_empty()				{ ""	.to_owned()	} else {format!(	  " ‹{}›",	o.key							 )}
			,alts				= if	o.alts		.is_empty()				{ ""	.to_owned()	} else {format!(	 " {{{}}}",	o.alts							 )}
			,opt_close			= if!	o.optional							{ ""	.to_owned()	} else {				"]"							.to_owned	()}
			,value				= format!(" ({})",	parsed.value_by_opt(														o								 ))
			,default			= if	o.default	.is_empty()				{ ""	.to_owned()	} else {format!(	" || {}",	o.default						 )}
			)}).collect::<Vec<String>>().join(" ")
		;let full			= opts.iter().map(|o| {format!(
			"{opt_open:1}{short:2}{short_long_pipe:1}{long:16}{key:16}{opt_close:1}{value}{default}{alts}{help}"
			,opt_open			= if!	o.optional							{ ""	.to_owned()	} else {				"["							.to_owned	()}
			,short				= if	o.short		.is_empty()				{ ""	.to_owned()	} else {format!(	   "-{}" ,	o.short							 )}
			,short_long_pipe	= if	o.short		.is_empty()
				||						o.long		.is_empty()				{ ""	.to_owned()	} else {				"|"							.to_owned	()}
			,long				= if	o.short		.is_empty()				{ ""	.to_owned()	} else {format!(	  "--{}" ,	o.long							 )}
			,key				= if	o.key		.is_empty()				{ ""	.to_owned()	} else {format!(	  " ‹{}›",	o.key							 )}
			,alts				= if	o.alts		.is_empty()				{ ""	.to_owned()	} else {format!("\n    {{{}}}",	o.alts							 )}
			,opt_close			= if!	o.optional							{ ""	.to_owned()	} else {				"]"							.to_owned	()}
			,value				= format!(" ({})",	parsed.value_by_opt(														o								 ))
			,default			= if	o.default	.is_empty()				{ ""	.to_owned()	} else {format!(	" || {}",	o.default						 )}
			,help				= if	o.help		.is_empty()				{ ""	.to_owned()	} else {format!(  "\n    {}",	o.help				.clone		())}
			)}).collect::<Vec<String>>().join("\n  ")
		;let positional = args.iter().map(|a| {format!(
			"{opt_open}{last}{key}{opt_close}{value}"
			,opt_open			= if!	a.optional							{ ""	.to_owned()	} else {				"["							.to_owned	()}
			,last				= if!	a.last								{ ""	.to_owned()	} else {				"-- "						.to_owned	()}
			,key				=																	format!(	   "‹{}›",	a.key							 )
			,opt_close			= if!	a.optional							{ ""	.to_owned()	} else {				"]"							.to_owned	()}
			,value				=					parsed.args_by_arg(a).to_string()
			)}).collect::<Vec<String>>().join(" ")
		;print!("\
			{version}usage: {name}{summary}{positional}\n  \
			{full}"
			,summary			= if	summary		.is_empty()				{ ""	.to_owned()	} else {format!(	   " {}",	summary							 )}
			,positional			= if	summary		.is_empty()				{ ""	.to_owned()	} else {format!(	   " {}",	positional						 )})
		;return code}

	impl	TypeStr
	for		 syn::Item {
		fn type_str(&self) -> &'static str {match self // copy from definitions and do s!^\s+//[^\n]*\n!! (@vscode) to get started
			{syn::Item::Const		(_/*ItemConst		*/	) => "Const"
			,syn::Item::Enum		(_/*ItemEnum		*/	) => "Enum"
			,syn::Item::ExternCrate	(_/*ItemExternCrate	*/	) => "ExternCrate"
			,syn::Item::Fn			(_/*ItemFn			*/	) => "Fn"
			,syn::Item::ForeignMod	(_/*ItemForeignMod	*/	) => "ForeignMod"
			,syn::Item::Impl		(_/*ItemImpl		*/	) => "Impl"
			,syn::Item::Macro		(_/*ItemMacro		*/	) => "Macro"
			,syn::Item::Mod			(_/*ItemMod			*/	) => "Mod"
			,syn::Item::Static		(_/*ItemStatic		*/	) => "Static"
			,syn::Item::Struct		(_/*ItemStruct		*/	) => "Struct"
			,syn::Item::Trait		(_/*ItemTrait		*/	) => "Trait"
			,syn::Item::TraitAlias	(_/*ItemTraitAlias	*/	) => "TraitAlias"
			,syn::Item::Type		(_/*ItemType		*/	) => "Type"
			,syn::Item::Union		(_/*ItemUnion		*/	) => "Union"
			,syn::Item::Use			(_/*ItemUse			*/	) => "Use"
			,syn::Item::Verbatim	(_/*TokenStream		*/	) => "Verbatim"
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}

	impl	TypeStr
	for		 syn::Expr {
		fn type_str(&self) -> &'static str {match self
			{syn::Expr::Array		(_/*ExprArray		*/	) => "Array"
			,syn::Expr::Assign		(_/*ExprAssign		*/	) => "Assign"
			,syn::Expr::Async		(_/*ExprAsync		*/	) => "Async"
			,syn::Expr::Await		(_/*ExprAwait		*/	) => "Await"
			,syn::Expr::Binary		(_/*ExprBinary		*/	) => "Binary"
			,syn::Expr::Block		(_/*ExprBlock		*/	) => "Block"
			,syn::Expr::Break		(_/*ExprBreak		*/	) => "Break"
			,syn::Expr::Call		(_/*ExprCall		*/	) => "Call"
			,syn::Expr::Cast		(_/*ExprCast		*/	) => "Cast"
			,syn::Expr::Closure		(_/*ExprClosure		*/	) => "Closure"
			,syn::Expr::Const		(_/*ExprConst		*/	) => "Const"
			,syn::Expr::Continue	(_/*ExprContinue	*/	) => "Continue"
			,syn::Expr::Field		(_/*ExprField		*/	) => "Field"
			,syn::Expr::ForLoop		(_/*ExprForLoop		*/	) => "ForLoop"
			,syn::Expr::Group		(_/*ExprGroup		*/	) => "Group"
			,syn::Expr::If			(_/*ExprIf			*/	) => "If"
			,syn::Expr::Index		(_/*ExprIndex		*/	) => "Index"
			,syn::Expr::Infer		(_/*ExprInfer		*/	) => "Infer"
			,syn::Expr::Let			(_/*ExprLet			*/	) => "Let"
			,syn::Expr::Lit			(_/*ExprLit			*/	) => "Lit"
			,syn::Expr::Loop		(_/*ExprLoop		*/	) => "Loop"
			,syn::Expr::Macro		(_/*ExprMacro		*/	) => "Macro"
			,syn::Expr::Match		(_/*ExprMatch		*/	) => "Match"
			,syn::Expr::MethodCall	(_/*ExprMethodCall	*/	) => "MethodCall"
			,syn::Expr::Paren		(_/*ExprParen		*/	) => "Paren"
			,syn::Expr::Path		(_/*ExprPath		*/	) => "Path"
			,syn::Expr::Range		(_/*ExprRange		*/	) => "Range"
			,syn::Expr::RawAddr		(_/*ExprRawAddr		*/	) => "RawAddr"
			,syn::Expr::Reference	(_/*ExprReference	*/	) => "Reference"
			,syn::Expr::Repeat		(_/*ExprRepeat		*/	) => "Repeat"
			,syn::Expr::Return		(_/*ExprReturn		*/	) => "Return"
			,syn::Expr::Struct		(_/*ExprStruct		*/	) => "Struct"
			,syn::Expr::Try			(_/*ExprTry			*/	) => "Try"
			,syn::Expr::TryBlock	(_/*ExprTryBlock	*/	) => "TryBlock"
			,syn::Expr::Tuple		(_/*ExprTuple		*/	) => "Tuple"
			,syn::Expr::Unary		(_/*ExprUnary		*/	) => "Unary"
			,syn::Expr::Unsafe		(_/*ExprUnsafe		*/	) => "Unsafe"
			,syn::Expr::Verbatim	(_/*TokenStream		*/	) => "Verbatim"
			,syn::Expr::While		(_/*ExprWhile		*/	) => "While"
			,syn::Expr::Yield		(_/*ExprYield		*/	) => "Yield"
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}

	impl	TypeStr
	for		 syn::Type {
		fn type_str(&self) -> &'static str {match self
			{syn::Type::Array		(_/*TypeArray		*/	) => "Array"
			,syn::Type::BareFn		(_/*TypeBareFn		*/	) => "BareFn"
			,syn::Type::Group		(_/*TypeGroup		*/	) => "Group"
			,syn::Type::ImplTrait	(_/*TypeImplTrait	*/	) => "ImplTrait"
			,syn::Type::Infer		(_/*TypeInfer		*/	) => "Infer"
			,syn::Type::Macro		(_/*TypeMacro		*/	) => "Macro"
			,syn::Type::Never		(_/*TypeNever		*/	) => "Never"
			,syn::Type::Paren		(_/*TypeParen		*/	) => "Paren"
			,syn::Type::Path		(_/*TypePath		*/	) => "Path"
			,syn::Type::Ptr			(_/*TypePtr			*/	) => "Ptr"
			,syn::Type::Reference	(_/*TypeReference	*/	) => "Reference"
			,syn::Type::Slice		(_/*TypeSlice		*/	) => "Slice"
			,syn::Type::TraitObject	(_/*TypeTraitObject	*/	) => "TraitObject"
			,syn::Type::Tuple		(_/*TypeTuple		*/	) => "Tuple"
			,syn::Type::Verbatim	(_/*TokenStream		*/	) => "Verbatim"
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}

	impl	TypeStr
	for		 syn::Lit {
		fn type_str(&self) -> &'static str {match self
			{syn::Lit::Str			(_/*LitStr			*/	) => "Str"
			,syn::Lit::ByteStr		(_/*LitByteStr		*/	) => "ByteStr"
			,syn::Lit::CStr			(_/*LitCStr			*/	) => "CStr"
			,syn::Lit::Byte			(_/*LitByte			*/	) => "Byte"
			,syn::Lit::Char			(_/*LitChar			*/	) => "Char"
			,syn::Lit::Int			(_/*LitInt			*/	) => "Int"
			,syn::Lit::Float		(_/*LitFloat		*/	) => "Float"
			,syn::Lit::Bool			(_/*LitBool			*/	) => "Bool"
			,syn::Lit::Verbatim		(_/*Literal			*/	) => "Verbatim"
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}

	impl	TypeStr
	for		 syn::Pat {
		fn type_str(&self) -> &'static str {match &self // copy from definitions and do s!^\s+//[^\n]*\n!! (@vscode) to get started
			{syn::Pat::Const		(_/*expr_const		*/	) => "Const"
			,syn::Pat::Ident		(_/*pat_ident		*/	) => "Ident"
			,syn::Pat::Lit			(_/*expr_lit		*/	) => "Lit"
			,syn::Pat::Macro		(_/*expr_macro		*/	) => "Macro"
			,syn::Pat::Or			(_/*pat_or			*/	) => "Or"
			,syn::Pat::Paren		(_/*pat_paren		*/	) => "Paren"
			,syn::Pat::Path			(_/*expr_path		*/	) => "Path"
			,syn::Pat::Range		(_/*expr_range		*/	) => "Range"
			,syn::Pat::Reference	(_/*pat_reference	*/	) => "Reference"
			,syn::Pat::Rest			(_/*pat_rest		*/	) => "Rest"
			,syn::Pat::Slice		(_/*pat_slice		*/	) => "Slice"
			,syn::Pat::Struct		(_/*pat_struct		*/	) => "Struct"
			,syn::Pat::Tuple		(_/*pat_tuple		*/	) => "Tuple"
			,syn::Pat::TupleStruct	(_/*pat_tuple_struct*/	) => "TupleStruct"
			,syn::Pat::Type			(_/*pat_type		*/	) => "Type"
			,syn::Pat::Verbatim		(_/*token_stream	*/	) => "Verbatim"
			,syn::Pat::Wild			(_/*pat_wild		*/	) => "Wild"
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}
	
	impl	ToSrc
	for		 syn::Pat {
		fn to_src_debug(&self, debug: i8, source: &String) -> String {match &self // copy from definitions and do s!^\s+//[^\n]*\n!! (@vscode) to get started
			{syn::Pat::Const		(	expr_const			) => span_src_debug(debug,	expr_const			.block	.span(),				source)
			,syn::Pat::Ident		(	pat_ident			) =>						pat_ident			.ident						.to_string()
			,syn::Pat::Lit			(	expr_lit			) =>						expr_lit			.lit	.type_str()			.to_string()
			,syn::Pat::Macro		(	expr_macro			) => span_src_debug(debug,	expr_macro			.mac	.span(),				source)
			,syn::Pat::Or			(	pat_or				) => span_src_debug(debug,	pat_or						.span(),				source)
			,syn::Pat::Paren		(	pat_paren			) => span_src_debug(debug,	pat_paren					.span(),				source)
			,syn::Pat::Path			(	expr_path			) => span_src_debug(debug,	expr_path					.span(),				source)
			,syn::Pat::Range		(	expr_range			) => span_src_debug(debug,	expr_range					.span(),				source)
			,syn::Pat::Reference	(	pat_reference		) =>						pat_reference		.pat	.to_src_debug(debug,	source)
			,syn::Pat::Rest			(  _pat_rest			) =>						".."											.to_owned()
			,syn::Pat::Slice		(	pat_slice			) => span_src_debug(debug,	pat_slice					.span(),				source)
			,syn::Pat::Struct		(	pat_struct			) => span_src_debug(debug,	pat_struct					.span(),				source)
			,syn::Pat::Tuple		(	pat_tuple			) => span_src_debug(debug,	pat_tuple					.span(),				source)
			,syn::Pat::TupleStruct	(	pat_tuple_struct	) => span_src_debug(debug,	pat_tuple_struct			.span(),				source)
			,syn::Pat::Type			(	pat_type			) =>						pat_type			.ty		.type_str()			.to_string()
			,syn::Pat::Verbatim		(	token_stream		) =>						token_stream									.to_string()
			,syn::Pat::Wild			(  _pat_wild			) =>						"_"												.to_owned()
			,_t => panic!("unmatched type: {}", quote::quote!("{}", _t))}}}

	#[derive(Default, Debug)]
	struct Stats
		{method_calls		: usize
		,files				: usize
		,}

	#[derive(Default)]
	struct VisitingContext
		{visited_files		: HashSet<PathBuf>
		,type_aliases		: HashMap<String, syn::Type>
		,structs			: HashMap<String, StructDef>
		,functions			: HashMap<String, syn::ItemFn>
		,/// None = module-level, Some = userdata
		 current_userdata	: Option<String>
		,module_vars		: HashMap<String, Vec<String>>
		,current_module		: Vec<String>
		,lua_functions		: HashMap<LuaFunctionKey, LuaFunction>
		,}

	struct FunctionContext
		{file_path			: PathBuf
		,source				: String}

	#[derive(Hash, Eq, PartialEq, Debug)]
	struct LuaFunctionKey
		{owner				: LuaFunctionOwnerKey
		,name				: String
		,kind				: LuaFunctionKind
		,arity				: usize
		,}

	#[derive(Hash, Eq, PartialEq, Debug)]
	enum LuaFunctionOwnerKey
		{Module(Vec<String>)
		,UserData(String)
		,}

	#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
	enum LuaFunctionKind
		{Module
		,Method
		,AsyncMethod
		,MetaMethod
		,}

	#[derive(Debug)]
	enum LuaFunctionOwner {
		Module(Vec<String>),
		UserData(String),
	}

	#[allow(dead_code)]
	#[derive(Debug)]
	struct LuaFunction
		{file				: PathBuf // CHECK implement for what?
		,name				: String
		,kind				: LuaFunctionKind
		,owner				: LuaFunctionOwner
		,params				: Vec<LuaParam>
		,returns			: Vec<LuaType>
		,is_async			: bool // TODO implement for extraction and emission (emit_…)
		,}

	impl LuaFunction {
		fn key(&self) -> LuaFunctionKey
			{LuaFunctionKey
				{owner		: match &self.owner
					{LuaFunctionOwner::Module(v)	=> LuaFunctionOwnerKey::Module(v.clone())
					,LuaFunctionOwner::UserData(u)	=> LuaFunctionOwnerKey::UserData(u.clone())}
				,name		: self.name.clone()
				,kind		: self.kind.clone()
				,arity		: self.params.len()}}}


	struct StructDef
		{tuple_fields: Vec<syn::Type>
		,named_fields: HashMap<String, syn::Type>}

	#[allow(dead_code)]
	#[derive(Debug)]
	struct LuaParam
		{name				: Option<String>
		,lua_type			: LuaType
		,}

	#[allow(dead_code)]
	#[derive(Debug)]
	enum LuaType
		{Nil
		,Number
		,Boolean
		,Value
		,Unit
		,String
		,Function
		,Optional(Box<LuaType>)
		,Variadic(Box<LuaType>)
		,UserData(String)
		,Unknown(String)
		,Any
		,}

	struct	TypeAliasCollector<'a>
		{visiting_context: &'a mut	VisitingContext}
	struct	StructCollector<'a>
		{visiting_context: &'a mut	VisitingContext}
	struct FunctionCollector<'a>
		{visiting_context: &'a mut	VisitingContext}
	struct	LuaRegistrationVisitor<'a>
		{visiting_context: &'a mut	VisitingContext
		,function_context: &'a		FunctionContext
		,args: &'a Args
		,stats: &'a mut Stats}

	#[derive(Parser)]
	#[command(disable_help_flag = true)]
	struct	Args
		{#[arg(short, long, required = false)]
		 help: bool
		,#[arg(short = 'B', long, value_name = "level")]
		 debug: Option<i8>
		,#[arg(name = "path")]
		 paths: Vec<String>
		,}

	impl	HelpValueByOpt
	for		Args
		{fn value_by_opt(&self, o: &HelpOpt) -> String			{match o
				{o if o.long == "help"		=> self.help.to_string()
				,o if o.long == "debug"		=> self.debug.unwrap_or_default().to_string()
				,o => panic!("internal error: unmatched option, long: {}", o.long)}}}
	impl	HelpArgsByArg
	for		Args
		{fn args_by_arg(&self, a: &HelpArg) -> &dyn HelpArgs	{match a
				{a if a.key == "path"		=> &self.paths
				,a => panic!("internal error: unmatched argument key: {}", a.key)}}}
	impl	HelpArgs
	for	Vec<String>
		{fn to_string(&self) -> String {self.join(" ")}}
	fn main() -> Result<()>
		{let args = Args::parse()
		
		;if args.help
			{std::process::exit(help(Args::command(), args, 0))}
		;if args.paths.is_empty()
			{std::process::exit(help(Args::command(), args, 2))}
		
		;let mut stats = Stats::default()
		;let mut visiting_context = VisitingContext::default()
		
		;for path in resolve_input_paths(&args.paths, false)?
			{if args.debug.unwrap_or_default() > 2 {eprintln!("main: enter {}", path.display())}
			;for rs_file in collect_rs_files(&path)
				{parse_file(&args, rs_file, &mut visiting_context, &mut stats)?}}
		
		;let					api
		=				  build_api(visiting_context.lua_functions)
		
		;let module_root_class	= "Wezterm" //cspell:ignore Wezterm
		;let module_root_name	= "wezterm"
		
		;			println!("---@meta")
		;			println!("")
		;			println!("---@class {}", module_root_class)
		;			println!("local {} = {{}}", module_root_name)
		;			println!("")
		;			println!("---@alias UserDataRef any")
		;			println!("---@alias Value any")
		;			println!("---@alias SplitPane any")
		;			println!("---@alias SpawnTab any")
		;			println!("---@alias SpawnWindow any")
		;			println!("")
		
		;			println!("{}",	emit_module_tables(module_root_name, &api.module_paths))
		
		;for (name, methods)
		in					&	api		.userdata
			{		println!("{}",	emit_userdata(&format!("{}.{}", module_root_name, name), methods))}

		;			println!("{}",	emit_module_functions(module_root_name
			,				&	api		.modules							))

		;			println!("return {}", module_root_name)

		;if args.debug.unwrap_or_default() > 0
			{	   eprint!		(	"visiting_context.type_aliases: {{"		)
			;if						!visiting_context.type_aliases.is_empty()
				{  eprintln!()												}
			;for (name, ty) in		&visiting_context.type_aliases
				{  eprintln!("	\"{}\" = \"{}\",", name, quote::quote!(#ty))}
			;	   eprintln!	(									"}}"	)}

		;if args.debug.unwrap_or_default() > 0
			{	   eprintln!	(	"stats: {:#?}", stats					)}

		;Ok(())}

fn resolve_input_paths(patterns: &[String], literal: bool) -> anyhow::Result<Vec<PathBuf>> {
	let mut out = Vec::new();

	for p in patterns {
		if literal {
			out.push(PathBuf::from(p));
		} else {
			let mut matched = false;
			for entry in glob::glob(p)? {
				matched = true;
				out.push(entry?);
			}

			if !matched {
				return Err(anyhow::anyhow!("no matches for pattern: {}", p));
			}
		}
	}

	Ok(out)
}

fn collect_rs_files(path: &Path) -> Vec<PathBuf> {
	if path.is_dir() {
		WalkDir::new(path)
			.into_iter()
			.filter_map(|e| e.ok())
			.map(|e| e.into_path())
			.filter(|p| p.extension().map(|x| x == "rs").unwrap_or(false))
			.collect()
	} else {
		vec![path.to_path_buf()]
	}
}

fn parse_file(args: &Args, file_path: PathBuf, visiting_context: &mut VisitingContext, stats: &mut Stats) -> Result<()> {
	let file_path_canonical = file_path.canonicalize()?;
	if !visiting_context.visited_files.insert(file_path.clone()) {
		return Ok(());
	}

	stats.files += 1;

	let function_context = &FunctionContext {
		file_path,
		source: fs::read_to_string(&file_path_canonical)?
	};
	if args.debug.unwrap_or_default() > 2 {eprintln!("parse_file: enter {}", function_context.file_path.display())}
	let file = match syn::parse_file(&function_context.source) {
		Ok(file) => {file}
		Err(err) => {
			if args.debug.unwrap_or_default() > 3 {eprintln!("parse_file: skipping {}: {}", function_context.file_path.display(), err);}
			return Ok(());
		}
	};

	let parent_dir = file_path_canonical.parent()
		.expect("file should have parent dir");

	for item in &file.items {
		if let syn::Item::Mod(m) = item {
			if m.content.is_none() {
				let mod_name = m.ident.to_string();

				let candidate1 = parent_dir.join(format!("{}.rs", mod_name));
				let candidate2 = parent_dir.join(&mod_name).join("mod.rs");

				if candidate1.exists() {
					parse_file(&args, Path::new(&candidate1).to_path_buf(), visiting_context, stats)?;
				} else if candidate2.exists() {
					parse_file(&args, Path::new(&candidate2).to_path_buf(), visiting_context, stats)?;
				}
			}
		}
	}

	TypeAliasCollector		{visiting_context								}.visit_file(&file);
	StructCollector			{visiting_context								}.visit_file(&file);
	FunctionCollector		{visiting_context								}.visit_file(&file);
	LuaRegistrationVisitor	{visiting_context, function_context, args, stats}.visit_file(&file);

	Ok(())
}

impl<'ast, 'a> Visit<'ast>
for		TypeAliasCollector<'a> {
	fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
		let name = item.ident.to_string();
		let ty = (*item.ty).clone();
		self.visiting_context.type_aliases.insert(name, ty);
		syn::visit::visit_item_type(self, item);
	}
}

impl<'ast, 'a> Visit<'ast>
for		StructCollector<'a> {
	fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
		let name = item.ident.to_string();

		let mut def = StructDef {
			tuple_fields: Vec::new(),
			named_fields: HashMap::new(),
		};

		match &item.fields {
			syn::Fields::Unnamed(fields) => {
				for f in &fields.unnamed {
					def.tuple_fields.push(f.ty.clone());
				}
			}
			syn::Fields::Named(fields) => {
				for f in &fields.named {
					if let Some(ident) = &f.ident {
						def.named_fields.insert(ident.to_string(), f.ty.clone());
					}
				}
			}
			syn::Fields::Unit => {}
		}

		self.visiting_context.structs.insert(name, def);

		syn::visit::visit_item_struct(self, item);
	}
}

impl<'ast, 'a> Visit<'ast>
for		FunctionCollector<'a> {
	fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
		let name = item.sig.ident.to_string();
		self.visiting_context.functions.insert(name, item.clone());
		syn::visit::visit_item_fn(self, item);
	}
}

impl<'ast, 'a>	LuaRegistrationVisitor<'a> {
	fn try_capture_module_var(&mut self, local: &'ast syn::Local) {
		let syn::Pat::Ident(pat_ident) = &local.pat else { return };
		let Some(init) = &local.init else { return };

		let expr = unwrap_try(init.expr.as_ref());

		let syn::Expr::Call(call) = expr else { return };

		let syn::Expr::Path(func) = call.func.as_ref() else { return };
		let Some(fname) = func.path.segments.last() else { return };

		let module_name = match fname.ident.to_string().as_str() {
			"get_or_create_module" => None,
			"get_or_create_sub_module" => {
				let Some(syn::Expr::Lit(syn::ExprLit {
					lit: syn::Lit::Str(s),
					..
				})) = call.args.get(1) else { return };
				Some(s.value())
			}
			_ => return,
		};

		let mut path = vec![];
		if let Some(sub) = module_name {
			path.push(sub);
		}

		self.visiting_context
			.module_vars
			.insert(pat_ident.ident.to_string(), path);
	}
}

impl<'ast, 'a> syn::visit::Visit<'ast>
for				LuaRegistrationVisitor<'a> {
	fn visit_local(&mut self, local: &'ast syn::Local) {
		self.try_capture_module_var(local);
		syn::visit::visit_local(self, local);
	}

	fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
		let name = node.ident.to_string();

		self.visiting_context.current_module.push(name);

		if let Some((_, items)) = &node.content {
			for item in items {
				self.visit_item(item);
			}
		}

		self.visiting_context.current_module.pop();
	}

	fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
		// Preserve your existing UserData handling
		if node.trait_.as_ref().is_some_and(|(_, path, _)| {
			path.segments.last().unwrap().ident == "UserData"
		}) {
			visit_userdata_impl(
				self.args,
				self.function_context,
				self.visiting_context,
				self.stats,
				node.clone(),
			);
		} else {
			syn::visit::visit_item_impl(self, node);
		}
	}

	fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
		self.stats.method_calls += 1;

		let name = if call.args.len() < 1 {None} else {match &call.args[0] {
			syn::Expr::Lit(syn::ExprLit {
				lit: syn::Lit::Str(s),
				..
			}) => Some(s.value()),
			_ => None,
		}};

		let arg1unwrapped = if call.args.len() < 2 {None} else {Some(unwrap_try(&call.args[1]))};

/*		if call.method == "set" && self.args.debug.unwrap_or_default() > 0 {
			eprintln!("found .set() at {}:{}",
				call.span().start().line,
				call.span().start().column);
			for (i, arg) in call.args.iter().enumerate() {
				eprintln!("  arg[{}] = Expr::{}", i, arg.type_str());
			}
			if let Some(n) = &name {
				eprintln!("  arg[0] as str = {}", n);
			}
			if let Some(u) = &arg1unwrapped {
				eprintln!("  arg[1] unwrapped.type_str() = {}", u.type_str());
			}
		}*/

		if call.method == "set" && call.args.len() >= 2 {
			if let Some(lua_name) = &name {
				let (closure, fn_item) = match arg1unwrapped {
					Some(syn::Expr::MethodCall(m))
						if m.method == "create_function" || m.method == "create_async_function" =>
					{
						match m.args.first() {
							Some(syn::Expr::Closure(c)) => (Some(c), None),

							Some(syn::Expr::Path(p)) => {
								if let Some(last_seg) = p.path.segments.last() {
									let ident = last_seg.ident.to_string();
									let f = self.visiting_context.functions.get(&ident).cloned();
									(None, f)
								} else {
									(None, None)
								}
							}

							_ => (None, None),
						}
					}
					_ => (None, None),
				};

				if self.visiting_context.current_userdata.is_some() {
					return; // never treat `.set` inside userdata impl as module function
				}

				if let Some(closure) = closure {
					let owner = if let Some(userdata) = &self.visiting_context.current_userdata {
						LuaFunctionOwner::UserData(userdata.clone())
					} else if let syn::Expr::Path(p) = call.receiver.as_ref() {
						if let Some(ident) = p.path.get_ident() {
							if let Some(path) = self.visiting_context.module_vars.get(&ident.to_string()) {
								LuaFunctionOwner::Module(path.clone())
							} else {
								LuaFunctionOwner::Module(self.visiting_context.current_module.clone())
							}
						} else {
							LuaFunctionOwner::Module(self.visiting_context.current_module.clone())
						}
					} else {
						LuaFunctionOwner::Module(self.visiting_context.current_module.clone())
					};

					extract_closure_signature(self.args, self.function_context, self.visiting_context, self.stats, owner, lua_name, LuaFunctionKind::Module, closure);
				}

				if let Some(fn_item) = fn_item {
					let owner = if let Some(userdata) = self.visiting_context.current_userdata.clone()
					{
						LuaFunctionOwner::UserData(userdata)
					} else {
						LuaFunctionOwner::Module(self.visiting_context.current_module.clone())
					};

					extract_fn_signature(self.args, self.function_context, self.visiting_context, self.stats, owner, lua_name, LuaFunctionKind::Module, &fn_item);
				}
			}
		}

		syn::visit::visit_expr_method_call(self, call);
	}
}
fn unwrap_try(expr: &syn::Expr) -> &syn::Expr {
	match expr {
		syn::Expr::Try(t) => unwrap_try(&t.expr),
		_ => expr,
	}
}
fn extract_userdata_name(imp: &syn::ItemImpl) -> Option<String> {
	let trait_path = imp.trait_.as_ref()?.1.segments.last()?;
	if trait_path.ident != "UserData" {
		return None;
	}

	match &*imp.self_ty {
		syn::Type::Path(tp) => {
			tp.path.segments.last().map(|seg| seg.ident.to_string())
		}
		_ => None,
	}
}

fn visit_userdata_impl(args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, stats: &mut Stats, imp: syn::ItemImpl) {
	let Some(userdata_type) = extract_userdata_name(&imp) else {
		return;
	};

	// Save and clear module context
	let prev_module = visiting_context.current_module.clone();
	let prev_userdata = visiting_context.current_userdata.take();

	visiting_context.current_module.clear();
	visiting_context.current_userdata = Some(userdata_type);

	for item in imp.items {
		if let syn::ImplItem::Fn(func) = item {
			if func.sig.ident == "add_methods" {
				visit_add_methods(args, function_context, visiting_context, stats, func);
			}
		}
	}

	// Restore context
	visiting_context.current_userdata = prev_userdata;
	visiting_context.current_module = prev_module;
}

fn visit_add_methods(args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, stats: &mut Stats, func: syn::ImplItemFn) {
    debug_assert!(visiting_context.current_userdata.is_some(), "visit_add_methods called outside of UserData scope");

    for stmt in func.block.stmts {
        if let syn::Stmt::Expr(expr, _) = stmt {
            visit_lua_registration_expr(args, function_context, visiting_context, stats, expr);
        }
    }
}

fn visit_lua_registration_expr(
	args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, stats: &mut Stats, expr: syn::Expr) {
	let syn::Expr::MethodCall(call) = expr else { return };

	let method = call.method.to_string();

	let kind = match method.as_str() {
		"set" => LuaFunctionKind::Module,
		"add_method" => LuaFunctionKind::Method,
		"add_async_method" => LuaFunctionKind::AsyncMethod,
		"add_meta_method" => LuaFunctionKind::MetaMethod,
		_ => return,
	};


	let is_lua_reg = matches!(
		method.as_str(),
		"set" | "add_method" | "add_async_method" | "add_meta_method"
	);

	if !is_lua_reg {
		return;
	}

	let Some(syn::Expr::Lit(syn::ExprLit {
		lit: syn::Lit::Str(name),
		..
	})) = call.args.first()
	else {
		return;
	};

	let lua_name = name.value();

	let Some(syn::Expr::Closure(closure)) = call.args.get(1) else {
		return;
	};

	let owner = match kind {
		LuaFunctionKind::Module => {
			if visiting_context.current_userdata.is_some() {
				return; // do NOT treat module `.set` inside userdata impl
			}
			LuaFunctionOwner::Module(visiting_context.current_module.clone())
		}
		_ => {
			LuaFunctionOwner::UserData(visiting_context.current_userdata.clone().unwrap())
		}
	};

	extract_closure_signature(&args, function_context, visiting_context, stats, owner, &lua_name, kind, closure);
}

fn is_mlua_multivalue(ty: &syn::Type) -> bool { //cspell:ignore multivalue
	match ty {
		syn::Type::Path(tp) => {
			tp.path.segments.last().is_some_and(|seg| {
				seg.ident.to_string() == "MultiValue"
		})
		}
		_ => false,
	}
}

fn fn_signature_param(args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, _stats: &mut Stats, ty: Option<syn::Type>, pat: syn::Pat) -> LuaParam {
	LuaParam {
		name: Some(if args.debug.unwrap_or_default() > 0
				{format!("/* {} */ {}", pat.type_str(),	pat.to_src_debug(args.debug.unwrap_or_default(), &function_context.source))	}
			else{										pat.to_src_debug(args.debug.unwrap_or_default(), &function_context.source)	}),
		lua_type: match ty {
			Some(t) => map_rust_type(&t, visiting_context),
			None => LuaType::Any,
		},
	}
}

fn extract_fn_signature(args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, stats: &mut Stats, owner: LuaFunctionOwner, lua_name: &str, kind: LuaFunctionKind, func: &syn::ItemFn) {
	let mut params = Vec::new();

	for (idx, input) in func.sig.inputs.iter().enumerate() {
		// mlua functions always have lua context first //cspell:ignore mlua
		if idx == 0 {
			continue;
		}

		match input {
			syn::FnArg::Typed(pat_type) => {
				if is_mlua_multivalue(&pat_type.ty) {
					params.push(LuaParam {
						name: None,
						lua_type: LuaType::Variadic(Box::new(LuaType::Any)),
					});
					break; // must be last parameter
				}
				
				match (&*pat_type.pat, &*pat_type.ty) {
					(syn::Pat::Tuple(pat_tuple), syn::Type::Tuple(ty_tuple)) => {
						for (pat, ty) in pat_tuple.elems.iter().zip(ty_tuple.elems.iter()) {
							if is_mlua_multivalue(ty) {
								params.push(LuaParam {
									name: None,
									lua_type: LuaType::Variadic(Box::new(LuaType::Any)),
								});
								break;
							}
							params.push(fn_signature_param(args, function_context, visiting_context, stats, Some(ty.clone()), pat.clone()));
						}
					}

					_ => {
						if is_mlua_multivalue(&pat_type.ty) {
							params.push(LuaParam {
								name: None,
								lua_type: LuaType::Variadic(Box::new(LuaType::Any)),
							});
						} else {
							params.push(fn_signature_param(args, function_context, visiting_context, stats, Some(*pat_type.ty.clone()), *pat_type.pat.clone()))
						}
					}
				}
			}
			syn::FnArg::Receiver(_) => {},
		}
	}

	let returns = match &func.sig.output {
		syn::ReturnType::Default => vec![],
		syn::ReturnType::Type(_, ty) => vec![map_rust_type(ty, visiting_context)],
	};

	let func = LuaFunction {
		file: function_context.file_path.clone(),
		name: lua_name.to_string(),
		kind,
		owner,
		params,
		returns,
		is_async: func.sig.asyncness.is_some(),
	};

	visiting_context.lua_functions.insert(func.key(), func);
}

fn extract_closure_signature(args: &Args, function_context: &FunctionContext, visiting_context: &mut VisitingContext, _stats: &mut Stats, owner: LuaFunctionOwner, lua_name: &str, kind: LuaFunctionKind, closure: &syn::ExprClosure) {
	let mut params = Vec::new();

	for (idx, input) in closure.inputs.iter().enumerate() {
		if idx == 0 {
			continue;
		}

		match input {
			syn::Pat::Type(pat_type) => {
				match pat_type.pat.as_ref() {
					syn::Pat::Tuple(tuple) => {
						if let syn::Type::Tuple(ty_tuple) = &*pat_type.ty {
							for (pat, ty) in tuple.elems.iter().zip(ty_tuple.elems.iter()) {
								if is_mlua_multivalue(ty) {
									params.push(LuaParam {
										name: None,
										lua_type: LuaType::Variadic(Box::new(LuaType::Any)),
									});
									break;
								}

								if let syn::Pat::Ident(ident) = pat {
									params.push(LuaParam {
										name: Some(ident.ident.to_string()),
										lua_type: map_rust_type(ty, visiting_context),
									});
								} else {
									params.push(LuaParam {
										name: Some("_".to_string()),
										lua_type: LuaType::Any,
									});
								}
							}
						}
					}
					syn::Pat::Ident(ident) => {
						params.push(LuaParam {
							name: Some(ident.ident.to_string()),
							lua_type: map_rust_type(&pat_type.ty, visiting_context),
						});
					}
					_ => {}
				}
			}

			syn::Pat::Ident(ident) => {
				params.push(LuaParam {
					name: Some(ident.ident.to_string()),
					lua_type: LuaType::Any,
				});
			}

			_ => {}
		}
	}

	let mut returns = extract_return_from_signature(visiting_context, closure);
	if returns.is_empty() {
		returns = infer_return_from_body(args, function_context, visiting_context, &closure.body);
	}

	let func = LuaFunction {
		file: function_context.file_path.clone(),
		name: lua_name.to_string(),
		kind,
		owner,
		params,
		returns,
		is_async: closure.asyncness.is_some(), //cspell:ignore asyncness
	};

	visiting_context.lua_functions.insert(func.key(), func);
}

fn extract_return_from_signature(visiting_context: &VisitingContext, closure: &syn::ExprClosure) -> Vec<LuaType> {
	match &closure.output {
		syn::ReturnType::Default => vec![],

		syn::ReturnType::Type(_, ty) => match &**ty {
			syn::Type::Tuple(t) => t
				.elems
				.iter()
				.map(|t| map_rust_type(t, visiting_context))
				.collect(),

			other => vec![map_rust_type(other, visiting_context)],
		},
	}
}


fn infer_return_from_body(args: &Args, function_context: &FunctionContext, visiting_context: &VisitingContext, body: &syn::Expr) -> Vec<LuaType> {
	match body {
		syn::Expr::Block(b) => {
			for stmt in &b.block.stmts {
				if let syn::Stmt::Expr(syn::Expr::Return(ret), _) = stmt {
					if let Some(expr) = &ret.expr {
						return infer_return_from_expr(args, function_context, visiting_context, expr);
					}
				}
			}
			if let Some(last) = b.block.stmts.last() {
				if let syn::Stmt::Expr(expr, _) = last {
					return infer_return_from_expr(args, function_context, visiting_context, expr);
				}
			}

			vec![]
		}

		syn::Expr::Async(a) => {
			infer_return_from_block(args, function_context, visiting_context, &a.block)
		}

		_ => vec![LuaType::Unknown(span_src_debug(args.debug.unwrap_or_default(), body.span(), &function_context.source))]

	}
}

fn infer_return_from_block(args: &Args, function_context: &FunctionContext, visiting_context: &VisitingContext, block: &syn::Block) -> Vec<LuaType> {
	for stmt in &block.stmts {
		if let syn::Stmt::Expr(syn::Expr::Return(ret), _) = stmt {
			if let Some(expr) = &ret.expr {
				return infer_return_from_expr(args, function_context, visiting_context, expr);
			}
		}
	}
	if let Some(last) = block.stmts.last() {
		if let syn::Stmt::Expr(expr, _) = last {
			return infer_return_from_expr(args, function_context, visiting_context, expr);
		}
	}

	vec![LuaType::Unknown(span_src_debug(args.debug.unwrap_or_default(), block.span(), &function_context.source))]
}

fn infer_return_from_expr(args: &Args, function_context: &FunctionContext, visiting_context: &VisitingContext, expr: &Expr) -> Vec<LuaType> {
	match expr {
		Expr::Async(async_expr) => {
			infer_return_from_block(args, function_context, visiting_context, &async_expr.block)
		}

		Expr::Block(b) => infer_return_from_block(args, function_context, visiting_context, &b.block),

		Expr::Call(call) => {
			if let Expr::Path(p) = &*call.func {
				if p.path.segments.last().unwrap().ident == "Ok" {
					if let Some(arg) = call.args.first() {
						return infer_return_from_expr(args, function_context, visiting_context, arg);
					}
				}
			}
			vec![LuaType::Any]
		}

		Expr::Tuple(t) => t
			.elems
			.iter()
			.flat_map(|e| infer_return_from_expr(args, function_context, visiting_context, e))
			.collect(),

		Expr::Return(ret) => {
			if let Some(expr) = &ret.expr {
				infer_return_from_expr(args, function_context, visiting_context, expr)
			} else {
				vec![LuaType::Unit]
			}
		}

		Expr::MethodCall(call) => {
			match call.method.to_string().as_str() {
				"to_string" => return vec![LuaType::String],
				"len" => vec![LuaType::Number],
				"is_empty" => vec![LuaType::Boolean],

				_ => vec![LuaType::Unknown(span_src_debug(args.debug.unwrap_or_default(), call.span(), &function_context.source))],
			}
		}

		Expr::Lit(lit) => match &lit.lit {
			syn::Lit::Str(_) => vec![LuaType::String],
			syn::Lit::Bool(_) => vec![LuaType::Boolean],
			syn::Lit::Int(_) | syn::Lit::Float(_) => vec![LuaType::Number],
			_ => vec![LuaType::UserData(lit.lit.type_str().to_string())],
		},

		Expr::Field(field) => {
			if let Expr::Path(p) = &*field.base {
				if p.path.is_ident("this") {
					if let Some(userdata) = &visiting_context.current_userdata {
						if let Some(def) = visiting_context.structs.get(userdata) {
							match &field.member {
								syn::Member::Unnamed(idx) => {
									if let Some(ty) = def.tuple_fields.get(idx.index as usize) {
										return vec![map_rust_type(ty, visiting_context)];
									}
								}
								syn::Member::Named(ident) => {
									if let Some(ty) = def.named_fields.get(&ident.to_string()) {
										return vec![map_rust_type(ty, visiting_context)];
									}
								}
							}
						}
					}
				}
			}
			vec![LuaType::Any]
		}

		Expr::Path(path) => {
			vec![LuaType::Unknown(path.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("."))]
		}

		_ => vec![LuaType::Unknown(expr.type_str().to_string())],
	}
}

fn map_rust_type(ty: &syn::Type, visiting_context: &VisitingContext) -> LuaType {
	_map_rust_type(ty, visiting_context, &mut vec![])
}
fn _map_rust_type(ty: &syn::Type, visiting_context: &VisitingContext, resolving: &mut Vec<String>) -> LuaType {
	match ty {
		syn::Type::Path(p) => {
			let seg = p.path.segments.last().unwrap();
			let ident = seg.ident.to_string();
			
			if let Some(resolved) = visiting_context.type_aliases.get(&ident) {
				if resolving.contains(&ident) {
					return LuaType::Unknown(format!("alias cycle: {}", ident));
				}
				resolving.push(ident.clone());
				let result = _map_rust_type(resolved, visiting_context, resolving);
				resolving.pop();
				return result;
			}
			
			if ident == "Variadic" {
				if let syn::PathArguments::AngleBracketed(seg_args) = &seg.arguments {
					if let Some(syn::GenericArgument::Type(inner_ty)) = seg_args.args.first() {
						return LuaType::Variadic(Box::new(
							map_rust_type(inner_ty, visiting_context),
						));
					}
				}
				
				// Fallback if generics are missing or malformed
				return LuaType::Variadic(Box::new(LuaType::Any));
			}
			
			if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
				let mut generic_types =
					args.args.iter().filter_map(|arg| {
						if let syn::GenericArgument::Type(t) = arg {
							Some(t)
						} else {
							None
						}
					});
				
				match ident.as_str() {
					"Result" => {
						if let Some(ok_ty) = generic_types.next() {
							return _map_rust_type(ok_ty, visiting_context, resolving);
						}
					}
					
					"Option" => {
						if let Some(inner) = generic_types.next() {
							return LuaType::Optional(Box::new(
								_map_rust_type(inner, visiting_context, resolving),
							));
						}
					}
					
					"Variadic" => {
						if let Some(inner) = generic_types.next() {
							return LuaType::Variadic(Box::new(
								_map_rust_type(inner, visiting_context, resolving),
							));
						} else {
							return LuaType::Variadic(Box::new(LuaType::Any));
						}
					}
					
					_ => {}
				}
			}
			
			match ident.as_str() {
				"String" | "str" => LuaType::String,
				"Function" => LuaType::Function,
				"bool" => LuaType::Boolean,
				"usize"
				| "u8" | "u16" | "u32" | "u64"
				| "i8" | "i16" | "i32" | "i64"
				| "f8" | "f16" | "f32" | "f64" => LuaType::Number,
				_ => LuaType::UserData(ident),
			}
		}

		syn::Type::Reference(r) => _map_rust_type(&r.elem, visiting_context, resolving),

		syn::Type::Tuple(t) if t.elems.is_empty() => LuaType::Nil,

		_ => LuaType::Unknown(ty.type_str().to_string()),
	}
}

#[allow(dead_code)]
fn	span_src<'a>(span: proc_macro2::Span, src: &'a str) -> String {
	span_src_debug(0, span, src)
}

fn	span_src_debug<'a>(debug: i8, span: proc_macro2::Span, src: &'a str) -> String {
	let start	= span.start	();
	let end		= span.end		();

	if		start.line == 1 && start.column == 0
		&&	 end .line == 1 &&  end .column == 0
		{return "<unknown>".to_owned()}

	let mut line_count = 0;
	
	let mut last_line_byte_start = 0;
	let mut byte_start = 0;
	for line in src.split("\n") { // .lines() splits on either "\n" or "\r\n" (both if they exist in the same span, but not "\r" - unless followed by "\n") which risks the byte_* to differ by one per line...
		line_count += 1;
		if line_count < start.line {
			byte_start += line.len() + 1;
		} else if line_count == start.line + 1 {
			last_line_byte_start = byte_start;
			byte_start += start.column;
			break;
		}
	}

	let mut byte_end = byte_start;
	for line in src[byte_start..].split("\n") {
		if line_count < end.line + 1 {
			byte_end += line.len() + 1;
		} else if line_count == start.line + 1 {
			byte_end = last_line_byte_start + end.column;
			break;
		} else {
			byte_end += end.column;
			break;
		}
		line_count += 1;
	}

	let mut result = src.get(byte_start..byte_end).unwrap_or_else(|| "<unknown>").to_string();
	if debug > 3
		{result = format!("/* start.line {}, start.column {}, \
				end.line {}, end.column {}, \
				byte_start {}, byte_end {} */ {}",
				start.line.to_string(), start.column.to_string(),
				end.line.to_string(),   end.column.to_string(),
				byte_start, byte_end, result);}
	result
}

fn lua_type_to_luals(t: &LuaType) -> String { //cspell:ignore luals
	match t {
		LuaType::Value => "any".into(),
		LuaType::Any => "any".into(),
		LuaType::Nil => "nil".into(),
		LuaType::Unit => "nil".into(),
		LuaType::Boolean => "boolean".into(),
		LuaType::Number => "number".into(),
		LuaType::String => "string".into(),
		LuaType::Function => "function".into(),
		LuaType::Variadic(inner) => format!("... {}", lua_type_to_luals(inner)),
		LuaType::UserData(name) => name.clone(),
		LuaType::Optional(inner) => format!("{}?", lua_type_to_luals(inner)),
		LuaType::Unknown(comment) => {let mut c = comment.to_string();
			c = Regex::new(r"[\r\n]+"	).unwrap().replace_all(&c, ""	).to_string();
			c = Regex::new(r"\s{2,}"	).unwrap().replace_all(&c, " "	).to_string();
			format!("any --[[ {} --]]", c)}
	}
}

struct LuaApi {
	module_paths: HashSet::<Vec<String>>,
	modules: Vec<LuaFunction>,
	userdata: HashMap<String, Vec<LuaFunction>>,
}

fn build_api(functions: HashMap<LuaFunctionKey, LuaFunction>) -> LuaApi {
	let mut api = LuaApi {
		module_paths: HashSet::new(),
		modules: Vec::new(),
		userdata: HashMap::new(),
	};

	let mut module_paths = HashSet::<Vec<String>>::new();
	
	for key in functions.keys() {
		if let LuaFunctionOwnerKey::Module(path) = &key.owner {
			if !path.is_empty() {
				module_paths.insert(path.clone());
			}
		}
	}

	for path in &module_paths {
		for i in 1..=path.len() {
			api.module_paths.insert(path[..i].to_vec());
		}
	}

	for f in functions.into_values() {
		match &f.owner {
			LuaFunctionOwner::Module(_) => api.modules.push(f),
			LuaFunctionOwner::UserData(name) => {
				api.userdata.entry(name.clone()).or_default().push(f);
			}
		}
	}

	api
}

fn emit_userdata(name: &str, methods: &[LuaFunction]) -> String {
	let mut out = String::new();

	out.push_str(&format!("---@class {}\n", name));
	out.push_str(&format!("{} = {{}}\n\n", name));

	for f in methods {
		let params = f.params.iter().filter(|p| p.name.as_ref().is_none_or(|n| n != "this")).collect::<Vec<_>>();
		for p in &params {match &p.name {
			None	=> out.push_str(&format!("---@param {}\n", lua_type_to_luals(&p.lua_type))),
			Some(n)	=> out.push_str(&format!("---@param {} {}\n", n, lua_type_to_luals(&p.lua_type))),
		}}

		for ret in &f.returns {
			out.push_str(&format!("---@return {}\n",lua_type_to_luals(ret)));
		}

		let params = params.iter()
			.map(|p| {match &p.lua_type {
				LuaType::Variadic(_) => "...",
				_ => p.name.as_deref().unwrap_or_default(),
			}})
			.collect::<Vec<_>>().join(", ");
		out.push_str(&format!("function {}:{}({}) end\n\n", name, f.name, params));
	}

	out
}

fn emit_module_functions(name_root: &str, functions: &[LuaFunction]) -> String {
	let mut out = String::new();

	for f in functions {
		match &f.owner {
			LuaFunctionOwner::Module(path) => {
				for p in &f.params {match &p.name {
					None	=> out.push_str(&format!("---@param {}\n", lua_type_to_luals(&p.lua_type))),
					Some(n)	=> out.push_str(&format!("---@param {} {}\n", n, lua_type_to_luals(&p.lua_type))),
				}}

				for ret in &f.returns {
					out.push_str(&format!("---@return {}\n", lua_type_to_luals(ret)));
				}

				let name_full = std::iter::once(name_root)
					.chain(path.iter().map(String::as_str))
					.collect::<Vec<_>>().join(".");
				let params = f.params.iter()
					.map(|p| {match &p.lua_type {
						LuaType::Variadic(_) => "...",
						_ => p.name.as_deref().unwrap_or_default(),
					}})
					.collect::<Vec<_>>().join(", ");
				out.push_str(&format!("function {}.{}({}) end\n\n", name_full, f.name, params));
			}
			_ => {}
		}
	}

	out
}

fn emit_module_tables(name_root: &str, paths: &HashSet<Vec<String>>) -> String {
	let mut paths: Vec<_> = paths.iter().collect();

	paths.sort_by_key(|p| p.len());

	let mut out = String::new();

	for path in paths {
		let name_full = std::iter::once(name_root).chain(path.iter().map(String::as_str))
			.collect::<Vec<_>>().join(".");

		out.push_str(&format!("{} = {{}}\n", name_full));
	}

	out
}
