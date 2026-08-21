<!--
## Cargo Workspaces
-->

## Cargoのワークスペース

<!--
In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split your package further into
multiple library crates. Cargo offers a feature called *workspaces* that can
help manage multiple related packages that are developed in tandem.
-->

第12章で、バイナリクレートとライブラリクレートを含むパッケージを構築しました。プロジェクトの開発が進むにつれて、
ライブラリクレートの肥大化が続き、その上で複数のライブラリクレートにパッケージを分割したくなることでしょう。
Cargoは*ワークスペース*という協調して開発された関連のある複数のパッケージを管理するのに役立つ機能を提供しています。

<!--
### Creating a Workspace
-->

### ワークスペースを生成する

<!--
A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory. Let’s make a project using a workspace—we’ll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace, so we'll just show one common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We’ll start by creating
a new directory for the workspace:
-->

*ワークスペース*は、同じ*Cargo.lock*と出力ディレクトリを共有する一連のパッケージです。
ワークスペースを使用したプロジェクトを作成し、ワークスペースの構造に集中できるよう、瑣末なコードを使用しましょう。
ワークスペースを構築する方法は複数あるので、よく使われる方法だけを提示しましょう。バイナリ1つとライブラリ2つを含むワークスペースを作ります。
バイナリは、主要な機能を提供しますが、2つのライブラリに依存しています。
一方のライブラリは、`add_one`関数を提供し、2番目のライブラリは、`add_two`関数を提供します。
これら3つのクレートが同じワークスペースの一部になります。ワークスペース用の新しいディレクトリを作ることから始めましょう:

```console
$ mkdir add
$ cd add
```

<!--
Next, in the *add* directory, we create the *Cargo.toml* file that will
configure the entire workspace. This file won’t have a `[package]` section.
Instead, it will start with a `[workspace]` section that will allow us to add
members to the workspace by specifying the path to the package with our binary
crate; in this case, that path is *adder*:
-->

次に*add*ディレクトリにワークスペース全体を設定する*Cargo.toml*ファイルを作成します。
このファイルには`[package]`セクションはありません。
代わりに、バイナリクレートを含むパッケージへのパスを指定することでワークスペースにメンバを追加させてくれる`[workspace]`セクションから開始します;
今回の場合、そのパスは*adder*です:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

<!--
Next, we’ll create the `adder` binary crate by running `cargo new` within the
*add* directory:
-->

次に、*add*ディレクトリ内で`cargo new`を実行することで`adder`バイナリクレートを作成しましょう:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
```

<!--
At this point, we can build the workspace by running `cargo build`. The files
in your *add* directory should look like this:
-->

この時点で、`cargo build`を走らせるとワークスペースを構築できます。*add*ディレクトリに存在するファイルは、
以下のようになるはずです:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
The workspace has one *target* directory at the top level that the compiled
artifacts will be placed into; the `adder` package doesn’t have its own
*target* directory. Even if we were to run `cargo build` from inside the
*adder* directory, the compiled artifacts would still end up in *add/target*
rather than *add/adder/target*. Cargo structures the *target* directory in a
workspace like this because the crates in a workspace are meant to depend on
each other. If each crate had its own *target* directory, each crate would have
to recompile each of the other crates in the workspace to place the artifacts
in its own *target* directory. By sharing one *target* directory, the crates
can avoid unnecessary rebuilding.
-->

ワークスペースは、コンパイルした生成物が置かれる単一の*target*ディレクトリを最上位に持ちます;
`adder`パッケージには*target*ディレクトリはありません。
*adder*ディレクトリ内部から`cargo build`を走らせることになっていたとしても、コンパイルされる生成物は、
*add/adder/target*ではなく、*add/target*に落ち着くでしょう。ワークスペースのクレートは、
お互いに依存しあうことを意味するので、Cargoはワークスペースの*target*ディレクトリをこのように構成します。
各クレートが*target*ディレクトリを持っていたら、各クレートは自身の*target*ディレクトリにワークスペースの他のクレートの生成物を置くために、
ワークスペースの他のクレートを再コンパイルしなくてはならなくなるでしょう。一つの*target*ディレクトリを共有することで、
クレートは不必要な再ビルドを回避できるのです。

<!--
### Creating the Second Package in the Workspace
-->

### ワークスペース内に2番目のパッケージを作成する

<!--
Next, let’s create another member package in the workspace and call it
`add_one`. Change the top-level *Cargo.toml* to specify the *add_one* path in
the `members` list:
-->

次に、ワークスペースに別のメンバパッケージを作成し、`add_one`と呼びましょう。
最上位の*Cargo.toml*を変更して`members`リストで*add_one*パスを指定するようにしてください:

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

<!--
Then generate a new library crate named `add_one`:
-->

それから、`add_one`という名前のライブラリクレートを生成してください:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
```

<!--
Your *add* directory should now have these directories and files:
-->

これで*add*ディレクトリには、以下のディレクトリやファイルが存在するはずです:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
In the *add_one/src/lib.rs* file, let’s add an `add_one` function:
-->

*add_one/src/lib.rs*ファイルに`add_one`関数を追加しましょう:

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">ファイル名: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

<!--
Now we can have the `adder` package with our binary depend on the `add_one`
package that has our library. First, we’ll need to add a path dependency on
`add_one` to *adder/Cargo.toml*.
-->

これで、バイナリを持つパッケージ`adder`を、ライブラリを持つパッケージ`add_one`に依存させることができるようになりました。
まず、`add_one`へのパス依存を*adder/Cargo.toml*に追加する必要があります。

<!--
<span class="filename">Filename: adder/Cargo.toml</span>
-->

<span class="filename">ファイル名: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

<!--
Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships.
-->

Cargoはワークスペースのクレートが、お互いに依存しているとは想定していないので、
依存関係について明示する必要があります。

<!--
Next, let’s use the `add_one` function (from the `add_one` crate) in the
`adder` crate. Open the *adder/src/main.rs* file and add a `use` line at the
top to bring the new `add_one` library crate into scope. Then change the `main`
function to call the `add_one` function, as in Listing 14-7.
-->

次に、`adder`クレート内で(`add_one`クレートの)`add_one`関数を使用しましょう。*adder/src/main.rs*ファイルを開き、
冒頭に`use`行を追加して新しい`add_one`ライブラリクレートをスコープに導入してください。
それから`main`関数を変更し、`add_one`関数を呼び出します。リスト14-7のようにですね。

<!--
<span class="filename">Filename: adder/src/main.rs</span>
-->

<span class="filename">ファイル名: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<!--
<span class="caption">Listing 14-7: Using the `add_one` library crate from the
 `adder` crate</span>
-->

<span class="caption">リスト14-7: `adder`クレートから`add_one`ライブラリクレートを使用する</span>

<!--
Let’s build the workspace by running `cargo build` in the top-level *add*
directory!
-->

最上位の*add*ディレクトリで`cargo build`を実行することでワークスペースをビルドしましょう！

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

<!--
To run the binary crate from the *add* directory, we can specify which
package in the workspace we want to run by using the `-p` argument and the
package name with `cargo run`:
-->

*add*ディレクトリからバイナリクレートを実行するには、`-p`引数とパッケージ名を`cargo run`と共に使用して、
実行したいワークスペースのパッケージを指定することができます:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

<!--
This runs the code in *adder/src/main.rs*, which depends on the `add_one` crate.
-->

これにより、*adder/src/main.rs*のコードが実行され、これは`add_one`クレートに依存しています。

<!--
#### Depending on an External Package in a Workspace
-->

#### ワークスペースの外部パッケージに依存する

<!--
Notice that the workspace has only one *Cargo.lock* file at the top level,
rather than having a *Cargo.lock* in each crate’s directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`
package to the *adder/Cargo.toml* and *add_one/Cargo.toml* files, Cargo will
resolve both of those to one version of `rand` and record that in the one
*Cargo.lock*. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let’s add the
`rand` crate to the `[dependencies]` section in the *add_one/Cargo.toml* file
so we can use the `rand` crate in the `add_one` crate:
-->

ワークスペースには、各クレートのディレクトリそれぞれに*Cargo.lock*が存在するのではなく、
最上位階層にただ一つの*Cargo.lock*が存在するだけのことに注目してください。
これにより、全クレートが全依存の同じバージョンを使用していることが確認されます。
`rand`パッケージを*adder/Cargo.toml*と*add_one/Cargo.toml*ファイルに追加すると、
Cargoは両者をあるバージョンの`rand`に解決し、それを一つの*Cargo.lock*に記録します。
ワークスペースの全クレートに同じ依存を使用させるということは、
クレートが相互に互換性を常に維持するということになります。
`add_one`クレートで`rand`クレートを使用できるように、
*add_one/Cargo.toml*ファイルの`[dependencies]`セクションに`rand`クレートを追加しましょう:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<!--
<span class="filename">Filename: add_one/Cargo.toml</span>
-->

<span class="filename">ファイル名: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

<!--
We can now add `use rand;` to the *add_one/src/lib.rs* file, and building the
whole workspace by running `cargo build` in the *add* directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:
-->

これで、*add_one/src/lib.rs*ファイルに`use rand;`を追加でき、
*add*ディレクトリで`cargo build`を実行することでワークスペース全体をビルドすると、
`rand`クレートを持ってきてコンパイルするでしょう。
スコープ内に持ち込んだ`rand`を参照していないので、警告が出るでしょう:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
(警告: 未使用のインポート: `rand`)
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

<!--
The top-level *Cargo.lock* now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* files as well. For example, if we add `use rand;`
to the *adder/src/main.rs* file for the `adder` package, we’ll get an error:
-->

さて、最上位の*Cargo.lock*は、`rand`に対する`add_one`の依存の情報を含むようになりました。
ですが、`rand`はワークスペースのどこかで使用されているにも関わらず、それぞれの*Cargo.toml*ファイルにも、
`rand`を追加しない限り、ワークスペースの他のクレートでそれを使用することはできません。
例えば、`adder`パッケージの*adder/src/main.rs*ファイルに`use rand;`を追加すると、
エラーが出ます:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
(エラー: 未解決のインポート`rand`)
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
  |         (外部クレート`rand`は存在しません)
```

<!--
To fix this, edit the *Cargo.toml* file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured that every
crate in every package in the workspace using the `rand` package will be using
the same version, saving us space and ensuring that the crates in the workspace
will be compatible with each other.
-->

これを修正するには、`adder`パッケージの*Cargo.toml*ファイルを編集し、これも`rand`に依存していることを示してください。
`adder`パッケージをビルドすると、`rand`を*Cargo.lock*の`adder`の依存一覧に追加しますが、
`rand`のファイルが追加でダウンロードされることはありません。Cargoが、
ワークスペース内で`rand`パッケージを使用するすべてのパッケージ内のすべてのクレートが、
同じバージョンを使用することを保証してくれるのです。これによりスペースを節約し、
ワークスペースのクレートが相互に互換性があることを保証してくれます。

<!--
#### Adding a Test to a Workspace
-->

#### ワークスペースにテストを追加する

<!--
For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:
-->

さらなる改善として、`add_one`クレート内に`add_one::add_one`関数のテストを追加しましょう:

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">ファイル名: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

<!--
Now run `cargo test` in the top-level *add* directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:
-->

では、最上位の*add*ディレクトリで`cargo test`を実行してください。
このような構造をしたワークスペースで`cargo test`を走らせると、ワークスペースの全クレートのテストを実行します:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add_one` crate.
-->

出力の最初の区域が、`add_one`クレートの`it_works`テストが通ったことを示しています。
次の区域には、`adder`クレートにはテストが見つからなかったことが示され、
さらに最後の区域には、`add_one`クレートにドキュメンテーションテストは見つからなかったと表示されています。

<!--
We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:
-->

`-p`フラグを使用し、テストしたいクレートの名前を指定することで最上位ディレクトリから、
ワークスペースのある特定のクレート用のテストを実行することもできます:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.
-->

この出力は、`cargo test`が`add_one`クレートのテストのみを実行し、`adder`クレートのテストは実行しなかったことを示しています。

<!--
If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. Like `cargo
test`, we can publish a particular crate in our workspace by using the `-p`
flag and specifying the name of the crate we want to publish.
-->

ワークスペースのクレートを [crates.io](https://crates.io/) に公開したら、ワークスペースのクレートは個別に公開される必要があります。
`cargo test`のように、`-p`フラグを使用して公開したいクレートの名前を指定することで、ワークスペース内の特定のクレートを公開することができます。

<!--
For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!
-->

鍛錬を積むために、`add_one`クレートと同様の方法でワークスペースに`add_two`クレートを追加してください！

<!--
As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between crates easier if they
are often changed at the same time.
-->

プロジェクトが肥大化してきたら、ワークスペースの使用を考えてみてください: 大きな一つのコードの塊よりも、
微細で個別のコンポーネントの方が理解しやすいです。またワークスペースにクレートを保持することは、
同時に変更されることが多いのなら、クレート間の協調をしやすくなることにも繋がります。
