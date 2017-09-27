<!-- # Common Programming Concepts -->

# 一般的なプログラミングの概念

<!-- This chapter covers concepts that appear in almost every programming language -->
<!-- and how they work in Rust. Many programming languages have much in common at -->
<!-- their core. None of the concepts presented in this chapter are unique to Rust, -->
<!-- but we’ll discuss them in the context of Rust and explain their conventions. -->

この章では、ほとんど全てのプログラミング言語で見られる概念を解説し、それらがRustにおいて、
どう動作するかを見ていきましょう。多くのプログラミング言語は、その核心において、いろいろなものを共有しています。
この章で提示する概念は、全てRustに固有のものではありませんが、Rustの文脈で議論し、その規格を
解説していきます。

<!-- Specifically, you’ll learn about variables, basic types, functions, comments, -->
<!-- and control flow. These foundations will be in every Rust program, and learning -->
<!-- them early will give you a strong core to start from. -->

具体的には、変数、基本的な型、関数、コメント、そしてフロー制御について学びます。
これらの基礎は全てのRustプログラムに存在するものであり、それらを早期に学ぶことは
強力な基礎を築くことになるでしょう。

<!--  ### Keywords -->
<!--  -->
<!--  The Rust language has a set of *keywords* that have been reserved for use by -->
<!--  the language only, much like other languages do. Keep in mind that you cannot -->
<!--  use these words as names of variables or functions. Most of the keywords have -->
<!--  special meanings, and you’ll be using them to do various tasks in your Rust -->
<!--  programs; a few have no current functionality associated with them but have -->
<!--  been reserved for functionality that might be added to Rust in the future. You -->
<!--  can find a list of the keywords in Appendix A. -->

> ### キーワード
>
> Rust言語にも他の言語同様、キーワードが存在し、これらは言語だけが使用できるようになっています。
> これらの単語は、変数や関数名には使えないことを弁えておいてください。多数のキーワードは、特別な意味を持っており、
> 自らのRustプログラムにおいて、様々な作業をこなすために使用することができます;
> いくつかは、紐付けられた機能がないものの、将来Rustに追加されるかもしれない機能用に担保されています。
> キーワードの一覧は、付録Aで確認できます。

