<!--
## Comments
-->

## コメント

<!--
All programmers strive to make their code easy to understand, but sometimes
extra explanation is warranted. In these cases, programmers leave notes, or
*comments*, in their source code that the compiler will ignore but people
reading the source code may find useful.
-->

全プログラマは、自分のコードがわかりやすくなるよう努めますが、時として追加の説明が許されることもあります。
このような場合、プログラマは注釈または*コメント*をソースコードに残し、コメントをコンパイラは無視しますが、
ソースコードを読む人間には有益なものと思えるでしょう。

<!--
Here’s a simple comment:
-->

こちらが単純なコメントです：

```rust
// hello, world
```

<!--
In Rust, comments must start with two slashes and continue until the end of the
line. For comments that extend beyond a single line, you’ll need to include
`//` on each line, like this:
-->

Rust では、コメントは 2 連スラッシュで始め、行の終わりまで続きます。コメントが複数行にまたがる場合、
各行に`//`を含める必要があります。こんな感じに：

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
// ここで何か複雑なことをしていて、長すぎるから複数行のコメントが必要なんだ。
// ふう！願わくば、このコメントで何が起きているか説明されていると嬉しい。
```

<!--
Comments can also be placed at the end of lines containing code:
-->

コメントは、コードが書かれた行の末尾にも配置することができます：

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

<!--
But you’ll more often see them used in this format, with the comment on a
separate line above the code it's annotating:
-->

しかし、こちらの形式のコメントの方が見かける機会は多いでしょう。注釈しようとしているコードの 1 行上に書く形式です：

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

<!--
Rust also has another kind of comment, documentation comments, which we’ll
discuss in Chapter 14.
-->

Rust には他の種類のコメント、ドキュメントコメントもあり、それについては第 14 章で議論します。
