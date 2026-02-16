<!--
## Pattern Syntax
-->

## パターン記法

<!--
In this section, we gather all the syntax valid in patterns and discuss why and
when you might want to use each one.
-->

この節ではパターンとして有効な記法をすべて集め、それぞれについて使用するとよい理由と状況について議論します。

<!--
### Matching Literals
-->

### リテラルにマッチする

<!--
As you saw in Chapter 6, you can match patterns against literals directly. The
following code gives some examples:
-->

第6章で目撃したように、パターンを直接リテラルに合致させられます。以下のコードが例を挙げています:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

<!--
This code prints `one` because the value in `x` is 1. This syntax is useful
when you want your code to take an action if it gets a particular concrete
value.
-->

このコードは、`x`の値が1なので、`one`を出力します。この記法は、コードが特定の具体的な値を得た時に行動を起こしてほしい時に有用です。

<!--
### Matching Named Variables
-->

### 名前付き変数にマッチする

<!--
Named variables are irrefutable patterns that match any value, and we’ve used
them many times in the book. However, there is a complication when you use
named variables in `match` expressions. Because `match` starts a new scope,
variables declared as part of a pattern inside the `match` expression will
shadow those with the same name outside the `match` construct, as is the case
with all variables. In Listing 18-11, we declare a variable named `x` with the
value `Some(5)` and a variable `y` with the value `10`. We then create a
`match` expression on the value `x`. Look at the patterns in the match arms and
`println!` at the end, and try to figure out what the code will print before
running this code or reading further.
-->

名前付き変数はどんな値にも合致する論駁不可能なパターンであり、この本の中で何度も使用してきました。
ですが、名前付き変数を`match`式で使うと、厄介な問題があります。`match`は新しいスコープを開始するので、
`match`式内のパターンの一部として宣言された変数は、あらゆる変数同様に`match`構文外部の同じ名前の変数を覆い隠します。
リスト18-11で、値`Some(5)`の`x`という変数と値`10`の変数`y`を宣言しています。それから値`x`に対して`match`式を生成します。
マッチアームのパターンと最後の`println!`を見て、このコードを実行したり、先まで読み進める前にこのコードが何を出力するか推測してみてください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-11: A `match` expression with an arm that
introduces a shadowed variable `y`</span>
-->

<span class="caption">リスト18-11: シャドーイングされた変数`y`を導入するアームのある`match`式</span>

<!--
Let’s walk through what happens when the `match` expression runs. The pattern
in the first match arm doesn’t match the defined value of `x`, so the code
continues.
-->

`match`式を実行した時に起こることを見ていきましょう。最初のマッチアームのパターンは、`x`の定義された値に合致しないので、
コードは継続します。

<!--
The pattern in the second match arm introduces a new variable named `y` that
will match any value inside a `Some` value. Because we’re in a new scope inside
the `match` expression, this is a new `y` variable, not the `y` we declared at
the beginning with the value 10. This new `y` binding will match any value
inside a `Some`, which is what we have in `x`. Therefore, this new `y` binds to
the inner value of the `Some` in `x`. That value is `5`, so the expression for
that arm executes and prints `Matched, y = 5`.
-->

2番目のマッチアームのパターンは、`Some`値内部のあらゆる値に合致する新しい`y`という変数を導入します。
`match`式内の新しいスコープ内にいるので、これは新しい`y`変数であり、最初に値10で宣言した`y`ではありません。
この新しい`y`束縛は、`Some`内のあらゆる値に合致し、`x`にあるものはこれです。故に、この新しい`y`は、
`x`の中身の値に束縛されます。その値は`5`なので、そのアームの式が実行され、`Matched, y = 5`と出力されます。

<!--
If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
two arms wouldn’t have matched, so the value would have matched to the
underscore. We didn’t introduce the `x` variable in the pattern of the
underscore arm, so the `x` in the expression is still the outer `x` that hasn’t
been shadowed. In this hypothetical case, the `match` would print `Default
case, x = None`.
-->

`x`が`Some(5)`ではなく`None`値だったなら、最初の2つのアームのパターンはマッチしなかったので、
値はアンダースコアに合致したでしょう。アンダースコアのアームのパターンでは`x`変数を導入しなかったので、
その式の`x`は、まだシャドーイングされない外側の`x`のままです。この架空の場合、
`match`は`Default case, x = None`と出力するでしょう。

<!--
When the `match` expression is done, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.
-->

`match`式が完了すると、スコープが終わるので、中の`y`のスコープも終わります。
最後の`println!`は`at the end: x = Some(5), y = 10`を生成します。

<!--
To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a shadowed variable, we would need to use a match
guard conditional instead. We’ll talk about match guards later in the [“Extra
Conditionals with Match Guards”](#extra-conditionals-with-match-guards)
section.
-->

シャドーイングされた変数を導入するのではなく、外側の`x`と`y`の値を比較する`match`式を生成するには、
代わりにマッチガード条件式を使用する必要があるでしょう。マッチガードについては、後ほど、
[「マッチガードで追加の条件式」](#マッチガードで追加の条件式)節で語ります。

<!--
### Multiple Patterns
-->

### 複数のパターン

<!--
In `match` expressions, you can match multiple patterns using the `|` syntax,
which is the pattern *or* operator. For example, in the following code we match
the value of `x` against the match arms, the first of which has an *or* option,
meaning if the value of `x` matches either of the values in that arm, that
arm’s code will run:
-->

`match`式で`|`記法で複数のパターンに合致させることができ、これがパターン*or*演算子です。例えば、以下のコードでは`x`の値をマッチアームに合致させ、
最初のマッチアームには*or*選択肢があり、`x`の値がそのアームのどちらかの値に合致したら、そのアームのコードが走ることを意味します:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

<!--
This code prints `one or two`.
-->

このコードは、`one or two`を出力します。

<!--
### Matching Ranges of Values with `..=`
-->

### `..=`で値の範囲に合致させる

<!--
The `..=` syntax allows us to match to an inclusive range of values. In the
following code, when a pattern matches any of the values within the given
range, that arm will execute:
-->

`..=`記法により、限度値を含む値の範囲にマッチさせることができます。以下のコードでは、
パターンが与えられた範囲内のどれかの値に合致すると、そのアームが実行されます:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

<!--
If `x` is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more
convenient for multiple match values than using the `|` operator to express the
same idea; if we were to use `|` we would have to specify `1 | 2 | 3 | 4 | 5`.
Specifying a range is much shorter, especially if we want to match, say, any
number between 1 and 1,000!
-->

`x`が1、2、3、4か5なら、最初のアームが合致します。複数のマッチ値に対しては、この記法は`|`演算子を使用して同じ考えを表現するより便利です;
もし`|`を使うとしたら、`1 | 2 | 3 | 4 | 5`と指定しなければならないでしょう。
範囲を指定する方が遥かに短いのです。特に1から1000までの値と合致させたいとかなら！

<!--
The compiler checks that the range isn’t empty at compile time, and because the
only types for which Rust can tell if a range is empty or not are `char` and
numeric values, ranges are only allowed with numeric or `char` values.
-->

コンパイラはコンパイル時に範囲が空でないことを確認します。
範囲が空かそうでないかコンパイラにわかる型は`char`と数値のみなので、
範囲は、数値か`char`値でのみ許可されます。

<!--
Here is an example using ranges of `char` values:
-->

こちらは、`char`値の範囲を使用する例です:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

<!--
Rust can tell that `'c'` is within the first pattern’s range and prints `early
ASCII letter`.
-->

コンパイラには`'c'`が最初のパターンの範囲にあることがわかり、`early ASCII letter`と出力されます。

<!--
### Destructuring to Break Apart Values
-->

### 分配して値を分解する

<!--
We can also use patterns to destructure structs, enums, and tuples to use
different parts of these values. Let’s walk through each value.
-->

また、パターンを使用して構造体、enum、およびタプルを分配し、これらの値の異なる部分を使用することもできます。
各値を見ていきましょう。

<!--
#### Destructuring Structs
-->

#### 構造体を分配する

<!--
Listing 18-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement.
-->

リスト18-12は、`let`文でパターンを使用して分解できる2つのフィールド`x`と`y`のある`Point`構造体を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```

<!--
<span class="caption">Listing 18-12: Destructuring a struct’s fields into
separate variables</span>
-->

<span class="caption">リスト18-12: 構造体のフィールドを個別の変数に分配する</span>

<!--
This code creates the variables `a` and `b` that match the values of the `x`
and `y` fields of the `p` struct. This example shows that the names of the
variables in the pattern don’t have to match the field names of the struct.
However, it’s common to match the variable names to the field names to make it
easier to remember which variables came from which fields. Because of this
common usage, and because writing `let Point { x: x, y: y } = p;` contains a
lot of duplication, Rust has a shorthand for patterns that match struct fields:
you only need to list the name of the struct field, and the variables created
from the pattern will have the same names. Listing 18-13 behaves in the same
way as the code in Listing 18-12, but the variables created in the `let`
pattern are `x` and `y` instead of `a` and `b`.

-->

このコードは、`p`構造体の`x`と`y`フィールドの値に合致する変数`a`と`b`を生成します。この例は、
パターンの変数の名前は、構造体のフィールド名と合致する必要はないことを示しています。しかし、
この一般的な用法から、そして`let Point{ x: x, y: y } = p;`と書くことは多くの重複を含むことから、
Rustには構造体のフィールドと一致するパターンには省略法があります: 構造体のフィールドの名前を列挙するだけで、
パターンから生成される変数は同じ名前になるのです。リスト18-13は、リスト18-12と同じように振る舞いますが、
`let`パターンで生成される変数は`a`と`b`ではなく、`x`と`y`です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```

<!--
<span class="caption">Listing 18-13: Destructuring struct fields using struct
field shorthand</span>
-->

<span class="caption">リスト18-13: 構造体フィールド省略法で構造体のフィールドを分配する</span>

<!--
This code creates the variables `x` and `y` that match the `x` and `y` fields
of the `p` variable. The outcome is that the variables `x` and `y` contain the
values from the `p` struct.
-->

このコードは、`p`変数の`x`と`y`フィールドに一致する変数`x`と`y`を生成します。
結果は、変数`x`と`y`が`p`構造体の値を含むというものです。

<!--
We can also destructure with literal values as part of the struct pattern
rather than creating variables for all the fields. Doing so allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.
-->

また、全フィールドに対して変数を生成するのではなく、リテラル値を構造体パターンの一部にして分配することもできます。
そうすることで他のフィールドは分配して変数を生成しつつ、一部のフィールドは特定の値と一致するか確認できます。

<!--
In Listing 18-14, we have a `match` expression that separates `Point` values
into three cases: points that lie directly on the `x` axis (which is true when
`y = 0`), on the `y` axis (`x = 0`), or neither.
-->

リスト18-14には、`Point`値を3つの場合に区別する`match`式があります: `x`軸上の点(`y = 0`ならそうなる)、
`y`軸上の点(`x = 0`)、あるいはどちらでもありません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-14: Destructuring and matching literal values
in one pattern</span>
-->

<span class="caption">リスト18-14: 分配とリテラル値との一致を1つのパターンで</span>

<!--
The first arm will match any point that lies on the `x` axis by specifying that
the `y` field matches if its value matches the literal `0`. The pattern still
creates an `x` variable that we can use in the code for this arm.
-->

最初のアームは、`y`フィールドの値がリテラル`0`と一致するならマッチすると指定することで、`x`軸上にあるどんな点とも一致します。
このパターンはそれでも、このアームのコードで使用できる`x`変数を生成します。

<!--
Similarly, the second arm matches any point on the `y` axis by specifying that
the `x` field matches if its value is `0` and creates a variable `y` for the
value of the `y` field. The third arm doesn’t specify any literals, so it
matches any other `Point` and creates variables for both the `x` and `y` fields.
-->

同様に、2番目のアームは、`x`フィールドが`0`ならマッチすると指定することで`y`軸上のどんな点とも一致し、
`y`フィールドの値には変数`y`を生成します。3番目のアームは何もリテラルを指定しないので、
それ以外のあらゆる`Point`に合致し、`x`と`y`フィールド両方に変数を生成します。

<!--
In this example, the value `p` matches the second arm by virtue of `x`
containing a 0, so this code will print `On the y axis at 7`.
-->

この例で、値`p`は0を含む`x`の力で2番目のアームに一致するので、このコードは`On the y axis at 7`と出力します。

<!--
Remember that a `match` expression stops checking arms once it has found the
first matching pattern, so even though `Point { x: 0, y: 0}` is on the `x` axis
and the `y` axis, this code would only print `On the x axis at 0`.
-->

`match`式は、マッチする最初のパターンを見つけたら、アームの確認をやめることを思い出してください。
そのため、`Point { x: 0, y: 0}`は`x`軸上および`y`軸上にありますが、このコードは`On the x axis at 0`を出力するでしょう。

<!--
#### Destructuring Enums
-->

#### enumを分配する

<!--
We've destructured enums in this book (for example, Listing 6-5 in Chapter 6),
but haven’t yet explicitly discussed that the pattern to destructure an enum
corresponds to the way the data stored within the enum is defined. As an
example, in Listing 18-15 we use the `Message` enum from Listing 6-2 and write
a `match` with patterns that will destructure each inner value.
-->

この本の中でenumを分配してきました(例えば、第6章のリスト6-5)が、
enumを分配するパターンはenum内に格納されているデータの定義され方に対応する、ということについてはまだ明示的に議論していませんでした。
例として、リスト18-15では、リスト6-2から`Message` enumを使用し、内部の値それぞれを分配するパターンを伴う`match`を書いています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```

<!--
<span class="caption">Listing 18-15: Destructuring enum variants that hold
different kinds of values</span>
-->

<span class="caption">リスト18-15: 異なる種類の値を保持するenumの列挙子を分配する</span>

<!--
This code will print `Change the color to red 0, green 160, and blue 255`. Try
changing the value of `msg` to see the code from the other arms run.
-->

このコードは、`Change the color to red 0, green 160, blue 255`と出力します。
試しに`msg`の値を変更して、他のアームのコードが走るところを確認してください。

<!--
For enum variants without any data, like `Message::Quit`, we can’t destructure
the value any further. We can only match on the literal `Message::Quit` value,
and no variables are in that pattern.
-->

`Message::Quit`のようなデータのないenum列挙子については、それ以上値を分配することができません。
リテラル`Message::Quit`値にマッチするだけで、変数はそのパターンに存在しません。

<!--
For struct-like enum variants, such as `Message::Move`, we can use a pattern
similar to the pattern we specify to match structs. After the variant name, we
place curly brackets and then list the fields with variables so we break apart
the pieces to use in the code for this arm. Here we use the shorthand form as
we did in Listing 18-13.
-->

`Message::Move`のような構造体に似たenumの列挙子については、構造体と一致させるために指定するパターンと似たパターンを使用できます。
列挙子の名前の後に波括弧を配置し、それから変数とともにフィールドを列挙するので、部品を分解してこのアームのコードで使用します。
ここでは、リスト18-13のように省略形態を使用しています。

<!--
For tuple-like enum variants, like `Message::Write` that holds a tuple with one
element and `Message::ChangeColor` that holds a tuple with three elements, the
pattern is similar to the pattern we specify to match tuples. The number of
variables in the pattern must match the number of elements in the variant we’re
matching.
-->

1要素タプルを保持する`Message::Write`や、3要素タプルを保持する`Message::ChangeColor`のようなタプルに似たenumの列挙子について、
パターンは、タプルと一致させるために指定するパターンと類似しています。パターンの変数の数は、
マッチ対象の列挙子の要素数と一致しなければなりません。

<!--
#### Destructuring Nested Structs and Enums
-->

#### ネストされた構造体と列挙体を分配する

<!--
So far, our examples have all been matching structs or enums one level deep,
but matching can work on nested items too! For example, we can refactor the
code in Listing 18-15 to support RGB and HSV colors in the `ChangeColor`
message, as shown in Listing 18-16.
-->
 
今のところ、ここでの例はすべて一階層の深さの構造体または列挙体にマッチしていますが、
マッチングはネストされた要素に対しても行うことができます！例えばリスト18-15のコードは、
リスト18-16で示すように、`ChangeColor`メッセージでRGBおよびHSV色をサポートするようにリファクタすることができます。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```

<!--
<span class="caption">Listing 18-16: Matching on nested enums</span>
-->

<span class="caption">リスト 18-16: ネストされたenumにマッチする</span>

<!--
The pattern of the first arm in the `match` expression matches a
`Message::ChangeColor` enum variant that contains a `Color::Rgb` variant; then
the pattern binds to the three inner `i32` values. The pattern of the second
arm also matches a `Message::ChangeColor` enum variant, but the inner enum
matches `Color::Hsv` instead. We can specify these complex conditions in one
`match` expression, even though two enums are involved.
-->

`match`式の最初のアームのパターンは、`Color::Rgb`列挙子を含む`Message::ChangeColor` enum 列挙子にマッチします;
その後パターンは内側の`i32`値に束縛します。2番目のアームのパターンも`Message::ChangeColor` enum 列挙子にマッチしますが、
内側のenumは、こちらは`Color::Hsv`にマッチします。2つのenumが関連し合っている場合でも、1つの`match`式内で、
これらの複合的な条件を指定することができます。

<!--
#### Destructuring Structs and Tuples
-->

#### 構造体とタプルを分配する

<!--
We can mix, match, and nest destructuring patterns in even more complex ways.
The following example shows a complicated destructure where we nest structs and
tuples inside a tuple and destructure all the primitive values out:
-->

分配パターンをさらに複雑な方法で混ぜてマッチさせ、ネストすることができます。以下の例は、
構造体とタプルをタプルにネストし、全ての基本的な値を取り出している複雑な分配を表示しています:

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

<!--
This code lets us break complex types into their component parts so we can use
the values we’re interested in separately.
-->

このコードは、複雑な型を構成する部品に分配させてくれるので、興味のある値を個別に使用できます。

<!--
Destructuring with patterns is a convenient way to use pieces of values, such
as the value from each field in a struct, separately from each other.
-->

パターンで分配することは、構造体の各フィールドからの値のように、複数の値をお互いに区別して使用する便利な方法です。

<!--
### Ignoring Values in a Pattern
-->

### パターンの値を無視する

<!--
You’ve seen that it’s sometimes useful to ignore values in a pattern, such as
in the last arm of a `match`, to get a catchall that doesn’t actually do
anything but does account for all remaining possible values. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which you’ve seen), using the `_` pattern within another pattern,
using a name that starts with an underscore, or using `..` to ignore remaining
parts of a value. Let’s explore how and why to use each of these patterns.
-->

`match`の最後のアームのように、パターンの値を無視して実際には何もしないけれども、
残りの全ての値の可能性を考慮する包括的なものを得ることは、時として有用であると認識しましたね。
値全体やパターンの一部の値を無視する方法はいくつかあります: `_`パターンを使用すること(もう見かけました)、
他のパターン内で`_`パターンを使用すること、アンダースコアで始まる名前を使用すること、`..`を使用して値の残りの部分を無視することです。
これらのパターンそれぞれを使用する方法と理由を探究しましょう。

<!--
#### Ignoring an Entire Value with `_`
-->

#### `_`で値全体を無視する

<!--
We’ve used the underscore as a wildcard pattern that will match any value but
not bind to the value. This is especially useful as the last arm in a `match`
expression, but we can also use it in any pattern, including function
parameters, as shown in Listing 18-17.
-->

どんな値にも一致するけれども、値を束縛しないワイルドカードパターンとしてアンダースコアを使用してきました。
これは特に`match`式の最後のアームとして役に立ちますが、
関数の引数も含めてあらゆるパターンで使えます。リスト18-17に示したようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```

<!--
<span class="caption">Listing 18-17: Using `_` in a function signature</span>
-->

<span class="caption">リスト18-17: 関数シグニチャで`_`を使用する</span>

<!--
This code will completely ignore the value `3` passed as the first argument,
and will print `This code only uses the y parameter: 4`.
-->

このコードは、最初の引数として渡された値`3`を完全に無視し、`This code only uses the y parameter: 4`と出力します。

<!--
In most cases when you no longer need a particular function parameter, you
would change the signature so it doesn’t include the unused parameter. Ignoring
a function parameter can be especially useful in cases when, for example,
you're implementing a trait when you need a certain type signature but the
function body in your implementation doesn’t need one of the parameters. You
then avoid getting a compiler warning about unused function parameters, as you
would if you used a name instead.
-->

特定の関数の引数が最早必要ないほとんどの場合、未使用の引数が含まれないようにシグニチャを変更するでしょう。
関数の引数を無視することが特に有用なケースもあり、例えば、トレイトを実装する際、
特定の型シグニチャが必要だけれども、自分の実装の関数本体では引数の1つが必要ない時などです。
名前を使った場合には未使用関数引数についてのコンパイラ警告が出るでしょうが、こうすればそれをを避けられます。

<!--
#### Ignoring Parts of a Value with a Nested `_`
-->

#### ネストされた`_`で値の一部を無視する

<!--
We can also use `_` inside another pattern to ignore just part of a value, for
example, when we want to test for only part of a value but have no use for the
other parts in the corresponding code we want to run. Listing 18-18 shows code
responsible for managing a setting’s value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting but can unset the setting and give it a value if it is currently unset.
-->

また、他のパターンの内部で`_`を使用して、値の一部だけを無視することもでき、例えば、
値の一部だけを確認したいけれども、走らせたい対応するコードでは他の部分を使用することがない時などです。
リスト18-18は、設定の値を管理する責任を負ったコードを示しています。業務要件は、
ユーザが既存の設定の変更を上書きすることはできないべきだけれども、設定を解除することができるべきで、
現在設定がされていなければそれに値を与えることができるべきというものです。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-18: Using an underscore within patterns that
match `Some` variants when we don’t need to use the value inside the
`Some`</span>
-->

<span class="caption">リスト18-18: `Some`内の値を使用する必要がない時に`Some`列挙子と合致するパターンでアンダースコアを使用する</span>

<!--
This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`. In the first match arm, we don’t need to match on or use
the values inside either `Some` variant, but we do need to test for the case
when `setting_value` and `new_setting_value` are the `Some` variant. In that
case, we print the reason for not changing `setting_value`, and it doesn’t get
changed.
-->

このコードは、`Can't overwrite an existing customized value`、そして`setting is Some(5)`と出力するでしょう。
最初のマッチアームで、どちらの`Some`列挙子内部の値にも合致させたり、使用する必要はありませんが、
`setting_value`と`new_setting_value`が`Some`列挙子の場合を確かに確認する必要があります。
その場合、`setting_value`を変更しない理由を出力し、変更しません。

<!--
In all other cases (if either `setting_value` or `new_setting_value` are
`None`) expressed by the `_` pattern in the second arm, we want to allow
`new_setting_value` to become `setting_value`.
-->

2番目のアームの`_`パターンで表現される他のあらゆる場合(`setting_value`と`new_setting_value`どちらかが`None`なら)には、
`new_setting_value`に`setting_value`になってほしいです。

<!--
We can also use underscores in multiple places within one pattern to ignore
particular values. Listing 18-19 shows an example of ignoring the second and
fourth values in a tuple of five items.
-->

また、1つのパターンの複数箇所でアンダースコアを使用して特定の値を無視することもできます。
リスト18-19は、5要素のタプルで2番目と4番目の値を無視する例です。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-19: Ignoring multiple parts of a tuple</span>
-->

<span class="caption">リスト18-19: タプルの複数の部分を無視する</span>

<!--
This code will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.
-->

このコードは、`Some numbers: 2, 8, 32`と出力し、値4と16は無視されます。

<!--
#### Ignoring an Unused Variable by Starting Its Name with `_`
-->

#### 名前を`_`で始めて未使用の変数を無視する

<!--
If you create a variable but don’t use it anywhere, Rust will usually issue a
warning because an unused variable could be a bug. However, sometimes it’s
useful to be able to create a variable you won’t use yet, such as when you’re
prototyping or just starting a project. In this situation, you can tell Rust
not to warn you about the unused variable by starting the name of the variable
with an underscore. In Listing 18-20, we create two unused variables, but when
we compile this code, we should only get a warning about one of them.
-->

変数を作っているのにどこでも使用していなければ、未使用変数はバグかもしれないのでコンパイラは通常、警告を発します。
しかし時として、まだ使用しない変数を作れるほうが有用なこともあります。プロトタイプを開発していたり、
プロジェクトを始めた直後だったりなどです。このような場面では、変数名をアンダースコアで始めることで、
コンパイラに未使用変数について警告しないよう指示することができます。リスト18-20で2つの未使用変数を生成していますが、
このコードをコンパイルすると、そのうちの1つにしか警告が出ないはずです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```

<!--
<span class="caption">Listing 18-20: Starting a variable name with an
underscore to avoid getting unused variable warnings</span>
-->

<span class="caption">リスト18-20: アンダースコアで変数名を始めて未使用変数警告が出るのを回避する</span>

<!--
Here we get a warning about not using the variable `y`, but we don’t get a
warning about not using `_x`.
-->

ここで、変数`y`を使用していないことに対して警告が出ていますが、`_x`には、
使用していないという警告が出ていません。

<!--
Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. The syntax `_x` still binds the value to the
variable, whereas `_` doesn’t bind at all. To show a case where this
distinction matters, Listing 18-21 will provide us with an error.
-->

`_`だけを使うのとアンダースコアで始まる名前を使うことには微妙な違いがあることに注意してください。
`_x`記法はそれでも、値を変数に束縛する一方で、`_`は全く束縛しません。この差異が問題になる場合を示すために、
リスト18-21はエラーを提示するでしょう。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-21: An unused variable starting with an
underscore still binds the value, which might take ownership of the value</span>
-->

<span class="caption">リスト18-21: それでも、アンダースコアで始まる未使用の変数は値を束縛し、値の所有権を奪う可能性がある</span>

<!--
We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. However, using the underscore by itself
doesn’t ever bind to the value. Listing 18-22 will compile without any errors
because `s` doesn’t get moved into `_`.
-->

それでも`s`値は`_s`にムーブされ、再度`s`を使用できなくするので、エラーを受け取るでしょう。ですが、
アンダースコアを単独で使用すれば、値を束縛することは全くありません。
`s`が`_`にムーブされないので、リスト18-22はエラーなくコンパイルできます。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-22: Using an underscore does not bind the
value</span>
-->

<span class="caption">リスト18-22: アンダースコアを使用すると、値を束縛しない</span>

<!--
This code works just fine because we never bind `s` to anything; it isn’t moved.
-->

このコードは、`s`を何にも束縛しないので、ただ単に上手く動きます。つまり、ムーブされないのです。

<!--
#### Ignoring Remaining Parts of a Value with `..`
-->

#### `..`で値の残りの部分を無視する

<!--
With values that have many parts, we can use the `..` syntax to use specific
parts and ignore the rest, avoiding the need to list underscores for each
ignored value. The `..` pattern ignores any parts of a value that we haven’t
explicitly matched in the rest of the pattern. In Listing 18-23, we have a
`Point` struct that holds a coordinate in three-dimensional space. In the
`match` expression, we want to operate only on the `x` coordinate and ignore
the values in the `y` and `z` fields.
-->

多くの部分がある値では、`..`記法を使用して特定の部分を使用して残りを無視し、
無視する値それぞれにアンダースコアを列挙する必要性を回避できます。`..`パターンは、
パターンの残りで明示的にマッチさせていない値のどんな部分も無視します。リスト18-23では、
3次元空間で座標を保持する`Point`構造体があります。`match`式で`x`座標のみ処理し、
`y`と`z`フィールドの値は無視したいです。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-23: Ignoring all fields of a `Point` except
for `x` by using `..`</span>
-->

<span class="caption">リスト18-23: `..`で`x`以外の`Point`のフィールド全てを無視する</span>

<!--
We list the `x` value and then just include the `..` pattern. This is quicker
than having to list `y: _` and `z: _`, particularly when we’re working with
structs that have lots of fields in situations where only one or two fields are
relevant.
-->

`x`値を列挙し、それから`..`パターンを含んでいるだけです。これは、`y: _`や`z: _`と列挙しなければいけないのに比べて、
手っ取り早いです。特に1つや2つのフィールドのみが関連する場面で多くのフィールドがある構造体に取り掛かっている時には。

<!--
The syntax `..` will expand to as many values as it needs to be. Listing 18-24
shows how to use `..` with a tuple.
-->

`..`記法は、必要な数だけ値に展開されます。リスト18-24は、タプルで`..`を使用する方法を表示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```

<!--
<span class="caption">Listing 18-24: Matching only the first and last values in
a tuple and ignoring all other values</span>
-->

<span class="caption">リスト18-24: タプルの最初と最後の値にだけ合致し、他の値を無視する</span>

<!--
In this code, the first and last value are matched with `first` and `last`. The
`..` will match and ignore everything in the middle.
-->

このコードにおいて、最初と最後の値は`first`と`last`に合致します。`..`は、
途中のもの全部に合致し、無視します。

<!--
However, using `..` must be unambiguous. If it is unclear which values are
intended for matching and which should be ignored, Rust will give us an error.
Listing 18-25 shows an example of using `..` ambiguously, so it will not
compile.
-->

しかしながら、`..`を使うのは明確でなければなりません。どの値がマッチしてどの値が無視されるべきかが不明瞭なら、
コンパイラはエラーを出します。リスト18-25は、`..`を曖昧に使用する例なので、コンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```

<!--
<span class="caption">Listing 18-25: An attempt to use `..` in an ambiguous
way</span>
-->

<span class="caption">リスト18-25: `..`を曖昧に使用しようとする試み</span>

<!--
When we compile this example, we get this error:
-->

この例をコンパイルすると、こんなエラーが出ます:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-25/output.txt}}
```

<!--
It’s impossible for Rust to determine how many values in the tuple to ignore
before matching a value with `second` and then how many further values to
ignore thereafter. This code could mean that we want to ignore `2`, bind
`second` to `4`, and then ignore `8`, `16`, and `32`; or that we want to ignore
`2` and `4`, bind `second` to `8`, and then ignore `16` and `32`; and so forth.
The variable name `second` doesn’t mean anything special to Rust, so we get a
compiler error because using `..` in two places like this is ambiguous.
-->

コンパイラが、`second`の値に合致する前にタプルの幾つの値を無視し、それからそれによってさらに幾つの値を無視するかを決めることは不可能です。
このコードは、`2`を無視し、`second`に`4`を束縛し、それから`8`、`16`、`32`を無視したり、
`2`と`4`を無視して`second`に`8`を束縛し、それから`16`と`32`を無視するなどを意味することもあるでしょう。
変数名の`second`は、コンパイラにとってなんの特別な意味もなく、このように2箇所で`..`を使うのは曖昧なので、
コンパイルエラーになります。

<!--
### Extra Conditionals with Match Guards
-->

### マッチガードで追加の条件式

<!--
A *match guard* is an additional `if` condition, specified after the pattern in
a `match` arm, that must also match for that arm to be chosen. Match guards are
useful for expressing more complex ideas than a pattern alone allows.
-->

*マッチガード*は、`match`アームのパターンの後に指定される、
そのアームが選択されるのにマッチしなければならない追加の`if`条件です。マッチガードは、
1つのパターン単独でできるよりも複雑な考えを表現するのに役に立ちます。

<!--
The condition can use variables created in the pattern. Listing 18-26 shows a
`match` where the first arm has the pattern `Some(x)` and also has a match
guard of `if x % 2 == 0` (which will be true if the number is even).
-->

この条件は、パターンで生成された変数を使用できます。リスト18-26は、
最初のアームにパターン`Some(x)`と`if x % 2 == 0` (数が偶数の場合に真)というマッチガードもある`match`を示しています。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-26: Adding a match guard to a pattern</span>
-->

<span class="caption">リスト18-26: パターンにマッチガードを追記する</span>

<!--
This example will print `The number 4 is even`. When `num` is compared to the
pattern in the first arm, it matches, because `Some(4)` matches `Some(x)`. Then
the match guard checks whether the remainder of dividing `x` by 2 is equal to
0, and because it is, the first arm is selected.
-->

この例は、`The number 4 is even`と出力します。`num`が最初のアームのパターンと比較されると、
`Some(4)`は`Some(x)`に一致するので、マッチします。そして、マッチガードが`x`を2で割った余りが0と等しいか確認し、
そうなっているので、最初のアームが選択されます。

<!--
If `num` had been `Some(5)` instead, the match guard in the first arm would
have been false because the remainder of 5 divided by 2 is 1, which is not
equal to 0. Rust would then go to the second arm, which would match because the
second arm doesn’t have a match guard and therefore matches any `Some` variant.
-->

代わりに`num`が`Some(5)`だったなら、最初のアームのマッチガードは偽になったでしょう。
5を2で割った余りは1であり、0と等しくないからです。Rustはそうしたら2番目のアームに移動し、マッチするでしょう。
2番目のアームにはマッチガードがなく、それ故にあらゆる`Some`列挙子に一致するからです。

<!--
There is no way to express the `if x % 2 == 0` condition within a pattern, so
the match guard gives us the ability to express this logic. The downside of
this additional expressiveness is that the compiler doesn't try to check for
exhaustiveness when match guard expressions are involved.
-->

パターン内で`if x % 2 == 0`という条件を表現する方法はありませんので、マッチガードにより、
この論理を表現する能力が得られるのです。この追加の表現力の欠点としては、コンパイラは、
マッチガード式が関連するときに網羅性をチェックしないという点があります。

<!--
In Listing 18-11, we mentioned that we could use match guards to solve our
pattern-shadowing problem. Recall that we created a new variable inside the
pattern in the `match` expression instead of using the variable outside the
`match`. That new variable meant we couldn’t test against the value of the
outer variable. Listing 18-27 shows how we can use a match guard to fix this
problem.
-->

リスト18-11において、マッチガードを使用すれば、パターンがシャドーイングする問題を解決できると述べました。
`match`の外側の変数を使用するのではなく、`match`式のパターン内部では新しい変数を作成したことを思い出してください。
その新しい変数は、外側の変数の値と比較することができないことを意味しました。リスト18-27は、
マッチガードを使ってこの問題を修正する方法を表示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```

<!--
<span class="caption">Listing 18-27: Using a match guard to test for equality
with an outer variable</span>
-->

<span class="caption">リスト18-27: マッチガードを使用して外側の変数と等しいか確認する</span>

<!--
This code will now print `Default case, x = Some(5)`. The pattern in the second
match arm doesn’t introduce a new variable `y` that would shadow the outer `y`,
meaning we can use the outer `y` in the match guard. Instead of specifying the
pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`. This creates a new variable `n` that doesn’t shadow anything because
there is no `n` variable outside the `match`.
-->

このコードは今度は、`Default case, x = Some(5)`と出力するでしょう。2番目のマッチアームのパターンは、
外側の`y`を覆い隠してしまう新しい変数`y`を導入せず、マッチガード内で外側の`y`を使用できることを意味します。
外側の`y`を覆い隠してしまう`Some(y)`としてパターンを指定するのではなく、`Some(n)`を指定しています。
これにより、何も覆い隠さない新しい変数`n`が生成されます。`match`の外側には`n`変数は存在しないからです。

<!--
The match guard `if n == y` is not a pattern and therefore doesn’t introduce
new variables. This `y` *is* the outer `y` rather than a new shadowed `y`, and
we can look for a value that has the same value as the outer `y` by comparing
`n` to `y`.
-->

マッチガードの`if n == y`はパターンではなく、故に新しい変数を導入しません。この`y`は、
新しいシャドーイングされた`y`ではなく、外側の`y`*であり*、`n`と`y`を比較することで、
外側の`y`と同じ値を探すことができます。

<!--
You can also use the *or* operator `|` in a match guard to specify multiple
patterns; the match guard condition will apply to all the patterns. Listing
18-28 shows the precedence when combining a pattern that uses `|` with a match
guard. The important part of this example is that the `if y` match guard
applies to `4`, `5`, *and* `6`, even though it might look like `if y` only
applies to `6`.
-->

また、マッチガードで*or*演算子の`|`を使用して複数のパターンを指定することもできます;
マッチガードの条件は全てのパターンに適用されます。リスト18-28は、
`|`を使用するパターンをマッチガードと組み合わせるときの優先順位を示しています。この例で重要な部分は、
`if y`は`6`にしか適用されないように見えるのに、`if y`マッチガードが`4`、`5`、*そして*`6`に適用されることです。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-28: Combining multiple patterns with a match
guard</span>
-->

<span class="caption">リスト18-28: 複数のパターンとマッチガードを組み合わせる</span>

<!--
The match condition states that the arm only matches if the value of `x` is
equal to `4`, `5`, or `6` *and* if `y` is `true`. When this code runs, the
pattern of the first arm matches because `x` is `4`, but the match guard `if y`
is false, so the first arm is not chosen. The code moves on to the second arm,
which does match, and this program prints `no`. The reason is that the `if`
condition applies to the whole pattern `4 | 5 | 6`, not only to the last value
`6`. In other words, the precedence of a match guard in relation to a pattern
behaves like this:
-->

マッチの条件は、`x`の値が`4`、`5`、`6`に等しく*かつ*`y`が`true`の場合だけにアームがマッチすると宣言しています。
このコードが走ると、最初のアームのパターンは`x`が`4`なので、合致しますが、マッチガード`if y`は偽なので、
最初のアームは選ばれません。コードは2番目のアームに移動して、これがマッチし、このプログラムは`no`と出力します。
理由は、`if`条件が最後の値の`6`だけでなく、パターン全体`4 | 5 | 6`に適用されるからです。
言い換えると、パターンと関わるマッチガードの優先度は、以下のように振る舞います:

```text
(4 | 5 | 6) if y => ...
```

<!--
rather than this:
-->

以下のようにではありません:

```text
4 | 5 | (6 if y) => ...
```

<!--
After running the code, the precedence behavior is evident: if the match guard
were applied only to the final value in the list of values specified using the
`|` operator, the arm would have matched and the program would have printed
`yes`.
-->

コードを実行後には、優先度の動作は明らかになります: マッチガードが`|`演算子で指定される値のリストの最後の値にしか適用されないなら、
アームはマッチし、プログラムは`yes`と出力したでしょう。

<!--
### `@` Bindings
-->

### `@`束縛

<!--
The *at* operator `@` lets us create a variable that holds a value at the same
time as we’re testing that value for a pattern match. In Listing 18-29, we want
to test that a `Message::Hello` `id` field is within the range `3..=7`. We also
want to bind the value to the variable `id_variable` so we can use it in the
code associated with the arm. We could name this variable `id`, the same as the
field, but for this example we’ll use a different name.
-->

*at*演算子`@`により、値のパターンマッチを検査するのと同時に、その値を保持する変数を生成することができます。
リスト18-29では、`Message::Hello`の`id`フィールドが範囲`3..=7`にあるかを確かめたいです。
アームに紐づいたコードで使用できるように変数`id_variable`に値を束縛もしたいです。この変数をフィールドと同じ、
`id`と名付けることもできますが、この例では異なる名前にします。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-29: Using `@` to bind to a value in a pattern
while also testing it</span>
-->

<span class="caption">リスト18-29: `@`を使用してテストしつつ、パターンの値に束縛する</span>

<!--
This example will print `Found an id in range: 5`. By specifying `id_variable
@` before the range `3..=7`, we’re capturing whatever value matched the range
while also testing that the value matched the range pattern.
-->

この例は、`Found an id in range: 5`と出力します。範囲`3..=7`の前に`id_variable @`と指定することで、
値が範囲パターンに一致することを確認しつつ、範囲にマッチしたどんな値も捕捉しています。

<!--
In the second arm, where we only have a range specified in the pattern, the code
associated with the arm doesn’t have a variable that contains the actual value
of the `id` field. The `id` field’s value could have been 10, 11, or 12, but
the code that goes with that pattern doesn’t know which it is. The pattern code
isn’t able to use the value from the `id` field, because we haven’t saved the
`id` value in a variable.
-->

パターンで範囲しか指定していない2番目のアームでは、アームに紐づいたコードに`id`フィールドの実際の値を含む変数はありません。
`id`フィールドの値は10、11、12だった可能性があるでしょうが、そのパターンに来るコードは、
どれなのかわかりません。パターンのコードは`id`フィールドの値を使用することは叶いません。
`id`の値を変数に保存していないからです。

<!--
In the last arm, where we’ve specified a variable without a range, we do have
the value available to use in the arm’s code in a variable named `id`. The
reason is that we’ve used the struct field shorthand syntax. But we haven’t
applied any test to the value in the `id` field in this arm, as we did with the
first two arms: any value would match this pattern.
-->

範囲なしに変数を指定している最後のアームでは、確かにアームのコードで使用可能な値が`id`という変数にあります。
理由は、構造体フィールド省略記法を使ったからです。しかし、このアームで`id`フィールドの値に対して、
最初の2つのアームのようには、確認を行っていません: どんな値でも、このパターンに一致するでしょう。

<!--
Using `@` lets us test a value and save it in a variable within one pattern.
-->

`@`を使用することで、値を検査しつつ、1つのパターン内で変数に保存させてくれるのです。

<!--
## Summary
-->

## まとめ

<!--
Rust’s patterns are very useful in distinguishing between different kinds of
data. When used in `match` expressions, Rust ensures your patterns cover every
possible value, or your program won’t compile. Patterns in `let` statements and
function parameters make those constructs more useful, enabling the
destructuring of values into smaller parts at the same time as assigning to
variables. We can create simple or complex patterns to suit our needs.
-->

Rustのパターンは、異なる種類のデータを区別するのにとても有用です。`match`式で使用されると、
コンパイラはパターンが全ての可能性を網羅しているか保証し、そうでなければプログラムはコンパイルできません。
`let`文や関数の引数のパターンは、その構文をより有用にし、値を分配して小さな部品にすると同時に変数に代入できるようにしてくれます。
単純だったり複雑だったりするパターンを生成してニーズに合わせることができます。

<!--
Next, for the penultimate chapter of the book, we’ll look at some advanced
aspects of a variety of Rust’s features.
-->

次の本書の末尾から2番目の章では、Rustの多彩な機能の高度な視点に目を向けます。
