<!--
## How to Write Tests
-->

## テストの記述法

<!--
Tests are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform these three
actions:
-->

テストは、テスト以外のコードが想定された方法で機能していることを実証するRustの関数です。
テスト関数の本体は、典型的には以下の3つの動作を行います:

<!--
1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.
-->

1. 必要なデータや状態をセットアップする。
2. テスト対象のコードを走らせる。
3. 結果が想定通りであることを断定（以下、アサーションという）する。

<!--
Let’s look at the features Rust provides specifically for writing tests that
take these actions, which include the `test` attribute, a few macros, and the
`should_panic` attribute.
-->

Rustが、特にこれらの動作を行うテストを書くために用意している機能を見ていきましょう。
これには、`test`属性、いくつかのマクロ、`should_panic`属性が含まれます。

<!--
### The Anatomy of a Test Function
-->

### テスト関数の構成

<!--
At its simplest, a test in Rust is a function that’s annotated with the `test`
attribute. Attributes are metadata about pieces of Rust code; one example is
the `derive` attribute we used with structs in Chapter 5. To change a function
into a test function, add `#[test]` on the line before `fn`. When you run your
tests with the `cargo test` command, Rust builds a test runner binary that runs
the annotated functions and reports on whether each
test function passes or fails.
-->
最も単純には、Rustにおけるテストは`test`属性で注釈された関数のことです。属性とは、
Rustコードの部品に関するメタデータです; 一例を挙げれば、構造体とともに第5章で使用した`derive`属性です。
関数をテスト関数に変えるには、`fn`の前に`#[test]`を付け加えてください。
`cargo test`コマンドでテストを実行したら、コンパイラは注釈された関数を走らせるテスト用バイナリをビルドし、
各テスト関数が通過したか失敗したかを報告します。

<!--
Whenever we make a new library project with Cargo, a test module with a test
function in it is automatically generated for us. This module gives you a
template for writing your tests so you don’t have to look up the exact
structure and syntax every time you start a new project. You can add as many
additional test functions and as many test modules as you want!
-->
新しいライブラリプロジェクトをCargoで作ると、テスト関数付きのテストモジュールが自動的に生成されます。
このモジュールがテストを書くためのテンプレートを提供してくれるので、
新しいプロジェクトを始めるたびに正しい構造とか文法をいちいち検索しなくてすみます。
ここに好きな数だけテスト関数やテストモジュールを追加すればいいというわけです！

<!--
We’ll explore some aspects of how tests work by experimenting with the template
test before we actually test any code. Then we’ll write some real-world tests
that call some code that we’ve written and assert that its behavior is correct.
-->

まずは実際にコードをテストする前に、自動生成されたテンプレートのテストで実験して、テストの動作の性質をいくらか学びましょう。
その後で、以前書いたコードを呼び出し、振る舞いが正しいことをアサーションする、ホンモノのテストを書きましょう。

<!--
Let’s create a new library project called `adder` that will add two numbers:
-->

2つの数を足す、`adder`という新しいライブラリプロジェクトを生成しましょう:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

<!--
The contents of the *src/lib.rs* file in your `adder` library should look like
Listing 11-1.
-->

`adder`ライブラリの*src/lib.rs*ファイルの中身は、リスト11-1のような見た目のはずです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
cargo test
git co output.txt
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-1: The test module and function generated
automatically by `cargo new`</span>
-->

<span class="caption">リスト11-1: `cargo new`で自動生成されたテストモジュールと関数</span>

<!--
For now, let’s ignore the top two lines and focus on the function. Note the
`#[test]` annotation: this attribute indicates this is a test function, so the
test runner knows to treat this function as a test. We might also have non-test
functions in the `tests` module to help set up common scenarios or perform
common operations, so we always need to indicate which functions are tests.
-->

とりあえず、最初の2行は無視し、関数に集中しましょう。
`#[test]`注釈に注目してください: この属性は、これがテスト関数であることを示すので、
テスト実行機はこの関数をテストとして扱うとわかるのです。さらに、`tests`モジュール内にはテスト関数以外の関数を入れ、
一般的なシナリオをセットアップしたり、共通の処理を行う手助けをしたりもできるので、
必ずどの関数がテストかを示す必要があるのです。

<!--
The example function body uses the `assert_eq!` macro to assert that `result`,
which contains the result of adding 2 and 2, equals 4. This assertion serves as
an example of the format for a typical test. Let’s run it to see that this test
passes.
-->

例の関数本体は、`assert_eq!`マクロを使用して、2と2を足した結果を含む`result`が、4に等しいことをアサーションしています。
このアサーションは、典型的なテストのフォーマット例をなしているわけです。走らせてこのテストが通る（訳注：テストが成功する、の意味。英語でpassということから、このように表現される）ことを確かめましょう。

<!--
The `cargo test` command runs all tests in our project, as shown in Listing
11-2.
-->

`cargo test`コマンドでプロジェクトにあるテストが全て実行されます。リスト11-2に示したようにですね。


```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<!--
<span class="caption">Listing 11-2: The output from running the automatically
generated test</span>
-->

<span class="caption">リスト11-2: 自動生成されたテストを走らせた出力</span>

<!--
Cargo compiled and ran the test. We see the line `running 1 test`. The next
line shows the name of the generated test function, called `it_works`, and that
the result of running that test is `ok`. The overall summary `test result: ok.`
means that all the tests passed, and the portion that reads `1 passed; 0
failed` totals the number of tests that passed or failed.
-->

Cargoがテストをコンパイルし、走らせました。`running 1 test`という行が見えます。
その次の行は、生成されたテスト関数の`it_works`という名前と、このテストの実行結果が`ok`であることを示しています。
テスト全体のまとめである`test result:ok.`は、全テストが通ったことを意味し、
`1 passed; 0 failed`と読める部分は、通過または失敗したテストの数を合計しているのです。

<!--
It’s possible to mark a test as ignored so it doesn’t run in a particular
instance; we’ll cover that in the [“Ignoring Some Tests Unless Specifically
Requested”][ignoring] section later in this chapter. Because we
haven’t done that here, the summary shows `0 ignored`. We can also pass an
argument to the `cargo test` command to run only tests whose name matches a
string; this is called *filtering* and we’ll cover that in the [“Running a
Subset of Tests by Name”][subset] section. We also haven’t
filtered the tests being run, so the end of the summary shows `0 filtered out`.
-->

特定の場合にテスト実行しないように、テストを無視するように指定することができます;
これについては後でこの章の[「特に要望のない限りテストを無視する」][ignoring]節で扱います。
ここではそれを行っていないので、まとめは`0 ignored`と示しています。
`cargo test` コマンドに引数を渡すことで、名前が文字列にマッチするテストのみを実行することもできます;
*フィルタリング*と呼ばれますが、これについては[「名前でテストの一部を実行する」][subset]節で扱います。
また、実行するテストにフィルタをかけもしなかったので、まとめの最後に`0 filtered out`と表示されています。

<!--
The `0 measured` statistic is for benchmark tests that measure performance.
Benchmark tests are, as of this writing, only available in nightly Rust. See
[the documentation about benchmark tests][bench] to learn more.
-->

`0 measured`という統計は、パフォーマンスを測定するベンチマークテスト用です。
ベンチマークテストは、本書記述の時点では、nightly版のRustでのみ利用可能です。
詳しくは、[ベンチマークテストのドキュメンテーション][bench]を参照されたし。

<!--
The next part of the test output starting at `Doc-tests adder` is for the
results of any documentation tests. We don’t have any documentation tests yet,
but Rust can compile any code examples that appear in our API documentation.
This feature helps keep your docs and your code in sync! We’ll discuss how to
write documentation tests in the [“Documentation Comments as
Tests”][doc-comments] section of Chapter 14. For now, we’ll
ignore the `Doc-tests` output.
-->

テスト出力の次の部分、つまり`Doc-tests adder`で始まる部分は、ドキュメンテーションテストの結果用のものです。
まだドキュメンテーションテストは何もないものの、コンパイラは、APIドキュメントに現れるどんなコード例もコンパイルできます。
この機能により、ドキュメントとコードを同期することができるわけです。ドキュメンテーションテストの書き方については、
第14章の[テストとしてのドキュメンテーションコメント][doc-comments]節で議論しましょう。今は、`Doc-tests`出力は無視します。

<!--
Let’s start to customize the test to our own needs. First change the name of
the `it_works` function to a different name, such as `exploration`, like so:
-->

それでは必要に応じてテストをカスタマイズしていきましょう。
まずは`it_works`関数の名前を違う名前に、例えば以下の`exploration`のように変更してください:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

<!--
Then run `cargo test` again. The output now shows `exploration` instead of
`it_works`:
-->

そして、`cargo test`を再度走らせます。これで出力が`it_works`の代わりに`exploration`と表示しています:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

<!--
Now we’ll add another test, but this time we’ll make a test that fails! Tests
fail when something in the test function panics. Each test is run in a new
thread, and when the main thread sees that a test thread has died, the test is
marked as failed. In Chapter 9, we talked about how the simplest way to panic
is to call the `panic!` macro. Enter the new test as a function named
`another`, so your *src/lib.rs* file looks like Listing 11-3.
-->

それでは別のテストを追加していきます。ただし、今回は失敗するテストにしましょう！
テスト関数内の何かがパニックすると、テストは失敗します。
各テストは、新規スレッドで実行され、メインスレッドが、テストスレッドが死んだと確認した時、テストは失敗と印づけられます。
第9章で、パニックを引き起こす最も単純な方法は`panic!`マクロを呼び出すことだと語りました。
*src/lib.rs*ファイルがリスト11-3のような見た目になるよう、`another`という名前の関数として新しいテストを入力してください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-3: Adding a second test that will fail because
we call the `panic!` macro</span>
-->

<span class="caption">リスト11-3: `panic!`マクロを呼び出したために失敗する2番目のテストを追加する</span>

<!--
Run the tests again using `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed.
-->

`cargo test`で再度テストを走らせてください。出力はリスト11-4のようになるはずであり、
`exploration`テストは通り、`another`は失敗したと表示されます。

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<!--
<span class="caption">Listing 11-4: Test results when one test passes and one
test fails</span>
-->

<span class="caption">リスト11-4: 一つのテストが通り、一つが失敗するときのテスト結果</span>

<!--
Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new
sections appear between the individual results and the summary: the first
displays the detailed reason for each test failure. In this case, we get the
details that `another` failed because it `panicked at 'Make this test fail'` on
line 10 in the *src/lib.rs* file. The next section lists just the names of all
the failing tests, which is useful when there are lots of tests and lots of
detailed failing test output. We can use the name of a failing test to run just
that test to more easily debug it; we’ll talk more about ways to run tests in
the [“Controlling How Tests Are Run”][controlling-how-tests-are-run]
section.
-->

`ok`の代わりに`test test::another`の行は、`FAILED`を表示しています。個々の結果とまとめの間に、
2つ新たな区域ができました: 最初の区域は、失敗したテスト各々の具体的な理由を表示しています。
今回の場合、`another`は*src/lib.rs*ファイルの10行目で`'Make this test fail'でパニックした`ために失敗した、という詳細が得られました。
次の区域は失敗したテストの名前だけを列挙しています。
これは、テストがたくさんあり、失敗したテストの詳細がたくさん表示されるときに有用になります。
失敗したテストの名前を使用してそのテストだけを実行し、より簡単にデバッグすることができます。
テストの実行方法については、[テストの実行され方を制御する][controlling-how-tests-are-run]節でもっと語りましょう。

<!--
The summary line displays at the end: overall, our test result is `FAILED`. We
had one test pass and one test fail.
-->

サマリー行が最後に出力されています: 総合的に言うと、テスト結果は`FAILED`でした。
一つのテストが通り、一つが失敗したわけです。

<!--
Now that you’ve seen what the test results look like in different scenarios,
let’s look at some macros other than `panic!` that are useful in tests.
-->

様々な状況でのテスト結果がどんな風になるか見てきたので、テストを行う際に有用になる`panic!`以外のマクロに目を向けましょう。

<!--
### Checking Results with the `assert!` Macro
-->

### `assert!`マクロで結果を確認する

<!--
The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a Boolean. If the value is
`true`, nothing happens and the test passes. If the value is `false`, the
`assert!` macro calls `panic!` to cause the test to fail. Using the `assert!`
macro helps us check that our code is functioning in the way we intend.
-->

`assert!`マクロは、標準ライブラリで提供されていますが、テスト内の何らかの条件が`true`と評価されることを確かめたいときに有効です。
`assert!`マクロには、論理値に評価される引数を与えます。その値が`true`なら、
何も起こらずにテストは通ります。その値が`false`なら、`assert!`マクロは`panic!`を呼び出し、
テストは失敗します。`assert!`マクロを使用することで、コードが意図した通りに機能していることを確認する助けになるわけです。

<!--
In Chapter 5, Listing 5-15, we used a `Rectangle` struct and a `can_hold`
method, which are repeated here in Listing 11-5. Let’s put this code in the
*src/lib.rs* file, then write some tests for it using the `assert!` macro.
-->

第5章のリスト5-15で、`Rectangle`構造体と`can_hold`メソッドを使用しました。リスト11-5でもそれを繰り返しています。
このコードを*src/lib.rs*ファイルに放り込み、`assert!`マクロでそれ用のテストを何か書いてみましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-5: Using the `Rectangle` struct and its
`can_hold` method from Chapter 5</span>
-->

<span class="caption">リスト11-5: 第5章から`Rectangle`構造体とその`can_hold`メソッドを使用する</span>

<!--
The `can_hold` method returns a Boolean, which means it’s a perfect use case
for the `assert!` macro. In Listing 11-6, we write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a width of 8 and
a height of 7 and asserting that it can hold another `Rectangle` instance that
has a width of 5 and a height of 1.
-->

`can_hold`メソッドは論理値を返すので、`assert!`マクロの完璧なユースケースになるわけです。
リスト11-6で、幅が8、高さが7の`Rectangle`インスタンスを生成し、これが幅5、
高さ1の別の`Rectangle`インスタンスを保持できるとアサーションすることで`can_hold`を用いるテストを書きます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-6: A test for `can_hold` that checks whether a
larger rectangle can indeed hold a smaller rectangle</span>
-->

<span class="caption">リスト11-6: より大きな長方形がより小さな長方形を確かに保持できるかを確認する`can_hold`用のテスト</span>

<!--
Note that we’ve added a new line inside the `tests` module: `use super::*;`.
The `tests` module is a regular module that follows the usual visibility rules
we covered in Chapter 7 in the [“Paths for Referring to an Item in the Module
Tree”][paths-for-referring-to-an-item-in-the-module-tree]
section. Because the `tests` module is an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We use
a glob here so anything we define in the outer module is available to this
`tests` module.
-->

`tests`モジュール内に新しい行を加えたことに注目してください: `use super::*`です。
`tests`モジュールは、第7章の[モジュールツリーの要素を示すためのパス][paths-for-referring-to-an-item-in-the-module-tree]節で講義した通常の公開ルールに従う普通のモジュールです。
`tests`モジュールは、内部モジュールなので、外部モジュール内のテスト配下にあるコードを内部モジュールのスコープに持っていく必要があります。
ここではglobを使用して、外部モジュールで定義したもの全てがこの`tests`モジュールでも使用可能になるようにしています。

<!--
We’ve named our test `larger_can_hold_smaller`, and we’ve created the two
`Rectangle` instances that we need. Then we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression is
supposed to return `true`, so our test should pass. Let’s find out!
-->

テストは`larger_can_hold_smaller`と名付け、必要な`Rectangle`インスタンスを2つ生成しています。
そして、`assert!`マクロを呼び出し、`larger.can_hold(&smaller)`の呼び出し結果を渡しました。
この式は、`true`を返すと考えられるので、テストは通るはずです。確かめましょう！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

<!--
It does pass! Let’s add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:
-->

通ります！別のテストを追加しましょう。今回は、小さい長方形は、より大きな長方形を保持できないことをアサーションします。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

<!--
Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. As a
result, our test will pass if `can_hold` returns `false`:
-->

今回の場合、`can_hold`関数の正しい結果は`false`なので、その結果を`assert!`マクロに渡す前に反転させる必要があります。
結果として、`can_hold`が`false`を返せば、テストは通ります。

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

<!--
Two tests that pass! Now let’s see what happens to our test results when we
introduce a bug in our code. We’ll change the implementation of the `can_hold`
method by replacing the greater-than sign with a less-than sign when it
compares the widths:
-->
通るテストが2つ！さて、コードにバグを導入したらテスト結果がどうなるか確認してみましょう。
幅を比較する大なり記号を小なり記号で置き換えて`can_hold`メソッドの実装を変更しましょう:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

<!--
Running the tests now produces the following:
-->
テストを実行すると、以下のような出力をします:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

<!--
Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is
5, the comparison of the widths in `can_hold` now returns `false`: 8 is not
less than 5.
-->

テストによりバグが捕捉されました！`larger.width`が8、`smaller.width`が5なので、
`can_hold`内の幅の比較が今は`false`を返すようになったのです: 8は5より小さくないですからね。

<!--
### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
-->

### `assert_eq!`と`assert_ne!`マクロで等値性をテストする

<!--
A common way to verify functionality is to test for equality between the result
of the code under test and the value you expect the code to return. You could
do this using the `assert!` macro and passing it an expression using the `==`
operator. However, this is such a common test that the standard library
provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test
more conveniently. These macros compare two arguments for equality or
inequality, respectively. They’ll also print the two values if the assertion
fails, which makes it easier to see *why* the test failed; conversely, the
`assert!` macro only indicates that it got a `false` value for the `==`
expression, without printing the values that led to the `false` value.
-->

機能性を検証する一般的な方法は、テスト下にあるコードの結果と、コードが返すと期待される値との、等値性を確かめることです。
これを`assert`マクロを使用して`==`演算子を使用した式を渡すことで行うこともできます。
しかしながら、これはありふれたテストなので、標準ライブラリには1組のマクロ(`assert_eq!`と`assert_ne!`)が提供され、
このテストをより便利に行うことができます。これらのマクロはそれぞれ、二つの引数を比べ、等しいかと等しくないかを確かめます。
また、アサーションが失敗したら二つの値の出力もし、テストが失敗した*原因*を確認しやすくなります。
一方で`assert!`マクロは、`==`式の値が`false`になったことしか示さず、`false`になった原因の値は出力しません。

<!--
In Listing 11-7, we write a function named `add_two` that adds `2` to its
parameter, then we test this function using the `assert_eq!` macro.
-->

リスト11-7では、引数に`2`を加える`add_two`という名前の関数を書いて、
この関数を`assert_eq!`マクロでテストしています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro</span>
-->

<span class="caption">リスト11-7: `assert_eq!`マクロで`add_two`関数をテストする</span>

<!--
Let’s check that it passes!
-->

これが通ることを確認しましょう！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

<!--
We pass `4` as the argument to `assert_eq!`, which is equal to the result of
calling `add_two(2)`. The line for this test is `test tests::it_adds_two ...
ok`, and the `ok` text indicates that our test passed!
-->

`assert_eq!`マクロに引数として`4`を渡していますが、これは`add_two(2)`の呼び出し結果と等しいです。
このテストの行は`test tests::it_adds_two ... ok`であり、`ok`というテキストはテストが通ったことを示しています！

<!--
Let’s introduce a bug into our code to see what `assert_eq!` looks like when it
fails. Change the implementation of the `add_two` function to instead add `3`:
-->

コードにバグを仕込んで、`assert_eq!`が失敗した時にそれがどうなるのか確認してみましょう。
`add_two`関数の実装を代わりに`3`を足すように変えてください:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

<!--
Run the tests again:
-->
テストを再度実行します:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

<!--
Our test caught the bug! The `it_adds_two` test failed, and the message tells
us that the assertion that fails was `` assertion failed: `(left == right)` ``
and what the `left` and `right` values are. This message helps us start
debugging: the `left` argument was `4` but the `right` argument, where we had
`add_two(2)`, was `5`. You can imagine that this would be especially helpful
when we have a lot of tests going on.
-->

テストがバグを捕捉しました！
`it_adds_two`のテストは失敗し、そのメッセージは、
失敗したアサーションが`` assertion failed: `(left == right)` ``であったこと、
そして`left`と`right`の値が何だったかを示しています。
このメッセージはデバッグを開始する助けになります:
`left`引数は`4`だったが、`add_two(2)`がある`right`引数は`5`でした。
実行中のテストが多数あるときは特に、この出力は役に立つだろうと想像できるでしょう。

<!--
Note that in some languages and test frameworks, the parameters to equality
assertion functions are called `expected` and `actual`, and the order in which
we specify the arguments matters. However, in Rust, they’re called `left` and
`right`, and the order in which we specify the value we expect and the value
the code produces doesn’t matter. We could write the assertion in this test as
`assert_eq!(add_two(2), 4)`, which would result in the same failure message
that displays `` assertion failed: `(left == right)` ``.
-->

言語やテストフレームワークによっては、等値性アサーション関数の引数を`expected`と`actual`と呼び、
引数を指定する順序が重要であることに注意してください。
ですがRustでは、これらは`left`と`right`と呼ばれ、期待する値とコードが生成する値を指定する順序は重要ではありません。
今回のテストのアサーションを`assert_eq!(add_two(2), 4)`と書くこともでき、
そうすると失敗メッセージは、同じく`` assertion failed: `(left == right)` ``を表示するでしょう。

<!--
The `assert_ne!` macro will pass if the two values we give it are not equal and
fail if they’re equal. This macro is most useful for cases when we’re not sure
what a value *will* be, but we know what the value definitely *shouldn’t* be.
For example, if we’re testing a function that is guaranteed to change its input
in some way, but the way in which the input is changed depends on the day of
the week that we run our tests, the best thing to assert might be that the
output of the function is not equal to the input.
-->

`assert_ne!`マクロは、与えた2つの値が等しくなければ通り、等しければ失敗します。
このマクロは、値が何になる*だろう*か確信が持てないけれども、
確実にこの値にはなる*べきでない*とわかっているような場合に最も有用になります。例えば、
入力を何らかの手段で変え（て出力す）ることが保証されているけれども、入力の変え方がテストを実行する曜日に依存する関数をテストしているなら、
アサーションすべき最善の事柄は、関数の出力が入力と等しくないことかもしれません。

<!--
Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All primitive types and most of
the standard library types implement these traits. For structs and enums that
you define yourself, you’ll need to implement `PartialEq` to assert equality of
those types. You’ll also need to implement `Debug` to print the values when the
assertion fails. Because both traits are derivable traits, as mentioned in
Listing 5-12 in Chapter 5, this is usually as straightforward as adding the
`#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See
Appendix C, [“Derivable Traits,”][derivable-traits] for more
details about these and other derivable traits.
-->

内部的には、`assert_eq!`と`assert_ne!`マクロは、それぞれ`==`と`!=`演算子を使用しています。
アサーションが失敗すると、これらのマクロは引数をデバッグフォーマットを使用してプリントするので、
比較対象の値は`PartialEq`と`Debug`トレイトを実装していなければなりません。
すべての組み込み型と、ほぼすべての標準ライブラリの型はこれらのトレイトを実装しています。
自分で定義した構造体やenumについては、
その型の値の等値性をアサーションするために、`PartialEq`を実装する必要があるでしょう。
それが失敗した時にその値をプリントできるように、`Debug`も実装する必要もあるでしょう。
第5章のリスト5-12で触れたように、どちらのトレイトも導出可能なトレイトなので、
これは通常、単純に構造体やenum定義に`#[derive(PartialEq, Debug)]`という注釈を追加するだけですみます。
これらやその他の導出可能なトレイトに関する詳細については、付録C、[導出可能なトレイト][derivable-traits]をご覧ください。

<!--
### Adding Custom Failure Messages
-->
### カスタムの失敗メッセージを追加する

<!--
You can also add a custom message to be printed with the failure message as
optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any
arguments specified after the required arguments are passed along to the
`format!` macro (discussed in Chapter 8 in the [“Concatenation with the `+`
Operator or the `format!`
Macro”][concatenation-with-the--operator-or-the-format-macro]
section), so you can pass a format string that contains `{}` placeholders and
values to go in those placeholders. Custom messages are useful for documenting
what an assertion means; when a test fails, you’ll have a better idea of what
the problem is with the code.
-->
さらに、`assert!`、`assert_eq!`、`assert_ne!`の追加引数として、失敗メッセージと共にカスタムのメッセージが表示されるよう、
追加することもできます。必須引数の後に指定された引数はすべて`format!`マクロに渡されるので、
（format!マクロについては第8章の[`+`演算子、または`format!`マクロで連結][concatenation-with-the--operator-or-the-format-macro]節で議論しました）、
`{}`プレースホルダーを含むフォーマット文字列とこのプレースホルダーに置き換えられる値を渡すことができます。
カスタムメッセージは、アサーションがどんな意味を持つかドキュメント化するのに役に立ちます;
もしテストが失敗した時、コードにどんな問題があるのかをよりしっかり把握できるはずです。

<!--
For example, let’s say we have a function that greets people by name and we
want to test that the name we pass into the function appears in the output:
-->
例として、人々に名前で挨拶をする関数があり、関数に渡した名前が出力に出現することをテストしたいとしましょう:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

<!--
The requirements for this program haven’t been agreed upon yet, and we’re
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don’t want to have to update the test when the requirements change,
so instead of checking for exact equality to the value returned from the
`greeting` function, we’ll just assert that the output contains the text of the
input parameter.
-->

このプログラムの要件はまだ取り決められておらず、挨拶の先頭の`Hello`というテキストはおそらく変わります。
要件が変わった時にテストを更新しなくてもよいようにしたいと考え、
`greeting`関数から返る値と正確な等値性を確認するのではなく、出力が入力引数のテキストを含むことをアサーションするだけにします。

<!--
Now let’s introduce a bug into this code by changing `greeting` to exclude
`name` to see what the default test failure looks like:
-->

それでは`greeting`が`name`を含まないように変更してこのコードにバグを仕込み、テストの失敗がデフォルトでどんな風になるのか確かめましょう:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

<!--
Running this test produces the following:
-->

このテストを実行すると、以下のように出力されます:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

<!--
This result just indicates that the assertion failed and which line the
assertion is on. A more useful failure message would print the value from the
`greeting` function. Let’s add a custom failure message composed of a format
string with a placeholder filled in with the actual value we got from the
`greeting` function:
-->

この結果は、アサーションが失敗し、どの行にアサーションがあるかを示しているだけです。
失敗メッセージが`greeting`関数からの値を出力していればより有用でしょう。
`greeting`関数から得た実際の値で埋められるプレースホルダーを含むフォーマット文字列からなるカスタムの失敗メッセージを追加してみましょう:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

<!--
Now when we run the test, we’ll get a more informative error message:
-->

これでテストを実行したら、より有益なエラーメッセージが得られるでしょう:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

<!--
We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.
-->

実際に得られた値がテスト出力に表示されているので、起こると想定していたものではなく、
起こったものをデバッグするのに役に立ちます。

<!--
### Checking for Panics with `should_panic`
-->

### `should_panic`でパニックを確認する

<!--
In addition to checking return values, it’s important to check that our code
handles error conditions as we expect. For example, consider the `Guess` type
that we created in Chapter 9, Listing 9-13. Other code that uses `Guess`
depends on the guarantee that `Guess` instances will contain only values
between 1 and 100. We can write a test that ensures that attempting to create a
`Guess` instance with a value outside that range panics.
-->

戻り値を確認することに加えて、想定通りにコードがエラー状態を扱っていることを確認することが重要です。
例えば、第9章のリスト9-10で生成した`Guess`型を考えてください。`Guess`を使用する他のコードは、
`Guess`のインスタンスは1から100の範囲の値しか含まないという保証に依存しています。
その範囲外の値で`Guess`インスタンスを生成しようとするとパニックすることを確認するテストを書くことができます。

<!--
We do this by adding the attribute `should_panic` to our test function. The
test passes if the code inside the function panics; the test fails if the code
inside the function doesn’t panic.
-->

これは、テスト関数に`should_panic`という属性を追加することで達成できます。
このテストは、関数内のコードがパニックする場合に通過します。つまり、
関数内のコードがパニックしなかったら、テストは失敗するわけです。

<!--
Listing 11-8 shows a test that checks that the error conditions of `Guess::new`
happen when we expect them to.
-->

リスト11-8は、予想どおりに`Guess::new`のエラー条件が発生していることを確認するテストを示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!`</span>
-->

<span class="caption">リスト11-8: 状況が`panic!`を引き起こすとテストする</span>

<!--
We place the `#[should_panic]` attribute after the `#[test]` attribute and
before the test function it applies to. Let’s look at the result when this test
passes:
-->

`#[test]`属性の後、適用するテスト関数の前に`#[should_panic]`属性を配置しています。
このテストが通るときの結果を見ましょう:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

<!--
Looks good! Now let’s introduce a bug in our code by removing the condition
that the `new` function will panic if the value is greater than 100:
-->
よさそうですね！では、値が100より大きいときに`new`関数がパニックするという条件を除去することでコードにバグを導入しましょう:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

<!--
When we run the test in Listing 11-8, it will fail:
-->

リスト11-8のテストを実行すると、失敗するでしょう:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

<!--
We don’t get a very helpful message in this case, but when we look at the test
function, we see that it’s annotated with `#[should_panic]`. The failure we got
means that the code in the test function did not cause a panic.
-->

この場合、それほど役に立つメッセージは得られませんが、テスト関数に目を向ければ、
`#[should_panic]`で注釈されていることがわかります。得られた失敗は、
テスト関数のコードがパニックを引き起こさなかったことを意味するのです。

<!--
Tests that use `should_panic` can be imprecise. A `should_panic` test would
pass even if the test panics for a different reason from the one we were
expecting. To make `should_panic` tests more precise, we can add an optional
`expected` parameter to the `should_panic` attribute. The test harness will
make sure that the failure message contains the provided text. For example,
consider the modified code for `Guess` in Listing 11-9 where the `new` function
panics with different messages depending on whether the value is too small or
too large.
-->

`should_panic`を使用するテストは不正確なこともあります。
`should_panic`のテストは、想定していたもの以外の理由でテストがパニックしても通ってしまうのです。
`should_panic`のテストの正確を期すために、`should_panic`属性に`expected`引数を追加することもできます。
このテストハーネスは、失敗メッセージに与えられたテキストが含まれていることを確かめてくれます。
例えば、リスト11-9の修正された`Guess`のコードを考えてください。ここでは、
`new`関数は、値が大きすぎるか小さすぎるかによって異なるメッセージでパニックします。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 11-9: Testing for a `panic!` with a panic message
containing a specified substring</span>
-->

<span class="caption">リスト11-9: 指定された部分文字列を含むパニックメッセージで`panic!`することをテストする</span>

<!--
This test will pass because the value we put in the `should_panic` attribute’s
`expected` parameter is a substring of the message that the `Guess::new`
function panics with. We could have specified the entire panic message that we
expect, which in this case would be `Guess value must be less than or equal to
100, got 200.` What you choose to specify depends on how much of the panic
message is unique or dynamic and how precise you want your test to be. In this
case, a substring of the panic message is enough to ensure that the code in the
test function executes the `else if value > 100` case.
-->

`should_panic`属性の`expected`引数に置いた値が`Guess::new`関数がパニックしたメッセージの一部になっているので、
このテストは通ります。予想されるパニックメッセージ全体を指定することもでき、今回の場合、
`Guess value must be less than or equal to 100, got 200.`となります。
何を指定するかは、パニックメッセージのどこが固有でどこが動的か、
またテストをどの程度正確に行いたいかによります。今回の場合、パニックメッセージの一部でも、テスト関数内のコードが、
`else if value > 100`の場合を実行していると確認するのに事足りるのです。

<!--
To see what happens when a `should_panic` test with an `expected` message
fails, let’s again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:
-->
`expected`メッセージありの`should_panic`テストが失敗すると何が起きるのが確かめるために、
`if value < 1`と`else if value > 100`ブロックの本体を入れ替えることで再度コードにバグを仕込みましょう:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

<!--
This time when we run the `should_panic` test, it will fail:
-->

`should_panic`テストを実行すると、今回は失敗するでしょう:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

<!--
The failure message indicates that this test did indeed panic as we expected,
but the panic message did not include the expected string `'Guess value must be
less than or equal to 100'`. The panic message that we did get in this case was
`Guess value must be greater than or equal to 1, got 200.` Now we can start
figuring out where our bug is!
-->

この失敗メッセージは、このテストが確かに予想通りパニックしたことを示していますが、
パニックメッセージは、予想される文字列の`'Guess value must be less than or equal to 100'`を含んでいませんでした。
実際に得られたパニックメッセージは今回の場合、`Guess value must be greater than or equal to 1, got 200.`でした。
そうしてバグの所在地を割り出し始めることができるわけです！

<!--
### Using `Result<T, E>` in Tests
-->
### `Result<T, E>`をテストで使う

<!--
Our tests so far all panic when they fail. We can also write tests that use
`Result<T, E>`! Here’s the test from Listing 11-1, rewritten to use `Result<T,
E>` and return an `Err` instead of panicking:
-->
これまで書いてきたテストは失敗するとパニックしていましたが、
`Result<T, E>`を使うようなテストを書くこともできます！
以下は、リスト11-1のテストを、`Result<T, E>`を使い、パニックする代わりに`Err`を返すように書き直したものです：

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

<!--
The `it_works` function now has the `Result<(), String>` return type. In the
body of the function, rather than calling the `assert_eq!` macro, we return
`Ok(())` when the test passes and an `Err` with a `String` inside when the test
fails.
-->
`it_works`関数の戻り値の型は`Result<(), String>`になりました。
関数内で`assert_eq!`マクロを呼び出す代わりに、テストが成功すれば`Ok(())`を、失敗すれば`Err`に`String`を入れて返すようにします。

<!--
Writing tests so they return a `Result<T, E>` enables you to use the question
mark operator in the body of tests, which can be a convenient way to write
tests that should fail if any operation within them returns an `Err` variant.
-->
`Result<T, E>` を返すようなテストを書くと、`?`演算子をテストの中で使えるようになります。
これは、テスト内で何らかの工程が`Err`ヴァリアントを返したときに失敗するべきテストを書くのに便利です。

<!--
You can’t use the `#[should_panic]` annotation on tests that use `Result<T,
E>`. To assert that an operation returns an `Err` variant, *don’t* use the
question mark operator on the `Result<T, E>` value. Instead, use
`assert!(value.is_err())`.
-->
`Result<T, E>`を使うテストに`#[should_panic]`注釈を使うことはできません。
操作が`Err`列挙子を返すことをアサーションするためには、`Result<T, E>`値に対して`?`演算子を使用*しないでください*。
代わりに、`assert!(value.is_err())`を使用してください。

<!--
Now that you know several ways to write tests, let’s look at what is happening
when we run our tests and explore the different options we can use with `cargo
test`.
-->

今やテスト記法を複数知ったので、テストを走らせる際に起きていることに目を向け、
`cargo test`で使用できるいろんなオプションを探究しましょう。

<!--
[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
-->

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#演算子またはformatマクロで連結
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#特に要望のない限りテストを無視する
[subset]: ch11-02-running-tests.html#名前でテストの一部を実行する
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#テストの実行のされ方を制御する
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#テストとしてのドキュメンテーションコメント
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
