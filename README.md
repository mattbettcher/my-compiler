# My Compiler

## How it works -

1. Simple [lexer][1] returns next token.
2. Parser uses [precedence climbing][2] to generate an AST, supports left and right associtivity.
3. Evaluator uses simple [postorder traversal][3] of AST to evaluate expression.

## TODO -
- [ ] Add more tokens to lexer
- [ ] Implement statements
- [ ] Implement functions
- [ ] So much more!
- [ ] Create a `trait` to interface from AST to backend generation
- [ ] Implement a compiler to WASM

[1]: https://en.wikipedia.org/wiki/Lexical_analysis
[2]: https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing
[3]: https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/