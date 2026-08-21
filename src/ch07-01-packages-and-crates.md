<!--
## Packages and Crates
-->
## パッケージとクレート

<!--
The first parts of the module system we’ll cover are packages and crates.
-->
最初に学ぶモジュールシステムの要素は、パッケージとクレートです。

<!--
A *crate* is the smallest amount of code that the Rust compiler considers at a
time. Even if you run `rustc` rather than `cargo` and pass a single source code
file (as we did all the way back in the “Writing and Running a Rust Program”
section of Chapter 1), the compiler considers that file to be a crate. Crates
can contain modules, and the modules may be defined in other files that get
compiled with the crate, as we’ll see in the coming sections.
-->
*クレート (crate)* は、Rustコンパイラが一度に考慮する最小限のコードです。
`cargo`を実行するのではなく、（ちょうど第1章の「Rustプログラムを書いて走らせる」節でやったように）単一のソースコードファイルを渡して`rustc`を実行した場合でも、
コンパイラはそのファイルをクレートとしてみなします。
クレートはモジュールを含むことができ、モジュールは、クレートとともにコンパイルされる他のファイル内で定義することができます。
このことについてはこれからの節で見ていきます。

<!--
A crate can come in one of two forms: a binary crate or a library crate.
*Binary crates* are programs you can compile to an executable that you can run,
such as a command-line program or a server. Each must have a function called
`main` that defines what happens when the executable runs. All the crates we’ve
created so far have been binary crates.
-->
クレートは2種類の形態のうちいずれかです: バイナリクレートか、ライブラリクレートです。
*バイナリクレート (binary crate)* は、コマンドラインプログラムやサーバなどの実行可能形式にコンパイルされ、実行することができるプログラムです。
どのバイナリクレートも、その実行可能形式が実行されたときに何が起きるかを定義する`main`と呼ばれる関数を持たなくてはなりません。
今まで作ってきたクレートはすべてバイナリクレートでした。

<!--
*Library crates* don’t have a `main` function, and they don’t compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. For example, the `rand` crate we used in [Chapter
2][rand] provides functionality that generates random numbers.
Most of the time when Rustaceans say “crate”, they mean library crate, and they
use “crate” interchangeably with the general programming concept of a “library".
-->
*ライブラリクレート (library crate)* は`main`関数を持たず、実行可能形式へとコンパイルされません。
代わりに、複数のプロジェクトで共有されることを想定した機能を定義するものです。
例えば、[第2章][rand]で使用した`rand`クレートは乱数を生成する機能を提供します。
Rustaceanが「クレート」と言うときの、ほとんどの場合それはライブラリクレートのことであり、
Rustaceanは「クレート」を一般的なプログラミング概念の「ライブラリ」と同じ意味で使用します。

<!--
The *crate root* is a source file that the Rust compiler starts from and makes
up the root module of your crate (we’ll explain modules in depth in the
[“Defining Modules to Control Scope and Privacy”][modules]
section).
-->
*クレートルート (crate root)* とは、Rustコンパイラの開始点となり、クレートのルートモジュールを作るソースファイルのことです（モジュールについて詳しくは[「モジュールを定義して、スコープとプライバシーを制御する」][modules]のセクションで説明します）。

<!--
A *package* is a bundle of one or more crates that provides a set of
functionality. A package contains a *Cargo.toml* file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command-line tool you’ve been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command-line tool uses.
-->
*パッケージ (package)* はある機能群を提供する1つ以上のクレートのまとまりです。
パッケージは *Cargo.toml* という、それらのクレートをどのようにビルドするかを説明するファイルを持っています。
Cargoは実のところパッケージで、今までコードをビルドするために使ってきたコマンドラインツールのためのバイナリクレートを含んでいます。
Cargoパッケージは、このバイナリクレートが依存するライブラリクレートも含んでいます。
他のプロジェクトはCargoライブラリクレートに依存することで、Cargoコマンドラインツールが使用するのと同じロジックを使用することもできます。

<!--
A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that’s a
library or binary crate.
-->
パッケージは好きなだけバイナリクレートを持つことができますが、ライブラリクレートは最大で1個しか持つことができません。
パッケージはライブラリクレートかバイナリクレートを問わず、少なくとも1個のクレートを持っていないといけません。

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
After we run `cargo new`, we use `ls` to see what Cargo creates. In the project
directory, there’s a *Cargo.toml* file, giving us a package. There’s also a
*src* directory that contains *main.rs*. Open *Cargo.toml* in your text editor,
and note there’s no mention of *src/main.rs*. Cargo follows a convention that
*src/main.rs* is the crate root of a binary crate with the same name as the
package. Likewise, Cargo knows that if the package directory contains
*src/lib.rs*, the package contains a library crate with the same name as the
package, and *src/lib.rs* is its crate root. Cargo passes the crate root files
to `rustc` to build the library or binary.
-->
`cargo new`を実行した後、Cargoが作成したものを確認するために`ls`を使っています。
プロジェクトディレクトリ内には *Cargo.toml* ファイルがあり、これがパッケージを構成します。
また、*main.rs* を含む *src* ディレクトリもあります。
*Cargo.toml* をテキストエディタで開くと、*src/main.rs* については何も書いていないことに気づくでしょう。
Cargoは *src/main.rs* が、パッケージと同じ名前を持つバイナリクレートのクレートルートであるという慣習に従っています。
同じように、Cargoはパッケージディレクトリに *src/lib.rs* が含まれていたら、パッケージにはパッケージと同じ名前のライブラリクレートが含まれており、*src/lib.rs* がそのクレートルートなのだと判断します。
Cargoはクレートルートファイルを `rustc`に渡し、ライブラリやバイナリをビルドします。

<!--
Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.
-->
今、このパッケージには *src/main.rs* しか含まれておらず、つまりこのパッケージは`my-project`という名前のバイナリクレートのみを持っているということです。
もしパッケージが *src/main.rs* と *src/lib.rs* を持っていたら、クレートは2つになります：どちらもパッケージと同じ名前を持つ、バイナリクレートとライブラリクレートです。
ファイルを *src/bin* ディレクトリに置くことで、パッケージは複数のバイナリクレートを持つことができます。それぞれのファイルが別々のバイナリクレートになります。

<!--
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
-->
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#乱数を生成する
