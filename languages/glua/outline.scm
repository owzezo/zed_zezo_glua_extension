(function_declaration
  "function" @context
  name: [
    (identifier) @name
    (dot_index_expression) @name
    (method_index_expression) @name
  ]) @item

(assignment_statement
  (variable_list name: (_) @name)
  (expression_list value: (function_definition)) @item)
