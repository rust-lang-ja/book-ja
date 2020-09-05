# Rustプログラミング言語

[Rustプログラミング言語](title-page.md)
[まえがき](foreword.md)
[導入](ch00-00-introduction.md)

## 事始め

- [事始め](ch01-00-getting-started.md)
    - [インストール](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [数当てゲームをプログラムする](ch02-00-guessing-game-tutorial.md)

- [一般的なプログラミングの概念](ch03-00-common-programming-concepts.md)
    - [変数と可変性](ch03-01-variables-and-mutability.md)
    - [データ型](ch03-02-data-types.md)
    - [関数](ch03-03-how-functions-work.md)
    - [コメント](ch03-04-comments.md)
    - [制御フロー](ch03-05-control-flow.md)

- [所有権を理解する](ch04-00-understanding-ownership.md)
    - [所有権とは？](ch04-01-what-is-ownership.md)
    - [参照と借用](ch04-02-references-and-borrowing.md)
    - [スライス型](ch04-03-slices.md)

- [構造体を使用して関係のあるデータを構造化する](ch05-00-structs.md)
    - [構造体を定義し、インスタンス化する](ch05-01-defining-structs.md)
    - [構造体を使ったプログラム例](ch05-02-example-structs.md)
    - [メソッド記法](ch05-03-method-syntax.md)

- [Enumとパターンマッチング](ch06-00-enums.md)
    - [Enumを定義する](ch06-01-defining-an-enum.md)
    - [`match`フロー制御演算子](ch06-02-match.md)
    - [`if let`で簡潔なフロー制御](ch06-03-if-let.md)

## 基本的なRustリテラシー

- [肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [パッケージとクレート](ch07-01-packages-and-crates.md)
    - [モジュールを定義して、スコープとプライバシーを制御する](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [モジュールツリーの要素を示すためのパス](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use`キーワードでパスをスコープに持ち込む](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [モジュールを複数のファイルに分割する](ch07-05-separating-modules-into-different-files.md)

- [一般的なコレクション](ch08-00-common-collections.md)
    - [ベクタで一連の値を保持する](ch08-01-vectors.md)
    - [文字列でUTF-8でエンコードされたテキストを保持する](ch08-02-strings.md)
    - [ハッシュマップに値に紐づいたキーを格納する](ch08-03-hash-maps.md)

- [エラー処理](ch09-00-error-handling.md)
    - [`panic!`で回復不能なエラー](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result`で回復可能なエラー](ch09-02-recoverable-errors-with-result.md)
    - [`panic!`すべきかするまいか](ch09-03-to-panic-or-not-to-panic.md)

- [ジェネリック型、トレイト、ライフタイム](ch10-00-generics.md)
    - [ジェネリックなデータ型](ch10-01-syntax.md)
    - [トレイト：共通の振る舞いを定義する](ch10-02-traits.md)
    - [ライフタイムで参照を検証する](ch10-03-lifetime-syntax.md)

- [自動テストを書く](ch11-00-testing.md)
    - [テストの記述法](ch11-01-writing-tests.md)
    - [テストの実行のされ方を制御する](ch11-02-running-tests.md)
    - [テストの体系化](ch11-03-test-organization.md)

- [入出力プロジェクト：コマンドラインプログラムを構築する](ch12-00-an-io-project.md)
    - [コマンドライン引数を受け付ける](ch12-01-accepting-command-line-arguments.md)
    - [ファイルを読み込む](ch12-02-reading-a-file.md)
    - [リファクタリンクしてモジュール性とエラー処理を向上させる](ch12-03-improving-error-handling-and-modularity.md)
    - [テスト駆動開発でライブラリの機能を開発する](ch12-04-testing-the-librarys-functionality.md)
    - [環境変数を取り扱う](ch12-05-working-with-environment-variables.md)
    - [標準出力ではなく標準エラーにエラーメッセージを書き込む](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [Functional Language Features: Iterators and Closures](ch13-00-functional-features.md)
    - [Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.md)
    - [Processing a Series of Items with Iterators](ch13-02-iterators.md)
    - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
    - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md)

- [More about Cargo and Crates.io](ch14-00-more-about-cargo.md)
    - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
    - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
    - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.md)
    - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
    - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
    - [Treating Smart Pointers Like Regular References with the `Deref` Trait](ch15-02-deref.md)
    - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
    - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
    - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
    - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Fearless Concurrency](ch16-00-concurrency.md)
    - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
    - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
    - [Shared-State Concurrency](ch16-03-shared-state.md)
    - [Extensible Concurrency with the `Sync` and `Send` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [Object Oriented Programming Features of Rust](ch17-00-oop.md)
    - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.md)
    - [Using Trait Objects That Allow for Values of Different Types](ch17-02-trait-objects.md)
    - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.md)

## Advanced Topics

- [Patterns and Matching](ch18-00-patterns.md)
    - [All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.md)
    - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.md)
    - [Pattern Syntax](ch18-03-pattern-syntax.md)

- [Advanced Features](ch19-00-advanced-features.md)
    - [Unsafe Rust](ch19-01-unsafe-rust.md)
    - [Advanced Traits](ch19-03-advanced-traits.md)
    - [Advanced Types](ch19-04-advanced-types.md)
    - [Advanced Functions and Closures](ch19-05-advanced-functions-and-closures.md)
    - [Macros](ch19-06-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.md)
    - [Building a Single-Threaded Web Server](ch20-01-single-threaded.md)
    - [Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md)
    - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
