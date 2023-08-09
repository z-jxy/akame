open Lexer
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

type ast =
  | Function of string * string * ast list
  | Return of string
  | Let of string * string
  | Call of string * string
  | Statement of token
;;

let parse tokens =
  let error_msg expected token next = 
    Printf.sprintf "Expected %s after %s but received %s" expected (token_to_string token) (token_to_string next)
  in

  let rec consume tokens ast =
    match tokens with
    | [] -> List.rev ast
    | FN :: IDENT fname :: LPAREN :: IDENT arg :: RPAREN :: LBRACE :: rest ->
      let body, remaining = consume_block rest [] in
      consume remaining (Function (fname, arg, body) :: ast)
    | FN :: IDENT fname :: LPAREN :: RPAREN :: LBRACE :: rest ->
        let body, remaining = consume_block rest [] in
        consume remaining (Function (fname, "", body) :: ast)
    | RETURN :: IDENT value :: SEMICOLON :: rest -> consume rest (Return value :: ast)
    | LET :: IDENT name :: EQUAL :: IDENT value :: SEMICOLON :: rest -> consume rest (Let (name, value) :: ast)
    | IDENT fname :: LPAREN :: IDENT arg :: RPAREN :: SEMICOLON :: rest -> consume rest (Call (fname, arg) :: ast)
    | token :: rest -> consume rest (Statement token :: ast)
  and consume_block tokens block =
    match tokens with
    | RBRACE :: rest -> (List.rev block, rest)
    | _ ->
      let stmt, remaining = consume_statement tokens in
      consume_block remaining (stmt :: block)
  and consume_statement tokens =
    match tokens with
    | RETURN :: IDENT value :: SEMICOLON :: rest -> (Return value, rest)
    | LET :: IDENT name :: EQUAL :: INT value :: SEMICOLON :: rest -> (Let (name, string_of_int value), rest)
    | IDENT fname :: LPAREN :: IDENT arg :: RPAREN :: SEMICOLON :: rest -> (Call (fname, arg), rest)
    | token :: next :: _ -> 
      failwith (error_msg "a valid statement" token next)
    | _ -> failwith "Unexpected end of input"
  in
  consume tokens []
