<!--
## Defining an Enum
-->

## Enumを定義する

<!--
Where structs give you a way of grouping together related fields and data, like
a `Rectangle` with its `width` and `height`, enums give you a way of saying a
value is one of a possible set of values. For example, we may want to say that
`Rectangle` is one of a set of possible shapes that also includes `Circle` and
`Triangle`. To do this, Rust allows us to encode these possibilities as an enum.
-->

構造体は、`width`と`height`を持つ`Rectangle`のように、関連するフィールドとデータをひとつにまとめる方法を提供してくれます。
一方でenumは、ある値が、とりうる値の集合のうちのいずれかひとつであることを表現する方法を提供するものです。
例えば、`Rectangle`は、`Circle`や`Triangle`も含めたとりうる形の集合のいずれかひとつである、と表現したいことがあります。
これを達成するために、Rustではこれらの可能性をenumとしてエンコードすることができます。

<!--
Let’s look at a situation we might want to express in code and see why enums
are useful and more appropriate than structs in this case. Say we need to work
with IP addresses. Currently, two major standards are used for IP addresses:
version four and version six. Because these are the only possibilities for an
IP address that our program will come across, we can *enumerate* all possible
variants, which is where enumeration gets its name.
-->

コードで表現したくなるかもしれない場面に目を向けて、enumが有用でこの場合、構造体よりも適切である理由を確認しましょう。
IPアドレスを扱う必要が出たとしましょう。現在、IPアドレスの規格は二つあります: バージョン4とバージョン6です。
これらはプログラムが遭遇するIPアドレスの可能性のすべてですので、取りうる列挙子をすべて*列挙*できます。
これが列挙型の名前の由来です。

<!--
Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate because an enum value can only be one of its variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.
-->

どんなIPアドレスも、バージョン4かバージョン6のどちらかになりますが、同時に両方にはなり得ません。
IPアドレスのその特性により、enumデータ構造が適切なものになります。というのも、
enumの値は、その列挙子のいずれか一つにしかなり得ないからです。バージョン4とバージョン6のアドレスは、
どちらも根源的にはIPアドレスですから、コードがいかなる種類のIPアドレスにも適用される場面を扱う際には、
同じ型として扱われるべきです。

<!--
We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`. These are the
variants of the enum:
-->

この概念をコードでは、`IpAddrKind`列挙型を定義し、IPアドレスがなりうる種類、`V4`と`V6`を列挙することで、
表現できます。これらがenumの列挙子です:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

<!--
`IpAddrKind` is now a custom data type that we can use elsewhere in our code.
-->

これで、`IpAddrKind`はコードの他の場所で使用できる独自のデータ型になります。

<!--
### Enum Values
-->

### Enumの値

<!--
We can create instances of each of the two variants of `IpAddrKind` like this:
-->

以下のようにして、`IpAddrKind`の各列挙子のインスタンスは生成できます:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

<!--
Note that the variants of the enum are namespaced under its identifier, and we
use a double colon to separate the two. This is useful because now both values
`IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`. We
can then, for instance, define a function that takes any `IpAddrKind`:
-->

enumの列挙子は、その識別子の元に名前空間分けされていることと、
2連コロンを使ってその二つを区別していることに注意してください。
これが有効な理由は、こうすることで、値`IpAddrKind::V4`と`IpAddrKind::V6`という値は両方とも、
同じ型`IpAddrKind`になったからです。そうしたら、例えば、どんな`IpAddrKind`を取る関数も定義できるようになります。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

<!--
And we can call this function with either variant:
-->

そして、この関数をどちらの列挙子に対しても呼び出せます:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

<!--
Using enums has even more advantages. Thinking more about our IP address type,
at the moment we don’t have a way to store the actual IP address *data*; we
only know what *kind* it is. Given that you just learned about structs in
Chapter 5, you might be tempted to tackle this problem with structs as shown in
Listing 6-1.
-->

enumの利用には、さらなる利点さえもあります。このIPアドレス型についてもっと考えてみると、現状では、
実際のIPアドレスの*データ*を保持する方法がありません。つまり、どんな*種類*であるかを知っているだけです。
構造体について第5章で学んだばっかりとすると、この問題に対して、あなたはリスト6-1のように構造体を使って対処したくなるかもしれません。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-1: Storing the data and `IpAddrKind` variant of
an IP address using a `struct`</span>
-->

<span class="caption">リスト6-1: IPアドレスのデータと`IpAddrKind`の列挙子を`struct`を使って保持する</span>

<!--
Here, we’ve defined a struct `IpAddr` that has two fields: a `kind` field that
is of type `IpAddrKind` (the enum we defined previously) and an `address` field
of type `String`. We have two instances of this struct. The first is `home`,
and it has the value `IpAddrKind::V4` as its `kind` with associated address
data of `127.0.0.1`. The second instance is `loopback`. It has the other
variant of `IpAddrKind` as its `kind` value, `V6`, and has address `::1`
associated with it. We’ve used a struct to bundle the `kind` and `address`
values together, so now the variant is associated with the value.
-->

ここでは、二つのフィールドを持つ`IpAddr`という構造体を定義しています: `IpAddrKind`型(先ほど定義したenumですね)の`kind`フィールドと、
`String`型の`address`フィールドです。この構造体のインスタンスが2つあります。最初のインスタンスは`home`で、
これには`kind`として`IpAddrKind::V4`があり、紐付けられたアドレスデータは`127.0.0.1`です。
2番目のインスタンスは`loopback`です。これには`kind`の値として、`IpAddrKind`のもう一つの列挙子、`V6`があり、
アドレス`::1`が紐付いています。構造体を使って`kind`と`address`値を一緒に包んだので、
もう列挙子は値と紐付けられています。

<!--
However, representing the same concept using just an enum is more concise:
rather than an enum inside a struct, we can put data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:
-->

しかしながら、enumだけを使って同じ概念を表現するほうがより簡潔です:
構造体の中にenumを持たせるのではなく、enumの各列挙子に直接データを格納することができるのです。
この新しい`IpAddr`の定義は、`V4`と`V6`列挙子両方に`String`値が紐付けられていることを述べています。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

<!--
We attach data to each variant of the enum directly, so there is no need for an
extra struct. Here, it’s also easier to see another detail of how enums work:
the name of each enum variant that we define also becomes a function that
constructs an instance of the enum. That is, `IpAddr::V4()` is a function call
that takes a `String` argument and returns an instance of the `IpAddr` type. We
automatically get this constructor function defined as a result of defining the
enum.
-->

enumの各列挙子にデータを直接添付できるので、余計な構造体を作る必要は全くありません。
またここでは、enumがどう機能するかについての別の詳細も確認することができます:
ここで定義したenumの各列挙子の名前は、そのenumを構築する関数にもなるのです。
つまり、`IpAddr::V4()`は`String`引数を受け取り`IpAddr`型のインスタンスを返す関数の呼び出しになるのです。
enumを定義した結果として、自動でこのコンストラクタ関数が定義されます。

<!--
There’s another advantage to using an enum rather than a struct: each variant
can have different types and amounts of associated data. Version four IP
addresses will always have four numeric components that will have values
between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but
still express `V6` addresses as one `String` value, we wouldn’t be able to with
a struct. Enums handle this case with ease:
-->

構造体よりもenumを使うことには、別の利点もあります: 各列挙子に紐付けるデータの型と量は、異なってもいいのです。
バージョン4のIPアドレスには、常に0から255の値を持つ4つの数値があります。`V4`のアドレスは、4つの`u8`型の値として格納するけれども、
`V6`のアドレスは引き続き、単独の`String`型の値で格納したかったとしても、構造体では不可能です。
enumなら、こんな場合も容易に対応できます:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

<!--
We’ve shown several different ways to define data structures to store version
four and version six IP addresses. However, as it turns out, wanting to store
IP addresses and encode which kind they are is so common that [the standard
library has a definition we can use!][IpAddr] Let’s look at how
the standard library defines `IpAddr`: it has the exact enum and variants that
we’ve defined and used, but it embeds the address data inside the variants in
the form of two different structs, which are defined differently for each
variant:
-->

バージョン4とバージョン6のIPアドレスを格納するデータ構造を定義する複数の異なる方法を示してきました。
しかしながら、蓋を開けてみれば、IPアドレスを格納してその種類をコード化したくなるということは一般的なので、
[標準ライブラリに使用可能な定義があります！][IpAddr] 標準ライブラリでの`IpAddr`の定義のされ方を見てみましょう:
私たちが定義し、使用したのと全く同じenumと列挙子がありますが、アドレスデータを二種の異なる構造体の形で列挙子に埋め込み、
この構造体は各列挙子用に異なる形で定義されています。

<!--
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
-->

```rust
struct Ipv4Addr {
    // --略--
}

struct Ipv6Addr {
    // --略--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

<!--
This code illustrates that you can put any kind of data inside an enum variant:
strings, numeric types, or structs, for example. You can even include another
enum! Also, standard library types are often not much more complicated than
what you might come up with.
-->

このコードは、enum列挙子内にいかなる種類のデータでも格納できることを描き出しています:
例を挙げれば、文字列、数値型、構造体などです。他のenumを含むことさえできます！また、
標準ライブラリの型は、あなたの想像するよりも複雑ではないことがしばしばあります。

<!--
Note that even though the standard library contains a definition for `IpAddr`,
we can still create and use our own definition without conflict because we
haven’t brought the standard library’s definition into our scope. We’ll talk
more about bringing types into scope in Chapter 7.
-->

標準ライブラリに`IpAddr`に対する定義は含まれるものの、標準ライブラリの定義をまだ我々のスコープに導入していないので、
干渉することなく自分自身の定義を生成して使用できることに注意してください。型をスコープに導入することについては、
第7章でもっと詳しく言及します。

<!--
Let’s look at another example of an enum in Listing 6-2: this one has a wide
variety of types embedded in its variants.
-->

リスト6-2でenumの別の例を見てみましょう: 今回のコードは、幅広い種類の型が列挙子に埋め込まれています。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-2: A `Message` enum whose variants each store
different amounts and types of values</span>
-->

<span class="caption">リスト6-2: 列挙子各々が異なる型と量の値を格納する`Message` enum</span>

<!--
This enum has four variants with different types:
-->

このenumには、異なる型の列挙子が4つあります:

<!--
* `Quit` has no data associated with it at all.
* `Move` has named fields, like a struct does.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32` values.
-->

* `Quit`は関連付けられたデータをまったく持ちません。
* `Move`は構造体のように名前付きのフィールドを持っています。
* `Write`は`String`オブジェクトを1個だけ含んでいます。
* `ChangeColor`は3個の`i32`値を含んでいます。

<!--
Defining an enum with variants such as the ones in Listing 6-2 is similar to
defining different kinds of struct definitions, except the enum doesn’t use the
`struct` keyword and all the variants are grouped together under the `Message`
type. The following structs could hold the same data that the preceding enum
variants hold:
-->

リスト6-2のような列挙子を含むenumを定義することは、enumの場合、`struct`キーワードを使わず、
全部の列挙子が`Message`型の元に分類される点を除いて、異なる種類の構造体定義を定義するのと類似しています。
以下の構造体も、先ほどのenumの列挙子が保持しているのと同じデータを格納することができるでしょう:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

<!--
But if we used the different structs, each of which has its own type, we
couldn’t as easily define a function to take any of these kinds of messages as
we could with the `Message` enum defined in Listing 6-2, which is a single type.
-->

ですがそれぞれ独自の型を持つ異なる構造体を使ってしまうと、リスト6-2で定義した単一の型である`Message` enumを使う場合と比較して、
これらの任意の種類のメッセージを取る関数を簡単には定義できません。

<!--
There is one more similarity between enums and structs: just as we’re able to
define methods on structs using `impl`, we’re also able to define methods on
enums. Here’s a method named `call` that we could define on our `Message` enum:
-->

enumと構造体にはもう1点似通っているところがあります: `impl`を使って構造体にメソッドを定義できるのと全く同様に、
enumにもメソッドを定義することができるのです。こちらは、`Message` enum上に定義できる`call`という名前のメソッドです:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

<!--
The body of the method would use `self` to get the value that we called the
method on. In this example, we’ve created a variable `m` that has the value
`Message::Write(String::from("hello"))`, and that is what `self` will be in the
body of the `call` method when `m.call()` runs.
-->

メソッドの本体では、`self`を使用して、メソッドを呼び出した相手の値を取得できるでしょう。この例では、
`Message::Write(String::from("hello"))`という値を持つ、変数`m`を生成したので、これが`m.call()`を走らせた時に、
`call`メソッドの本体内で`self`が表す値になります。

<!--
Let’s look at another enum in the standard library that is very common and
useful: `Option`.
-->

非常に一般的で有用な別の標準ライブラリのenumを見てみましょう: `Option`です。

<!--
### The `Option` Enum and Its Advantages Over Null Values
-->

### `Option` enumとNull値に勝る利点

<!--
This section explores a case study of `Option`, which is another enum defined
by the standard library. The `Option` type encodes the very common scenario in
which a value could be something or it could be nothing.
-->

この節では、`Option`のケーススタディを掘り下げていきます。この型も標準ライブラリにより定義されているenumです。
この`Option`型は、値が何かかそうでないかという非常に一般的な筋書きをコード化します。

<!--
For example, if you request the first item in a non-empty list, you would get
a value. If you request the first item in an empty list, you would get nothing.
Expressing this concept in terms of the type system means the compiler can
check whether you’ve handled all the cases you should be handling; this
functionality can prevent bugs that are extremely common in other programming
languages.
-->

例えば、空でないリストの最初のアイテムをリクエストすると、値が得られるでしょう。
空リストの最初のアイテムをリクエストすると、何も得られないでしょう。
この概念を型システムの観点で表現することは、コンパイラが、プログラマが処理すべき場面全てを処理しているかどうかチェックできることを意味します;
この機能は、他の言語において、究極的にありふれたバグを阻止することができます。

<!--
Programming language design is often thought of in terms of which features you
include, but the features you exclude are important too. Rust doesn’t have the
null feature that many other languages have. *Null* is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.
-->

プログラミング言語のデザインは、しばしばどの機能を入れるかという観点で考えられるが、
除いた機能も重要なのです。Rustには、他の多くの言語にはあるnull機能がありません。
*null*とはそこに何も値がないことを意味する値です。nullのある言語において、
変数は常に二者択一どちらかの状態になります: nullかそうでないかです。

<!--
In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony
Hoare, the inventor of null, has this to say:
-->

nullの開発者であるトニー・ホーア(Tony Hoare)の2009年のプレゼンテーション、
"Null References: The Billion Dollar Mistake"(Null参照: 10億ドルの間違い)では、こんなことが語られています。

<!--
> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn’t resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
-->

> 私はそれを10億ドルの失敗と呼んでいます。その頃、私は、オブジェクト指向言語の参照に対する、
> 最初のわかりやすい型システムを設計していました。私の目標は、
> どんな参照の使用も全て完全に安全であるべきことを、コンパイラにそのチェックを自動で行ってもらって保証することだったのです。
> しかし、null参照を入れるという誘惑に打ち勝つことができませんでした。それは、単純に実装が非常に容易だったからです。
> これが無数のエラーや脆弱性、システムクラッシュにつながり、過去40年で10億ドルの苦痛や損害を引き起こしたであろうということなのです。

<!--
The problem with null values is that if you try to use a null value as a
not-null value, you’ll get an error of some kind. Because this null or not-null
property is pervasive, it’s extremely easy to make this kind of error.
-->

null値の問題は、nullの値をnullでない値のように使用しようとしたら、何らかの種類のエラーが出ることです。
このnullかそうでないかという特性は広く存在するので、この種の間違いを大変犯しやすいのです。

<!--
However, the concept that null is trying to express is still a useful one: a
null is a value that is currently invalid or absent for some reason.
-->

しかしながら、nullが表現しようとしている概念は、それでも役に立つものです: nullは、
何らかの理由で現在無効、または存在しない値のことなのです。

<!--
The problem isn’t really with the concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option]
as follows:
-->

問題は、全く概念にあるのではなく、特定の実装にあるのです。そんな感じなので、Rustにはnullがありませんが、
値が存在するか不在かという概念をコード化するenumならあります。このenumが`Option<T>`で、
以下のように[標準ライブラリに定義][option]されています。

```rust
enum Option<T> {
    None,
    Some(T),
}
```

<!--
The `Option<T>` enum is so useful that it’s even included in the prelude; you
don’t need to bring it into scope explicitly. Its variants are also included in
the prelude: you can use `Some` and `None` directly without the `Option::`
prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and
`None` are still variants of type `Option<T>`.
-->

`Option<T>`は有益すぎて、preludeにさえ含まれています; 明示的にスコープに導入する必要がないのです。
そしてその列挙子もまたpreludeに含まれています: `Some`と`None`を`Option::`の接頭辞なしに直接使えるわけです。
ただ、`Option<T>` enumはそうは言っても、普通のenumであり、`Some(T)`と`None`も`Option<T>`型のただの列挙子です。

<!--
The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
generic type parameter, and we’ll cover generics in more detail in Chapter 10.
For now, all you need to know is that `<T>` means that the `Some` variant of
the `Option` enum can hold one piece of data of any type, and that each
concrete type that gets used in place of `T` makes the overall `Option<T>` type
a different type. Here are some examples of using `Option` values to hold
number types and string types:
-->

`<T>`という記法は、まだ語っていないRustの機能です。
これはジェネリック型引数で、ジェネリクスについて詳しくは第10章で解説します。
とりあえず`<T>`の意味に関して知っておく必要があることは、`Option` enumの`Some`列挙子はあらゆる型のデータを1つだけ持つことができ、
`T`の代わりに使用される具体的な型に応じて、`Option<T>`型はそれぞれ異なる型になるということだけです。
以下は`Option`値を使って数値型や文字列型を保持する例です。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

<!--
The type of `some_number` is `Option<i32>`. The type of `some_char` is
`Option<char>`, which is a different type. Rust can infer these types because
we’ve specified a value inside the `Some` variant. For `absent_number`, Rust
requires us to annotate the overall `Option` type: the compiler can’t infer the
type that the corresponding `Some` variant will hold by looking only at a
`None` value. Here, we tell Rust that we mean for `absent_number` to be of type
`Option<i32>`.
-->

`some_number`の型は`Option<i32>`です。`some_char`の型は`Option<char>`で、これは先ほどとは異なる型です。
`Some`列挙子の中で値を指定しているので、コンパイラはこれらの型を推論することができます。
`absent_number`に関しては、コンパイラはジェネリクス適用後の`Option`型を注釈することを要求します:
`None`値を見ただけでは、それに対応する`Some`列挙子が保持する型をコンパイラは推論できないからです。
ここでは、`absent_number`が`Option<i32>`型を持つことをコンパイラに伝えています。

<!--
When we have a `Some` value, we know that a value is present and the value is
held within the `Some`. When we have a `None` value, in some sense it means the
same thing as null: we don’t have a valid value. So why is having `Option<T>`
any better than having null?
-->

`Some`値がある時、値が存在するとわかり、その値は、`Some`に保持されています。`None`値がある場合、
ある意味、nullと同じことを意図します: 有効な値がないのです。では、なぜ`Option<T>`の方が、
nullよりも少しでも好ましいのでしょうか？

<!--
In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won’t let us use an `Option<T>` value as if it were
definitely a valid value. For example, this code won’t compile, because it’s
trying to add an `i8` to an `Option<i8>`:
-->

簡潔に述べると、`Option<T>`と`T`(ここで`T`はどんな型でもいい)は異なる型なので、
コンパイラが`Option<T>`値を確実に有効な値かのようには使用させてくれません。
例えば、このコードは`i8`を`Option<i8>`に足そうとしているので、コンパイルできません。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

<!--
If we run this code, we get an error message like this one:
-->

このコードを動かしたら、以下のようなエラーメッセージが出ます。

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

<!--
Intense! In effect, this error message means that Rust doesn’t understand how
to add an `i8` and an `Option<i8>`, because they’re different types. When we
have a value of a type like `i8` in Rust, the compiler will ensure that we
always have a valid value. We can proceed confidently without having to check
for null before using that value. Only when we have an `Option<i8>` (or
whatever type of value we’re working with) do we have to worry about possibly
not having a value, and the compiler will make sure we handle that case before
using the value.
-->

なんて強烈な！実際に、このエラーメッセージは、`i8`と`Option<i8>`が異なる型なので、
足し合わせる方法がコンパイラにはわからないことを意味します。Rustにおいて、`i8`のような型の値がある場合、
コンパイラが常に有効な値であることを確認してくれます。この値を使う前にnullであることをチェックする必要なく、
自信を持って先に進むことができるのです。`Option<i8>`がある時(あるいはどんな型を扱おうとしていても)のみ、
値を保持していない可能性を心配する必要があるわけであり、
コンパイラはプログラマが値を使用する前にそのような場面を扱っているか確かめてくれます。

<!--
In other words, you have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn’t null when it actually is.
-->

言い換えると、`T`型の処理を行う前には、`Option<T>`を`T`に変換する必要があるわけです。一般的に、
これにより、nullの最もありふれた問題の一つを捕捉する一助になります: 実際にはnullなのに、
そうでないかのように想定することです。

<!--
Eliminating the risk of incorrectly assuming a not-null value helps you to be
more confident in your code. In order to have a value that can possibly be
null, you must explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn’t an
`Option<T>`, you *can* safely assume that the value isn’t null. This was a
deliberate design decision for Rust to limit null’s pervasiveness and increase
the safety of Rust code.
-->

値がnullでないと誤って想定するリスクを削減することは、コードに確信を持つための助けになります。
nullになる可能性のある値を保持するには、その値の型を`Option<T>`にすることで明示的に同意しなければなりません。
それからその値を使用する際には、値がnullである場合を明示的に処理する必要があります。
値が`Option<T>`以外の型であるところ全てにおいて、値がnullでないと安全に想定することが*できます*。
これは、Rustにとって、意図的な設計上の決定であり、nullの普遍性を制限し、Rustコードの安全性を向上させます。

<!--
So how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so that you can use that value? The `Option<T>` enum has a
large number of methods that are useful in a variety of situations; you can
check them out in [its documentation][docs]. Becoming familiar
with the methods on `Option<T>` will be extremely useful in your journey with
Rust.
-->

では、`Option<T>`型の値がある時、その値を使えるようにするには、どのように`Some`列挙子から`T`型の値を取り出せばいいのでしょうか？
`Option<T>`には様々な場面で有効に活用できる非常に多くのメソッドが用意されています;
[ドキュメント][docs]でそれらを確認できます。`Option<T>`のメソッドに馴染むと、
Rustの旅が極めて有益になるでしょう。

<!--
In general, in order to use an `Option<T>` value, you want to have code that
will handle each variant. You want some code that will run only when you have a
`Some(T)` value, and this code is allowed to use the inner `T`. You want some
other code to run only if you have a `None` value, and that code doesn’t have a
`T` value available. The `match` expression is a control flow construct that
does just this when used with enums: it will run different code depending on
which variant of the enum it has, and that code can use the data inside the
matching value.
-->

一般的に、`Option<T>`値を使うには、各列挙子を処理するコードが欲しくなります。
`Some(T)`値がある場合だけ走る何らかのコードが欲しくなり、このコードが内部の`T`を使用できます。
`None`値がある場合だけ走る別のコードが欲しくなり、そちらのコードは`T`を使用できない状態になります。
`match`式が、enumとともに使用した時にこれだけの動作をする制御フロー文法要素になります:
enumの列挙子によって、違うコードが走り、そのコードがマッチした値の中のデータを使用できるのです。

<!--
[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
-->

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[docs]: https://doc.rust-lang.org/std/option/enum.Option.html
