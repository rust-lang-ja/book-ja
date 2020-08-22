<!--
## Appendix A: Keywords
-->

## 付録A: キーワード

<!--
The following list contains keywords that are reserved for current or future
use by the Rust language. As such, they cannot be used as identifiers, such as
names of functions, variables, parameters, struct fields, modules, crates,
constants, macros, static values, attributes, types, traits, or lifetimes.
-->

以下のリストは、現在、あるいは将来Rust言語により使用されるために予約されているキーワードを含んでいます。
そのため、識別子として使用することはできません。識別子の例は、関数名、変数名、引数名、構造体のフィールド名、
モジュール名、クレート名、定数名、マクロ名、静的な値の名前、属性名、型名、トレイト名、ライフタイム名です。

<!--
### Keywords Currently in Use
-->

### 現在使用されているキーワード

<!--
The following keywords currently have the functionality described.
-->

以下のキーワードは、解説された通りの機能が現状あります。

<!--
* `as` - perform primitive casting, disambiguate the specific trait containing
an item, or rename items in `use` and `extern crate` statements
* `break` - exit a loop immediately
* `const` - define constant items or constant raw pointers
* `continue` - continue to the next loop iteration
* `crate` - link an external crate or a macro variable representing the crate in
which the macro is defined
* `else` - fallback for `if` and `if let` control flow constructs
* `enum` - define an enumeration
* `extern` - link an external crate, function, or variable
* `false` - Boolean false literal
* `fn` - define a function or the function pointer type
* `for` - loop over items from an iterator, implement a trait, or specify a
higher-ranked lifetime
* `if` - branch based on the result of a conditional expression
* `impl` - implement inherent or trait functionality
* `in` - part of `for` loop syntax
* `let` - bind a variable
* `loop` - loop unconditionally
* `match` - match a value to patterns
* `mod` - define a module
* `move` - make a closure take ownership of all its captures
* `mut` - denote mutability in references, raw pointers, or pattern bindings
* `pub` - denote public visibility in struct fields, `impl` blocks, or modules
* `ref` - bind by reference
* `return` - return from function
* `Self` - a type alias for the type implementing a trait
* `self` - method subject or current module
* `static` - global variable or lifetime lasting the entire program execution
* `struct` - define a structure
* `super` - parent module of the current module
* `trait` - define a trait
* `true` - Boolean true literal
* `type` - define a type alias or associated type
* `unsafe` - denote unsafe code, functions, traits, or implementations
* `use` - import symbols into scope
* `where` - denote clauses that constrain a type
* `while` - loop conditionally based on the result of an expression
-->

<!--
higher-ranked lifetimeについては議論の余地ありか
-->

* `as` - 基礎的なキャストの実行、要素を含む特定のトレイトの明確化、`use`や`extern crate`文の要素名を変更する
* `break` - 即座にループを抜ける
* `const` - 定数要素か定数の生ポインタを定義する
* `continue` - 次のループの繰り返しに継続する
* `crate` - 外部のクレートかマクロが定義されているクレートを表すマクロ変数をリンクする
* `else` - `if`と`if let`フロー制御構文の規定
* `enum` - 列挙型を定義する
* `extern` - 外部のクレート、関数、変数をリンクする
* `false` - bool型のfalseリテラル
* `fn` - 関数か関数ポインタ型を定義する
* `for` - イテレータの要素を繰り返す、トレイトの実装、高階ライフタイムの指定
* `if` - 条件式の結果によって条件分岐
* `impl` - 固有の機能やトレイトの機能を実装する
* `in` - `for`ループ記法の一部
* `let` - 変数を束縛する
* `loop` - 無条件にループする
* `match` - 値をパターンとマッチさせる
* `mod` - モジュールを定義する
* `move` - クロージャにキャプチャした変数全ての所有権を奪わせる
* `mut` - 参照、生ポインタ、パターン束縛で可変性に言及する
* `pub` - 構造体フィールド、`impl`ブロック、モジュールで公開性について言及する
* `ref` - 参照で束縛する
* `return` - 関数から帰る
* `Self` - トレイトを実装する型の型エイリアス
* `self` - メソッドの主題、または現在のモジュール
* `static` - グローバル変数、またはプログラム全体に渡るライフタイム
* `struct` - 構造体を定義する
* `super` - 現在のモジュールの親モジュール
* `trait` - トレイトを定義する
* `true` - bool型のtrueリテラル
* `type` - 型エイリアスか関連型を定義する
* `unsafe` - unsafeなコード、関数、トレイト、実装に言及する
* `use` - スコープにシンボルをインポートする
* `where` - 型を制限する節に言及する
* `while` - 式の結果に基づいて条件的にループする

<!--
### Keywords Reserved for Future Use
-->

### 将来的な使用のために予約されているキーワード

<!--
The following keywords do not have any functionality but are reserved by Rust
for potential future use.
-->

以下のキーワードには機能が何もないものの、将来的に使用される可能性があるので、Rustにより予約されています。

<!--
* `abstract`
* `alignof`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `offsetof`
* `override`
* `priv`
* `proc`
* `pure`
* `sizeof`
* `typeof`
* `unsized`
* `virtual`
* `yield`
-->

* `abstract`
* `alignof`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `offsetof`
* `override`
* `priv`
* `proc`
* `pure`
* `sizeof`
* `typeof`
* `unsized`
* `virtual`
* `yield`
