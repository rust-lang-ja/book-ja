<!--
## Controlling How Tests Are Run
-->

## テストの実行のされ方を制御する

<!--
Just as `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. You can specify command line options to change the default behavior of
`cargo test`. For example, the default behavior of the binary produced by
`cargo test` is to run all the tests in parallel and capture output generated
during test runs, preventing the output from being displayed and making it
easier to read the output related to the test results.
-->

`cargo run`がコードをコンパイルし、出来上がったバイナリを走らせるのと全く同様に、
`cargo test`はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行します。
コマンドラインオプションを指定して`cargo test`の既定動作を変更することができます。
例えば、`cargo test`で生成されるバイナリの既定動作は、テストを全て並行に実行し、
テスト実行中に生成された出力をキャプチャして出力が表示されるのを防ぎ、
テスト結果に関係する出力を読みやすくすることです。

<!--
Some command line options go to `cargo test` and some go to the resulting test
binary. To separate these two types of arguments, you list the arguments that
go to `cargo test` followed by the separator `--` and then the ones that go to
the test binary. Running `cargo test --help` displays the options you can use
with `cargo test`, and running `cargo test -- --help` displays the options you
can use after the separator `--`.
-->

コマンドラインオプションの中には`cargo test`にかかるものや、出来上がったテストバイナリにかかるものがあります。
この 2 種の引数を区別するために、`cargo test`にかかる引数を`--`という区分記号の後に列挙し、
それからテストバイナリにかかる引数を列挙します。`cargo test --help`を走らせると、`cargo test`で使用できるオプションが表示され、
`cargo test -- --help`を走らせると、`--`という区分記号の後に使えるオプションが表示されます。

<!--
### Running Tests in Parallel or Consecutively
-->

### テストを並行または連続して実行する

<!--
When you run multiple tests, by default they run in parallel using threads.
This means the tests will finish running faster so you can get feedback quicker
on whether or not your code is working. Because the tests are running at the
same time, make sure your tests don’t depend on each other or on any shared
state, including a shared environment, such as the current working directory or
environment variables.
-->

複数のテストを実行するとき、標準では、スレッドを使用して並行に走ります。これはつまり、
テストが早く実行し終わり、コードが機能しているいかんにかかわらず、反応をより早く得られることを意味します。
テストは同時に実行されているので、テストが相互や共有された環境を含む他の共通の状態に依存してないことを確かめてください。
現在の作業対象ディレクトリや環境変数などですね。

<!--
For example, say each of your tests runs some code that creates a file on disk
named *test-output.txt* and writes some data to that file. Then each test reads
the data in that file and asserts that the file contains a particular value,
which is different in each test. Because the tests run at the same time, one
test might overwrite the file between when another test writes and reads the
file. The second test will then fail, not because the code is incorrect but
because the tests have interfered with each other while running in parallel.
One solution is to make sure each test writes to a different file; another
solution is to run the tests one at a time.
-->

例えば、各テストがディスクに*test_output.txt*というファイルを作成し、何らかのデータを書き込むコードを走らせるとしてください。
そして、各テストはそのファイルのデータを読み取り、ファイルが特定の値を含んでいるとアサーションし、
その値は各テストで異なります。テストが同時に走るので、あるテストが、
他のテストが書き込んだり読み込んだりする間隙にファイルを上書きするかもしれません。
それから 2 番目のテストが失敗します。コードが不正だからではなく、
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
以下の例に目を向けてください：

```text
$ cargo test -- --test-threads=1
```

<!--
We set the number of test threads to `1`, telling the program not to use any
parallelism. Running the tests using one thread will take longer than running
them in parallel, but the tests won’t interfere with each other if they share
state.
-->

テストスレッドの数を`1`にセットし、並行性を使用しないようにプログラムに指示しています。
1 スレッドのみを使用してテストを実行すると、並行に実行するより時間がかかりますが、
状態を共有していても、お互いに邪魔をすることはありません。

<!--
### Showing Function Output
-->

### 関数の出力を表示する

<!--
By default, if a test passes, Rust’s test library captures anything printed to
standard output. For example, if we call `println!` in a test and the test
passes, we won’t see the `println!` output in the terminal; we’ll only see the
line that indicates the test passed. If a test fails, we’ll see whatever was
printed to standard output with the rest of the failure message.
-->

標準では、テストが通ると、Rust のテストライブラリは標準出力に出力されたものを全てキャプチャします。例えば、
テストで`println!`を呼び出してテストが通ると、`println!`の出力は、端末に表示されません;
テストが通ったことを示す行しか見られないでしょう。テストが失敗すれば、
残りの失敗メッセージと共に、標準出力に出力されたものが全て見えるでしょう。

<!--
As an example, Listing 11-10 has a silly function that prints the value of its
parameter and returns 10, as well as a test that passes and a test that fails.
-->

例として、リスト 11-10 は引数の値を出力し、10 を返す馬鹿げた関数と通過するテスト 1 つ、失敗するテスト 1 つです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    //{}という値を得た
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

<!--
<span class="caption">Listing 11-10: Tests for a function that calls
`println!`</span>
-->

<span class="caption">リスト 11-10: `println!`を呼び出す関数用のテスト</span>

<!--
When we run these tests with `cargo test`, we’ll see the following output:
-->

これらのテストを`cargo test`で実行すると、以下のような出力を目の当たりにするでしょう：

```text
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
        I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
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
If we want to see printed values for passing tests as well, we can disable the
output capture behavior by using the `--nocapture` flag:
-->

通過するテストについても出力される値が見たかったら、出力キャプチャ機能を`--nocapture`フラグで無効化することができます：

```text
$ cargo test -- --nocapture
```

<!--
When we run the tests in Listing 11-10 again with the `--nocapture` flag, we
see the following output:
-->

リスト 11-10 のテストを`--nocapture`フラグと共に再度実行したら、以下のような出力を目の当たりにします：

```text
running 2 tests
I got the value 4
I got the value 8
test tests::this_test_will_pass ... ok
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
test tests::this_test_will_fail ... FAILED

failures:

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
Note that the output for the tests and the test results are interleaved; the
reason is that the tests are running in parallel, as we talked about in the
previous section. Try using the `--test-threads=1` option and the `--nocapture`
flag, and see what the output looks like then!
-->

テスト用の出力とテスト結果の出力がまぜこぜになっていることに注意してください;
その理由は、前節で語ったようにテストが並行に実行されているからです。
`-test-threads=1`オプションと`--nocapture`フラグを使ってみて、
その時、出力がどうなるか確かめてください！

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
To demonstrate how to run a subset of tests, we’ll create three tests for our
`add_two` function, as shown in Listing 11-11, and choose which ones to run.
-->

テストの一部を走らせる方法を模擬するために、リスト 11-11 に示したように、
`add_two`関数用に 3 つテストを作成し、走らせるテストを選択します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

<!--
<span class="caption">Listing 11-11: Three tests with three different
names</span>
-->

<span class="caption">リスト 11-11: 異なる名前の 3 つのテスト</span>

<!--
If we run the tests without passing any arguments, as we saw earlier, all the
tests will run in parallel:
-->

以前見かけたように、引数なしでテストを走らせたら、全テストが並行に走ります：

```text
running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
#### Running Single Tests
-->

#### 単独のテストを走らせる

<!--
We can pass the name of any test function to `cargo test` to run only that test:
-->

あらゆるテスト関数の名前を`cargo test`に渡して、そのテストのみを実行することができます：

```text
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

<!--
Only the test with the name `one_hundred` ran; the other two tests didn't match
that name. The test output lets us know we had more tests than what this
command ran by displaying `2 filtered out` at the end of the summary line.
-->

`one_hundred`という名前のテストだけが走りました; 他の 2 つのテストはその名前に合致しなかったのです。
まとめ行の最後に`2 filtered out`と表示することでテスト出力は、このコマンドが走らせた以上のテストがあることを知らせてくれています。

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
我々のテストの 2 つが`add`という名前を含むので、`cargo test add`を実行することで、その二つを走らせることができます：

```text
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

<!--
This command ran all tests with `add` in the name and filtered out the test
named `one_hundred`. Also note that the module in which tests appear becomes
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

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 実行に 1 時間かかるコード
    // code that takes an hour to run
}
```

<!--
After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now
when we run our tests, `it_works` runs, but `expensive_test` doesn’t:
-->

`#[test]`の後の除外したいテストに`#[ignore]`行を追加しています。これで、
テストを実行したら、`it_works`は実行されるものの、`expensive_test`は実行されません：

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

<!--
The `expensive_test` function is listed as `ignored`. If we want to run only
the ignored tests, we can use `cargo test -- --ignored`:
-->

`expensive_test`関数は、`ignored`と列挙されています。無視されるテストのみを実行したかったら、
`cargo test -- --ignored`を使うことができます：

```text
$ cargo test -- --ignored
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

<!--
1 行目後半、"make sure ..."のところを「結果が早く出る」と訳しているが、この書き方では「結果が早い」としか読めない。どうしたものか
-->

<!--
By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you’re at a point where it makes sense to check the results
of the `ignored` tests and you have time to wait for the results, you can run
`cargo test -- --ignored` instead.
-->

どのテストを走らせるか制御することで、結果が早く出ることを確かめることができるのです。
`ignored`テストの結果を確認することが道理に合い、結果を待つだけの時間ができたときに、
代わりに`cargo test -- --ignored`を走らせることができます。
