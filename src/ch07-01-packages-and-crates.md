<!--
## Packages and Crates
-->
## パッケージとクレート

<!--
The first parts of the module system we’ll cover are packages and crates. A
crate is a binary or library. The *crate root* is a source file that the Rust
compiler starts from and makes up the root module of your crate (we’ll explain
modules in depth in the [“Defining Modules to Control Scope and
Privacy”][modules] section). A *package* is one or more crates
that provide a set of functionality. A package contains a *Cargo.toml* file
that describes how to build those crates.
-->
最初に学ぶモジュールシステムの要素は、パッケージとクレートです。
クレートはバイナリかライブラリのどちらかです。
*クレートルート (crate root)* とは、Rust コンパイラの開始点となり、クレートのルートモジュールを作るソースファイルのことです（モジュールについて詳しくは[「モジュールを定義して、スコープとプライバシーを制御する」][modules]<!-- ignore -->のセクションで説明します）。
*パッケージ* はある機能群を提供する 1 つ以上のクレートです。
パッケージは *Cargo.toml* という、それらのクレートをどのようにビルドするかを説明するファイルを持っています。

<!--
Several rules determine what a package can contain. A package *must* contain
zero or one library crates, and no more. It can contain as many binary crates
as you’d like, but it must contain at least one crate (either library or
binary).
-->
パッケージが何を持ってよいかはいくつかのルールで決まっています。
パッケージは 0 個か 1 個のライブラリクレートを持っていないといけません。それ以上は駄目です。
バイナリクレートはいくらでも持って良いですが、少なくとも（ライブラリでもバイナリでも良いですが）1 つのクレートを持っていないといけません。

<!--
Let’s walk through what happens when we create a package. First, we enter the
command `cargo new`:
-->
パッケージを作る時に何が起こるか見てみましょう。
まず、`cargo new`というコマンドを入力します：

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

<!--
When we entered the command, Cargo created a *Cargo.toml* file, giving us a
package. Looking at the contents of *Cargo.toml*, there’s no mention of
*src/main.rs* because Cargo follows a convention that *src/main.rs* is the
crate root of a binary crate with the same name as the package. Likewise, Cargo
knows that if the package directory contains *src/lib.rs*, the package contains
a library crate with the same name as the package, and *src/lib.rs* is its
crate root. Cargo passes the crate root files to `rustc` to build the library
or binary.
-->
このコマンドを入力したとき、Cargo は *Cargo.toml* ファイルを作り、パッケージを作ってくれました。
*Cargo.toml* の中身を見ても、*src/main.rs* については何も書いてありません。これは、Cargo は *src/main.rs* が、パッケージと同じ名前を持つバイナリクレートのクレートルートであるという慣習に従っているためです。
同じように、Cargo はパッケージディレクトリに *src/lib.rs* が含まれていたら、パッケージにはパッケージと同じ名前のライブラリクレートが含まれており、*src/lib.rs* がそのクレートルートなのだと判断します。
Cargo はクレートルートファイルを `rustc`に渡し、ライブラリやバイナリをビルドします。

<!--
Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a library and a binary, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.
-->
今、このパッケージには *src/main.rs* しか含まれておらず、つまりこのパッケージは`my-project`という名前のバイナリクレートのみを持っているということです。
もしパッケージが *src/main.rs* と *src/lib.rs* を持っていたら、クレートは 2 つになります：どちらもパッケージと同じ名前を持つ、ライブラリクレートとバイナリクレートです。
ファイルを *src/bin* ディレクトリに置くことで、パッケージは複数のバイナリクレートを持つことができます。それぞれのファイルが別々のバイナリクレートになります。


<!--
A crate will group related functionality together in a scope so the
functionality is easy to share between multiple projects. For example, the
`rand` crate we used in [Chapter 2][rand] provides functionality
that generates random numbers. We can use that functionality in our own
projects by bringing the `rand` crate into our project’s scope. All the
functionality provided by the `rand` crate is accessible through the crate’s
name, `rand`.
-->
クレートは、関連した機能を一つのスコープにまとめることで、その機能が複数のプロジェクト間で共有しやすいようにします。
例えば、[2 章][rand]で使った`rand`クレートは、乱数を生成する機能を提供します。
`rand`クレートを私達のプロジェクトのスコープに持ち込むことで、この機能を私達のプロジェクトで使うことができます。
`rand`クレートが提供する機能にはすべて、クレートの名前`rand`を使ってアクセスできます。

<!--
Keeping a crate’s functionality in its own scope clarifies whether particular
functionality is defined in our crate or the `rand` crate and prevents
potential conflicts. For example, the `rand` crate provides a trait named
`Rng`. We can also define a `struct` named `Rng` in our own crate. Because a
crate’s functionality is namespaced in its own scope, when we add `rand` as a
dependency, the compiler isn’t confused about what the name `Rng` refers to. In
our crate, it refers to the `struct Rng` that we defined. We would access the
`Rng` trait from the `rand` crate as `rand::Rng`.
-->
クレートの機能をそれ自身のスコープの中に入れたままにしておくことは、ある機能が私達のクレートで定義されたのか`rand`クレートで定義されたのかを明確にし、名前の衝突を予防してくれます。
例えば、`rand`クレートは`Rng`という名前のトレイトを提供しています。
更に、私達のクレートで`Rng`という名前の`struct`を定義することもできます。
クレートの機能はそのスコープ内の名前空間に位置づけられているので、`rand`を依存先として追加しても、コンパイラは`Rng`という名前が何を意味するのかについて混乱することはないのです。
私達のクレートでは、私達の定義した`struct Rng`のことであり、`rand`クレートの`Rng`トレイトには`rand::Rng`でアクセスするというわけです。

<!--
Let’s move on and talk about the module system!
-->
では、モジュールシステムの話に移りましょう！

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#乱数を生成する
