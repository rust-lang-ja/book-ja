<!--
## Appendix A: Keywords
-->

## 付録 A: キーワード

<!--
The following list contains keywords that are reserved for current or future
use by the Rust language. As such, they cannot be used as identifiers (except
as raw identifiers as we’ll discuss in the “[Raw
Identifiers][raw-identifiers]” section), including names of
functions, variables, parameters, struct fields, modules, crates, constants,
macros, static values, attributes, types, traits, or lifetimes.
-->

以下のリストは、現在、あるいは将来 Rust 言語により使用されるために予約されているキーワードです。
そのため、識別子として使用することはできません。識別子には、関数名、変数名、引数名、構造体のフィールド名、モジュール名、クレート名、定数名、マクロ名、静的な値の名前、属性名、型名、トレイト名、ライフタイム名などがあります。
ただし、[生識別子][raw-identifiers]のところで議論する生識別子は例外です。

[raw-identifiers]: #raw-identifiers

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
* `async` -  return a `Future` instead of blocking the current thread
* `await` - suspend execution until the result of a `Future` is ready
* `break` - exit a loop immediately
* `const` - define constant items or constant raw pointers
* `continue` - continue to the next loop iteration
* `crate` - link an external crate or a macro variable representing the crate in
  which the macro is defined
* `dyn` - dynamic dispatch to a trait object
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
* `Self` - a type alias for the type we are defining or implementing
* `self` - method subject or current module
* `static` - global variable or lifetime lasting the entire program execution
* `struct` - define a structure
* `super` - parent module of the current module
* `trait` - define a trait
* `true` - Boolean true literal
* `type` - define a type alias or associated type
* `union` - define a [union] and is only a keyword when used in a union declaration
* `unsafe` - denote unsafe code, functions, traits, or implementations
* `use` - bring symbols into scope
* `where` - denote clauses that constrain a type
* `while` - loop conditionally based on the result of an expression

[union]: ../reference/items/unions.html
-->

<!--
higher-ranked lifetime については議論の余地ありか
-->

* `as` - 基礎的なキャストの実行、要素を含む特定のトレイトの明確化、`use`や`extern crate`文の要素名を変更する
* `async` - 現在のスレッドをブロックする代わりに`Future`を返す
* `await` - `Future`の結果が準備できるまで実行を停止する
* `break` - 即座にループを抜ける
* `const` - 定数要素か定数の生ポインタを定義する
* `continue` - 次のループの繰り返しに継続する
* `crate` - 外部のクレートかマクロが定義されているクレートを表すマクロ変数をリンクする
* `else` - `if`と`if let`制御フロー構文の規定
* `enum` - 列挙型を定義する
* `extern` - 外部のクレート、関数、変数をリンクする
* `false` - bool 型の false リテラル
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
* `Self` - 定義しようとしている・実装 (implement) しようとしている型の型エイリアス
* `self` - メソッドの主題、または現在のモジュール
* `static` - グローバル変数、またはプログラム全体に渡るライフタイム
* `struct` - 構造体を定義する
* `super` - 現在のモジュールの親モジュール
* `trait` - トレイトを定義する
* `true` - bool 型の true リテラル
* `type` - 型エイリアスか関連型を定義する
* `unsafe` - unsafe なコード、関数、トレイト、実装に言及する
* `use` - スコープにシンボルを持ち込む
* `where` - 型を制限する節に言及する
* `while` - 式の結果に基づいて条件的にループする

[union]: https://doc.rust-lang.org/reference/items/unions.html

<!--
### Keywords Reserved for Future Use
-->

### 将来的な使用のために予約されているキーワード

<!--
The following keywords do not have any functionality but are reserved by Rust
for potential future use.
-->

以下のキーワードには機能が何もないものの、将来的に使用される可能性があるので、Rust により予約されています。

<!--
* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`
-->

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

<!--
### Raw Identifiers
-->
### 生識別子

<!--
*Raw identifiers* are the syntax that lets you use keywords where they wouldn’t
normally be allowed. You use a raw identifier by prefixing a keyword with `r#`.
-->
*生識別子* とは、普段は使うことが許されないキーワードを使わせてくれる構文です。
生識別子はキーワードの前に`r#`を置いて使うことができます。

<!--
For example, `match` is a keyword. If you try to compile the following function
that uses `match` as its name:
-->
たとえば、`match`はキーワードです。
次の、名前が`match`である関数をコンパイルしようとすると：

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
// 訳注：引数名は、"a needle in a haystack" すなわち「干し草の中の針」という、
// 「見つかりそうにない捜し物」を意味する成句からもじった命名。
// 検索をする関数でよく使われる。
```

<!--
you’ll get this error:
-->
次のエラーを得ます：

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

<!--
The error shows that you can’t use the keyword `match` as the function
identifier. To use `match` as a function name, you need to use the raw
identifier syntax, like this:
-->
このエラーは`match`というキーワードを関数の識別子としては使えないと示しています。
`match`を関数名として使うには、次のように、生識別子構文を使う必要があります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

<!--
This code will compile without any errors. Note the `r#` prefix on the function
name in its definition as well as where the function is called in `main`.
-->
このコードはなんのエラーもなくコンパイルできます。
`r#`は、定義のときも、`main`内で呼ばれたときにも、関数名の前につけられていることに注意してください。

<!--
Raw identifiers allow you to use any word you choose as an identifier, even if
that word happens to be a reserved keyword. In addition, raw identifiers allow
you to use libraries written in a different Rust edition than your crate uses.
For example, `try` isn’t a keyword in the 2015 edition but is in the 2018
edition. If you depend on a library that’s written using the 2015 edition and
has a `try` function, you’ll need to use the raw identifier syntax, `r#try` in
this case, to call that function from your 2018 edition code. See [Appendix
E][appendix-e] for more information on editions.
-->
生識別子を使えば、仮にそれが予約されたキーワードであろうとも、任意の単語を識別子として使えるようになります。
更に、あなたのクレートが使っている Rust の edition とは異なる edition で書かれたライブラリを呼び出すこともできるようになります。
たとえば、`try`は 2015 edition ではキーワードではありませんでしたが、2018 edition ではキーワードです。
もし、2015 edition で書かれており、`try`関数を持っているライブラリに依存している場合、あなたの 2018 edition のコードからその関数を呼び出すためには、生識別子構文を使う必要がでてくるでしょう。今回なら`r#try`ですね。
edition に関して、より詳しくは[付録 E][appendix-e]を見てください。

[appendix-e]: appendix-05-editions.html
