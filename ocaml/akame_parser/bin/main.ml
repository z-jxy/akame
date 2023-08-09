open Akame_parser
(*
 * This is a simple REPL for the language. It will read in a line of input,
 * tokenize it, parse it, and then print the AST.
 *)

 let token_to_string = function
  | Lexer.FN -> "FN"
  | Lexer.IDENT s -> Printf.sprintf "IDENT(%s)" s
  | Lexer.STRING s -> Printf.sprintf "STRING(%s)" s
  | Lexer.INT i -> Printf.sprintf "INT(%d)" i
  | Lexer.ILLEGAL s -> Printf.sprintf "ILLEGAL(%s)" s
  | Lexer.LBRACE -> "LBRACE"
  | Lexer.RBRACE -> "RBRACE"
  | Lexer.LPAREN -> "LPAREN"
  | Lexer.RPAREN -> "RPAREN"
  | Lexer.RETURN -> "RETURN"
  | Lexer.LET -> "LET"
  | Lexer.EQUAL -> "EQUAL"
  | Lexer.PLUS -> "PLUS"
  | Lexer.SEMICOLON -> "SEMICOLON"
;;

let rec pretty_print_ast ast =
  match ast with
  | Parser.Function(name, arg, body) ->
      Printf.printf "Function(%s, %s, [" name arg;
      List.iter pretty_print_ast body;
      print_endline "])";
  | Parser.Return value -> Printf.printf "Return(%s)\n" value;
  | Parser.Let(name, value) -> Printf.printf "Let(%s, %s)\n" name value;
  | Parser.Call(fname, arg) -> Printf.printf "Call(%s, %s)\n" fname arg;
  | Parser.Statement token -> Printf.printf "Statement(%s)\n" (token_to_string token)
;;


let test_input = "fn hello(num) {let num = 4;}; fn main() {let numz = 4;};" ;;
let tokens = Lexer.tokenize test_input;;

List.iter (fun token -> 
  print_endline (token_to_string token)
) tokens

let ast = Parser.parse tokens;;
List.iter pretty_print_ast ast;
