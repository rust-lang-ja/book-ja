<!--
# Advanced Features
-->

# 高度な機能

<!--
By now, you’ve learned the most commonly used parts of the Rust programming
language. Before we do one more project in Chapter 20, we’ll look at a few
aspects of the language you might run into every once in a while, but may not
use every day. You can use this chapter as a reference for when you encounter
any unknowns. The features covered here are useful in very specific situations.
Although you might not reach for them often, we want to make sure you have a
grasp of all the features Rust has to offer.
-->

今までに、Rustプログラミング言語の最もよく使われる部分を学んできました。第20章でもう1つ別のプロジェクトを行う前に、
時折遭遇するが、毎日は使わないような言語の側面をいくつか見ましょう。この章は、知らないことに遭遇した時に参考にすることができます。
ここで扱う機能は、かなり限定的な場面でしか役に立ちません。あまり頻繁には手を伸ばすことがない可能性はありますが、
Rustが提供しなければならない機能全ての概要を確かに把握してもらいたいのです。

<!--
In this chapter, we’ll cover:
-->

この章で講義するのは:

<!--
* Unsafe Rust: how to opt out of some of Rust’s guarantees and take
  responsibility for manually upholding those guarantees
* Advanced traits: associated types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced types: more about the newtype pattern, type aliases, the never type,
  and dynamically sized types
* Advanced functions and closures: function pointers and returning closures
* Macros: ways to define code that defines more code at compile time
-->

* Unsafe Rust: Rustの保証の一部を抜けてその保証を手動で保持する責任を負う方法
* 高度なトレイト: 関連型、デフォルト型引数、フルパス記法、スーパートレイト、トレイトに関連するニュータイプパターン
* 高度な型: ニュータイプパターンについてもっと、型エイリアス、never型、動的サイズ決定型
* 高度な関数とクロージャ: 関数ポインタとクロージャの返却
* マクロ: コンパイル時に、より多くのコードを定義するコードを定義する方法

<!--
It’s a panoply of Rust features with something for everyone! Let’s dive in!
-->

皆さんのための何かがあるRustの機能の盛大な儀式です！さあ、飛び込みましょう！
