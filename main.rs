use std::fs;
use std::process::exit;
use std::collections::HashMap;

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
}

static mut TOKEN_INDEX: i64 = 0;
static mut STRING_INDEX: i64 = 0;
static mut LABEL_INDEX: i64 = 0;

fn tagging(token: &String) -> TOKEN{
	if is_type(&token){
		return TOKEN{name: token.to_string(), token_type: "TYPE".to_string()};
	}
	else if token == "return"{
		return TOKEN{name: token.to_string(), token_type: "RETURN".to_string()};
	}
	else if token == "("{
		return TOKEN{name: token.to_string(), token_type: "LPAREN".to_string()};
	}
	else if token == ")"{
		return TOKEN{name: token.to_string(), token_type: "RPAREN".to_string()};
	}
	else if token == "{"{
		return TOKEN{name: token.to_string(), token_type: "LBRACK".to_string()};
	}
	else if token == "}"{
		return TOKEN{name: token.to_string(), token_type: "RBRACK".to_string()};
	}
	else if token == "+"{
		return TOKEN{name: token.to_string(), token_type: "PLUS".to_string()};
	}else if token == "-"{
		return TOKEN{name: token.to_string(), token_type: "MINUS".to_string()};
	}else if token == "*"{
		return TOKEN{name: token.to_string(), token_type: "STAR".to_string()};
	}else if token == "/"{
		return TOKEN{name: token.to_string(), token_type: "SLASH".to_string()};
	}
	else if token == "="{
		return TOKEN{name: token.to_string(), token_type: "ASSIGN".to_string()};
	}
	else if token == "!"{
		return TOKEN{name: token.to_string(), token_type: "NOT".to_string()};
	}
	else if is_numerical(&token){
		return TOKEN{name: token.to_string(), token_type: "NUMBER".to_string()};
	}
	else if token == ";"{
		return TOKEN{name: token.to_string(), token_type: "SEMICOLON".to_string()};
	}
	else if token == ","{
		return TOKEN{name: token.to_string(), token_type: "COMMA".to_string()};
	}
	else if token == "\'"{
		return TOKEN{name: token.to_string(), token_type: "SINGLEQUOTE".to_string()};
	}
	else if token == "\""{
		return TOKEN{name: token.to_string(), token_type: "DOUBLEQUOTE".to_string()};
	}
	else if token == "true" || token == "false" {
		return TOKEN{name: token.to_string(), token_type: "BOOLEAN".to_string()};
	}
	else if token == "."{
		return TOKEN{name: token.to_string(), token_type: "DECIMAL".to_string()};
	}
	else if token == "#"{
		return TOKEN{name: token.to_string(), token_type: "PREPROCESSORDECL".to_string()};
	}
	else if token == "define" || token == "include"{
		return TOKEN{name: token.to_string(), token_type: "PREPROCESSOR".to_string()};
	}
	else if token == "<"{
		return TOKEN{name: token.to_string(), token_type: "LCHEV".to_string()};
	}
	else if token == ">"{
		return TOKEN{name: token.to_string(), token_type: "RCHEV".to_string()};
	}
	else if token == "if"{
		return TOKEN{name: token.to_string(), token_type: "IF".to_string()};
	}
	else if token == "while"{
		return TOKEN{name: token.to_string(), token_type: "WHILE".to_string()};
	}
	else if token == "for"{
		return TOKEN{name: token.to_string(), token_type: "FOR".to_string()};
	}
	else {
		return TOKEN{name: token.to_string(), token_type: "NAME".to_string()};
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
		Type::Unknown => 0,
	}
}

fn error(cause: &str){
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
		error(&format!("Expected {0} : \"{1}\", got {2} : \"{3}\"", expected.token_type, expected.name, current_token.token_type, current_token.name));
	}
	return TOKEN{name: "THIS SHOULD NEVER HAPPEN".to_string(), token_type: "THIS SHOULD NEVER HAPPEN".to_string()};
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

#[derive(Debug, Clone)]
enum Type{
	Int,
	Char,
	Bool,
	Float,
	Double,
	Pointer(Box<Type>),
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

	Unknown,
}

#[derive(Debug)]
enum Statement {
	Declaration(Type, String, Expr),
	Return(Expr),
	Call(String, Vec<Expr>),
	Reassign(String, Expr),
	If(Expr, Vec<Statement>, i64),
	While(Expr, Vec<Statement>, i64),
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

struct VarInfo{
	offset: i64,
	var_type: Type,
}

struct VarTable{
	symbole_table: HashMap<String, VarInfo>,
	stack_offset: i64,
}

struct AssemblyStructureFunction{
	header: String,
	body: String,
}

struct AssemblyStructure{
	data: String,
	text: Vec<AssemblyStructureFunction>,
	function_index: usize,
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

fn parse_call(fn_name: &TOKEN, token: &Vec<TOKEN>) -> (String, Vec<Expr>){
	next_token();
	let mut arguments = Vec::new();
	loop {
		arguments.push(parse_expression(token, 0));
		if get_token(token).token_type == "COMMA"{
			next_token();
		}
		else if get_token(token).token_type == "RPAREN"{
			break;
		}
		else{
			println!("Parsing call to {} with arguments {:?}", fn_name.name, arguments);
			error(&format!("Expected COMMA : \",\" or RPAREN : \")\", got {}", get_token(token).token_type));
		}
	}
	expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string()});
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

fn parse_bool(string: &str) -> bool{
	match string{
		"true"=>true,
		"false"=>false,
		_=>{
			error("Not a boolean");
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
			expect(token, &TOKEN{name: "\'".to_string(), token_type: "SINGLEQUOTE".to_string()});
			return char_literal;
		}
		"DOUBLEQUOTE"=>{
			let string = expect(token, &TOKEN{name: "Any String".to_string(), token_type: "STRING".to_string()});
			let string_litteral = Expr::StringLiteral(string.name, update_string_index());
			expect(token, &TOKEN{name: "\"".to_string(), token_type: "DOUBLEQUOTE".to_string()});
			return string_litteral;
		}
		"BOOLEAN" => {
			return Expr::BoolLiteral(parse_bool(&primary.name));
		}
		"NOT" => {
			error("Not operation is not yet implemented");
			return Expr::Unknown
		}
		_ => {
			error(&format!("Expression incorrect, got {0}.... fuck you", primary.token_type));
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
				else{error("Expected comparison, got assign"); BinaryOp::UNKNOWN}
			},
			"NOT" => {
				if get_token(token).token_type == "ASSIGN"{
					next_token();
					BinaryOp::NEQUAL
				}
				else if get_token(token).token_type == "NAME"{
					error("Not-ing variable not yet implemented");BinaryOp::UNKNOWN
				}
				else{error("Expected comparison, got assign"); BinaryOp::UNKNOWN}
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
		if get_token(&token).token_type == "STAR"{
			var_type = Type::Pointer(Box::new(var_type));
			next_token();
		}
		let var_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string()});
		expect(token, &TOKEN{name: "=".to_string(), token_type: "ASSIGN".to_string()});
		let expr = parse_expression(token, 0);
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string()});
		return Statement::Declaration(var_type, var_name.name, expr);

	}
	else if current_token.token_type == "RETURN"{
		let expression = parse_expression(token, 0);
		println!("Expression : \n{:?}", expression);
		expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string()});
		return Statement::Return(expression);
	}
	else if current_token.token_type == "NAME"{
		if get_token(token).token_type == "LPAREN"{
			let call = parse_call(current_token, token);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string()});
			return Statement::Call(call.0, call.1);
		}
		if get_token(token).token_type == "ASSIGN"{
			next_token();
			let expr = parse_expression(token, 0);
			expect(token, &TOKEN{name: ";".to_string(), token_type: "SEMICOLON".to_string()});
			return Statement::Reassign(current_token.name.clone(), expr)
		}
		error(&format!("Expected LPAREN, got {} : {}", get_token(token).token_type, get_token(token).name));
		return Statement::Unknown;
	}
	else if current_token.token_type == "IF" || current_token.token_type == "WHILE"{
		expect(token, &TOKEN{name: "(".to_string(), token_type: "LPAREN".to_string()});
		let expression = parse_expression(token, 0);
		expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string()});
		expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACK".to_string()});
		let mut body = Vec::new();
		while get_token(token).token_type != "RBRACK"{
			body.push(parse_statement(token));
		}
		println!("body : {:?}", body);
		expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACK".to_string()});
		let idx = update_label_index();
		match current_token.token_type.as_str(){
			"IF"=>{return Statement::If(expression, body, idx);}
			"WHILE"=>{return Statement::While(expression, body, idx);}
			_ => {error("I really should move away from string token type and implement an enum");Statement::Unknown}
		}
	}
	else{
		error(&format!("Invalid token found in statement : {} : {}", current_token.token_type, current_token.name));
		return Statement::Unknown;
	}	
}

fn parse_function(token : &Vec<TOKEN>) -> Function{
	expect(token, &TOKEN{name: "Any Type".to_string(), token_type: "TYPE".to_string()});
	let fn_name = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string()});
	expect(token, &TOKEN{name: "(".to_string(), token_type: "LPAREN".to_string()});
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
				let argument = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string()});
				arguments.push(FunctionArgDeclaration{var_type: var_type, name: argument.name});
				if get_token(&token).token_type == "COMMA"{
					next_token();
				}
				else if get_token(&token).token_type == "RPAREN"{
					break;
				}
				else{
					error(&format!("Expected RPAREN : \")\" or COMMA : \",\", got {}", get_token(&token).token_type));
				} 
			}
			else{
				error(&format!("Expected TYPE, got {}", get_token(&token).token_type));
			}
		}
	}
	expect(token, &TOKEN{name: ")".to_string(), token_type: "RPAREN".to_string()});
	expect(token, &TOKEN{name: "{".to_string(), token_type: "LBRACK".to_string()});
	let mut body = Vec::new();
	while get_token(token).token_type != "RBRACK"{
		body.push(parse_statement(token));
	}
	expect(token, &TOKEN{name: "}".to_string(), token_type: "RBRACK".to_string()});
	return Function{name: fn_name.name, args: arguments, body: body};
}

fn gen_call(name: &String, args: &Vec<Expr>, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure){
	if args.len() > 0{
		let regs = vec!["edi", "esi", "edx", "ecx", "r8d", "r9d"];
		for i in 0..args.len(){
    		gen_expr(&args[i], variable_table, asm_struct);
    		asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov {}, eax\n", regs[i]));
		}
	}
	asm_struct.text[asm_struct.function_index].body.push_str(&format!("    call {}\n", name));
}

fn gen_expr(expr: &Expr, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure) {
    match expr {
        Expr::IntLiteral(n) => asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov eax, {}\n", n)),
        Expr::BoolLiteral(n) => asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov eax, {}\n", *n as u8)),
        Expr::CharLiteral(n) => asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov eax, {}\n", n)),
        Expr::Binary(left, op, right) =>{
        	gen_expr(left, variable_table, asm_struct);
        	asm_struct.text[asm_struct.function_index].body.push_str("    push rax\n");

    		gen_expr(right, variable_table, asm_struct);
        	asm_struct.text[asm_struct.function_index].body.push_str("    pop rcx\n");
        	match op{
        		BinaryOp::PLUS=>{
        			asm_struct.text[asm_struct.function_index].body.push_str("    add eax, ecx\n");
        		}
        		BinaryOp::MINUS=>{
        			asm_struct.text[asm_struct.function_index].body.push_str("    sub ecx, eax\n");
        			asm_struct.text[asm_struct.function_index].body.push_str("    mov eax, ecx\n");
        		}
        		BinaryOp::MULTIPLICATION=>{
        			asm_struct.text[asm_struct.function_index].body.push_str("    imul rax, rcx\n");
        		}
        		BinaryOp::DIVISION=>{
        			asm_struct.text[asm_struct.function_index].body.push_str("    mov r10d, eax\n");
                    asm_struct.text[asm_struct.function_index].body.push_str("    mov eax, ecx\n");
                    asm_struct.text[asm_struct.function_index].body.push_str("    cdq\n");
                    asm_struct.text[asm_struct.function_index].body.push_str("    idiv r10d\n");
        		}
        		BinaryOp::EQUAL | BinaryOp::NEQUAL | BinaryOp::GREATER | BinaryOp::GREATEREQ | BinaryOp::LESSER | BinaryOp::LESSEREQ => {
            		asm_struct.text[asm_struct.function_index].body.push_str(&format!("    cmp ecx, eax\n"));
            		match op {
            			BinaryOp::EQUAL=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    sete al\n"));}
            			BinaryOp::NEQUAL=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    setne al\n"));}
            			BinaryOp::GREATER=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    setg al\n"));}
            			BinaryOp::GREATEREQ=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    setge al\n"));}
            			BinaryOp::LESSER=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    setl al\n"));}
            			BinaryOp::LESSEREQ=>{asm_struct.text[asm_struct.function_index].body.push_str(&format!("    setle al\n"));}
            			_ =>{error("THIS SHOULD NEVER HAPPEN");}
            		}
            		// asm_struct.text[asm_struct.function_index].body.push_str(&format!("    sete al\n"));
            		asm_struct.text[asm_struct.function_index].body.push_str(&format!("    movzx eax, al\n"));
            		asm_struct.text[asm_struct.function_index].body.push_str(&format!("    cmp eax, 0\n"));
        		}
        		BinaryOp::UNKNOWN =>{
        			error("Somehow stumbeled uppon unknown operation, this should never happen");
        		}
        		_ => {
        			error("Binary Operator Not Yet Implemented");
        		}
        	}
        } ,
        Expr::Variable(name) => {
        	if let Some(var_info) = variable_table.symbole_table.get(name){
        		match var_info.var_type{
        			Type::Int => {
        				asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov eax, [rbp {}]\n", var_info.offset));
        			}
        			Type::Char | Type::Bool => {
        				asm_struct.text[asm_struct.function_index].body.push_str(&format!("    movzx eax, BYTE PTR [rbp {}]\n", var_info.offset));
        			}
        			Type::Pointer(_) => {
        				asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov rax, QWORD PTR [rbp {}]\n", var_info.offset));
        			}
        			_ =>{
        				error(&format!("Variable Type {:?} Not Yet Implemented", var_info.var_type));
        			}
        		}
        	}
        	else{
        		error(&format!("Variable \"{}\" used before declaration", name));
        	}
        },
        Expr::Call(name, args) => {
        	gen_call(name, args, variable_table, asm_struct);
        },
        Expr::StringLiteral(string, index) => {
        	asm_struct.data.push_str(&format!(".LC{}:\n", index));
        	asm_struct.data.push_str(&format!("    .string \"{}\"\n", string));
        	asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov rax, OFFSET FLAT: .LC{}\n", index));
        },
        Expr::Unknown => {
        	error("Unknown expression");
        }
        _ => {
        	error("Expr Not yet implemented");
        }
    }
}

fn gen_statement(statement: &Statement, variable_table: &mut VarTable, asm_struct: &mut AssemblyStructure) {
    match statement {
        Statement::Return(expr) => {
            gen_expr(expr, variable_table, asm_struct);
            asm_struct.text[asm_struct.function_index].body.push_str("    mov rsp, rbp\n");
            asm_struct.text[asm_struct.function_index].body.push_str("    pop rbp\n");
            asm_struct.text[asm_struct.function_index].body.push_str("    ret");
        }
        Statement::Declaration(var_type, name, expr) => {
            gen_expr(expr, variable_table, asm_struct);
            variable_table.stack_offset -= get_offset(var_type);
            variable_table.symbole_table.insert(name.clone(), VarInfo{var_type: var_type.clone(), offset: variable_table.stack_offset});
            match var_type{
            	Type::Bool=>{
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov BYTE PTR [rbp {}], al\n", variable_table.stack_offset));
            	}
            	_ =>{
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov [rbp {}], eax\n", variable_table.stack_offset));
            	}
            }
        }
        Statement::Call(name, args)=>{
        	gen_call(name, args, variable_table, asm_struct);
        }
        Statement::Reassign(name, expr)=>{
            gen_expr(expr, variable_table, asm_struct);
            let var_offset = variable_table.symbole_table.get(name).unwrap().offset;
			asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov [rbp {}], eax\n", var_offset));
        }
        Statement::If(condition, statement_body, index) => {
        	match condition{
        		Expr::Variable(name) => {
            		let var_offset = variable_table.symbole_table.get(name).unwrap().offset;
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    cmp BYTE PTR [rbp {}], 0\n", var_offset));
        		}
        		Expr::IntLiteral(integer) => {
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov eax, {}\n", integer));
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    cmp eax, 0\n"));
        		}
        		Expr::Binary(_, op, _)=>{
            		gen_expr(condition, variable_table, asm_struct);
        		}
        		_ => {
        			error("Invalid condition in if statement");
        		}
        	}
			asm_struct.text[asm_struct.function_index].body.push_str(&format!("    je .L{}\n", index));
			for if_body_statement in statement_body{
				println!("statement : {:?}\n", if_body_statement);
				gen_statement(if_body_statement, variable_table, asm_struct);
			}
			asm_struct.text[asm_struct.function_index].body.push_str(&format!(".L{}:\n", index));
        }
        Statement::While(condition, statement_body, index)=>{
        	let first_index = index;
			asm_struct.text[asm_struct.function_index].body.push_str(&format!("    jmp .L{}\n", first_index));
        	let second_index = update_label_index();
			asm_struct.text[asm_struct.function_index].body.push_str(&format!(".L{}:\n", second_index));
        	for if_body_statement in statement_body{
				println!("statement : {:?}\n", if_body_statement);
				gen_statement(if_body_statement, variable_table, asm_struct);
			}
			asm_struct.text[asm_struct.function_index].body.push_str(&format!(".L{}:\n", first_index));
            gen_expr(condition, variable_table, asm_struct);
			asm_struct.text[asm_struct.function_index].body.push_str(&format!("    jne .L{}\n", second_index));
        }
        _ => {
        	error("Statement Not Yet Implemented");
        }
    }
}

fn gen_function(function : &Function, asm_struct: &mut AssemblyStructure){
	let mut variable_table = VarTable{
		symbole_table: HashMap::new(),
		stack_offset: 0,
	};
	asm_struct.text.push(AssemblyStructureFunction{header: String::new(), body: String::new()});

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
		    variable_table.symbole_table.insert(function.args[i].name.clone(), VarInfo{var_type: function.args[i].var_type.clone(), offset: variable_table.stack_offset});
		    match function.args[i].var_type{
		    	Type::Char => {
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov BYTE PTR [rbp {}], {}\n", variable_table.stack_offset, regs8[i]));
		    	}
		    	Type::Int => {
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov [rbp {}], {}\n", variable_table.stack_offset, regs32[i]));
		    	}
		    	Type::Pointer(_) => {
					asm_struct.text[asm_struct.function_index].body.push_str(&format!("    mov QWORD PTR [rbp {}], {}\n", variable_table.stack_offset, regs64[i]));
		    	}
		    	_ => {
		    		error("Type In Function Args Not Yet Implemented");
		    	}
		    }
		}
	}
    for statement in &function.body{
		gen_statement(&statement, &mut variable_table, asm_struct);
    }
	asm_struct.text[asm_struct.function_index].header.push_str(&format!("    sub rsp, {}\n\n", variable_table.stack_offset*(-1)));
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
    	final_assembly.push_str(&function.body);
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
		error("Not Yet Supported");
	}
	else if get_token(&token).name == "include"{
		next_token();
		if get_token(&token).token_type == "DOUBLEQUOTE"{
			error("Not Yet Supported");
		}
		else{
			let mut filename = String::new();
			expect(token, &TOKEN{name: "<".to_string(), token_type: "LCHEV".to_string()});
			let include = expect(token, &TOKEN{name: "Any Name".to_string(), token_type: "NAME".to_string()});
			filename.push_str(&include.name);
			if get_token(&token).token_type == "DECIMAL"{
				filename.push_str(".");
				next_token();
			}
			if get_token(&token).token_type == "NAME"{
				filename.push_str(&get_token(&token).name);
				next_token();
			}
			expect(token, &TOKEN{name: ">".to_string(), token_type: "RCHEV".to_string()});
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
	const FILE: &str = "main.c";
	let mut raw_data = fs::read_to_string(FILE).expect("Could not open file");
	let mut i = 0;
	loop{
		match raw_data.chars().nth(i){
			Some(comment_symb_one) =>{
				if comment_symb_one == '/'{
					match raw_data.chars().nth(i+1){
						Some(comment_symb_two) => {
							if comment_symb_two == '/' {
								while raw_data.chars().nth(i) != Some('\n') {
									match raw_data.chars().nth(i){
										Some(_)=>{
											raw_data.remove(i);
										}
										None =>{
											break;
										}
									}
								}
							}
							else if comment_symb_two == '*' {
								raw_data.remove(i);
								raw_data.remove(i);
								while raw_data.chars().nth(i) != Some('*') || raw_data.chars().nth(i+1) != Some('/') {
									match raw_data.chars().nth(i){
										Some(_)=>{
											raw_data.remove(i);
										}
										None =>{
											error("Unclosed multiline comment");
										}
									}
								}
								raw_data.remove(i);
								raw_data.remove(i);

							}
						}
						None => {break;}
					}
				}
				i+=1;
			}
			None => {break;}
		}
	}
	let mut data = raw_data.chars();

	let tokens = ["int", "double", "char", "float", "bool", "return", ";", "{", "}", "(", ")", "+", "-", "*", "/", "=", "!", ",", "\'", "\"", "true", "false", ".", "#", "define", "include", "<", ">", "if", "while", "for"];

	let mut found_tokens:Vec<TOKEN> = Vec::new();

	let mut continue_reading = true;
	let mut token = String::new();

	let mut is_in_str_or_char = false;
	while continue_reading {
		match data.nth(0)
		{
			Some(chara) => {
				token.push(chara);
				if chara == '\'' || chara == '\"'{
					is_in_str_or_char = !is_in_str_or_char;
					continue;
				}
				if !is_in_str_or_char{
				let mut trimmed_token = token.trim().to_owned();
					if chara == ' '{
						for defined_token in tokens{
							if defined_token == trimmed_token{
								let tagged_token = tagging(&trimmed_token);
								println!("found token pass 1 \"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
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
										let tagged_token = tagging(&first_token.to_string());
										println!("found token pass 2.1\"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
										found_tokens.push(tagged_token);
										let tagged_token = TOKEN{name: trimmed_token.to_string(), token_type: token_type.to_string()};
										println!("found token pass 2.2\"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
										found_tokens.push(tagged_token);
										let tagged_token = tagging(&first_token.to_string());
										println!("found token pass 2.3\"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
										found_tokens.push(tagged_token);
									}
									else{
										let tagged_token = tagging(&trimmed_token.trim().to_string());
										println!("found token pass 2.0\"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
										found_tokens.push(tagged_token);
									}
								}
								token.clear();
								let tagged_token = tagging(&chara.to_string());
								println!("found token pass 3\"{}\" : \"{}\"", tagged_token.token_type, tagged_token.name);
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
	std::fs::write("out.s", asm_output).unwrap();
}