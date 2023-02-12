<!--
## Macros
-->
## マクロ

<!--
We’ve used macros like `println!` throughout this book, but we haven’t fully
explored what a macro is and how it works. The term *macro* refers to a family
of features in Rust: *declarative* macros with `macro_rules!` and three kinds
of *procedural* macros:
-->
本全体を通じて`println!`のようなマクロを使用してきましたが、マクロがなんなのかや、
どう動いているのかということは完全には探究していませんでした。
Rustにおいて、*マクロ*という用語はある機能の集合のことを指します：`macro_rules!`を使った *宣言的 (declarative)* マクロと、3種類の *手続き的 (procedural)* マクロ：

<!--
* Custom `#[derive]` macros that specify code added with the `derive` attribute
  used on structs and enums
* Attribute-like macros that define custom attributes usable on any item
* Function-like macros that look like function calls but operate on the tokens
  specified as their argument
-->
* 構造体とenumに`derive`属性を使ったときに追加されるコードを指定する、カスタムの`#[derive]`マクロ
* 任意の要素に使えるカスタムの属性を定義する、属性風のマクロ
* 関数のように見えるが、引数として指定されたトークンに対して作用する関数風のマクロ

です。

<!--
We’ll talk about each of these in turn, but first, let’s look at why we even
need macros when we already have functions.
-->
それぞれについて一つずつ話していきますが、その前にまず、どうして関数がすでにあるのにマクロなんてものが必要なのか見てみましょう。

<!--
### The Difference Between Macros and Functions
-->

### マクロと関数の違い

<!--
Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In Appendix C, we discuss the `derive`
attribute, which generates an implementation of various traits for you. We’ve
also used the `println!` and `vec!` macros throughout the book. All of these
macros *expand* to produce more code than the code you’ve written manually.
-->

基本的に、マクロは、他のコードを記述するコードを書く術であり、これは*メタプログラミング*として知られています。
付録Cで、`derive`属性を議論し、これは、色々なトレイトの実装を生成してくれるのでした。
また、本を通して`println!`や`vec!`マクロを使用してきました。これらのマクロは全て、*展開*され、
手で書いたよりも多くのコードを生成します。

<!--
Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t.
-->

メタプログラミングは、書いて管理しなければならないコード量を減らすのに有用で、これは、関数の役目の一つでもあります。
ですが、マクロには関数にはない追加の力があります。

<!--
A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call `println!("hello")` with one argument or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t, because it gets
called at runtime and a trait needs to be implemented at compile time.
-->

関数シグニチャは、関数の引数の数と型を宣言しなければなりません。一方、マクロは可変長の引数を取れます:
`println!("hello")`のように1引数で呼んだり、`println!("hello {}", name)`のように2引数で呼んだりできるのです。
また、マクロは、コンパイラがコードの意味を解釈する前に展開されるので、例えば、
与えられた型にトレイトを実装できます。関数ではできません。何故なら、関数は実行時に呼ばれ、
トレイトはコンパイル時に実装される必要があるからです。

<!--
The downside to implementing a macro instead of a function is that macro
definitions are more complex than function definitions because you’re writing
Rust code that writes Rust code. Due to this indirection, macro definitions are
generally more difficult to read, understand, and maintain than function
definitions.
-->

関数ではなくマクロを実装する欠点は、Rustコードを記述するRustコードを書いているので、
関数定義よりもマクロ定義は複雑になることです。この間接性のために、マクロ定義は一般的に、
関数定義よりも、読みにくく、わかりにくく、管理しづらいです。

<!--
Another important difference between macros and functions is that you must
define macros or bring them into scope *before* you call them in a file, as
opposed to functions you can define anywhere and call anywhere.
-->

マクロと関数にはもう一つ、重要な違いがあります: ファイル内で呼び出す*前*にマクロは定義したりスコープに導入しなければなりませんが、
一方で関数はどこにでも定義でき、どこでも呼び出せます。

<!--
### Declarative Macros with `macro_rules!` for General Metaprogramming
-->

### 一般的なメタプログラミングのために`macro_rules!`で宣言的なマクロ

<!--
The most widely used form of macros in Rust is *declarative macros*. These are
also sometimes referred to as “macros by example,” “`macro_rules!` macros,” or
just plain “macros.” At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that are
associated with particular code: in this situation, the value is the literal
Rust source code passed to the macro; the patterns are compared with the
structure of that source code; and the code associated with each pattern, when
matched, replaces the code passed to the macro. This all happens during
compilation.
-->

Rustにおいて、最もよく使用される形態のマクロは、*宣言的マクロ*です。これらは時として、
*例によるマクロ*、*`macro_rules!`マクロ*、あるいはただ単に*マクロ*とも称されます。
核となるのは、宣言的マクロは、Rustの`match`式に似た何かを書けるということです。第6章で議論したように、
`match`式は、式を取り、式の結果の値をパターンと比較し、それからマッチしたパターンに紐づいたコードを実行する制御構造です。
マクロも、あるコードと紐付けられたパターンと値を比較します。ここで、値とは
マクロに渡されたリテラルのRustのソースコードそのもののこと。パターンがそのソースコードの構造と比較されます。
各パターンに紐づいたコードは、それがマッチしたときに、マクロに渡されたコードを置き換えます。これは全て、コンパイル時に起きます。

<!--
To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by looking at how the `vec!` macro is defined. Chapter 8
covered how we can use the `vec!` macro to create a new vector with particular
values. For example, the following macro creates a new vector containing three
integers:
-->

マクロを定義するには、`macro_rules!`構文を使用します。`vec!`マクロが定義されている方法を見て、
`macro_rules!`を使用する方法を探究しましょう。`vec!`マクロを使用して特定の値で新しいベクタを生成する方法は、
第8章で講義しました。例えば、以下のマクロは、3つの整数を持つ新しいベクタを生成します:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

<!--
We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices. We wouldn’t be able to use a function to do the same
because we wouldn’t know the number or type of values up front.
-->
また、`vec!`マクロを使用して2整数のベクタや、5つの文字列スライスのベクタなども生成できます。
同じことを関数を使って行うことはできません。予め、値の数や型がわかっていないからです。

<!--
Listing 19-28 shows a slightly simplified definition of the `vec!` macro.
-->
リスト19-28で<ruby>些<rp>(</rp><rt>いささ</rt><rp>)</rp></ruby>か簡略化された`vec!`マクロの定義を見かけましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-28: A simplified version of the `vec!` macro
definition</span>
-->

<span class="caption">リスト19-28: `vec!`マクロ定義の簡略化されたバージョン</span>

<!--
> Note: The actual definition of the `vec!` macro in the standard library
> includes code to preallocate the correct amount of memory up front. That code
> is an optimization that we don’t include here to make the example simpler.
-->

> 標準ライブラリの`vec!`マクロの実際の定義は、予め正確なメモリ量を確保するコードを含みます。
> その最適化コードは、ここでは簡略化のために含みません。

<!--
The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which the macro is defined is brought into
scope. Without this annotation, the macro can’t be brought into scope.
-->

`#[macro_export]`注釈は、マクロを定義しているクレートがスコープに持ち込まれたなら、無条件でこのマクロが利用可能になるべきということを示しています。
この注釈がなければ、このマクロはスコープに導入されることができません。

<!--
We then start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark. The name, in this case
`vec`, is followed by curly brackets denoting the body of the macro definition.
-->

それから、`macro_rules!`でマクロ定義と定義しているマクロの名前をビックリマーク*なしで*始めています。
名前はこの場合`vec`であり、マクロ定義の本体を意味する波括弧が続いています。

<!--
The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there is only one valid way to match; any
other pattern will result in an error. More complex macros will have more than
one arm.
-->

`vec!`本体の構造は、`match`式の構造に類似しています。ここではパターン`( $( $x:expr ),* )`の1つのアーム、
`=>`とこのパターンに紐づくコードのブロックが続きます。パターンが合致すれば、紐づいたコードのブロックが発されます。
これがこのマクロの唯一のパターンであることを踏まえると、合致する合法的な方法は一つしかありません;
それ以外は、全部エラーになるでしょう。より複雑なマクロには、2つ以上のアームがあるでしょう。

<!--
Valid pattern syntax in macro definitions is different than the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pattern pieces in
Listing 19-28 mean; for the full macro pattern syntax, see [the reference].
-->

マクロ定義で合法なパターン記法は、第18章で講義したパターン記法とは異なります。というのも、
マクロのパターンは値ではなく、Rustコードの構造に対してマッチされるからです。リスト19-28のパターンの部品がどんな意味か見ていきましょう;
マクロパターン記法全ては[参考文献]をご覧ください。

<!--
[the reference]: ../reference/macros-by-example.html
-->

[参考文献]: https://doc.rust-lang.org/reference/macros.html

<!--
First, a set of parentheses encompasses the whole pattern. A dollar sign (`$`)
is next, followed by a set of parentheses that captures values that match the
pattern within the parentheses for use in the replacement code. Within `$()` is
`$x:expr`, which matches any Rust expression and gives the expression the name
`$x`.
-->

まず、1組のカッコがパターン全体を囲んでいます。次にドル記号(`$`)、そして1組のカッコが続き、
このかっこは、置き換えるコードで使用するためにかっこ内でパターンにマッチする値をキャプチャします。
`$()`の内部には、`$x:expr`があり、これは任意のRust式にマッチし、その式に`$x`という名前を与えます。

<!--
The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code in `$()`. The `*`
specifies that the pattern matches zero or more of whatever precedes the `*`.
-->

`$()`に続くカンマは、`$()`にキャプチャされるコードにマッチするコードの後に、区別を意味するリテラルのカンマ文字が現れるという選択肢もあることを示唆しています。
`*`は、パターンが`*`の前にあるもの0個以上にマッチすることを指定しています。

<!--
When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.
-->

このマクロを`vec![1, 2, 3];`と呼び出すと、`$x`パターンは、3つの式`1`、`2`、`3`で3回マッチします。

<!--
Now let’s look at the pattern in the body of the code associated with this arm:
`temp_vec.push()` within `$()*` is generated for each part that matches `$()`
in the pattern zero or more times depending on how many times the pattern
matches. The `$x` is replaced with each expression matched. When we call this
macro with `vec![1, 2, 3];`, the code generated that replaces this macro call
will be the following:
-->

さて、このアームに紐づくコードの本体のパターンに目を向けましょう: `$()*`部分内部の`temp_vec.push()`コードは、
パターンがマッチした回数に応じて0回以上パターン内で`$()`にマッチする箇所ごとに生成されます。
`$x`はマッチした式それぞれに置き換えられます。このマクロを`vec![1, 2, 3];`と呼び出すと、
このマクロ呼び出しを置き換え、生成されるコードは以下のようになるでしょう:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

<!--
We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.
-->

任意の型のあらゆる数の引数を取り、指定した要素を含むベクタを生成するコードを生成できるマクロを定義しました。

<!--
There are some strange edge cases with `macro_rules!`. In the future, Rust will
have a second kind of declarative macro that will work in a similar fashion but
fix some of these edge cases. After that update, `macro_rules!` will be
effectively deprecated. With this in mind, as well as the fact that most Rust
programmers will *use* macros more than *write* macros, we won’t discuss
`macro_rules!` any further. To learn more about how to write macros, consult
the online documentation or other resources, such as [“The Little Book of Rust
Macros”][tlborm].
-->
`macro_rules!`には、いくつかの奇妙なコーナーケースがあります。
将来、Rustには別種の宣言的マクロが登場する予定です。これは、同じように働くけれども、それらのコーナーケースのうちいくらかを修正します。
そのアップデート以降、`macro_rules!`は事実上非推奨 (deprecated) となる予定です。
この事実と、ほとんどのRustプログラマーはマクロを*書く*よりも*使う*ことが多いということを考えて、`macro_rules!`についてはこれ以上語らないことにします。
もしマクロの書き方についてもっと知りたければ、オンラインのドキュメントや、[“The Little Book of Rust Macros”][tlborm]のようなその他のリソースを参照してください。

[tlborm]: https://veykril.github.io/tlborm/

<!--
### Procedural Macros for Generating Code from Attributes
-->
### 属性からコードを生成する手続き的マクロ

<!--
The second form of macros is *procedural macros*, which act more like functions
(and are a type of procedure). Procedural macros accept some code as an input,
operate on that code, and produce some code as an output rather than matching
against patterns and replacing the code with other code as declarative macros
do.
-->
2つ目のマクロの形は、*手続き的マクロ*と呼ばれ、より関数のように働きます（そして一種の手続きです）。
宣言的マクロがパターンマッチングを行い、マッチしたコードを他のコードで置き換えていたのとは違い、
手続き的マクロは、コードを入力として受け取り、そのコードに対して作用し、出力としてコードを生成します。

<!--
The three kinds of procedural macros (custom derive, attribute-like, and
function-like) all work in a similar fashion.
-->
3種の手続き的マクロ (カスタムのderiveマクロ, 属性風マクロ、関数風マクロ)はみな同じような挙動をします。


<!--
When creating procedural macros, the definitions must reside in their own crate
with a special crate type. This is for complex technical reasons that we hope
to eliminate in the future. Using procedural macros looks like the code in
Listing 19-29, where `some_attribute` is a placeholder for using a specific
macro.
-->
手続き的マクロを作る際は、その定義はそれ専用の特殊なクレート内に置かれる必要があります。
これは複雑な技術的理由によるものであり、将来的には解消したいです。
手続き的マクロを使うとListing 19-29のコードのようになります。`some_attribute`がそのマクロを使うためのプレースホールダーです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<!--
<span class="caption">Listing 19-29: An example of using a procedural
macro</span>
-->
<span class="caption">Listing 19-29: 手続き的マクロの使用例</span>

<!--
The function that defines a procedural macro takes a `TokenStream` as an input
and produces a `TokenStream` as an output. The `TokenStream` type is defined by
the `proc_macro` crate that is included with Rust and represents a sequence of
tokens. This is the core of the macro: the source code that the macro is
operating on makes up the input `TokenStream`, and the code the macro produces
is the output `TokenStream`. The function also has an attribute attached to it
that specifies which kind of procedural macro we’re creating. We can have
multiple kinds of procedural macros in the same crate.
-->

手続き的マクロを定義する関数は`TokenStream`を入力として受け取り、`TokenStream`を出力として生成します。
`TokenStream`型はRustに内蔵されている`proc_macro`クレートで定義されており、トークンの列を表します。
ここがマクロの一番重要なところなのですが、マクロが作用するソースコードは、入力の`TokenStream`へと変換され、マクロが生成するコードが出力の`TokenStream`なのです。
この関数には属性もつけられていますが、これはどの種類の手続き的マクロを作っているのかを指定します。
同じクレート内に複数の種類の手続き的マクロを持つことも可能です。

<!--
Let’s look at the different kinds of procedural macros. We’ll start with a
custom derive macro and then explain the small dissimilarities that make the
other forms different.
-->
様々な種類の手続き的マクロを見てみましょう。カスタムのderiveマクロから始めて、そのあと他の種類との小さな相違点を説明します。

<!--
### How to Write a Custom `derive` Macro
-->
### カスタムの`derive` マクロの書き方

<!--
Let’s create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making our crate users implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined. In other words, we’ll write a crate that enables another
programmer to write code like Listing 19-30 using our crate.
-->

`hello_macro`という名前のクレートを作成してみましょう。
このクレートは、`hello_macro`という関連関数が1つある`HelloMacro`というトレイトを定義します。
クレートの使用者に使用者の型に`HelloMacro`トレイトを実装することを強制するのではなく、
使用者が型を`#[derive(HelloMacro)]`で注釈して`hello_macro`関数の既定の実装を得られるように、
手続き的マクロを提供します。既定の実装は、`Hello, Macro! My name is TypeName!`(`訳注`: こんにちは、マクロ！僕の名前はTypeNameだよ！)と出力し、
ここで`TypeName`はこのトレイトが定義されている型の名前です。言い換えると、他のプログラマに我々のクレートを使用して、
リスト19-30のようなコードを書けるようにするクレートを記述します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-30/src/main.rs}}
```

<!--
<span class="caption">Listing 19-30: The code a user of our crate will be able
to write when using our procedural macro</span>
-->

<span class="caption">リスト19-30: 我々の手続き的マクロを使用した時にクレートの使用者が書けるようになるコード</span>

<!--
This code will print `Hello, Macro! My name is Pancakes!` when we’re done. The
first step is to make a new library crate, like this:
-->
このコードは完成したら、`Hello, Macro! My name is Pancakes!`(`Pancakes`: ホットケーキ)と出力します。最初の手順は、
新しいライブラリクレートを作成することです。このように:

```console
$ cargo new hello_macro --lib
```

<!--
Next, we’ll define the `HelloMacro` trait and its associated function:
-->

次に`HelloMacro`トレイトと関連関数を定義します:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```

<!--
We have a trait and its function. At this point, our crate user could implement
the trait to achieve the desired functionality, like so:
-->

トレイトと関数があります。この時点でクレートの使用者は、以下のように、
このトレイトを実装して所望の機能を達成できるでしょう。

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```

<!--
However, they would need to write the implementation block for each type they
wanted to use with `hello_macro`; we want to spare them from having to do this
work.
-->

しかしながら、使用者は、`hello_macro`を使用したい型それぞれに実装ブロックを記述する必要があります;
この作業をしなくても済むようにしたいです。

<!--
Additionally, we can’t yet provide the `hello_macro` function with default
implementation that will print the name of the type the trait is implemented
on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s
name at runtime. We need a macro to generate code at compile time.
-->

さらに、まだ`hello_macro`関数にトレイトが実装されている型の名前を出力する既定の実装を提供することはできません:
Rustにはリフレクションの能力がないので、型の名前を実行時に検索することができないのです。
コンパイル時にコード生成するマクロが必要です。

> 注釈: リフレクションとは、実行時に型名や関数の中身などを取得する機能のことです。
> 言語によって提供されていたりいなかったりしますが、実行時にメタデータがないと取得できないので、
> RustやC++のようなアセンブリコードに翻訳され、パフォーマンスを要求される高級言語では、提供されないのが一般的と思われます。

<!--
The next step is to define the procedural macro. At the time of this writing,
procedural macros need to be in their own crate. Eventually, this restriction
might be lifted. The convention for structuring crates and macro crates is as
follows: for a crate named `foo`, a custom derive procedural macro crate is
called `foo_derive`. Let’s start a new crate called `hello_macro_derive` inside
our `hello_macro` project:
-->

次の手順は、手続き的マクロを定義することです。これを執筆している時点では、手続き的マクロは、
独自のクレートに存在する必要があります。最終的には、この制限は持ち上げられる可能性があります。
クレートとマクロクレートを構成する慣習は以下の通りです: `foo`というクレートに対して、
カスタムのderive手続き的マクロクレートは`foo_derive`と呼ばれます。`hello_macro`プロジェクト内に、
`hello_macro_derive`と呼ばれる新しいクレートを開始しましょう:

```console
$ cargo new hello_macro_derive --lib
```

<!--
Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, and programmers using these crates will need to add
both as dependencies and bring them both into scope. We could instead have the
`hello_macro` crate use `hello_macro_derive` as a dependency and re-export the
procedural macro code. However, the way we’ve structured the project makes it
possible for programmers to use `hello_macro` even if they don’t want the
`derive` functionality.
-->

2つのクレートは緊密に関係しているので、`hello_macro`クレートのディレクトリ内に手続き的マクロクレートを作成しています。
`hello_macro`のトレイト定義を変更したら、`hello_macro_derive`の手続き的マクロの実装も変更しなければならないでしょう。
2つのクレートは個別に公開される必要があり、これらのクレートを使用するプログラマは、
両方を依存に追加し、スコープに導入する必要があるでしょう。`hello_macro`クレートに依存として、
`hello_macro_derive`を使用させ、手続き的マクロのコードを再エクスポートすることもできるかもしれませんが、
このようなプロジェクトの構造にすることで、プログラマが`derive`機能を使用したくなくても、`hello_macro`を使用することが可能になります。

<!--
We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We’ll also need functionality from the `syn` and `quote` crates, as you’ll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:
-->

`hello_macro_derive`クレートを手続き的マクロクレートとして宣言する必要があります。
また、すぐにわかるように、`syn`と`quote`クレートの機能も必要になるので、依存として追加する必要があります。
以下を`hello_macro_derive`の*Cargo.toml*ファイルに追加してください:

<!--
<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>
-->

<span class="filename">ファイル名: hello_macro_derive/Cargo.toml</span>

```toml
{{#include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:7:12}}
```

<!--
To start defining the procedural macro, place the code in Listing 19-31 into
your *src/lib.rs* file for the `hello_macro_derive` crate. Note that this code
won’t compile until we add a definition for the `impl_hello_macro` function.
-->

手続き的マクロの定義を開始するために、`hello_macro_derive`クレートの*src/lib.rs*ファイルにリスト19-31のコードを配置してください。
`impl_hello_macro`関数の定義を追加するまでこのコードはコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->
<span class="filename">ファイル名: hello_macro_derive/src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-31: Code that most procedural macro crates
will require in order to process Rust code</span>
-->

<span class="caption">リスト19-31: Rustコードを処理するためにほとんどの手続き的マクロクレートに必要になるコード</span>

<!--
Notice that we’ve split the code into the `hello_macro_derive` function, which
is responsible for parsing the `TokenStream`, and the `impl_hello_macro`
function, which is responsible for transforming the syntax tree: this makes
writing a procedural macro more convenient. The code in the outer function
(`hello_macro_derive` in this case) will be the same for almost every
procedural macro crate you see or create. The code you specify in the body of
the inner function (`impl_hello_macro` in this case) will be different
depending on your procedural macro’s purpose.
-->

`TokenStream`をパースする役割を持つ`hello_macro_derive`関数と、構文木を変換する役割を持つ`impl_hello_macro`関数にコードを分割したことに注目してください：これにより手続き的マクロを書くのがより簡単になります。
外側の関数（今回だと`hello_macro_derive`）のコードは、あなたが見かけたり作ったりするであろうほとんどすべての手続き的マクロのクレートで同じです。
内側の関数（今回だと`impl_hello_macro`）の内部に書き込まれるコードは、手続き的マクロの目的によって異なってくるでしょう。

<!--
We’ve introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn’t need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate is the compiler’s API that
allows us to read and manipulate Rust code from our code.
-->

3つの新しいクレートを導入しました: `proc_macro`、[`syn`]、[`quote`]です。`proc_macro`クレートは、
Rustに付随してくるので、*Cargo.toml*の依存に追加する必要はありませんでした。`proc_macro`クレートはコンパイラのAPIで、私達のコードからRustのコードを読んだり操作したりすることを可能にします。

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

<!--
The `syn` crate parses Rust code from a string into a data structure that we
can perform operations on. The `quote` crate turns `syn` data structures back
into Rust code. These crates make it much simpler to parse any sort of Rust
code we might want to handle: writing a full parser for Rust code is no simple
task.
-->
`syn`クレートは、文字列からRustコードを構文解析し、
処理を行えるデータ構造にします。`quote`クレートは、`syn`データ構造を取り、Rustコードに変換し直します。
これらのクレートにより、扱いたい可能性のあるあらゆる種類のRustコードを構文解析するのがはるかに単純になります:
Rustコードの完全なパーサを書くのは、単純な作業ではないのです。

<!--
The `hello_macro_derive` function will be called when a user of our library
specifies `#[derive(HelloMacro)]` on a type. This is possible because we’ve
annotated the `hello_macro_derive` function here with `proc_macro_derive` and
specified the name, `HelloMacro`, which matches our trait name; this is the
convention most procedural macros follow.
-->
`hello_macro_derive`関数は、ライブラリの使用者が型に`#[derive(HelloMacro)]`を指定した時に呼び出されます。
それが可能な理由は、ここで`hello_macro_derive`関数を`proc_macro_derive`で注釈し、トレイト名に一致する`HelloMacro`を指定したからです;
これは、ほとんどの手続き的マクロが倣う慣習です。

<!--
The `hello_macro_derive` function first converts the `input` from a
`TokenStream` to a data structure that we can then interpret and perform
operations on. This is where `syn` comes into play. The `parse` function in
`syn` takes a `TokenStream` and returns a `DeriveInput` struct representing the
parsed Rust code. Listing 19-32 shows the relevant parts of the `DeriveInput`
struct we get from parsing the `struct Pancakes;` string:
-->

この関数はまず、`TokenStream`からの`input`をデータ構造に変換し、解釈したり操作したりできるようにします。
ここで`syn`が登場します。
`syn`の`parse`関数は`TokenStream`を受け取り、パースされたRustのコードを表現する`DeriveInput`構造体を返します。
Listing 19-32は`struct Pancakes;`という文字列をパースすることで得られる`DeriveInput`構造体の関係ある部分を表しています。

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<!--
<span class="caption">Listing 19-32: The `DeriveInput` instance we get when
parsing the code that has the macro’s attribute in Listing 19-30</span>
-->
<span class="caption">Listing 19-32: このマクロを使った属性を持つListing 19-30のコードをパースしたときに得られる`DeriveInput`インスタンス</span>

<!--
The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
documentation for `DeriveInput`][syn-docs] for more information.
-->

[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html

この構造体のフィールドは、構文解析したRustコードが`Pancakes`という`ident`(識別子、つまり名前)のユニット構造体であることを示しています。
この構造体にはRustコードのあらゆる部分を記述するフィールドがもっと多くあります;
[`DeriveInput`の`syn`ドキュメンテーション][syn-docs]で詳細を確認してください。


<!--
Soon we’ll define the `impl_hello_macro` function, which is where we’ll build
the new Rust code we want to include. But before we do, note that the output
for our derive macro is also a `TokenStream`. The returned `TokenStream` is
added to the code that our crate users write, so when they compile their crate,
they’ll get the extra functionality that we provide in the modified
`TokenStream`.
-->

まもなく`impl_hello_macro`関数を定義し、そこにインクルードしたい新しいRustコードを構築します。
でもその前に、私達のderiveマクロのための出力もまた`TokenStream`であることに注目してください。
返された`TokenStream`をクレートの使用者が書いたコードに追加しているので、クレートをコンパイルすると、
我々が修正した`TokenStream`で提供している追加の機能を得られます。

<!--
You might have noticed that we’re calling `unwrap` to cause the
`hello_macro_derive` function to panic if the call to the `syn::parse` function
fails here. It’s necessary for our procedural macro to panic on errors because
`proc_macro_derive` functions must return `TokenStream` rather than `Result` to
conform to the procedural macro API. We’ve simplified this example by using
`unwrap`; in production code, you should provide more specific error messages
about what went wrong by using `panic!` or `expect`.
-->

ここで、`unwrap`を呼び出すことで、`syn::parse`関数が失敗したときに`hello_macro_derive`関数をパニックさせていることにお気付きかもしれません。
エラー時にパニックするのは、手続き的マクロコードでは必要なことです。何故なら、
`proc_macro_derive`関数は、手続き的マクロのAPIに従うために、`Result`ではなく
`TokenStream`を返さなければならないからです。この例については、`unwrap`を使用して簡略化することを選択しました;
プロダクションコードでは、`panic!`か`expect`を使用して何が間違っていたのかより具体的なエラーメッセージを提供すべきです。

<!--
Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `DeriveInput` instance, let’s generate the code that implements the
`HelloMacro` trait on the annotated type, as shown in Listing 19-33.
-->

今や、`TokenStream`からの注釈されたRustコードを`DeriveInput`インスタンスに変換するコードができたので、
Listing 19-33のように、注釈された型に`HelloMacro`トレイトを実装するコードを生成しましょう:

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->

<span class="filename">ファイル名: hello_macro_derive/src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 19-33: Implementing the `HelloMacro` trait using
the parsed Rust code</span>
-->
<span class="caption">Listing 19-33: パースされたRustコードを用いて`HelloMacro`トレイトを実装する</span>

<!--
We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. The struct in Listing 19-32 shows that when
we run the `impl_hello_macro` function on the code in Listing 19-30, the
`ident` we get will have the `ident` field with a value of `"Pancakes"`. Thus,
the `name` variable in Listing 19-33 will contain an `Ident` struct instance
that, when printed, will be the string `"Pancakes"`, the name of the struct in
Listing 19-30.
-->

`ast.ident`を使って、注釈された型の名前(識別子)を含む`Ident`構造体インスタンスを得ています。
Listing 19-32の構造体を見ると、`impl_hello_macro`関数をListing 19-30のコードに実行したときに私達の得る`ident`は、フィールド`ident`の値として`"Pancakes"`を持つだろうとわかります。
従って、Listing 19-33における変数`name`は構造体`Ident`のインスタンスをもちます。このインスタンスは、printされた時は文字列`"Pancakes"`、即ちListing 19-30の構造体の名前となります。

<!--
The `quote!` macro lets us define the Rust code that we want to return. The
compiler expects something different to the direct result of the `quote!`
macro’s execution, so we need to convert it to a `TokenStream`. We do this by
calling the `into` method, which consumes this intermediate representation and
returns a value of the required `TokenStream` type.
-->

`quote!`マクロを使うことで、私達が返したいRustコードを定義することができます。
ただ、コンパイラが期待しているものは`quote!`マクロの実行結果とはちょっと違うものです。なので、`TokenStream`に変換してやる必要があります。
マクロの出力する直接表現を受け取り、必要とされている`TokenStream`型の値を返す`into`メソッドを呼ぶことでこれを行います。

<!--
The `quote!` macro also provides some very cool templating mechanics: we can
enter `#name`, and `quote!` will replace it with the value in the variable
`name`. You can even do some repetition similar to the way regular macros work.
Check out [the `quote` crate’s docs][quote-docs] for a thorough introduction.
-->

このマクロはまた、非常にかっこいいテンプレート機構も提供してくれます; `#name`と書くと、`quote!`は
それを`name`という変数の値と置き換えます。普通のマクロが動作するのと似た繰り返しさえ行えます。
本格的に入門したいなら、[`quote`クレートのdoc][quote-docs]をご確認ください。

[quote-docs]: https://docs.rs/quote

<!--
We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.
-->

手続き的マクロには使用者が注釈した型に対して`HelloMacro`トレイトの実装を生成してほしいですが、
これは`#name`を使用することで得られます。トレイトの実装には1つの関数`hello_macro`があり、
この本体に提供したい機能が含まれています: `Hello, Macro! My name is`、そして、注釈した型の名前を出力する機能です。

<!--
The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different than `format!` or
`println!`, macros which evaluate the expression and then turn the result into
a `String`. There is a possibility that the `#name` input might be an
expression to print literally, so we use `stringify!`. Using `stringify!` also
saves an allocation by converting `#name` to a string literal at compile time.
-->

ここで使用した`stringify!`マクロは、言語に組み込まれています。`1 + 2`などのようなRustの式を取り、
コンパイル時に`"1 + 2"`のような文字列リテラルにその式を変換します。
これは、`format!`や`println!`のような、式を評価し、そしてその結果を`String`に変換するマクロとは異なります。
`#name`入力が文字通り出力されるべき式という可能性もあるので、`stringify!`を使用しています。
`stringify!`を使用すると、コンパイル時に`#name`を文字列リテラルに変換することで、メモリ確保しなくても済みます。

<!--
At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook up these crates to the code in Listing
19-30 to see the procedural macro in action! Create a new binary project in
your *projects* directory using `cargo new pancakes`. We need to add
`hello_macro` and `hello_macro_derive` as dependencies in the `pancakes`
crate’s *Cargo.toml*. If you’re publishing your versions of `hello_macro` and
`hello_macro_derive` to [crates.io](https://crates.io/), they would be regular
dependencies; if not, you can specify them as `path` dependencies as follows:
-->

この時点で、`cargo build`は`hello_macro`と`hello_macro_derive`の両方で成功するはずです。
これらのクレートをリスト19-30のコードにフックして、手続き的マクロが動くところを確認しましょう！
`cargo new pancakes`であなたの*プロジェクトの*ディレクトリ（訳注：これまでに作った2つのクレート内ではないということ）に新しいバイナリプロジェクトを作成してください。
`hello_macro`と`hello_macro_derive`を依存として`pancakes`クレートの*Cargo.toml*に追加する必要があります。
自分のバージョンの`hello_macro`と`hello_macro_derive`を[crates.io](https://crates.io/) に公開しているなら、
普通の依存になるでしょう; そうでなければ、以下のように`path`依存として指定すればよいです:

```toml
{{#include ../listings/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

<!--
Put the code in Listing 19-30 into *src/main.rs*, and run `cargo run`: it
should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the
trait implementation.
-->

リスト19-30のコードを*src/main.rs*に配置し、`cargo run`を実行してください: `Hello, Macro! My name is Pancakes`と出力するはずです。
手続き的マクロの`HelloMacro`トレイトの実装は、`pancakes`クレートが実装する必要なく、包含されました;
`#[derive(HelloMacro)]`がトレイトの実装を追加したのです。

<!--
Next, let’s explore how the other kinds of procedural macros differ from custom
derive macros.
-->
続いて、他の種類の手続き的マクロがカスタムのderiveマクロとどのように異なっているか見てみましょう。

<!--
### Attribute-like macros
-->
### 属性風マクロ

<!--
Attribute-like macros are similar to custom derive macros, but instead of
generating code for the `derive` attribute, they allow you to create new
attributes. They’re also more flexible: `derive` only works for structs and
enums; attributes can be applied to other items as well, such as functions.
Here’s an example of using an attribute-like macro: say you have an attribute
named `route` that annotates functions when using a web application framework:
-->
属性風マクロはカスタムのderiveマクロと似ていますが、`derive`属性のためのコードを生成するのではなく、新しい属性を作ることができます。
また、属性風マクロはよりフレキシブルでもあります：`derive`は構造体とenumにしか使えませんでしたが、属性は関数のような他の要素に対しても使えるのです。
属性風マクロを使った例を以下に示しています：webアプリケーションフレームワークを使っているときに、`route`という関数につける属性名があるとします：

```rust,ignore
#[route(GET, "/")]
fn index() {
```

<!--
This `#[route]` attribute would be defined by the framework as a procedural
macro. The signature of the macro definition function would look like this:
-->
この`#[route]`属性はそのフレームワークによって手続き的マクロとして定義されたものなのでしょう。
マクロを定義する関数のシグネチャは以下のようになっているでしょう：

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

<!--
Here, we have two parameters of type `TokenStream`. The first is for the
contents of the attribute: the `GET, "/"` part. The second is the body of the
item the attribute is attached to: in this case, `fn index() {}` and the rest
of the function’s body.
-->
ここで、2つ`TokenStream`型の引数がありますね。
1つ目は属性の中身：`GET, "/"`に対応しており、2つ目は属性が付けられた要素の中身に対応しています。今回だと`fn index() {}`と関数の本体の残りですね。

<!--
Other than that, attribute-like macros work the same way as custom derive
macros: you create a crate with the `proc-macro` crate type and implement a
function that generates the code you want!
-->
それ以外において、属性風マクロはカスタムのderiveマクロと同じ動きをします：
クレートタイプとして`proc-macro`を使ってクレートを作り、あなたのほしいコードを生成してくれる関数を実装すればよいです！

<!--
### Function-like macros
-->
### 関数風マクロ

<!--
Function-like macros define macros that look like function calls. Similarly to
`macro_rules!` macros, they’re more flexible than functions; for example, they
can take an unknown number of arguments. However, `macro_rules!` macros can be
defined only using the match-like syntax we discussed in the section
[“Declarative Macros with `macro_rules!` for General Metaprogramming”][decl]
earlier. Function-like macros take a `TokenStream` parameter and their
definition manipulates that `TokenStream` using Rust code as the other two
types of procedural macros do. An example of a function-like macro is an `sql!`
macro that might be called like so:

[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
-->
関数風マクロは、関数呼び出しのように見えるマクロを定義します。
これらは、`macro_rules!`マクロのように、関数よりフレキシブルです。
たとえば、これらは任意の数の引数を取ることができます。
しかし、[一般的なメタプログラミングのために`macro_rules!`で宣言的なマクロ][decl]で話したように、`macro_rules!`マクロはmatch風の構文を使ってのみ定義できたのでした。
関数風マクロは引数として`TokenStream`をとり、その`TokenStream`を定義に従って操作します。操作には、他の2つの手続き的マクロと同じように、Rustコードが使われます。
例えば、`sql!`マクロという関数風マクロで、以下のように呼び出されるものを考えてみましょう：

[decl]: #%E4%B8%80%E8%88%AC%E7%9A%84%E3%81%AA%E3%83%A1%E3%82%BF%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E3%81%AE%E3%81%9F%E3%82%81%E3%81%ABmacro_rules%E3%81%A7%E5%AE%A3%E8%A8%80%E7%9A%84%E3%81%AA%E3%83%9E%E3%82%AF%E3%83%AD

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

<!--
This macro would parse the SQL statement inside it and check that it’s
syntactically correct, which is much more complex processing than a
`macro_rules!` macro can do. The `sql!` macro would be defined like this:
-->
このマクロは、中に入れられたSQL文をパースし、それが構文的に正しいことを確かめます。これは`macro_rules!`マクロが可能とするよりも遥かに複雑な処理です。
`sql!`マクロは以下のように定義することができるでしょう：

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

<!--
This definition is similar to the custom derive macro’s signature: we receive
the tokens that are inside the parentheses and return the code we wanted to
generate.
-->
この定義はカスタムのderiveマクロのシグネチャと似ています：カッコの中のトークンを受け取り、生成したいコードを返すのです。

<!--
## Summary
-->
## まとめ

<!--
Whew! Now you have some Rust features in your toolbox that you won’t use often,
but you’ll know they’re available in very particular circumstances. We’ve
introduced several complex topics so that when you encounter them in error
message suggestions or in other peoples’ code, you’ll be able to recognize
these concepts and syntax. Use this chapter as a reference to guide you to
solutions.
-->
ふう！
あなたがいま手にしたRustの機能はあまり頻繁に使うものではありませんが、非常に特殊な状況ではその存在を思い出すことになるでしょう。
たくさんの難しいトピックを紹介しましたが、これは、もしあなたがエラー時の推奨メッセージや他の人のコードでそれらに遭遇した時、その概念と文法を理解できるようになっていてほしいからです。
この章を、解決策にたどり着くためのリファレンスとして活用してください。

<!--
Next, we’ll put everything we’ve discussed throughout the book into practice
and do one more project!
-->
次は、この本で話してきたすべてのことを実際に使って、もう一つプロジェクトをやってみましょう！
