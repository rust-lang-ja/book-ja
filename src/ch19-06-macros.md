<!--
## Macros
-->

## マクロ

<!--
We’ve used macros like `println!` throughout this book but haven’t fully
explored what a macro is and how it works. This appendix explains macros as
follows:
-->

本全体で`println!`のようなマクロを使用してきましたが、マクロがなんなのかや、
どう動いているのかということは完全には探究していません。この付録は、マクロを以下のように説明します:

<!--
* What macros are and how they differ from functions
* How to define a declarative macro to do metaprogramming
* How to define a procedural macro to create custom `derive` traits
-->

* マクロとはなんなのかと関数とどう違うのか
* 宣言的なマクロを定義してメタプログラミングをする方法
* プロシージャルなマクロを定義して独自の`derive`トレイトを生成する方法

<!--
We’re covering the details of macros in an appendix because they’re still
evolving in Rust. Macros have changed and, in the near future, will change at a
quicker rate than the rest of the language and standard library since Rust 1.0,
so this section is more likely to become out-of-date than the rest of the book.
Due to Rust’s stability guarantees, the code shown here will continue to work
with future versions, but there may be additional capabilities or easier ways
to write macros that weren’t available at the time of this publication. Bear
that in mind when you try to implement anything from this appendix.
-->

マクロは今でも、Rustにおいては発展中なので、付録でマクロの詳細を講義します。マクロは変わってきましたし、
近い将来、Rust1.0からの言語の他の機能や標準ライブラリに比べて速いスピードで変化するので、
この節は、本の残りの部分よりも時代遅れになる可能性が高いです。Rustの安定性保証により、
ここで示したコードは、将来のバージョンでも動き続けますが、この本の出版時点では利用可能ではないマクロを書くための追加の能力や、
より簡単な方法があるかもしれません。この付録から何かを実装しようとする場合には、そのことを肝に銘じておいてください。

<!--
### The Difference Between Macros and Functions
-->

### マクロと関数の違い

<!--
Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In Appendix C, we discussed the `derive`
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
some additional powers that functions don’t have.
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
Another difference between macros and functions is that macro definitions
aren’t namespaced within modules like function definitions are. To prevent
unexpected name clashes when using external crates, you have to explicitly
bring the macros into the scope of your project at the same time as you bring
the external crate into scope, using the `#[macro_use]` annotation. The
following example would bring all the macros defined in the `serde` crate into
the scope of the current crate:
-->

マクロと関数の別の違いは、マクロ定義は、関数定義のようには、モジュール内で名前空間分けされないことです。
外部クレートを使用する際に予期しない名前衝突を回避するために、`#[macro_use]`注釈を使用して、
外部クレートをスコープに導入するのと同時に、自分のプロジェクトのスコープにマクロを明示的に導入しなければなりません。
以下の例は、`serde`クレートに定義されているマクロ全部を現在のクレートのスコープに導入するでしょう:

```rust,ignore
#[macro_use]
extern crate serde;
```

<!--
If `extern crate` was able to bring macros into scope by default without this
explicit annotation, you would be prevented from using two crates that happened
to define macros with the same name. In practice, this conflict doesn’t occur
often, but the more crates you use, the more likely it is.
-->

この明示的注釈なしに`extern crate`が既定でスコープにマクロを導入できたら、偶然同じ名前のマクロを定義している2つのクレートを使用できなくなるでしょう。
現実的には、この衝突はあまり起きませんが、使用するクレートが増えるほど、可能性は高まります。

<!--
There is one last important difference between macros and functions: you must
define or bring macros into scope *before* you call them in a file, whereas you
can define functions anywhere and call them anywhere.
-->

マクロと関数にはもう一つ、重要な違いがあります: ファイル内で呼び出す*前*にマクロはスコープに導入しなければなりませんが、
一方で関数はどこにでも定義でき、どこでも呼び出せます。

<!--
### Declarative Macros with `macro_rules!` for General Metaprogramming
-->

### 一般的なメタプログラミングのために`macro_rules!`で宣言的なマクロ

<!--
The most widely used form of macros in Rust are *declarative macros*. These are
also sometimes referred to as *macros by example*, *`macro_rules!` macros*, or
just plain *macros*. At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that have
code associated with them; in this situation, the value is the literal Rust
source code passed to the macro, the patterns are compared with the structure
of that source code, and the code associated with each pattern is the code that
replaces the code passed to the macro. This all happens during compilation.
-->

Rustにおいて、最もよく使用される形態のマクロは、*宣言的マクロ*です。これらは時として、
*例によるマクロ*、*`macro_rules!`マクロ*、あるいはただ単に*マクロ*とも称されます。
核となるのは、宣言的マクロは、Rustの`match`式に似た何かを書けるということです。第6章で議論したように、
`match`式は、式を取り、式の結果の値をパターンと比較し、それからマッチしたパターンに紐づいたコードを実行する制御構造です。
マクロも自身に紐づいたコードがあるパターンと値を比較します; この場面で値とは、
マクロに渡されたリテラルのRustのソースコードそのもの、パターンは、そのソースコードの構造と比較され、
各パターンに紐づいたコードは、マクロに渡されたコードを置き換えるコードです。これは全て、コンパイル時に起きます。  

<!--
To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by looking at how the `vec!` macro is defined. Chapter 8
covered how we can use the `vec!` macro to create a new vector with particular
values. For example, the following macro creates a new vector with three
integers inside:
-->

マクロを定義するには、`macro_rules!`構文を使用します。`vec!`マクロが定義されている方法を見て、
`macro_rules!`を使用する方法を探究しましょう。`vec!`マクロを使用して特定の値で新しいベクタを生成する方法は、
第8章で講義しました。例えば、以下のマクロは、3つの整数を中身にする新しいベクタを生成します:

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
Let’s look at a slightly simplified definition of the `vec!` macro in Listing
D-1.
-->

リストD-1で<ruby>些<rp>(</rp><rt>いささ</rt><rp>)</rp></ruby>か簡略化された`vec!`マクロの定義を見かけましょう。

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

<!--
<span class="caption">Listing D-1: A simplified version of the `vec!` macro
definition</span>
-->

<span class="caption">リストD-1: `vec!`マクロ定義の簡略化されたバージョン</span>

<!--
> Note: The actual definition of the `vec!` macro in the standard library
> includes code to preallocate the correct amount of memory up front. That code
> is an optimization that we don’t include here to make the example simpler.
-->

> 標準ライブラリの`vec!`マクロの実際の定義は、予め正確なメモリ量を確保するコードを含みます。
> そのコードは、ここでは簡略化のために含まない最適化です。

<!--
The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which we’re defining the macro is imported.
Without this annotation, even if someone depending on this crate uses the
`#[macro_use]` annotation, the macro wouldn’t be brought into scope.
-->

`#[macro_export]`注釈は、マクロを定義しているクレートがインポートされる度にこのマクロが利用可能になるべきということを示しています。
この注釈がなければ、このクレートに依存する誰かが`#[macro_use]`注釈を使用していても、
このマクロはスコープに導入されないでしょう。

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
other will be an error. More complex macros will have more than one arm.
-->

`vec!`本体の構造は、`match`式の構造に類似しています。ここではパターン`( $( $x:expr ),* )`の1つのアーム、
`=>`とこのパターンに紐づくコードのブロックが続きます。パターンが合致すれば、紐づいたコードのブロックが発されます。
これがこのマクロの唯一のパターンであることを踏まえると、合致する合法的な方法は一つしかありません;
それ以外は、全部エラーになるでしょう。より複雑なマクロには、2つ以上のアームがあるでしょう。

<!--
Valid pattern syntax in macro definitions is different than the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pieces of the pattern
in Listing D-1 mean; for the full macro pattern syntax, see [the reference].
-->

マクロ定義で合法なパターン記法は、第18章で講義したパターン記法とは異なります。というのも、
マクロのパターンは値ではなく、Rustコードの構造に対してマッチされるからです。リストD-1のパターンの部品がどんな意味か見ていきましょう;
マクロパターン記法全ては[参考文献]をご覧ください。

[参考文献]: https://doc.rust-lang.org/reference/macros.html

<!--
First, a set of parentheses encompasses the whole pattern. Next comes a dollar
sign (`$`) followed by a set of parentheses, which captures values that match
the pattern within the parentheses for use in the replacement code. Within
`$()` is `$x:expr`, which matches any Rust expression and gives the expression
the name `$x`.
-->

まず、1組のカッコがパターン全体を囲んでいます。次にドル記号(`$`)、そして1組のカッコが続き、
このかっこは、置き換えるコードで使用するためにかっこ内でパターンにマッチする値をキャプチャします。
`$()`の内部には、`$x:expr`があり、これは任意のRust式にマッチし、その式に`$x`という名前を与えます。

<!--
The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code captured in `$()`.
The `*` following the comma specifies that the pattern matches zero or more of
whatever precedes the `*`.
-->

`$()`に続くカンマは、`$()`にキャプチャされるコードにマッチするコードの後に、区別を意味するリテラルのカンマ文字が現れるという選択肢もあることを示唆しています。
カンマに続く`*`は、パターンが`*`の前にあるもの0個以上にマッチすることを指定しています。

<!--
When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.
-->

このマクロを`vec![1, 2, 3];`と呼び出すと、`$x`パターンは、3つの式`1`、`2`、`3`で3回マッチします。

<!--
Now let’s look at the pattern in the body of the code associated with this arm:
the `temp_vec.push()` code within the `$()*` part is generated for each part
that matches `$()` in the pattern, zero or more times depending on how many
times the pattern matches. The `$x` is replaced with each expression matched.
When we call this macro with `vec![1, 2, 3];`, the code generated that replaces
this macro call will be the following:
-->

さて、このアームに紐づくコードの本体のパターンに目を向けましょう: `$()*`部分内部の`temp_vec.push()`コードは、
パターンがマッチした回数に応じて0回以上パターン内で`$()`にマッチする箇所ごとに生成されます。
`$x`はマッチした式それぞれに置き換えられます。このマクロを`vec![1, 2, 3];`と呼び出すと、
このマクロ呼び出しを置き換え、生成されるコードは以下のようになるでしょう:

```rust,ignore
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

<!--
We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.
-->

任意の型のあらゆる数の引数を取り、指定した要素を含むベクタを生成するコードを生成できるマクロを定義しました。

<!--
Given that most Rust programmers will *use* macros more than *write* macros, we
won’t discuss `macro_rules!` any further. To learn more about how to write
macros, consult the online documentation or other resources, such as [“The
Little Book of Rust Macros”][tlborm].
-->

多くのRustプログラマは、マクロを*書く*よりも*使う*方が多いことを踏まえて、これ以上`macro_rules!`を議論しません。
マクロの書き方をもっと学ぶには、オンラインドキュメンテーションか他のリソース、
[“The Little Book of Rust Macros][tlborm](`訳注`: Rustのマクロの小さな本)などを調べてください。

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

<!--
### Procedural Macros for Custom `derive`
-->

### 独自の`derive`のためのプロシージャルマクロ

<!--
The second form of macros is called *procedural macros* because they’re more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output rather than matching against patterns and replacing the code with other
code as declarative macros do. At the time of this writing, you can only define
procedural macros to allow your traits to be implemented on a type by
specifying the trait name in a `derive` annotation.
-->

2番目の形態のマクロは、より関数(1種の手続きです)に似ているので、*プロシージャル・マクロ*(procedural macro; `訳注`:
手続きマクロ)と呼ばれます。プロシージャルマクロは、宣言的マクロのようにパターンにマッチさせ、
そのコードを他のコードと置き換えるのではなく、入力として何らかのRustコードを受け付け、そのコードを処理し、
出力として何らかのRustコードを生成します。これを執筆している時点では、`derive`注釈にトレイト名を指定することで、
型に自分のトレイトを実装できるプロシージャルマクロを定義できるだけです。

<!--
We’ll create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making our crate users implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined. In other words, we’ll write a crate that enables another
programmer to write code like Listing D-2 using our crate.
-->

`hello_macro`という関連関数が1つある`HelloMacro`というトレイトを定義する`hello_macro`というクレートを作成します。
クレートの使用者に使用者の型に`HelloMacro`トレイトを実装することを強制するのではなく、
使用者が型を`#[derive(HelloMacro)]`で注釈して`hello_macro`関数の既定の実装を得られるように、
プロシージャルマクロを提供します。既定の実装は、`Hello, Macro! My name is TypeName!`(`訳注`: こんにちは、マクロ！僕の名前はTypeNameだよ！)と出力し、
ここで`TypeName`はこのトレイトが定義されている型の名前です。言い換えると、他のプログラマに我々のクレートを使用して、
リストD-2のようなコードを書けるようにするクレートを記述します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

<!--
<span class="caption">Listing D-2: The code a user of our crate will be able to
write when using our procedural macro</span>
-->

<span class="caption">リストD-2: 我々のプロシージャルマクロを使用した時にクレートの使用者が書けるようになるコード</span>

<!--
This code will print `Hello, Macro! My name is Pancakes!` when we’re done. The
first step is to make a new library crate, like this:
-->

このコードは完成したら、`Hello, Macro! My name is Pancakes!`(`Pancakes`: ホットケーキ)と出力します。最初の手順は、
新しいライブラリクレートを作成することです。このように:

```text
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
pub trait HelloMacro {
    fn hello_macro();
}
```

<!--
We have a trait and its function. At this point, our crate user could implement
the trait to achieve the desired functionality, like so:
-->

トレイトと関数があります。この時点でクレートの使用者は、以下のように、
このトレイトを実装して所望の機能を達成できるでしょう。

```rust,ignore
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

<!--
However, they would need to write the implementation block for each type they
wanted to use with `hello_macro`; we want to spare them from having to do this
work.
-->

しかしながら、使用者は、`hello_macro`を使用したい型それぞれに実装ブロックを記述する必要があります;
この作業をしなくても済むようにしたいです。

<!--
Additionally, we can’t yet provide a default implementation for the
`hello_macro` function that will print the name of the type the trait is
implemented on: Rust doesn’t have reflection capabilities, so it can’t look up
the type’s name at runtime. We need a macro to generate code at compile time.
-->

さらに、まだトレイトが実装されている型の名前を出力する`hello_macro`関数に既定の実装を提供することはできません:
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

次の手順は、プロシージャルマクロを定義することです。これを執筆している時点では、プロシージャルマクロは、
独自のクレートに存在する必要があります。最終的には、この制限は持ち上げられる可能性があります。
クレートとマクロクレートを構成する慣習は以下の通りです: `foo`というクレートに対して、
独自のderiveプロシージャルマクロクレートは`foo_derive`と呼ばれます。`hello_macro`プロジェクト内に、
`hello_macro_derive`と呼ばれる新しいクレートを開始しましょう:

```text
$ cargo new hello_macro_derive --lib
```

<!--
Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, and programmers using these crates will need to add
both as dependencies and bring them both into scope. We could instead have the
`hello_macro` crate use `hello_macro_derive` as a dependency and reexport the
procedural macro code. But the way we’ve structured the project makes it
possible for programmers to use `hello_macro` even if they don’t want the
`derive` functionality.
-->

2つのクレートは緊密に関係しているので、`hello_macro`クレートのディレクトリ内にプロシージャルマクロクレートを作成しています。
`hello_macro`のトレイト定義を変更したら、`hello_macro_derive`のプロシージャルマクロの実装も変更しなければならないでしょう。
2つのクレートは個別に公開される必要があり、これらのクレートを使用するプログラマは、
両方を依存に追加し、スコープに導入する必要があるでしょう。代わりに、`hello_macro`クレートに依存として、
`hello_macro_derive`を使用させ、プロシージャルマクロのコードを再エクスポートすることもできるでしょう。
プロジェクトの構造によっては、プログラマが`derive`機能を使用したくなくても、`hello_macro`を使用することが可能になります。

<!--
We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We’ll also need functionality from the `syn` and `quote` crates, as you’ll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:
-->

`hello_macro_derive`クレートをプロシージャルマクロクレートとして宣言する必要があります。
また、すぐにわかるように、`syn`と`quote`クレートの機能も必要になるので、依存として追加する必要があります。
以下を`hello_macro_derive`の*Cargo.toml*ファイルに追加してください:

<!--
<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>
-->

<span class="filename">ファイル名: hello_macro_derive/Cargo.toml</span>

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

<!--
To start defining the procedural macro, place the code in Listing D-3 into your
*src/lib.rs* file for the `hello_macro_derive` crate. Note that this code won’t
compile until we add a definition for the `impl_hello_macro` function.
-->

プロシージャルマクロの定義を開始するために、`hello_macro_derive`クレートの*src/lib.rs*ファイルにリストD-3のコードを配置してください。
`impl_hello_macro`関数の定義を追加するまでこのコードはコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->

<span class="filename">ファイル名: hello_macro_derive/src/lib.rs</span>

```rust,ignore
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 型定義の文字列表現を構築する
    // Construct a string representation of the type definition
    let s = input.to_string();

    // 文字列表現を構文解析する
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // implを構築する
    // Build the impl
    let gen = impl_hello_macro(&ast);

    // 生成されたimplを返す
    // Return the generated impl
    gen.parse().unwrap()
}
```

<!--
<span class="caption">Listing D-3: Code that most procedural macro crates will
need to have for processing Rust code</span>
-->

<span class="caption">リストD-3: Rustコードを処理するためにほとんどのプロシージャルマクロクレートに必要になるコード</span>

<!--
Notice the way we’ve split the functions in D-3; this will be the same for
almost every procedural macro crate you see or create, because it makes writing
a procedural macro more convenient. What you choose to do in the place where
the `impl_hello_macro` function is called will be different depending on your
procedural macro’s purpose.
-->

D-3での関数の分け方に注目してください; これは、目撃あるいは作成するほとんどのプロシージャルマクロクレートで同じになるでしょう。
プロシージャルマクロを書くのが便利になるからです。`impl_hello_macro`関数が呼ばれる箇所で行うことを選ぶものは、
プロシージャルマクロの目的によって異なるでしょう。

<!--
We’ve introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn’t need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate allows us to convert Rust
code into a string containing that Rust code. The `syn` crate parses Rust code
from a string into a data structure that we can perform operations on. The
`quote` crate takes `syn` data structures and turns them back into Rust code.
These crates make it much simpler to parse any sort of Rust code we might want
to handle: writing a full parser for Rust code is no simple task.
-->

3つの新しいクレートを導入しました: `proc_macro`、[`syn`]、[`quote`]です。`proc_macro`クレートは、
Rustに付随してくるので、*Cargo.toml*の依存に追加する必要はありませんでした。`proc_macro`クレートにより、
RustコードをRustコードを含む文字列に変換できます。`syn`クレートは、文字列からRustコードを構文解析し、
処理を行えるデータ構造にします。`quote`クレートは、`syn`データ構造を取り、Rustコードに変換し直します。
これらのクレートにより、扱いたい可能性のあるあらゆる種類のRustコードを構文解析するのがはるかに単純になります:
Rustコードの完全なパーサを書くのは、単純な作業ではないのです。

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

<!--
The `hello_macro_derive` function will get called when a user of our library
specifies `#[derive(HelloMacro)]` on a type. The reason is that we’ve annotated
the `hello_macro_derive` function here with `proc_macro_derive` and specified
the name, `HelloMacro`, which matches our trait name; that’s the convention
most procedural macros follow.
-->

`hello_macro_derive`関数は、ライブラリの使用者が型に`#[derive(HelloMacro)]`を指定した時に呼び出されます。
その理由は、ここで`hello_macro_derive`関数を`proc_macro_derive`で注釈し、トレイト名に一致する`HelloMacro`を指定したからです;
これは、ほとんどのプロシージャルマクロが倣う慣習です。

<!--
This function first converts the `input` from a `TokenStream` to a `String` by
calling `to_string`. This `String` is a string representation of the Rust code
for which we are deriving `HelloMacro`. In the example in Listing D-2, `s` will
have the `String` value `struct Pancakes;` because that is the Rust code we
added the `#[derive(HelloMacro)]` annotation to.
-->

この関数はまず、`TokenStream`からの`input`を`to_string`を呼び出して`String`に変換します。
この`String`は、`HelloMacro`を導出しているRustコードの文字列表現になります。
リストD-2の例で、`s`は`struct Pancakes;`という`String`値になります。
それが`#[derive(HelloMacro)]`注釈を追加したRustコードだからです。

<!--
> Note: At the time of this writing, you can only convert a `TokenStream` to a
> string. A richer API will exist in the future.
-->

> 注釈: これを執筆している時点では、`TokenStream`は文字列にしか変換できません。
> 将来的にはよりリッチなAPIになるでしょう。

<!--
Now we need to parse the Rust code `String` into a data structure that we can
then interpret and perform operations on. This is where `syn` comes into play.
The `parse_derive_input` function in `syn` takes a `String` and returns a
`DeriveInput` struct representing the parsed Rust code. The following code
shows the relevant parts of the `DeriveInput` struct we get from parsing the
string `struct Pancakes;`:
-->

さて、Rustコードの`String`をそれから解釈して処理を実行できるデータ構造に構文解析する必要があります。
ここで`syn`が登場します。`syn`の`parse_derive_input`関数は、`String`を取り、
構文解析されたRustコードを表す`DeriveInput`構造体を返します。以下のコードは、
文字列`struct Pancakes;`を構文解析して得られる`DeriveInput`構造体の関係のある部分を表示しています:

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident(
        "Pancakes"
    ),
    body: Struct(
        Unit
    )
}
```

<!--
The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
documentation for `DeriveInput`][syn-docs] for more information.
-->

この構造体のフィールドは、構文解析したRustコードが`Pancakes`という`ident`(識別子、つまり名前)のユニット構造体であることを示しています。
この構造体にはRustコードのあらゆる部分を記述するフィールドがもっと多くあります;
[`DeriveInput`の`syn`ドキュメンテーション][syn-docs]で詳細を確認してください。

[syn-docs]: https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html

<!--
At this point, we haven’t defined the `impl_hello_macro` function, which is
where we’ll build the new Rust code we want to include. But before we do, note
that the last part of this `hello_macro_derive` function uses the `parse`
function from the `quote` crate to turn the output of the `impl_hello_macro`
function back into a `TokenStream`. The returned `TokenStream` is added to the
code that our crate users write, so when they compile their crate, they’ll get
extra functionality that we provide.
-->

この時点では、含みたい新しいRustコードを構築する`impl_hello_macro`関数を定義していません。
でもその前に、この`hello_macro_derive`関数の最後の部分で`quote`クレートの`parse`関数を使用して、
`impl_hello_macro`関数の出力を`TokenStream`に変換し直していることに注目してください。
返された`TokenStream`をクレートの使用者が書いたコードに追加しているので、クレートをコンパイルすると、
我々が提供している追加の機能を得られます。

<!--
You might have noticed that we’re calling `unwrap` to panic if the calls to the
`parse_derive_input` or `parse` functions fail here. Panicking on errors is
necessary in procedural macro code because `proc_macro_derive` functions must
return `TokenStream` rather than `Result` to conform to the procedural macro
API. We’ve chosen to simplify this example by using `unwrap`; in production
code, you should provide more specific error messages about what went wrong by
using `panic!` or `expect`.
-->

`parse_derive_input`か`parse`関数がここで失敗したら、`unwrap`を呼び出してパニックしていることにお気付きかもしれません。
エラー時にパニックするのは、プロシージャルマクロコードでは必要なことです。何故なら、
`proc_macro_derive`関数は、プロシージャルマクロAPIに従うように`Result`ではなく、
`TokenStream`を返さなければならないからです。`unwrap`を使用してこの例を簡略化することを選択しました;
プロダクションコードでは、`panic!`か`expect`を使用して何が間違っていたのかより具体的なエラーメッセージを提供すべきです。

<!--
Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `String` and a `DeriveInput` instance, let’s generate the code that
implements the `HelloMacro` trait on the annotated type:
-->

今や、`TokenStream`からの注釈されたRustコードを`String`と`DeriveInput`インスタンスに変換するコードができたので、
注釈された型に`HelloMacro`トレイトを実装するコードを生成しましょう:

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->

<span class="filename">ファイル名: hello_macro_derive/src/lib.rs</span>

```rust,ignore
fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    }
}
```

<!--
We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. The code in Listing D-2 specifies that the
`name` will be `Ident("Pancakes")`.
-->

`ast.ident`で注釈された型の名前(識別子)を含む`Ident`構造体インスタンスを得ています。
リストD-2のコードは、`name`が`Ident("Pancakes")`になることを指定しています。

<!--
The `quote!` macro lets us write the Rust code that we want to return and
convert it into `quote::Tokens`. This macro also provides some very cool
templating mechanics; we can write `#name`, and `quote!` will replace it with
the value in the variable named `name`. You can even do some repetition similar
to the way regular macros work. Check out [the `quote` crate’s
docs][quote-docs] for a thorough introduction.
-->

`quote!`マクロは、返却し`quote::Tokens`に変換したいRustコードを書かせてくれます。このマクロはまた、
非常にかっこいいテンプレート機構も提供してくれます; `#name`と書け、`quote!`は、
それを`name`という変数の値と置き換えます。普通のマクロが動作するのと似た繰り返しさえ行えます。
完全なイントロダクションは、[`quote`クレートのdoc][quote-docs]をご確認ください。

[quote-docs]: https://docs.rs/quote

<!--
We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.
-->

プロシージャルマクロに使用者が注釈した型に対して`HelloMacro`トレイトの実装を生成してほしく、
これは`#name`を使用することで得られます。トレイトの実装には1つの関数`hello_macro`があり、
この本体に提供したい機能が含まれています: `Hello, Macro! My name is`、そして、注釈した型の名前を出力する機能です。

<!--
The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different than `format!` or
`println!`, which evaluate the expression and then turn the result into a
`String`. There is a possibility that the `#name` input might be an expression
to print literally, so we use `stringify!`. Using `stringify!` also saves an
allocation by converting `#name` to a string literal at compile time.
-->

ここで使用した`stringify!`マクロは、言語に埋め込まれています。`1 + 2`などのようなRustの式を取り、
コンパイル時に`"1 + 2"`のような文字列リテラルにその式を変換します。これは、`format!`や`println!`とは異なります。
こちらは、式を評価し、そしてその結果を`String`に変換します。`#name`入力が文字通り出力される式という可能性もあるので、
`stringify!`を使用しています。`stringify!`を使用すると、コンパイル時に`#name`を文字列リテラルに変換することで、
メモリ確保しなくても済みます。

<!--
At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook up these crates to the code in Listing D-2
to see the procedural macro in action! Create a new binary project in your
*projects* directory using `cargo new --bin pancakes`. We need to add
`hello_macro` and `hello_macro_derive` as dependencies in the `pancakes`
crate’s *Cargo.toml*. If you’re publishing your versions of `hello_macro` and
`hello_macro_derive` to *https://crates.io/*, they would be regular
dependencies; if not, you can specify them as `path` dependencies as follows:
-->

この時点で、`cargo build`は`hello_macro`と`hello_macro_derive`の両方で成功するはずです。
これらのクレートをリストD-2のコードにフックして、プロシージャルマクロが動くところを確認しましょう！
`cargo new --bin pancakes`で*projects*ディレクトリに新しいバイナリプロジェクトを作成してください。
`hello_macro`と`hello_macro_derive`を依存として`pancakes`クレートの*Cargo.toml*に追加する必要があります。
自分のバージョンの`hello_macro`と`hello_macro_derive`を*https://crates.io/* に公開するつもりなら、
普通の依存になるでしょう; そうでなければ、以下のように`path`依存として指定できます:

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

<!--
Put the code from Listing D-2 into *src/main.rs*, and run `cargo run`: it
should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the
trait implementation.
-->

リストD-2のコードを*src/main.rs*に配置し、`cargo run`を実行してください: `Hello, Macro! My name is Pancakes`と出力するはずです。
プロシージャルマクロの`HelloMacro`トレイトの実装は、`pancakes`クレートが実装する必要なく、包含されました;
`#[derive(HelloMacro)]`がトレイトの実装を追加したのです。

<!--
### The Future of Macros
-->

### マクロの未来

<!--
In the future, Rust will expand declarative and procedural macros. Rust will
use a better declarative macro system with the `macro` keyword and will add
more types of procedural macros for more powerful tasks than just `derive`.
These systems are still under development at the time of this publication;
please consult the online Rust documentation for the latest information.
-->

将来的にRustは、宣言的マクロとプロシージャルマクロを拡張するでしょう。`macro`キーワードでより良い宣言的マクロシステムを使用し、
`derive`だけよりもよりパワフルな作業のより多くの種類のプロシージャルマクロを追加するでしょう。
この本の出版時点ではこれらのシステムはまだ開発中です; 最新の情報は、オンラインのRustドキュメンテーションをお調べください。
