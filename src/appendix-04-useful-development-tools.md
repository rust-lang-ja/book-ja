<!--
## Appendix D - Useful Development Tools
-->
## 付録 D - 便利な開発ツール

<!--
In this appendix, we talk about some useful development tools that the Rust
project provides. We’ll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.
-->
この付録では、Rust プロジェクトの提供する便利な開発ツールについていくつかお話します。
自動フォーマット、警告に対する修正をすばやく適用する方法、lint ツール、そして IDE との統合について見ていきます。

<!--
### Automatic Formatting with `rustfmt`
-->
### `rustfmt`を使った自動フォーマット


<!--
The `rustfmt` tool reformats your code according to the community code style.
Many collaborative projects use `rustfmt` to prevent arguments about which
style to use when writing Rust: everyone formats their code using the tool.
-->
`rustfmt`というツールは、コミュニティのコードスタイルに合わせてあなたのコードをフォーマットしてくれます。
Rust を書くときにどのスタイルを使うかで揉めないように、多くの共同で行われるプロジェクトが`rustfmt`を使っています：全員がこのツールでコードをフォーマットするのです。

<!--
To install `rustfmt`, enter the following:
-->
`rustfmt`をインストールするには、以下を入力してください：

```console
$ rustup component add rustfmt
```

<!--
This command gives you `rustfmt` and `cargo-fmt`, similar to how Rust gives you
both `rustc` and `cargo`. To format any Cargo project, enter the following:
-->
これで`rustfmt`と`cargo-fmt`が使えるようになります。これは`rustc`と`cargo`の両方のコマンドがあるのと似たようなものです。
どんな Cargo のプロジェクトも、次を入力するとフォーマットできます：

```console
$ cargo fmt
```

<!--
Running this command reformats all the Rust code in the current crate. This
should only change the code style, not the code semantics. For more information
on `rustfmt`, see [its documentation][rustfmt].
-->
このコマンドを実行すると、現在のクレートのあらゆる Rust コードをフォーマットし直します。
これを行うと、コードのスタイルのみが変わり、コードの意味は変わりません。
`rustfmt`についてより詳しく知るには[ドキュメント][rustfmt]を読んでください。

[rustfmt]: https://github.com/rust-lang/rustfmt

<!--
### Fix Your Code with `rustfix`
-->
### `rustfix`でコードを修正する

<!--
The rustfix tool is included with Rust installations and can automatically fix
some compiler warnings. If you’ve written code in Rust, you’ve probably seen
compiler warnings. For example, consider this code:
-->
rustfix というツールは Rust をインストールすると同梱されており、コンパイラの警告 (warning) を自動で直してくれます。
Rust でコードを書いたことがある人なら、コンパイラの警告を見たことがあるでしょう。
たとえば、下のコードを考えます：

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

<!--
Here, we’re calling the `do_something` function 100 times, but we never use the
variable `i` in the body of the `for` loop. Rust warns us about that:
-->
ここで、`do_something`関数を 100 回呼んでいますが、`for`ループの内部で変数`i`を一度も使っていません。
Rust はこれについて警告します：

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

<!--
The warning suggests that we use `_i` as a name instead: the underscore
indicates that we intend for this variable to be unused. We can automatically
apply that suggestion using the `rustfix` tool by running the command `cargo
fix`:
-->
警告は、変数名に`_i`を使ってはどうかと提案しています：アンダーバーはその変数を使わないという意図を示すのです。
`cargo fix`というコマンドを実行することで、この提案を`rustfix`ツールで自動で適用できます。

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

<!--
When we look at *src/main.rs* again, we’ll see that `cargo fix` has changed the
code:
-->
*src/main.rs*をもう一度見てみると、`cargo fix`によってコードが変更されていることがわかります。

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

<!--
The `for` loop variable is now named `_i`, and the warning no longer appears.
-->
`for`ループの変数は`_i`という名前になったので、警告はもう現れません。

<!--
You can also use the `cargo fix` command to transition your code between
different Rust editions. Editions are covered in Appendix E.
-->
`cargo fix`コマンドを使うと、異なる Rust edition の間でコードを変換することもできます。
edition については付録 E に書いています。

<!--
### More Lints with Clippy
-->
### Clippy でもっと lint を

<!--
The Clippy tool is a collection of lints to analyze your code so you can catch
common mistakes and improve your Rust code.
-->
Clippy というツールは、コードを分析することで、よくある間違いを見つけ、Rust のコードを改善させてくれる lint を集めたものです（訳注：いわゆる静的解析ツール）。

<!--
To install Clippy, enter the following:
-->
Clippy をインストールするには、次を入力してください：

```console
$ rustup component add clippy
```

<!--
To run Clippy’s lints on any Cargo project, enter the following:
-->
Clippy の lint は、次のコマンドでどんな Cargo プロジェクトに対しても実行できます：

```console
$ cargo clippy
```

<!--
For example, say you write a program that uses an approximation of a
mathematical constant, such as pi, as this program does:
-->
たとえば、下のように、円周率などの数学定数の近似を使ったプログラムを書いているとします。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

<!--
Running `cargo clippy` on this project results in this error:
-->
`cargo clippy`をこのプロジェクトに実行すると次のエラーを得ます：

```text
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

<!--
This error lets you know that Rust has this constant defined more precisely and
that your program would be more correct if you used the constant instead. You
would then change your code to use the `PI` constant. The following code
doesn’t result in any errors or warnings from Clippy:
-->
あなたは、このエラーのおかげで、Rust にはより正確に定義された定数がすでにあり、これを代わりに使うとプログラムがより正しくなるかもしれないと気づくことができます。
なので、あなたはコードを定数`PI`を使うように変更するでしょう。
以下のコードはもう Clippy からエラーや警告は受けません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

<!--
For more information on Clippy, see [its documentation][clippy].
-->
Clippy についてより詳しく知るには、[ドキュメント][clippy]を読んでください。

[clippy]: https://github.com/rust-lang/rust-clippy

<!--
### IDE Integration Using the Rust Language Server
-->
### Rust Language Server を使って IDE と統合する

<!--
To help IDE integration, the Rust project distributes the *Rust Language
Server* (`rls`). This tool speaks the [Language Server
Protocol][lsp], which is a specification for IDEs and programming
languages to communicate with each other. Different clients can use the `rls`,
such as [the Rust plug-in for Visual Studio Code][vscode].
-->
IDE での開発の助けになるよう、Rust プロジェクトは *Rust Language Server* (`rls`) を配布しています。
このツールは、[Language Server Protocol][lsp]という、IDEとプログラミング言語が対話するための仕様に対応しています。
[Visual Studio Code の Rust プラグイン][vscode]をはじめ、様々なクライアントが`rls`を使うことができます。

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

<!--
To install the `rls`, enter the following:
-->
`rls`をインストールするには、以下を入力してください：

```console
$ rustup component add rls
```

<!--
Then install the language server support in your particular IDE; you’ll gain
abilities such as autocompletion, jump to definition, and inline errors.
-->
つづけて、あなたの IDE 向けの language server サポートをインストールしてください。
すると、自動補完、定義へのジャンプ、インラインのエラー表示などの機能が得られるはずです。

<!--
For more information on the `rls`, see [its documentation][rls].
-->
`rls`についてより詳しく知るには[ドキュメント][rls]を読んでください。

[rls]: https://github.com/rust-lang/rls
