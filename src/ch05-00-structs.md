<!--
# Using Structs to Structure Related Data
-->

# 構造体を使用して関係のあるデータを構造化する

<!--
A *struct*, or *structure*, is a custom data type that lets you package
together and name multiple related values that make up a meaningful group. If
you’re familiar with an object-oriented language, a *struct* is like an
object’s data attributes. In this chapter, we’ll compare and contrast tuples
with structs to build on what you already know and demonstrate when structs are
a better way to group data.
-->

*struct*または、*構造体*は、意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型です。
あなたがオブジェクト指向言語に造詣が深いなら、*struct*はオブジェクトのデータ属性みたいなものです。
この章では、すでに学習したものに積み重ねる目的でタプルと構造体を対照的に比較し、データをまとめるのに構造体がより良い方法となるのはどういう場合かを示します。

<!--
We’ll demonstrate how to define and instantiate structs. We’ll discuss how to
define associated functions, especially the kind of associated functions called
*methods*, to specify behavior associated with a struct type. Structs and enums
(discussed in Chapter 6) are the building blocks for creating new types in your
program’s domain to take full advantage of Rust’s compile-time type checking.
-->

構造体を定義してインスタンス化する方法を実演します。
関連関数、特に*メソッド*と呼ばれる種類の関連関数を定義して、構造体型に紐付く振る舞いを指定する方法について議論します。
構造体と*enum*(第6章で議論します)は、自分のプログラム領域で新しい型を定義し、Rustのコンパイル時型精査機能をフル活用する構成要素になります。