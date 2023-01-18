<!--
## Reading a File
-->

## ファイルを読み込む

<!--
Now we’ll add functionality to read the file that is specified in the
`filename` command line argument. First, we need a sample file to test it with:
the best kind of file to use to make sure `minigrep` is working is one with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
*poem.txt* at the root level of your project, and enter the poem “I’m Nobody!
Who are you?”
-->

では、`filename`コマンドライン引数で指定されたファイルを読み込む機能を追加しましょう。
まず、テスト実行するためのサンプルファイルが必要ですね：`minigrep`が動作していることを確かめるために使用するのに最適なファイルは、
複数行にわたって同じ単語の繰り返しのある少量のテキストです。リスト 12-3 は、
うまくいくであろうエミリー・ディキンソン (Emily Dickinson) の詩です！
プロジェクトのルート階層に*poem.txt*というファイルを作成し、この詩「私は誰でもない！あなたは誰？」を入力してください。

<!--
<span class="filename">Filename: poem.txt</span>
-->

<span class="filename">ファイル名：poem.txt</span>

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い 1 日に告げるのなんて。
感服するような沼地にね！
```

<!--
<span class="caption">Listing 12-3: A poem by Emily Dickinson makes a good test
case</span>
-->

<span class="caption">リスト 12-3: エミリー・ディキンソンの詩は、いいテストケースになる</span>

<!--
With the text in place, edit *src/main.rs* and add code to open the file, as
shown in Listing 12-4.
-->

テキストを適当な場所に置いて、*src/main.rs*を編集し、ファイルを開くコードを追加してください。
リスト 12-4 に示したようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,should_panic
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let query = &args[1];
#     let filename = &args[2];
#
#     println!("Searching for {}", query);
    // --snip--
    println!("In file {}", filename);

    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("With text:\n{}", contents);
}
```

<!--
<span class="caption">Listing 12-4: Reading the contents of the file specified
by the second argument</span>
-->

<span class="caption">リスト 12-4: 第 2 引数で指定されたファイルの中身を読み込む</span>

<!--
First, we add some more `use` statements to bring in relevant parts of the
standard library: we need `std::fs::File` to handle files, and
`std::io::prelude::*` contains various useful traits for doing I/O, including
file I/O. In the same way that Rust has a general prelude that brings certain
types and functions into scope automatically, the `std::io` module has its own
prelude of common types and functions you’ll need when working with I/O. Unlike
the default prelude, we must explicitly add a `use` statement for the
prelude from `std::io`.
-->

最初に、もう何個か`use`文を追記して、標準ライブラリの関係のある箇所を持ってきています：
ファイルを扱うのに`std::fs::File`が必要ですし、
`std::io::prelude::*`はファイル入出力を含む入出力処理をするのに有用なトレイトを色々含んでいます。
言語が一般的な初期化処理で特定の型や関数を自動的にスコープに導入するように、
`std::io`モジュールにはそれ独自の共通の型や関数の初期化処理があり、入出力を行う際に必要になるわけです。
標準の初期化処理とは異なり、`std::io`の初期化処理には明示的に`use`文を加えなければなりません。

<!--
In `main`, we’ve added three statements: first, we get a mutable handle to the
file by calling the `File::open` function and passing it the value of the
`filename` variable. Second, we create a variable called `contents` and set it
to a mutable, empty `String`. This will hold the content of the file after we
read it in. Third, we call `read_to_string` on our file handle and pass a
mutable reference to `contents` as an argument.
-->

`main`に 3 文を追記しました：一つ目が、`File::open`関数を呼んで`filename`変数の値に渡して、
ファイルへの可変なハンドルを得る処理です。二つ目が、`contents`という名の変数を生成して、
可変で空の`String`を割り当てる処理です。この変数が、ファイル読み込み後に中身を保持します。
三つ目が、ファイルハンドルに対して`read_to_string`を呼び出し、引数として`contents`への可変参照を渡す処理です。

<!--
After those lines, we’ve again added a temporary `println!` statement that
prints the value of `contents` after the file is read, so we can check that the
program is working so far.
-->

それらの行の後に、今回もファイル読み込み後に`contents`の値を出力する一時的な`println!`文を追記したので、
ここまでプログラムがきちんと動作していることを確認できます。

<!--
Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:
-->

第 1 コマンドライン引数には適当な文字列 (まだ検索する箇所は実装してませんからね) を、第 2 引数に*poem.txt*ファイルを入れて、
このコードを実行しましょう：

```text
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<!--
4 行目の冒頭は、末端の one idea をあえて訳していない。こちらの方が日本語としては自然と思われる
-->

<!--
Great! The code read and then printed the content of the file. But the code
has a few flaws. The `main` function has multiple responsibilities: generally,
functions are clearer and easier to maintain if each function is responsible
for only one idea. The other problem is that we’re not handling errors as well
as we could. The program is still small, so these flaws aren’t a big problem,
but as the program grows, it will be harder to fix them cleanly. It’s good
practice to begin refactoring early on when developing a program, because it’s
much easier to refactor smaller amounts of code. We’ll do that next.
-->

素晴らしい！コードがファイルの中身を読み込み、出力するようになりました。しかし、このコードにはいくつか欠陥があります。
`main`関数が複数の責任を受け持っています：一般に、各関数がただ一つの責任だけを持つようになれば、
関数は明確かつ、管理しやすくなります。もう一つの問題点は、できうる限りのエラー処理を怠っていることです。
まだプログラムが小規模なので、これらの欠陥は大きな問題にはなりませんが、プログラムが大規模になるにつれ、
それを綺麗に解消するのは困難になっていきます。プログラムを開発する際に早い段階でリファクタリングを行うのは、
良い戦術です。リファクタリングするコードの量が少なければ、はるかに簡単になりますからね。次は、それを行いましょう。
