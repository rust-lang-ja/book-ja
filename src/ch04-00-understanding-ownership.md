<!--
# Understanding Ownership
-->

# 所有権を理解する

<!--
Ownership is Rust’s most unique feature and has deep implications for the rest
of the language. It enables Rust to make memory safety guarantees without
needing a garbage collector, so it’s important to understand how ownership
works. In this chapter, we’ll talk about ownership as well as several related
features: borrowing, slices, and how Rust lays data out in memory.
-->

所有権はRustの最もユニークな機能であり、それ以外の言語機能と深く関連しています。
所有権のおかげで、Rustはガベージコレクタの必要なしにメモリ安全性を担保することができていて、
そのため所有権の仕組みを理解しておくことが重要です。この章では、所有権以外にも、関連する機能を
いくつか話していきます: 借用、スライス、そして、コンパイラがデータをメモリにどう配置するかです。
