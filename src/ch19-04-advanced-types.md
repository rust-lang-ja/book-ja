<!--
## Advanced Types
-->

## 高度な型

<!--
The Rust type system has some features that we’ve so far mentioned but haven’t
yet discussed. We’ll start by discussing newtypes in general as we examine why
newtypes are useful as types. Then we’ll move on to type aliases, a feature
similar to newtypes but with slightly different semantics. We’ll also discuss
the `!` type and dynamically sized types.
-->

Rustの型システムには、ここまで触れたけれども、まだ議論していない機能があります。ニュータイプが何故型として有用なのかを調査するため、
一般化してニュータイプを議論することから始めます。そして、型エイリアスに移ります。ニュータイプに類似しているけれども、
多少異なる意味を持つ機能です。また、`!`型と動的サイズ決定型も議論します。

<!--
### Using the Newtype Pattern for Type Safety and Abstraction
-->

### 型安全性と抽象化を求めてニュータイプパターンを使用する

<!--
> Note: This section assumes you’ve read the earlier section [“Using the
> Newtype Pattern to Implement External Traits on External
> Types.”][using-the-newtype-pattern]
-->

> 注釈: この節は、以前の節[「ニュータイプパターンを使用して外部の型に外部のトレイトを実装する」][using-the-newtype-pattern]を読了済みであることを前提にしています。

<!--
The newtype pattern is also useful for tasks beyond those we’ve discussed so
far, including statically enforcing that values are never confused and
indicating the units of a value. You saw an example of using newtypes to
indicate units in Listing 19-15: recall that the `Millimeters` and `Meters`
structs wrapped `u32` values in a newtype. If we wrote a function with a
parameter of type `Millimeters`, we couldn’t compile a program that
accidentally tried to call that function with a value of type `Meters` or a
plain `u32`.
-->

ここまでに議論した以上の作業についてもニュータイプパターンは有用で、静的に絶対に値を混同しないことを強制したり、
値の単位を示すことを含みます。ニュータイプを使用して単位を示す例をリスト19-15で見かけました:
`Millimeters`と`Meters`構造体は、`u32`値をニュータイプにラップしていたことを思い出してください。
型`Millimeters`を引数にする関数を書いたら、誤ってその関数を型`Meters`や普通の`u32`で呼び出そうとするプログラムはコンパイルできないでしょう。

<!--
We can also use the newtype pattern to abstract away some implementation
details of a type: the new type can expose a public API that is different from
the API of the private inner type.
-->

ニュータイプパターンは、型の実装の詳細を抽象化するためにも使用することができます:
非公開の内部の型のAPIとは異なる公開APIを新しい型は露出できます。

<!--
Newtypes can also hide internal implementation. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person’s ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection; that code wouldn’t need to know that we assign an `i32` ID to names
internally. The newtype pattern is a lightweight way to achieve encapsulation
to hide implementation details, which we discussed in the [“Encapsulation that
Hides Implementation
Details”][encapsulation-that-hides-implementation-details]
section of Chapter 17.
-->

ニュータイプはまた、内部の実装を<ruby>隠匿<rp>(</rp><rt>いんとく</rt><rp>)</rp></ruby>することもできます。例を挙げれば、`People`型を提供して、
人のIDと名前を紐づけて格納する`HashMap<i32, String>`をラップすることができるでしょう。
`People`を使用するコードは、名前の文字列を`People`コレクションに追加するメソッドなど、
提供している公開APIとだけ相互作用するでしょう; そのコードは、内部で`i32`IDを名前に代入していることを知る必要はないでしょう。
ニュータイプパターンは、カプセル化を実現して実装の詳細を隠匿する軽い方法であり、
実装の詳細を隠匿することは、第17章の[「カプセル化は、実装詳細を隠蔽する」][encapsulation-that-hides-implementation-details]節で議論しましたね。

<!--
### Creating Type Synonyms with Type Aliases
-->

### 型エイリアスで型同義語を生成する

<!--
Rust provides the ability to declare a *type alias* to give an existing type
another name. For this we use the `type` keyword. For example, we can create
the alias `Kilometers` to `i32` like so:
-->

Rustでは、既存の型に別の名前を与える*型エイリアス*(type alias: 型別名)を宣言する能力が提供されています。
このために、`type`キーワードを使用します。例えば、以下のように`i32`に対して`Kilometers`というエイリアスを作れます。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

<!--
Now, the alias `Kilometers` is a *synonym* for `i32`; unlike the `Millimeters`
and `Meters` types we created in Listing 19-15, `Kilometers` is not a separate,
new type. Values that have the type `Kilometers` will be treated the same as
values of type `i32`:
-->

これで、別名の`Kilometers`は`i32`と*同義語*になりました; リスト19-15で生成した`Millimeters`と`Meters`とは異なり、
`Kilometers`は個別の新しい型ではありません。型`Kilometers`の値は、型`i32`の値と同等に扱われます。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

<!--
Because `Kilometers` and `i32` are the same type, we can add values of both
types and we can pass `Kilometers` values to functions that take `i32`
parameters. However, using this method, we don’t get the type checking benefits
that we get from the newtype pattern discussed earlier. In other words, if we
mix up `Kilometers` and `i32` values somewhere, the compiler will not give us
an error.
-->

`Kilometers`と`i32`が同じ型なので、両方の型の値を足し合わせたり、`Kilometers`の値を`i32`引数を取る関数に渡せたりします。
ですが、この方策を使用すると、先ほど議論したニュータイプパターンで得られる型チェックの利便性は得られません。
つまり、`Kilometers`と`i32`の値をどこかでごちゃ混ぜにしてしまっても、コンパイラはエラーを出してくれないでしょう。

<!--
The main use case for type synonyms is to reduce repetition. For example, we
might have a lengthy type like this:
-->

型同義語の主なユースケースは、繰り返しを減らすことです。例えば、こんな感じの長い型があるかもしれません:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

<!--
Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error prone. Imagine having a project full of
code like that in Listing 19-24.
-->

この長ったらしい型を関数シグニチャや型注釈としてコードのあちこちで記述するのは、面倒で間違いも起きやすいです。
リスト19-24のそのようなコードで溢れかえったプロジェクトがあることを想像してください。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-24: Using a long type in many places</span>
-->

<span class="caption">リスト19-24: 長い型を多くの場所で使用する</span>

<!--
A type alias makes this code more manageable by reducing the repetition. In
Listing 19-25, we’ve introduced an alias named `Thunk` for the verbose type and
can replace all uses of the type with the shorter alias `Thunk`.
-->

型エイリアスは、繰り返しを減らすことでこのコードをより管理しやすくしてくれます。リスト19-25で、
冗長な型に`Thunk`(`注釈`: 塊)を導入し、その型の使用全部をより短い別名の`Thunk`で置き換えることができます。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```

<!--
<span class="caption">Listing 19-25: Introducing a type alias `Thunk` to reduce
repetition</span>
-->

<span class="caption">リスト19-25: 型エイリアスの`Thunk`を導入して繰り返しを減らす</span>

<!--
This code is much easier to read and write! Choosing a meaningful name for a
type alias can help communicate your intent as well (*thunk* is a word for code
to be evaluated at a later time, so it’s an appropriate name for a closure that
gets stored).
-->

このコードの方が遥かに読み書きしやすいです！型エイリアスに意味のある名前を選択すると、
意図を伝えるのにも役に立つことがあります(*thunk*は後ほど評価されるコードのための単語なので、
格納されるクロージャーには適切な名前です)。

<!--
Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition. Consider the `std::io` module in the standard library. I/O
operations often return a `Result<T, E>` to handle situations when operations
fail to work. This library has a `std::io::Error` struct that represents all
possible I/O errors. Many of the functions in `std::io` will be returning
`Result<T, E>` where the `E` is `std::io::Error`, such as these functions in
the `Write` trait:
-->

型エイリアスは、繰り返しを減らすために`Result<T, E>`型ともよく使用されます。標準ライブラリの`std::io`モジュールを考えてください。
I/O処理はしばしば、`Result<T, E>`を返して処理がうまく動かなかった時を扱います。このライブラリには、
全ての可能性のあるI/Oエラーを表す`std::io::Error`構造体があります。`std::io`の関数の多くは、
`Write`トレイトの以下の関数のように`E`が`std::io::Error`の`Result<T, E>`を返すでしょう:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

<!--
The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type
alias declaration:
-->

`Result<..., Error>`が何度も繰り返されてます。そんな状態なので、`std::io`にはこんな型エイリアス宣言があります:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

<!--
Because this declaration is in the `std::io` module, we can use the fully
qualified alias `std::io::Result<T>`; that is, a `Result<T, E>` with the `E`
filled in as `std::io::Error`. The `Write` trait function signatures end up
looking like this:
-->

この宣言は`std::io`モジュール内にあるので、フルパスエイリアスの`std::io::Result<T>`を使用できます。
つまり、`E`が`std::io::Error`で埋められた`Result<T, E>`です。その結果、`Write`トレイトの関数シグニチャは、
以下のような見た目になります:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

<!--
The type alias helps in two ways: it makes code easier to write *and* it gives
us a consistent interface across all of `std::io`. Because it’s an alias, it’s
just another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, as well as special syntax like the `?` operator.
-->

型エイリアスは、2通りの方法で役に立っています: コードを書きやすくすること*と*`std::io`を通して首尾一貫したインターフェイスを与えてくれることです。
別名なので、ただの`Result<T, E>`であり、要するに`Result<T, E>`に対して動くメソッドはなんでも使えるし、
`?`演算子のような特殊な記法も使えます。

<!--
### The Never Type that Never Returns
-->

### never型は絶対に返らない

<!--
to stand inで「代役を務める」という意味だが、ここではあえて直訳にした
-->

<!--
Rust has a special type named `!` that’s known in type theory lingo as the
*empty type* because it has no values. We prefer to call it the *never type*
because it stands in the place of the return type when a function will never
return. Here is an example:
-->

Rustには、`!`という名前の特別な型があります。それは型理論の専門用語では *Empty型* と呼ばれ値なしを表します。私たちは、
関数が値を返すことが決して (never) ない時に戻り値の型を記す場所に使われるので、*never type*(`訳注`: 日本語にはできないので、never型と呼ぶしかないか)と呼ぶのが好きです。
こちらが例です:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

<!--
This code is read as “the function `bar` returns never.” Functions that return
never are called *diverging functions*. We can’t create values of the type `!`
so `bar` can never possibly return.
-->

このコードは、「関数`bar`はneverを返す」と解読します。neverを返す関数は、*発散する関数*(diverging function)と呼ばれます。
型`!`の値は生成できないので、`bar`からリターンする（呼び出し元に制御を戻す）ことは決してできません。

<!--
But what use is a type you can never create values for? Recall the code from
Listing 2-5, part of the number guessing game; we’ve reproduced a bit of it
here in Listing 19-26.
-->

ですが、値を絶対に生成できない型をどう使用するのでしょうか？数当てゲームの一部である、
リスト2-5のコードを思い出してください; リスト19-26にその一部を再掲します。


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<!--
<span class="caption">Listing 19-26: A `match` with an arm that ends in
`continue`</span>
-->

<span class="caption">リスト19-26: `continue`になるアームがある`match`</span>

<!--
At the time, we skipped over some details in this code. In Chapter 6 in [“The
`match` Control Flow Operator”][the-match-control-flow-operator]
section, we discussed that `match` arms must all return the same type. So, for
example, the following code doesn’t work:
-->

この時点では、このコードの詳細の一部を飛ばしました。第6章の[「`match`制御フロー演算子」][the-match-control-flow-operator]節で、
`match`アームは全て同じ型を返さなければならないと議論しました。従って、例えば以下のコードは動きません:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

<!--
The type of `guess` in this code would have to be an integer *and* a string,
and Rust requires that `guess` have only one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 19-26?
-->

このコードの`guess`は整数*かつ*文字列にならなければならないでしょうが、Rustでは、`guess`は1つの型にしかならないことを要求されます。
では、`continue`は何を返すのでしょうか？どうやってリスト19-26で1つのアームからは`u32`を返し、別のアームでは、
`continue`で終わっていたのでしょうか？

<!--
As you might have guessed, `continue` has a `!` value. That is, when Rust
computes the type of `guess`, it looks at both match arms, the former with a
value of `u32` and the latter with a `!` value. Because `!` can never have a
value, Rust decides that the type of `guess` is `u32`.
-->

もうお気付きかもしれませんが、`continue`は`!`値です。つまり、コンパイラが`guess`の型を計算する時、
両方のmatchアームを見て、前者は`u32`の値、後者は`!`値となります。`!`は絶対に値を持ち得ないので、
コンパイラは、`guess`の型は`u32`と決定するのです。

<!--
The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t return a value; instead, it moves control
back to the top of the loop, so in the `Err` case, we never assign a value to
`guess`.
-->

この振る舞いを解説する公式の方法は、型`!`の式は、他のどんな型にも型強制され得るということです。
この`match`アームを`continue`で終えることができます。何故なら、`continue`は値を返さないからです;
その代わりに制御をループの冒頭に戻すので、`Err`の場合、`guess`には絶対に値を代入しないのです。

<!--
The never type is useful with the `panic!` macro as well. Recall the `unwrap`
function that we call on `Option<T>` values to produce a value or panic with
this definition:
-->

never型は、`panic!`マクロとも有用です。`Option<T>`値に対して呼び出して、
値かパニックを生成する`unwrap`関数を思い出してください。こちらがその定義です:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

<!--
In this code, the same thing happens as in the `match` in Listing 19-26: Rust
sees that `val` has the type `T` and `panic!` has the type `!`, so the result
of the overall `match` expression is `T`. This code works because `panic!`
doesn’t produce a value; it ends the program. In the `None` case, we won’t be
returning a value from `unwrap`, so this code is valid.
-->

このコードにおいて、リスト19-26の`match`と同じことが起きています: コンパイラは、`val`の型は`T`で、
`panic!`の型は`!`なので、`match`式全体の結果は`T`と確認します。`panic!`は値を生成しないので、
このコードは動きます。つまり、プログラムを終了するのです。`None`の場合、`unwrap`から値は返さないので、
このコードは合法なのです。

<!--
One final expression that has the type `!` is a `loop`:
-->

型が`!`の最後の式は、`loop`です:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

<!--
Here, the loop never ends, so `!` is the value of the expression. However, this
wouldn’t be true if we included a `break`, because the loop would terminate
when it got to the `break`.
-->

ここで、ループは終わりませんので、`!`が式の値です。ところが、`break`を含んでいたら、これは真実にはならないでしょう。
`break`に到達した際にループが終了してしまうからです。

<!--
### Dynamically Sized Types and the `Sized` Trait
-->

### 動的サイズ決定型と`Sized`トレイト

<!--
Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type. This leaves one corner of its type
system a little confusing at first: the concept of *dynamically sized types*.
Sometimes referred to as *DSTs* or *unsized types*, these types let us write
code using values whose size we can know only at runtime.
-->

コンパイラは、特定の型の値1つにどれくらいのスペースのメモリを確保するのかなど、
その型についての特定の詳細を知る必要があります。ただしこれには、
最初は少し混乱しやすい型システムのコーナーケースがあります:
*動的サイズ決定型*の概念です。時として*DST*や*サイズなし型*とも称され、
これらの型により、実行時にしかサイズを知ることのできない値を使用するコードを書かせてくれます。

<!--
Let’s dig into the details of a dynamically sized type called `str`, which
we’ve been using throughout the book. That’s right, not `&str`, but `str` on
its own, is a DST. We can’t know how long the string is until runtime, meaning
we can’t create a variable of type `str`, nor can we take an argument of type
`str`. Consider the following code, which does not work:
-->

`str`と呼ばれる動的サイズ決定型の詳細を深掘りしましょう。本を通して使用してきましたね。
そうです。`&str`ではなく、`str`は単独でDSTなのです。実行時までは文字列の長さを知ることができず、
これは、型`str`の変数を生成したり、型`str`を引数に取ることはできないことを意味します。
動かない以下のコードを考えてください:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

<!--
Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If Rust
allowed us to write this code, these two `str` values would need to take up the
same amount of space. But they have different lengths: `s1` needs 12 bytes of
storage and `s2` needs 15. This is why it’s not possible to create a variable
holding a dynamically sized type.
-->

コンパイラは、特定の型のどんな値に対しても確保するメモリ量を知る必要があり、ある型の値は全て同じ量のメモリを使用しなければなりません。
Rustでこのコードを書くことが許容されたら、これら2つの`str`値は、同じ量のスペースを消費する必要があったでしょう。
ですが、長さが異なります: `s1`は、12バイトのストレージが必要で、`s2`は15バイトです。このため、
動的サイズ決定型を保持する変数を生成することはできないのです。

<!--
So what do we do? In this case, you already know the answer: we make the types
of `s1` and `s2` a `&str` rather than a `str`. Recall from the [“String
Slices”][string-slices] section of Chapter 4 that the slice data
structure just stores the starting position and the length of the slice. So
although a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is *two* values: the address of the `str` and its
length. As such, we can know the size of a `&str` value at compile time: it’s
twice the length of a `usize`. That is, we always know the size of a `&str`, no
matter how long the string it refers to is. In general, this is the way in
which dynamically sized types are used in Rust: they have an extra bit of
metadata that stores the size of the dynamic information. The golden rule of
dynamically sized types is that we must always put values of dynamically sized
types behind a pointer of some kind.
-->

では、どうすればいいのでしょうか？この場合、もう答えはご存知です: `s1`と`s2`の型を`str`ではなく、
`&str`にすればいいのです。第4章の[「文字列スライス」][string-slices]節で学んだように、
スライスデータ構造は単に開始地点とスライスの長さを格納したものだということを思い出してください。
従って、`&T`は、`T`がどこにあるかのメモリアドレスを格納する単独の値だけれども、`&str`は*2つ*の値なのです:
`str`のアドレスとその長さです。そのため、コンパイル時に`&str`のサイズを知ることができます:
`usize`の長さの2倍です。要するに、参照している文字列の長さによらず、常に`&str`のサイズがわかります。
通常、このようにしてRustでは動的サイズ決定型が使用されます: 動的情報のサイズを格納する追加のちょっとしたメタデータがあるのです。
動的サイズ決定型の黄金規則は、常に動的サイズ決定型の値をなんらかの種類のポインタの背後に配置しなければならないということです。

<!--
We can combine `str` with all kinds of pointers: for example, `Box<str>` or
`Rc<str>`. In fact, you’ve seen this before but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In Chapter 17 in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]
section, we mentioned that to use traits as trait objects, we must
put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn
Trait>` would work too).
-->

`str`を全ての種類のポインタと組み合わせられます: 例を挙げれば、`Box<str>`や`Rc<str>`などです。
実際、これまでに見かけましたが、異なる動的サイズ決定型でした: トレイトです。全てのトレイトは、
トレイト名を使用して参照できる動的サイズ決定型です。第17章の[「トレイトオブジェクトで異なる型の値を許容する」][using-trait-objects-that-allow-for-values-of-different-types]節で、
トレイトをトレイトオブジェクトとして使用するには、`&dyn Trait`や`Box<dyn Trait>`(`Rc<dyn Trait>`も動くでしょう)など、
ポインタの背後に配置しなければならないことに触れました。

<!--
To work with DSTs, Rust provides the `Sized` trait to determine whether or not
a type’s size is known at compile time. This trait is automatically implemented
for everything whose size is known at compile time. In addition, Rust
implicitly adds a bound on `Sized` to every generic function. That is, a
generic function definition like this:
-->

DSTを扱うために、Rustは`Sized`トレイトを提供しており、型のサイズがコンパイル時にわかるかどうかを決定します。
このトレイトは、コンパイル時にサイズの判明する全てのものに自動的に実装されます。加えて、
コンパイラは暗黙的に全てのジェネリックな関数に`Sized`の境界を追加します。つまり、こんな感じのジェネリック関数定義は:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

<!--
is actually treated as though we had written this:
-->

実際にはこう書いたかのように扱われます:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

<!--
By default, generic functions will work only on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:
-->

既定では、ジェネリック関数はコンパイル時に判明するサイズがある型に対してのみ動きます。
ですが、以下の特別な記法を用いてこの制限を緩めることができます:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

<!--
A trait bound on `?Sized` means “`T` may or may not be `Sized`” and this
notation overrides the default that generic types must have a known size at
compile time. The `?Trait` syntax with this meaning is only available for
`Sized`, not any other traits.
-->

`?Sized`のトレイト境界は「`T`は`Sized`かもしれないし、違うかもしれない」という意味であり、
この記法は、ジェネリック型はコンパイル時に既知のサイズを持たなくてはならないという、
デフォルトの制約を無効化します。この意味を持つ`?Trait`記法は、`Sized`にのみ利用可能で、
他のトレイトに対しては利用できません。

<!--
Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be `Sized`, we need to use it behind some kind of
pointer. In this case, we’ve chosen a reference.
-->

また、`t`引数の型を`T`から`&T`に切り替えたことにも注目してください。型は`Sized`でない可能性があるので、
なんらかのポインタの背後に使用する必要があるのです。今回は、参照を選択しました。

<!--
Next, we’ll talk about functions and closures!
-->

次は、関数とクロージャについて語ります！

<!--
[encapsulation-that-hides-implementation-details]:
ch17-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]:
ch06-02-match.html#the-match-control-flow-operator
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[using-the-newtype-pattern]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
-->

[encapsulation-that-hides-implementation-details]:
ch17-01-what-is-oo.html#カプセル化は実装詳細を隠蔽する
[string-slices]: ch04-03-slices.html#文字列スライス
[the-match-control-flow-operator]:
ch06-02-match.html#match制御フロー構造
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#トレイトオブジェクトで異なる型の値を許容する
[using-the-newtype-pattern]: ch19-03-advanced-traits.html#ニュータイプパターンを使用して外部の型に外部のトレイトを実装する
