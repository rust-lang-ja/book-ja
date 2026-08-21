<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="the-match-control-flow-operator"></a>
-->

<!--
## The `match` Control Flow Construct
-->

## `match`制御フロー構造

<!--
Rust has an extremely powerful control flow construct called `match` that
allows you to compare a value against a series of patterns and then execute
code based on which pattern matches. Patterns can be made up of literal values,
variable names, wildcards, and many other things; [Chapter
18][ch18-00-patterns] covers all the different kinds of patterns
and what they do. The power of `match` comes from the expressiveness of the
patterns and the fact that the compiler confirms that all possible cases are
handled.
-->

Rustには、一連のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行させてくれる`match`と呼ばれる、
非常に強力な制御フロー構造があります。パターンは、リテラル値、変数名、ワイルドカードやその他多数のもので構成することができます;
[第18章][ch18-00-patterns]で、全ての種類のパターンと、その目的については解説します。`match`のパワーは、
パターンの表現力とコンパイラが全てのありうるパターンを処理しているかを確認してくれるという事実に由来します。

<!--
Think of a `match` expression as being like a coin-sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value “fits,”
the value falls into the associated code block to be used during execution.
-->

`match`式をコイン並べ替え装置のようなものと考えてください: コインは、様々なサイズの穴が空いた通路を流れ落ち、
各コインは、サイズのあった最初の穴に落ちます。同様に、値は`match`の各パターンを通り抜け、値が「適合する」最初のパターンで、
値は紐付けられたコードブロックに落ち、実行中に使用されるわけです。

<!--
Speaking of coins, let’s use them as an example using `match`! We can write a
function that takes an unknown US coin and, in a similar way as the counting
machine, determines which coin it is and returns its value in cents, as shown
in Listing 6-3.
-->

せっかくコインについて話したので、それを`match`を使用する例にとってみましょう！数え上げ装置と同じ要領で未知のアメリカコインを一枚取り、
どの種類のコインなのか決定し、その価値をセントで返す関数をリスト6-3で示したように記述することができます。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-3: An enum and a `match` expression that has
the variants of the enum as its patterns</span>
-->

<span class="caption">リスト6-3: enumとそのenumの列挙子をパターンにした`match`式</span>

<!--
Let’s break down the `match` in the `value_in_cents` function. First we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to a conditional expression used with `if`, but
there’s a big difference: with `if`, the condition needs to evaluate to a
Boolean value, but here it can be any type. The type of `coin` in this example
is the `Coin` enum that we defined on the first line.
-->

`value_in_cents`関数内の`match`を噛み砕きましょう。まず、`match`キーワードに続けて式を並べています。
この式は今回の場合、値`coin`です。`if`で使用した条件式と非常に酷似しているみたいですね。しかし、大きな違いがあります:
`if`では、条件は論理値に評価される必要がありますが、ここでは、どんな型でも構いません。この例における`coin`の型は、
1行目で定義した`Coin` enumです。

<!--
Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.
-->

次は、`match`アームです。一本のアームには2つの部品があります: パターンと何らかのコードです。
今回の最初のアームは`Coin::Penny`という値のパターンであり、パターンと動作するコードを区別する`=>`演算子が続きます。
この場合のコードは、ただの値`1`です。各アームは次のアームとカンマで区切られています。

<!--
When the `match` expression executes, it compares the resultant value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: in Listing 6-3, our `match` has four arms.
-->

この`match`式が実行されると、結果の値を各アームのパターンと順番に比較します。パターンに値がマッチしたら、
そのコードに紐付けられたコードが実行されます。パターンが値にマッチしなければ、コイン並べ替え装置と全く同じように、
次のアームが継続して実行されます。必要なだけパターンは存在できます: リスト6-3では、`match`には4本のアームがあります。

<!--
The code associated with each arm is an expression, and the resultant value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.
-->

各アームに紐付けられるコードは式であり、マッチしたアームの式の結果が`match`式全体の戻り値になります。

<!--
We don’t typically use curly brackets if the match arm code is short, as it is
in Listing 6-3 where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you must use curly brackets, and the comma
following the arm is then optional. For example, the following code prints
“Lucky penny!” every time the method is called with a `Coin::Penny`, but still
returns the last value of the block, `1`:
-->

典型的に、アームのコードが短い場合、波かっこは使用しません。リスト6-3では、各アームが値を返すだけなので、
これに倣っています。マッチのアームで複数行のコードを走らせたいのなら、波かっこを使わなくてはなりませんが、
この場合アームの後のカンマは省略することができます。
例えば、以下のコードは、メソッドが`Coin::Penny`とともに呼び出されるたびに「Lucky penny!」と表示しつつ、
ブロックの最後の値、`1`を返します。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!--
### Patterns That Bind to Values
-->

### 値に束縛されるパターン

<!--
Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.
-->

マッチのアームの別の有益な機能は、パターンにマッチした値の一部に束縛できる点です。こうして、
enumの列挙子から値を取り出すことができます。

<!--
As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value
stored inside it, which we’ve done in Listing 6-4.
-->

例として、enumの列挙子の一つを中にデータを保持するように変えましょう。1999年から2008年まで、
アメリカは、片側に50の州それぞれで異なるデザインをしたクォーターコインを鋳造していました。
他のコインは州のデザインがなされることはなかったので、クォーターだけがこのおまけの値を保持します。
`Quarter`列挙子を変更して、`UsState`値が中に保持されるようにすることで`enum`にこの情報を追加でき、
それをしたのがリスト6-4のコードになります。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>
-->

<span class="caption">リスト6-4: `Quarter`列挙子が`UsState`の値も保持する`Coin` enum</span>

<!--
Let’s imagine that a friend is trying to collect all 50 state quarters. While
we sort our loose change by coin type, we’ll also call out the name of the
state associated with each quarter so that if it’s one our friend doesn’t have,
they can add it to their collection.
-->

友人の一人が50州全部のクォーターコインを収集しようとしているところを想像しましょう。コインの種類で小銭を並べ替えつつ、
友人が持っていない種類だったら、コレクションに追加できるように、各クォーターに関連した州の名前を出力します。

<!--
In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:
-->

このコードのmatch式では、`Coin::Quarter`列挙子の値にマッチする`state`という名の変数をパターンに追加します。
`Coin::Quarter`がマッチすると、`state`変数はそのクォーターのstateの値に束縛されます。それから、
`state`をそのアームのコードで使用できます。以下のようにですね:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

<!--
If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.
-->

`value_in_cents(Coin::Quarter(UsState::Alaska))`と呼び出すつもりだったなら、`coin`は
`Coin::Quarter(UsState::Alaska)`になります。その値をmatchの各アームと比較すると、
`Coin::Quarter(state)`に到達するまで、どれにもマッチしません。その時に、`state`に束縛されるのは、
`UsState::Alaska`という値です。そして、`println!`式でその束縛を使用することができ、
そのため、`Coin` enumの列挙子から`Quarter`に対する中身のstateの値を取得できたわけです。

<!--
### Matching with `Option<T>`
-->

### `Option<T>`とのマッチ

<!--
In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match`, as
we did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way the `match` expression works remains the
same.
-->

前節では、`Option<T>`を使用する際に、`Some`ケースから中身の`T`の値を取得したくなりました。要するに、
`Coin` enumに対して行ったように、`match`を使って`Option<T>`を扱うこともできるというわけです！
コインを比較する代わりに、`Option<T>`の列挙子を比較するのですが、`match`式の動作の仕方は同じままです。

<!--
Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.
-->

`Option<i32>`を取る関数を書きたくなったとし、中に値があったら、その値に1を足すことにしましょう。
中に値がなければ、関数は`None`値を返し、何も処理を試みるべきではありません。

<!--
This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.
-->

`match`のおかげで、この関数は大変書きやすく、リスト6-5のような見た目になります。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>
-->

<span class="caption">リスト6-5: `Option<i32>`に`match`式を使う関数</span>

<!--
Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm:
-->

`plus_one`の最初の実行についてもっと詳しく検証しましょう。`plus_one(five)`と呼び出した時、
`plus_one`の本体の変数`x`は`Some(5)`になります。そして、これをマッチの各アームと比較します:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm:
-->

`Some(5)`という値は、`None`というパターンにはマッチしませんので、次のアームに処理が移ります:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

<!--
Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i`
binds to the value contained in `Some`, so `i` takes the value `5`. The code in
the match arm is then executed, so we add 1 to the value of `i` and create a
new `Some` value with our total `6` inside.
-->


`Some(5)`は`Some(i)`にマッチしますか？しますね！列挙子が同じです。`i`は`Some`に含まれる値に束縛されるので、
`i`は値`5`になります。それから、このマッチのアームのコードが実行されるので、`i`の値に1を足し、
合計の`6`を中身にした新しい`Some`値を生成します。

<!--
Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm:
-->

さて、`x`が`None`になるリスト6-5の2回目の`plus_one`の呼び出しを考えましょう。`match`に入り、
最初のアームと比較します:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.
-->

マッチします！足し算する値がないので、プログラムは停止し、`=>`の右辺にある`None`値が返ります。
最初のアームがマッチしたため、他のアームは比較されません。

<!--
Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.
-->

`match`とenumの組み合わせは、多くの場面で有効です。Rustコードにおいて、このパターンはよく見かけるでしょう:
enumに対し`match`し、内部のデータに変数を束縛させ、それに基づいたコードを実行します。最初はちょっと巧妙ですが、
一旦慣れてしまえば、全ての言語にあってほしいと願うことになるでしょう。一貫してユーザのお気に入りなのです。

<!--
### Matches Are Exhaustive
-->

<!--
いい単語を探したい(Exhaustive)
-->

### マッチは包括的

<!--
There’s one other aspect of `match` we need to discuss: the arms’ patterns must
cover all possibilities. Consider this version of our `plus_one` function,
which has a bug and won’t compile:
-->

もう一つ議論する必要のある`match`の観点があります: アームのパターンはすべての可能性を網羅しなくてはなりません。
こんなバージョンの`plus_one`関数を考えてください、これにはバグがありコンパイルできないでしょう: 

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

<!--
We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:
-->

`None`の場合を扱っていないため、このコードはバグを生みます。幸い、コンパイラが捕捉できるバグです。
このコードのコンパイルを試みると、こんなエラーが出ます:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

<!--
Rust knows that we didn’t cover every possible case, and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.
-->

全可能性を網羅していないことをコンパイラは検知しています。もっと言えば、どのパターンを忘れているかさえ知っているのです。
Rustにおけるマッチは、*包括的*です: 全てのあらゆる可能性を網羅し尽くさなければ、コードは有効にならないのです。
特に`Option<T>`の場合には、私達が明示的に`None`の場合を処理するのを忘れないようにしてくれます。
nullになるかもしれないのに値があると思い込まないよう、すなわち前に議論した10億ドルの失敗を犯すことができないよう、
コンパイラが保護してくれるわけです。

<!--
### Catch-all Patterns and the `_` Placeholder
-->

### catch-allパターンとプレースホルダー（`_`）

<!--
Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we’re implementing a game
where, if you roll a 3 on a dice roll, your player doesn’t move, but instead
gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here’s
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:
-->

enumを使用することで、いくつかの特定の値に対して特別な操作を行うが、他のすべての値に対してはデフォルトの操作を行う、ということができます。
ゲームを実装しているところを想像してください。
サイコロを振って3の目が出たら、そのプレイヤーは移動できませんが、代わりにおしゃれな帽子をもらえます。
サイコロを振って7の目が出たら、そのプレイヤーはおしゃれな帽子を失います。
他のすべての値については、そのプレイヤーはゲーム盤上で同じ数だけマスを移動します。
以下はこのロジックを実装する`match`です。
ただしサイコロを振った結果はランダム値ではなくハードコードされており、また他のすべてのロジックは、それを実際に実装するのは本題ではないので、本体の無い関数によって表現されています:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

<!--
For the first two arms, the patterns are the literal values `3` and `7`. For
the last arm that covers every other possible value, the pattern is the
variable we’ve chosen to name `other`. The code that runs for the `other` arm
uses the variable by passing it to the `move_player` function.
-->

最初の2つのアームについては、パターンはリテラル値`3`および`7`です。
他のあらゆる可能な値を網羅する最後のアームについては、パターンは変数で、この変数には`other`と名付けることを選びました。
`other`アームで実行されるコードは、`move_player`関数にこの変数を渡すことで、この変数を使用しています。

<!--
This code compiles, even though we haven’t listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. If we put the catch-all arm earlier, the other
arms would never run, so Rust will warn us if we add arms after a catch-all!
-->

`u8`が取りうるすべての値を列挙していないにも関わらず、このコードはコンパイルできます。
最後のパターンが、個別に列挙していないすべての値にマッチするからです。
このcatch-allパターンのおかげで、`match`は包括的でなくてはならないという必要条件が満たされます。
パターンは順に評価されるので、catch-allアームは最後に書く必要があることに注意してください。
catch-allアームを先に書いてしまうと他のアームは絶対に実行されなくなってしまうため、
catch-allの後にアームを追加するとコンパイラが警告を発するでしょう！

<!--
Rust also has a pattern we can use when we want a catch-all but don’t want to
*use* the value in the catch-all pattern: `_` is a special pattern that matches
any value and does not bind to that value. This tells Rust we aren’t going to
use the value, so Rust won’t warn us about an unused variable.
-->

Rustには、catch-allしたいが、catch-allパターン内で値を*使用*したくない時に使用できるパターンもあります:
`_`は任意の値にマッチし、その値を束縛しない特別なパターンです。
これはコンパイラにその値を使用しないということを伝えるので、コンパイラは未使用の変数についての警告を発しなくなるしょう。

<!--
Let’s change the rules of the game: now, if you roll anything other than a 3 or
a 7, you must roll again. We no longer need to use the catch-all value, so we
can change our code to use `_` instead of the variable named `other`:
-->

ゲームのルールを変更しましょう: これからは、3または7以外の目を出したら、もう一度サイコロを振らなくてはなりません。
catch-allの値を使用する必要がなくなるので、`other`変数の代わりに`_`を使用するようにコードを変更します:


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

<!--
This example also meets the exhaustiveness requirement because we’re explicitly
ignoring all other values in the last arm; we haven’t forgotten anything.
-->

この例もまた、最後のアームで明示的にすべての他の値を無視しているので、網羅性要件を満たしています;
見落としている場合分けはありません。

<!--
Finally, we’ll change the rules of the game one more time so that nothing else
happens on your turn if you roll anything other than a 3 or a 7. We can express
that by using the unit value (the empty tuple type we mentioned in [“The Tuple
Type”][tuples] section) as the code that goes with the `_` arm:
-->

最後に、もう一度だけゲームのルールを変更することにします。
3または7以外の目を出したら、プレイヤーの番には何も起きません。
以下の`_`アームのコードのように、ユニット値（[「タプル型」][tuples]節で説明した空タプル型）を使用することでこれを表現できます:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

<!--
Here, we’re telling Rust explicitly that we aren’t going to use any other value
that doesn’t match a pattern in an earlier arm, and we don’t want to run any
code in this case.
-->

このコードでは、先の方のアームのパターンにマッチしないあらゆる値は使用せず、
この場合にはいかなるコードも実行したくないということを、コンパイラに明示的に伝えています。

<!--
There’s more about patterns and matching that we’ll cover in [Chapter
18][ch18-00-patterns].  For now, we’re going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.
-->

パターンとマッチングについては[第18章][ch18-00-patterns]でさらに深く取り扱います。
ひとまず、`match`式ではちょっと長ったらしいという状況で便利かもしれない、`if let`構文に進むことにましょう。

<!--
[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
-->

[tuples]: ch03-02-data-types.html#タプル型
[ch18-00-patterns]: ch18-00-patterns.html
