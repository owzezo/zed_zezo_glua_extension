; Keywords
"return" @keyword.return
[ "goto" "in" "local" ] @keyword
(break_statement) @keyword
(do_statement [ "do" "end" ] @keyword)
(while_statement [ "while" "do" "end" ] @keyword)
(repeat_statement [ "repeat" "until" ] @keyword)
(if_statement [ "if" "elseif" "else" "then" "end" ] @keyword)
(elseif_statement [ "elseif" "then" "end" ] @keyword)
(else_statement [ "else" "end" ] @keyword)
(for_statement [ "for" "do" "end" ] @keyword)
(function_declaration [ "function" "end" ] @keyword)
(function_definition [ "function" "end" ] @keyword)

; Operators
[ "and" "not" "or" ] @operator
[
  "+" "-" "*" "/" "%" "^" "#"
  "==" "~=" "<=" ">=" "<" ">" "="
  "&" "~" "|" "<<" ">>" "//" ".."
] @operator

; Punctuation
[ ";" ":" "::" "," "." ] @punctuation.delimiter
[ "(" ")" "[" "]" "{" "}" ] @punctuation.bracket

; Constants
(nil) @constant.builtin
[ (false) (true) ] @boolean
(vararg_expression) @variable.builtin

; Literals
(number) @number
(string) @string
(escape_sequence) @string.escape
(comment) @comment
(hash_bang_line) @comment

; Function calls
(function_call name: (identifier) @function)
(function_call name: (dot_index_expression field: (identifier) @function))
(function_call (method_index_expression method: (identifier) @function.method))

; Function definitions
(function_declaration name: (identifier) @function)
(function_declaration name: (dot_index_expression field: (identifier) @function))
(function_declaration name: (method_index_expression method: (identifier) @function.method))
(function_definition) @function

; Parameters and fields
(parameters (identifier) @variable.parameter)
(dot_index_expression field: (identifier) @property)
(field name: (identifier) @property)
(label_statement (identifier) @label)

; Variables (fallback)
(identifier) @variable
