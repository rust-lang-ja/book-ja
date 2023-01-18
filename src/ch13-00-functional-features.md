<!--
# Functional Language Features: Iterators and Closures
-->

# 関数型言語の機能：イテレータとクロージャ

<!--
Rust’s design has taken inspiration from many existing languages and
techniques, and one significant influence is *functional programming*.
Programming in a functional style often includes using functions as values by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth.
-->

Rust の設計は、多くの既存の言語やテクニックにインスピレーションを得ていて、
その一つの大きな影響が*関数型プログラミング*です。関数型でのプログラミングには、しばしば、
引数で渡したり、関数から関数を返したり、関数を後ほど使用するために変数に代入することで関数を値として使用することが含まれます。

<!--
In this chapter, we won’t debate the issue of what functional programming is or
isn’t but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.
-->

この章では、関数型プログラミングがどんなものであったり、なかったりするかという問題については議論しませんが、
代わりに関数型とよく言及される多くの言語の機能に似た Rust の機能の一部について議論しましょう。

<!--
More specifically, we’ll cover:
-->

具体的には、以下を講義します：

<!--
* *Closures*, a function-like construct you can store in a variable
* *Iterators*, a way of processing a series of elements
* How to use these two features to improve the I/O project in Chapter 12
* The performance of these two features (Spoiler alert: they’re faster than you
might think!)
-->

* *クロージャ*、変数に保存できる関数に似た文法要素
* *イテレータ*、一連の要素を処理する方法
* これら 2 つの機能を使用して第 12 章の入出力プロジェクトを改善する方法
* これら 2 つの機能のパフォーマンス (ネタバレ：思ったよりも速いです)

<!--
Other Rust features, such as pattern matching and enums, which we've covered in
other chapters, are influenced by the functional style as well. Mastering
closures and iterators is an important part of writing idiomatic, fast Rust
code, so we’ll devote this entire chapter to them.
-->

パターンマッチングや enum など、他の Rust の機能も関数型に影響されていますが、他の章で講義してきました。
クロージャとイテレータをマスターすることは、慣用的で速い Rust コードを書くことの重要な部分なので、
この章を丸ごと捧げます。
