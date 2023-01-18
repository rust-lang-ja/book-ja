<!--
## Defining an Enum
-->

## Enum を定義する

<!--
Let’s look at a situation we might want to express in code and see why enums
are useful and more appropriate than structs in this case. Say we need to work
with IP addresses. Currently, two major standards are used for IP addresses:
version four and version six. These are the only possibilities for an IP
address that our program will come across: we can *enumerate* all possible
values, which is where enumeration gets its name.
-->

コードで表現したくなるかもしれない場面に目を向けて、enum が有用でこの場合、構造体よりも適切である理由を確認しましょう。
IP アドレスを扱う必要が出たとしましょう。現在、IP アドレスの規格は二つあります：バージョン 4 とバージョン 6 です。
これらは、プログラムが遭遇する IP アドレスのすべての可能性です：列挙型は、取りうる値をすべて*列挙*でき、
これが列挙型の名前の由来です。

<!--
Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate, because enum values can only be one of the variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.
-->

どんな IP アドレスも、バージョン 4 かバージョン 6 のどちらかになりますが、同時に両方にはなり得ません。
IP アドレスのその特性により、enum データ構造が適切なものになります。というのも、
enum の値は、その列挙子のいずれか一つにしかなり得ないからです。バージョン 4 とバージョン 6 のアドレスは、
どちらも根源的には IP アドレスですから、コードがいかなる種類の IP アドレスにも適用される場面を扱う際には、
同じ型として扱われるべきです。

<!--
We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`. These are known
as the *variants* of the enum:
-->

この概念をコードでは、`IpAddrKind`列挙型を定義し、IP アドレスがなりうる種類、`V4`と`V6`を列挙することで、
表現できます。これらは、enum の*列挙子*として知られています：

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

<!--
`IpAddrKind` is now a custom data type that we can use elsewhere in our code.
-->

これで、`IpAddrKind`はコードの他の場所で使用できる独自のデータ型になります。

<!--
### Enum Values
-->

### Enum の値

<!--
We can create instances of each of the two variants of `IpAddrKind` like this:
-->

以下のようにして、`IpAddrKind`の各列挙子のインスタンスは生成できます：

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

<!--
Note that the variants of the enum are namespaced under its identifier, and we
use a double colon to separate the two. The reason this is useful is that now
both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type:
`IpAddrKind`. We can then, for instance, define a function that takes any
`IpAddrKind`:
-->

enum の列挙子は、その識別子の元に名前空間分けされていることと、
2 連コロンを使ってその二つを区別していることに注意してください。
これが有効な理由は、こうすることで、値`IpAddrKind::V4`と`IpAddrKind::V6`という値は両方とも、
同じ型`IpAddrKind`になったからです。そうしたら、例えば、どんな`IpAddrKind`を取る関数も定義できるようになります。

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
fn route(ip_type: IpAddrKind) { }
```

<!--
And we can call this function with either variant:
-->

そして、この関数をどちらの列挙子に対しても呼び出せます：

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
# fn route(ip_type: IpAddrKind) { }
#
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

<!--
Using enums has even more advantages. Thinking more about our IP address type,
at the moment we don’t have a way to store the actual IP address *data*; we
only know what *kind* it is. Given that you just learned about structs in
Chapter 5, you might tackle this problem as shown in Listing 6-1.
-->

enum の利用には、さらなる利点さえもあります。この IP アドレス型についてもっと考えてみると、現状では、
実際の IP アドレスの*データ*を保持する方法がありません。つまり、どんな*種類*であるかを知っているだけです。
構造体について第 5 章で学んだばっかりとすると、この問題に対して、あなたはリスト 6-1 のように対処するかもしれません。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

<!--
<span class="caption">Listing 6-1: Storing the data and `IpAddrKind` variant of
an IP address using a `struct`</span>
-->

<span class="caption">リスト 6-1: IP アドレスのデータと`IpAddrKind`の列挙子を`struct`を使って保持する</span>

<!--
Here, we’ve defined a struct `IpAddr` that has two fields: a `kind` field that
is of type `IpAddrKind` (the enum we defined previously) and an `address` field
of type `String`. We have two instances of this struct. The first, `home`, has
the value `IpAddrKind::V4` as its `kind` with associated address data of
`127.0.0.1`. The second instance, `loopback`, has the other variant of
`IpAddrKind` as its `kind` value, `V6`, and has address `::1` associated with
it. We’ve used a struct to bundle the `kind` and `address` values together, so
now the variant is associated with the value.
-->

ここでは、二つのフィールドを持つ`IpAddr`という構造体を定義しています：`IpAddrKind`型 (先ほど定義した enum ですね) の`kind`フィールドと、
`String`型の`address`フィールドです。この構造体のインスタンスが 2 つあります。最初のインスタンス、
`home`には`kind`として`IpAddrKind::V4`があり、紐付けられたアドレスデータは`127.0.0.1`です。
2 番目のインスタンス、`loopback`には、`kind`の値として、`IpAddrKind`のもう一つの列挙子、`V6`があり、
アドレス`::1`が紐付いています。構造体を使って`kind`と`address`値を一緒に包んだので、
もう列挙子は値と紐付けられています。

<!--
We can represent the same concept in a more concise way using just an enum
rather than an enum inside a struct, by putting data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:
-->

各 enum の列挙子に直接データを格納して、enum を構造体内に使うというよりも enum だけを使って、
同じ概念をもっと簡潔な方法で表現することができます。この新しい`IpAddr`の定義は、
`V4`と`V6`列挙子両方に`String`値が紐付けられていることを述べています。

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

<!--
We attach data to each variant of the enum directly, so there is no need for an
extra struct.
-->

enum の各列挙子にデータを直接添付できるので、余計な構造体を作る必要は全くありません。

<!--
There’s another advantage to using an enum rather than a struct: each variant
can have different types and amounts of associated data. Version four type IP
addresses will always have four numeric components that will have values
between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but
still express `V6` addresses as one `String` value, we wouldn’t be able to with
a struct. Enums handle this case with ease:
-->

構造体よりも enum を使うことには、別の利点もあります：各列挙子に紐付けるデータの型と量は、異なってもいいのです。
バージョン 4 の IP アドレスには、常に 0 から 255 の値を持つ 4 つの数値があります。`V4`のアドレスは、4 つの`u8`型の値として格納するけれども、
`V6`のアドレスは引き続き、単独の`String`型の値で格納したかったとしても、構造体では不可能です。
enum なら、こんな場合も容易に対応できます：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
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

バージョン 4 とバージョン 6 の IP アドレスを格納するデータ構造を定義する複数の異なる方法を示してきました。
しかしながら、蓋を開けてみれば、IP アドレスを格納してその種類をコード化したくなるということは一般的なので、
[標準ライブラリに使用可能な定義があります！][IpAddr] 標準ライブラリでの`IpAddr`の定義のされ方を見てみましょう:
私たちが定義し、使用したのと全く同じ enum と列挙子がありますが、アドレスデータを二種の異なる構造体の形で列挙子に埋め込み、
この構造体は各列挙子用に異なる形で定義されています。

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html

<!--
```rust
struct Ipv4Addr {
// details elided
}
-->

<!--
struct Ipv6Addr {
// details elided
}
-->

<!--
enum IpAddr {
V4(Ipv4Addr),
V6(Ipv6Addr),
}
```
-->

```rust
struct Ipv4Addr {
    // 省略
}

struct Ipv6Addr {
    // 省略
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

このコードは、enum 列挙子内にいかなる種類のデータでも格納できることを描き出しています：
例を挙げれば、文字列、数値型、構造体などです。他の enum を含むことさえできます！また、
標準ライブラリの型は、あなたの想像するよりも複雑ではないことがしばしばあります。

<!--
Note that even though the standard library contains a definition for `IpAddr`,
we can still create and use our own definition without conflict because we
haven’t brought the standard library’s definition into our scope. We’ll talk
more about bringing types into scope in Chapter 7.
-->

標準ライブラリに`IpAddr`に対する定義は含まれるものの、標準ライブラリの定義をまだ我々のスコープに導入していないので、
干渉することなく自分自身の定義を生成して使用できることに注意してください。型をスコープに導入することについては、
第 7 章でもっと詳しく言及します。

<!--
Let’s look at another example of an enum in Listing 6-2: this one has a wide
variety of types embedded in its variants.
-->

リスト 6-2 で enum の別の例を見てみましょう：今回のコードは、幅広い種類の型が列挙子に埋め込まれています。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

<!--
<span class="caption">Listing 6-2: A `Message` enum whose variants each store
different amounts and types of values</span>
-->

<span class="caption">リスト 6-2: 列挙子各々が異なる型と量の値を格納する`Message` enum</span>

<!--
This enum has four variants with different types:
-->

この enum には、異なる型の列挙子が 4 つあります：

<!--
* `Quit` has no data associated with it at all.
* `Move` includes an anonymous struct inside it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32` values.
-->

* `Quit`には紐付けられたデータは全くなし。
* `Move`は、中に匿名構造体を含む。
* `Write`は、単独の`String`オブジェクトを含む。
* `ChangeColor`は、3 つの`i32`値を含む。

<!--
Defining an enum with variants such as the ones in Listing 6-2 is similar to
defining different kinds of struct definitions, except the enum doesn’t use the
`struct` keyword and all the variants are grouped together under the `Message`
type. The following structs could hold the same data that the preceding enum
variants hold:
-->

リスト 6-2 のような列挙子を含む enum を定義することは、enum の場合、`struct`キーワードを使わず、
全部の列挙子が`Message`型の元に分類される点を除いて、異なる種類の構造体定義を定義するのと類似しています。
以下の構造体も、先ほどの enum の列挙子が保持しているのと同じデータを格納することができるでしょう：

<!--
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
x: i32,
y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
-->

```rust
struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体
```

<!--
But if we used the different structs, which each have their own type, we
couldn't as easily define a function to take any of these kinds of messages as
we could with the `Message` enum defined in Listing 6-2, which is a single type.
-->

<!--
ちょっと文意を適切に表せているか怪しいかも
-->

ですが、異なる構造体を使っていたら、各々、それ自身の型があるので、単独の型になるリスト 6-2 で定義した`Message` enum ほど、
これらの種のメッセージいずれもとる関数を簡単に定義することはできないでしょう。

<!--
There is one more similarity between enums and structs: just as we’re able to
define methods on structs using `impl`, we’re also able to define methods on
enums. Here’s a method named `call` that we could define on our `Message` enum:
-->

enum と構造体にはもう 1 点似通っているところがあります：`impl`を使って構造体にメソッドを定義できるのと全く同様に、
enum にもメソッドを定義することができるのです。こちらは、`Message` enum 上に定義できる`call`という名前のメソッドです：

```rust
# enum Message {
#     Quit,
#     Move { x: i32, y: i32 },
#     Write(String),
#     ChangeColor(i32, i32, i32),
# }
#
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
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

非常に一般的で有用な別の標準ライブラリの enum を見てみましょう：`Option`です。

<!--
### The `Option` Enum and Its Advantages Over Null Values
-->

### `Option` enum と Null 値に勝る利点

<!--
In the previous section, we looked at how the `IpAddr` enum let us use Rust’s
type system to encode more information than just the data into our program.
This section explores a case study of `Option`, which is another enum defined
by the standard library. The `Option` type is used in many places because it
encodes the very common scenario in which a value could be something or it
could be nothing. Expressing this concept in terms of the type system means the
compiler can check that you’ve handled all the cases you should be handling;
this functionality can prevent bugs that are extremely common in other
programming languages.
-->

前節で、`IpAddr` enum が Rust の型システムを使用して、プログラムにデータ以上の情報をコード化できる方法を目撃しました。
この節では、`Option`のケーススタディを掘り下げていきます。この型も標準ライブラリにより定義されている enum です。
この`Option`型はいろんな箇所で使用されます。なぜなら、値が何かかそうでないかという非常に一般的な筋書きをコード化するからです。
この概念を型システムの観点で表現することは、コンパイラが、プログラマが処理すべき場面全てを処理していることをチェックできることを意味します;
この機能は、他の言語において、究極的にありふれたバグを阻止することができます。

<!--
Programming language design is often thought of in terms of which features you
include, but the features you exclude are important too. Rust doesn’t have the
null feature that many other languages have. *Null* is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.
-->

プログラミング言語のデザインは、しばしばどの機能を入れるかという観点で考えられるが、
除いた機能も重要なのです。Rust には、他の多くの言語にはある null 機能がありません。
*null*とはそこに何も値がないことを意味する値です。null のある言語において、
変数は常に二者択一どちらかの状態になります：null かそうでないかです。

<!--
In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony
Hoare, the inventor of null, has this to say:
-->

null の開発者であるトニー・ホーア (Tony Hoare) の 2009 年のプレゼンテーション、
"Null References: The Billion Dollar Mistake"(Null 参照：10 億ドルの間違い) では、こんなことが語られています。

<!--
> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn't resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
-->

> 私はそれを 10 億ドルの失敗と呼んでいます。その頃、私は、オブジェクト指向言語の参照に対する、
> 最初のわかりやすい型システムを設計していました。私の目標は、
> どんな参照の使用も全て完全に安全であるべきことを、コンパイラにそのチェックを自動で行ってもらって保証することだったのです。
> しかし、null 参照を入れるという誘惑に打ち勝つことができませんでした。それは、単純に実装が非常に容易だったからです。
> これが無数のエラーや脆弱性、システムクラッシュにつながり、過去 40 年で 10 億ドルの苦痛や損害を引き起こしたであろうということなのです。

<!--
The problem with null values is that if you try to use a null value as a
not-null value, you’ll get an error of some kind. Because this null or not-null
property is pervasive, it’s extremely easy to make this kind of error.
-->

null 値の問題は、null の値を null でない値のように使用しようとしたら、何らかの種類のエラーが出ることです。
この null かそうでないかという特性は広く存在するので、この種の間違いを大変犯しやすいのです。

<!--
However, the concept that null is trying to express is still a useful one: a
null is a value that is currently invalid or absent for some reason.
-->

しかしながら、null が表現しようとしている概念は、それでも役に立つものです：null は、
何らかの理由で現在無効、または存在しない値のことなのです。

<!--
The problem isn’t really with the concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option]
as follows:
-->

問題は、全く概念にあるのではなく、特定の実装にあるのです。そんな感じなので、Rust には null がありませんが、
値が存在するか不在かという概念をコード化する enum ならあります。この enum が`Option<T>`で、
以下のように[標準ライブラリに定義][option]されています。

[option]: https://doc.rust-lang.org/std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<!--
The `Option<T>` enum is so useful that it’s even included in the prelude; you
don’t need to bring it into scope explicitly. In addition, so are its variants:
you can use `Some` and `None` directly without the `Option::` prefix. The
`Option<T>` is still just a regular enum, and `Some(T)` and `None` are
still variants of type `Option<T>`.
-->

`Option<T>`は有益すぎて、初期化処理 (prelude) にさえ含まれています。つまり、明示的にスコープに導入する必要がないのです。
さらに、列挙子もそうなっています：`Some`と`None`を`Option::`の接頭辞なしに直接使えるわけです。
ただ、`Option<T>`はそうは言っても、普通の enum であり、`Some(T)`と`None`も`Option<T>`型のただの列挙子です。

<!--
The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
generic type parameter, and we’ll cover generics in more detail in Chapter 10.
For now, all you need to know is that `<T>` means the `Some` variant of the
`Option` enum can hold one piece of data of any type. Here are some examples of
using `Option` values to hold number types and string types:
-->

`<T>`という記法は、まだ語っていない Rust の機能です。これは、ジェネリック型引数であり、ジェネリクスについて詳しくは、
第 10 章で解説します。とりあえず、知っておく必要があることは、`<T>`は、`Option` enum の`Some`列挙子が、
あらゆる型のデータを 1 つだけ持つことができることを意味していることだけです。こちらは、
`Option`値を使って、数値型や文字列型を保持する例です。

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

<!--
If we use `None` rather than `Some`, we need to tell Rust what type of
`Option<T>` we have, because the compiler can't infer the type that the `Some`
variant will hold by looking only at a `None` value.
-->

`Some`ではなく、`None`を使ったら、コンパイラに`Option<T>`の型が何になるかを教えなければいけません。
というのも、`None`値を見ただけでは、`Some`列挙子が保持する型をコンパイラが推論できないからです。

<!--
When we have a `Some` value, we know that a value is present and the value is
held within the `Some`. When we have a `None` value, in some sense, it means
the same thing as null: we don’t have a valid value. So why is having
`Option<T>` any better than having null?
-->

`Some`値がある時、値が存在するとわかり、その値は、`Some`に保持されています。`None`値がある場合、
ある意味、null と同じことを意図します：有効な値がないのです。では、なぜ`Option<T>`の方が、
null よりも少しでも好ましいのでしょうか？

<!--
In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won’t let us use an `Option<T>` value as if it was
definitely a valid value. For example, this code won’t compile because it’s
trying to add an `i8` to an `Option<i8>`:
-->

簡潔に述べると、`Option<T>`と`T`(ここで`T`はどんな型でもいい) は異なる型なので、
コンパイラが`Option<T>`値を確実に有効な値かのようには使用させてくれません。
例えば、このコードは`i8`を`Option<i8>`に足そうとしているので、コンパイルできません。

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

<!--
If we run this code, we get an error message like this:
-->

このコードを動かしたら、以下のようなエラーメッセージが出ます。

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
(エラー: `i8: std::ops::Add<std::option::Option<i8>>`というトレイト境界が満たされていません)
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
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
足し合わせる方法がコンパイラにはわからないことを意味します。Rust において、`i8`のような型の値がある場合、
コンパイラが常に有効な値であることを確認してくれます。この値を使う前に null であることをチェックする必要なく、
自信を持って先に進むことができるのです。`Option<i8>`がある時 (あるいはどんな型を扱おうとしていても) のみ、
値を保持していない可能性を心配する必要があるわけであり、
コンパイラはプログラマが値を使用する前にそのような場面を扱っているか確かめてくれます。

<!--
In other words, you have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn’t null when it actually
is.
-->

言い換えると、`T`型の処理を行う前には、`Option<T>`を`T`に変換する必要があるわけです。一般的に、
これにより、null の最もありふれた問題の一つを捕捉する一助になります：実際には null なのに、
そうでないかのように想定することです。

<!--
Not having to worry about incorrectly assuming a not-null value helps you to be
more confident in your code. In order to have a value that can possibly be
null, you must explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn’t an
`Option<T>`, you *can* safely assume that the value isn’t null. This was a
deliberate design decision for Rust to limit null’s pervasiveness and increase
the safety of Rust code.
-->

不正確に null でない値を想定する心配をしなくてもよいということは、コード内でより自信を持てることになります。
null になる可能性のある値を保持するには、その値の型を`Option<T>`にすることで明示的に同意しなければなりません。
それからその値を使用する際には、値が null である場合を明示的に処理する必要があります。
値が`Option<T>`以外の型であるところ全てにおいて、値が null でないと安全に想定することが*できます*。
これは、Rust にとって、意図的な設計上の決定であり、null の普遍性を制限し、Rust コードの安全性を向上させます。

<!--
So, how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so you can use that value? The `Option<T>` enum has a large
number of methods that are useful in a variety of situations; you can check
them out in [its documentation][docs]. Becoming familiar with
the methods on `Option<T>` will be extremely useful in your journey with Rust.
-->

では、`Option<T>`型の値がある時、その値を使えるようにするには、どのように`Some`列挙子から`T`型の値を取り出せばいいのでしょうか？
`Option<T>`には様々な場面で有効に活用できる非常に多くのメソッドが用意されています;
[ドキュメント][docs]でそれらを確認できます。`Option<T>`のメソッドに馴染むと、
Rust の旅が極めて有益になるでしょう。

[docs]: https://doc.rust-lang.org/std/option/enum.Option.html

<!--
In general, in order to use an `Option<T>` value, you want to have code that
will handle each variant. We want some code that will run only when you have a
`Some(T)` value, and this code is allowed to use the inner `T`. We want some
other code to run if we have a `None` value, and that code doesn’t have a `T`
value available. The `match` expression is a control flow construct that does
just this when used with enums: it will run different code depending on which
variant of the enum it has, and that code can use the data inside the matching
value.
-->

一般的に、`Option<T>`値を使うには、各列挙子を処理するコードが欲しくなります。
`Some(T)`値がある時だけ走る何らかのコードが欲しくなり、このコードが内部の`T`を使用できます。
`None`値があった場合に走る別のコードが欲しくなり、そちらのコードは`T`値は使用できない状態になります。
`match`式が、enum とともに使用した時にこれだけの動作をする制御フロー文法要素になります：
enum の列挙子によって、違うコードが走り、そのコードがマッチした値の中のデータを使用できるのです。
