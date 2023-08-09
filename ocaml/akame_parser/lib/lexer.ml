type token =
  | FN
  | IDENT of string
  | LBRACE
  | RBRACE
  | LPAREN
  | RPAREN
  | RETURN
  | LET
  | EQUAL
  | PLUS
  | SEMICOLON
  | STRING of string
  | INT of int
  | ILLEGAL of string

let tokenize input =
  let current = ref 0 in
  let length = String.length input in
  let tokens = ref [] in

  let is_alpha c = (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' in
  let is_digit c = c >= '0' && c <= '9' in

  let read_ident() =
    let start = !current in
    while !current < length && is_alpha input.[!current] do
      current := !current + 1;
    done;
    String.sub input start (!current - start)
  in

  let read_string() =
    let start = !current in
    current := !current + 1;
    while !current < length && input.[!current] <> '"' do
      current := !current + 1;
    done;
    if !current < length && input.[!current] = '"' then
      current := !current + 1;
    String.sub input (start + 1) (!current - start - 2)
  in

  let read_number() =
    let start = !current in
    while !current < length && is_digit input.[!current] do
      current := !current + 1;
    done;
    int_of_string (String.sub input start (!current - start))
  in

  while !current < length do
    match input.[!current] with
    | ' ' | '\t' | '\n' | '\r' -> current := !current + 1
    | 'f' when String.sub input !current 2 = "fn" ->
      tokens := FN :: !tokens;
      current := !current + 2
    | 'l' when String.sub input !current 3 = "let" ->
      tokens := LET :: !tokens;
      current := !current + 3
    | 'r' when String.sub input !current 6 = "return" ->
      tokens := RETURN :: !tokens;
      current := !current + 6
    | '{' ->
      tokens := LBRACE :: !tokens;
      current := !current + 1
    | '}' ->
      tokens := RBRACE :: !tokens;
      current := !current + 1
    | '(' ->
      tokens := LPAREN :: !tokens;
      current := !current + 1
    | ')' ->
      tokens := RPAREN :: !tokens;
      current := !current + 1
    | '=' ->
      tokens := EQUAL :: !tokens;
      current := !current + 1
    | '+' ->
      tokens := PLUS :: !tokens;
      current := !current + 1
    | ';' ->
      tokens := SEMICOLON :: !tokens;
      current := !current + 1
    | '"' ->
      tokens := STRING (read_string()) :: !tokens
    | c when is_digit c ->
      tokens := INT (read_number()) :: !tokens
    | c when is_alpha c ->
      tokens := IDENT (read_ident()) :: !tokens
    | c ->
      tokens := ILLEGAL (String.make 1 c) :: !tokens;
      current := !current + 1
  done;
  List.rev !tokens
