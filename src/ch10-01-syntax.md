<!--
## Generic Data Types
-->

## ジェネリックなデータ型

<!--
We use generics to create definitions for items like function signatures or
structs, which we can then use with many different concrete data types. Let’s
first look at how to define functions, structs, enums, and methods using
generics. Then we’ll discuss how generics affect code performance.
-->

関数シグニチャや構造体などの要素の定義を生成するのにジェネリクスを使用することができ、
それはさらに他の多くの具体的なデータ型と使用することもできます。まずは、
ジェネリクスで関数、構造体、enum、メソッドを定義する方法を見ましょう。それから、
ジェネリクスがコードのパフォーマンスに与える影響を議論します。

<!--
### In Function Definitions
-->

### 関数定義では

<!--
When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code duplication.
-->

ジェネリクスを使用する関数を定義する時、通常、引数や戻り値のデータ型を指定する関数のシグニチャにジェネリクスを配置します。
そうすることでコードがより柔軟になり、コードの重複を阻止しつつ、関数の呼び出し元により多くの機能を提供します。

<!--
Continuing with our `largest` function, Listing 10-4 shows two functions that
both find the largest value in a slice. We'll then combine these into a single
function that uses generics.
-->

`largest`関数を続けます。リスト10-4はどちらもスライスから最大値を探す2つの関数を示しています。
ここから、これらをジェネリクスを使用する単一の関数にまとめます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-4: Two functions that differ only in their
names and the types in their signatures</span>
-->

<span class="caption">リスト10-4: 名前とシグニチャの型のみが異なる2つの関数</span>

<!--
The `largest_i32` function is the one we extracted in Listing 10-3 that finds
the largest `i32` in a slice. The `largest_char` function finds the largest
`char` in a slice. The function bodies have the same code, so let’s eliminate
the duplication by introducing a generic type parameter in a single function.
-->

`largest_i32`関数は、リスト10-3で抽出したスライスから最大の`i32`を探す関数です。
`largest_char`関数は、スライスから最大の`char`を探します。関数本体には同じコードがあるので、
単独の関数にジェネリックな型引数を導入してこの重複を排除しましょう。

<!--
To parameterize the types in a new single function, we need to name the type
parameter, just as we do for the value parameters to a function. You can use
any identifier as a type parameter name. But we’ll use `T` because, by
convention, type parameter names in Rust are short, often just a letter, and
Rust’s type-naming convention is UpperCamelCase. Short for “type,” `T` is the
default choice of most Rust programmers.
-->

新しい単一の関数の型を引数にするには、ちょうど関数の値引数のように型引数に名前をつける必要があります。
型引数の名前にはどんな識別子も使用できますが、`T`を使用します。というのも、慣習では、
Rustの型引数名は短く(しばしばたった1文字になります)、Rustの型の命名規則がアッパーキャメルケースだからです。
"type"の省略形なので、`T`が多くのRustプログラマの既定の選択なのです。

<!--
When we use a parameter in the body of the function, we have to declare the
parameter name in the signature so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have
to declare the type parameter name before we use it. To define the generic
`largest` function, place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list, like this:
-->

関数の本体で引数を使用するとき、コンパイラがその名前の意味を把握できるようにシグニチャでその引数名を宣言しなければなりません。
同様に、型引数名を関数シグニチャで使用する際には、使用する前に型引数名を宣言しなければなりません。
ジェネリックな`largest`関数を定義するために、型名宣言を山カッコ(`<>`)内、関数名と引数リストの間に配置してください。
こんな感じに:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

<!--
We read this definition as: the function `largest` is generic over some type
`T`. This function has one parameter named `list`, which is a slice of values
of type `T`. The `largest` function will return a reference to a value of the
same type `T`.
-->

この定義は以下のように解読します: 関数`largest`は、なんらかの型`T`に関してジェネリックであると。
この関数には`list`という引数が1つあり、これは型`T`の値のスライスです。
`largest`関数は同じ`T`型の値への参照を返します。

<!--
Listing 10-5 shows the combined `largest` function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of `i32` values or `char` values. Note that this code won’t
compile yet, but we’ll fix it later in this chapter.
-->

リスト10-5は、シグニチャにジェネリックなデータ型を使用して`largest`関数定義を組み合わせたものを示しています。
このリストはさらに、この関数を`i32`値か`char`値のどちらかで呼べる方法も表示しています。
このコードはまだコンパイルできないことに注意してください。ですが、この章の後ほど修正します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<!--
<span class="caption">Listing 10-5: The `largest` function using generic type
parameters; this doesn’t yet compile</span>
-->

<span class="caption">リスト10-5: ジェネリックな型引数を使用する`largest`関数; まだコンパイルできません</span>

<!--
If we compile this code right now, we’ll get this error:
-->

直ちにこのコードをコンパイルしたら、以下のようなエラーが出ます:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

<!--
The help text mentions `std::cmp::PartialOrd`, which is a *trait*, and we’re
going to talk about traits in the next section. For now, know that this error
states that the body of `largest` won’t work for all possible types that `T`
could be. Because we want to compare values of type `T` in the body, we can
only use types whose values can be ordered. To enable comparisons, the standard
library has the `std::cmp::PartialOrd` trait that you can implement on types
(see Appendix C for more on this trait). By following the help text's
suggestion, we restrict the types valid for `T` to only those that implement
`PartialOrd` and this example will compile, because the standard library
implements `PartialOrd` on both `i32` and `char`.
-->

ヘルプメッセージが`std::cmp::PartialOrd`に触れています。これは、*トレイト*です。トレイトについては、次の節で語ります。
今のところは、このエラーは、`largest`の本体は、`T`がなりうる可能性のある全ての型に対して動作するとは限らないと主張している、と理解してください。
本体で型`T`の値を比較したいので、値が順序付け可能な型のみしか使用できないのです。比較を可能にするために、
標準ライブラリには型に実装できる`std::cmp::PartialOrd`トレイトがあります(このトレイトについて詳しくは付録Cを参照されたし)。
ヘルプメッセージの提案に従うことで、`T`として有効な型を`PartialOrd`を実装するもののみに制限することができ、
そうすると、標準ライブラリは`i32`と`char`の両方に`PartialOrd`を実装しているので、この例はコンパイルできるようになるでしょう。

<!--
### In Struct Definitions
-->

### 構造体定義では

<!--
We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. Listing 10-6 defines a `Point<T>` struct to hold
`x` and `y` coordinate values of any type.
-->

構造体を定義して`<>`記法で1つ以上のフィールドにジェネリックな型引数を使用することもできます。
リスト10-6は、`Point<T>`構造体を定義してあらゆる型の`x`と`y`座標を保持しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<!--
<span class="caption">Listing 10-6: A `Point<T>` struct that holds `x` and `y`
values of type `T`</span>
-->

<span class="caption">リスト10-6: 型`T`の`x`と`y`値を保持する`Point<T>`構造体</span>

<!--
The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we use the generic type
in the struct definition where we would otherwise specify concrete data types.
-->

構造体定義でジェネリクスを使用する記法は、関数定義のものと似ています。
まず、山カッコ内に型引数の名前を構造体名の直後に宣言します。
そして、本来具体的なデータ型を記述する構造体定義の箇所に、ジェネリックな型を使用します。

<!--
Note that because we’ve used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are *both* that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as in
Listing 10-7, our code won’t compile.
-->

ジェネリックな型を1つだけ使用して`Point<T>`を定義したので、この定義は、`Point<T>`構造体がなんらかの型`T`に関して、
ジェネリックであると述べていて、その型がなんであれ、`x`と`y`のフィールドは*両方*その同じ型になっていることに注意してください。
リスト10-7のように、異なる型の値のある`Point<T>`のインスタンスを生成すれば、コードはコンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<!--
<span class="caption">Listing 10-7: The fields `x` and `y` must be the same
type because both have the same generic data type `T`.</span>
-->

<span class="caption">リスト10-7: どちらも同じジェネリックなデータ型`T`なので、`x`と`y`というフィールドは同じ型でなければならない</span>

<!--
In this example, when we assign the integer value 5 to `x`, we let the compiler
know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then when we specify 4.0 for `y`, which we’ve defined to have the
same type as `x`, we’ll get a type mismatch error like this:
-->

この例で、`x`に整数値5を代入すると、この`Point<T>`のインスタンスに対するジェネリックな型`T`は整数になるとコンパイラに知らせます。
それから`y`に4.0を指定する時に、このフィールドは`x`と同じ型と定義したはずなので、このように型不一致エラーが出ます:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

<!--
To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
Listing 10-8, we change the definition of `Point` to be generic over types `T`
and `U` where `x` is of type `T` and `y` is of type `U`.
-->

`x`と`y`が両方ジェネリックだけれども、異なる型になり得る`Point`構造体を定義するには、
複数のジェネリックな型引数を使用できます。例えば、リスト10-8では、`Point`の定義を変更して、
型`T`と`U`に関してジェネリックにし、`x`が型`T`で、`y`が型`U`になります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<!--
<span class="caption">Listing 10-8: A `Point<T, U>` generic over two types so
that `x` and `y` can be values of different types</span>
-->

<span class="caption">リスト10-8: `Point<T, U>`は2つの型に関してジェネリックなので、`x`と`y`は異なる型の値になり得る</span>

<!--
Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. If you're finding you need lots of generic types in
your code, it could indicate that your code needs restructuring into smaller
pieces.
-->

これで、示された`Point`インスタンスは全部使用可能です！所望の数だけ定義でジェネリックな型引数を使用できますが、
数個以上使用すると、コードが読みづらくなります。コードで多くのジェネリックな型が必要になることに気づいた時は、
コードの小分けが必要なサインかもしれません。

<!--
### In Enum Definitions
-->

### enum定義では

<!--
As we did with structs, we can define enums to hold generic data types in their
variants. Let’s take another look at the `Option<T>` enum that the standard
library provides, which we used in Chapter 6:
-->

構造体のように、列挙子にジェネリックなデータ型を保持するenumを定義することができます。
標準ライブラリが提供している`Option<T>` enumをもう一度見ましょう。このenumは第6章で使用しました:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<!--
This definition should now make more sense to you. As you can see, the
`Option<T>` enum is generic over type `T` and has two variants: `Some`, which
holds one value of type `T`, and a `None` variant that doesn’t hold any value.
By using the `Option<T>` enum, we can express the abstract concept of an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.
-->

この定義はもう、あなたにとってより道理が通っているはずです。ご覧の通り、`Option<T>` enumは、
型`T`に関してジェネリックで2つの列挙子のあるenumです: その列挙子は、型`T`の値を保持する`Some`と、
値を何も保持しない`None`です。`Option<T>` enumを使用することで、オプショナルな値があるという抽象的な概念を表現でき、
`Option<T>`はジェネリックなので、オプショナルな値の型に関わらず、この抽象を使用できます。

<!--
Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:
-->

enumも複数のジェネリックな型を使用できます。第9章で使用した`Result` enumの定義が一例です:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`). In fact, this is what we used to open a
file in Listing 9-3, where `T` was filled in with the type `std::fs::File` when
the file was opened successfully and `E` was filled in with the type
`std::io::Error` when there were problems opening the file.
-->

`Result` enumは2つの型`T`、`E`に関してジェネリックで、2つの列挙子があります: 型`T`の値を保持する`Ok`と、
型`E`の値を保持する`Err`です。この定義により、`Result` enumを、成功する(なんらかの型`T`の値を返す)か、
失敗する(なんらかの型`E`のエラーを返す)可能性のある処理がある、あらゆる箇所に使用するのが便利になります。
事実、ファイルを開くのに成功した時に`T`に型`std::fs::File`が入り、ファイルを開く際に問題があった時に`E`に型`std::io::Error`が入ったものが、
リスト9-3でファイルを開くのに使用したものです。

<!--
When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
avoid duplication by using generic types instead.
-->

自分のコード内で、保持している値の型のみが異なる構造体やenum定義の場面を認識したら、
代わりにジェネリックな型を使用することで重複を避けることができます。

<!--
### In Method Definitions
-->

### メソッド定義では

<!--
We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions, too. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6 with a method named `x` implemented on it.
-->

(第5章のように、)定義にジェネリックな型を使うメソッドを構造体やenumに実装することもできます。リスト10-9は、
リスト10-6で定義した`Point<T>`構造体に`x`というメソッドを実装したものを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<!--
<span class="caption">Listing 10-9: Implementing a method named `x` on the
`Point<T>` struct that will return a reference to the `x` field of type
`T`</span>
-->

<span class="caption">リスト10-9: 型`T`の`x`フィールドへの参照を返す`x`というメソッドを`Point<T>`構造体に実装する</span>

<!--
Here, we’ve defined a method named `x` on `Point<T>` that returns a reference
to the data in the field `x`.
-->

ここで、フィールド`x`のデータへの参照を返す`x`というメソッドを`Point<T>`に定義しました。

<!--
Note that we have to declare `T` just after `impl` so we can use `T` to specify
that we’re implementing methods on the type `Point<T>`. By declaring `T` as a
generic type after `impl`, Rust can identify that the type in the angle
brackets in `Point` is a generic type rather than a concrete type. We could
have chosen a different name for this generic parameter than the generic
parameter declared in the struct definition, but using the same name is
conventional. Methods written within an `impl` that declares the generic type
will be defined on any instance of the type, no matter what concrete type ends
up substituting for the generic type.
-->

`impl`の直後に`T`を宣言しなければならないことに注意してください。こうすることで、型`Point<T>`にメソッドを実装していることを指定するために、`T`を使用することができます。
`impl`の後に`T`をジェネリックな型として宣言することで、コンパイラは、`Point`の山カッコ内の型が、
具体的な型ではなくジェネリックな型であることを認識できるのです。
このジェネリック引数には、構造体定義内で宣言したジェネリック引数とは異なる名前を選択することもできますが、
同じ名前を使用するのが慣用的です。
ジェネリック型を宣言する`impl`内に書かれたメソッドは、そのジェネリック型を置き換えることになる具体型が何であっても、その型のインスタンスの任意の値に対して定義されます。

<!--
We can also specify constraints on generic types when defining methods on the
type. We could, for example, implement methods only on `Point<f32>` instances
rather than on `Point<T>` instances with any generic type. In Listing 10-10 we
use the concrete type `f32`, meaning we don’t declare any types after `impl`.
-->

型にメソッドを定義するときに、ジェネリック型に制約を指定することもできます。
例えば、ジェネリックな型を持つ`Point<T>`インスタンスではなく、`Point<f32>`だけにメソッドを実装することもできるでしょう。
リスト10-10では、具体的な型`f32`を使用しています。つまり、`impl`の後に型を宣言しません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-10: An `impl` block that only applies to a
struct with a particular concrete type for the generic type parameter `T`</span>
-->

<span class="caption">リスト10-10: ジェネリックな型引数`T`に対して特定の具体的な型がある構造体にのみ適用される`impl`ブロック</span>

<!--
This code means the type `Point<f32>` will have a `distance_from_origin`
method; other instances of `Point<T>` where `T` is not of type `f32` will not
have this method defined. The method measures how far our point is from the
point at coordinates (0.0, 0.0) and uses mathematical operations that are
available only for floating point types.
-->

このコードは、`Point<f32>`には`distance_from_origin`メソッドが存在するが、
`T`が`f32`ではない`Point<T>`の他のインスタンスにはこのメソッドが定義されないことを意味します。
このメソッドは、この点が座標(0.0, 0.0)の点からどれだけ離れているかを測定し、
浮動小数点数にのみ利用可能な数学的処理を使用します。

<!--
Generic type parameters in a struct definition aren’t always the same as those
you use in that same struct’s method signatures. Listing 10-11 uses the generic
types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the `mixup` method
signature to make the example clearer. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `X1`) and the `y`
value from the passed-in `Point` (of type `Y2`).
-->

構造体定義のジェネリックな型引数は、必ずしもその構造体のメソッドシグニチャで使用するものと同じにはなりません。
リスト10-11では、例をより見通しよくするために、`Point`構造体に対してはジェネリック型`X1`と`Y1`を使用し、
`mixup`メソッドのシグネチャに対しては`X2`と`Y2`を使用しています。
このメソッドは、(型`X1`の)`self`の`Point`の`x`値と渡した(型`Y2`の)`Point`の`y`値から新しい`Point`インスタンスを生成します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<!--
<span class="caption">Listing 10-11: A method that uses generic types different
from its struct’s definition</span>
-->

<span class="caption">リスト10-11: 構造体定義とは異なるジェネリックな型を使用するメソッド</span>

<!--
In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable
will have a `char` for `y`, because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.
-->

`main`で、`x`(値は`5`)に`i32`、`y`(値は`10.4`)に`f64`を持つ`Point`を定義しました。`p2`変数は、
`x`(値は`"Hello"`)に文字列スライス、`y`(値は`c`)に`char`を持つ`Point`構造体です。
引数`p2`で`p1`に`mixup`を呼び出すと、`p3`が得られ、`x`は`i32`になります。`x`は`p1`由来だからです。
`p3`変数の`y`は、`char`になります。`y`は`p2`由来だからです。`println!`マクロの呼び出しは、
`p3.x = 5, p3.y = c`と出力するでしょう。

<!--
The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `X1` and `Y1` are declared after
`impl` because they go with the struct definition. The generic parameters `X2`
and `Y2` are declared after `fn mixup`, because they’re only relevant to the
method.
-->

この例の目的は、一部のジェネリックな引数は`impl`で宣言され、他の一部はメソッド定義で宣言される場面をデモすることです。
ここで、ジェネリックな引数`X1`と`Y1`は`impl`の後に宣言されています。構造体定義にはまるからです。
ジェネリックな引数`X2`と`Y2`は`fn mixup`の後に宣言されています。何故なら、このメソッドにしか関係ないからです。

<!--
### Performance of Code Using Generics
-->

### ジェネリクスを使用したコードのパフォーマンス

<!--
You might be wondering whether there is a runtime cost when using generic type
parameters. The good news is that using generic types won't make your program run
any slower than it would with concrete types.
-->

ジェネリックな型引数を使用すると、実行時にコストが発生するのかな、と思うかもしれません。
嬉しいことに、ジェネリックな型を使用したからといって、具体的な型を使って同じことを行う場合よりもプログラムの実行が遅くなることはありません。

<!--
Rust accomplishes this by performing monomorphization of the code using
generics at compile time. *Monomorphization* is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled. In this process, the compiler does the opposite of the steps we used
to create the generic function in Listing 10-5: the compiler looks at all the
places where generic code is called and generates code for the concrete types
the generic code is called with.

-->

コンパイラはこれを、ジェネリクスを使用しているコードの単相化をコンパイル時に行うことで達成しています。
*単相化*(monomorphization)は、コンパイル時に使用されている具体的な型を入れることで、
ジェネリックなコードを特定のコードに変換する過程のことです。
この過程において、コンパイラは、リスト10-5でジェネリックな関数を生成するために使用した手順と真逆のことをしています:
コンパイラは、ジェネリックなコードが呼び出されている箇所全部を見て、
ジェネリックなコードが呼び出されている具体的な型のコードを生成するのです。

<!--
Let’s look at how this works by using the standard library’s generic
`Option<T>` enum:
-->

標準ライブラリのジェネリックな`Option<T>` enumを使用して、これがどう機能するのか見てみましょう:

```rust
let integer = Some(5);
let float = Some(5.0);
```

<!--
When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: one is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into two
definitions specialized to `i32` and `f64`, thereby replacing the generic
definition with the specific ones.
-->

コンパイラがこのコードをコンパイルすると、単相化を行います。その過程で、コンパイラは`Option<T>`のインスタンスに使用された値を読み取り、
2種類の`Option<T>`を識別します: 一方は`i32`で、もう片方は`f64`です。そのように、
コンパイラは、`Option<T>`のジェネリックな定義を`i32`と`f64`に特化した2つの定義に展開し、
それにより、ジェネリックな定義を特定の定義と置き換えます。

<!--
The monomorphized version of the code looks similar to the following (the
compiler uses different names than what we’re using here for illustration):
-->

単相化されたバージョンのコードは、以下のようなものになります
(コンパイラは、ここで説明のために使用しているものとは異なる名前を使用します):

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

<!--
The generic `Option<T>` is replaced with the specific definitions created by
the compiler. Because Rust compiles generic code into code that specifies the
type in each instance, we pay no runtime cost for using generics. When the code
runs, it performs just as it would if we had duplicated each definition by
hand. The process of monomorphization makes Rust’s generics extremely efficient
at runtime.
-->

ジェネリックな`Option<T>`は、コンパイラによって生成される特化された定義で置き換えられます。
Rustでは、ジェネリックなコードを各インスタンスで型を指定したコードにコンパイルするので、
ジェネリクスを使用することに対して実行時コストを払うことはありません。コードを実行すると、
それぞれの定義を手作業で複製した時のように振る舞います。単相化の過程により、
Rustのジェネリクスは実行時に究極的に効率的になるのです。
