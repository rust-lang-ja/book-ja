<!--
# An I/O Project: Building a Command Line Program
-->

# 入出力プロジェクト: コマンドラインプログラムを構築する

<!--
This chapter is a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’ll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.
-->

この章では、これまでに学んできた多くのスキルの復習を行い，さらにいくつか標準ライブラリの機能を探求します。
ファイルやコマンドラインの入出力を扱うコマンドラインツールを構築し、ここまでに習ったRustの概念の一部を実践します。

<!--
Rust’s speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we’ll
make our own version of the classic command line tool `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). In the simplest use case,
`grep` searches a specified file for a specified string. To do so, `grep` takes
as its arguments a filename and a string. Then it reads the file, finds lines
in that file that contain the string argument, and prints those lines.
-->

Rustの高速性、安全性、単バイナリ出力、クロスプラットフォームサポートは、コマンドラインツールを作るのに理想的です。そこでこのプロジェクトでは、古典的なコマンドライン検索ツールの`grep`(**g**lobally search a **r**egular **e**xpression and **p**rint: 正規表現をグローバルで検索し表示する)を独自に作成していきます。最も単純な使用法では、
`grep`は指定したファイルから指定した文字列を検索します。そのために、`grep`は引数としてファイル名と文字列を受け取ります。それからファイルを読み込んでそのファイルの中から引数の文字列を含む行を探し、表示するのです。

<!--
Along the way, we’ll show how to make our command line tool use features of the
terminal that many command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print to the standard error console stream (`stderr`) instead of
standard output (`stdout`), so, for example, the user can redirect successful
output to a file while still seeing error messages onscreen.
-->

その過程で、他の多くのコマンドラインツールが使用しているターミナルの機能を、私たちのコマンドラインツールにも使用させる方法を紹介します。環境変数の値を読み取ってユーザがツールの振る舞いを設定できるようにするのです。また、エラーメッセージを標準出力(`stdout`)の代わりに、標準エラー(`stderr`)に出力します。これにより、ユーザはエラーメッセージを画面上で確認しながら、成功した出力をファイルにリダイレクトすることができます。

<!--
One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version of `grep` will be fairly simple, but this chapter will give you some of
the background knowledge you need to understand a real-world project like
`ripgrep`.
-->

Rustコミュニティのメンバーの1人であるアンドリュー・ガラント(Andrew Gallant)が、既に`ripgrep`という、非常に高速で完全な機能を持った`grep`のバージョンを作成しています。それに比べると、この章で私たちが作るものはかなりシンプルですが、そこから`ripgrep`のような現実世界のプロジェクトを理解するのに必要な背景知識の一部を身に付けられるでしょう。

<!--
Our `grep` project will combine a number of concepts you’ve learned so far:
-->

この`grep`プロジェクトでは、ここまでに学んできた多くの概念を組み合わせていきます:

<!--
* Organizing code (using what you learned in modules, Chapter 7)
* Using vectors and strings (collections, Chapter 8)
* Handling errors (Chapter 9)
* Using traits and lifetimes where appropriate (Chapter 10)
* Writing tests (Chapter 11)
-->

* コードを体系化する(モジュール、第7章で学んだことを使用)
* ベクタと文字列を使用する(コレクション、第8章)
* エラーを処理する(第9章)
* 適切な箇所でトレイトとライフタイムを使用する(第10章)
* テストを記述する(第11章)

<!--
We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters 13 and 17 will cover in detail.
-->

さらに、クロージャ、イテレータ、トレイトオブジェクトなど、第13章、17章で詳しく講義するものもちょっとだけ紹介します。
