<!--
## Working with Environment Variables
-->

## 環境変数を取り扱う

<!--
We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but by instead making it an
environment variable, we allow our users to set the environment variable once
and have all their searches be case insensitive in that terminal session.
-->

おまけの機能を追加して`minigrep`を改善します: 環境変数でユーザがオンにできる大文字小文字無視の検索用のオプションです。
この機能をコマンドラインオプションにして、適用したい度にユーザが入力しなければならないようにすることもできますが、
代わりに環境変数とすることで、ユーザは1回環境変数をセットすれば、そのターミナルセッションの間は大文字小文字無視の検索を行うことができるようにします。

<!--
### Writing a Failing Test for the Case-Insensitive `search` Function
-->

### 大文字小文字を区別しない`search`関数用に失敗するテストを書く

<!--
We first add a new `search_case_insensitive` function that will be called when
the environment variable has a value. We’ll continue to follow the TDD process,
so the first step is again to write a failing test. We’ll add a new test for
the new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20.
-->

まず、環境変数が値を持つ場合に呼び出される`search_case_insensitive`関数を新しく追加します。テスト駆動開発の過程に従い続けるので、
最初の手順は、今回も失敗するテストを書くことです。新しい`search_case_insensitive`関数用の新規テストを追加し、
古いテストを`one_result`から`case_sensitive`に名前変更して、二つのテストの差異を明確化します。
リスト12-20に示したようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>
-->

<span class="caption">リスト12-20: 追加しようとしている大文字小文字を区別しない関数用の失敗するテストを新しく追加する</span>

<!--
Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `"Duct tape."` using a capital D that shouldn’t match the query
`"duct"` when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.
-->

古いテストの`contents`も変更していることに注意してください。大文字小文字を区別する検索を行う際に、
`"duct"`というクエリに合致しないはずの大文字Dを使用した`"Duct tape"`(ガムテープ)という新しい行を追加しました。
このように古いテストを変更することで、既に実装済みの大文字小文字を区別する検索機能を誤って壊してしまわないことを保証する助けになります。
このテストはもう通り、大文字小文字を区別しない検索に取り掛かっても通り続けるはずです。

<!--
The new test for the case-*insensitive* search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we’re about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital R and match the line
`"Trust me."` even though both have different casing from the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.
-->

大文字小文字を区別*しない*検索の新しいテストは、クエリに`"rUsT"`を使用しています。
追加直前の`search_case_insensitive`関数では、`"rUsT"`というクエリは、
両方ともクエリとは大文字小文字が異なるのに、大文字Rの`"Rust:"`を含む行と、
`"Trust me."`という行にもマッチするはずです。これが失敗するテストであり、まだ`search_case_insensitive`関数を定義していないので、
コンパイルは失敗するでしょう。リスト12-16の`search`関数で行ったのと同様に空のベクタを常に返すような仮実装を追加し、テストがコンパイルされるものの、失敗する様をご自由に確認してください。

<!--
### Implementing the `search_case_insensitive` Function
-->

### `search_case_insensitive`関数を実装する

<!--
The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so whatever the case of the input arguments,
they’ll be the same case when we check whether the line contains the query.
-->

`search_case_insensitive`関数は、リスト12-21に示しましたが、`search`関数とほぼ同じです。
唯一の違いは、`query`と各`line`を小文字化していることなので、入力引数の大文字小文字によらず、
行がクエリを含んでいるか確認する際には、同じになるわけです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>
-->

<span class="caption">リスト12-21: 比較する前にクエリと行を小文字化するよう、`search_case_insensitive`関数を定義する</span>

<!--
First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no
matter whether the user’s query is `"rust"`, `"RUST"`, `"Rust"`, or `"rUsT"`,
we’ll treat the query as if it were `"rust"` and be insensitive to the case.
While `to_lowercase` will handle basic Unicode, it won’t be 100% accurate. If
we were writing a real application, we’d want to do a bit more work here, but
this section is about environment variables, not Unicode, so we’ll leave it at
that here.
-->

まず、`query`文字列を小文字化し、同じ名前の覆い隠された変数に保存します。ユーザのクエリが`"rust"`や`"RUST"`、
`"Rust"`、`"rUsT"`などだったりしても、`"rust"`であり、大文字小文字を区別しないかのようにクエリを扱えるように、
`to_lowercase`をクエリに対して呼び出すことは必須です。
`to_lowercase`は基本的なUnicodeを処理しますが、100%正確ではありません。
現実のアプリケーションを書いているとしたら、ここでもう少し処理を入れたくなるでしょうが、
この節は環境変数についての節であってUnicodeについての節ではないので、ここではそのままにしておきましょう。

<!--
Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.
-->

`query`は最早、文字列スライスではなく`String`であることに注意してください。というのも、
`to_lowercase`を呼び出すと、既存のデータを参照するというよりも、新しいデータを作成するからです。
例として、クエリは`"rUsT"`だとしましょう: その文字列スライスは、小文字の`u`や`t`を使えるように含んでいないので、
`"rust"`を含む新しい`String`のメモリを確保しなければならないのです。今、`contains`メソッドに引数として`query`を渡すと、
アンド記号を追加する必要があります。`contains`のシグニチャは、文字列スライスを取るよう定義されているからです。

<!--
Next, we add a call to `to_lowercase` on each `line` to lowercase all
characters. Now that we’ve converted `line` and `query` to lowercase, we’ll
find matches no matter what the case of the query is.
-->

次に、各`line`に対して`to_lowercase`の呼び出しを追加し、全文字を小文字化しています。
今や`line`と`query`を小文字に変換したので、クエリが大文字であろうと小文字であろうとマッチを検索するでしょう。

<!--
Let’s see if this implementation passes the tests:
-->

この実装がテストを通過するか確認しましょう:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

<!--
Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors because we aren’t initializing
this field anywhere yet:
-->

素晴らしい！どちらも通りました。では、`run`関数から新しい`search_case_insensitive`関数を呼び出しましょう。
1番目に大文字小文字の区別を切り替えられるよう、`Config`構造体に設定オプションを追加します。
まだどこでも、このフィールドの初期化をしていないので、追加するとコンパイルエラーが起きます:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

<!--
We added the `ignore_case` field that holds a Boolean. Next, we need the `run`
function to check the `ignore_case` field’s value and use that to decide
whether to call the `search` function or the `search_case_insensitive`
function, as shown in Listing 12-22. This still won’t compile yet.
-->

論理値を持つ`ignore_case`フィールドを追加しました。次に、`run`関数に、
`ignore_case`フィールドの値を確認し、`search`関数か`search_case_insensitive`関数を呼ぶかを決定するのに使ってもらう必要があります。
リスト12-22のようにですね。それでも、これはまだコンパイルできません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

<!--
<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.ignore_case`</span>
-->

<span class="caption">リスト12-22: `config.ignore_case`の値に基づいて`search`か`search_case_insensitive`を呼び出す</span>

<!--
Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we bring that module into scope at the top of *src/lib.rs*. Then
we’ll use the `var` function from the `env` module to check to see if any value
has been set for an environment variable named `IGNORE_CASE`, as shown in
Listing 12-23.
-->

最後に、環境変数を確認する必要があります。環境変数を扱う関数は、標準ライブラリの`env`モジュールにあるので、
*src/lib.rs*の冒頭でそのモジュールをスコープ内に持ち込みます。そして、
`env`モジュールから`var`関数を使用して、`IGNORE_CASE`という環境変数に何らかの値が設定されているかチェックします。
リスト12-23のようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-23: Checking for any value in an environment
variable named `IGNORE_CASE`</span>
-->

<span class="caption">リスト12-23: `IGNORE_CASE`という環境変数に何らかの値が設定されているかチェックする</span>

<!--
Here, we create a new variable `ignore_case`. To set its value, we call the
`env::var` function and pass it the name of the `IGNORE_CASE` environment
variable. The `env::var` function returns a `Result` that will be the
successful `Ok` variant that contains the value of the environment variable if
the environment variable is set to any value. It will return the `Err` variant
if the environment variable is not set.
-->

ここで、`ignore_case`という新しい変数を生成しています。その値をセットするために、
`env::var`関数を呼び出し、`IGNORE_CASE`環境変数の名前を渡しています。`env::var`関数は、
環境変数に何らかの値がセットされていたら、環境変数の値を含む`Ok`列挙子の成功値になる`Result`を返します。
環境変数がセットされていなければ、`Err`列挙子を返すでしょう。

<!--
We’re using the `is_ok` method on the `Result` to check whether the environment
variable is set, which means the program should do a case-insensitive search.
If the `IGNORE_CASE` environment variable isn’t set to anything, `is_ok` will
return false and the program will perform a case-sensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_ok` rather than using `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.
-->

`Result`の`is_ok`メソッドを使用して、環境変数が設定されているか、つまりプログラムが大文字小文字を区別しない検索を行うべきかどうかを、
チェックしています。`IGNORE_CASE`環境変数が何にも設定されていなければ、
`is_ok`はfalseを返し、プログラムは大文字小文字を区別する検索を実行するでしょう。環境変数の*値*はどうでもよく、
セットされているかどうかだけ気にするので、`unwrap`や`expect`あるいは、他のここまで見かけた`Result`のメソッドを使用するのではなく、
`is_ok`をチェックしています。

<!--
We pass the value in the `ignore_case` variable to the `Config` instance so the
`run` function can read that value and decide whether to call
`search_case_insensitive` or `search`, as we implemented in Listing 12-22.
-->

`ignore_case`変数の値を`Config`インスタンスに渡しているので、リスト12-22で実装したように、
`run`関数はその値を読み取り、`search_case_insensitive`か`search`を呼び出すか決定できるのです。

<!--
Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:
-->

試行してみましょう！まず、環境変数をセットせずにクエリは`to`でプログラムを実行し、
この時は全て小文字で"to"という言葉を含むあらゆる行が合致するはずです。

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

<!--
Looks like that still works! Now, let’s run the program with `IGNORE_CASE`
set to `1` but with the same query `to`.
-->

まだ機能しているようです！では、`IGNORE_CASE`を1にしつつ、同じクエリの`to`でプログラムを実行しましょう。

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

<!--
If you’re using PowerShell, you will need to set the environment variable and
run the program as separate commands:
-->

PowerShellを使用しているなら、環境変数の設定とプログラムの実行を別々のコマンドとして実行する必要があるでしょう:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

<!--
This will make `IGNORE_CASE` persist for the remainder of your shell
session. It can be unset with the `Remove-Item` cmdlet:
-->

これを実行すると、`IGNORE_CASE`は以降のシェルセッションでも残り続けるでしょう。
これは`Remove-Item`コマンドレットで解除することができます:

```console
PS> Remove-Item Env:IGNORE_CASE
```


<!--
We should get lines that contain “to” that might have uppercase letters:
-->

大文字も含む可能性のある"to"を含有する行が得られるはずです:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

<!--
Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.
-->

素晴らしい、"To"を含む行も出てきましたね！`minigrep`プログラムはこれで、
環境変数によって制御できる大文字小文字を区別しない検索も行えるようになりました。もうコマンドライン引数か、
環境変数を使ってオプションを管理する方法も知りましたね。

<!--
Some programs allow arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case sensitivity
through either a command line argument or an environment variable. Decide
whether the command line argument or the environment variable should take
precedence if the program is run with one set to case sensitive and one set to
ignore case.
-->

引数*と*環境変数で同じ設定を行うことができるプログラムもあります。そのような場合、
プログラムはどちらが優先されるか決定します。自身の別の鍛錬として、コマンドライン引数か、
環境変数で大文字小文字の区別を制御できるようにしてみてください。
片方は大文字小文字を区別するようにセットされ、もう片方は無視するようにセットしてプログラムが実行された時に、
コマンドライン引数と環境変数のどちらの優先度が高くなるかを決めてください。

<!--
The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
-->

`std::env`モジュールは、環境変数を扱うもっと多くの有用な機能を有しています:
ドキュメンテーションを確認して、何が利用可能か確かめてください。
