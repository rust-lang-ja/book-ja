<!--
# The Rust Programming Language
-->
# The Rust Programming Language

<!--
*by Steve Klabnik and Carol Nichols, with contributions from the Rust Community*
-->
*著：Steve Klabnik, Carol Nichols, 貢献：Rustコミュニティ*

<!--
This version of the text assumes you’re using Rust 1.41.0 or later with
`edition="2018"` in *Cargo.toml* of all projects to use Rust 2018 Edition
idioms. See the [“Installation” section of Chapter 1][install]
to install or update Rust, and see the new [Appendix E][editions]
for information on editions.
-->
このテキストのこの版では、Rust 2018 Editionのイディオムを使うため、Rust 1.41.0かそれ以降を使っており、すべてのプロジェクトの *Cargo.toml* に `edition="2018"` とあることを前提にしています。
Rustをインストールしたりアップデートするには[1章の「インストール」節][install]を、editionに関しては[付録E][editions]を読んでください。

<!--
The 2018 Edition of the Rust language includes a number of improvements that
make Rust more ergonomic and easier to learn. This iteration of the book
contains a number of changes to reflect those improvements:
-->
Rust言語の2018 Editionには、Rustをもっと使いやすく、学びやすくするための多くの改善点があります。
それらの改善点を反映するために、今回の改版は多くの変更点を含んでいます：

<!--
- Chapter 7, “Managing Growing Projects with Packages, Crates, and Modules,”
  has been mostly rewritten. The module system and the way paths work in the
  2018 Edition were made more consistent.
- Chapter 10 has new sections titled “Traits as Parameters” and “Returning
  Types that Implement Traits” that explain the new `impl Trait` syntax.
- Chapter 11 has a new section titled “Using `Result<T, E>` in Tests” that
  shows how to write tests that use the `?` operator.
- The “Advanced Lifetimes” section in Chapter 19 was removed because compiler
  improvements have made the constructs in that section even rarer.
- The previous Appendix D, “Macros,” has been expanded to include procedural
  macros and was moved to the “Macros” section in Chapter 19.
- Appendix A, “Keywords,” also explains the new raw identifiers feature that
  enables code written in the 2015 Edition and the 2018 Edition to interoperate.
- Appendix D is now titled “Useful Development Tools” and covers recently
  released tools that help you write Rust code.
- We fixed a number of small errors and imprecise wording throughout the book.
  Thank you to the readers who reported them!
-->
- 7章「肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する」はほとんど書き直されました。2018 Editionにおいてモジュールシステムとパスの仕組みはより一貫性を持つようになりました。
- 10章には、新しい`impl Trait`構文を説明するために "Traits as Parameters" と "Returning Types that Implement Traits" という新しい節があります。
- 11章には、`?`演算子を使うテストの書き方を説明する "Using `Result<T, E>` in Tests" という新しい節があります。
- 19章の「高度なライフタイム」節はなくなりました。コンパイラの改善により、この節の内容が現れることはいっそう稀になったからです。
- 付録D「マクロ」は、手続き的マクロも説明するようになり、19章の「マクロ」節に移動しました。
- 付録A「キーワード」では、2015 Editionと2018 Editionで書かれたコードを一緒に使えるようにしてくれる、raw identifiersという新しい機能も説明します。
- 付録Dは "Useful Development Tools" という名前に変わり、Rustコードを書く手助けになる最近登場したツールを説明します。
- 多くの細かい誤りや不正確な言葉遣いを直しました。報告してくれた読者の皆様、ありがとうございます！

<!--
Note that any code in earlier iterations of *The Rust Programming Language*
that compiled will continue to compile without `edition="2018"` in the
project’s *Cargo.toml*, even as you update the Rust compiler version you’re
using. That’s Rust’s backward compatibility guarantees at work!
-->
`edition="2018"` を *Cargo.toml* に書かなければ、Rustコンパイラのバージョンをアップデートしたとしても、 *The Rust Programming Language* のこれまでの版のコードは変わらずコンパイルできるということを知っておいてください。
Rustの後方互換性の約束は守られています！

<!--
The HTML format is available online at
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
and offline with installations of Rust made with `rustup`; run `rustup docs
--book` to open.
-->
HTML版は[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)で手に入ります。
オフラインで見たい場合は、`rustup`でインストールしたRustを使って`rustup docs --book`としてください。

<!--
This text is available in [paperback and ebook format from No Starch
Press][nsprust].
-->
このテキストの[ペーパーバック版と電子書籍版はNo Starch出版][nsprust]から発売されています。

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust
