// TODO : split this file into tokenizer, parser, generator

use std::fs;
use std::process::exit;
use std::collections::HashMap;
use std::env;
use std::any::Any;

fn is_type(token: &String) -> bool {
	if token == "int" || token == "char" || token == "float" || token == "double" || token == "bool"{
		true
	}
	else{
		false
	}
}

fn is_numerical(token: &String)->bool{
	token.parse::<f64>().is_ok()
}

#[derive(Debug, Clone)]
struct TOKEN{
	name: String,
	token_type: String,
	line: i64,
	column :i64,
}

// TODO: pass these in a context struct instead of having them global
static mut TOKEN_INDEX: i64 = 0;
static mut STRING_INDEX: i64 = 0;
static mut LABEL_INDEX: i64 = 0;
static mut BREAK_IDX_VEC: Vec<i64> = Vec::new();
static mut LOOP_CHECK: Vec<bool> = Vec::new(); // used to track if we are in a loop
static mut COND_CHECK_LOCS: Vec<i64> = Vec::new();


fn tagging(token: &String, line: i64, column: i64) -> TOKEN{
	if is_type(&token){
		return TOKEN{name: token.to_string(), token_type: "TYPE".to_string(), line: line, column: column};
	}
	else if token == "return"{
		return TOKEN{name: token.to_string(), token_type: "RETURN".to_string(), line: line, column: column};
	}
	else if token == "("{
		return TOKEN{name: token.to_string(), token_type: "LPAREN".to_string(), line: line, column: column};
	}
	else if token == ")"{
		return TOKEN{name: token.to_string(), token_type: "RPAREN".to_string(), line: line, column: column};
	}
	else if token == "{"{
		return TOKEN{name: token.to_string(), token_type: "LBRACE".to_string(), line: line, column: column};
	}
	else if token == "}"{
		return TOKEN{name: token.to_string(), token_type: "RBRACE".to_string(), line: line, column: column};
	}
	else if token == "["{
		return TOKEN{name: token.to_string(), token_type: "LBRACK".to_string(), line: line, column: column};
	}
	else if token == "]"{
		return TOKEN{name: token.to_string(), token_type: "RBRACK".to_string(), line: line, column: column};
	}
	else if token == "+"{
		return TOKEN{name: token.to_string(), token_type: "PLUS".to_string(), line: line, column: column};
	}
	else if token == "-"{
		return TOKEN{name: token.to_string(), token_type: "MINUS".to_string(), line: line, column: column};
	}
	else if token == "*"{
		return TOKEN{name: token.to_string(), token_type: "STAR".to_string(), line: line, column: column};
	}
	else if token == "/"{
		return TOKEN{name: token.to_string(), token_type: "SLASH".to_string(), line: line, column: column};
	}
	else if token == "="{
		return TOKEN{name: token.to_string(), token_type: "ASSIGN".to_string(), line: line, column: column};
	}
	else if token == "!"{
		return TOKEN{name: token.to_string(), token_type: "NOT".to_string(), line: line, column: column};
	}
	else if is_numerical(&token){
		return TOKEN{name: token.to_string(), token_type: "NUMBER".to_string(), line: line, column: column};
	}
	else if token == ";"{
		return TOKEN{name: token.to_string(), token_type: "SEMICOLON".to_string(), line: line, column: column};
	}
	else if token == ","{
		return TOKEN{name: token.to_string(), token_type: "COMMA".to_string(), line: line, column: column};
	}
	else if token == "\'"{
		return TOKEN{name: token.to_string(), token_type: "SINGLEQUOTE".to_string(), line: line, column: column};
	}
	else if token == "\""{
		return TOKEN{name: token.to_string(), token_type: "DOUBLEQUOTE".to_string(), line: line, column: column};
	}
	else if token == "&"{
		return TOKEN{name: token.to_string(), token_type: "ADDRESSOF".to_string(), line: line, column: column};
	}
	else if token == "true" || token == "false" {
		return TOKEN{name: token.to_string(), token_type: "BOOLEAN".to_string(), line: line, column: column};
	}
	else if token == "."{
		return TOKEN{name: token.to_string(), token_type: "DECIMAL".to_string(), line: line, column: column};
	}
	else if token == "#"{
		return TOKEN{name: token.to_string(), token_type: "PREPROCESSORDECL".to_string(), line: line, column: column};
	}
	else if token == "define" || token == "include"{
		return TOKEN{name: token.to_string(), token_type: "PREPROCESSOR".to_string(), line: line, column: column};
	}
	else if token == "<"{
		return TOKEN{name: token.to_string(), token_type: "LCHEV".to_string(), line: line, column: column};
	}
	else if token == ">"{
		return TOKEN{name: token.to_string(), token_type: "RCHEV".to_string(), line: line, column: column};
	}
	else if token == "if"{
		return TOKEN{name: token.to_string(), token_type: "IF".to_string(), line: line, column: column};
	}
	else if token == "else"{
		return TOKEN{name: token.to_string(), token_type: "ELSE".to_string(), line: line, column: column};
	}
	else if token == "while"{
		return TOKEN{name: token.to_string(), token_type: "WHILE".to_string(), line: line, column: column};
	}
	else if token == "for"{
		return TOKEN{name: token.to_string(), token_type: "FOR".to_string(), line: line, column: column};
	}
	else if token == "struct"{
		return TOKEN{name: token.to_string(), token_type: "STRUCT".to_string(), line: line, column: column};
	}
	else if token == "break"{
		return TOKEN{name: token.to_string(), token_type: "BREAK".to_string(), line: line, column: column};
	}
	else if token == "continue"{
		return TOKEN{name: token.to_string(), token_type: "CONTINUE".to_string(), line: line, column: column};
	}
	else if token == "sizeof"{
		return TOKEN{name: token.to_string(), token_type: "SIZEOF".to_string(), line: line, column: column};
	}
	else {
		return TOKEN{name: token.to_string(), token_type: "NAME".to_string(), line: line, column: column};
	}
}

fn get_token(token: &Vec<TOKEN>) -> &TOKEN{
	unsafe{
		let new_token = &token[TOKEN_INDEX as usize];
		return new_token;
	}
}

fn next_token(){
	unsafe{
		TOKEN_INDEX += 1;
	}
}

fn get_precedence(token_type: &str) -> u8 {
    match token_type {
        "STAR" | "SLASH" => 3, // Hight priority
        "PLUS" | "MINUS" => 2, // Low priority
        "ASSIGN" | "NOT" | "RCHEV" | "LCHEV" => 1,
        _ => 0,                // Not an operator
    }
}

fn get_offset(var_type: &Type) -> i64{
	match var_type{
		Type::Char => 1,
		Type::Bool => 1,
		Type::Int => 4,
		Type::Float => 4,
		Type::Double => 8,
		Type::Pointer(_) => 8,
		Type::Array(arr_type, size) => get_offset(arr_type)*size,
		Type::Struct(_) => 0,
		Type::Unknown => 0,
	}
}

fn parsing_error(cause: &str, tokens: &Vec<TOKEN>){
	let token = get_token(tokens);
	println!("Error : (l.{}:{}) {cause} !", token.line, token.column);
	exit(1);
}

fn compiler_panic(cause :&str){
	println!("Error : {cause} !");
	exit(1);
}

fn expect(tokens: &Vec<TOKEN>, expected: &TOKEN) -> TOKEN {
	let current_token = get_token(tokens);
	next_token();
	if current_token.token_type == expected.token_type{
		return current_token.clone();
	}
	else{
		parsing_error(&format!("Expected {0} : \"{1}\", got {2} : \"{3}\"", expected.token_type, expected.name, current_token.token_type, current_token.name), tokens);
	}
	return TOKEN{name: "THIS SHOULD NEVER HAPPEN".to_string(), token_type: "THIS SHOULD NEVER HAPPEN".to_string(), line: 0, column: 0};
}

#[derive(Debug, Clone, PartialEq)]
enum BinaryOp{
	PLUS,
	MINUS,
	MULTIPLICATION,
	DIVISION,
	EQUAL,
	NEQUAL,
	GREATER,
	GREATEREQ,
	LESSER,
	LESSEREQ,
	UNKNOWN,
}

#[derive(Debug, Clone, PartialEq)]
enum Type{
	Int,
	Char,
	Bool,
	Float,
	Double,
	Pointer(Box<Type>),
	Array(Box<Type>, i64),
	Struct(String),
	Unknown,
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
	IntLiteral(i64),
	FloatLiteral(f32),
	DoubleLiteral(f64),
	CharLiteral(u8),
	StringLiteral(String, i64),
    BoolLiteral(bool),
	Variable(String),
	Binary(Box<Expr>,BinaryOp, Box<Expr>),
	Call(String, Vec<Expr>),
	Dereference(String),
	AddressOf(String),
	Struct(String, String),
	SizeofType(Type),
	SizeofVar(String),
	None,
	Unknown,
}

#[derive(Debug, Clone, PartialEq)]
enum Statement {
	Declaration(Type, String, Expr),
	Return(Expr),
	Call(String, Vec<Expr>),
	Reassign(Expr, Expr),
	If(Expr, Vec<Statement>, i64),
	Else(Vec<Statement>, i64),
	While(Expr, Vec<Statement>, i64),
	Struct(String, Vec<Statement>),
	BlockScope(Vec<Statement>),
	Break(i64),
	Continue,
	Unknown,
}

#[derive(Debug, Clone)]
struct FunctionArgDeclaration{
	var_type: Type,
	name: String,
}

#[derive(Debug)]
struct Function{
	name: String,
	args: Vec<FunctionArgDeclaration>,
	body: Vec<Statement>,
}

#[derive(Debug)]
struct Program{
	functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq)]
struct VarInfo{
	offset: i64,
	var_type: Type,
	//optionnal for structs only, decratarion statement + offset
	struct_fields: Vec<(Statement, i64)>,
}

struct VarTable{
	symbole_table: Vec<HashMap<String, VarInfo>>,
	stack_offset: i64,
}

struct AssemblyStructureFunction{
	header: String,
	body: Vec<String>,
}

struct AssemblyStructure{
	data: String,
	text: Vec<AssemblyStructureFunction>,
	function_index: usize,
}

struct GenContext{
	in_a_binary: bool,
	pointer_in_binary: bool,
	call_context: String,
	return_context: Box<dyn Any>,
}

fn parse_type(token: &TOKEN) -> Type{
	match token.name.as_str(){
		"int" => Type::Int,
		"bool" => Type::Bool,
		"char" => Type::Char,
		"float" => Type::Float,
		"double" => Type::Double,
		_ => Type::Unknown,
	}

}

fn parse_call(fn_name: &TOKEN, tokens: &Vec<TOKEN>) -> (String, Vec<Expr>){
	next_token();
	let mut arguments = Vec::new();
	if get_token(tokens).token_type != "RPAREN"{
		loop {
			arguments.push(parse_expression(tokens, 0));
			if get_token(tokens).token_type == "COMMA"{
				next_token();
			}
			else if get_token(tokens).token_type == "RPAREN"{
				break;
			}
			else{
				// println!("Parsing call to {} with arguments {:?}", );
				parsing_error(&format!("While parsing call to {} with arguments {:?}, expected COMMA : \",\" or RPAREN : \")\", got {} : \"{}\"", fn_name.name, arguments, get_token(tokens).token_type, get_token(tokens).name), tokens);
			}
		}
	}
	expect(tokens, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
	return (fn_name.name.clone(), arguments);
}

fn update_string_index()->i64{
	unsafe{
		let index = STRING_INDEX;
		STRING_INDEX+=1;
		return index;
	}
}
fn update_label_index()->i64{
	unsafe{
		let index = LABEL_INDEX;
		LABEL_INDEX+=1;
		return index;
	}
}

fn pop_last_break_idx() -> Option<i64>{
	// let idx = ;
	unsafe{
		return BREAK_IDX_VEC.pop();
	}
}

fn append_break_idx(idx: i64){
	// let idx = ;
	unsafe{
		return BREAK_IDX_VEC.push(idx);
	}
}

fn parse_bool(string: &str) -> bool{
	match string{
		"true"=>true,
		"false"=>false,
		_=>{
			compiler_panic(&format!("{string} is not a boolean"));
			false
		}
	}
}

fn parse_primary(token: &Vec<TOKEN>)->Expr{
	let primary = get_token(&token);
	next_token();
	match primary.token_type.as_str() {
		"NUMBER" => {
			return Expr::IntLiteral(primary.name.parse::<i64>().unwrap());
		},
		"NAME" => {
			if get_token(token).token_type == "LPAREN"{
				let call = parse_call(&primary, token);
				return Expr::Call(call.0, call.1);
			}
			else if get_token(token).token_type == "DECIMAL"{
				next_token();
				let field_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0}).name;
				return Expr::Struct(primary.name.clone(), field_name);
			}
			else{
				return Expr::Variable(primary.name.clone());
			}
		}
		"SINGLEQUOTE" => {
			let mut character = get_token(&token).name.as_bytes()[0];
			if get_token(&token).name.len() > 1{
				if get_token(&token).name == "\\n"{character = b'\n'; println!("replaced nl : {character}");}
				else if get_token(&token).name == "\\\\"{character = b'\\'}
				else if get_token(&token).name == "\\'"{character = b'\''}
				else if get_token(&token).name == "\\\""{character = b'\"'}
			}
			let char_literal = Expr::CharLiteral(character);
			next_token();
			expect(token, &TOKEN{name: "\'".to_string(), token_type: "SINGLEQUOTE".to_string(), line: 0, column: 0});
			return char_literal;
		}
		"DOUBLEQUOTE"=>{
			let string = expect(token, &TOKEN{name: "Any String".to_string(), token_type: "STRING".to_string(), line: 0, column: 0});
			let string_litteral = Expr::StringLiteral(string.name, update_string_index());
			expect(token, &TOKEN{name: "\"".to_string(), token_type: "DOUBLEQUOTE".to_string(), line: 0, column: 0});
			return string_litteral;
		}
		"BOOLEAN" => {
			return Expr::BoolLiteral(parse_bool(&primary.name));
		}
		"NOT" => {
			parsing_error("Not operation is not yet implemented", token);
			return Expr::Unknown
		}
		"STAR" => {
			let variable = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
			return Expr::Dereference(variable.name);
		}
		"ADDRESSOF" => {
			let variable = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
			return Expr::AddressOf(variable.name);
		}
		"SIZEOF" =>{
			// TODO : only with parents when type, and also support for like char* pointers and typedefs with structs and type renaming
			// if get_token(token).token_type == "TYPE"{
			// 	let get_offset(parse_type(get_token(token));
			// }
			if get_token(token).token_type == "NAME"{
				let var_name = &get_token(token).name;
				next_token();
				return Expr::SizeofVar(var_name.to_string());
			}
			else if get_token(token).token_type == "LPAREN"{
				next_token();
				if get_token(token).token_type == "NAME"{
					let var_name = &get_token(token).name;
					next_token();
					expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
					return Expr::SizeofVar(var_name.to_string());
				}
				else if get_token(token).token_type == "TYPE"{
					let mut var_type = parse_type(get_token(token));
					next_token();
					// TODO : do this in parse_type
					// also supoport pointers of pointers
					if get_token(&token).token_type == "STAR"{
						var_type = Type::Pointer(Box::new(var_type));
						next_token();
					}
					expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
					return Expr::SizeofType(var_type);
				}
				else{
					parsing_error(&format!("With sizeof, expected NAME or TYPE, got {}", get_token(token).token_type), token);
					return Expr::Unknown;
				}
			}
			else{
				parsing_error(&format!("With sizeof, expected NAME or, TYPE inside paren, got {}\nHint: try \"sizeof ({})\"", get_token(token).token_type, get_token(token).name), token);
				return Expr::Unknown;
			}
		}
		_ => {
			parsing_error(&format!("{} is not a valid primary expression", primary.token_type), token);
			return Expr::Unknown;
		}
	}
}

fn parse_expression(token: &Vec<TOKEN>, min_precedence: u8) -> Expr{
	let mut left_expr = parse_primary(token);
	loop{
		let next_token_peek = get_token(token);
		let precedence = get_precedence(&next_token_peek.token_type);
		println!("for token {} precedence is : {}", next_token_peek.token_type, precedence);
		if precedence == 0 || precedence < min_precedence{
			break;
		}
		next_token();
		let op = match next_token_peek.token_type.as_str(){
			"PLUS" => BinaryOp::PLUS,
			"MINUS" => BinaryOp::MINUS,
			"STAR" => BinaryOp::MULTIPLICATION,
			"SLASH" => BinaryOp::DIVISION,
			"ASSIGN" => {
				if get_token(token).token_type == "ASSIGN"{
					next_token();
					BinaryOp::EQUAL
				}
				else{parsing_error("Expected comparison, got assign", token); BinaryOp::UNKNOWN}
			},
			"NOT" => {
				if get_token(token).token_type == "ASSIGN"{
					next_token();
					BinaryOp::NEQUAL
				}
				else if get_token(token).token_type == "NAME"{
					parsing_error("Not-ing variable not yet implemented", token);BinaryOp::UNKNOWN
				}
				else{parsing_error("Expected comparison, got assign", token); BinaryOp::UNKNOWN}
			},
			"LCHEV"=>{
				if get_token(token).token_type == "ASSIGN"{
					next_token();
					BinaryOp::LESSEREQ
				}
				else{
					BinaryOp::LESSER
				}
			}
			"RCHEV"=>{
				if get_token(token).token_type == "ASSIGN"{
					next_token();
					BinaryOp::GREATEREQ
				}
				else{
					BinaryOp::GREATER
				}
			}
			_ => BinaryOp::UNKNOWN,
		};
		let right_expr = parse_expression(token, precedence+1);

		left_expr = Expr::Binary(Box::new(left_expr.clone()), op, Box::new(right_expr));
	}
	left_expr
}

fn parse_statement(token: &Vec<TOKEN>) -> Statement{
	let current_token = get_token(token);
	next_token();
	if current_token.token_type == "TYPE"{
		let mut var_type = parse_type(current_token);
		// TODO : do this in parse_type
		if get_token(&token).token_type == "STAR"{
			var_type = Type::Pointer(Box::new(var_type));
			next_token();
		}
		let var_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
		let mut expr = Expr::None;
		if get_token(&token).token_type == "ASSIGN"{
			next_token();
			expr = parse_expression(token, 0);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		}
		else if get_token(&token).token_type == "LBRACK"{
			next_token();
			let size = get_token(&token).name.parse::<i64>().unwrap();
			next_token();
			// expr = parse_expression(token, 0);
			expect(token, &TOKEN{name: "]".to_string(), token_type: "RBRACK".to_string(), line: 0, column: 0});
			var_type = Type::Array(Box::new(var_type), size);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		}
		else if get_token(&token).token_type == "SEMICOLON"{
			next_token();
		}
		else{
			expr = Expr::Unknown;
		}
		let a = Statement::Declaration(var_type, var_name.name, expr);
		println!("{:?}", a);
		return a;

	}
	else if current_token.token_type == "RETURN"{
		let expression = parse_expression(token, 0);
		println!("Expression : \n{:?}", expression);
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		return Statement::Return(expression);
	}
	else if current_token.token_type == "NAME"{
		if get_token(token).token_type == "LPAREN"{
			let call = parse_call(current_token, token);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
			return Statement::Call(call.0, call.1);
		}
		else if get_token(token).token_type == "ASSIGN"{
			next_token();
			let expr = parse_expression(token, 0);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
			return Statement::Reassign(Expr::Variable(current_token.name.clone()), expr)
		}
		else if get_token(token).token_type == "DECIMAL"{
			// println!("struct reassing");
			let struct_instance_name = current_token.name.clone();
			next_token();
			let field_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
			expect(token, &TOKEN{name: "=".to_string(), token_type: "ASSIGN".to_string(), line: 0, column: 0});
			let expression = parse_expression(token, 0);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
			return Statement::Reassign(Expr::Struct(struct_instance_name, field_name.name), expression);
		}
		else if get_token(token).token_type == "PLUS"{
			next_token();
			if get_token(token).token_type == "PLUS"{
				next_token();
				if get_token(token).token_type == "SEMICOLON"{
					next_token();
				} else if get_token(token).token_type != "RPAREN" {
					compiler_panic(&format!("Expected SEMICOLON or RPAREN, got {}", get_token(token).token_type));
				}
				return Statement::Reassign(Expr::Variable(current_token.name.clone()), Expr::Binary(Box::new(Expr::Variable(current_token.name.clone())), BinaryOp::PLUS, Box::new(Expr::IntLiteral(1))));
			}
		}
		else if get_token(token).token_type == "SEMICOLON"{
			// don't throw an error, but don't do anything
			next_token();
			return Statement::Unknown;
		}
		parsing_error(&format!("Statement invalid, after name expected LPAREN or ASSIGN, got {} : {}", get_token(token).token_type, get_token(token).name), token);
		return Statement::Unknown;
	}
	else if current_token.token_type == "IF" || current_token.token_type == "WHILE"{
		expect(token, &TOKEN{name: "(".to_string(), token_type: "LPAREN".to_string(), line: 0, column: 0});
		let expression = parse_expression(token, 0);
		expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
		expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACE".to_string(), line: 0, column: 0});
		let mut body = Vec::new();
		while get_token(token).token_type != "RBRACE"{
			body.push(parse_statement(token));
		}
		println!("body : {:?}", body);
		expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
		let idx = update_label_index();
		match current_token.token_type.as_str(){
			"IF"=>{return Statement::If(expression, body, idx);}
			"WHILE"=>{return Statement::While(expression, body, idx);}
			_ => {parsing_error("I really should move away from string token type and implement an enum", token);Statement::Unknown}
		}
	}
	else if current_token.token_type == "ELSE"{
		expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACE".to_string(), line: 0, column: 0});
		let mut body = Vec::new();
		let idx = update_label_index();
		while get_token(token).token_type != "RBRACE"{
			body.push(parse_statement(token));
		}
		expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
		return Statement::Else(body, idx);
	}
	else if current_token.token_type == "STRUCT"{
		let name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0}).name;
		println!("struct name : {}", name);
		if get_token(&token).token_type == "LBRACE"{
			next_token();
			// expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACE".to_string()});
			let mut declarations = Vec::new();
			while get_token(token).token_type != "RBRACE"{
				let statement = parse_statement(token);
				match statement{
					Statement::Declaration(_,_,ref expr)=>{
						match expr{
							Expr::None=>{
								declarations.push(statement);
							}
							_=>{
								parsing_error("Expected declaration without value", token);
							}
						}
					}
					_ => {
						parsing_error(&format!("Expected a variable declaration, got {:?}", statement), token);
					}
				}
			}
			expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
			return Statement::Struct(name, declarations);
		}
		else if get_token(&token).token_type == "NAME"{
			let var_name = get_token(&token);
			next_token();
			// for now only empry declaration
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
			return Statement::Declaration(Type::Struct(name), var_name.name.clone(), Expr::None);
		}
		else{parsing_error(&format!("invalid structure declaration, found token : {:?}", get_token(&token).token_type), token);Statement::Unknown}
	}
	else if current_token.token_type == "LBRACE"{
		let mut body = Vec::new();
		while get_token(token).token_type != "RBRACE"{
			body.push(parse_statement(token));
		}
		expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
		return Statement::BlockScope(body);
	}
	else if current_token.token_type == "FOR"{
		expect(token, &TOKEN{name: "(".to_string(), token_type: "LPAREN".to_string(), line: 0, column: 0});
		let mut block_scope_body = Vec::new();
		// TODO : support for if just for(i; i<...), instead of (int i =0;...)
		// also support syntax for (;;)
		block_scope_body.push(parse_statement(token));// parse int i = 0; or just i = 0
		// expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		let condition = parse_expression(token, 0);
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		let i_reassigment = parse_statement(token);
		expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
		expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACE".to_string(), line: 0, column: 0});
		let mut for_loop_body = Vec::new();
		while get_token(token).token_type != "RBRACE"{
			for_loop_body.push(parse_statement(token));
		}
		for_loop_body.push(i_reassigment);
		let idx = update_label_index();
		block_scope_body.push(Statement::While(condition, for_loop_body, idx));
		expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
		return Statement::BlockScope(block_scope_body);
	}
	else if current_token.token_type == "BREAK"{
		let break_statement = Statement::Break(update_label_index());
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		return break_statement;
	}
	else if current_token.token_type == "CONTINUE"{
		let break_statement = Statement::Continue;
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string(), line: 0, column: 0});
		return break_statement;
	}
	else{
		parsing_error(&format!("Invalid token found in statement : {} : {}", current_token.token_type, current_token.name), token);
		return Statement::Unknown;
	}	
}

fn parse_function(token : &Vec<TOKEN>) -> Function{
	expect(token, &TOKEN{name: "Any Type".to_string(), token_type: "TYPE".to_string(), line: 0, column: 0});
	let fn_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
	expect(token, &TOKEN{name: "(".to_string(), token_type: "LPAREN".to_string(), line: 0, column: 0});
	let mut arguments = Vec::new();
	if get_token(&token).token_type != "RPAREN"{
		loop{
			if is_type(&get_token(&token).name){
				let mut var_type = parse_type(get_token(&token));
				next_token();
				if get_token(&token).token_type == "STAR"{
					var_type = Type::Pointer(Box::new(var_type));
					next_token();
				}
				let argument = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
				arguments.push(FunctionArgDeclaration{var_type: var_type, name: argument.name});
				if get_token(&token).token_type == "COMMA"{
					next_token();
				}
				else if get_token(&token).token_type == "RPAREN"{
					break;
				}
				else{
					parsing_error(&format!("Expected RPAREN : \")\" or COMMA : \",\", got {}", get_token(&token).token_type), token);
				} 
			}
			else{
				parsing_error(&format!("Expected TYPE, got {}", get_token(&token).token_type), token);
			}
		}
	}
	expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string(), line: 0, column: 0});
	expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACE".to_string(), line: 0, column: 0});
	let mut body = Vec::new();
	while get_token(token).token_type != "RBRACE"{
		body.push(parse_statement(token));
	}
	expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACE".to_string(), line: 0, column: 0});
	return Function{name: fn_name.name, args: arguments, body: body};
}

fn get_scope_variable(variable_table: &mut VarTable, var_name: &str) -> Option<VarInfo>{
	for scope in variable_table.symbole_table.iter().rev(){
		if let Some(v) = scope.get(var_name){
			return Some(v.clone());
		}
	}
    compiler_panic(&format!("Variable \"{}\" used before declaration", var_name));
    return None;
}

fn gen_call(name: &String, args: &Vec<Expr>, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure, context: &mut GenContext){
	if args.len() > 0{
		let regs64 = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
		let regs32 = vec!["edi", "esi", "edx", "ecx", "r8d", "r9d"];
		let mut fn_calls:Vec<(Expr, usize)> = Vec::new();
		for i in 0..args.len(){
			match args[i]{
				Expr::Call(_, _)=>{
					fn_calls.push((args[i].clone(), i));
				}
				_=>{/*do not treat the rest*/}
			}
		}
		for (call, i) in fn_calls {
			match call{
				Expr::Call(name, args)=>{
					gen_call(&name, &args, variable_table, asm_struct, context);
    				asm_struct.text[asm_struct.function_index].body.push(format!("    mov {}, eax\n", regs32[i]));
				}
				_=>{/*do not treat*/}
			}
		}

		for i in 0..args.len(){
    		match &args[i]{
    			Expr::AddressOf(_) => {
    				gen_expr(&args[i], variable_table, asm_struct, context);
    				asm_struct.text[asm_struct.function_index].body.push(format!("    mov {}, rax\n", regs64[i]));
    			}
    			Expr::Variable(name)=>{
    				gen_expr(&args[i], variable_table, asm_struct, context);
    				let var = get_scope_variable(variable_table, name);
    				let var_type = var.unwrap().var_type.clone();
    				match var_type{
    					Type::Array(_, _) => {
	    					asm_struct.text[asm_struct.function_index].body.push(format!("    mov {}, rax\n", regs64[i]));
    					}
    					_ => {
	    					asm_struct.text[asm_struct.function_index].body.push(format!("    mov {}, eax\n", regs32[i]));
    					}
    				}
    				// println!("!!!!!!!!!variable type : {:?}", var_type);
    			}
    			Expr::Call(_,_)=>{/*do not treat*/}
    			_ => {
    				gen_expr(&args[i], variable_table, asm_struct, context);
	    			asm_struct.text[asm_struct.function_index].body.push(format!("    mov {}, eax\n", regs32[i]));
    			}
    		}
		}
	}
	// since printf is a function that takes variadic arguments, eax needs to hold the number of floating point numbers in the arguments
	// i did not yet implement type checking to detect variadics so we manually check if the function is printf and allways null out eax
	// since we also don't support floats yet
	if name == "printf" || name == "sprintf"{
		asm_struct.text[asm_struct.function_index].body.push("    xor eax, eax\n".to_string());
	}
	asm_struct.text[asm_struct.function_index].body.push(format!("    call {}\n", name));
}

fn store_type(var_type: &Type, offset: i64, asm_struct: &mut AssemblyStructure){
	match var_type{
		Type::Bool | Type::Char => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    mov BYTE PTR [rbp {}], al\n", offset));
		}
		Type::Pointer(_) => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    mov QWORD PTR [rbp {}], rax\n", offset));
		}
		Type::Array(_,_) => {
			println!("array declaraion, do nothing");
		}
		// Type::Struct(name) =>{
		// 	// println!("AAAAAAAAAAA");
		// }
		_ =>{
			asm_struct.text[asm_struct.function_index].body.push(format!("    mov [rbp {}], eax\n", offset));
		}
	}
}

fn get_type(var_type: &Type, offset: i64, asm_struct: &mut AssemblyStructure){
	match var_type{
		Type::Int => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, [rbp {}]\n", offset));
		}
		Type::Char | Type::Bool => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    movzx eax, BYTE PTR [rbp {}]\n", offset));
		}
		Type::Pointer(_) => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    mov rax, QWORD PTR [rbp {}]\n", offset));
		}
		Type::Array(_, _) => {
			asm_struct.text[asm_struct.function_index].body.push(format!("    lea rax, [rbp {}]\n", offset));
		}
		_ =>{
			compiler_panic(&format!("Variable Type {:?} Not Yet Implemented", var_type));
		}
	}
}

fn gen_expr(expr: &Expr, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure, context: &mut GenContext) {
    match expr {
        Expr::IntLiteral(n) => asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", n)),
        Expr::BoolLiteral(n) => asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", *n as u8)),
        Expr::CharLiteral(n) => asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", n)),
        Expr::Binary(left, op, right) =>{
        	context.in_a_binary = true;
			context.call_context = "left".to_string();
        	gen_expr(left, variable_table, asm_struct, context);
        	asm_struct.text[asm_struct.function_index].body.push("    push rax\n".to_string());
			
			context.call_context = "right".to_string();
    		gen_expr(right, variable_table, asm_struct, context);
        	asm_struct.text[asm_struct.function_index].body.push("    pop rcx\n".to_string());
        	match op {
        		BinaryOp::PLUS | BinaryOp::MINUS =>{
        			if context.pointer_in_binary {
        				println!("FOUND BINARY ADD WITH POINTER : {:?}", context.return_context.downcast_ref::<(String, Type)>());
        				context.pointer_in_binary = false;
        				if let Some(ret_ctx) = context.return_context.downcast_ref::<(String, Type)>(){
	        				if  context.return_context.downcast_ref::<(String, Type)>().unwrap().0 == "left"{
	        					println!("AND ITS THE LEFT ONE (THE POINTER) : {:?}", context.return_context.downcast_ref::<(String, Type)>().unwrap().1);
	        					// asm_struct.text[asm_struct.function_index].body.push("    push rax\n".to_string());
	        					asm_struct.text[asm_struct.function_index].body.push("    push rcx\n".to_string()); // save pointer to stack
	        					match &context.return_context.downcast_ref::<(String, Type)>().unwrap().1{
									Type::Pointer(var_type)=>{
	        							asm_struct.text[asm_struct.function_index].body.push(format!("    mov ecx, {} \n", get_offset(&var_type))); // get size of type of pointer
									}
									_=>{compiler_panic("THIS SHOULD NEVER HAPPEN");}
								}
	        					asm_struct.text[asm_struct.function_index].body.push("    imul eax, ecx\n".to_string()); // size of type of pointer * slots to move (the "+1" in "str+1")
	        					// asm_struct.text[asm_struct.function_index].body.push("    mov eax, ecx\n".to_string());
	        					asm_struct.text[asm_struct.function_index].body.push("    pop rcx\n".to_string()); //  get pointer back
	        				}
	        				else if context.return_context.downcast_ref::<(String, Type)>() != None && context.return_context.downcast_ref::<(String, Type)>().unwrap().0 == "right"{
	        					println!("AND ITS THE RIGHT ONE (THE POINTER) : {:?}", context.return_context.downcast_ref::<(String, Type)>().unwrap().1);
								asm_struct.text[asm_struct.function_index].body.push("    push rax\n".to_string()); // save pointer to stack
								match &context.return_context.downcast_ref::<(String, Type)>().unwrap().1{
									Type::Pointer(var_type)=>{
	        							asm_struct.text[asm_struct.function_index].body.push(format!("    mov rax, {} \n", get_offset(&var_type))); // get size of type of pointer
									}
									_=>{compiler_panic("THIS SHOULD NEVER HAPPEN");}
								}
	        					asm_struct.text[asm_struct.function_index].body.push("    imul ecx, eax\n".to_string()); // size of type of pointer * slots to move (the "+1" in "1+str")
	        					// asm_struct.text[asm_struct.function_index].body.push("    mov eax, ecx\n".to_string());
	        					asm_struct.text[asm_struct.function_index].body.push("    pop rax\n".to_string()); //  get pointer back
	        				}
	        				else{}
        				}
        			}
        		}
        		_=>{}
        	}
        	match op{
        		BinaryOp::PLUS=>{
        			asm_struct.text[asm_struct.function_index].body.push("    add eax, ecx\n".to_string());
        		}
        		// TODO : implement pointer arithmetics like plus in minus too
        		BinaryOp::MINUS=>{
        			asm_struct.text[asm_struct.function_index].body.push("    sub ecx, eax\n".to_string());
        			asm_struct.text[asm_struct.function_index].body.push("    mov eax, ecx\n".to_string());
        		}
        		BinaryOp::MULTIPLICATION=>{
        			asm_struct.text[asm_struct.function_index].body.push("    imul rax, rcx\n".to_string());
        		}
        		BinaryOp::DIVISION=>{
        			asm_struct.text[asm_struct.function_index].body.push("    mov r10d, eax\n".to_string());
                    asm_struct.text[asm_struct.function_index].body.push("    mov eax, ecx\n".to_string());
                    asm_struct.text[asm_struct.function_index].body.push("    cdq\n".to_string());
                    asm_struct.text[asm_struct.function_index].body.push("    idiv r10d\n".to_string());
        		}
        		BinaryOp::EQUAL | BinaryOp::NEQUAL | BinaryOp::GREATER | BinaryOp::GREATEREQ | BinaryOp::LESSER | BinaryOp::LESSEREQ => {
            		asm_struct.text[asm_struct.function_index].body.push(format!("    cmp ecx, eax\n"));
            		match op {
            			BinaryOp::EQUAL=>{asm_struct.text[asm_struct.function_index].body.push(format!("    sete al\n"));}
            			BinaryOp::NEQUAL=>{asm_struct.text[asm_struct.function_index].body.push(format!("    setne al\n"));}
            			BinaryOp::GREATER=>{asm_struct.text[asm_struct.function_index].body.push(format!("    setg al\n"));}
            			BinaryOp::GREATEREQ=>{asm_struct.text[asm_struct.function_index].body.push(format!("    setge al\n"));}
            			BinaryOp::LESSER=>{asm_struct.text[asm_struct.function_index].body.push(format!("    setl al\n"));}
            			BinaryOp::LESSEREQ=>{asm_struct.text[asm_struct.function_index].body.push(format!("    setle al\n"));}
            			_ =>{compiler_panic("THIS SHOULD NEVER HAPPEN");}
            		}
            		// asm_struct.text[asm_struct.function_index].body.push(format!("    sete al\n"));
            		asm_struct.text[asm_struct.function_index].body.push(format!("    movzx eax, al\n"));
            		asm_struct.text[asm_struct.function_index].body.push(format!("    cmp eax, 0\n"));
        		}
        		BinaryOp::UNKNOWN =>{
        			compiler_panic("Somehow stumbeled uppon unknown operation, this should never happen");
        		}
        		// _ => {
        		// 	compiler_panic("Binary Operator Not Yet Implemented");
        		// }
        	}
			context.in_a_binary = false;
        } ,
        Expr::Variable(name) => {
    		let var_info = get_scope_variable(variable_table, name);
        	if context.in_a_binary {
        		// println!("IN A BINARY FROM VARIABLE THINGY");
        		match var_info.as_ref().unwrap().var_type{Type::Pointer(_)=>{context.pointer_in_binary = true; context.return_context = Box::new((context.call_context.clone(), var_info.as_ref().unwrap().var_type.clone()))}, _=>{}}
        	}
        	get_type(&var_info.as_ref().unwrap().var_type, var_info.as_ref().unwrap().offset, asm_struct);
        },
        Expr::Call(name, args) => {
        	gen_call(name, args, variable_table, asm_struct, context);
        },
        Expr::StringLiteral(string, index) => {
        	asm_struct.data.push_str(&format!(".LC{}:\n", index));
        	asm_struct.data.push_str(&format!("    .string \"{}\"\n", string));
        	asm_struct.text[asm_struct.function_index].body.push(format!("    mov rax, OFFSET FLAT: .LC{}\n", index));
        },
        Expr::Dereference(name)=>{
    		let variable = get_scope_variable(variable_table, name);
        	// println!("dereferincig type : {:?}", variable.unwrap().var_type);
        	match variable.as_ref().unwrap().var_type{
        		Type::Pointer(_)=>{
		        	asm_struct.text[asm_struct.function_index].body.push(format!("    mov rax, QWORD PTR [rbp {}]\n", variable.unwrap().offset));
		        	asm_struct.text[asm_struct.function_index].body.push(format!("    movzx eax, BYTE PTR [rax]\n"));
        		}
        		Type::Array(_,_)=>{
		        	asm_struct.text[asm_struct.function_index].body.push(format!("    movzx eax, BYTE PTR [rbp {}]\n", variable.unwrap().offset));
        		}
        		_=>{
        			compiler_panic("Cannot dereference this type");
        		}
        	}
        	asm_struct.text[asm_struct.function_index].body.push(format!("    movsx eax, al\n"));
        }
        Expr::AddressOf(_) => {
        	asm_struct.text[asm_struct.function_index].body.push(format!("    lea rax, [rbp-8]\n"));
        }
        Expr::Struct(instance_name, field_name)=>{
        	println!("instance_name : {}, field_name : {}", instance_name, field_name);
    		println!("Symbol table : {:?}", variable_table.symbole_table);
    		let struct_type_name = &get_scope_variable(variable_table, instance_name).unwrap().var_type;
    		let mut struct_name = "";
    		match struct_type_name{
    			Type::Struct(name)=>{
					struct_name = name;
    			}
    			_=>{compiler_panic("THIS SHOULD NEVER HAPPEN");}
    		}
    		let struct_info = get_scope_variable(variable_table, struct_name).unwrap();
    		for field in &struct_info.struct_fields{
    			match &field.0{
    				Statement::Declaration(var_type,name,_)=>{
    					if *name == *field_name{
        					get_type(var_type, field.1, asm_struct);
							// asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, [rbp {}]\n", ));
    					}
    				}
    				_=>{
    					compiler_panic(&format!("Expected declarations in structure, got : {:?} instead",field));
    				}
    			}
    		}
        }
        // TODO: Sizeof doesn't work on structs as get_offset returns 0
        Expr::SizeofVar(var) =>{
        	let var_type = get_scope_variable(variable_table, var).unwrap().var_type;
        	let size = get_offset(&var_type);
		    asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", size));
        }
        Expr::SizeofType(var_type)=>{
        	let size = get_offset(&var_type);
		    asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", size));
        }
        Expr::Unknown => {
        	compiler_panic("Unknown expression");
        }
        Expr::None=>{println!("encountered expr none, do nothing for now");}
        _ => {
        	compiler_panic(&format!("Expr \"{:?}\" Not yet implemented", expr));
        }
    }
}


fn gen_statement(statement: &Statement, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure, context: &mut GenContext) {
    match statement {
        Statement::Return(expr) => {
            gen_expr(expr, variable_table, asm_struct, context);
            asm_struct.text[asm_struct.function_index].body.push("    mov rsp, rbp\n".to_string());
            asm_struct.text[asm_struct.function_index].body.push("    pop rbp\n".to_string());
            asm_struct.text[asm_struct.function_index].body.push("    ret".to_string());
        }
        Statement::Declaration(var_type, name, expr) => {
            gen_expr(expr, variable_table, asm_struct, context);
            variable_table.stack_offset -= get_offset(var_type);
            let symbole_table_index = variable_table.symbole_table.len()-1;
            variable_table.symbole_table[symbole_table_index].insert(name.clone(), VarInfo{var_type: var_type.clone(), offset: variable_table.stack_offset, struct_fields: Vec::new()});
            if *expr != Expr::None{
            	store_type(var_type, variable_table.stack_offset, asm_struct);
            }
        }
        Statement::Call(name, args)=>{
        	gen_call(name, args, variable_table, asm_struct, context);
        }
        Statement::Reassign(variable, expr)=>{
        	match variable{
        		Expr::Variable(name)=>{
			        gen_expr(expr, variable_table, asm_struct, context);
		            let var_offset = get_scope_variable(variable_table, name).unwrap().offset;
					asm_struct.text[asm_struct.function_index].body.push(format!("    mov [rbp {}], eax\n", var_offset));
        		},
        		Expr::Struct(instance_name, field_name)=>{
        			println!("instance_name : {}, field_name : {}", instance_name, field_name);
            		println!("Symbol table : {:?}", variable_table.symbole_table);
			        gen_expr(expr, variable_table, asm_struct, context);
            		let struct_type_name = &get_scope_variable(variable_table, instance_name).unwrap().var_type;
            		let mut struct_name = "";
            		match struct_type_name{
            			Type::Struct(name)=>{
							struct_name = name;
            			}
            			_=>{compiler_panic("THIS SHOULD NEVER HAPPEN");}
            		}
            		let struct_fields = get_scope_variable(variable_table, struct_name).unwrap().struct_fields.clone();
            		for field in &struct_fields{
            			match &field.0{
            				Statement::Declaration(var_type,name,_)=>{
            					if *name == *field_name{
            						store_type(var_type, field.1, asm_struct);
            					}
            				}
            				_=>{
            					compiler_panic(&format!("Expected declarations in structure, got : {:?} instead",field));
            				}
            			}
            		}

        			// error("reassigning structs not yet implemented");
        		}
        		_=>{
        			compiler_panic(&format!("Can't reassign a {:?}", variable));
        		}
        	}
        }
        Statement::If(condition, statement_body, index) => {
        	variable_table.symbole_table.push(HashMap::new());
        	match condition{
        		Expr::Variable(name) => {
            		let var_offset = get_scope_variable(variable_table, name).unwrap().offset;
					asm_struct.text[asm_struct.function_index].body.push(format!("    cmp BYTE PTR [rbp {}], 0\n", var_offset));
        		}
        		Expr::IntLiteral(integer) => {
					asm_struct.text[asm_struct.function_index].body.push(format!("    mov eax, {}\n", integer));
					asm_struct.text[asm_struct.function_index].body.push(format!("    cmp eax, 0\n"));
        		}
        		Expr::Binary(_, _, _)=>{
            		gen_expr(condition, variable_table, asm_struct, context);
        		}
        		_ => {
        			compiler_panic("Invalid condition in if statement");
        		}
        	}
			asm_struct.text[asm_struct.function_index].body.push(format!("    je .L{}\n", index));
			for if_body_statement in statement_body{
				println!("statement : {:?}\n", if_body_statement);
				gen_statement(if_body_statement, variable_table, asm_struct, context);
			}
			asm_struct.text[asm_struct.function_index].body.push(format!(".L{}:\n", index));
			variable_table.symbole_table.pop();
        }
        Statement::Else(body, index)=>{
        	let mut asm_body = &mut asm_struct.text[asm_struct.function_index].body;
			asm_body.insert(asm_body.len()-1, format!("    jmp .L{}\n", index));
        	variable_table.symbole_table.push(HashMap::new());
        	for else_body_statement in body{
				println!("statement : {:?}\n", else_body_statement);
				gen_statement(else_body_statement, variable_table, asm_struct, context);
			}
			variable_table.symbole_table.pop();
			asm_struct.text[asm_struct.function_index].body.push(format!(".L{}:\n", index));
        }
        Statement::While(condition, statement_body, index)=>{
			unsafe{
				LOOP_CHECK.push(true);
			}
        	variable_table.symbole_table.push(HashMap::new());
        	let first_index = index;
			asm_struct.text[asm_struct.function_index].body.push(format!("    jmp .L{}\n", first_index));
			unsafe{
				COND_CHECK_LOCS.push(*first_index);
			}
        	let second_index = update_label_index();
			asm_struct.text[asm_struct.function_index].body.push(format!(".L{}:\n", second_index));
        	for while_body_statement in statement_body{
				gen_statement(while_body_statement, variable_table, asm_struct, context);
			}
			asm_struct.text[asm_struct.function_index].body.push(format!(".L{}:\n", first_index));
            gen_expr(condition, variable_table, asm_struct, context);
			asm_struct.text[asm_struct.function_index].body.push(format!("    jne .L{}\n", second_index));
			let break_idx = pop_last_break_idx();
			println!("break idx : {:?}",break_idx);
			if break_idx != None{
				asm_struct.text[asm_struct.function_index].body.push(format!(".L{} :\n", break_idx.unwrap()));
			}
			unsafe{
				LOOP_CHECK.pop();
				COND_CHECK_LOCS.pop();
			}
			variable_table.symbole_table.pop();
        }
        Statement::Struct(name, fields)=>{
        	let mut fields_offet = Vec::new();
        	for field in fields{
        		match field{
        			Statement::Declaration(var_type, _, _)=>{
			        	variable_table.stack_offset -= get_offset(&var_type);
			        	fields_offet.push((field.clone(), variable_table.stack_offset));
        			}
        			_=>{
        				compiler_panic("Expected field declaration in struct");
        			}
        		}
        	}
        	// println!("struct declaration, do nothing for now");
        	let symbole_table_index = variable_table.symbole_table.len()-1;
			variable_table.symbole_table[symbole_table_index].insert(name.clone(), VarInfo{var_type: Type::Struct(name.clone()), offset: variable_table.stack_offset, struct_fields: fields_offet.clone()});
        }
        Statement::BlockScope(statements)=>{
        	variable_table.symbole_table.push(HashMap::new());
        	for statement in statements{
        		gen_statement(statement, variable_table, asm_struct, context);
        	}
			variable_table.symbole_table.pop();
        }
        Statement::Break(idx)=>{
        	unsafe {
        		if LOOP_CHECK.len() == 0{
        			compiler_panic("\"break\" shoud be called inside a loop")
        		}
        	}
        	append_break_idx(*idx);
			asm_struct.text[asm_struct.function_index].body.push(format!("    jmp .L{}\n", idx));
        }
        Statement::Continue=>{
        	unsafe{
        		let cond_loc = COND_CHECK_LOCS.pop();
        		if cond_loc != None{
					asm_struct.text[asm_struct.function_index].body.push(format!("    jmp .L{}\n", cond_loc.unwrap()));
				}
				else{compiler_panic("\"continue\" shoud be called inside a loop");}
        	}
        }
        Statement::Unknown=>{
        	// don't throw an error, but don't do anything, is met when statement is just "var;"
        }
        _ => {
        	compiler_panic(&format!("{:?} Statement Not Yet Implemented", statement));
        }
    }
}

fn gen_function(function : &Function, asm_struct: &mut AssemblyStructure){
	let mut variable_table = VarTable{
		symbole_table: Vec::new(),
		stack_offset: 0,
	};
	variable_table.symbole_table.push(HashMap::new());

	asm_struct.text.push(AssemblyStructureFunction{header: String::new(), body: Vec::new()});

	asm_struct.text[asm_struct.function_index].header.push_str("\n.global ");
	asm_struct.text[asm_struct.function_index].header.push_str(&function.name);
	asm_struct.text[asm_struct.function_index].header.push_str("\n");

	asm_struct.text[asm_struct.function_index].header.push_str(&function.name);
    asm_struct.text[asm_struct.function_index].header.push_str(":\n");
	asm_struct.text[asm_struct.function_index].header.push_str("    push rbp\n");
	asm_struct.text[asm_struct.function_index].header.push_str("    mov rbp, rsp\n");

	if function.args.len() > 0{
		let regs64 = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
		let regs32 = vec!["edi", "esi", "edx", "ecx", "r8d", "r9d"];
		let regs8 = vec!["dil", "sil", "dl", "cl", "r8b", "r9b"];
		for i in 0..function.args.len(){
            variable_table.stack_offset -= get_offset(&function.args[i].var_type);
            let symbole_table_index = variable_table.symbole_table.len()-1;
		    variable_table.symbole_table[symbole_table_index].insert(function.args[i].name.clone(), VarInfo{var_type: function.args[i].var_type.clone(), offset: variable_table.stack_offset, struct_fields: Vec::new()});
		    match function.args[i].var_type{
		    	Type::Char => {
					asm_struct.text[asm_struct.function_index].body.push(format!("    mov BYTE PTR [rbp {}], {}\n", variable_table.stack_offset, regs8[i]));
		    	}
		    	Type::Int => {
					asm_struct.text[asm_struct.function_index].body.push(format!("    mov [rbp {}], {}\n", variable_table.stack_offset, regs32[i]));
		    	}
		    	Type::Pointer(_) => {
					asm_struct.text[asm_struct.function_index].body.push(format!("    mov QWORD PTR [rbp {}], {}\n", variable_table.stack_offset, regs64[i]));
		    	}
		    	_ => {
		    		compiler_panic("Type In Function Args Not Yet Implemented");
		    	}
		    }
		}
	}
	let mut context = GenContext{pointer_in_binary:false, in_a_binary : false, call_context: String::new(), return_context: Box::new(0)};
    for statement in &function.body{
		gen_statement(&statement, &mut variable_table, asm_struct, &mut context);
    }
    println!("in function {}, stack pointer is {}", function.name, variable_table.stack_offset);
	asm_struct.text[asm_struct.function_index].header.push_str(&format!("    sub rsp, {}\n\n", (((variable_table.stack_offset*(-1))+16-1) & !(16-1)))); // alignes to 16 bytes
	asm_struct.function_index += 1;
}

fn gen_program(program_ast: &Program) -> String{
	let mut asm_struct = AssemblyStructure{data: String::new(), text: Vec::new(), function_index: 0};
	asm_struct.data.push_str(".intel_syntax noprefix\n\n");
	for function in &program_ast.functions {
        gen_function(&function, &mut asm_struct);
    }
    let mut final_assembly = String::new();
    final_assembly.push_str(&asm_struct.data);
    final_assembly.push_str("\n");
    for function in asm_struct.text{
    	final_assembly.push_str(&function.header);
    	for statement in &function.body{
    		final_assembly.push_str(statement);
    	}
    	final_assembly.push_str("\n");
    }
    final_assembly
}

fn is_eof(token: &Vec<TOKEN>) -> bool{
	unsafe{
		TOKEN_INDEX as usize >= token.len()
	}
}

fn parse_preprocessor(token: &Vec<TOKEN>){
	if get_token(&token).name == "define"{
		parsing_error("Not Yet Supported", token);
	}
	else if get_token(&token).name == "include"{
		next_token();
		if get_token(&token).token_type == "DOUBLEQUOTE"{
			parsing_error("Not Yet Supported", token);
		}
		else{
			let mut filename = String::new();
			expect(token, &TOKEN{name: "<".to_string(), token_type: "LCHEV".to_string(), line: 0, column: 0});
			let include = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string(), line: 0, column: 0});
			filename.push_str(&include.name);
			if get_token(&token).token_type == "DECIMAL"{
				filename.push_str(".");
				next_token();
			}
			if get_token(&token).token_type == "NAME"{
				filename.push_str(&get_token(&token).name);
				next_token();
			}
			expect(token, &TOKEN{name: ">".to_string(), token_type: "RCHEV".to_string(), line: 0, column: 0});
		}
	}
}

fn parse_file(token : &Vec<TOKEN>) -> Program{
	let mut function_vec = Vec::new();
	if get_token(&token).token_type == "PREPROCESSORDECL"{
		next_token();
		parse_preprocessor(&token);
	}
	while !is_eof(&token){
		function_vec.push(parse_function(&token));
	}
	Program{
		functions: function_vec,
	}
}

fn main(){
	let args: Vec<String> = env::args().collect();
	if args.len() != 3{
		compiler_panic(&format!("Expected 2 arguments : input file and output file, got {} instead", args.len()-1));
	}
	let input_file = &args[1];
	let output_file = &args[2];
	let input_file: &str = input_file;
	let raw_data = fs::read_to_string(input_file).expect("Could not open file");
	let mut data = raw_data.chars().peekable();
	println!("raw data : \n{:?}", raw_data);

	let tokens = ["int", "double", "char", "float", "bool", "return", ";", "{", "}", "(", ")", "[", "]", "+", "-", "*", "/", "=", "!", ",", "\'", "\"", "&", "true", "false", ".", "#", "define", "include", "<", ">", "if", "else", "while", "for", "struct", "break", "continue", "sizeof"];

	let mut found_tokens:Vec<TOKEN> = Vec::new();

	let mut continue_reading = true;
	let mut token = String::new();

	let mut is_in_str_or_char = false;
	let mut line = 1;
	let mut column = 1;
	let mut single_line_comment = false;
	let mut multi_line_comment = false;
	while continue_reading {
		match data.next()
		{
			Some(chara) => {
				if single_line_comment {
					while data.next() != Some('\n'){
						continue;
					}
					line+=1;
					single_line_comment = false;
					continue;
				}
				if multi_line_comment{
					loop {
						let next_data = data.next();
						if next_data == Some('*'){
							if data.peek() == Some(&'/'){
								data.next();
								multi_line_comment = false;
								break;
							}
						}
						if next_data == Some('\n'){
							line+=1;
						}
						// println!("skipping over chara : {:?}", next_data);
					}
					// println!("OUT COMMENT");
					continue;
				}
				if chara == '/'{
					match data.peek(){
						Some('/') => {single_line_comment = true;continue}
						Some('*') => {multi_line_comment = true;continue;}
						_=>{}
					}
				}
				// println!("pushing chara : {}", chara);
				token.push(chara);
				if chara == '\n'{
					line+=1;
					column = 0;
				}else{
					column += 1;
				}
				if chara == '\'' || chara == '\"'{
					is_in_str_or_char = !is_in_str_or_char;
					continue;
				}
				if !is_in_str_or_char{
				let mut trimmed_token = token.trim().to_owned();
					if chara == ' '{
						for defined_token in tokens{
							if defined_token == trimmed_token{
								let tagged_token = tagging(&trimmed_token, line, column);
								println!("found token pass 1 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
								found_tokens.push(tagged_token);
								token.clear();
							}
						}
					}
					else{
						for defined_token in tokens{
							if defined_token == String::from(chara){
								trimmed_token.pop();
								if trimmed_token != ""{
									let first_token = trimmed_token.chars().nth(0).unwrap();
									let last_token = trimmed_token.chars().last().unwrap();
									if (first_token == '\'' || last_token == '\'') || (first_token == '\"' || last_token == '\"'){
										let mut token_type = "UNKNOWN";
										if first_token == '\''{
											token_type = "CHAR";
										}
										else if first_token == '\"'{
											token_type = "STRING";
										}
										trimmed_token.pop();
										trimmed_token.remove(0);
										let tagged_token = tagging(&first_token.to_string(), line, column);
										println!("found token pass 2.1 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
										found_tokens.push(tagged_token);
										let tagged_token = TOKEN{name: trimmed_token.to_string(), token_type: token_type.to_string(), line: 0, column: 0};
										println!("found token pass 2.2 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
										found_tokens.push(tagged_token);
										let tagged_token = tagging(&first_token.to_string(), line, column);
										println!("found token pass 2.3 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
										found_tokens.push(tagged_token);
									}
									else{
										let split_tokens = trimmed_token.trim().split_whitespace();
										// println!("split tokens in pass 2.0{:?}", split_tokens);
										for split_token in split_tokens{
											let tagged_token = tagging(&split_token.to_string(), line, column);
											println!("found token pass 2.0 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
											found_tokens.push(tagged_token);
										}
										// let split_tokens = tagged_token;
									}
								}
								token.clear();
								let tagged_token = tagging(&chara.to_string(), line, column);
								println!("found token pass 3 \"{}\" : \"{}\" (l.{}:{})", tagged_token.token_type, tagged_token.name, line, column);
								found_tokens.push(tagged_token);
							}
						}
					}
				}
			},
			None => {
				continue_reading = false;
			}
		}
	}
	
	println!("{:?}", found_tokens);
	let program_ast = parse_file(&found_tokens);
	println!("sucessful parsing");
	println!("{:?}", program_ast);
	let asm_output = gen_program(&program_ast);
	println!("{}", asm_output);
	std::fs::write(output_file, asm_output).unwrap();
}