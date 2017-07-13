## コメント

全プログラマは、自分のコードがわかりやすくなるよう努めますが、時として追加の説明が許されることもあります。
このような場合、プログラマは注釈または*コメント*をソースコードに残し、コメントをコンパイラは無視しますが、
ソースコードを読む人間には便利なものと思えるでしょう。

こちらが単純なコメントです:

```rust
// Hello, world.
```

Rustでは、コメントは2連スラッシュで始め、行の終わりまで続きます。コメントが複数行にまたがる場合、
各行に`//`を含める必要があります。こんな感じに:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
// ここで何か複雑なことをしていて、長すぎるから複数行のコメントが必要なんだ。
// ふう！願わくば、このコメントで何が起きているか説明されていると嬉しい。
```

コメントは、コードが書かれた行の末尾にも配置することができます:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.(今日はラッキーな気がするよ)
}
```

しかし、こちらの形式のコメントの方が見かける機会は多いでしょう。注釈しようとしているコードの1行上に書く形式です:

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn main() {
    // I’m feeling lucky today.
    // 今日はラッキーな気がするよ
    let lucky_number = 7;
}
```

コメントについては以上。特に複雑ではなかったでしょ。
