# My Compiler

## How it works -

1. Simple [lexer][1] returns next token.
2. Parser uses [precedence climbing][2] to generate an AST, supports left and right associtivity.
3. Evaluator uses simple [postorder traversal][3] of AST to evaluate expression.

## How to play -

1. Install [Rust][4] (if your here I'm gonna assume you can figure this out)
2. git clone https://github.com/mattbettcher/my-compiler.git
3. cargo run -- samples/{name of sample}  (in the cloned directory of course)

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
[4]: https://www.rust-lang.org/