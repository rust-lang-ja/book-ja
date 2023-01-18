<!--
## Test Organization
-->

## テストの体系化

<!--
As mentioned at the start of the chapter, testing is a complex discipline, and
different people use different terminology and organization. The Rust community
thinks about tests in terms of two main categories: *unit tests* and
*integration tests*. Unit tests are small and more focused, testing one module
in isolation at a time, and can test private interfaces. Integration tests are
entirely external to your library and use your code in the same way any other
external code would, using only the public interface and potentially exercising
multiple modules per test.
-->

章の初めで触れたように、テストは複雑な鍛錬であり、人によって専門用語や体系化が異なります。
Rust のコミュニティでは、テストを 2 つの大きなカテゴリで捉えています：*単体テスト*と*結合テスト*です。
単体テストは小規模でより集中していて、個別に 1 回に 1 モジュールをテストし、非公開のインターフェイスもテストすることがあります。
結合テストは、完全にライブラリ外になり、他の外部コード同様に自分のコードを使用し、公開インターフェイスのみ使用し、
1 テストにつき複数のモジュールを用いることもあります。

<!--
Writing both kinds of tests is important to ensure that the pieces of your
library are doing what you expect them to, separately and together.
-->

どちらのテストを書くのも、ライブラリの一部が個別かつ共同でしてほしいことをしていることを確認するのに重要なのです。

<!--
### Unit Tests
-->

### 単体テスト

<!--
The purpose of unit tests is to test each unit of code in isolation from the
rest of the code to quickly pinpoint where code is and isn’t working as
expected. We put unit tests in the *src* directory in each file with the
code that they’re testing. The convention is to create a module named `tests`
in each file to contain the test functions and to annotate the module with
`cfg(test)`.
-->

単体テストの目的は、残りのコードから切り離して各単位のコードをテストし、
コードが想定通り、動いたり動いていなかったりする箇所を迅速に特定することです。
単体テストは、テスト対象となるコードと共に、*src*ディレクトリの各ファイルに置きます。
慣習は、各ファイルに`tests`という名前のモジュールを作り、テスト関数を含ませ、
そのモジュールを`cfg(test)`で注釈することです。

<!--
#### The Tests Module and `#[cfg(test)]`
-->

#### テストモジュールと`#[cfg(test)]`

<!--
The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run
the test code only when we run `cargo test`, but not when we run `cargo build`.
This saves compile time when we only want to build the library and saves space
in the resulting compiled artifact because the tests are not included. You’ll
see that because integration tests go in a different directory, they don’t need
the `#[cfg(test)]` annotation. However, because unit tests go in the same files
as the code, we use `#[cfg(test)]` to specify that they shouldn’t be
included in the compiled result.
-->

tests モジュールの`#[cfg(test)]`という注釈は、コンパイラに`cargo build`を走らせた時ではなく、`cargo test`を走らせた時にだけ、
テストコードをコンパイルし走らせるよう指示します。これにより、ライブラリをビルドしたいだけの時にはコンパイルタイムを節約し、
テストが含まれないので、コンパイル後の成果物のサイズも節約します。結合テストは別のディレクトリに存在することになるので、
`#[cfg(test)]`注釈は必要ないとわかるでしょう。しかしながら、単体テストはコードと同じファイルに存在するので、
`#[cfg(test)]`を使用してコンパイル結果に含まれないよう指定するのです。

<!--
Recall that when we generated the new `adder` project in the first section of
this chapter, Cargo generated this code for us:
-->

この章の最初の節で新しい`adder`プロジェクトを生成した時に、Cargo がこのコードも生成してくれたことを思い出してください：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

<!--
This code is the automatically generated test module. The attribute `cfg`
stands for *configuration* and tells Rust that the following item should only
be included given a certain configuration option. In this case, the
configuration option is `test`, which is provided by Rust for compiling and
running tests. By using the `cfg` attribute, Cargo compiles our test code only
if we actively run the tests with `cargo test`. This includes any helper
functions that might be within this module, in addition to the functions
annotated with `#[test]`.
-->

このコードが自動生成されたテストモジュールです。`cfg`という属性は、*configuration*を表していて、
コンパイラに続く要素が、ある特定の設定オプションを与えられたら、含まれるように指示します。
今回の場合、設定オプションは、`test`であり、言語によって提供されているテストをコンパイルし、
走らせるためのものです。`cfg`属性を使用することで、`cargo test`で積極的にテストを実行した場合のみ、
Cargo がテストコードをコンパイルします。これには、このモジュールに含まれるかもしれないヘルパー関数全ても含まれ、
`#[test]`で注釈された関数だけにはなりません。

<!--
#### Testing Private Functions
-->

#### 非公開関数をテストする

<!--
There’s debate within the testing community about whether or not private
functions should be tested directly, and other languages make it difficult or
impossible to test private functions. Regardless of which testing ideology you
adhere to, Rust’s privacy rules do allow you to test private functions.
Consider the code in Listing 11-12 with the private function `internal_adder`.
-->

テストコミュニティ内で非公開関数を直接テストするべきかについては議論があり、
他の言語では非公開関数をテストするのは困難だったり、不可能だったりします。
あなたがどちらのテストイデオロギーを支持しているかに関わらず、Rust の公開性規則により、
非公開関数をテストすることが確かに可能です。リスト 11-12 の非公開関数`internal_adder`を含むコードを考えてください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

<!--
<span class="caption">Listing 11-12: Testing a private function</span>
-->

<span class="caption">リスト 11-12: 非公開関数をテストする</span>

<!--
Note that the `internal_adder` function is not marked as `pub`, but because
tests are just Rust code and the `tests` module is just another module, you can
import and call `internal_adder` in a test just fine. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.
-->

`internal_adder`関数は`pub`とマークされていないものの、テストも単なる Rust のコードであり、
`tests`モジュールもただのモジュールでしかないので、テスト内で`internal_adder`を普通にインポートし呼び出すことができます。
非公開関数はテストするべきではないとお考えなら、Rust にはそれを強制するものは何もありません。

<!--
### Integration Tests
-->

### 結合テスト

<!--
In Rust, integration tests are entirely external to your library. They use your
library in the same way any other code would, which means they can only call
functions that are part of your library’s public API. Their purpose is to test
whether many parts of your library work together correctly. Units of code that
work correctly on their own could have problems when integrated, so test
coverage of the integrated code is important as well. To create integration
tests, you first need a *tests* directory.
-->

Rust において、結合テストは完全にライブラリ外のものです。他のコードと全く同様にあなたのライブラリを使用するので、
ライブラリの公開 API の一部である関数しか呼び出すことはできません。その目的は、
ライブラリのいろんな部分が共同で正常に動作しているかをテストすることです。
単体では正常に動くコードも、結合した状態だと問題を孕む可能性もあるので、
結合したコードのテストの範囲も同様に重要になるのです。結合テストを作成するには、
まず*tests*ディレクトリが必要になります。

<!--
#### The *tests* Directory
-->

#### *tests*ディレクトリ

<!--
We create a *tests* directory at the top level of our project directory, next
to *src*. Cargo knows to look for integration test files in this directory. We
can then make as many test files as we want to in this directory, and Cargo
will compile each of the files as an individual crate.
-->

プロジェクトディレクトリのトップ階層、*src*の隣に*tests*ディレクトリを作成します。
Cargo は、このディレクトリに結合テストのファイルを探すことを把握しています。
そして、このディレクトリ内にいくらでもテストファイルを作成することができ、
Cargo はそれぞれのファイルを個別のクレートとしてコンパイルします。

<!--
Let’s create an integration test. With the code in Listing 11-12 still in the
*src/lib.rs* file, make a *tests* directory, create a new file named
*tests/integration_test.rs*, and enter the code in Listing 11-13.
-->

結合テストを作成しましょう。リスト 11-12 のコードが*src/lib.rs*ファイルにあるまま、
*tests*ディレクトリを作成し、*tests/integration_test.rs*という名前の新しいファイルを生成し、
リスト 11-13 のコードを入力してください。

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">ファイル名：tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

<!--
<span class="caption">Listing 11-13: An integration test of a function in the
`adder` crate</span>
-->

<span class="caption">リスト 11-13: `adder`クレートの関数の結合テスト</span>

<!--
We’ve added `extern crate adder` at the top of the code, which we didn’t need
in the unit tests. The reason is that each test in the `tests` directory is a
separate crate, so we need to import our library into each of them.
-->

コードの頂点に`extern crate adder`を追記しましたが、これは単体テストでは必要なかったものです。
理由は、`tests`ディレクトリのテストはそれぞれ個別のクレートであるため、
各々ライブラリをインポートする必要があるためです。

<!--
We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files
in this directory only when we run `cargo test`. Run `cargo test` now:
-->

*tests/integration_test.rs*のどんなコードも`#[cfg(test)]`で注釈する必要はありません。
Cargo は`tests`ディレクトリを特別に扱い、`cargo test`を走らせた時にのみこのディレクトリのファイルをコンパイルするのです。
さあ、`cargo test`を実行してください：

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
The three sections of output include the unit tests, the integration test, and
the doc tests. The first section for the unit tests is the same as we’ve been
seeing: one line for each unit test (one named `internal` that we added in
Listing 11-12) and then a summary line for the unit tests.
-->

3 つの区域の出力が単体テスト、結合テスト、ドックテストを含んでいます。単体テスト用の最初の区域は、
今まで見てきたものと同じです：各単体テストに 1 行 (リスト 11-12 で追加した`internal`という名前のもの) と、
単体テストのサマリー行です。

<!--
The integration tests section starts with the line `Running
target/debug/deps/integration-test-ce99bcc2479f4607` (the hash at the end of
your output will be different). Next, there is a line for each test function in
that integration test and a summary line for the results of the integration
test just before the `Doc-tests adder` section starts.
-->

結合テストの区域は、
`Running target/debug/deps/integration-test-ce99bcc2479f4607`という行で始まっています (最後のハッシュはあなたの出力とは違うでしょう)。
次に、この結合テストの各テスト関数用の行があり、`Doc-tests adder`区域が始まる直前に、
結合テストの結果用のサマリー行があります。

<!--
Similarly to how adding more unit test functions adds more result lines to the
unit tests section, adding more test functions to the integration test file
adds more result lines to this integration test file’s section. Each
integration test file has its own section, so if we add more files in the
*tests* directory, there will be more integration test sections.
-->

単体テスト関数を追加することで単体テスト区域のテスト結果の行が増えたように、
作成した結合テストファイルにテスト関数を追加することでそのファイルの区域に結果の行が増えることになります。
結合テストファイルはそれぞれ独自の区域があるため、*tests*ディレクトリにさらにファイルを追加すれば、
結合テストの区域が増えることになるでしょう。

<!--
We can still run a particular integration test function by specifying the test
function’s name as an argument to `cargo test`. To run all the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file:
-->

それでも、テスト関数の名前を引数として`cargo test`に指定することで、特定の結合テスト関数を走らせることができます。
特定の結合テストファイルにあるテストを全て走らせるには、`cargo test`に`--test`引数、
その後にファイル名を続けて使用してください：

```text
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
This command runs only the tests in the *tests/integration_test.rs* file.
-->

このコマンドは、*tests/integration_test.rs*ファイルにあるテストのみを実行します。

<!--
#### Submodules in Integration Tests
-->

#### 結合テスト内のサブモジュール

<!--
As you add more integration tests, you might want to make more than one file in
the *tests* directory to help organize them; for example, you can group the
test functions by the functionality they’re testing. As mentioned earlier, each
file in the *tests* directory is compiled as its own separate crate.
-->

結合テストを追加するにつれて、*tests*ディレクトリに 2 つ以上のファイルを作成して体系化したくなるかもしれません;
例えば、テスト対象となる機能でテスト関数をグループ化することができます。前述したように、
*tests*ディレクトリの各ファイルは、個別のクレートとしてコンパイルされます。

<!--
Treating each integration test file as its own crate is useful to create
separate scopes that are more like the way end users will be using your crate.
However, this means files in the *tests* directory don’t share the same
behavior as files in *src* do, as you learned in Chapter 7 regarding how to
separate code into modules and files.
-->

各結合テストファイルをそれ自身のクレートとして扱うと、
エンドユーザがあなたのクレートを使用するかのように個別のスコープを生成するのに役立ちます。
ですが、これは*tests*ディレクトリのファイルが、コードをモジュールとファイルに分ける方法に関して第 7 章で学んだように、
*src*のファイルとは同じ振る舞いを共有しないことを意味します。

<!--
The different behavior of files in the *tests* directory is most noticeable
when you have a set of helper functions that would be useful in multiple
integration test files and you try to follow the steps in the “Moving Modules
to Other Files” section of Chapter 7 to extract them into a common module. For
example, if we create *tests/common.rs* and place a function named `setup` in
it, we can add some code to `setup` that we want to call from multiple test
functions in multiple test files:
-->

*tests*ディレクトリのファイルの異なる振る舞いは、複数の結合テストファイルで役に立ちそうなヘルパー関数ができ、
第 7 章の「モジュールを別のファイルに移動する」節の手順に従って共通モジュールに抽出しようとした時に最も気付きやすくなります。
例えば、*tests/common.rs*を作成し、そこに`setup`という名前の関数を配置したら、
複数のテストファイルの複数のテスト関数から呼び出したい`setup`に何らかのコードを追加することができます：

<!--
<span class="filename">Filename: tests/common.rs</span>
-->

<span class="filename">ファイル名：tests/common.rs</span>

```rust
pub fn setup() {
    // ここにライブラリテスト固有のコードが来る
    // setup code specific to your library's tests would go here
}
```

<!--
When we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions nor
did we call the `setup` function from anywhere:
-->

再度テストを実行すると、*common.rs*ファイルは何もテスト関数を含んだり、`setup`関数をどこかから呼んだりしてないのに、
テスト出力に*common.rs*用の区域が見えるでしょう。

```text
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
Having `common` appear in the test results with `running 0 tests` displayed for
it is not what we wanted. We just wanted to share some code with the other
integration test files.
-->

`common`が`running 0 tests`とテスト結果に表示されるのは、望んだ結果ではありません。
ただ単に他の結合テストファイルと何らかのコードを共有したかっただけです。

<!--
To avoid having `common` appear in the test output, instead of creating
*tests/common.rs*, we’ll create *tests/common/mod.rs*. In the “Rules of Module
Filesystems” section of Chapter 7, we used the naming convention
*module_name/mod.rs* for files of modules that have submodules. We don’t have
submodules for `common` here, but naming the file this way tells Rust not to
treat the `common` module as an integration test file. When we move the `setup`
function code into *tests/common/mod.rs* and delete the *tests/common.rs* file,
the section in the test output will no longer appear. Files in subdirectories
of the *tests* directory don’t get compiled as separate crates or have sections
in the test output.
-->

`common`がテスト出力に出現するのを防ぐには、*tests/common.rs*を作成する代わりに、
*tests/common/mod.rs*を作成します。第7章の「モジュールファイルシステムの規則」節において、
*module_name/mod.rs*という命名規則をサブモジュールのあるモジュールのファイルに使用しました。
ここでは`common`にサブモジュールはありませんが、
このように命名することでコンパイラに`common`モジュールを結合テストファイルとして扱わないように指示します。
`setup`関数のコードを*tests/common/mod.rs*に移動し、*tests/common.rs*ファイルを削除すると、
テスト出力に区域はもう表示されなくなります。*tests*ディレクトリのサブディレクトリ内のファイルは個別クレートとしてコンパイルされたり、
テスト出力に区域が表示されることがないのです。

<!--
After we’ve created *tests/common/mod.rs*, we can use it from any of the
integration test files as a module. Here’s an example of calling the `setup`
function from the `it_adds_two` test in *tests/integration_test.rs*:
-->

*tests/common/mod.rs*を作成した後、それをどの結合テストファイルからもモジュールとして使用することができます。
こちらは、*tests/integration_test.rs*内の`it_adds_two`テストから`setup`関数を呼び出す例です：

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">ファイル名：tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

<!--
Note that the mod common; declaration is the same as the module declaration we demonstrated in Listing 7-21. Then in the test function, we can call the common::setup() function.
-->

`mod common;`という宣言は、リスト 7-21 で模擬したモジュール宣言と同じであることに注意してください。それから、テスト関数内で`common::setup()`関数を呼び出すことができます。

<!--
#### Integration Tests for Binary Crates
-->

#### バイナリクレート用の結合テスト

<!--
If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and use `extern crate` to import functions defined in the
*src/main.rs* file. Only library crates expose functions that other crates can
call and use; binary crates are meant to be run on their own.
-->

もしもプロジェクトが*src/main.rs*ファイルのみを含み、*src/lib.rs*ファイルを持たないバイナリクレートだったら、
*tests*ディレクトリに結合テストを作成し、
`extern crate`を使用して*src/main.rs*ファイルに定義された関数をインポートすることはできません。
ライブラリクレートのみが、他のクレートが呼び出して使用できる関数を晒せるのです; 
バイナリクレートはそれ単体で実行することを意味しています。

<!--
This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* file that calls logic that lives in the
*src/lib.rs* file. Using that structure, integration tests *can* test the
library crate by using `extern crate` to exercise the important functionality.
If the important functionality works, the small amount of code in the
*src/main.rs* file will work as well, and that small amount of code doesn’t
need to be tested.
-->

これは、バイナリを提供する Rust のプロジェクトに、
*src/lib.rs*ファイルに存在するロジックを呼び出す単純な*src/main.rs*ファイルがある一因になっています。
この構造を使用して結合テストは、`extern crate`を使用して重要な機能を用いることでライブラリクレートをテストすることが*できます*。
この重要な機能が動作すれば、*src/main.rs*ファイルの少量のコードも動作し、その少量のコードはテストする必要がないわけです。

<!--
## Summary
-->

## まとめ

<!--
Rust’s testing features provide a way to specify how code should function to
ensure it continues to work as we expect, even as we make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests check that many parts of the library
work together correctly, and they use the library’s public API to test the code
in the same way external code will use it. Even though Rust’s type system and
ownership rules help prevent some kinds of bugs, tests are still important to
help reduce logic bugs having to do with how your code is expected to behave.
-->

Rust のテスト機能は、変更を加えた後でさえ想定通りにコードが機能し続けることを保証して、
コードが機能すべき方法を指定する手段を提供します。単体テストはライブラリの異なる部分を個別に用い、
非公開の実装詳細をテストすることができます。結合テストは、ライブラリのいろんな部分が共同で正常に動作することを確認し、
ライブラリの公開 API を使用して外部コードが使用するのと同じ方法でコードをテストします。
Rust の型システムと所有権ルールにより防がれるバグの種類もあるものの、それでもテストは、
コードが振る舞うと予想される方法に関するロジックのバグを減らすのに重要なのです。

<!--
Let’s combine the knowledge you learned in this chapter and in previous
chapters and work on a project!
-->

この章と以前の章で学んだ知識を結集して、とあるプロジェクトに取り掛かりましょう！
