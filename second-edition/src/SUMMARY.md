<!-- # The Rust Programming Language -->

# Rustプログラミング言語

<!-- [Foreword](foreword.md) -->
<!-- [Introduction](ch00-00-introduction.md) -->

[まえがき](foreword.md)
[導入](ch00-00-introduction.md)

<!-- ## Getting started -->

## 事始め

<!-- - [Getting Started](ch01-00-getting-started.md) -->
<!--     - [Installation](ch01-01-installation.md) -->
<!--     - [Hello, World!](ch01-02-hello-world.md) -->
<!--     - [Hello, Cargo!](ch01-03-hello-cargo.md) -->

- [事始め](ch01-00-getting-started.md)
    - [インストール](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

<!-- - [Programming a Guessing Game](ch02-00-guessing-game-tutorial.md) -->

- [数当てゲームをプログラムする](ch02-00-guessing-game-tutorial.md)

<!-- - [Common Programming Concepts](ch03-00-common-programming-concepts.md) -->
<!--     - [Variables and Mutability](ch03-01-variables-and-mutability.md) -->
<!--     - [Data Types](ch03-02-data-types.md) -->
<!--     - [How Functions Work](ch03-03-how-functions-work.md) -->
<!--     - [Comments](ch03-04-comments.md) -->
<!--     - [Control Flow](ch03-05-control-flow.md) -->

- [普遍的なプログラミング概念](ch03-00-common-programming-concepts.md)
    - [変数と可変性](ch03-01-variables-and-mutability.md)
    - [データ型](ch03-02-data-types.md)
    - [関数の動作法](ch03-03-how-functions-work.md)
    - [コメント](ch03-04-comments.md)
    - [制御フロー](ch03-05-control-flow.md)

<!-- - [Understanding Ownership](ch04-00-understanding-ownership.md) -->
<!--     - [What is Ownership?](ch04-01-what-is-ownership.md) -->
<!--     - [References & Borrowing](ch04-02-references-and-borrowing.md) -->
<!--     - [Slices](ch04-03-slices.md) -->

- [所有権を理解する](ch04-00-understanding-ownership.md)
    - [所有権とは？](ch04-01-what-is-ownership.md)
    - [参照と借用](ch04-02-references-and-borrowing.md)
    - [スライス](ch04-03-slices.md)

<!-- - [Using Structs to Structure Related Data](ch05-00-structs.md) -->
<!--     - [Defining and Instantiating Structs](ch05-01-defining-structs.md) -->
<!--     - [An Example Program Using Structs](ch05-02-example-structs.md) -->
<!--     - [Method Syntax](ch05-03-method-syntax.md) -->

- [構造体を使用して関連のあるデータを構造化する](ch05-00-structs.md)
    - [構造体を定義し、インスタンス化する](ch05-01-defining-structs.md)
    - [構造体を使用したプログラム例](ch05-02-example-structs.md)
    - [メソッド記法](ch05-03-method-syntax.md)

<!-- - [Enums and Pattern Matching](ch06-00-enums.md) -->
<!--     - [Defining an Enum](ch06-01-defining-an-enum.md) -->
<!--     - [The `match` Control Flow Operator](ch06-02-match.md) -->
<!--     - [Concise Control Flow with `if let`](ch06-03-if-let.md) -->

- [Enumとパターンマッチング](ch06-00-enums.md)
    - [Enumを定義する](ch06-01-defining-an-enum.md)
    - [`match`制御フロー演算子](ch06-02-match.md)
    - [`if let`で簡潔な制御フロー](ch06-03-if-let.md)

<!-- ## Basic Rust Literacy -->

## 基本的なRustリテラシー

<!-- - [Modules](ch07-00-modules.md) -->
<!--     - [`mod` and the Filesystem](ch07-01-mod-and-the-filesystem.md) -->
<!--     - [Controlling Visibility with `pub`](ch07-02-controlling-visibility-with-pub.md) -->
<!--     - [Referring to Names in Different Modules](ch07-03-importing-names-with-use.md) -->

- [モジュール](ch07-00-modules.md)
    - [`mod`とファイルシステム](ch07-01-mod-and-the-filesystem.md)
    - [`pub`で公開するか制御する](ch07-02-controlling-visibility-with-pub.md)
    - [異なるモジュールの名前を参照する](ch07-03-importing-names-with-use.md)

<!-- - [Common Collections](ch08-00-common-collections.md) -->
<!--     - [Vectors](ch08-01-vectors.md) -->
<!--     - [Strings](ch08-02-strings.md) -->
<!--     - [Hash Maps](ch08-03-hash-maps.md) -->

- [一般的なコレクション](ch08-00-common-collections.md)
    - [ベクタ型](ch08-01-vectors.md)
    - [文字列型](ch08-02-strings.md)
    - [ハッシュマップ](ch08-03-hash-maps.md)

<!-- - [Error Handling](ch09-00-error-handling.md) -->
<!--     - [Unrecoverable Errors with `panic!`](ch09-01-unrecoverable-errors-with-panic.md) -->
<!--     - [Recoverable Errors with `Result`](ch09-02-recoverable-errors-with-result.md) -->
<!--     - [To `panic!` or Not To `panic!`](ch09-03-to-panic-or-not-to-panic.md) -->

- [エラー処理](ch09-00-error-handling.md)
    - [`panic!`で回復不能なエラー](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result`で回復可能なエラー](ch09-02-recoverable-errors-with-result.md)
    - [`panic!`すべきかするまいか](ch09-03-to-panic-or-not-to-panic.md)

<!-- - [Generic Types, Traits, and Lifetimes](ch10-00-generics.md) -->
<!--     - [Generic Data Types](ch10-01-syntax.md) -->
<!--     - [Traits: Defining Shared Behavior](ch10-02-traits.md) -->
<!--     - [Validating References with Lifetimes](ch10-03-lifetime-syntax.md) -->

- [ジェネリック型、トレイト、ライフタイム](ch10-00-generics.md)
    - [ジェネリックなデータ型](ch10-01-syntax.md)
    - [トレイト: 共通の振る舞いを定義する](ch10-02-traits.md)
    - [ライフタイムで参照を有効化する](ch10-03-lifetime-syntax.md)

<!-- - [Testing](ch11-00-testing.md) -->
<!--     - [Writing tests](ch11-01-writing-tests.md) -->
<!--     - [Running tests](ch11-02-running-tests.md) -->
<!--     - [Test Organization](ch11-03-test-organization.md) -->

- [テスト](ch11-00-testing.md)
    - [テストを書く](ch11-01-writing-tests.md)
    - [テストを走らせる](ch11-02-running-tests.md)
    - [テストの体系化](ch11-03-test-organization.md)

<!-- - [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md) -->
<!--     - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md) -->
<!--     - [Reading a File](ch12-02-reading-a-file.md) -->
<!--     - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md) -->
<!--     - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md) -->
<!--     - [Working with Environment Variables](ch12-05-working-with-environment-variables.md) -->
<!--     - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md) -->

- [入出力プロジェクト: コマンドラインプログラムを構築する](ch12-00-an-io-project.md)
    - [コマンドライン引数を受け付ける](ch12-01-accepting-command-line-arguments.md)
    - [ファイルを読み込む](ch12-02-reading-a-file.md)
    - [リファクタリングしてモジュール性の向上とエラー処理](ch12-03-improving-error-handling-and-modularity.md)
    - [テスト駆動開発でライブラリの機能を開発する](ch12-04-testing-the-librarys-functionality.md)
    - [環境変数を取り扱う](ch12-05-working-with-environment-variables.md)
    - [標準出力ではなく標準エラーにエラーメッセージを書き込む](ch12-06-writing-to-stderr-instead-of-stdout.md)

<!-- ## Thinking in Rust -->

## Rustで思考する

<!-- - [Functional Language Features: Iterators and Closures](ch13-00-functional-features.md) -->
<!--     - [Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.md) -->
<!--     - [Processing a Series of Items with Iterators](ch13-02-iterators.md) -->
<!--     - [Improving Our I/O Project](ch13-03-improving-our-io-project.md) -->
<!--     - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md) -->

- [関数型言語の機能: イテレータとクロージャ](ch13-00-functional-features.md)
    - [クロージャ: 環境をキャプチャできる匿名関数](ch13-01-closures.md)
    - [一連の要素をイテレータで処理する](ch13-02-iterators.md)
    - [入出力プロジェクトを改善する](ch13-03-improving-our-io-project.md)
    - [パフォーマンス比較: ループVSイテレータ](ch13-04-performance.md)

<!-- - [More about Cargo and Crates.io](ch14-00-more-about-cargo.md) -->
<!--     - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md) -->
<!--     - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md) -->
<!--     - [Cargo Workspaces](ch14-03-cargo-workspaces.md) -->
<!--     - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.md) -->
<!--     - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md) -->

- [CargoとCrates.ioについてより詳しく](ch14-00-more-about-cargo.md)
    - [リリースプロファイルでビルドをカスタマイズする](ch14-01-release-profiles.md)
    - [Crates.ioにクレートを公開する](ch14-02-publishing-to-crates-io.md)
    - [Cargoのワークスペース](ch14-03-cargo-workspaces.md)
    - [`cargo install`でCrates.ioからバイナリをインストールする](ch14-04-installing-binaries.md)
    - [独自のコマンドでCargoで拡張する](ch14-05-extending-cargo.md)

<!-- - [Smart Pointers](ch15-00-smart-pointers.md) -->
<!--     - [`Box<T>` Points to Data on the Heap and Has a Known Size](ch15-01-box.md) -->
<!--     - [The `Deref` Trait Allows Access to the Data Through a Reference](ch15-02-deref.md) -->
<!--     - [The `Drop` Trait Runs Code on Cleanup](ch15-03-drop.md) -->
<!--     - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md) -->
<!--     - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md) -->
<!--     - [Creating Reference Cycles and Leaking Memory is Safe](ch15-06-reference-cycles.md) -->

- [スマートポインタ](ch15-00-smart-pointers.md)
    - [`Box<T>`はヒープのデータを指し、既知のサイズである](ch15-01-box.md)
    - [`Deref`トレイトにより、参照を通してデータにアクセスできる](ch15-02-deref.md)
    - [`Drop`トレイトにより、片付けの時にコードを実行する](ch15-03-drop.md)
    - [`Rc<T>`は、参照カウント方式のスマートポインタ](ch15-04-rc.md)
    - [`RefCell<T>`と内部可変性パターン](ch15-05-interior-mutability.md)
    - [循環参照し、メモリをリークするのは安全である](ch15-06-reference-cycles.md)

<!-- - [Fearless Concurrency](ch16-00-concurrency.md) -->
<!--     - [Threads](ch16-01-threads.md) -->
<!--     - [Message Passing](ch16-02-message-passing.md) -->
<!--     - [Shared State](ch16-03-shared-state.md) -->
<!--     - [Extensible Concurrency: `Sync` and `Send`](ch16-04-extensible-concurrency-sync-and-send.md) -->

- [恐れるな！非同期処理](ch16-00-concurrency.md)
    - [スレッド](ch16-01-threads.md)
    - [メッセージ受け渡し](ch16-02-message-passing.md)
    - [状態共有](ch16-03-shared-state.md)
    - [拡張可能な非同期: `Sync`と`Send`](ch16-04-extensible-concurrency-sync-and-send.md)

<!-- - [Object-Oriented Programming Features of Rust](ch17-00-oop.md) -->
<!--     - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.md) -->
<!--     - [Using Trait Objects that Allow for Values of Different Types](ch17-02-trait-objects.md) -->
<!--     - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.md) -->

- [Rustのオブジェクト指向プログラミング機能](ch17-00-oop.md)
    - [オブジェクト指向言語の特徴](ch17-01-what-is-oo.md)
    - [異なる型の値を許容するトレイトオブジェクトを使用する](ch17-02-trait-objects.md)
    - [オブジェクト指向デザインパターンを実装する](ch17-03-oo-design-patterns.md)

<!-- ## Advanced Topics -->

## 高度なトピック

<!-- - [Patterns Match the Structure of Values](ch18-00-patterns.md) -->
<!--     - [All the Places Patterns May be Used](ch18-01-all-the-places-for-patterns.md) -->
<!--     - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.md) -->
<!--     - [All the Pattern Syntax](ch18-03-pattern-syntax.md) -->

- [パターンは値の構造に合致する](ch18-00-patterns.md)
    - [パターンが使用されるかもしれない箇所全部](ch18-01-all-the-places-for-patterns.md)
    - [論駁可能性: パターンが合致しないか](ch18-02-refutability.md)
    - [パターン記法全部](ch18-03-pattern-syntax.md)

<!-- - [Advanced Features](ch19-00-advanced-features.md) -->
<!--     - [Unsafe Rust](ch19-01-unsafe-rust.md) -->
<!--     - [Advanced Lifetimes](ch19-02-advanced-lifetimes.md) -->
<!--     - [Advanced Traits](ch19-03-advanced-traits.md) -->
<!--     - [Advanced Types](ch19-04-advanced-types.md) -->
<!--     - [Advanced Functions & Closures](ch19-05-advanced-functions-and-closures.md) -->

- [高度な機能](ch19-00-advanced-features.md)
    - [Unsafe Rust](ch19-01-unsafe-rust.md)
    - [高度なライフタイム](ch19-02-advanced-lifetimes.md)
    - [高度なトレイト](ch19-03-advanced-traits.md)
    - [高度な型](ch19-04-advanced-types.md)
    - [高度な関数とクロージャ](ch19-05-advanced-functions-and-closures.md)

<!-- - [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.md) -->
<!--     - [A Single Threaded Web Server](ch20-01-single-threaded.md) -->
<!--     - [Turning our Single Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md) -->
<!--     - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.md) -->

- [最後のプロジェクト: マルチスレッドのWebサーバを構築する](ch20-00-final-project-a-web-server.md)
    - [シングルスレッドのWebサーバ](ch20-01-single-threaded.md)
    - [シングルスレッドのサーバをマルチスレッド化する](ch20-02-multithreaded.md)
    - [優美なシャットダウンとお片付け](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Macros](appendix-04-macros.md)
    - [E - Translations](appendix-05-translation.md)
    - [F - Newest Features](appendix-06-newest-features.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
