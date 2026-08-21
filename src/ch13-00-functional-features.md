<!--
# Functional Language Features: Iterators and Closures
-->

# 関数型言語の機能: イテレータとクロージャ

<!--
Rust’s design has taken inspiration from many existing languages and
techniques, and one significant influence is *functional programming*.
Programming in a functional style often includes using functions as values by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth.
-->

Rustの設計は、多くの既存の言語やテクニックにインスピレーションを得ていて、
その一つの大きな影響が*関数型プログラミング*です。関数型でのプログラミングには、しばしば、
引数で渡したり、関数から関数を返したり、関数を後ほど使用するために変数に代入することで関数を値として使用することが含まれます。

<!--
In this chapter, we won’t debate the issue of what functional programming is or
isn’t but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.
-->

この章では、関数型プログラミングがどんなものであったり、なかったりするかという問題については議論しませんが、
代わりに関数型とよく言及される多くの言語の機能に似たRustの機能の一部について議論しましょう。

<!--
More specifically, we’ll cover:
-->

具体的には、以下を講義します:

<!--
* *Closures*, a function-like construct you can store in a variable
* *Iterators*, a way of processing a series of elements
* How to use closures and iterators to improve the I/O project in Chapter 12
* The performance of closures and iterators (Spoiler alert: they’re faster than
  you might think!)
-->

* *クロージャ*、変数に保存できる関数に似た文法要素
* *イテレータ*、一連の要素を処理する方法
* クロージャとイテレータを使用して第12章の入出力プロジェクトを改善する方法
* クロージャとイテレータのパフォーマンス(ネタバレ: 思ったよりも速いです)

<!--
We’ve already covered some other Rust features, such as pattern matching and
enums, that are also influenced by the functional style. Because mastering
closures and iterators is an important part of writing idiomatic, fast Rust
code, we’ll devote this entire chapter to them.
-->

パターンマッチングやenumなども関数型スタイルに影響を受けた他のRustの機能ですが、これらについてはすでに他の章で講義してきました。
クロージャとイテレータをマスターすることは、慣用的で速いRustコードを書くことの重要な部分なので、
この章を丸ごと捧げます。
