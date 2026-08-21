<!--
## Reading a File
-->

## ファイルを読み込む

<!--
Now we’ll add functionality to read the file specified in the `file_path`
argument. First, we need a sample file to test it with: we’ll use a file with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
*poem.txt* at the root level of your project, and enter the poem “I’m Nobody!
Who are you?”
-->

では、`file_path`引数で指定されたファイルを読み込む機能を追加しましょう。
まず、テスト実行するためのサンプルファイルが必要ですね:
複数行にわたって、同じ単語の繰り返しのある、少量のテキストを含むようなファイルを使いましょう。
リスト12-3は、ちょうど良さそうなエミリー・ディキンソン(Emily Dickinson)の詩です！
プロジェクトのルート階層に*poem.txt*というファイルを作成し、この詩“I’m Nobody! Who are you?”を入力してください。

<!--
<span class="filename">Filename: poem.txt</span>
-->

<span class="filename">ファイル名: poem.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

<!--
<span class="caption">Listing 12-3: A poem by Emily Dickinson makes a good test
case</span>
-->

<span class="caption">リスト12-3: エミリー・ディキンソンの詩は、いいテストケースになる</span>

<!--
With the text in place, edit *src/main.rs* and add code to read the file, as
shown in Listing 12-4.
-->

テキストを適当な場所に置いて、*src/main.rs*を編集し、ファイルを読むコードを追加してください。
リスト12-4に示したようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-4: Reading the contents of the file specified
by the second argument</span>
-->

<span class="caption">リスト12-4: 第2引数で指定されたファイルの中身を読み込む</span>

<!--
First, we bring in a relevant part of the standard library with a `use`
statement: we need `std::fs` to handle files.
-->

最初に、`use`文で、標準ライブラリの関係のある箇所を持ってきています:
ファイルを扱うのには`std::fs`が必要です。

<!--
In `main`, the new statement `fs::read_to_string` takes the `file_path`, opens
that file, and returns a `std::io::Result<String>` of the file’s contents.
-->

`main`に追加した`fs::read_to_string`文は、`file_path`を受け取り、そのファイルを開き、
そのファイルの内容を含む`std::io::Result<String>`を返します。

<!--
After that, we again add a temporary `println!` statement that prints the value
of `contents` after the file is read, so we can check that the program is
working so far.
-->

その後、今回もファイル読み込み後に`contents`の値を出力する一時的な`println!`文を追記しているので、
ここまでプログラムがきちんと動作していることを確認できます。

<!--
Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:
-->

第1コマンドライン引数には適当な文字列(まだ検索する箇所は実装してませんからね)を、第2引数に*poem.txt*ファイルを入れて、
このコードを実行しましょう:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

<!--
4行目の冒頭は、末端のone ideaをあえて訳していない。こちらの方が日本語としては自然と思われる
-->

<!--
Great! The code read and then printed the contents of the file. But the code
has a few flaws. At the moment, the `main` function has multiple
responsibilities: generally, functions are clearer and easier to maintain if
each function is responsible for only one idea. The other problem is that we’re
not handling errors as well as we could. The program is still small, so these
flaws aren’t a big problem, but as the program grows, it will be harder to fix
them cleanly. It’s good practice to begin refactoring early on when developing
a program, because it’s much easier to refactor smaller amounts of code. We’ll
do that next.
-->

素晴らしい！コードがファイルの中身を読み込み、出力するようになりました。しかし、このコードにはいくつか欠陥があります。
現時点で、`main`関数は複数の責任を受け持っています: 一般に、各関数がただ一つの責任だけを持つようになれば、
関数は明確かつ、管理しやすくなります。もう一つの問題点は、できうる限りのエラー処理を怠っていることです。
まだプログラムが小規模なので、これらの欠陥は大きな問題にはなりませんが、プログラムが大規模になるにつれ、
それを綺麗に解消するのは困難になっていきます。プログラムを開発する際に早い段階でリファクタリングを行うのは、
良い戦術です。リファクタリングするコードの量が少なければ、はるかに簡単になりますからね。次は、それを行いましょう。
