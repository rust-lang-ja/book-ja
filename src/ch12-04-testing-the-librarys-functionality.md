<!--
## Developing the Library’s Functionality with Test-Driven Development
-->

## テスト駆動開発でライブラリの機能を開発する

<!--
Now that we’ve extracted the logic into *src/lib.rs* and left the argument
collecting and error handling in *src/main.rs*, it’s much easier to write tests
for the core functionality of our code. We can call functions directly with
various arguments and check return values without having to call our binary
from the command line.
-->

今や、ロジックを*src/lib.rs*に抜き出し、引数集めとエラー処理を*src/main.rs*に残したので、
コードの核となる機能のテストを書くのが非常に容易になりました。いろんな引数で関数を直接呼び出し、
コマンドラインからバイナリを呼び出す必要なく戻り値を確認できます。

<!--
In this section, we’ll add the searching logic to the `minigrep` program
using the test-driven development (TDD) process with the following steps:
-->

この節では、以下の手順に従ってテスト駆動開発(TDD)プロセスを活用して、`minigrep`プログラムに検索ロジックを追加します:

<!--
1. Write a test that fails and run it to make sure it fails for the reason you
   expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests
   continue to pass.
4. Repeat from step 1!
-->

1. 失敗するテストを書き、走らせて想定通りの理由で失敗することを確かめる。
2. 十分な量のコードを書くか変更して新しいテストを通過するようにする。
3. 追加または変更したばかりのコードをリファクタリングし、テストが通り続けることを確認する。
4. 手順1から繰り返す！

<!--
Though it’s just one of many ways to write software, TDD can help drive code
design. Writing the test before you write the code that makes the test pass
helps to maintain high test coverage throughout the process.
-->

TDDはソフトウェアを書く多くの方法のうちの一つに過ぎませんが、コードデザインを駆動するために役立てることができます。
テストを通過させるコードを書く前にテストを書くことで、過程を通して高いテストカバー率を保つ助けになります。

<!--
We’ll test drive the implementation of the functionality that will actually do
the searching for the query string in the file contents and produce a list of
lines that match the query. We’ll add this functionality in a function called
`search`.
-->

実際にクエリ文字列の検索を行う機能の実装をテスト駆動し、クエリに合致する行のリストを生成します。
この機能を`search`という関数に追加しましょう。

<!--
### Writing a Failing Test
-->

### 失敗するテストを記述する

<!--
Because we don’t need them anymore, let’s remove the `println!` statements from
*src/lib.rs* and *src/main.rs* that we used to check the program’s behavior.
Then, in *src/lib.rs*, add a `tests` module with a test function, as we did in
[Chapter 11][ch11-anatomy]. The test function specifies the
behavior we want the `search` function to have: it will take a query and the
text to search, and it will return only the lines from the text that contain
the query. Listing 12-15 shows this test, which won’t compile yet.
-->

もう必要ないので、プログラムの振る舞いを確認していた`println!`文を*src/lib.rs*と*src/main.rs*から削除しましょう。
それから*src/lib.rs*で、テスト関数のある`tests`モジュールを追加してください。[第11章][ch11-anatomy]のようにですね。
このテスト関数が`search`関数に欲しい振る舞いを指定します: クエリと検索対象のテキストを受け取り、
クエリを含む行だけをテキストから返します。リスト12-15にこのテストを示していますが、まだコンパイルは通りません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-15: Creating a failing test for the `search`
function we wish we had</span>
-->

<span class="caption">リスト12-15: こうだったらいいなという`search`関数の失敗するテストを作成する</span>

<!--
This test searches for the string `"duct"`. The text we’re searching is three
lines, only one of which contains `"duct"` (Note that the backslash after the
opening double quote tells Rust not to put a newline character at the beginning
of the contents of this string literal). We assert that the value returned from
the `search` function contains only the line we expect.
-->

このテストは、`"duct"`という文字列を検索します。検索対象の文字列は3行で、うち1行だけが`"duct"`を含みます
(開き二重引用符の後のバックスラッシュは、この文字列リテラルの内容の先頭に改行文字を置かないように、
コンパイラに指示しているということに注意してください)。
`search`関数から返る値が想定している行だけを含むことをアサーションします。

<!--
We aren’t yet able to run this test and watch it fail because the test doesn’t
even compile: the `search` function doesn’t exist yet! In accordance with TDD
principles, we’ll add just enough code to get the test to compile and run by
adding a definition of the `search` function that always returns an empty
vector, as shown in Listing 12-16. Then the test should compile and fail
because an empty vector doesn’t match a vector containing the line `"safe,
fast, productive."`
-->

このテストを走らせ、失敗するところを観察することは、まだできません。このテストはコンパイルもできないからです:
まだ`search`関数が存在していません！TDDの原則に従って、空のベクタを常に返す`search`関数の定義を追加することで、
テストをコンパイルし走らせるだけのコードを追記します。リスト12-16に示したようにですね。そうすれば、
テストはコンパイルでき、失敗するはずです。なぜなら、空のベクタは、
`"safe, fast, productive."`という行を含むベクタとは合致しないからです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-16: Defining just enough of the `search`
function so our test will compile</span>
-->

<span class="caption">リスト12-16: テストがコンパイルできるのに十分なだけ`search`関数を定義する</span>

<!--
3行目後半、which argument lifetimeをwhich argument's lifetimeの形で訳している。記述ミス？
-->

<!--
Notice that we need to define an explicit lifetime `'a` in the signature of
`search` and use that lifetime with the `contents` argument and the return
value. Recall in [Chapter 10][ch10-lifetimes] that the lifetime
parameters specify which argument lifetime is connected to the lifetime of the
return value. In this case, we indicate that the returned vector should contain
string slices that reference slices of the argument `contents` (rather than the
argument `query`).
-->

`search`のシグニチャ内で、明示的なライフタイム`'a`を定義し、そのライフタイムを`contents`引数と戻り値で使用していることに注目してください。
[第10章][ch10-lifetimes]からライフタイム仮引数は、どの実引数のライフタイムが戻り値のライフタイムに関連づけられているかを指定することを思い出してください。
この場合、返却されるベクタは、
(`query`引数ではなく)`contents`引数のスライスを参照する文字列スライスを含むべきと示唆しています。

<!--
In other words, we tell Rust that the data returned by the `search` function
will live as long as the data passed into the `search` function in the
`contents` argument. This is important! The data referenced *by* a slice needs
to be valid for the reference to be valid; if the compiler assumes we’re making
string slices of `query` rather than `contents`, it will do its safety checking
incorrectly.
-->

言い換えると、コンパイラに`search`関数に返されるデータは、
`search`関数に`contents`引数で渡されているデータと同期間生きることを教えています。
これは重要なことです！スライス*に*参照されるデータは、参照が有効になるために有効である必要があるのです;
コンパイラが`contents`ではなく`query`の文字列スライスを生成すると想定してしまったら、
安全性チェックを間違って行うことになってしまいます。

<!--
If we forget the lifetime annotations and try to compile this function, we’ll
get this error:
-->

ライフタイム注釈を忘れてこの関数をコンパイルしようとすると、こんなエラーが出ます:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

<!--
Rust can’t possibly know which of the two arguments we need, so we need to tell
it explicitly. Because `contents` is the argument that contains all of our text
and we want to return the parts of that text that match, we know `contents` is
the argument that should be connected to the return value using the lifetime
syntax.
-->

コンパイラには、二つの引数のどちらが必要なのか知る由がないので、明示的に教えてあげる必要があるのです。
`contents`がテキストを全て含む引数で、合致するそのテキストの一部を返したいので、
`contents`がライフタイム記法で戻り値に関連づくはずの引数であることをプログラマは知っています。

<!--
Other programming languages don’t require you to connect arguments to return
values in the signature, but this practice will get easier over time. You might
want to compare this example with the [“Validating References with
Lifetimes”][validating-references-with-lifetimes] section in
Chapter 10.
-->

他のプログラミング言語では、シグニチャで引数と戻り値を関連づける必要はありませんが、この実践は時間とともに楽になっていくでしょう。
この例を第10章の[「ライフタイムで参照を検証する」][validating-references-with-lifetimes]節と比較してみるといいかもしれません。

<!--
Now let’s run the test:
-->

さあ、テストを実行しましょう:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

<!--
Great, the test fails, exactly as we expected. Let’s get the test to pass!
-->

素晴らしい。テストは全く想定通りに失敗しています。テストが通るようにしましょう！

<!--
### Writing Code to Pass the Test
-->

### テストを通過させるコードを書く

<!--
Currently, our test is failing because we always return an empty vector. To fix
that and implement `search`, our program needs to follow these steps:
-->

空のベクタを常に返しているために、現状テストは失敗しています。それを修正し、`search`を実装するには、
プログラムは以下の手順に従う必要があります:

<!--
* Iterate through each line of the contents.
* Check whether the line contains our query string.
* If it does, add it to the list of values we’re returning.
* If it doesn’t, do nothing.
* Return the list of results that match.
-->

* 中身を各行ごとに繰り返す。
* 行にクエリ文字列が含まれるか確認する。
* するなら、それを返却する値のリストに追加する。
* しないなら、何もしない。
* 一致する結果のリストを返す。

<!--
Let’s work through each step, starting with iterating through lines.
-->

各行を繰り返す作業から、この手順に順に取り掛かりましょう。

<!--
#### Iterating Through Lines with the `lines` Method
-->

#### `lines`メソッドで各行を繰り返す

<!--
Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works as shown in Listing 12-17. Note this
won’t compile yet.
-->

Rustには、文字列を行ごとに繰り返す役立つメソッドがあり、利便性のために`lines`と名付けられ、
リスト12-17のように動作します。まだ、これはコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-17: Iterating through each line in `contents`
</span>
-->

<span class="caption">リスト12-17: `contents`の各行を繰り返す</span>

<!--
The `lines` method returns an iterator. We’ll talk about iterators in depth in
[Chapter 13][ch13-iterators], but recall that you saw this way
of using an iterator in [Listing 3-5][ch3-iter], where we used a
`for` loop with an iterator to run some code on each item in a collection.
-->

`lines`メソッドはイテレータを返します。イテレータについて詳しくは、[第13章][ch13-iterators]で話しますが、
[リスト3-5][ch3-iter]でこのようなイテレータの使用法は見かけたことを思い出してください。
そこでは、イテレータに`for`ループを使用してコレクションの各要素に対して何らかのコードを走らせていました。

<!--
#### Searching Each Line for the Query
-->

#### クエリを求めて各行を検索する

<!--
Next, we’ll check whether the current line contains our query string.
Fortunately, strings have a helpful method named `contains` that does this for
us! Add a call to the `contains` method in the `search` function, as shown in
Listing 12-18. Note this still won’t compile yet.
-->

次に現在の行がクエリ文字列を含むか確認します。幸運なことに、
文字列にはこれを行ってくれる`contains`という役に立つメソッドがあります！`search`関数に、
`contains`メソッドの呼び出しを追加してください。リスト12-18のようにですね。
それでもまだコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-18: Adding functionality to see whether the
line contains the string in `query`</span>
-->

<span class="caption">リスト12-18: 行が`query`の文字列を含むか確認する機能を追加する</span>

<!--
At the moment, we’re building up functionality. To get it to compile, we need
to return a value from the body as we indicated we would in the function
signature.
-->

ここまでで機能を組み上げてきました。これをコンパイルできるようにするためには、
関数のシグネチャでそうすると示したように、本体から値を返す必要があります。

<!--
#### Storing Matching Lines
-->

#### 合致した行を保存する

<!--
To finish this function, we need a way to store the matching lines that we want
to return. For that, we can make a mutable vector before the `for` loop and
call the `push` method to store a `line` in the vector. After the `for` loop,
we return the vector, as shown in Listing 12-19.
-->

この関数を完成させるには、返そうとしている、合致した行を保存する方法が必要です。そのために、`for`ループの前に可変なベクタを生成し、
`push`メソッドを呼び出して`line`をベクタに保存することができます。`for`ループの後でベクタを返却します。
リスト12-19のようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-19: Storing the lines that match so we can
return them</span>
-->

<span class="caption">リスト12-19: 合致する行を保存したので、返すことができる</span>

<!--
Now the `search` function should return only the lines that contain `query`,
and our test should pass. Let’s run the test:
-->

これで`search`関数は、`query`を含む行だけを返すはずであり、テストも通るはずです。
テストを実行しましょう:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

<!--
Our test passed, so we know it works!
-->

テストが通り、動いていることがわかりました！

<!--
At this point, we could consider opportunities for refactoring the
implementation of the search function while keeping the tests passing to
maintain the same functionality. The code in the search function isn’t too bad,
but it doesn’t take advantage of some useful features of iterators. We’ll
return to this example in [Chapter 13][ch13-iterators], where
we’ll explore iterators in detail, and look at how to improve it.
-->

ここで、テストが通過するよう保ったまま、同じ機能を保持しながら、検索関数の実装をリファクタリングする機会を考えることもできます。
検索関数のコードは悪すぎるわけではありませんが、イテレータの有用な機能の一部を活用していません。
この例には[第13章][ch13-iterators]で再度触れ、そこでは、イテレータをより深く探究し、さらに改善する方法に目を向けます。

<!--
#### Using the `search` Function in the `run` Function
-->

#### `run`関数内で`search`関数を使用する

<!--
Now that the `search` function is working and tested, we need to call `search`
from our `run` function. We need to pass the `config.query` value and the
`contents` that `run` reads from the file to the `search` function. Then `run`
will print each line returned from `search`:
-->

`search`関数が動きテストできたので、`run`関数から`search`を呼び出す必要があります。`config.query`の値と、
ファイルから`run`が読み込む`contents`の値を`search`関数に渡す必要があります。
それから`run`は、`search`から返ってきた各行を出力するでしょう:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

<!--
We’re still using a `for` loop to return each line from `search` and print it.
-->

それでも`for`ループで`search`から各行を返し、出力しています。

<!--
Now the entire program should work! Let’s try it out, first with a word that
should return exactly one line from the Emily Dickinson poem, “frog”:
-->

さて、プログラム全体が動くはずです！試してみましょう。まずはエミリー・ディキンソンの詩から、
ちょうど1行だけを返すはずの言葉から。"frog"です:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

<!--
Cool! Now let’s try a word that will match multiple lines, like “body”:
-->

かっこいい！今度は、複数行にマッチするであろう言葉を試しましょう。"body"とかね:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

<!--
And finally, let’s make sure that we don’t get any lines when we search for a
word that isn’t anywhere in the poem, such as “monomorphization”:
-->

そして最後に、詩のどこにも現れない単語を探したときに、何も出力がないことを確かめましょう。
"monomorphization"などね:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

<!--
Excellent! We’ve built our own mini version of a classic tool and learned a lot
about how to structure applications. We’ve also learned a bit about file input
and output, lifetimes, testing, and command line parsing.
-->

最高です！古典的なツールの独自のミニバージョンを構築し、アプリケーションを構造化する方法を多く学びました。
また、ファイル入出力、ライフタイム、テスト、コマンドライン引数の解析についても、少し学びました。

<!--
To round out this project, we’ll briefly demonstrate how to work with
environment variables and how to print to standard error, both of which are
useful when you’re writing command line programs.
-->

このプロジェクトをまとめ上げるために、環境変数を扱う方法と標準エラー出力に出力する方法を少しだけデモします。
これらはどちらも、コマンドラインプログラムを書く際に有用です。

<!--
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
-->

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#ライフタイムで参照を検証する
[ch11-anatomy]: ch11-01-writing-tests.html#テスト関数の構成
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#forでコレクションを覗き見る
[ch13-iterators]: ch13-02-iterators.html
