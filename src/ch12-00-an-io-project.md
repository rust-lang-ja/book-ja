<!--
# An I/O Project: Building a Command Line Program
-->

# 入出力プロジェクト：コマンドラインプログラムを構築する

<!--
This chapter is a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’ll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.
-->

この章は、ここまでに学んできた多くのスキルを思い出すきっかけであり、もういくつか標準ライブラリの機能も探究します。
ファイルやコマンドラインの入出力と相互作用するコマンドラインツールを構築し、
今やあなたの支配下にある Rust の概念の一部を練習していきます。

<!--
Rust’s speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we’ll
make our own version of the classic command line tool `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). In the simplest use case,
`grep` searches a specified file for a specified string. To do so, `grep` takes
as its arguments a filename and a string. Then it reads the file, finds lines
in that file that contain the string argument, and prints those lines.
-->

Rust の速度、安全性、単バイナリ出力、クロスプラットフォームサポートにより、コマンドラインツールを作るのにふさわしい言語なので、
このプロジェクトでは、独自の伝統的なコマンドラインツールの`grep`(**g**lobally search a **r**egular **e**xpression
and **p**rint: 正規表現をグローバルで検索し表示する) を作成していきます。最も単純な使用法では、
`grep`は指定したファイルから指定した文字列を検索します。そうするには、
`grep`は引数としてファイル名と文字列を受け取ります。それからファイルを読み込んでそのファイル内で文字列引数を含む行を探し、
検索した行を出力するのです。

<!--
Along the way, we’ll show how to make our command line tool use features of the
terminal that many command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print to the standard error console stream (`stderr`) instead of
standard output (`stdout`), so, for example, the user can redirect successful
output to a file while still seeing error messages onscreen.
-->

その過程で、多くのコマンドラインツールが使用している端末の機能を使用させる方法を示します。
環境変数の値を読み取ってユーザがこのツールの振る舞いを設定できるようにします。また、
標準出力 (`stdout`) の代わりに、標準エラーに出力 (`stderr`) するので、例えば、
ユーザはエラーメッセージは画面上で確認しつつ、成功した出力はファイルにリダイレクトできます。

<!--
One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version of `grep` will be fairly simple, but this chapter will give you some of
the background knowledge you need to understand a real-world project like
`ripgrep`.
-->

Rust コミュニティのあるメンバであるアンドリュー・ガラント (Andrew Gallant) が既に全機能装備の非常に高速な`grep`、
`ripgrep`と呼ばれるものを作成しました。比較対象として、我々の`grep`はとても単純ですが、
この章により、`ripgrep`のような現実世界のプロジェクトを理解するのに必要な背景知識の一部を身に付けられるでしょう。

<!--
Our `grep` project will combine a number of concepts you’ve learned so far:
-->

この`grep`プロジェクトは、ここまでに学んできた多くの概念を集結させます：

<!--
* Organizing code (using what you learned in modules, Chapter 7)
* Using vectors and strings (collections, Chapter 8)
* Handling errors (Chapter 9)
* Using traits and lifetimes where appropriate (Chapter 10)
* Writing tests (Chapter 11)
-->

* コードを体系化する (モジュール、第 7 章で学んだことを使用)
* ベクタと文字列を使用する (コレクション、第 8 章)
* エラーを処理する (第 9 章)
* 適切な箇所でトレイトとライフタイムを使用する (第 10 章)
* テストを記述する (第 11 章)

<!--
We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters 13 and 17 will cover in detail.
-->

さらに、クロージャ、イテレータ、トレイトオブジェクトなど、第 13 章、17 章で詳しく講義するものもちょっとだけ紹介します。
