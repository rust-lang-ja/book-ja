<!-- # Using Structs to Structure Related Data -->

# 構造体を使用して関係のあるデータを構造化する

<!-- A *struct*, or *structure*, is a custom data type that lets us name and package -->
<!-- together multiple related values that make up a meaningful group. If you’re -->
<!-- familiar with an object-oriented language, a *struct* is like an object’s data -->
<!-- attributes. In this chapter, we’ll compare and contrast tuples with structs, -->
<!-- demonstrate how to use structs, and discuss how to define methods and -->
<!-- associated functions on structs to specify behavior associated with a struct’s -->
<!-- data. The struct and *enum* (which is discussed in Chapter 6) concepts are the -->
<!-- building blocks for creating new types in your program’s domain to take full -->
<!-- advantage of Rust’s compile time type checking. -->

*struct*または、*構造体*は、意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型です。
オブジェクト指向言語に造詣が深いなら、*struct*はオブジェクトのデータ属性みたいなものです。
この章では、タプルと構造体を対照的に比較し、構造体の使用法をデモし、構造体にメソッドや関連関数を定義して、
構造体のデータに紐付く振る舞いを指定する方法について議論します。構造体と*enum*(こちらは第6章で議論します)の概念は、
自分のプログラム領域で新しい型を定義し、Rustのコンパイル時型精査機能をフル活用する構成要素になります。
