<!--
# Object-Oriented Programming Features of Rust
-->

# Rustのオブジェクト指向プログラミング機能

<!--
Object-oriented programming (OOP) is a way of modeling programs. Objects as a
programmatic concept were introduced in the programming language Simula in the
1960s. Those objects influenced Alan Kay’s programming architecture in which
objects pass messages to each other. To describe this architecture, he coined
the term *object-oriented programming* in 1967. Many competing definitions
describe what OOP is, and by some of these definitions Rust is object-oriented,
but by others it is not. In this chapter, we’ll explore certain characteristics
that are commonly considered object-oriented and how those characteristics
translate to idiomatic Rust. We’ll then show you how to implement an
object-oriented design pattern in Rust and discuss the trade-offs of doing so
versus implementing a solution using some of Rust’s strengths instead.
-->

オブジェクト指向プログラミング(OOP)は、プログラムをモデル化する手段です。プログラム上の概念としてのオブジェクトは、
1960年代のプログラミング言語Simulaで導入されました。このオブジェクトは、
お互いにメッセージを渡し合うというアラン・ケイ(Alan Kay)のプログラミングアーキテクチャに影響を及ぼしました。
彼は、このアーキテクチャを記述するために、*オブジェクト指向プログラミング*という用語を造語しました。
多くの競合する定義がOOPが何かを記述しており、こうした定義の一部によれば、Rustはオブジェクト指向であり
他の定義によれば、Rustはオブジェクト指向ではありません。この章では、広くオブジェクト指向と捉えられる特定の特徴と、
それらの特徴がこなれたRustでどう表現されるかを探究します。それからオブジェクト指向のデザインパターンをRustで実装する方法を示し、
そうすることとRustの強みを活用して代わりの解決策を実装する方法の代償を議論します。
