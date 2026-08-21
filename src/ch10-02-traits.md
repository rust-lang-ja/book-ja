<!--
## Traits: Defining Shared Behavior
-->

## トレイト: 共通の振る舞いを定義する

<!--
A *trait* defines functionality a particular type has and can share with other
types. We can use traits to define shared behavior in an abstract way. We can
use *trait bounds* to specify that a generic type can be any type that has
certain behavior.
-->

*トレイト*は、特定の型に存在し、他の型と共有できる機能を定義します。
トレイトを使用すると、共通の振る舞いを抽象的に定義できます。*トレイト境界*を使用すると、
あるジェネリック型が、特定の振る舞いをもつあらゆる型になり得ることを指定できます。

<!--
> Note: Traits are similar to a feature often called *interfaces* in other
> languages, although with some differences.
-->

> 注釈: 違いはあるものの、トレイトは他の言語でよくインターフェイスと呼ばれる機能に類似しています。

<!--
### Defining a Trait
-->

### トレイトを定義する

<!--
A type’s behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.
-->

型の振る舞いは、その型に対して呼び出せるメソッドから構成されます。異なる型は、それらの型全てに対して同じメソッドを呼び出せるなら、
同じ振る舞いを共有することになります。トレイト定義は、メソッドシグニチャをあるグループにまとめ、なんらかの目的を達成するのに必要な一連の振る舞いを定義する手段です。

<!--
For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.
-->

例えば、いろんな種類や量のテキストを保持する複数の構造体があるとしましょう: 特定の場所から送られる新しいニュースを保持する`NewsArticle`と、
新規ツイートか、リツイートか、はたまた他のツイートへのリプライなのかを示すメタデータを伴う最大で280文字までの`Tweet`です。

<!--
We want to make a media aggregator library crate named `aggregator` that can
display summaries of data that might be stored in a `NewsArticle` or `Tweet`
instance. To do this, we need a summary from each type, and we’ll request
that summary by calling a `summarize` method on an instance. Listing 10-12
shows the definition of a public `Summary` trait that expresses this behavior.
-->

`NewsArticle` または `Tweet` インスタンスに保存されているデータのサマリーを表示できる`aggregator`という名前のメディア アグリゲータ ライブラリ クレートを作成します。
これをするには、各型のサマリーが必要で、インスタンスで `summarize` メソッドを呼び出してサマリーを要求することになるでしょう。
リスト10-12は、この振る舞いを表現する公開の`Summary`トレイトの定義を表示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<!--
<span class="caption">Listing 10-12: A `Summary` trait that consists of the
behavior provided by a `summarize` method</span>
-->

<span class="caption">リスト10-12: `summarize`メソッドで提供される振る舞いからなる`Summary`トレイト</span>

<!--
Here, we declare a trait using the `trait` keyword and then the trait’s name,
which is `Summary` in this case. We’ve also declared the trait as `pub` so that
crates depending on this crate can make use of this trait too, as we’ll see in
a few examples. Inside the curly brackets, we declare the method signatures
that describe the behaviors of the types that implement this trait, which in
this case is `fn summarize(&self) -> String`.
-->

ここでは、`trait`キーワード、それからトレイト名を使用してトレイトを宣言していて、その名前は今回の場合、
`Summary`です。
また、いくつかの例で見ていきますが、このクレートに依存するクレートがこのトレイトを利用できるように、トレイトを`pub`として宣言しています。
波括弧の中にこのトレイトを実装する型の振る舞いを記述するメソッドシグニチャを定義し、
今回の場合は、`fn summarize(&self) -> String`です。

<!--
After the method signature, instead of providing an implementation within curly
brackets, we use a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method. The compiler will enforce
that any type that has the `Summary` trait will have the method `summarize`
defined with this signature exactly.
-->

メソッドシグニチャの後に、波括弧内に実装を提供する代わりに、セミコロンを使用しています。
このトレイトを実装する型はそれぞれ、メソッドの本体に独自の振る舞いを提供しなければなりません。
コンパイラにより、`Summary`トレイトを保持するあらゆる型に、このシグニチャと全く同じメソッド`summarize`が定義されていることが
強制されます。

<!--
A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.
-->

トレイトには、本体に複数のメソッドを含むことができます: メソッドシグニチャは行ごとに並べられ、
各行はセミコロンで終わります。

<!--
### Implementing a Trait on a Type
-->

### トレイトを型に実装する

<!--
Now that we’ve defined the desired signatures of the `Summary` trait’s methods,
we can implement it on the types in our media aggregator. Listing 10-13 shows
an implementation of the `Summary` trait on the `NewsArticle` struct that uses
the headline, the author, and the location to create the return value of
`summarize`. For the `Tweet` struct, we define `summarize` as the username
followed by the entire text of the tweet, assuming that tweet content is
already limited to 280 characters.
-->

これで `Summary` トレイトのメソッドのシグネチャを希望通りに定義できたので、メディア アグリゲータ内の型に対してこれを実装できます。
リスト10-13は、 `Summary` トレイトを `NewsArticle` 構造体上に実装したもので、ヘッドライン、著者、そして地域情報を使って`summarize` の戻り値を作っています。
`Tweet` 構造体に関しては、ツイートの内容が既に280文字に制限されていると仮定して、ユーザー名の後にツイートのテキスト全体が続くものとして `summarize` を定義します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 10-13: Implementing the `Summary` trait on the
`NewsArticle` and `Tweet` types</span>
-->

<span class="caption">リスト10-13: `Summary`トレイトを`NewsArticle`と`Tweet`型に実装する</span>

<!--
Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name we want to implement,
then use the `for` keyword, and then specify the name of the type we want to
implement the trait for. Within the `impl` block, we put the method signatures
that the trait definition has defined. Instead of adding a semicolon after each
signature, we use curly brackets and fill in the method body with the specific
behavior that we want the methods of the trait to have for the particular type.
-->

型にトレイトを実装することは、普通のメソッドを実装することに似ています。違いは、`impl`の後に、
実装したいトレイトの名前を置き、それから`for`キーワード、さらにトレイトの実装対象の型の名前を指定することです。
`impl`ブロック内に、トレイト定義で定義したメソッドシグニチャを置きます。各シグニチャの後にセミコロンを追記するのではなく、
波括弧を使用し、メソッド本体に特定の型のトレイトのメソッドに欲しい特定の振る舞いを入れます。

<!--
Now that the library has implemented the `Summary` trait on `NewsArticle` and
`Tweet`, users of the crate can call the trait methods on instances of
`NewsArticle` and `Tweet` in the same way we call regular methods. The only
difference is that the user must bring the trait into scope as well as the
types. Here’s an example of how a binary crate could use our `aggregator`
library crate:
-->

これでライブラリは`NewsArticle`と`Tweet`に対して`Summary`トレイトを実装できたので、クレートの利用者は普通のメソッド同様に`NewsArticle`や`Tweet`のインスタンスに対してこのトレイトメソッドを呼び出せます。
唯一の違いは、ユーザは型だけではなくトレイトもスコープ内に持ち込まなくてはならないということです。
以下は、バイナリクレートが私たちの`aggregator`ライブラリクレートをどうやって使用できるかの例です:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

<!--
This code prints `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.
-->

このコードは、`1 new tweet: horse_ebooks: of course, as you probably already know, people`と出力します。

<!--
Other crates that depend on the `aggregator` crate can also bring the `Summary`
trait into scope to implement `Summary` on their own types. One restriction to
note is that we can implement a trait on a type only if at least one of the
trait or the type is local to our crate. For example, we can implement standard
library traits like `Display` on a custom type like `Tweet` as part of our
`aggregator` crate functionality, because the type `Tweet` is local to our
`aggregator` crate. We can also implement `Summary` on `Vec<T>` in our
`aggregator` crate, because the trait `Summary` is local to our `aggregator`
crate.
-->

`aggregator`クレートに依存する他のクレートも、自身の型に対して`Summary`を実装するために、`Summary`トレイトをスコープに持ち込むことができます。
注意すべき制限の1つは、トレイトか、対象の型のうち、少なくとも一方が自分のクレートに固有(local)である時のみ、
型に対してトレイトを実装できるということです。例えば、`Display`のような標準ライブラリのトレイトを`aggregator`クレートの機能の一部として、
`Tweet`のような独自の型に実装できます。型`Tweet`が`aggregator`クレートに固有だからです。
また、`Summary`を`aggregator`クレートで`Vec<T>`に対して実装することもできます。
トレイト`Summary`は、`aggregator`クレートに固有だからです。

<!--
But we can’t implement external traits on external types. For example, we can’t
implement the `Display` trait on `Vec<T>` within our `aggregator` crate,
because `Display` and `Vec<T>` are both defined in the standard library and
aren’t local to our `aggregator` crate. This restriction is part of a property
called *coherence*, and more specifically the *orphan rule*, so named because
the parent type is not present. This rule ensures that other people’s code
can’t break your code and vice versa. Without the rule, two crates could
implement the same trait for the same type, and Rust wouldn’t know which
implementation to use.
-->

しかし、外部のトレイトを外部の型に対して実装することはできません。例として、
`aggregator`クレート内で`Vec<T>`に対して`Display`トレイトを実装することはできません。
`Display`と`Vec<T>`はどちらも標準ライブラリで定義され、`aggregator`クレートに固有ではないからです。
この制限は、*コヒーレンス*(coherence)、特に*孤児のルール*(orphan rule)と呼ばれる特性の一部で、
親の型が存在しないためにそう命名されました。この規則により、他の人のコードが自分のコードを壊したり、
その逆が起きないことを保証してくれます。この規則がなければ、2つのクレートが同じ型に対して同じトレイトを実装できてしまい、
コンパイラはどちらの実装を使うべきかわからなくなってしまうでしょう。

<!--
### Default Implementations
-->

### デフォルト実装

<!--
Sometimes it’s useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can keep or override
each method’s default behavior.
-->

時として、全ての型の全メソッドに対して実装を要求するのではなく、トレイトの全てあるいは一部のメソッドに対してデフォルトの振る舞いがあると有用です。
そうすれば、特定の型にトレイトを実装する際、各メソッドのデフォルト実装を保持するかオーバーライドするか選べるわけです。

<!--
In Listing 10-14 we specify a default string for the `summarize` method of the
`Summary` trait instead of only defining the method signature, as we did in
Listing 10-12.
-->

リスト10-14では、リスト10-12のように、メソッドシグニチャだけを定義するのではなく、
`Summary`トレイトの`summarize`メソッドにデフォルトの文字列を指定しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 10-14: Defining a `Summary` trait with a default
implementation of the `summarize` method</span>
-->

<span class="caption">リスト10-14: `summarize`メソッドのデフォルト実装がある`Summary`トレイトを定義する</span>

<!--
To use a default implementation to summarize instances of `NewsArticle`, we
specify an empty `impl` block with `impl Summary for NewsArticle {}`.
-->
デフォルト実装を利用して`NewsArticle`のインスタンスをまとめるには、
`impl Summary for NewsArticle {}`と空の`impl`ブロックを指定します。

<!--
Even though we’re no longer defining the `summarize` method on `NewsArticle`
directly, we’ve provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:
-->
もはや`NewsArticle`に直接`summarize`メソッドを定義してはいませんが、私達はデフォルト実装を提供しており、
`NewsArticle`は`Summary`トレイトを実装すると指定しました。そのため、
`NewsArticle`のインスタンスに対して`summarize`メソッドを同じように呼び出すことができます。
このように:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

<!--
This code prints `New article available! (Read more...)`.
-->

このコードは、`New article available! (Read more...)`（`新しい記事があります！（もっと読む）`）と出力します。

<!--
Creating a default implementation doesn’t require us to change anything about
the implementation of `Summary` on `Tweet` in Listing 10-13. The reason is that
the syntax for overriding a default implementation is the same as the syntax
for implementing a trait method that doesn’t have a default implementation.
-->

デフォルト実装を用意しても、リスト10-13の`Tweet`の`Summary`実装を変える必要はありません。
理由は、デフォルト実装をオーバーライドする記法はデフォルト実装のないトレイトメソッドを実装する記法と同じだからです。

<!--
Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, and then define a
`summarize` method that has a default implementation that calls the
`summarize_author` method:
-->

デフォルト実装は、自らのトレイトのデフォルト実装を持たない他のメソッドを呼び出すことができます。
このようにすれば、トレイトは多くの有用な機能を提供しつつ、実装者は僅かな部分しか指定しなくて済むようになります。
例えば、`Summary`トレイトを、（実装者が）内容を実装しなければならない`summarize_author`メソッドを持つように定義し、
それから`summarize_author`メソッドを呼び出すデフォルト実装を持つ`summarize`メソッドを定義することもできます:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

<!--
To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:
-->

このバージョンの`Summary`を使用するために、型にトレイトを実装する際、実装する必要があるのは`summarize_author`だけです:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

<!--
After we define `summarize_author`, we can call `summarize` on instances of the
`Tweet` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we’ve provided. Because we’ve implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code.
-->

`summarize_author`定義後、`Tweet`構造体のインスタンスに対して`summarize`を呼び出せ、
`summarize`のデフォルト実装は、私達が提供した`summarize_author`の定義を呼び出すでしょう。
`summarize_author`を実装したので、追加のコードを書く必要なく、`Summary`トレイトは、
`summarize`メソッドの振る舞いを与えてくれました。

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

<!--
This code prints `1 new tweet: (Read more from @horse_ebooks...)`.
-->

このコードは、`1 new tweet: (Read more from @horse_ebooks...)`（`1つの新しいツイート：（@horse_ebooksさんの文章をもっと読む）`）と出力します。

<!--
Note that it isn’t possible to call the default implementation from an
overriding implementation of that same method.
-->

デフォルト実装を、そのメソッドをオーバーライドしている実装から呼び出すことはできないことに注意してください。

<!--
### Traits as Parameters
-->

### 引数としてのトレイト

<!--
Now that you know how to define and implement traits, we can explore how to use
traits to define functions that accept many different types. We'll use the
`Summary` trait we implemented on the `NewsArticle` and `Tweet` types in
Listing 10-13 to define a `notify` function that calls the `summarize` method
on its `item` parameter, which is of some type that implements the `Summary`
trait. To do this, we use the `impl Trait` syntax, like this:
-->
トレイトを定義し実装する方法はわかったので、トレイトを使っていろんな種類の型を受け付ける関数を定義する方法を学んでいきましょう。
リスト10-13で`NewsArticle`と`Tweet`に対して実装した`Summary`トレイトを使用して、`notify`関数を定義しましょう。
この関数は、`Summary`トレイトを実装する何らかの型を持つ引数`item`を持ち、それに対して`summarize`メソッドを呼び出します。
これを行うためには、このように`impl Trait`構文を使います:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

<!--
Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait name. This parameter accepts any type that implements the
specified trait. In the body of `notify`, we can call any methods on `item`
that come from the `Summary` trait, such as `summarize`. We can call `notify`
and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the
function with any other type, such as a `String` or an `i32`, won’t compile
because those types don’t implement `Summary`.
-->
引数の`item`には、具体的な型の代わりに、`impl`キーワードとトレイト名を指定します。
この引数は、指定されたトレイトを実装しているあらゆる型を受け付けます。
`notify`の中身では、`summarize`のような、`Summary`トレイトに由来する`item`のあらゆるメソッドを呼び出すことができます。
私達は、`notify`を呼びだし、`NewsArticle`か`Tweet`のどんなインスタンスでも渡すことができます。
この関数を呼び出すときに、`String`や`i32`のような他の型を渡すようなコードはコンパイルできません。
なぜなら、これらの型は`Summary`を実装していないからです。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="fixing-the-largest-function-with-trait-bounds"></a>
-->

<!--
#### Trait Bound Syntax
-->
#### トレイト境界構文

<!--
The `impl Trait` syntax works for straightforward cases but is actually syntax
sugar for a longer form known as a *trait bound*; it looks like this:
-->
`impl Trait`構文は単純なケースを解決しますが、実はより長い*トレイト境界 (trait bound)* として知られる姿の糖衣構文 (syntax sugar) なのです。
それは以下のようなものです：

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
	// 速報！ {}
    println!("Breaking news! {}", item.summarize());
}
```

<!--
This longer form is equivalent to the example in the previous section but is
more verbose. We place trait bounds with the declaration of the generic type
parameter after a colon and inside angle brackets.
-->
この「より長い」姿は前節の例と等価ですが、より冗長です。
山カッコの中にジェネリックな型引数の宣言を書き、型引数の後ろにコロンを挟んでトレイト境界を置いています。

<!--
The `impl Trait` syntax is convenient and makes for more concise code in simple
cases, while the fuller trait bound syntax can express more complexity in other
cases. For example, we can have two parameters that implement `Summary`. Doing
so with the `impl Trait` syntax looks like this:
-->
簡単なケースでは`impl Trait`構文は便利で、コードを簡潔にしてくれます。
一方でそうでないケースでは、完全なトレイト境界構文を使えばより複雑な制約を表現できます。
たとえば、`Summary`を実装する2つのパラメータを持つような関数を考えることができます。
`impl Trait`構文を使ってそうするのはこのようになるでしょう：

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

<!--
Using `impl Trait` is appropriate if we want this function to allow `item1` and
`item2` to have different types (as long as both types implement `Summary`). If
we want to force both parameters to have the same type, however, we must use a
trait bound, like this:
-->
この関数が受け取る`item1`と`item2`の型が（どちらも`Summary`を実装する限り）異なっても良い場合、`impl Trait`の使用は適切です。
しかし、両方の引数が同じ型であることを強制したい場合は、次のようにトレイト境界を使用しなくてはなりません:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

<!--
The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.
-->
引数である`item1`と`item2`の型としてジェネリックな型`T`を指定しました。
これにより、`item1`と`item2`として関数に渡される値の具体的な型が同一でなければならない、という制約を与えています。

<!--
#### Specifying Multiple Trait Bounds with the `+` Syntax
-->
#### 複数のトレイト境界を`+`構文で指定する

<!--
We can also specify more than one trait bound. Say we wanted `notify` to use
display formatting as well as `summarize` on `item`: we specify in the `notify`
definition that `item` must implement both `Display` and `Summary`. We can do
so using the `+` syntax:
-->
複数のトレイト境界も指定できます。
たとえば、`notify`に、`item`に対する`summarize`に加えて画面出力形式（ディスプレイフォーマット）も使わせたいとします。
その場合は、`notify`の定義に`item`は`Display`と`Summary`の両方を実装していなくてはならないと指定することになります。
これは、以下のように`+`構文で行うことができます：

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

<!--
The `+` syntax is also valid with trait bounds on generic types:
-->
`+`構文はジェネリック型につけたトレイト境界に対しても使えます：

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

<!--
With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.
-->
これら2つのトレイト境界が指定されていれば、`notify`の中では`summarize`を呼び出すことと、`{}`を使って`item`をフォーマットすることの両方が行なえます。

<!--
#### Clearer Trait Bounds with `where` Clauses
-->
#### `where`句を使ったより明確なトレイト境界

<!--
Using too many trait bounds has its downsides. Each generic has its own trait
bounds, so functions with multiple generic type parameters can contain lots of
trait bound information between the function’s name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So instead of writing this:
-->
あまりたくさんのトレイト境界を使うことには欠点もあります。
それぞれのジェネリック（な型）がそれぞれのトレイト境界をもつので、複数のジェネリック型の引数をもつ関数は、関数名と引数リストの間に大量のトレイト境界に関する情報を含むことがあります。
これでは関数のシグネチャが読みにくくなってしまいます。
このため、Rustはトレイト境界を関数シグネチャの後の`where`句の中で指定するという別の構文を用意しています。
なので、このように書く：

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

<!--
we can use a `where` clause, like this:
-->
代わりに、`where`句を使い、このように書くことができます：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

<!--
This function’s signature is less cluttered: the function name, parameter list,
and return type are close together, similar to a function without lots of trait
bounds.
-->
この関数シグニチャは、よりさっぱりとしています。トレイト境界を多く持たない関数と同じように、関数名、引数リスト、戻り値の型が一緒になって近くにあるからですね。

<!--
### Returning Types that Implement Traits
-->
### トレイトを実装している型を返す


<!--
We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:
-->
以下のように、`impl Trait`構文を戻り値型のところで使うことにより、あるトレイトを実装する何らかの型を返すことができます。

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

<!--
By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `Tweet`, but the code calling this function doesn’t need to know that.
-->
戻り値の型として`impl Summary`を使うことにより、具体的な型が何かを言うことなく、`returns_summarizable`関数は`Summary`トレイトを実装している何らかの型を返すのだ、と指定することができます。
今回`returns_summarizable`は`Tweet`を返しますが、この関数を呼び出すコードはそのことを知る必要はありません。

<!--
The ability to specify a return type only by the trait it implements is
especially useful in the context of closures and iterators, which we cover in
Chapter 13. Closures and iterators create types that only the compiler knows or
types that are very long to specify. The `impl Trait` syntax lets you concisely
specify that a function returns some type that implements the `Iterator` trait
without needing to write out a very long type.
-->
実装しているトレイトだけで戻り値型を指定できることは、13章で学ぶ、クロージャとイテレータを扱うときに特に便利です。
クロージャとイテレータの作り出す型は、コンパイラだけが知っているものであったり、指定するには長すぎるものであったりします。
`impl Trait`構文を使えば、非常に長い型を書くことなく、ある関数は`Iterator`トレイトを実装するある型を返すのだ、と簡潔に指定することができます。

<!--
However, you can only use `impl Trait` if you’re returning a single type. For
example, this code that returns either a `NewsArticle` or a `Tweet` with the
return type specified as `impl Summary` wouldn’t work:
-->
ただし、`impl Trait`は一種類の型を返す場合にのみ使えます。
たとえば、以下のように、戻り値の型は`impl Summary`で指定しつつ、`NewsArticle`か`Tweet`を返すようなコードは失敗します：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

<!--
Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions
around how the `impl Trait` syntax is implemented in the compiler. We’ll cover
how to write a function with this behavior in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]
section of Chapter 17.
-->
`NewsArticle`か`Tweet`を返すというのは、コンパイラの`impl Trait`構文の実装まわりの制約により許されていません。
このような振る舞いをする関数を書く方法は、17章の[「トレイトオブジェクトで異なる型の値を許容する」][using-trait-objects-that-allow-for-values-of-different-types]節で学びます。

<!--
### Using Trait Bounds to Conditionally Implement Methods
-->
### トレイト境界を使用して、メソッド実装を条件分けする

<!--
By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-15 always implements the
`new` function to return a new instance of `Pair<T>` (recall from the
[“Defining Methods”][methods] section of Chapter 5 that `Self`
is a type alias for the type of the `impl` block, which in this case is
`Pair<T>`). But in the next `impl` block, `Pair<T>` only implements the
`cmp_display` method if its inner type `T` implements the `PartialOrd` trait
that enables comparison *and* the `Display` trait that enables printing.
-->
ジェネリックな型引数を持つ`impl`ブロックにトレイト境界を与えることで、
特定のトレイトを実装する型に対するメソッド実装を条件分けできます。例えば、
リスト10-15の型`Pair<T>`は、`Pair<T>`の新しいインスタンスを返す`new`関数を常に実装します
（第5章の[「メソッドを定義する」][methods]節で学んだ、`Self`は`impl`ブロックの型に対する型エイリアスだということを思い出してください、今回は`Pair<T>`です）。しかし次の`impl`ブロック内では、`Pair<T>`は、
内部の型`T`が比較を可能にする`PartialOrd`トレイト*と*出力を可能にする`Display`トレイトを実装している時のみ、
`cmp_display`メソッドを実装します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->
<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

<!--
<span class="caption">Listing 10-15: Conditionally implementing methods on a
generic type depending on trait bounds</span>
-->
<span class="caption">リスト10-15: トレイト境界によってジェネリックな型に対するメソッド実装を条件分けする</span>

<!--
We can also conditionally implement a trait for any type that implements
another trait. Implementations of a trait on any type that satisfies the trait
bounds are called *blanket implementations* and are extensively used in the
Rust standard library. For example, the standard library implements the
`ToString` trait on any type that implements the `Display` trait. The `impl`
block in the standard library looks similar to this code:
-->
また、別のトレイトを実装するあらゆる型に対するトレイト実装を条件分けすることもできます。
トレイト境界を満たすあらゆる型にトレイトを実装することは、*ブランケット実装*(blanket implementation)と呼ばれ、
Rustの標準ライブラリで広く使用されています。例を挙げれば、標準ライブラリは、
`Display`トレイトを実装するあらゆる型に`ToString`トレイトを実装しています。
標準ライブラリの`impl`ブロックは以下のような見た目です:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

<!--
Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:
-->
標準ライブラリにはこのブランケット実装があるので、`Display`トレイトを実装する任意の型に対して、
`ToString`トレイトで定義された`to_string`メソッドを呼び出せるのです。
例えば、整数は`Display`を実装するので、このように整数値を対応する`String`値に変換できます:

```rust
let s = 3.to_string();
```

<!--
Blanket implementations appear in the documentation for the trait in the
“Implementors” section.
-->
ブランケット実装は、トレイトのドキュメンテーションの「実装したもの」節に出現します。

<!--
Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type which didn’t define the method. But Rust
moves these errors to compile time so we’re forced to fix the problems before
our code is even able to run. Additionally, we don’t have to write code that
checks for behavior at runtime because we’ve already checked at compile time.
Doing so improves performance without having to give up the flexibility of
generics.
-->
トレイトとトレイト境界により、ジェネリックな型引数を使用して重複を減らしつつ、コンパイラに対して、
そのジェネリックな型に特定の振る舞いが欲しいことを指定するコードを書くことができます。
それからコンパイラは、トレイト境界の情報を活用してコードに使用された具体的な型が正しい振る舞いを提供しているか確認できます。
動的型付き言語では、その型に定義されていないメソッドを呼び出せば、実行時 (runtime) にエラーが出るでしょう。
しかし、Rustはこの種のエラーをコンパイル時に移したので、コードが動かせるようになる以前に問題を修正することを強制されるのです。
加えて、コンパイル時に既に確認したので、実行時の振る舞いを確認するコードを書かなくても済みます。
そうすることで、ジェネリクスの柔軟性を諦めることなくパフォーマンスを向上させます。

<!--
[using-trait-objects-that-allow-for-values-of-different-types]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[methods]: ch05-03-method-syntax.html#defining-methods
-->

[using-trait-objects-that-allow-for-values-of-different-types]: ch17-02-trait-objects.html#トレイトオブジェクトで異なる型の値を許容する
[methods]: ch05-03-method-syntax.html#メソッドを定義する
