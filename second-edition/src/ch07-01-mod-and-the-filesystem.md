<!-- ## `mod` and the Filesystem -->

## `mod`とファイルシステム

<!-- We’ll start our module example by making a new project with Cargo, but instead -->
<!-- of creating a binary crate, we’ll make a library crate: a project that other -->
<!-- people can pull into their projects as a dependency. For example, the `rand` -->
<!-- crate discussed in Chapter 2 is a library crate that we used as a dependency in -->
<!-- the guessing game project. -->

モジュールの例をCargoで新規プロジェクトを生成することから始めるが、バイナリクレートの代わりに、
ライブラリクレートを作成します: 他人が依存として自分のプロジェクトに引き込めるプロジェクトです。
例を挙げると、第2章で議論した`rand`クレートは、数当てゲームプロジェクトで依存に使用したライブラリクレートです。

<!-- We’ll create a skeleton of a library that provides some general networking -->
<!-- functionality; we’ll concentrate on the organization of the modules and -->
<!-- functions, but we won’t worry about what code goes in the function bodies. -->
<!-- We'll call our library `communicator`. To create a library, pass the `--lib` -->
<!-- option instead of `--bin`: -->

何らかの一般的なネットワーク機能を提供するライブラリの骨格を作成します; モジュールと関数の体系化に集中し、
関数の本体にどんなコードが入るかについては気にかけません。このライブラリを`communicator`と呼びましょう。
ライブラリを生成するために、`--bin`の代わりに`--lib`オプションを渡してください:

```text
$ cargo new communicator --lib
$ cd communicator
```

<!-- Notice that Cargo generated *src/lib.rs* instead of *src/main.rs*. Inside -->
<!-- *src/lib.rs* we’ll find the following: -->

Cargoが*src/main.rs*の代わりに*src/lib.rs*を生成したことに注目してください。*src/lib.rs*には、
以下のような記述があります:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

<!-- Cargo creates an empty test to help us get our library started, rather than the -->
<!-- the “Hello, world!” binary that we get when we use the `--bin` option. We’ll -->
<!-- look at the `#[]` and `mod tests` syntax in the “Using `super` to Access a -->
<!-- Parent Module” section later in this chapter, but for now, leave this code at -->
<!-- the bottom of *src/lib.rs*. -->

Cargoは、`--bin`オプションを使った時に得られる"Hello, world!"バイナリではなく、空のテストを生成して、
ライブラリの事始めをしてくれました。`#[]`と`mod tests`という記法については、この章の後ほど、
「`super`を使用して親モジュールにアクセスする」節で見ますが、今のところは、
このコードを*src/lib.rs*の最後に残しておきましょう。

<!-- Because we don’t have a *src/main.rs* file, there’s nothing for Cargo to -->
<!-- execute with the `cargo run` command. Therefore, we’ll use the `cargo build` -->
<!-- command to compile our library crate’s code. -->

*src/main.rs*ファイルがないので、`cargo run`コマンドでCargoが実行できるものは何もないわけです。
従って、`cargo build`コマンドを使用してライブラリクレートのコードをコンパイルします。

<!-- We’ll look at different options for organizing your library’s code that will be -->
<!-- suitable in a variety of situations, depending on the intent of the code. -->

コードの意図によって、いろんなシチュエーションで最適になるライブラリコードを体系化する別のオプションをお目にかけます。

<!-- ### Module Definitions -->

### モジュール定義

<!-- For our `communicator` networking library, we’ll first define a module named -->
<!-- `network` that contains the definition of a function called `connect`. Every -->
<!-- module definition in Rust starts with the `mod` keyword. Add this code to the -->
<!-- beginning of the *src/lib.rs* file, above the test code: -->

`communicator`ネットワークライブラリについて、まずは`connect`という関数定義を含む`network`という名前のモジュールを定義します。
Rustにおいて、モジュール定義は全て、`mod`キーワードから開始します。このコードを*src/lib.rs*ファイルの頭、
テストコードの上に追記してください。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

<!-- After the `mod` keyword, we put the name of the module, `network`, and then a -->
<!-- block of code in curly brackets. Everything inside this block is inside the -->
<!-- namespace `network`. In this case, we have a single function, `connect`. If we -->
<!-- wanted to call this function from a script outside the `network` module, we -->
<!-- would need to specify the module and use the namespace syntax `::`, like so: -->
<!-- `network::connect()`. -->

`mod`キーワードに続いて、モジュール名の`network`、さらに一連のコードを波かっこ内に記述します。
このブロック内に存在するものは全て、`network`という名前空間に属します。今回の場合、
`connect`という単独の関数があります。この関数を`network`モジュール外のスクリプトから呼び出したい場合、
モジュールを指定し、以下のように名前空間記法の`::`を使用する必要があるでしょう:
`network::connect()`。

<!-- We can also have multiple modules, side by side, in the same *src/lib.rs* file. -->
<!-- For example, to also have a `client` module that has a function named -->
<!-- `connect`, we can add it as shown in Listing 7-1: -->

同じ*src/lib.rs*ファイル内に複数のモジュールを並べることもできます。例として、
`connect`という関数を含む`client`モジュールも用意するには、リスト7-1に示したように追記すればいいわけです:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

<!-- <span class="caption">Listing 7-1: The `network` module and the `client` module -->
<!-- defined side by side in *src/lib.rs*</span> -->

<span class="caption">リスト7-1: *src/lib.rs*に並べて定義された`network`モジュールと`client`モジュール</span>

<!-- Now we have a `network::connect` function and a `client::connect` function. -->
<!-- These can have completely different functionality, and the function names do -->
<!-- not conflict with each other because they’re in different modules. -->

これで、`network::connect`関数と`client::connect`関数が用意できました。これらは全く異なる機能を有する可能性があり、
異なるモジュールに存在するので、関数名がお互いに衝突することはありません。

<!-- In this case, because we’re building a library, the file that serves as the -->
<!-- entry point for building our library is *src/lib.rs*. However, in respect to -->
<!-- creating modules, there’s nothing special about *src/lib.rs*. We could also -->
<!-- create modules in *src/main.rs* for a binary crate in the same way as we’re -->
<!-- creating modules in *src/lib.rs* for the library crate. In fact, we can put -->
<!-- modules inside of modules, which can be useful as your modules grow to keep -->
<!-- related functionality organized together and separate functionality apart. The -->
<!-- way you choose to organize your code depends on how you think about the -->
<!-- relationship between the parts of your code. For instance, the `client` code -->
<!-- and its `connect` function might make more sense to users of our library if -->
<!-- they were inside the `network` namespace instead, as in Listing 7-2: -->

今回の場合、ライブラリを構成しているので、ライブラリビルド時にエントリーポイントとなるファイルは、
*src/lib.rs*になります。しかし、モジュールを作成するという点に関しては、*src/lib.rs*には何も特別なことはありません。
ライブラリクレートに対して*src/lib.rs*にモジュールを生成するのと全く同様に、
バイナリクレートに対して*src/main.rs*にモジュールを生成することもできます。実は、モジュール内にモジュールを書くこともでき、
モジュールが肥大化するにつれて、関連のある機能を一緒くたにし、機能を切り離すのに有用なのです。
コードを体系化すると選択する方法は、コードの部分部分の関連性に対する考え方によって選択することになります。
例ですが、`client`コードとその`connect`関数は、リスト7-2のように、代わりに`network`名前空間内に存在したら、
ライブラリの使用者にとって意味のあるものになるかもしれません。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

<!-- <span class="caption">Listing 7-2: Moving the `client` module inside the -->
<!-- `network` module</span> -->

<span class="caption">リスト7-2: `client`モジュールを`network`モジュール内に移動させる</span>

<!-- In your *src/lib.rs* file, replace the existing `mod network` and `mod client` -->
<!-- definitions with the ones in Listing 7-2, which have the `client` module as an -->
<!-- inner module of `network`. The functions `network::connect` and -->
<!-- `network::client::connect` are both named `connect`, but they don’t conflict -->
<!-- with each other because they’re in different namespaces. -->

*src/lib.rs*ファイル内で、すでにある`mod network`と`mod client`の定義をリスト7-2のものと置き換えると、
`client`モジュールは`network`の内部モジュールになるわけです。関数、
`network::connect`と`network::client::connect`はどちらも`connect`という名前ですが、
異なる名前空間にあるので、互いに干渉することはありません。

<!-- In this way, modules form a hierarchy. The contents of *src/lib.rs* are at the -->
<!-- topmost level, and the submodules are at lower levels. Here’s what the -->
<!-- organization of our example in Listing 7-1 looks like when thought of as a -->
<!-- hierarchy: -->

このように、モジュールは階層構造を形成します。*src/lib.rs*の中身が頂点に立ち、サブモジュールが子供になるわけです。
リスト7-1の例を階層構造という観点で見たときの構造は、以下のような感じになります:

```text
communicator
 ├── network
 └── client
```

<!-- And here’s the hierarchy corresponding to the example in Listing 7-2: -->

さらに、リスト7-2の例に対応する階層構造は、以下の通りです:

```text
communicator
 └── network
     └── client
```

<!-- The hierarchy shows that in Listing 7-2, `client` is a child of the `network` -->
<!-- module rather than a sibling. More complicated projects can have many modules, -->
<!-- and they’ll need to be organized logically in order fro you to keep track of -->
<!-- them. What “logically” means in your project is up to you and depends on how -->
<!-- you and your library’s users think about your project’s domain. Use the -->
<!-- techniques shown here to create side-by-side modules and nested modules in -->
<!-- whatever structure you would like. -->

この階層構造は、リスト7-2において、`client`モジュールは`network`モジュールの兄弟というよりも、子供になっていることを示しています。
より複雑なプロジェクトなら、たくさんのモジュールが存在し、把握するのに論理的に体系化しておく必要があるでしょう。
プロジェクト内で「論理的」とは、あなた次第であり、ライブラリ作成者と使用者がプロジェクトの領域についてどう考えるか次第でもあるわけです。
こちらで示したテクニックを使用して、並列したモジュールや、ネストしたモジュールなど、どんな構造のモジュールでも、
作成してください。

<!-- ### Moving Modules to Other Files -->

### モジュールを別ファイルに移す

<!-- Modules form a hierarchical structure, much like another structure in computing -->
<!-- that you’re used to: filesystems! We can use Rust’s module system along with -->
<!-- multiple files to split up Rust projects so not everything lives in -->
<!-- *src/lib.rs* or *src/main.rs*. For this example, let’s start with the code in -->
<!-- Listing 7-3: -->

モジュールは階層構造をなす……コンピュータにおいて、もっと見慣れた構造に似ていませんか: そう、ファイルシステムです！
Rustのモジュールシステムを複数のファイルで使用して、プロジェクトを分割するので、
全部が*src/lib.rs*や*src/main.rs*に存在することにはならなくなります。これの例として、
リスト7-3のようなコードから始めましょう:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<!-- <span class="caption">Listing 7-3: Three modules, `client`, `network`, and -->
<!-- `network::server`, all defined in *src/lib.rs*</span> -->

<span class="caption">リスト7-3: 全て*src/lib.rs*に定義された三つのモジュール、
`client`、`network`、`network::server`</span>

<!-- The file *src/lib.rs* has this module hierarchy: -->

*src/lib.rs*ファイルのモジュール階層は、こうなっています:

```text
communicator
 ├── client
 └── network
     └── server
```

<!-- If these modules had many functions, and those functions were becoming lengthy, -->
<!-- it would be difficult to scroll through this file to find the code we wanted to -->
<!-- work with. Because the functions are nested inside one or more mod blocks, -->
<!-- the lines of code inside the functions will start getting lengthy as well. -->
<!-- These would be good reasons to separate the `client`, `network`, and `server` -->
<!-- modules from *src/lib.rs* and place them into their own files. -->

これらのモジュールが多数の関数を含み、その関数が長ったらしくなってきたら、このファイルをスクロールして、
弄りたいコードを探すのが困難になるでしょう。関数が一つ以上のmodブロックにネストされているので、
関数の中身となるコードも長ったらしくなってしまうのです。これだけで、`client`、`network`、`server`モジュールを*src/lib.rs*から分け、
単独のファイルに配置するには十分でしょう。

<!-- First, let's replace the `client` module code with only the declaration of the -->
<!-- `client` module so that your *src/lib.rs* looks like code shown in Listing 7-4: -->

最初に、`client`モジュールのコードを`client`モジュールの宣言だけに置き換えましょう。
すると、*src/lib.rs*はリスト7-4のコードのようになります:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<!-- <span class="caption">Listing 7-4: Extracting the contents of the `client` module but leaving the declaration in *src/lib.rs*</span> -->

<span class="caption">リスト7-4: `client`モジュールの中身を抽出するが、宣言は*src/lib.rs*に残したまま</span>

<!-- We’re still *declaring* the `client` module here, but by replacing the block -->
<!-- with a semicolon, we’re telling Rust to look in another location for the code -->
<!-- defined within the scope of the `client` module. In other words, the line `mod -->
<!-- client;` means this: -->

一応、`client`モジュールをここで*宣言*していますが、ブロックをセミコロンで置換したことで、
`client`モジュールのスコープのコードは別の場所を探すようにコンパイラに指示しているわけです。
言い換えると、`mod client;`の行は、以下のような意味になります:

```rust,ignore
mod client {
    // contents of client.rs
}
```

<!-- Now we need to create the external file with that module name. Create a -->
<!-- *client.rs* file in your *src/* directory and open it. Then enter the -->
<!-- following, which is the `connect` function in the `client` module that we -->
<!-- removed in the previous step: -->

<!-- 何故か「*src/*ディレクトリ内に*client.rs*」までが斜体になってしまう。mdbookのバグ？ -->

さて、このモジュール名の外部ファイルを作成する必要が出てきました。*src/*ディレクトリ内に*client.rs*ファイルを作成し、
開いてください。それから以下のように入力してください。前段階で削除した`client`モジュールの`connect`関数です:

<!-- <span class="filename">Filename: src/client.rs</span> -->

<span class="filename">ファイル名: src/client.rs</span>

```rust
fn connect() {
}
```

<!-- Note that we don’t need a `mod` declaration in this file because we already -->
<!-- declared the `client` module with `mod` in *src/lib.rs*. This file just -->
<!-- provides the *contents* of the `client` module. If we put a `mod client` here, -->
<!-- we’d be giving the `client` module its own submodule named `client`! -->

このファイルには、`mod`宣言が必要ないことに着目してください。なぜなら、*src/lib.rs*に`mod`を使って、
もう`client`モジュールを宣言しているからです。このファイルは、`client`モジュールの*中身*を提供するだけなのです。
ここにも`mod client`を記述したら、`client`に`client`という名前のサブモジュールを与えることになってしまいます！

<!-- Rust only knows to look in *src/lib.rs* by default. If we want to add more -->
<!-- files to our project, we need to tell Rust in *src/lib.rs* to look in other -->
<!-- files; this is why `mod client` needs to be defined in *src/lib.rs* and can’t -->
<!-- be defined in *src/client.rs*. -->

コンパイラは、標準で*src/lib.rs*だけを検索します。プロジェクトにもっとファイルを追加したかったら、
*src/lib.rs*で他のファイルも検索するよう、コンパイラに指示する必要があるのです; このため、`mod client`を*src/lib.rs*に定義し、
*src/client.rs*には定義できなかったのです。

<!-- Now the project should compile successfully, although you’ll get a few -->
<!-- warnings. Remember to use `cargo build` instead of `cargo run` because we have -->
<!-- a library crate rather than a binary crate: -->

これでプロジェクトは問題なくコンパイルできるはずです。まあ、警告がいくつか出るんですが。
`cargo run`ではなく、`cargo build`を使うことを忘れないでください。バイナリクレートではなく、
ライブラリクレートだからですね:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`
(警告: 関数は使用されていません: `connect`)
 --> src/client.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/lib.rs:4:5
  |
4 | /     fn connect() {
5 | |     }
  | |_____^

warning: function is never used: `connect`
 --> src/lib.rs:8:9
  |
8 | /         fn connect() {
9 | |         }
  | |_________^
```

<!-- These warnings tell us that we have functions that are never used. Don’t worry -->
<!-- about these warnings for now; we’ll address them in this chapter in the  -->
<!-- "Controlling Visibility with `pub`” section. The good news is that they’re just -->
<!-- warnings; our project built successfully! -->

これらの警告は、全く使用されていない関数があると忠告してくれています。今は、警告を危惧する必要はありません;
この章の後ほど、「`pub`で公開するか制御する」節で特定します。嬉しいことにただの警告です;
プロジェクトはビルドに成功しました！

<!-- Next, let’s extract the `network` module into its own file using the same -->
<!-- pattern. In *src/lib.rs*, delete the body of the `network` module and add a -->
<!-- semicolon to the declaration, like so: -->

次に、同様のパターンを使用して`network`モジュールも単独のファイルに抽出しましょう。
*src/lib.rs*で、`network`モジュールの本体を削除し、宣言にセミコロンを付加してください。こんな感じです:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

<!-- Then create a new *src/network.rs* file and enter the following: -->

それから新しい*src/network.rs*ファイルを作成して、以下のように入力してください:

<!-- <span class="filename">Filename: src/network.rs</span> -->

<span class="filename">ファイル名: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

<!-- Notice that we still have a `mod` declaration within this module file; this is -->
<!-- because we still want `server` to be a submodule of `network`. -->

このモジュールファイル内にもまだ`mod`宣言があることに気付いてください; 
`server`はまだ`network`のサブモジュールにしたいからです。

<!-- Run `cargo build` again. Success! We have one more module to extract: `server`. -->
<!-- Because it’s a submodule—that is, a module within a module—our current tactic -->
<!-- of extracting a module into a file named after that module won’t work. We’ll -->
<!-- try anyway so you can see the error. First, change *src/network.rs* to have -->
<!-- `mod server;` instead of the `server` module’s contents: -->

再度`cargo build`してください。成功！抽出すべきモジュールがもう1個あります: `server`です。
これはサブモジュール(つまり、モジュール内のモジュール)なので、
モジュール名に倣ったファイルにモジュールを抽出するという今の手法は、通用しません。いずれにしても、
エラーが確認できるように、試してみましょう。まず、*src/network.rs*ファイルを`server`モジュールの中身を含む代わりに、
`mod server;`となるように変更してください。

<!-- <span class="filename">Filename: src/network.rs</span> -->

<span class="filename">ファイル名: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

<!-- Then create a *src/server.rs* file and enter the contents of the `server` -->
<!-- module that we extracted: -->

そして、*src/server.rs*ファイルを作成し、抽出した`server`モジュールの中身を入力してください:

<!-- <span class="filename">Filename: src/server.rs</span> -->

<span class="filename">ファイル名: src/server.rs</span>

```rust
fn connect() {
}
```

<!-- When we try to `cargo build`, we’ll get the error shown in Listing 7-5: -->

`cargo build`を試すと、リスト7-4に示したようなエラーが出ます:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
(エラー: この箇所では新規モジュールを宣言できません)
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `src/network.rs` to its own directory via `src/network/mod.rs`
(注釈: もしかして、`src/network.rs`というこのモジュールを`src/network/mod.rs`経由で独自のディレクトリに移すの)
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
(注釈: それとも、再度宣言する可能性はなく、`server`というモジュールを`use`したの)
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

<!-- <span class="caption">Listing 7-5: Error when trying to extract the `server` -->
<!-- submodule into *src/server.rs*</span> -->

<span class="caption">リスト7-5: `server`サブモジュールを*src/server.rs*に抽出しようとしたときのエラー</span>

<!-- The error says we `cannot declare a new module at this location` and is -->
<!-- pointing to the `mod server;` line in *src/network.rs*. So *src/network.rs* is -->
<!-- different than *src/lib.rs* somehow: keep reading to understand why. -->

エラーは、`この箇所では新規モジュールを宣言できません`と忠告し、*src/network.rs*の`mod server;`行を指し示しています。
故に、*src/network.rs*は、*src/lib.rs*と何かしら違うのです: 理由を知るために読み進めましょう。

<!-- The note in the middle of Listing 7-5 is actually very helpful because it -->
<!-- points out something we haven’t yet talked about doing: -->

リスト7-5の真ん中の注釈は、非常に有用です。というのも、まだ話題にしていないことを指摘しているからです。

```text
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```

<!-- Instead of continuing to follow the same file-naming pattern we used -->
<!-- previously, we can do what the note suggests: -->

以前行ったファイル命名パターンに従い続けるのではなく、注釈が提言していることをすることができます:

<!-- 1. Make a new *directory* named *network*, the parent module’s name. -->
<!-- 2. Move the *src/network.rs* file into the new *network* directory and -->
<!--    rename it *src/network/mod.rs*. -->
<!-- 3. Move the submodule file *src/server.rs* into the *network* directory. -->

1. 親モジュール名である*network*という名前の新規*ディレクトリ*を作成する。
2. *src/network.rs*ファイルを*network*ディレクトリに移し、
   *src/network/mod.rs*と名前を変える。
3. サブモジュールファイルの*src/server.rs*を*network*ディレクトリに移す。

<!-- Here are commands to carry out these steps: -->

以下が、これを実行するコマンドです:

```text
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

<!-- Now when we try to run `cargo build`, compilation will work (we’ll still have -->
<!-- warnings though). Our module layout still looks exactly the same as it did when -->
<!-- we had all the code in *src/lib.rs* in Listing 7-3: -->

`cargo build`を走らせたら、ようやくコンパイルは通ります(まだ警告はありますけどね)。
それでも、モジュールの配置は、リスト7-3で*src/lib.rs*に全てのコードを収めていたときと全く同じになります:

```text
communicator
 ├── client
 └── network
     └── server
```

<!-- The corresponding file layout now looks like this: -->

対応するファイルの配置は、以下のようになっています:

```text
└── src
    ├── client.rs
    ├── lib.rs
    └── network
        ├── mod.rs
        └── server.rs
```

<!-- So when we wanted to extract the `network::server` module, why did we have to -->
<!-- also change the *src/network.rs* file to the *src/network/mod.rs* file and put -->
<!-- the code for `network::server` in the *network* directory in -->
<!-- *src/network/server.rs*? Why couldn't we just extract the `network::server` -->
<!-- module into *src/server.rs*? The reason is that Rust wouldn’t be able to -->
<!-- recognize that `server` was supposed to be a submodule of `network` if the -->
<!-- *server.rs* file was in the *src* directory. To clarify Rust’s behavior here, -->
<!-- let’s consider a different example with the following module hierarchy, where -->
<!-- all the definitions are in *src/lib.rs*: -->

では、`network::server`モジュールを抽出したかったときに、
なぜ、*src/network.rs*ファイルを*src/network/mod.rs*ファイルに変更し、
`network::server`のコードを*network*ディレクトリ内の*src/network/server.rs*に置かなければならなかったのでしょうか？
なぜ、単に`network::server`モジュールを*src/server.rs*に抽出できなかったのでしょうか？
理由は、*server.rs*ファイルが*src*ディレクトリにあると、コンパイラが、
`server`は`network`のサブモジュールと考えられることを検知できないからです。
ここでのコンパイラの動作をはっきりさせるために、以下のようなモジュール階層をもつ別の例を考えましょう。
こちらでは、定義は全て*src/lib.rs*に存在します。

```text
communicator
 ├── client
 └── network
     └── client
```

<!-- In this example, we have three modules again: `client`, `network`, and -->
<!-- `network::client`. Following the same steps we did earlier for extracting -->
<!-- modules into files, we would create *src/client.rs* for the `client` module. -->
<!-- For the `network` module, we would create *src/network.rs*. But we wouldn’t be -->
<!-- able to extract the `network::client` module into a *src/client.rs* file -->
<!-- because that already exists for the top-level `client` module! If we could put -->
<!-- the code for *both* the `client` and `network::client` modules in the -->
<!-- *src/client.rs* file, Rust wouldn’t have any way to know whether the code was -->
<!-- for `client` or for `network::client`. -->

この例でも、モジュールは3つあります: `client`、`network`、`network::client`です。
以前と同じ段階を経てモジュールをファイルに抽出すると、`client`モジュール用に*src/client.rs*を作成することになるでしょう。
`network`モジュールに関しては、*src/network.rs*を作成します。しかし、
`network::client`モジュールを*src/client.rs*ファイルに抽出することはできません。
もうトップ階層の`client`モジュールとして存在するからです！
`client`と`network::client`*双方*のコードを*src/client.rs*ファイルに書くことができたら、
コンパイラは、コードが`client`用なのか、`network::client`用なのか知る術を失ってしまいます。

<!-- Therefore, in order to extract a file for the `network::client` submodule of -->
<!-- the `network` module, we needed to create a directory for the `network` module -->
<!-- instead of a *src/network.rs* file. The code that is in the `network` module -->
<!-- then goes into the *src/network/mod.rs* file, and the submodule -->
<!-- `network::client` can have its own *src/network/client.rs* file. Now the -->
<!-- top-level *src/client.rs* is unambiguously the code that belongs to the -->
<!-- `client` module. -->

従って、`network`モジュールの`network::client`サブモジュールをファイルに抽出するには、
*src/network.rs*ファイルではなく、`network`モジュールのディレクトリを作成する必要があったわけです。
そうすれば、`network`モジュールのコードは、*src/network/mod.rs*ファイルに移ることになり、
`network::client`というサブモジュールは専用の`src/network/client.rs`ファイルを持てるわけです。
これで、頂点にある*src/client.rs*は間違いなく、`client`モジュールに属するコードになるわけです。

<!-- ### Rules of Module Filesystems -->

### モジュールファイルシステムの規則

<!-- Let’s summarize the rules of modules with regard to files: -->

ファイルに関するモジュール規則をまとめましょう:

<!-- * If a module named `foo` has no submodules, you should put the declarations -->
<!--   for `foo` in a file named *foo.rs*. -->
<!-- * If a module named `foo` does have submodules, you should put the declarations -->
<!--   for `foo` in a file named *foo/mod.rs*. -->

* `foo`という名前のモジュールにサブモジュールがなければ、`foo`の定義は、
  *foo.rs*というファイルに書くべきです。
* `foo`というモジュールに本当にサブモジュールがあったら、`foo`の定義は、
  *foo/mod.rs*というファイルに書くべきです。

<!-- These rules apply recursively, so if a module named `foo` has a submodule named -->
<!-- `bar` and `bar` does not have submodules, you should have the following files -->
<!-- in your *src* directory: -->

これらのルールは再帰的に適用されるので、`foo`というモジュールに`bar`というサブモジュールがあり、
`bar`にはサブモジュールがなければ、*src*ディレクトリには以下のようなファイルが存在するはずです:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   │          (`foo::bar`内の定義を含む)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
               (`mod bar`を含む、`foo`内の定義を含む)
```

<!-- The modules should be declared in their parent module’s file using the `mod` -->
<!-- keyword. -->

モジュールは、親モジュールのファイル内で`mod`キーワードを使って宣言されるべきなのです。

<!-- Next, we’ll talk about the `pub` keyword and get rid of those warnings! -->

次は、`pub`キーワードについて話し、警告を取り除きます！
