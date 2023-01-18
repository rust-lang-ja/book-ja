<!--
## Writing Error Messages to Standard Error Instead of Standard Output
-->

## 標準出力ではなく標準エラーにエラーメッセージを書き込む

<!--
At the moment we’re writing all of our output to the terminal using the
`println!` function. Most terminals provide two kinds of output: *standard
output* (`stdout`) for general information and *standard error* (`stderr`)
for error messages. This distinction enables users to choose to direct the
successful output of a program to a file but still print error messages to the
screen.
-->

現時点では、すべての出力を`println!`関数を使用して端末に書き込んでいます。多くの端末は、
2 種類の出力を提供します：普通の情報用の*標準出力*(`stdout`) とエラーメッセージ用の*標準エラー出力*(`stderr`) です。
この差異のおかげで、ユーザは、エラーメッセージを画面に表示しつつ、
プログラムの成功した出力をファイルにリダイレクトすることを選択できます。

<!--
The `println!` function is only capable of printing to standard output, so we
have to use something else to print to standard error.
-->

`println!`関数は、標準出力に出力する能力しかないので、標準エラーに出力するには他のものを使用しなければなりません。

<!--
### Checking Where Errors Are Written to
-->

### エラーが書き込まれる場所を確認する

<!--
First, let’s observe how the content printed by `minigrep` is currently being
written to standard output, including any error messages we want to write to
standard error instead. We’ll do that by redirecting the standard output stream
to a file while also intentionally causing an error. We won’t redirect the
standard error stream, so any content sent to standard error will continue to
display on the screen.
-->

まず、`minigrep`に出力される中身が、代わりに標準エラーに書き込みたいいかなるエラーメッセージも含め、
どのように標準出力に書き込まれているかを観察しましょう。意図的にエラーを起こしつつ、
ファイルに標準出力ストリームをリダイレクトすることでそうします。標準エラーストリームはリダイレクトしないので、
標準エラーに送られる内容は、すべて画面に表示され続けます。

<!--
Command line programs are expected to send error messages to the standard error
stream so we can still see error messages on the screen even if we redirect the
standard output stream to a file. Our program is not currently well-behaved:
we’re about to see that it saves the error message output to a file instead!
-->

コマンドラインプログラムは、エラーメッセージを標準エラー出力に送信していると期待されているので、
標準出力ストリームをファイルにリダイレクトしても、画面にエラーメッセージが見られます。
我々のプログラムは、現状、いい振る舞いをしていません：代わりにファイルにエラーメッセージ出力を保存するところを、
目撃するところです！

<!--
The way to demonstrate this behavior is by running the program with `>` and the
filename, *output.txt*, that we want to redirect the standard output stream to.
We won’t pass any arguments, which should cause an error:
-->

この動作をデモする方法は、`>`と標準出力ストリームをリダイレクトする先のファイル名、*output.txt*でプログラムを走らせることによります。
引数は何も渡さず、そうするとエラーが起きるはずです：

```text
$ cargo run > output.txt
```

<!--
The `>` syntax tells the shell to write the contents of standard output to
*output.txt* instead of the screen. We didn’t see the error message we were
expecting printed to the screen, so that means it must have ended up in the
file. This is what *output.txt* contains:
-->

`>`記法により、標準出力の中身を画面の代わりに*output.txt*に書き込むようシェルは指示されます。
画面に出力されると期待していたエラーメッセージは見られないので、ファイルに入っているということでしょう。
以下が*output.txt*が含んでいる内容です：

```text
Problem parsing arguments: not enough arguments
```

<!--
Yup, our error message is being printed to standard output. It’s much more
useful for error messages like this to be printed to standard error so only
data from a successful run ends up in the file. We'll change that.
-->

そうです。エラーメッセージは標準出力に出力されているのです。このようなエラーメッセージは標準エラーに出力され、
成功した状態のデータのみがファイルに残ると遥かに有用です。それを変更します。

<!--
### Printing Errors to Standard Error
-->

### エラーを標準エラーに出力する

<!--
We’ll use the code in Listing 12-24 to change how error messages are printed.
Because of the refactoring we did earlier in this chapter, all the code that
prints error messages is in one function, `main`. The standard library provides
the `eprintln!` macro that prints to the standard error stream, so let’s change
the two places we were calling `println!` to print errors to use `eprintln!`
instead.
-->

リスト 12-24 のコードを使用して、エラーメッセージの出力の仕方を変更します。この章の前で行ったリファクタリングのため、
エラーメッセージを出力するコードはすべて 1 関数、`main`にあります。標準ライブラリは、
標準エラーストリームに出力する`eprintln!`マクロを提供しているので、
`println!`を呼び出してエラーを出力していた 2 箇所を代わりに`eprintln!`を使うように変更しましょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

<!--
<span class="caption">Listing 12-24: Writing error messages to standard error
instead of standard output using `eprintln!`</span>
-->

<span class="caption">リスト 12-24: `eprintln!`を使って標準出力ではなく、標準エラーにエラーメッセージを書き込む</span>

<!--
After changing `println!` to `eprintln!`, let’s run the program again in the
same way, without any arguments and redirecting standard output with `>`:
-->

`println!`を`eprintln!`に変えてから、再度同じようにプログラムを実行しましょう。
引数なしかつ、標準出力を`>`でリダイレクトしてね：

```text
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

<!--
Now we see the error onscreen and *output.txt* contains nothing, which is the
behavior we expect of command line programs.
-->

これで、エラーは画面に見えつつ、*output.txt*は何も含まなくなり、これはコマンドラインプログラムに期待する動作です。

<!--
Let’s run the program again with arguments that don’t cause an error but still
redirect standard output to a file, like so:
-->

再度、標準出力をファイルにリダイレクトしてエラーは起こさない引数でプログラムを走らせましょう。以下のようにですね：

```text
$ cargo run to poem.txt > output.txt
```

<!--
We won’t see any output to the terminal, and *output.txt* will contain our
results:
-->

ターミナルには出力は見られず、*output.txt*に結果が含まれます：

<!--
<span class="filename">Filename: output.txt</span>
-->

<span class="filename">ファイル名：output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

<!--
This demonstrates that we’re now using standard output for successful output
and standard error for error output as appropriate.
-->

これは、もう成功した出力には標準出力を、エラー出力には標準エラーを適切に使用していることをデモしています。

<!--
## Summary
-->

## まとめ

<!--
This chapter recapped some of the major concepts you’ve learned so far and
covered how to perform common I/O operations in Rust. By using command line
arguments, files, environment variables, and the `eprintln!` macro for printing
errors, you’re now prepared to write command line applications. By using the
concepts in previous chapters, your code will be well organized, store data
effectively in the appropriate data structures, handle errors nicely, and be
well tested.
-->

この章では、ここまでに学んできた主要な概念の一部を念押しし、Rust で入出力処理を行う方法を講義しました。
コマンドライン引数、ファイル、環境変数、そしてエラー出力に`eprintln!`マクロを使用することで、
もう、コマンドラインアプリケーションを書く準備ができています。以前の章の概念を使用することで、
コードはうまく体系化され、適切なデータ構造に効率的にデータを保存し、エラーをうまく扱い、
よくテストされるでしょう。

<!--
Next, we’ll explore some Rust features that were influenced by functional
languages: closures and iterators.
-->

次は、関数型言語に影響された Rust 機能を一部探究します：クロージャとイテレータです。
