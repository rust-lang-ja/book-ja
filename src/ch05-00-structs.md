<!--
# Using Structs to Structure Related Data
-->

# 構造体を使用して関係のあるデータを構造化する

<!--
A *struct*, or *structure*, is a custom data type that lets us name and
package together multiple related values that make up a meaningful group. If
you're familiar with an object-oriented language, a *struct* is like an
object's data attributes. In this chapter, we’ll compare and contrast tuples
with structs demonstrate how to use structs, and discuss how to define methods
and associated functions to specify behavior associated with a struct’s data.
Structs and enums (discussed in Chapter 6) are the building blocks for creating
new types in your program’s domain to take full advantage of Rust's compile
time type checking.
-->

*struct*または、*構造体*は、意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型です。
あなたがオブジェクト指向言語に造詣が深いなら、*struct*はオブジェクトのデータ属性みたいなものです。
この章では、タプルと構造体を対照的に比較し、構造体の使用法をデモし、メソッドや関連関数を定義して、
構造体のデータに紐付く振る舞いを指定する方法について議論します。構造体と*enum*(第 6 章で議論します) は、
自分のプログラム領域で新しい型を定義し、Rust のコンパイル時型精査機能をフル活用する構成要素になります。
