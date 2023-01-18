<!--
## Refutability: Whether a Pattern Might Fail to Match
-->

## 論駁可能性：パターンが合致しないかどうか

<!--
Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are *irrefutable*. An example would be `x` in the
statement `let x = 5;` because `x` matches anything and therefore cannot fail
to match. Patterns that can fail to match for some possible value are
*refutable*. An example would be `Some(x)` in the expression `if let Some(x) =
a_value`; because if the value in `a_value` variable is `None` rather than
`Some`, the `Some(x)` pattern will not match.
-->

パターンには 2 つの形態があります：論駁可能なものと論駁不可能なものです。渡される可能性のあるあらゆる値に合致するパターンは、
*論駁不可能*なものです。文`let x = 5;`の`x`は一例でしょう。`x`は何にでも合致し、故に合致に失敗することがあり得ないからです。
なんらかの可能性のある値に対して合致しないことがあるパターンは、*論駁可能*なものです。
一例は、式`if let Some(x) = a_value`の`Some(x)`になるでしょう; `a_value`変数の値が`Some`ではなく、
`None`なら、`Some(x)`パターンは合致しないでしょうから。

<!--
Function parameters, `let` statements, and `for` loops can only accept
irrefutable patterns, because the program cannot do anything meaningful when
values don’t match. The `if let` and `while let` expressions only accept
refutable patterns, because by definition they’re intended to handle possible
failure: the functionality of a conditional is in its ability to perform
differently depending on success or failure.
-->

関数の引数、`let`文、`for`ループは、値が合致しなかったら何も意味のあることをプログラムが実行できないので、
論駁不可能なパターンしか受け付けられません。`if let`と`while let`式は、定義により失敗する可能性を処理することを意図したものなので、
論駁可能なパターンのみを受け付けます：条件式の機能は、成功か失敗によって異なる振る舞いをする能力にあるのです。

<!--
In general, you shouldn’t have to worry about the distinction between refutable
and irrefutable patterns; however, you do need to be familiar with the concept
of refutability so you can respond when you see it in an error message. In
those cases, you’ll need to change either the pattern or the construct you’re
using the pattern with, depending on the intended behavior of the code.
-->

一般的に、論駁可能と論駁不可能なパターンの差異について心配しなくてもいいはずです; しかしながら、
エラーメッセージで見かけた際に対応できるように、論駁可能性の概念に確かに慣れておく必要があります。
そのような場合には、コードの意図した振る舞いに応じて、パターンかパターンを使用している構文を変える必要があるでしょう。

<!--
Let’s look at an example of what happens when we try to use a refutable pattern
where Rust requires an irrefutable pattern and vice versa. Listing 18-8 shows a
`let` statement, but for the pattern we’ve specified `Some(x)`, a refutable
pattern. As you might expect, this code will not compile.
-->

コンパイラが論駁不可能なパターンを必要とする箇所で論駁可能なパターンを使用しようとしたら、何が起きるかとその逆の例を見ましょう。
リスト 18-8 は`let`文を示していますが、パターンには`Some(x)`と指定し、論駁可能なパターンです。
ご想像通りかもしれませんが、このコードはコンパイルできません。

```rust,ignore
let Some(x) = some_option_value;
```

<!--
<span class="caption">Listing 18-8: Attempting to use a refutable pattern with
`let`</span>
-->

<span class="caption">リスト 18-8: `let`で論駁可能なパターンを使用しようとする</span>

<!--
If `some_option_value` was a `None` value, it would fail to match the pattern
`Some(x)`, meaning the pattern is refutable. However, the `let` statement can
only accept an irrefutable pattern because there is nothing valid the code can
do with a `None` value. At compile time, Rust will complain that we’ve tried to
use a refutable pattern where an irrefutable pattern is required:
-->

`some_option_value`が`None`値だったなら、パターン`Some(x)`に合致しないことになり、パターンが論駁可能であることを意味します。
ですが、`let`文は論駁不可能なパターンしか受け付けられません。`None`値に対してコードができる合法なことは何もないからです。
コンパイル時にコンパイラは、論駁不可能なパターンが必要な箇所に論駁可能なパターンを使用しようとしたと文句を言うでしょう：

```text
error[E0005]: refutable pattern in local binding: `None` not covered
(エラー: ローカル束縛に論駁可能なパターン：`None`がカバーされていません)
 -->
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

<!--
Because we didn’t cover (and couldn’t cover!) every valid value with the
pattern `Some(x)`, Rust rightfully produces a compiler error.
-->

パターン`Some(x)`で全ての合法な値をカバーしなかった (できませんでした！) ので、
コンパイラは当然、コンパイルエラーを生成します。

<!--
To fix the problem where we have a refutable pattern where an irrefutable
pattern is needed, we can change the code that uses the pattern: instead of
using `let`, we can use `if let`. Then if the pattern doesn’t match, the code
will just skip the code in the curly brackets, giving it a way to continue
validly. Listing 18-9 shows how to fix the code in Listing 18-8.
-->

論駁不可能なパターンが必要な箇所に論駁可能なパターンがある問題を修正するには、パターンを使用するコードを変えればいいのです：
`let`の代わりに`if let`を使用できます。そして、パターンが合致しなかったら、コードは合法に継続する手段を残して、
波括弧内のコードを飛ばすだけでしょう。リスト 18-9 は、リスト 18-8 のコードの修正方法を示しています。

```rust
# let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

<!--
<span class="caption">Listing 18-9: Using `if let` and a block with refutable
patterns instead of `let`</span>
-->

<span class="caption">リスト 18-9: `let`ではなく、`if let`と論駁可能なパターンを含むブロックを使用する</span>

<!--
We’ve given the code an out! This code is perfectly valid, although it means we
cannot use an irrefutable pattern without receiving an error. If we give `if
let` a pattern that will always match, such as `x`, as shown in Listing 18-10,
it will not compile.
-->

コードに逃げ道を与えました！このコードは完全に合法ですが、エラーを受け取らないで論駁不可能なパターンを使用することはできないことを意味します。
リスト 18-10 のように、`x`のような常にマッチするパターンを`if let`に与えたら、コンパイルできないでしょう。

```rust,ignore
if let x = 5 {
    println!("{}", x);
};
```

<!--
<span class="caption">Listing 18-10: Attempting to use an irrefutable pattern
with `if let`</span>
-->

<span class="caption">リスト 18-10: `if let`で論駁不可能なパターンを使用してみる</span>

<!--
Rust complains that it doesn’t make sense to use `if let` with an irrefutable
pattern:
-->

コンパイラは、論駁不可能なパターンと`if let`を使用するなんて道理が通らないと文句を言います：

```text
error[E0162]: irrefutable if-let pattern
(エラー: 論駁不可能な if-let パターン)
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

<!--
For this reason, match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern. Rust
allows us to use an irrefutable pattern in a `match` with only one arm, but
this syntax isn’t particularly useful and could be replaced with a simpler
`let` statement.
-->

このため、マッチアームは、論駁不可能なパターンで残りのあらゆる値に合致すべき最後のアームを除いて、
論駁可能なパターンを使用しなければなりません。コンパイラは、たった 1 つしかアームのない`match`で論駁不可能なパターンを使用させてくれますが、
この記法は特別有用なわけではなく、より単純な`let`文に置き換えることもできるでしょう。

<!--
Now that you know where to use patterns and the difference between refutable
and irrefutable patterns, let’s cover all the syntax we can use to create
patterns.
-->

今やパターンを使用すべき箇所と論駁可能と論駁不可能なパターンの違いを知ったので、
パターンを生成するために使用できる全ての記法を講義しましょう。
