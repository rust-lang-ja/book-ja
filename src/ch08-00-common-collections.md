<!--
# Common Collections
-->

# 一般的なコレクション

<!--
Rust’s standard library includes a number of very useful data structures called
*collections*. Most other data types represent one specific value, but
collections can contain multiple values. Unlike the built-in array and tuple
types, the data these collections point to is stored on the heap, which means
the amount of data does not need to be known at compile time and can grow or
shrink as the program runs. Each kind of collection has different capabilities
and costs, and choosing an appropriate one for your current situation is a
skill you’ll develop over time. In this chapter, we’ll discuss three
collections that are used very often in Rust programs:
-->

Rust の標準ライブラリは、*コレクション*と呼ばれる多くの非常に有益なデータ構造を含んでいます。他の多くのデータ型は、
ある一つの値を表しますが、コレクションは複数の値を含むことができます。組み込みの配列とタプル型とは異なり、
これらのコレクションが指すデータはヒープに確保され、データ量はコンパイル時にわかる必要はなく、
プログラムの実行にあわせて、伸縮可能であることになります。各種のコレクションには異なる能力とコストが存在し、
自分の現在の状況に最適なものを選び取るスキルは、時間とともに育っていきます。この章では、
Rust のプログラムにおいて、非常に頻繁に使用される 3 つのコレクションについて議論しましょう。

<!--
* A *vector* allows us to store a variable number of values next to each other.
* A *string* is a collection of characters. We’ve mentioned the `String` type
previously, but in this chapter we’ll talk about it in depth.
* A *hash map* allows us to associate a value with a particular key. It’s a
particular implementation of the more general data structure called a *map*.
-->

* *ベクタ型*は、可変長の値を並べて保持できる。
* *文字列*は、文字のコレクションである。以前、`String`型について触れたが、
この章ではより掘り下げていく。
* *ハッシュマップ*は、値を特定のキーと紐付けさせてくれる。より一般的なデータ構造である、
*マップ*の特定の実装である。

<!--
To learn about the other kinds of collections provided by the standard library,
see [the documentation][collections].
-->

標準ライブラリで提供されている他の種のコレクションについて学ぶには、
[ドキュメント][collections]を参照されたし。

[collections]: https://doc.rust-lang.org/std/collections/index.html

<!--
We’ll discuss how to create and update vectors, strings, and hash maps, as well
as what makes each special.
-->

ベクタ型、文字列、ハッシュマップの生成と更新方法や、各々が特別な点について議論していきましょう。
