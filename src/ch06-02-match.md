<!--
## The `match` Control Flow Operator
-->

## `match`制御フロー演算子

<!--
Rust has an extremely powerful control flow operator called `match` that allows
us to compare a value against a series of patterns and then execute code based
on which pattern matches. Patterns can be made up of literal values, variable
names, wildcards, and many other things; Chapter 18 covers all the different
kinds of patterns and what they do. The power of `match` comes from the
expressiveness of the patterns and the fact that the compiler confirms that all
possible cases are handled.
-->

Rust には、一連のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行させてくれる`match`と呼ばれる、
非常に強力な制御フロー演算子があります。パターンは、リテラル値、変数名、ワイルドカードやその他多数のもので構成することができます;
第 18 章で、全ての種類のパターンと、その目的については解説します。`match`のパワーは、
パターンの表現力とコンパイラが全てのありうるパターンを処理しているかを確認してくれるという事実に由来します。

<!--
Think of a `match` expression kind of like a coin sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value “fits,”
the value falls into the associated code block to be used during execution.
-->

`match`式をコイン並べ替え装置のようなものと考えてください：コインは、様々なサイズの穴が空いた通路を流れ落ち、
各コインは、サイズのあった最初の穴に落ちます。同様に、値は`match`の各パターンを通り抜け、値が「適合する」最初のパターンで、
値は紐付けられたコードブロックに落ち、実行中に使用されるわけです。

<!--
Because we just mentioned coins, let’s use them as an example using `match`! We
can write a function that can take an unknown United States coin and, in a
similar way as the counting machine, determine which coin it is and return its
value in cents, as shown here in Listing 6-3.
-->

コインについて話したので、それを`match`を使用する例にとってみましょう！数え上げ装置と同じ要領で未知のアメリカコインを一枚取り、
どの種類のコインなのか決定し、その価値をセントで返す関数をリスト 6-3 で示したように記述することができます。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

<!--
<span class="caption">Listing 6-3: An enum and a `match` expression that has
the variants of the enum as its patterns.</span>
-->

<span class="caption">リスト 6-3: enum とその enum の列挙子をパターンにした`match`式</span>

<!--
Let’s break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to an expression used with `if`, but there’s a
big difference: with `if`, the expression needs to return a Boolean value, but
here, it can be any type. The type of `coin` in this example is the `Coin` enum
that we defined on line 1.
-->

`value_in_cents`関数内の`match`を噛み砕きましょう。まず、`match`キーワードに続けて式を並べています。
この式は今回の場合、値`coin`です。`if`で使用した式と非常に酷似しているみたいですね。しかし、大きな違いがあります：
`if`では、式は論理値を返す必要がありますが、ここでは、どんな型でも構いません。この例における`coin`の型は、
1 行目で定義した`Coin` enum です。

<!--
Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.
-->

次は、`match`アームです。一本のアームには 2 つの部品があります：パターンと何らかのコードです。
今回の最初のアームは`Coin::Penny`という値のパターンであり、パターンと動作するコードを区別する`=>`演算子が続きます。
この場合のコードは、ただの値`1`です。各アームは次のアームとカンマで区切られています。

<!--
When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: in Listing 6-3, our `match` has four arms.
-->

この`match`式が実行されると、結果の値を各アームのパターンと順番に比較します。パターンに値がマッチしたら、
そのコードに紐付けられたコードが実行されます。パターンが値にマッチしなければ、コイン並べ替え装置と全く同じように、
次のアームが継続して実行されます。必要なだけパターンは存在できます：リスト 6-3 では、`match`には 4 本のアームがあります。

<!--
The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.
-->

各アームに紐付けられるコードは式であり、マッチしたアームの式の結果が`match`式全体の戻り値になります。

<!--
Curly brackets typically aren’t used if the match arm code is short, as it is
in Listing 6-3 where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you can use curly brackets. For example, the
following code would print out “Lucky penny!” every time the method was called with
a `Coin::Penny` but would still return the last value of the block, `1`:
-->

典型的に、アームのコードが短い場合、波かっこは使用されません。リスト 6-3 では、各アームが値を返すだけなので、
これに倣っています。マッチのアームで複数行のコードを走らせたいのなら、波かっこを使用することができます。
例えば、以下のコードは、メソッドが`Coin::Penny`とともに呼び出されるたびに「Lucky penny!」と表示しつつ、
ブロックの最後の値、`1`を返すでしょう。

```rust
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter,
# }
#
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

<!--
### Patterns that Bind to Values
-->

### 値に束縛されるパターン

<!--
Another useful feature of match arms is that they can bind to parts of the
values that match the pattern. This is how we can extract values out of enum
variants.
-->

マッチのアームの別の有益な機能は、パターンにマッチした値の一部に束縛できる点です。こうして、
enum の列挙子から値を取り出すことができます。

<!--
As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value stored
inside it, which we've done here in Listing 6-4.
-->

例として、enum の列挙子の一つを中にデータを保持するように変えましょう。1999 年から 2008 年まで、
アメリカは、片側に 50 の州それぞれで異なるデザインをしたクォーターコインを鋳造していました。
他のコインは州のデザインがなされることはなかったので、クォーターだけがこのおまけの値を保持します。
`Quarter`列挙子を変更して、`UsState`値が中に保持されるようにすることで`enum`にこの情報を追加でき、
それをしたのがリスト 6-4 のコードになります。

<!--
```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
Alabama,
Alaska,
// ... etc
}
-->

<!--
enum Coin {
Penny,
Nickel,
Dime,
Quarter(UsState),
}
```
-->

```rust
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

<!--
<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>
-->

<span class="caption">リスト 6-4: `Quarter`列挙子が`UsState`の値も保持する`Coin` enum</span>

<!--
Let’s imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type, we’ll also call out the name of
the state associated with each quarter so if it’s one our friend doesn’t have,
they can add it to their collection.
-->

友人の一人が 50 州全部のクォーターコインを収集しようとしているところを想像しましょう。コインの種類で小銭を並べ替えつつ、
友人が持っていない種類だったら、コレクションに追加できるように、各クォーターに関連した州の名前を出力します。

<!--
In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:
-->

このコードの match 式では、`Coin::Quarter`列挙子の値にマッチする`state`という名の変数をパターンに追加します。
`Coin::Quarter`がマッチすると、`state`変数はそのクォーターの state の値に束縛されます。それから、
`state`をそのアームのコードで使用できます。以下のようにですね：

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
#
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
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
`Coin::Quarter(UsState::Alaska)`になります。その値を match の各アームと比較すると、
`Coin::Quarter(state)`に到達するまで、どれにもマッチしません。その時に、`state`に束縛されるのは、
`UsState::Alaska`という値です。そして、`println!`式でその束縛を使用することができ、
そのため、`Coin` enum の列挙子から`Quarter`に対する中身の state の値を取得できたわけです。

<!--
### Matching with `Option<T>`
-->

### `Option<T>`とのマッチ

<!--
In the previous section we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match` as we
did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way that the `match` expression works remains
the same.
-->

前節では、`Option<T>`を使用する際に、`Some`ケースから中身の`T`の値を取得したくなりました。要するに、
`Coin` enum に対して行ったように、`match`を使って`Option<T>`を扱うこともできるというわけです！
コインを比較する代わりに、`Option<T>`の列挙子を比較するのですが、`match`式の動作の仕方は同じままです。

<!--
Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.
-->

`Option<i32>`を取る関数を書きたくなったとし、中に値があったら、その値に 1 を足すことにしましょう。
中に値がなければ、関数は`None`値を返し、何も処理を試みるべきではありません。

<!--
This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.
-->

`match`のおかげで、この関数は大変書きやすく、リスト 6-5 のような見た目になります。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

<!--
<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>
-->

<span class="caption">リスト 6-5: `Option<i32>`に`match`式を使う関数</span>

<!--
Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm.
-->

`plus_one`の最初の実行についてもっと詳しく検証しましょう。`plus_one(five)`と呼び出した時、
`plus_one`の本体の変数`x`は`Some(5)`になります。そして、これをマッチの各アームと比較します。

```rust,ignore
None => None,
```

<!--
The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm.
-->

`Some(5)`という値は、`None`というパターンにはマッチしませんので、次のアームに処理が移ります。

```rust,ignore
Some(i) => Some(i + 1),
```

<!--
Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add one to the value of `i` and
create a new `Some` value with our total `6` inside.
-->

<!--
Why yes が怪しい
-->

`Some(5)`は`Some(i)`にマッチしますか？なんと、します！列挙子が同じです。`i`は`Some`に含まれる値に束縛されるので、
`i`は値`5`になります。それから、このマッチのアームのコードが実行されるので、`i`の値に 1 を足し、
合計の`6`を中身にした新しい`Some`値を生成します。

<!--
Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm.
-->

さて、`x`が`None`になるリスト 6-5 の 2 回目の`plus_one`の呼び出しを考えましょう。`match`に入り、
最初のアームと比較します。

```rust,ignore
None => None,
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

`match`と enum の組み合わせは、多くの場面で有効です。Rust コードにおいて、このパターンはよく見かけるでしょう：
enum に対し`match`し、内部のデータに変数を束縛させ、それに基づいたコードを実行します。最初はちょっと巧妙ですが、
一旦慣れてしまえば、全ての言語にあってほしいと願うことになるでしょう。一貫してユーザのお気に入りなのです。

<!--
### Matches Are Exhaustive
-->

<!--
いい単語を探したい (Exhaustive)
-->

### マッチは包括的

<!--
There’s one other aspect of `match` we need to discuss. Consider this version
of our `plus_one` function that has a bug and won't compile:
-->

もう一つ議論する必要のある`match`の観点があります。一点バグがありコンパイルできないこんなバージョンの`plus_one`関数を考えてください：

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

<!--
We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:
-->

`None`の場合を扱っていないため、このコードはバグを生みます。幸い、コンパイラが捕捉できるバグです。
このコードのコンパイルを試みると、こんなエラーが出ます：

```text
error[E0004]: non-exhaustive patterns: `None` not covered
(エラー: 包括的でないパターン：`None`がカバーされてません)
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

<!--
Rust knows that we didn’t cover every possible case and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion dollar mistake discussed earlier.
-->

全可能性を網羅していないことをコンパイラは検知しています。もっと言えば、どのパターンを忘れているかさえ知っているのです。
Rust におけるマッチは、*包括的*です：全てのあらゆる可能性を網羅し尽くさなければ、コードは有効にならないのです。
特に`Option<T>`の場合には、私達が明示的に`None`の場合を処理するのを忘れないようにしてくれます。
null になるかもしれないのに値があると思い込まないよう、すなわち前に議論した 10 億ドルの失敗を犯さないよう、
コンパイラが保護してくれるわけです。

<!--
### The `_` Placeholder
-->

### `_`というプレースホルダー

<!--
Rust also has a pattern we can use when we don’t want to list all possible
values. For example, a `u8` can have valid values of 0 through 255. If we only
care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2,
4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the
special pattern `_` instead:
-->

Rust には、全ての可能性を列挙したくない時に使用できるパターンもあります。例えば、`u8`は、有効な値として、
0 から 255 までを取ります。1、3、5、7 の値にだけ興味があったら、0、2、4、6、8、9 と 255 までの数値を列挙する必要に迫られたくはないです。
幸運なことに、する必要はありません：代わりに特別なパターンの`_`を使用できます：

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

<!--
The `_` pattern will match any value. By putting it after our other arms, the
`_` will match all the possible cases that aren’t specified before it. The `()`
is just the unit value, so nothing will happen in the `_` case. As a result, we
can say that we want to do nothing for all the possible values that we don’t
list before the `_` placeholder.
-->

`_`というパターンは、どんな値にもマッチします。他のアームの後に記述することで、`_`は、
それまでに指定されていない全ての可能性にマッチします。`()`は、ただのユニット値なので、`_`の場合には、
何も起こりません。結果として、`_`プレースホルダーの前に列挙していない可能性全てに対しては、
何もしたくないと言えるわけです。

<!--
However, the `match` expression can be a bit wordy in a situation in which we
care about only *one* of the cases. For this situation, Rust provides `if let`.
-->

ですが、*一つ*のケースにしか興味がないような場面では、`match`式はちょっと長ったらしすぎます。
このような場面用に、Rust には、`if let`が用意されています。
