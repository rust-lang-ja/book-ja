<!--
# Managing Growing Projects with Packages, Crates, and Modules
-->
# 肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する

<!--
As you write large programs, organizing your code will be important because
keeping track of your entire program in your head will become impossible. By
grouping related functionality and separating code with distinct features,
you’ll clarify where to find code that implements a particular feature and
where to go to change how a feature works.
-->
大きなプログラムを書く時、そのすべてを頭の中に入れておくのは不可能になるため、コードのまとまりを良くすることが重要になります。
関係した機能をまとめ、異なる特徴を持つコードを分割することにより、特定の機能を実装しているコードを見つけたり、機能を変更したりするためにどこを探せば良いのかを明確にできます。

<!--
The programs we’ve written so far have been in one module in one file. As a
project grows, you can organize code by splitting it into multiple modules and
then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. This chapter covers all
these techniques. For very large projects of a set of interrelated packages
that evolve together, Cargo provides workspaces, which we’ll cover in the
[“Cargo Workspaces”][workspaces] section in Chapter 14.
-->
私達がこれまでに書いてきたプログラムは、一つのファイル内の一つのモジュール内にありました。
プロジェクトが大きくなるにつれて、これを複数のモジュールに、ついで複数のファイルに分割することで、プログラムを整理することができます。
パッケージは複数のバイナリクレートからなり、またライブラリクレートを 1 つもつこともできます。
パッケージが大きくなるにつれて、その一部を抜き出して分離したクレートにし、外部依存とするのもよいでしょう。
この章ではそれらのテクニックすべてを学びます。
相互に関係し合い、同時に成長するパッケージの集まりからなる巨大なプロジェクトには、
Cargo がワークスペースという機能を提供します。これは 14 章の[Cargo ワークスペース][workspaces]<!-- ignore -->で解説します。

<!--
In addition to grouping functionality, encapsulating implementation details
lets you reuse code at a higher level: once you’ve implemented an operation,
other code can call that code via the code’s public interface without knowing
how the implementation works. The way you write code defines which parts are
public for other code to use and which parts are private implementation details
that you reserve the right to change. This is another way to limit the amount
of detail you have to keep in your head.
-->
機能をグループにまとめられることに加え、実装の詳細がカプセル化されることにより、コードをより高いレベルで再利用できるようになります：
手続きを実装し終えてしまえば、他のコードはそのコードの公開されたインターフェースを通じて、実装の詳細を知ることなくそのコードを呼び出すことができるのです。
コードをどう書くかによって、どの部分が他のコードにも使える公開のものになるのか、それとも自分だけが変更できる非公開のものになるのかが決定されます。
これもまた、記憶しておくべき細部を制限してくれる方法のひとつです。

<!--
A related concept is scope: the nested context in which code is written has a
set of names that are defined as “in scope.” When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can’t have two items with the
same name in the same scope; tools are available to resolve name conflicts.
-->
関係する概念にスコープがあります：
コードが記述されているネストされた文脈には、「スコープ内」として定義される名前の集合があります。
コードを読んだり書いたりコンパイルしたりする時には、プログラマーやコンパイラは特定の場所にある特定の名前が、変数・関数・構造体・enum・モジュール・定数・その他のどの要素を表すのか、そしてその要素は何を意味するのかを知る必要があります。
そこでスコープを作り、どの名前がスコープ内/スコープ外にあるのかを変更することができます。
同じ名前のものを 2 つ同じスコープ内に持つことはできません。そこで、名前の衝突を解決するための方法があります。

<!--
Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the *module system*, include:
-->
Rust には、どの詳細を公開するか、どの詳細を非公開にするか、どの名前がプログラムのそれぞれのスコープにあるか、といったコードのまとまりを保つためのたくさんの機能があります。
これらの機能は、まとめて「モジュールシステム」と呼ばれることがあり、以下のようなものが含まれます。

<!--
* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module
-->
* **パッケージ：** クレートをビルドし、テストし、共有することができる Cargo の機能
* **クレート：** ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
* **モジュール** と **use:** これを使うことで、パスの構成、スコープ、公開するか否かを決定できます
* **パス：** 要素（例えば構造体や関数やモジュール）に名前をつける方法

<!--
In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!
-->
この章では、これらの機能をすべて学び、これらがどう相互作用するかについて議論し、これらをどう使ってスコープを制御するのかについて説明します。
この章を読み終わる頃には、モジュールシステムをしっかりと理解し、熟練者のごとくスコープを扱うことができるようになっているでしょう！

[workspaces]: ch14-03-cargo-workspaces.html
