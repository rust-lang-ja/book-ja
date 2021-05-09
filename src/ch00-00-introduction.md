<!--
# Introduction
-->

# はじめに

<!--
> Note: This edition of the book is the same as [The Rust Programming
> Language][nsprust] available in print and ebook format from [No Starch
> Press][nsp].
-->

> 注釈: この本のこの版は、本として利用可能な[The Rust Programming Language][nsprust]と、
> [No Starch Press][nsp]のebook形式と同じです。

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

<!--
Welcome to *The Rust Programming Language*, an introductory book about Rust.
-->

*The Rust Programming Language*へようこそ。Rustに関する入門書です。

<!--
The Rust programming language helps you write faster, more reliable software.
High-level ergonomics and low-level control are often at odds in programming
language design; Rust challenges that conflict. Through balancing powerful
technical capacity and a great developer experience, Rust gives you the option
to control low-level details (such as memory usage) without all the hassle
traditionally associated with such control.
-->

Rustプログラミング言語は、高速で信頼できるソフトウェアを書く手助けをしてくれます。
高レベルのエルゴノミクス(`訳注`: ergonomicsとは、人間工学的という意味。砕いて言えば、人間に優しいということ)と低レベルの制御は、
しばしばプログラミング言語の設計においてトレードオフの関係になります;
Rustは、その衝突に挑戦しています。バランスのとれた強力な技術の許容量と素晴らしい開発者経験を通して、
Rustは伝統的にそれらの制御と紐付いていた困難全てなしに低レベルの詳細(メモリ使用など)を制御する選択肢を与えてくれます。

<!--
## Who Rust Is For
-->

## Rustは誰のためのものなの

<!--
Rust is ideal for many people for a variety of reasons. Let’s look at a few of
the most important groups.
-->

Rustは、様々な理由により多くの人にとって理想的です。いくつか最も重要なグループを見ていきましょう。

<!--
### Teams of Developers
-->

### 開発者チーム

<!--
Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to a variety of subtle bugs, which in most other languages can be
caught only through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these elusive bugs, including concurrency bugs. By working
alongside the compiler, the team can spend their time focusing on the program’s
logic rather than chasing down bugs.
-->

Rustは、いろんなレベルのシステムプログラミングの知識を持つ開発者の巨大なチームとコラボするのに生産的なツールであると証明してきています。
低レベルコードは様々な種類の微細なバグを抱える傾向があり、そのようなバグは他の言語だと広範なテストと、
経験豊富な開発者による注意深いコードレビューによってのみ捕捉されるものです。Rustにおいては、
コンパイラが並行性のバグも含めたこのようなとらえどころのないバグのあるコードをコンパイルするのを拒むことで、
門番の役割を担います。コンパイラとともに取り組むことで、チームはバグを追いかけるよりもプログラムのロジックに集中することに、
時間を費やせるのです。

<!--
Rust also brings contemporary developer tools to the systems programming world:
-->

Rustはまた、現代的な開発ツールをシステムプログラミング世界に導入します。

<!--
* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers Integrated Development Environment (IDE)
  integration for code completion and inline error messages.
-->

* Cargoは、付属の依存関係管理ツール兼ビルドツールで、依存関係の追加、コンパイル、管理を容易にし、Rustのエコシステム全体で一貫性を持たせます。
* Rustfmtは開発者の間で一貫したコーディングスタイルを保証します。
* Rust言語サーバーは、IDE(統合開発環境)との統合により、コード補完やインラインエラーメッセージに対応しています。

<!--
By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.
-->

これらのツールやRustのエコシステムの他のツールを使用することで、開発者はシステムレベルのコードを書きながら生産性を高めることができます。

<!--
### Students
-->

### 学生

<!--
Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
student questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.
-->

Rustは、学生やシステムの概念を学ぶことに興味のある方向けです。Rustを使用して、
多くの人がOS開発などの話題を学んできました。コミュニティはとても暖かく、喜んで学生の質問に答えてくれます。
この本のような取り組みを通じて、Rustチームはシステムの概念をより多くの人、特にプログラミング初心者に知ってもらいたいと考えています。

<!--
### Companies
-->

### 企業

<!--
Hundreds of companies, large and small, use Rust in production for a variety of
tasks. Those tasks include command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.
-->

数百の企業が、大企業、中小企業を問わず、様々なタスクの本番環境でRustを使用しています。
そのタスクには、コマンドラインツール、Webサービス、DevOpsツール、組み込みデバイス、
オーディオやビデオの解析および変換、暗号通貨、生物情報学、サーチエンジン、IoTアプリケーション、
機械学習、さらにはFirefoxウェブブラウザの主要部分さえ含まれます。

<!--
### Open Source Developers
-->

### オープンソース開発者

<!--
Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love to have you contribute to the Rust
language.
-->

Rustは、Rustプログラミング言語やコミュニティ、開発者ツール、ライブラリを開発したい方向けです。
あなたがRust言語に貢献されることを心よりお待ちしております。

<!--
### People Who Value Speed and Stability
-->

### スピードと安定性に価値を見出す方

<!--
Rust is for people who crave speed and stability in a language. By speed, we
mean the speed of the programs that you can create with Rust and the speed at
which Rust lets you write them. The Rust compiler’s checks ensure stability
through feature additions and refactoring. This is in contrast to the brittle
legacy code in languages without these checks, which developers are often
afraid to modify. By striving for zero-cost abstractions, higher-level features
that compile to lower-level code as fast as code written manually, Rust
endeavors to make safe code be fast code as well.
-->

Rustは、スピードと安定性を言語に渇望する方向けです。ここでいうスピードとは、
Rustで作れるプログラムのスピードとソースコードを書くスピードのことです。Rustコンパイラのチェックにより、
機能の追加とリファクタリングを通して安定性を保証してくれます。これはこのようなチェックがない言語の脆いレガシーコードとは対照的で、
その場合開発者はしばしば、変更するのを恐れてしまいます。ゼロコスト抽象化を志向し、
手で書いたコードと同等の速度を誇る低レベルコードにコンパイルされる高レベル機能により、
Rustは安全なコードを高速なコードにもしようと努力しています。

<!--
The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust’s greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety *and* productivity, speed *and* ergonomics. Give
Rust a try and see if its choices work for you.
-->

Rust言語は他の多くのユーザのサポートも望んでいます; ここで名前を出した方は、
ただの最大の出資者の一部です。総合すると、Rustの最大の野望は、プログラマが数十年間受け入れてきた代償を、安全性*と*生産性、
スピード*と*エルゴノミクスを提供することで排除することです。Rustを試してみて、その選択が自分に合っているか確かめてください。

<!--
## Who This Book Is For
-->

## この本は誰のためのものなの

<!--
This book assumes that you’ve written code in another programming language but
doesn’t make any assumptions about which one. We’ve tried to make the material
broadly accessible to those from a wide variety of programming backgrounds. We
don’t spend a lot of time talking about what programming *is* or how to think
about it. If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.
-->

この本は、あなたが他のプログラミング言語でコードを書いたことがあることを想定していますが、
具体的にどの言語かという想定はしません。私たちは、様々なプログラミング背景を持つ人たちが幅広く理解できる内容にしています。プログラミングとはなん*なのか*やそれについて考える方法について多くを語るつもりはありません。
もし、あなたがプログラミングの完全な初心者であれば、プログラミングの入門書を読むと役に立つでしょう。

<!--
## How to Use This Book
-->

## この本の使い方

<!--
In general, this book assumes that you’re reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a topic; we typically revisit the
topic in a later chapter.
-->

一般的に、この本は、順番に読み進めていくことを前提にしています。後の章は、前の章の概念の上に成り立ち、
前の章では、ある話題にさほど深入りしない可能性がありますが、通常は後の章で同じ話題を復習します。

<!--
You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.
-->

この本には、概念の章とプロジェクトの章の2種類の章があります。概念の章では、
Rustの一面を学ぶでしょう。プロジェクトの章では、それまでに学んだことを応用して小さなプログラムを一緒に作っていきます。
第2、第12、第20章がプロジェクトの章で、残りは概念の章です。

<!--
Chapter 1 explains how to install Rust, how to write a “Hello, world!” program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to the Rust language. Here we cover concepts at a high
level, and later chapters will provide additional detail. If you want to get
your hands dirty right away, Chapter 2 is the place for that. At first, you
might even want to skip Chapter 3, which covers Rust features similar to those
of other programming languages, and head straight to Chapter 4 to learn about
Rust’s ownership system. However, if you’re a particularly meticulous learner
who prefers to learn every detail before moving on to the next, you might want
to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when
you’d like to work on a project applying the details you’ve learned.
-->

第1章はRustのインストール方法、“Hello, world!”プログラムの書き方、Rustのパッケージマネージャ兼、
ビルドツールのCargoの使用方法を説明します。第2章は、Rust言語の実践的な紹介です。ここでは概念について高いレベルで説明し、後ほどの章ではさらに詳しく説明します。
今すぐRustの世界に飛び込みたいなら、第2章こそがそのためのものです。最初は、他のプログラミング言語と同様なRustの機能を説明している第3章を飛ばして、Rustの所有権システムについて学ぶために第4章に直行するのもいいでしょう。しかし、「細かいところまで学んでから次の段階へ進みたい」という几帳面な学習者であれば、第2章を飛ばして第3章に進み、学んだ内容を応用してプロジェクトを進めたいときに第2章へ戻ってくるとよいでしょう。

<!--
Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match`
expressions, and the `if let` control flow construct. You’ll use structs and
enums to make custom types in Rust.
-->

第5章では、構造体とメソッドについて、第6章では、列挙型、`match`式、`if let`フロー制御構文について説明しています。
構造体や列挙型を使用して、Rustの独自の型を作成します。

<!--
In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.
-->

第7章では、Rustのモジュールシステム、自分のコードを体系化するプライバシー規則、公開されているAPI(Application Programming Interface)について学びます。
第8章では、ベクタ、文字列、ハッシュマップなどの標準ライブラリが提供する一般的なコレクションデータ構造について説明します。
第9章では、Rustのエラー処理哲学とテクニックについて探っていきます。

<!--
Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.
-->

第10章では、複数の型に適用されるコードを定義するための、ジェネリクス、トレイト、ライフタイムについて掘り下げています。
第11章では、Rustの安全性保証があってさえ、プログラムのロジックが正しいことを確認するために必要な、テストについて説明します。
第12章では、ファイル内のテキストを検索するコマンドラインツール`grep`の機能の一部を独自に実装します。そのためには、以前の章で説明した多くの概念を使用します。

<!--
Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.
-->

第13章では、関数型プログラミング言語に由来するRustの特徴であるクロージャとイテレータについて探っていきます。
第14章では、Cargoをより詳しく考察し、自分のライブラリを他人と共有する最善の策について語ります。
第15章では、標準ライブラリが提供するスマートポインタとその機能を可能にするトレイトについて説明します。

<!--
In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.
-->

第16章では、並行プログラミングの様々なモデルを紹介し、Rustを使えば、複数のスレッドを使ったプログラミングも恐れることがないことを語ります。
第17章では、皆さんがよく知っているオブジェクト指向プログラミングの原則とRustのイディオムが、どのように対照をなすかを見ていきます。

<!--
Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.
-->

第18章は、Rustプログラムの中で考えを表現する強力な手段である、パターンとパターンマッチングのリファレンスです。第19章では、unsafe Rustやマクロをはじめとする、ライフタイム、トレイト、型、関数、クロージャなどの詳細を含む、興味を引く高度な話題が盛りだくさんです。(`訳注`: 原文のスモーガスボードとは、日本でいうバイキング料理のことです。ここでは、盛りだくさんと意訳しました。)

<!--
In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!
-->

第20章では、低レベルなマルチスレッドのWebサーバを実装するプロジェクトを完成させます！

<!--
Finally, some appendixes contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.
-->

最後に、一部の付録には、言語に関する有用な情報が、よりリファレンス的な書式で記載されています。付録AではRustのキーワードについて、付録BではRustの演算子と記号について、付録Cでは標準ライブラリで提供される派生的なトレイトについて、付録Dでは便利な開発ツールについて、付録EではRustのエディションについて説明しています。

<!--
There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.
-->

この本の読み方に間違った方法なんてありません: 先に飛ばしたければ、どうぞお試しください！
混乱したら、前の章に戻る必要があるかもしれませんが、自分に合った方法でやってみてください。

<!--
<span id="ferris"></span>
-->

<span id="ferris"></span>

<!--
An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:
-->

Rustを学ぶ過程で重要なのは、コンパイラが表示するエラーメッセージの読み方を学ぶことです。これらは、コードが動くように導いてくれます。
そのため、コンパイルされない例題を多数用意し、それぞれの状況でコンパイラが表示するエラーメッセージを例示します。適当な例題を入力して実行すると、コンパイルできない場合があることを知っておいてください！周りのテキストも必ず読んで、実行しようとしている例題がエラーになることを意図しているものかどうかを確認してください。また、Ferrisは動くはずのないコードを見分けるのにも役に立ちます(`訳注`: `Ferris`とは、Rustのマスコットキャラクタの蟹の名称ですが、ここで触れている`Ferris`とは、コンパイラのエラーメッセージに含まれるキャラクタのことを示しています。)：

<!--
| Ferris                                                                 | Meaning                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | This code panics!                                |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | This code block contains unsafe code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| This code does not produce the desired behavior. |
-->

| Ferris                                                                 | Meaning                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | このコードはコンパイルできません！               |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | このコードはパニックします！                     |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | このコードはアンセーフなコードを含みます。       |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| このコードは求められている振る舞いをしません。   |

<!--
In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.
-->

ほとんどの場合、コンパイルされないコードを正解版へと導きます。

<!--
## Source Code
-->

## ソースコード

<!--
The source files from which this book is generated can be found on
[GitHub][book].
-->

この本が生成されるソースファイルは、[GitHub][book]で見つかります。

> 訳注: 日本語版は[こちら][book-ja]です。

[book]: https://github.com/rust-lang/book/tree/master/src
[book-ja]: https://github.com/rust-lang-ja/book-ja
