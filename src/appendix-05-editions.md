<!--
## Appendix E - Editions
-->
## 付録E：エディション

<!--
In Chapter 1, you saw that `cargo new` adds a bit of metadata to your
*Cargo.toml* file about an edition. This appendix talks about what that means!
-->
第1章で、`cargo new`が エディションに関するちょっとしたメタデータを *Cargo.toml* に追加しているのを見ましたね。
この付録ではその意味についてお話します！

<!--
The Rust language and compiler have a six-week release cycle, meaning users get
a constant stream of new features. Other programming languages release larger
changes less often; Rust releases smaller updates more frequently. After a
while, all of these tiny changes add up. But from release to release, it can be
difficult to look back and say, “Wow, between Rust 1.10 and Rust 1.31, Rust has
changed a lot!”
-->
Rust言語とコンパイラは6週間のリリースサイクルを採用しています。つまり、ユーザにはコンスタントに新しい機能が流れてくるというわけです。
他のプログラミング言語は、より少ない回数で、より大きなリリースを行いますが、Rustは小さなアップデートを頻繁に行います。
しばらくすると、これらの小さな変更が溜まっていきます。
しかし、これらを振り返って、「Rust 1.10とRust 1.31を比較すると、すごく変わったねえ！」などとリリースごとに言うのは難しいです。

<!--
Every two or three years, the Rust team produces a new Rust *edition*. Each
edition brings together the features that have landed into a clear package with
fully updated documentation and tooling. New editions ship as part of the usual
six-week release process.
-->
2、3年ごとに、RustチームはRustの新しい *エディション* を作ります。
それぞれのエディションには、それまでにRustにやってきた新しい機能が、完全に更新されたドキュメントとツール群とともに、一つのパッケージとなってまとめられています。
新しいエディションは通常の6週間ごとのリリースの一部として配布されます。

<!--
Editions serve different purposes for different people:
-->
それぞれの人々にとってエディションは異なる意味を持ちます。

<!--
* For active Rust users, a new edition brings together incremental changes into
  an easy-to-understand package.
* For non-users, a new edition signals that some major advancements have
  landed, which might make Rust worth another look.
* For those developing Rust, a new edition provides a rallying point for the
  project as a whole.
-->
* アクティブなRustユーザにとっては、新しいエディションは、少しずつ増えてきた変更点を理解しやすいパッケージにしてまとめるものです。
* Rustユーザでない人にとっては、新しいエディションは、何かしら大きな達成がなされたことを示します。Rustには今一度目を向ける価値があると感じさせるかもしれません。
* Rustを開発している人にとっては、新しいエディションは、プロジェクト全体の目標地点となります。

<!--
At the time of this writing, three Rust editions are available: Rust 2015, Rust
2018, and Rust 2021. This book is written using Rust 2021 edition idioms.
-->
この文書を書いている時点（訳注：原文のコミットは2021年12月23日）では、3つのRustのエディションが利用できます。
Rust 2015、Rust 2018、Rust 2021です。
この本はRust 2021エディションの慣例に従って書かれています。

<!--
The `edition` key in *Cargo.toml* indicates which edition the compiler should
use for your code. If the key doesn’t exist, Rust uses `2015` as the edition
value for backward compatibility reasons.
-->
*Cargo.toml* における`edition`キーは、コードに対してコンパイラがどのエディションを適用すべきかを示しています。
もしキーが存在しなければ、Rustは後方互換性のため`2015`をエディションの値として使います。

<!--
Each project can opt in to an edition other than the default 2015 edition.
Editions can contain incompatible changes, such as including a new keyword that
conflicts with identifiers in code. However, unless you opt in to those
changes, your code will continue to compile even as you upgrade the Rust
compiler version you use.
-->
標準の2015エディション以外のエディションを使うという選択はそれぞれのプロジェクトですることができます。
エディションには、コード内の識別子と衝突してしまう新しいキーワードの導入など、互換性のない変更が含まれる可能性があります。
しかし、それらの変更を選択しない限り、Rustのコンパイラのバージョンを更新しても、コードは変わらずコンパイルできます。

<!--
All Rust compiler versions support any edition that existed prior to that
compiler’s release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you’re using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.
-->
Rustコンパイラは全バージョンにおいて、そのコンパイラのリリースまでに存在したすべてのエディションをサポートしており、またサポートされているエディションのクレートはすべてリンクできます。
エディションの変更はコンパイラが最初にコードを構文解析するときにのみ影響します。
なので、あなたがRust 2015を使っていて、依存先にRust 2018を使うものがあったとしても、あなたのプロジェクトはコンパイルでき、その依存先を使うことができます。
逆に、あなたのプロジェクトがRust 2018を、依存先がRust 2015を使っていても、同じく問題はありません。

<!--
To be clear: most features will be available on all editions. Developers using
any Rust edition will continue to see improvements as new stable releases are
made. However, in some cases, mainly when new keywords are added, some new
features might only be available in later editions. You will need to switch
editions if you want to take advantage of such features.
-->
まあ実のところ、ほとんどの機能はすべてのエディションで利用可能でしょう。
どのRustエディションを使っている開発者も、新しい安定リリースが出ると改善したなと感じるのは変わらないでしょう。
しかし、場合によって（多くは新しいキーワードが追加されたとき）は、新機能が新しいエディションでしか利用できないことがあるかもしれません。
そのような機能を利用したいなら、エディションを切り替える必要があるでしょう。

<!--
For more details, the [*Edition
Guide*](https://doc.rust-lang.org/stable/edition-guide/) is a complete book
about editions that enumerates the differences between editions and explains
how to automatically upgrade your code to a new edition via `cargo fix`.
-->
より詳しく知りたいなら、[*エディションガイド*](https://doc.rust-lang.org/stable/edition-guide/)という、エディションに関するすべてを説明している本があります。
エディション同士の違いや、`cargo fix`を使って自動的にコードを新しいエディションにアップグレードする方法が書かれています。

> 訳注：日本語版のエディションガイドは[こちら](https://doc.rust-jp.rs/edition-guide/)にあります。
