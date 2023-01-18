<!--
## Cargo Workspaces
-->

## Cargo のワークスペース

<!--
In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split up your package further into
multiple library crates. In this situation, Cargo offers a feature called
*workspaces* that can help manage multiple related packages that are developed
in tandem.
-->

第 12 章で、バイナリクレートとライブラリクレートを含むパッケージを構築しました。プロジェクトの開発が進むにつれて、
ライブラリクレートの肥大化が続き、その上で複数のライブラリクレートにパッケージを分割したくなることでしょう。
この場面において、Cargo は*ワークスペース*という協調して開発された関連のある複数のパッケージを管理するのに役立つ機能を提供しています。

<!--
### Creating a Workspace
-->

### ワークスペースを生成する

<!--
A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory. Let’s make a project using a workspace-we'll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace; we’re going to show a common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We'll start by creating
a new directory for the workspace:
-->

*ワークスペース*は、同じ*Cargo.lock*と出力ディレクトリを共有する一連のパッケージです。
ワークスペースを使用したプロジェクトを作成し、ワークスペースの構造に集中できるよう、瑣末なコードを使用しましょう。
ワークスペースを構築する方法は複数ありますが、一般的な方法を提示しましょう。バイナリ 1 つとライブラリ 2 つを含むワークスペースを作ります。
バイナリは、主要な機能を提供しますが、2 つのライブラリに依存しています。
一方のライブラリは、`add_one`関数を提供し、2 番目のライブラリは、`add_two`関数を提供します。
これら 3 つのクレートが同じワークスペースの一部になります。ワークスペース用の新しいディレクトリを作ることから始めましょう：

```text
$ mkdir add
$ cd add
```

<!--
Next, in the *add* directory, we create the *Cargo.toml* file that will
configure the entire workspace. This file won't have a `[package]` section or
the metadata we’ve seen in other *Cargo.toml* files. Instead, it will start
with a `[workspace]` section that will allow us to add members to the workspace
by specifying the path to our binary crate; in this case, that path is *adder*:
-->

次に*add*ディレクトリにワークスペース全体を設定する*Cargo.toml*ファイルを作成します。
このファイルには、他の*Cargo.toml*ファイルで見かけるような`[package]`セクションやメタデータはありません。
代わりにバイナリクレートへのパスを指定することでワークスペースにメンバを追加させてくれる`[workspace]`セクションから開始します;
今回の場合、そのパスは*adder*です：

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名：Cargo.toml</span>

```toml
[workspace]

members = [
    "adder",
]
```

<!--
Next, we’ll create the `adder` binary crate by running `cargo new` within the
*add* directory:
-->

次に、*add*ディレクトリ内で`cargo new`を実行することで`adder`バイナリクレートを作成しましょう：

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
```

<!--
At this point, we can build the workspace by running `cargo build`. The files
in your *add* directory should look like this:
-->

この時点で、`cargo build`を走らせるとワークスペースを構築できます。*add*ディレクトリに存在するファイルは、
以下のようになるはずです：

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
The workspace has one *target* directory at the top level for the compiled
artifacts to be placed into; the `adder` crate doesn’t have its own *target*
directory. Even if we were to run `cargo build` from inside the *adder*
directory, the compiled artifacts would still end up in *add/target* rather
than *add/adder/target*. Cargo structures the *target* directory in a workspace
like this because the crates in a workspace are meant to depend on each other.
If each crate had its own *target* directory, each crate would have to
recompile each of the other crates in the workspace to have the artifacts in
its own *target* directory. By sharing one *target* directory, the crates can
avoid unnecessary rebuilding.
-->

ワークスペースには、コンパイルした生成物を置けるように最上位に*target*のディレクトリがあります;
`adder`クレートには*target*ディレクトリはありません。
*adder*ディレクトリ内部から`cargo build`を走らせることになっていたとしても、コンパイルされる生成物は、
*add/adder/target*ではなく、*add/target*に落ち着くでしょう。ワークスペースのクレートは、
お互いに依存しあうことを意味するので、Cargo はワークスペースの*target*ディレクトリをこのように構成します。
各クレートが*target*ディレクトリを持っていたら、各クレートがワークスペースの他のクレートを再コンパイルし、
*target*ディレクトリに生成物がある状態にしなければならないでしょう。一つの*target*ディレクトリを共有することで、
クレートは不必要な再ビルドを回避できるのです。

<!--
### Creating the Second Crate in the Workspace
-->

### ワークスペース内に 2 番目のクレートを作成する

<!--
Next, let’s create another member crate in the workspace and call it `add-one`.
Change the top-level *Cargo.toml* to specify the *add-one* path in the
`members` list:
-->

次に、ワークスペースに別のメンバクレートを作成し、`add-one`と呼びましょう。
最上位の*Cargo.toml*を変更して`members`リストで*add-one*パスを指定するようにしてください：

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">ファイル名：Cargo.toml</span>

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

<!--
Then generate a new library crate named `add-one`:
-->

それから、`add-one`という名前のライブラリクレートを生成してください：

```text
$ cargo new add-one --lib
     Created library `add-one` project
```

<!--
Your *add* directory should now have these directories and files:
-->

これで*add*ディレクトリには、以下のディレクトリやファイルが存在するはずです：

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
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
In the *add-one/src/lib.rs* file, let’s add an `add_one` function:
-->

*add-one/src/lib.rs*ファイルに`add_one`関数を追加しましょう：

<!--
<span class="filename">Filename: add-one/src/lib.rs</span>
-->

<span class="filename">ファイル名：add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

<!--
Now that we have a library crate in the workspace, we can have the binary crate
`adder` depend on the library crate `add-one`. First, we’ll need to add a path
dependency on `add-one` to *adder/Cargo.toml*:
-->

ワークスペースにライブラリクレートが存在するようになったので、バイナリクレート`adder`をライブラリクレートの`add-one`に依存させられます。
まず、`add-one`へのパス依存を*adder/Cargo.toml*に追加する必要があります：

<!--
<span class="filename">Filename: adder/Cargo.toml</span>
-->

<span class="filename">ファイル名：adder/Cargo.toml</span>

```toml
[dependencies]

add-one = { path = "../add-one" }
```

<!--
Cargo doesn't assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships between the crates.
-->

Cargo はワークスペースのクレートが、お互いに依存しているとは想定していないので、
クレート間の依存関係について明示する必要があります。

<!--
Next, let’s use the `add_one` function from the `add-one` crate in the `adder`
crate. Open the *adder/src/main.rs* file and add an `extern crate` line at
the top to bring the new `add-one` library crate into scope. Then change the
`main` function to call the `add_one` function, as in Listing 14-7:
-->

次に、`adder`クレートの`add-one`クレートから`add_one`関数を使用しましょう。*adder/src/main.rs*ファイルを開き、
冒頭に`extern crate`行を追加して新しい`add-one`ライブラリクレートをスコープに導入してください。
それから`main`関数を変更し、`add_one`関数を呼び出します。リスト 14-7 のようにですね：

<!--
<span class="filename">Filename: adder/src/main.rs</span>
-->

<span class="filename">ファイル名：adder/src/main.rs</span>

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    // こんにちは世界！{}+1 は{}!
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

<!--
<span class="caption">Listing 14-7: Using the `add-one` library crate from the
`adder` crate</span>
-->

<span class="caption">リスト 14-7: `adder`クレートから`add-one`ライブラリクレートを使用する</span>

<!--
Let’s build the workspace by running `cargo build` in the top-level *add*
directory!
-->

最上位の*add*ディレクトリで`cargo build`を実行することでワークスペースをビルドしましょう！

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68 secs
```

<!--
To run the binary crate from the *add* directory, we need to specify which
package in the workspace we want to use by using the `-p` argument and the
package name with `cargo run`:
-->

*add*ディレクトリからバイナリクレートを実行するには、`-p`引数とパッケージ名を`cargo run`と共に使用して、
使用したいワークスペースのパッケージを指定する必要があります：

```text
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

<!--
This runs the code in *adder/src/main.rs*, which depends on the `add-one` crate.
-->

これにより、*adder/src/main.rs*のコードが実行され、これは`add_one`クレートに依存しています。

<!--
#### Depending on an External Crate in a Workspace
-->

#### ワークスペースの外部クレートに依存する

<!--
Notice that the workspace has only one *Cargo.lock* file at the top level of
the workspace rather than having a *Cargo.lock* in each crate’s directory. This
ensures that all crates are using the same version of all dependencies. If we
add the `rand` crate to the *adder/Cargo.toml* and *add-one/Cargo.toml*
files, Cargo will resolve both of those to one version of `rand` and record
that in the one *Cargo.lock*. Making all crates in the workspace use the same
dependencies means the crates in the workspace will always be compatible with
each other. Let’s add the `rand` crate to the `[dependencies]` section in the
*add-one/Cargo.toml* file to be able to use the `rand` crate in the `add-one`
crate:
-->

ワークスペースには、各クレートのディレクトリそれぞれに*Cargo.lock*が存在するのではなく、
ワークスペースの最上位階層にただ一つの*Cargo.lock*が存在するだけのことに注目してください。
これにより、全クレートが全依存の同じバージョンを使用していることが確認されます。
`rand`クレートを*adder/Cargo.toml*と*add-one/Cargo.toml*ファイルに追加すると、
Cargo は両者をあるバージョンの`rand`に解決し、それを一つの*Cargo.lock*に記録します。
ワークスペースの全クレートに同じ依存を使用させるということは、
ワークスペースのクレートが相互に互換性を常に維持するということになります。
*add-one/Cargo.toml*ファイルの`[dependencies]`セクションに`rand`クレートを追加して、
`add-one`クレートで`rand`クレートを使用できます：

<!--
<span class="filename">Filename: add-one/Cargo.toml</span>
-->

<span class="filename">ファイル名：add-one/Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

<!--
We can now add `extern crate rand;` to the *add-one/src/lib.rs* file, and
building the whole workspace by running `cargo build` in the *add* directory
will bring in and compile the `rand` crate:
-->

これで、*add-one/src/lib.rs*ファイルに`extern crate rand;`を追加でき、
*add*ディレクトリで`cargo build`を実行することでワークスペース全体をビルドすると、
`rand`クレートを持ってきてコンパイルするでしょう：

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
   --snip--
   Compiling rand v0.3.14
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18 secs
```

<!--
The top-level *Cargo.lock* now contains information about the dependency of
`add-one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* files as well. For example, if we add `extern
crate rand;` to the *adder/src/main.rs* file for the `adder` crate, we’ll get
an error:
-->

さて、最上位の*Cargo.lock*は、`rand`に対する`add-one`の依存の情報を含むようになりました。
ですが、`rand`はワークスペースのどこかで使用されているにも関わらず、それぞれの*Cargo.toml*ファイルにも、
`rand`を追加しない限り、ワークスペースの他のクレートでそれを使用することはできません。
例えば、`adder`クレートの*adder/src/main.rs*ファイルに`extern crate rand;`を追加すると、
エラーが出ます：

```text
$ cargo build
   Compiling adder v0.1.0 (file:///projects/add/adder)
error: use of unstable library feature 'rand': use `rand` from crates.io (see
issue #27703)
(エラー: 不安定なライブラリの機能'rand'を使用しています：crates.io の`rand`を使用してください)
 --> adder/src/main.rs:1:1
  |
1 | extern crate rand;
```

<!--
To fix this, edit the *Cargo.toml* file for the `adder` crate and indicate that
`rand` is a dependency for that crate as well. Building the `adder` crate will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured that any
crate in the workspace using the `rand` crate will be using the same version.
Using the same version of `rand` across the workspace saves space because we
won’t have multiple copies and ensures that the crates in the workspace will be
compatible with each other.
-->

これを修正するには、`adder`クレートの*Cargo.toml*ファイルを編集し、同様にそのクレートが`rand`に依存していることを示してください。
`adder`クレートをビルドすると、`rand`を*Cargo.lock*の`adder`の依存一覧に追加しますが、
`rand`のファイルが追加でダウンロードされることはありません。Cargo が、ワークスペースの`rand`を使用するどのクレートも、
同じバージョンを使っていることを確かめてくれるのです。ワークスペース全体で`rand`の同じバージョンを使用することにより、
複数のコピーが存在しないのでスペースを節約し、ワークスペースのクレートが相互に互換性を維持することを確かめます。

<!--
#### Adding a Test to a Workspace
-->

#### ワークスペースにテストを追加する

<!--
For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:
-->

さらなる改善として、`add_one`クレート内に`add_one::add_one`関数のテストを追加しましょう：

<!--
<span class="filename">Filename: add-one/src/lib.rs</span>
-->

<span class="filename">ファイル名：add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

<!--
Now run `cargo test` in the top-level *add* directory:
-->

では、最上位の*add*ディレクトリで`cargo test`を実行してください：

```text
$ cargo test
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-f88af9d2cc175a5e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
The first section of the output shows that the `it_works` test in the `add-one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add-one` crate. Running `cargo test` in a workspace structured like this
one will run the tests for all the crates in the workspace.
-->

出力の最初の区域が、`add-one`クレートの`it_works`テストが通ったことを示しています。
次の区域には、`adder`クレートにはテストが見つからなかったことが示され、
さらに最後の区域には、`add-one`クレートにドキュメンテーションテストは見つからなかったと表示されています。
このような構造をしたワークスペースで`cargo test`を走らせると、ワークスペースの全クレートのテストを実行します。

<!--
We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:
-->

`-p`フラグを使用し、テストしたいクレートの名前を指定することで最上位ディレクトリから、
ワークスペースのある特定のクレート用のテストを実行することもできます：

```text
$ cargo test -p add-one
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<!--
This output shows `cargo test` only ran the tests for the `add-one` crate and
didn’t run the `adder` crate tests.
-->

この出力は、`cargo test`が`add-one`クレートのテストのみを実行し、`adder`クレートのテストは実行しなかったことを示しています。

<!--
If you publish the crates in the workspace to *https://crates.io/*, each crate
in the workspace will need to be published separately. The `cargo publish`
command does not have an `--all` flag or a `-p` flag, so you must change to
each crate’s directory and run `cargo publish` on each crate in the workspace
to publish the crates.
-->

ワークスペースのクレートを *https://crates.io/* に公開したら、ワークスペースのクレートは個別に公開される必要があります。
`cargo publish`コマンドには`--all`フラグや`-p`フラグはないので、各クレートのディレクトリに移動して、
ワークスペースの各クレートを`cargo publish`して、公開しなければなりません。

<!--
For additional practice, add an `add-two` crate to this workspace in a similar
way as the `add-one` crate!
-->

鍛錬を積むために、`add-one`クレートと同様の方法でワークスペースに`add-two`クレートを追加してください！

<!--
As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between them easier if they are
often changed at the same time.
-->

プロジェクトが肥大化してきたら、ワークスペースの使用を考えてみてください：大きな一つのコードの塊よりも、
微細で個別のコンポーネントの方が理解しやすいです。またワークスペースにクレートを保持することは、
同時に変更されることが多いのなら、協調しやすくなることにも繋がります。
