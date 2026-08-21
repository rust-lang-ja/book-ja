<!--
## Test Organization
-->

## テストの体系化

<!--
As mentioned at the start of the chapter, testing is a complex discipline, and
different people use different terminology and organization. The Rust community
thinks about tests in terms of two main categories: unit tests and integration
tests. *Unit tests* are small and more focused, testing one module in isolation
at a time, and can test private interfaces. *Integration tests* are entirely
external to your library and use your code in the same way any other external
code would, using only the public interface and potentially exercising multiple
modules per test.
-->

章の初めで触れたように、テストは複雑な鍛錬であり、人によって専門用語や体系化が異なります。
Rustのコミュニティでは、テストを2つの大きなカテゴリで捉えています: 単体テストと結合テストです。
*単体テスト*は小規模でより集中していて、個別に1回に1モジュールをテストし、非公開のインターフェイスもテストすることがあります。
*結合テスト*は、完全にライブラリ外になり、他の外部コード同様に自分のコードを使用し、公開インターフェイスのみ使用し、
1テストにつき複数のモジュールを用いることもあります。

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
expected. You’ll put unit tests in the *src* directory in each file with the
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
the test code only when you run `cargo test`, not when you run `cargo build`.
This saves compile time when you only want to build the library and saves space
in the resulting compiled artifact because the tests are not included. You’ll
see that because integration tests go in a different directory, they don’t need
the `#[cfg(test)]` annotation. However, because unit tests go in the same files
as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be
included in the compiled result.
-->

testsモジュールの`#[cfg(test)]`という注釈は、コンパイラに`cargo build`を走らせた時ではなく、`cargo test`を走らせた時にだけ、
テストコードをコンパイルし走らせるよう指示します。これにより、ライブラリをビルドしたいだけの時にはコンパイルタイムを節約し、
テストが含まれないので、コンパイル後の成果物のサイズも節約します。結合テストは別のディレクトリに存在することになるので、
`#[cfg(test)]`注釈は必要ないとわかるでしょう。しかしながら、単体テストはコードと同じファイルに存在するので、
`#[cfg(test)]`を使用してコンパイル結果に含まれないよう指定するのです。

<!--
Recall that when we generated the new `adder` project in the first section of
this chapter, Cargo generated this code for us:
-->

この章の最初の節で新しい`adder`プロジェクトを生成した時に、Cargoがこのコードも生成してくれたことを思い出してください:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
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
Cargoがテストコードをコンパイルします。これには、このモジュールに含まれるかもしれないヘルパー関数全ても含まれ、
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
あなたがどちらのテストイデオロギーを支持しているかに関わらず、Rustの公開性規則により、
非公開関数をテストすることが確かに可能です。リスト11-12の非公開関数`internal_adder`を含むコードを考えてください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 11-12: Testing a private function</span>
-->

<span class="caption">リスト11-12: 非公開関数をテストする</span>

<!--
Note that the `internal_adder` function is not marked as `pub`. Tests are just
Rust code, and the `tests` module is just another module. As we discussed in
the [“Paths for Referring to an Item in the Module Tree”][paths]
section, items in child modules can use the items in their ancestor modules. In
this test, we bring all of the `test` module’s parent’s items into scope with
`use super::*`, and then the test can call `internal_adder`. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.
-->

`internal_adder`関数は`pub`とマークされていないことに注意してください。
テストも単なるRustのコードであり、`tests`モジュールもただのモジュールでしかありません。
[「モジュールツリーの要素を示すためのパス」][paths]節で議論したように、子モジュール内の要素はその祖先モジュール内の要素を使用することができます。
このテストでは、`use super::*`によって`test`モジュールの親のすべての要素をスコープ内に持ち込み、そしてテストが`internal_adder`を呼び出しています。
非公開関数はテストするべきではないとお考えなら、Rustにはそれを強制するものは何もありません。

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

Rustにおいて、結合テストは完全にライブラリ外のものです。他のコードと全く同様にあなたのライブラリを使用するので、
ライブラリの公開APIの一部である関数しか呼び出すことはできません。その目的は、
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
can then make as many test files as we want, and Cargo will compile each of the
files as an individual crate.
-->

プロジェクトディレクトリのトップ階層、*src*の隣に*tests*ディレクトリを作成します。
Cargoは、このディレクトリに結合テストのファイルを探すことを把握しています。
テストファイルは好きなだけ作成することができ、Cargoはそれぞれのファイルを個別のクレートとしてコンパイルします。

<!--
Let’s create an integration test. With the code in Listing 11-12 still in the
*src/lib.rs* file, make a *tests* directory, and create a new file named
*tests/integration_test.rs*. Your directory structure should look like this:
-->

結合テストを作成しましょう。リスト11-12のコードが*src/lib.rs*ファイルにあるまま、
*tests*ディレクトリを作成し、*tests/integration_test.rs*という名前の新しいファイルを生成してください。
ディレクトリ構造は次のようになるはずです:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

<!--
Enter the code in Listing 11-13 into the *tests/integration_test.rs* file:
-->

リスト11-13のコードを*tests/integration_test.rs*ファイルに入力してください:

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">ファイル名: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<!--
<span class="caption">Listing 11-13: An integration test of a function in the
`adder` crate</span>
-->

<span class="caption">リスト11-13: `adder`クレートの関数の結合テスト</span>

<!--
Each file in the `tests` directory is a separate crate, so we need to bring our
library into each test crate’s scope. For that reason we add `use adder` at the
top of the code, which we didn’t need in the unit tests.
-->

`tests`ディレクトリのファイルはそれぞれ個別のクレートであるため、
ライブラリを各テストのスコープ内に持ち込む必要があります。
そのため、コードの先頭に`use adder`を追記していますが、これは単体テストでは必要なかったものです。

<!--
We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files
in this directory only when we run `cargo test`. Run `cargo test` now:
-->

*tests/integration_test.rs*のどんなコードも`#[cfg(test)]`で注釈する必要はありません。
Cargoは`tests`ディレクトリを特別に扱い、`cargo test`を走らせた時にのみこのディレクトリのファイルをコンパイルするのです。
さあ、`cargo test`を実行してください:


```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

<!--
The three sections of output include the unit tests, the integration test, and
the doc tests. Note that if any test in a section fails, the following sections
will not be run. For example, if a unit test fails, there won’t be any output
for integration and doc tests because those tests will only be run if all unit
tests are passing.
-->

3つの区域の出力が単体テスト、結合テスト、docテストを含んでいます。
ある区域内のテストがひとつでも失敗していれば、それ以降の区域は実行されないでしょう。
例えば、単体テストが失敗したら、結合テストとdocテストについての出力はされないでしょう。
すべての単体テストが通過する場合のみ、これらのテストが実行されるからです。

<!--
The first section for the unit tests is the same as we’ve been seeing: one line
for each unit test (one named `internal` that we added in Listing 11-12) and
then a summary line for the unit tests.
-->

単体テスト用の最初の区域は、今まで見てきたものと同じです:
各単体テストに1行(リスト11-12で追加した`internal`という名前のもの)と、単体テストのサマリー行です。

<!--
The integration tests section starts with the line `Running
tests/integration_test.rs`. Next, there is a line for each test function in
that integration test and a summary line for the results of the integration
test just before the `Doc-tests adder` section starts.
-->

結合テストの区域は、`Running tests/integration_test.rs`という行で始まっています。
次に、この結合テストの各テスト関数用の行があり、`Doc-tests adder`区域が始まる直前に、
結合テストの結果用のサマリー行があります。

<!--
Each integration test file has its own section, so if we add more files in the
*tests* directory, there will be more integration test sections.
-->

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
その後にファイル名を続けて使用してください:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
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
As you add more integration tests, you might want to make more files in the
*tests* directory to help organize them; for example, you can group the test
functions by the functionality they’re testing. As mentioned earlier, each file
in the *tests* directory is compiled as its own separate crate, which is useful
for creating separate scopes to more closely imitate the way end users will be
using your crate. However, this means files in the *tests* directory don’t
share the same behavior as files in *src* do, as you learned in Chapter 7
regarding how to separate code into modules and files.
-->

結合テストを追加するにつれて、*tests*ディレクトリにファイルを作成して体系化したくなるかもしれません;
例えば、テスト対象となる機能でテスト関数をグループ化することができます。前述したように、
*tests*ディレクトリの各ファイルは、個別のクレートとしてコンパイルされます。
これは、エンドユーザがあなたのクレートを使用する場合をより模倣するように個別のスコープを生成するのに役立ちます。
ですが、これは*tests*ディレクトリのファイルが、コードをモジュールとファイルに分ける方法に関して第7章で学んだように、
*src*のファイルとは同じ振る舞いを共有しないことを意味します。

<!--
The different behavior of *tests* directory files is most noticeable when you
have a set of helper functions to use in multiple integration test files and
you try to follow the steps in the [“Separating Modules into Different
Files”][separating-modules-into-files] section of Chapter 7 to
extract them into a common module. For example, if we create *tests/common.rs*
and place a function named `setup` in it, we can add some code to `setup` that
we want to call from multiple test functions in multiple test files:
-->

*tests*ディレクトリのファイルの異なる振る舞いは、複数の結合テストファイルで使用するヘルパー関数ができ、
第7章の[「モジュールを複数のファイルに分割する」][separating-modules-into-files]節の手順に従って共通モジュールに抽出しようとした時に最も気付きやすくなります。
例えば、*tests/common.rs*を作成し、そこに`setup`という名前の関数を配置したら、
複数のテストファイルの複数のテスト関数から呼び出したい`setup`に何らかのコードを追加することができます:

<!--
<span class="filename">Filename: tests/common.rs</span>
-->

<span class="filename">ファイル名: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

<!--
When we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions nor
did we call the `setup` function from anywhere:
-->

再度テストを実行すると、*common.rs*ファイルは何もテスト関数を含んだり、`setup`関数をどこかから呼んだりしてないのに、
テスト出力に*common.rs*用の区域が見えるでしょう。

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
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
*tests/common.rs*, we’ll create *tests/common/mod.rs*. The project directory
now looks like this:
-->

`common`がテスト出力に出現するのを防ぐには、*tests/common.rs*を作成する代わりに、
*tests/common/mod.rs*を作成します。これでプロジェクトディレクトリは次のようになります:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

<!--
This is the older naming convention that Rust also understands that we
mentioned in the [“Alternate File Paths”][alt-paths] section of
Chapter 7. Naming the file this way tells Rust not to treat the `common` module
as an integration test file. When we move the `setup` function code into
*tests/common/mod.rs* and delete the *tests/common.rs* file, the section in the
test output will no longer appear. Files in subdirectories of the *tests*
directory don’t get compiled as separate crates or have sections in the test
output.
-->

これは第7章の[「別のファイルパス」][alt-paths]節で言及した、コンパイラが理解するもう一つの古い命名規則です。
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
こちらは、*tests/integration_test.rs*内の`it_adds_two`テストから`setup`関数を呼び出す例です:

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">ファイル名: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

<!--
Note that the `mod common;` declaration is the same as the module declaration
we demonstrated in Listing 7-21. Then in the test function, we can call the
`common::setup()` function.
-->

`mod common;`という宣言は、リスト7-21で模擬したモジュール宣言と同じであることに注意してください。それから、テスト関数内で`common::setup()`関数を呼び出すことができます。

<!--
#### Integration Tests for Binary Crates
-->

#### バイナリクレート用の結合テスト

<!--
If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and bring functions defined in the *src/main.rs* file into
scope with a `use` statement. Only library crates expose functions that other
crates can use; binary crates are meant to be run on their own.
-->

もしもプロジェクトが*src/main.rs*ファイルのみを含み、*src/lib.rs*ファイルを持たないバイナリクレートだったら、
*tests*ディレクトリに結合テストを作成し、`use`文で*src/main.rs*ファイルに定義された関数をスコープ内に持ち込むことはできません。
ライブラリクレートのみが、他のクレートが使用できる関数を晒せるのです; 
バイナリクレートはそれ単体で実行することを意味しています。

<!--
This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* file that calls logic that lives in the
*src/lib.rs* file. Using that structure, integration tests *can* test the
library crate with `use` to make the important functionality available.
If the important functionality works, the small amount of code in the
*src/main.rs* file will work as well, and that small amount of code doesn’t
need to be tested.
-->

これは、バイナリを提供するRustのプロジェクトに、
*src/lib.rs*ファイルに存在するロジックを呼び出す単純な*src/main.rs*ファイルがある一因になっています。
この構造を使用して結合テストは、`use`で重要な機能を利用できるようにすることでライブラリクレートをテストすることが*できます*。
この重要な機能が動作すれば、*src/main.rs*ファイルの少量のコードも動作し、その少量のコードはテストする必要がないわけです。

<!--
## Summary
-->

## まとめ

<!--
Rust’s testing features provide a way to specify how code should function to
ensure it continues to work as you expect, even as you make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests check that many parts of the library
work together correctly, and they use the library’s public API to test the code
in the same way external code will use it. Even though Rust’s type system and
ownership rules help prevent some kinds of bugs, tests are still important to
reduce logic bugs having to do with how your code is expected to behave.
-->

Rustのテスト機能は、変更を加えた後でさえ想定通りにコードが機能し続けることを保証して、
コードが機能すべき方法を指定する手段を提供します。単体テストはライブラリの異なる部分を個別に用い、
非公開の実装詳細をテストすることができます。結合テストは、ライブラリのいろんな部分が共同で正常に動作することを確認し、
ライブラリの公開APIを使用して外部コードが使用するのと同じ方法でコードをテストします。
Rustの型システムと所有権ルールにより防がれるバグの種類もあるものの、それでもテストは、
コードが振る舞うと予想される方法に関するロジックのバグを減らすのに重要なのです。

<!--
Let’s combine the knowledge you learned in this chapter and in previous
chapters to work on a project!
-->

この章と以前の章で学んだ知識を結集して、とあるプロジェクトに取り掛かりましょう！

<!--
[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#alternate-file-paths
-->

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#別のファイルパス
