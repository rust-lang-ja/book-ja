<!--
## Concise Control Flow with `if let`
-->

## `if let`で簡潔な制御フロー

<!--
The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern while ignoring the rest. Consider the
program in Listing 6-6 that matches on an `Option<u8>` value in the
`config_max` variable but only wants to execute code if the value is the `Some`
variant.
-->

`if let`記法で`if`と`let`をより冗長性の少ない方法で組み合わせ、残りを無視しつつ、一つのパターンにマッチする値を扱うことができます。
`config_max`変数の`Option<u8>`値にマッチするけれど、値が`Some`列挙子の時にだけコードを実行したい、リスト6-6のプログラムを考えてください。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-6: A `match` that only cares about executing
code when the value is `Some`</span>
-->

<span class="caption">リスト6-6: 値が`Some`の時だけコードを実行する`match`</span>

<!--
If the value is `Some`, we print out the value in the `Some` variant by binding
the value to the variable `max` in the pattern. We don’t want to do anything
with the `None` value. To satisfy the `match` expression, we have to add `_ =>
()` after processing just one variant, which is annoying boilerplate code to
add.
-->

値が`Some`の場合は、その`Some`列挙子の値をパターン内の変数`max`に束縛することで、それを出力します。
`None`値に対しては何もしたくありません。
`match`式の要件を満たすためには、列挙子を一つだけ処理した後に`_ => ()`を追加しなければなりませんが、これは煩わしい定型コードです。

<!--
Instead, we could write this in a shorter way using `if let`. The following
code behaves the same as the `match` in Listing 6-6:
-->

その代わり、`if let`を使用してもっと短く書くことができます。以下のコードは、
リスト6-6の`match`と同じように振る舞います:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

<!--
The syntax `if let` takes a pattern and an expression separated by an equal
sign. It works the same way as a `match`, where the expression is given to the
`match` and the pattern is its first arm. In this case, the pattern is
`Some(max)`, and the `max` binds to the value inside the `Some`. We can then
use `max` in the body of the `if let` block in the same way we used `max` in
the corresponding `match` arm. The code in the `if let` block isn’t run if the
value doesn’t match the pattern.
-->

`if let`という記法は等号記号で区切られたパターンと式を取り、式が`match`に与えられ、パターンが最初のアームになった`match`と同じ動作をします。
この場合は、パターンは`Some(max)`で`max`は`Some`内の値に束縛されます。
そうすると、対応する`match`アームの中で`max`を使用したのと全く同じように、`if let`ブロックの本体の中で`max`を使用することができます。
値がパターンにマッチしない場合は、`if let`ブロック内のコードは実行されません。

<!--
Using `if let` means less typing, less indentation, and less boilerplate code.
However, you lose the exhaustive checking that `match` enforces. Choosing
between `match` and `if let` depends on what you’re doing in your particular
situation and whether gaining conciseness is an appropriate trade-off for
losing exhaustive checking.
-->

`if let`を使うと、タイプ数が減り、インデントも少なくなり、定型コードも減ります。しかしながら、
`match`では強制された包括性チェックを失ってしまいます。`match`か`if let`かの選択は、
特定の場面でどんなことをしたいかと簡潔性を得ることが包括性チェックを失うのに適切な代償となるかによります。

<!--
In other words, you can think of `if let` as syntax sugar for a `match` that
runs code when the value matches one pattern and then ignores all other values.
-->

言い換えると、`if let`は値が一つのパターンにマッチした時にコードを走らせ、
他は無視する`match`への糖衣構文と考えることができます。

<!--
We can include an `else` with an `if let`. The block of code that goes with the
`else` is the same as the block of code that would go with the `_` case in the
`match` expression that is equivalent to the `if let` and `else`. Recall the
`Coin` enum definition in Listing 6-4, where the `Quarter` variant also held a
`UsState` value. If we wanted to count all non-quarter coins we see while also
announcing the state of the quarters, we could do that with a `match`
expression, like this:
-->

`if let`では、`else`を含むこともできます。`else`に入るコードブロックは、
`if let`と`else`に等価な`match`式の`_`の場合に入るコードブロックと同じになります。
リスト6-4の`Coin` enum定義を思い出してください。ここでは、`Quarter`列挙子は、
`UsState`の値も保持していましたね。クォーターコインの状態を告げつつ、
見かけたクォーター以外のコインの枚数を数えたいなら、以下のように`match`式で実現することができるでしょう:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

<!--
Or we could use an `if let` and `else` expression, like this:
-->

または、以下のように`if let`と`else`を使うこともできるでしょう:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

<!--
If you have a situation in which your program has logic that is too verbose to
express using a `match`, remember that `if let` is in your Rust toolbox as well.
-->

`match`を使って表現するには冗長的すぎるロジックがプログラムにあるようなシチュエーションに遭遇したら、
`if let`もRust道具箱にあることを思い出してください。

<!--
## Summary
-->

## まとめ

<!--
We’ve now covered how to use enums to create custom types that can be one of a
set of enumerated values. We’ve shown how the standard library’s `Option<T>`
type helps you use the type system to prevent errors. When enum values have
data inside them, you can use `match` or `if let` to extract and use those
values, depending on how many cases you need to handle.
-->

これで、enumを使用してワンセットの列挙された値のどれかになりうる独自の型を生成する方法を講義しました。
標準ライブラリの`Option<T>`が型システムを使用して、エラーを回避する際に役立つ方法についても示しました。
enumの値がデータを内部に含む場合、処理すべきケースの数に応じて、`match`か`if let`を使用して値を取り出し、
使用できます。

<!--
Your Rust programs can now express concepts in your domain using structs and
enums. Creating custom types to use in your API ensures type safety: the
compiler will make certain your functions only get values of the type each
function expects.
-->

もうRustプログラムで構造体とenumを使用して、自分の領域の概念を表現できます。API内で使用するために独自の型を生成することで、
型安全性を保証することができます: コンパイラが、各関数の予期する型の値のみを関数が得ることを確かめてくれるのです。

<!--
In order to provide a well-organized API to your users that is straightforward
to use and only exposes exactly what your users will need, let’s now turn to
Rust’s modules.
-->

使用するのに率直な整理整頓されたAPIをユーザに提供し、ユーザが必要とするものだけを公開するために、
今度は、Rustのモジュールに目を向けてみましょう。
