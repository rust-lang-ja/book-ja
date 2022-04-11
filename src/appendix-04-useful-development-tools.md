<!--
## Appendix D - Useful Development Tools
-->
## 付録D - 便利な開発ツール

<!--
In this appendix, we talk about some useful development tools that the Rust
project provides. We’ll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.
-->
この付録では、Rustプロジェクトの提供する便利な開発ツールについていくつかお話します。
自動フォーマット、警告に対する修正をすばやく適用する方法、lintツール、そしてIDEとの統合について見ていきます。

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
Rustを書くときにどのスタイルを使うかで揉めないように、多くの共同で行われるプロジェクトが`rustfmt`を使っています：全員がこのツールでコードをフォーマットするのです。

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
どんなCargoのプロジェクトも、次を入力するとフォーマットできます：

```console
$ cargo fmt
```

<!--
Running this command reformats all the Rust code in the current crate. This
should only change the code style, not the code semantics. For more information
on `rustfmt`, see [its documentation][rustfmt].
-->
このコマンドを実行すると、現在のクレートのあらゆるRustコードをフォーマットし直します。
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
rustfixというツールはRustをインストールすると同梱されており、コンパイラの警告 (warning) を自動で直してくれます。
Rustでコードを書いたことがある人なら、コンパイラの警告を見たことがあるでしょう。
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
ここで、`do_something`関数を100回呼んでいますが、`for`ループの内部で変数`i`を一度も使っていません。
Rustはこれについて警告します：

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
`cargo fix`コマンドを使うと、異なるRust editionの間でコードを変換することもできます。
editionについては付録Eに書いています。

<!--
### More Lints with Clippy
-->
### Clippyでもっとlintを

<!--
The Clippy tool is a collection of lints to analyze your code so you can catch
common mistakes and improve your Rust code.
-->
Clippyというツールは、コードを分析することで、よくある間違いを見つけ、Rustのコードを改善させてくれるlintを集めたものです（訳注：いわゆる静的解析ツール）。

<!--
To install Clippy, enter the following:
-->
Clippyをインストールするには、次を入力してください：

```console
$ rustup component add clippy
```

<!--
To run Clippy’s lints on any Cargo project, enter the following:
-->
Clippyのlintは、次のコマンドでどんなCargoプロジェクトに対しても実行できます：

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
<span class="filename">ファイル名: src/main.rs</span>

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
あなたは、このエラーのおかげで、Rustにはより正確に定義された定数がすでにあり、これを代わりに使うとプログラムがより正しくなるかもしれないと気づくことができます。
なので、あなたはコードを定数`PI`を使うように変更するでしょう。
以下のコードはもうClippyからエラーや警告は受けません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名: src/main.rs</span>

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
Clippyについてより詳しく知るには、[ドキュメント][clippy]を読んでください。

[clippy]: https://github.com/rust-lang/rust-clippy

<!--
### IDE Integration Using `rust-analyzer`
-->
### `rust-analyzer` を使ってIDEと統合する

<!--
To help IDE integration, the Rust community recommends using
[`rust-analyzer`][rust-analyzer]. This tool is a set of compiler-centric
utilities that speaks the [Language Server Protocol][lsp], which is a
specification for IDEs and programming languages to communicate with each
other. Different clients can use `rust-analyzer`, such as [the Rust analyzer
plug-in for Visual Studio Code][vscode].
-->
IDEでの開発の助けになるよう、Rustコミュニティは [`rust-analyzer`][rust-analyzer] の使用を奨励しています。
このツールはコンパイラを中心としたユーティリティのセットで、[Language Server Protocol][lsp]という、IDEとプログラミング言語が対話するための仕様に対応しています。
[Visual Studio CodeのRust analyzerプラグイン][vscode]をはじめ、様々なクライアントが`rust-analyzer`を使うことができます。

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

<!-- Visit the `rust-analyzer` project’s [home page][rust-analyzer] for installation
instructions, then install the language server support in your particular IDE.
Your IDE will gain abilities such as autocompletion, jump to definition, and
inline errors. -->

`rust-analyzer` プロジェクトの [ホームページ][rust-analyzer] の説明にしたがってインストールを行い、つづけて、あなたのIDE向けのlanguage serverサポートをインストールしてください。
すると、自動補完、定義へのジャンプ、インラインのエラー表示などの機能が得られるはずです。

<!--
For more information on `rust-analyzer`, see [its documentation][rust-analyzer-manual].
-->
`rust-analyzer`についてより詳しく知るには[ドキュメント][rust-analyzer-manual]を読んでください。

[rust-analyzer]: https://rust-analyzer.github.io
[rust-analyzer-manual]: https://rust-analyzer.github.io/manual.html
