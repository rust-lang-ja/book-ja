<!--
## Controlling How Tests Are Run
-->

## テストの実行のされ方を制御する

<!--
Just as `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. The default behavior of the binary produced by `cargo test` is to run
all the tests in parallel and capture output generated during test runs,
preventing the output from being displayed and making it easier to read the
output related to the test results. You can, however, specify command line
options to change this default behavior.
-->

`cargo run`がコードをコンパイルし、出来上がったバイナリを走らせるのと全く同様に、
`cargo test`はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行します。
`cargo test`で生成されるバイナリの既定動作は、テストを全て並行に実行し、
テスト実行中に生成された出力をキャプチャして出力が表示されるのを防ぎ、
テスト結果に関係する出力を読みやすくすることです。
ですが、コマンドラインオプションを指定してこの既定動作を変更することができます。

<!--
Some command line options go to `cargo test`, and some go to the resulting test
binary. To separate these two types of arguments, you list the arguments that
go to `cargo test` followed by the separator `--` and then the ones that go to
the test binary. Running `cargo test --help` displays the options you can use
with `cargo test`, and running `cargo test -- --help` displays the options you
can use after the separator.
-->

コマンドラインオプションの中には`cargo test`にかかるものや、出来上がったテストバイナリにかかるものがあります。
この2種の引数を区別するために、`cargo test`にかかる引数を`--`という区分記号の後に列挙し、
それからテストバイナリにかかる引数を列挙します。`cargo test --help`を走らせると、`cargo test`で使用できるオプションが表示され、
`cargo test -- --help`を走らせると、区分記号の後に使えるオプションが表示されます。

<!--
### Running Tests in Parallel or Consecutively
-->

### テストを並行または連続して実行する

<!--
When you run multiple tests, by default they run in parallel using threads,
meaning they finish running faster and you get feedback quicker. Because the
tests are running at the same time, you must make sure your tests don’t depend
on each other or on any shared state, including a shared environment, such as
the current working directory or environment variables.
-->

複数のテストを実行するとき、デフォルトでは、テストは複数のスレッドを使用して並行に走ります。
つまり、テストが早く実行し終わり、反応をより早く得られることを意味します。
テストは同時に実行されるので、テストが他のテストや、共有された環境を含む他の共有状態に依存しないようにする必要があります。
現在の作業対象ディレクトリや環境変数などですね。

<!--
For example, say each of your tests runs some code that creates a file on disk
named *test-output.txt* and writes some data to that file. Then each test reads
the data in that file and asserts that the file contains a particular value,
which is different in each test. Because the tests run at the same time, one
test might overwrite the file in the time between another test writing and
reading the file. The second test will then fail, not because the code is
incorrect but because the tests have interfered with each other while running
in parallel. One solution is to make sure each test writes to a different file;
another solution is to run the tests one at a time.
-->

例えば、各テストがディスクに*test_output.txt*というファイルを作成し、何らかのデータを書き込むコードを走らせるとしてください。
そして、各テストはそのファイルのデータを読み取り、ファイルが特定の値を含んでいるとアサーションし、
その値は各テストで異なります。テストが同時に走るので、あるテストが、
他のテストが書き込んだり読み込んだりする間隙にファイルを上書きするかもしれません。
それから2番目のテストが失敗します。コードが不正だからではなく、
並行に実行されている間にテストがお互いに邪魔をしてしまったせいです。
各テストが異なるファイルに書き込むことを確かめるのが一つの解決策です; 別の解決策では、
一度に一つのテストを実行します。

<!--
If you don’t want to run the tests in parallel or if you want more fine-grained
control over the number of threads used, you can send the `--test-threads` flag
and the number of threads you want to use to the test binary. Take a look at
the following example:
-->

並行にテストを実行したくなかったり、使用されるスレッド数をよりきめ細かく制御したい場合、
`--test-threads`フラグと使用したいスレッド数をテストバイナリに送ることができます。
以下の例に目を向けてください:

```console
$ cargo test -- --test-threads=1
```

<!--
We set the number of test threads to `1`, telling the program not to use any
parallelism. Running the tests using one thread will take longer than running
them in parallel, but the tests won’t interfere with each other if they share
state.
-->

テストスレッドの数を`1`にセットし、並行性を使用しないようにプログラムに指示しています。
1スレッドのみを使用してテストを実行すると、並行に実行するより時間がかかりますが、
状態を共有していても、お互いに邪魔をすることはありません。

<!--
### Showing Function Output
-->

### 関数の出力を表示する

<!--
By default, if a test passes, Rust’s test library captures anything printed to
standard output. For example, if we call `println!` in a test and the test
passes, we won’t see the `println!` output in the terminal; we’ll see only the
line that indicates the test passed. If a test fails, we’ll see whatever was
printed to standard output with the rest of the failure message.
-->

標準では、テストが通ると、Rustのテストライブラリは標準出力に出力されたものを全てキャプチャします。例えば、
テストで`println!`を呼び出してテストが通ると、`println!`の出力は、端末に表示されません;
テストが通ったことを示す行しか見られないでしょう。テストが失敗すれば、
残りの失敗メッセージと共に、標準出力に出力されたものが全て見えるでしょう。

<!--
As an example, Listing 11-10 has a silly function that prints the value of its
parameter and returns 10, as well as a test that passes and a test that fails.
-->

例として、リスト11-10は引数の値を出力し、10を返す馬鹿げた関数と通過するテスト1つ、失敗するテスト1つです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-10: Tests for a function that calls
`println!`</span>
-->

<span class="caption">リスト11-10: `println!`を呼び出す関数用のテスト</span>

<!--
When we run these tests with `cargo test`, we’ll see the following output:
-->

これらのテストを`cargo test`で実行すると、以下のような出力を目の当たりにするでしょう:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

<!--
Note that nowhere in this output do we see `I got the value 4`, which is what
is printed when the test that passes runs. That output has been captured. The
output from the test that failed, `I got the value 8`, appears in the section
of the test summary output, which also shows the cause of the test failure.
-->

この出力のどこにも `I got the value 4` と表示されていないことに注意してください。
これは、テストに合格した場合に出力されるものです。
その出力はキャプチャされてしまいました。
失敗したテストのからの出力 `I got the value 8` がテストサマリー出力のセクションに表示され、テストが失敗した原因も示されます。

<!--
If we want to see printed values for passing tests as well, we can tell Rust
to also show the output of successful tests with `--show-output`.
-->

通過するテストについても出力される値が見たかったら、`--show-output`で成功するテストの出力も表示するようにRustに指示することができます:

```console
$ cargo test -- --show-output
```

<!--
When we run the tests in Listing 11-10 again with the `--show-output` flag, we
see the following output:
-->

リスト11-10のテストを`--show-output`フラグと共に再度実行したら、以下のような出力を目の当たりにします:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

<!--
### Running a Subset of Tests by Name
-->

### 名前でテストの一部を実行する

<!--
Sometimes, running a full test suite can take a long time. If you’re working on
code in a particular area, you might want to run only the tests pertaining to
that code. You can choose which tests to run by passing `cargo test` the name
or names of the test(s) you want to run as an argument.
-->

時々、全テストを実行すると時間がかかってしまうことがあります。特定の部分のコードしか対象にしていない場合、
そのコードに関わるテストのみを走らせたいかもしれません。`cargo test`に走らせたいテストの名前を引数として渡すことで、
実行するテストを選ぶことができます。

<!--
To demonstrate how to run a subset of tests, we’ll first create three tests for
our `add_two` function, as shown in Listing 11-11, and choose which ones to run.
-->

テストの一部を走らせる方法を模擬するために、まずリスト11-11に示したように`add_two`関数用に3つテストを作成し、
そして走らせるテストを選択します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-11: Three tests with three different
names</span>
-->

<span class="caption">リスト11-11: 異なる名前の3つのテスト</span>

<!--
If we run the tests without passing any arguments, as we saw earlier, all the
tests will run in parallel:
-->

以前見かけたように、引数なしでテストを走らせたら、全テストが並行に走ります:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

<!--
#### Running Single Tests
-->

#### 単独のテストを走らせる

<!--
We can pass the name of any test function to `cargo test` to run only that test:
-->

あらゆるテスト関数の名前を`cargo test`に渡して、そのテストのみを実行することができます:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

<!--
Only the test with the name `one_hundred` ran; the other two tests didn’t match
that name. The test output lets us know we had more tests that didn’t run by
displaying `2 filtered out` at the end.
-->

`one_hundred`という名前のテストだけが走りました; 他の2つのテストはその名前に合致しなかったのです。
このテスト出力は、最後に`2 filtered out`と表示することによって、実行されなかったテストがあったことを知らせてくれています。

<!--
We can’t specify the names of multiple tests in this way; only the first value
given to `cargo test` will be used. But there is a way to run multiple tests.
-->

この方法では、複数のテストの名前を指定することはできません; `cargo test`に与えられた最初の値のみが使われるのです。
ですが、複数のテストを走らせる方法もあります。

<!--
#### Filtering to Run Multiple Tests
-->

#### 複数のテストを実行するようフィルターをかける

<!--
We can specify part of a test name, and any test whose name matches that value
will be run. For example, because two of our tests’ names contain `add`, we can
run those two by running `cargo test add`:
-->

テスト名の一部を指定でき、その値に合致するあらゆるテストが走ります。例えば、
我々のテストの2つが`add`という名前を含むので、`cargo test add`を実行することで、その二つを走らせることができます:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

<!--
This command ran all tests with `add` in the name and filtered out the test
named `one_hundred`. Also note that the module in which a test appears becomes
part of the test’s name, so we can run all the tests in a module by filtering
on the module’s name.
-->

このコマンドは名前に`add`を含むテストを全て実行し、`one_hundred`という名前のテストを除外しました。
また、テストが出現するモジュールがテスト名の一部になっていて、
モジュール名でフィルターをかけることで、あるモジュール内のテスト全てを実行できることに注目してください。

<!--
### Ignoring Some Tests Unless Specifically Requested
-->

### 特に要望のない限りテストを無視する

<!--
Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, you can instead annotate the
time-consuming tests using the `ignore` attribute to exclude them, as shown
here:
-->

時として、いくつかの特定のテストが実行するのに非常に時間がかかることがあり、
`cargo test`の実行のほとんどで除外したくなるかもしれません。引数として確かに実行したいテストを全て列挙するのではなく、
ここに示したように代わりに時間のかかるテストを`ignore`属性で除外すると注釈することができます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

<!--
After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now
when we run our tests, `it_works` runs, but `expensive_test` doesn’t:
-->

`#[test]`の後の除外したいテストに`#[ignore]`行を追加しています。これで、
テストを実行したら、`it_works`は実行されるものの、`expensive_test`は実行されません:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

<!--
The `expensive_test` function is listed as `ignored`. If we want to run only
the ignored tests, we can use `cargo test -- --ignored`:
-->

`expensive_test`関数は、`ignored`と列挙されています。無視されるテストのみを実行したかったら、
`cargo test -- --ignored`を使うことができます:


```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

<!--
1行目後半、"make sure ..."のところを「結果が早く出る」と訳しているが、この書き方では「結果が早い」としか読めない。どうしたものか
-->

<!--
By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you’re at a point where it makes sense to check the results
of the `ignored` tests and you have time to wait for the results, you can run
`cargo test -- --ignored` instead. If you want to run all tests whether they’re
ignored or not, you can run `cargo test -- --include-ignored`.
-->

どのテストを走らせるか制御することで、結果が早く出ることを確かめることができるのです。
`ignored`テストの結果を確認することが道理に合い、結果を待つだけの時間ができたときに、
代わりに`cargo test -- --ignored`を走らせることができます。
無視されているかどうかにかかわらず、すべてのテストを実行したい場合は、`cargo test -- --include-ignored`で実行することができます。
