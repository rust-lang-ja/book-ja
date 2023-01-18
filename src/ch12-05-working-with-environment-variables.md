<!--
## Working with Environment Variables
-->

## 環境変数を取り扱う

<!--
We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but instead we’ll use an
environment variable. Doing so allows our users to set the environment variable
once and have all their searches be case insensitive in that terminal session.
-->

おまけの機能を追加して`minigrep`を改善します：環境変数でユーザがオンにできる大文字小文字無視の検索用のオプションです。
この機能をコマンドラインオプションにして、適用したい度にユーザが入力しなければならないようにすることもできますが、
代わりに環境変数を使用します。そうすることでユーザは 1 回環境変数をセットすれば、そのターミナルセッションの間は、
大文字小文字無視の検索を行うことができるようになるわけです。

<!--
### Writing a Failing Test for the Case-Insensitive `search` Function
-->

### 大文字小文字を区別しない`search`関数用に失敗するテストを書く

<!--
We want to add a new `search_case_insensitive` function that we’ll call when
the environment variable is on. We’ll continue to follow the TDD process, so
the first step is again to write a failing test. We’ll add a new test for the
new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20.
-->

環境変数がオンの場合に呼び出す`search_case_insensitive`関数を新しく追加したいです。テスト駆動開発の過程に従い続けるので、
最初の手順は、今回も失敗するテストを書くことです。新しい`search_case_insensitive`関数用の新規テストを追加し、
古いテストを`one_result`から`case_sensitive`に名前変更して、二つのテストの差異を明確化します。
リスト 12-20 に示したようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
// Rust
// 安全かつ高速で生産的
// 三つを選んで
// ガムテープ
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
// (最後の行のみ)
// 私を信じて
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<!--
<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>
-->

<span class="caption">リスト 12-20: 追加しようとしている大文字小文字を区別しない関数用の失敗するテストを新しく追加する</span>

<!--
Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `“Duct tape”` using a capital D that shouldn’t match the query
`“duct”` when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.
-->

古いテストの`contents`も変更していることに注意してください。大文字小文字を区別する検索を行う際に、
`"duct"`というクエリに合致しないはずの大文字 D を使用した`"Duct tape"`(ガムテープ) という新しい行を追加しました。
このように古いテストを変更することで、既に実装済みの大文字小文字を区別する検索機能を誤って壊してしまわないことを保証する助けになります。
このテストはもう通り、大文字小文字を区別しない検索に取り掛かっても通り続けるはずです。

<!--
The new test for the case-*insensitive* search uses “rUsT” as its query. In the
`search_case_insensitive` function we’re about to add, the query “rUsT”
should match the line containing “Rust:” with a capital R and match the line
`“Trust me.”` even though both have different casing than the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.
-->

大文字小文字を区別*しない*検索の新しいテストは、クエリに"rUsT"を使用しています。
追加直前の`search_case_insensitive`関数では、"rUsT"というクエリは、
両方ともクエリとは大文字小文字が異なるのに、大文字 R の"Rust:"を含む行と、
`“Trust me.”`という行にもマッチするはずです。これが失敗するテストであり、まだ`search_case_insensitive`関数を定義していないので、
コンパイルは失敗するでしょう。リスト 12-16 の`search`関数で行ったのと同様に空のベクタを常に返すような仮実装を追加し、テストがコンパイルされるものの、失敗する様をご自由に確認してください。

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

`search_case_insensitive`関数は、リスト 12-21 に示しましたが、`search`関数とほぼ同じです。
唯一の違いは、`query`と各`line`を小文字化していることなので、入力引数の大文字小文字によらず、
行がクエリを含んでいるか確認する際には、同じになるわけです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

<!--
<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>
-->

<span class="caption">リスト 12-21: 比較する前にクエリと行を小文字化するよう、`search_case_insensitive`関数を定義する</span>

<!--
First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no matter
whether the user’s query is `“rust”`, `“RUST”`, `“Rust”`, or `“rUsT”`, we’ll treat the
query as if it was `“rust”` and be insensitive to the case.
-->

まず、`query`文字列を小文字化し、同じ名前の覆い隠された変数に保存します。ユーザのクエリが`"rust"`や`"RUST"`、
`"Rust"`、`"rUsT"`などだったりしても、`"rust"`であり、大文字小文字を区別しないかのようにクエリを扱えるように、
`to_lowercase`をクエリに対して呼び出すことは必須です。

<!--
Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `“rUsT”`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`“rust”`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.
-->

`query`は最早、文字列スライスではなく`String`であることに注意してください。というのも、
`to_lowercase`を呼び出すと、既存のデータを参照するというよりも、新しいデータを作成するからです。
例として、クエリは`"rUsT"`だとしましょう：その文字列スライスは、小文字の`u`や`t`を使えるように含んでいないので、
`"rust"`を含む新しい`String`のメモリを確保しなければならないのです。今、`contains`メソッドに引数として`query`を渡すと、
アンド記号を追加する必要があります。`contains`のシグニチャは、文字列スライスを取るよう定義されているからです。

<!--
2 行目真ん中、to lowercase ...がかかる先が微妙。今の訳の通りなら、before の前に to を記述する気もする
-->

<!--
Next, we add a call to `to_lowercase` on each `line` before we check whether it
contains `query` to lowercase all characters. Now that we’ve converted `line`
and `query` to lowercase, we’ll find matches no matter what the case of the
query is.
-->

次に、各`line`が`query`を含むか確かめる前に`to_lowercase`の呼び出しを追加し、全文字を小文字化しています。
今や`line`と`query`を小文字に変換したので、クエリが大文字であろうと小文字であろうとマッチを検索するでしょう。

<!--
Let’s see if this implementation passes the tests:
-->

この実装がテストを通過するか確認しましょう：

```text
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors because we aren’t initializing
this field anywhere yet:
-->

素晴らしい！どちらも通りました。では、`run`関数から新しい`search_case_insensitive`関数を呼び出しましょう。
1 番目に大文字小文字の区別を切り替えられるよう、`Config`構造体に設定オプションを追加します。
まだどこでも、このフィールドの初期化をしていないので、追加するとコンパイルエラーが起きます：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

<!--
Note that we added the `case_sensitive` field that holds a Boolean. Next, we
need the `run` function to check the `case_sensitive` field’s value and use
that to decide whether to call the `search` function or the
`search_case_insensitive` function, as shown in Listing 12-22. Note this still
won’t compile yet.
-->

論理値を持つ`case_sensitive`フィールドを追加したことに注意してください。次に、`run`関数に、
`case_sensitive`フィールドの値を確認し、`search`関数か`search_case_insensitive`関数を呼ぶかを決定するのに使ってもらう必要があります。
リスト 12-22 のようにですね。それでも、これはまだコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# pub struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<!--
<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.case_sensitive`</span>
-->

<span class="caption">リスト 12-22: `config.case_sensitive`の値に基づいて`search`か`search_case_insensitive`を呼び出す</span>

<!--
Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we want to bring that module into scope with a `use std::env;` line
at the top of *src/lib.rs*. Then we’ll use the `var` function from the `env`
module to check for an environment variable named `CASE_INSENSITIVE`, as shown
in Listing 12-23.
-->

最後に、環境変数を確認する必要があります。環境変数を扱う関数は、標準ライブラリの`env`モジュールにあるので、
`use std::env;`行で*src/lib.rs*の冒頭でそのモジュールをスコープに持ってくる必要があります。そして、
`env`モジュールから`var`関数を使用して`CASE_INSENSITIVE`という環境変数のチェックを行います。
リスト 12-23 のようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
use std::env;
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<!--
<span class="caption">Listing 12-23: Checking for an environment variable named
`CASE_INSENSITIVE`</span>
-->

<span class="caption">リスト 12-23: `CASE_INSENSITIVE`という環境変数のチェックを行う</span>

<!--
Here, we create a new variable `case_sensitive`. To set its value, we call the
`env::var` function and pass it the name of the `CASE_INSENSITIVE` environment
variable. The `env::var` function returns a `Result` that will be the successful
`Ok` variant that contains the value of the environment variable if the
environment variable is set. It will return the `Err` variant if the
environment variable is not set.
-->

ここで、`case_sensitive`という新しい変数を生成しています。その値をセットするために、
`env::var`関数を呼び出し、`CASE_INSENSITIVE`環境変数の名前を渡しています。`env::var`関数は、
環境変数がセットされていたら、環境変数の値を含む`Ok`列挙子の成功値になる`Result`を返します。
環境変数がセットされていなければ、`Err`列挙子を返すでしょう。

<!--
We’re using the `is_err` method on the `Result` to check whether it’s an error
and therefore unset, which means it *should* do a case-sensitive search. If the
`CASE_INSENSITIVE` environment variable is set to anything, `is_err` will
return false and the program will perform a case-insensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_err` rather than `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.
-->

`Result`の`is_err`メソッドを使用して、エラーでありゆえに、セットされていないことを確認しています。
これは大文字小文字を区別する検索をす*べき*ことを意味します。`CASE_INSENSITIVE`環境変数が何かにセットされていれば、
`is_err`は false を返し、プログラムは大文字小文字を区別しない検索を実行するでしょう。環境変数の*値*はどうでもよく、
セットされているかどうかだけ気にするので、`unwrap`や`expect`あるいは、他のここまで見かけた`Result`のメソッドではなく、
`is_err`をチェックしています。

<!--
We pass the value in the `case_sensitive` variable to the `Config` instance so
the `run` function can read that value and decide whether to call `search` or
`search_case_insensitive`, as we implemented in Listing 12-22.
-->

`case_sensitive`変数の値を`Config`インスタンスに渡しているので、リスト 12-22 で実装したように、
`run`関数はその値を読み取り、`search`か`search_case_insensitive`を呼び出すか決定できるのです。

<!--
Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:
-->

試行してみましょう！まず、環境変数をセットせずにクエリは`to`でプログラムを実行し、
この時は全て小文字で"to"という言葉を含むあらゆる行が合致するはずです。

```text
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

<!--
Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to `1` but with the same query `to`.
-->

まだ機能しているようです！では、`CASE_INSENSITIVE`を 1 にしつつ、同じクエリの`to`でプログラムを実行しましょう。

<!--
If you’re using PowerShell, you will need to set the environment variable and
run the program in two commands rather than one:
-->

PowerShell を使用しているなら、1 コマンドではなく、2 コマンドで環境変数をセットし、プログラムを実行する必要があるでしょう：

```text
$ $env:CASE_INSENSITIVE=1
$ cargo run to poem.txt
```

<!--
We should get lines that contain “to” that might have uppercase letters:
-->

大文字も含む可能性のある"to"を含有する行が得られるはずです：

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
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
precedence. For another exercise on your own, try controlling case
insensitivity through either a command line argument or an environment
variable. Decide whether the command line argument or the environment variable
should take precedence if the program is run with one set to case sensitive and
one set to case insensitive.
-->

引数*と*環境変数で同じ設定を行うことができるプログラムもあります。そのような場合、
プログラムはどちらが優先されるか決定します。自身の別の鍛錬として、コマンドライン引数か、
環境変数で大文字小文字の区別を制御できるようにしてみてください。
片方は大文字小文字を区別するようにセットされ、もう片方は区別しないようにセットしてプログラムが実行された時に、
コマンドライン引数と環境変数のどちらの優先度が高くなるかを決めてください。

<!--
The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
-->

`std::env`モジュールは、環境変数を扱うもっと多くの有用な機能を有しています：
ドキュメンテーションを確認して、何が利用可能か確かめてください。
