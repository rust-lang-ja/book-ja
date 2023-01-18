<!--
# Understanding Ownership
-->

# 所有権を理解する

<!--
Ownership is Rust’s most unique feature, and it enables Rust to make memory
safety guarantees without needing a garbage collector. Therefore, it’s
important to understand how ownership works in Rust. In this chapter we’ll
talk about ownership as well as several related features: borrowing, slices,
and how Rust lays data out in memory.
-->

所有権は Rust の最もユニークな機能であり、これのおかげでガベージコレクタなしで安全性担保を行うことができるのです。
故に、Rust において、所有権がどう動作するのかを理解するのは重要です。この章では、所有権以外にも、関連する機能を
いくつか話していきます：借用、スライス、そして、コンパイラがデータをメモリにどう配置するかです。
