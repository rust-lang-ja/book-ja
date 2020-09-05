<!--
## Traits: Defining Shared Behavior
-->

## トレイト: 共通の振る舞いを定義する

<!--
A *trait* tells the Rust compiler about functionality a particular type has and
can share with other types. We can use traits to define shared behavior in an
abstract way. We can use trait bounds to specify that a generic can be any type
that has certain behavior.
-->

*トレイト*により、Rustコンパイラに特定の型に存在し、他の型と共有できる機能について知らせます。
トレイトを使用して共通の振る舞いを抽象的に定義できます。トレイト境界を使用して、
あるジェネリックが特定の振る舞いのあるあらゆる型になり得ることを指定できます。

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

型の振る舞いは、その型に対して呼び出せるメソッドから構成されます。異なる型は、それらの型全部に対して同じメソッドを呼び出せたら、
同じ振る舞いを共有します。トレイト定義は、メソッドシグニチャを一緒くたにしてなんらかの目的を達成するのに必要な一連の振る舞いを定義する手段です。

<!--
For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.
-->

例えば、いろんな種類や量のテキストを保持する複数の構造体があるとしましょう: 特定の場所で送られる新しいニュースを保持する`NewsArticle`と、
新規ツイートか、リツイートか、はたまた他のツイートへのリプライなのかを示すメタデータを伴う最大で280文字までの`Tweet`です。

<!--
We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we
need a summary from each type, and we need to request that summary by calling a
`summarize` method on an instance. Listing 10-12 shows the definition of a
`Summary` trait that expresses this behavior.
-->

`NewsArticle` または `Tweet` インスタンスに保存されているデータのサマリを表示できるメディア アグリゲータ ライブラリを作成します。
これをするには、各型のサマリーが必要で、インスタンスで `summarize` メソッドを呼び出してサマリーを要求する必要があります。
リスト10-12は、この振る舞いを表現する`Summary`トレイトの定義を表示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

<!--
<span class="caption">Listing 10-12: A `Summary` trait that consists of the
behavior provided by a `summarize` method</span>
-->

<span class="caption">リスト10-12: `summarize`メソッドで提供される振る舞いからなる`Summary`トレイト</span>

<!--
Here, we declare a trait using the `trait` keyword and then the trait’s name,
which is `Summary` in this case. Inside the curly brackets, we declare the
method signatures that describe the behaviors of the types that implement this
trait, which in this case is `fn summarize(&self) -> String`.
-->

ここでは、`trait`キーワード、それからトレイト名を使用してトレイトを定義していて、その名前は今回の場合、
`Summary`です。波括弧の中にこのトレイトを実装する型の振る舞いを記述するメソッドシグニチャを定義し、
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
コンパイラにより、`Summary`トレイトを保持するあらゆる型に、このシグニチャと全く同じメソッド`summarize`が定義されていることが、
強制されます。

<!--
A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.
-->

トレイトには、本体に複数のメソッドを含むことができます: メソッドシグニチャは行ごとに列挙され、
各行はセミコロンで終止します。

<!--
### Implementing a Trait on a Type
-->

### トレイトを型に実装する

<!--
Now that we’ve defined the desired behavior using the `Summary` trait, we can
implement it on the types in our media aggregator. Listing 10-13 shows an
implementation of the `Summary` trait on the `NewsArticle` struct that uses the
headline, the author, and the location to create the return value of
`summarize`. For the `Tweet` struct, we define `summarize` as the username
followed by the entire text of the tweet, assuming that tweet content is
already limited to 280 characters.
-->

今や `Summary` トレイトを使用して目的の動作を定義できたので、メディア アグリゲータで型に実装できます。
リスト10-13は、 `Summary` トレイトを `NewsArticle` 構造体上に実装したもので、ヘッドライン、著者、そして `summarize` の戻り値を示しています。
`Tweet` 構造体に関しては、ツイートの内容が既に280文字に制限されていると仮定して、ユーザー名の後にツイートのテキスト全体が続くものとして `summarize` を定義します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
# pub trait Summary {
#     fn summarize(&self) -> String;
# }
#
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<!--
<span class="caption">Listing 10-13: Implementing the `Summary` trait on the
`NewsArticle` and `Tweet` types</span>
-->

<span class="caption">リスト10-13: `Summary`トレイトを`NewsArticle`と`Tweet`型に実装する</span>

<!--
Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name that we want to
implement, then use the `for` keyword, and then specify the name of the type we
want to implement the trait for. Within the `impl` block, we put the method
signatures that the trait definition has defined. Instead of adding a semicolon
after each signature, we use curly brackets and fill in the method body with
the specific behavior that we want the methods of the trait to have for the
particular type.
-->

型にトレイトを実装することは、普通のメソッドを実装することに似ています。違いは、`impl`の後に、
実装したいトレイトの名前を置き、それから`for`キーワード、さらにトレイトの実装対象の型の名前を指定することです。
`impl`ブロック内に、トレイト定義で定義したメソッドシグニチャを置きます。各シグニチャの後にセミコロンを追記するのではなく、
波括弧を使用し、メソッド本体に特定の型のトレイトのメソッドに欲しい特定の振る舞いを入れます。

<!--
After implementing the trait, we can call the methods on instances of
`NewsArticle` and `Tweet` in the same way we call regular methods, like this:
-->

トレイトを実装後、普通のメソッド同様に`NewsArticle`や`Tweet`のインスタンスに対してこのメソッドを呼び出せます。
こんな感じで:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    // もちろん、ご存知かもしれないようにね、みなさん
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

<!--
This code prints `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.
-->

このコードは、`1 new tweet: horse_ebooks: of course, as you probably already know, people`と出力します。

<!--
Note that because we defined the `Summary` trait and the `NewsArticle` and
`Tweet` types in the same *lib.rs* in Listing 10-13, they’re all in the same
scope. Let’s say this *lib.rs* is for a crate we’ve called `aggregator` and
someone else wants to use our crate’s functionality to implement the `Summary`
trait on a struct defined within their library’s scope. They would need to
import the trait into their scope first. They would do so by specifying `use
aggregator::Summary;`, which then would enable them to implement `Summary` for
their type. The `Summary` trait would also need to be a public trait for
another crate to implement it, which it is because we put the `pub` keyword
before `trait` in Listing 10-12.
-->

リスト10-13で`Summary`トレイトと`NewArticle`、`Tweet`型を同じ*lib.rs*に定義したので、
全部同じスコープにあることに注目してください。この*lib.rs*を`aggregator`と呼ばれるクレート専用にして、
誰か他の人が私たちのクレートの機能を活用して自分のライブラリのスコープに定義された構造体に`Summary`トレイトを実装したいとしましょう。
まず、トレイトをスコープにインポートする必要があるでしょう。`use aggregator::Summary;`と指定してそれを行い、
これにより、自分の型に`Summary`を実装することが可能になるでしょう。`Summary`トレイトは、
他のクレートが実装するためには、公開トレイトである必要があり、ここでは、リスト10-12の`trait`の前に、
`pub`キーワードを置いたのでそうなっています。

<!--
One restriction to note with trait implementations is that we can implement a
trait on a type only if either the trait or the type is local to our crate.
For example, we can implement standard library traits like `Display` on a
custom type like `Tweet` as part of our `aggregator` crate functionality,
because the type `Tweet` is local to our `aggregator` crate. We can also
implement `Summary` on `Vec<T>` in our `aggregator` crate, because the
trait `Summary` is local to our `aggregator` crate.
-->

トレイト実装で注意すべき制限の1つは、トレイトか対象の型が自分のクレートに固有(local)である時のみ、
型に対してトレイトを実装できるということです。例えば、`Display`のような標準ライブラリのトレイトを`aggregator`クレートの機能の一部として、
`Tweet`のような独自の型に実装できます。型`Tweet`が`aggregator`クレートに固有だからです。
また、`Summary`を`aggregator`クレートで`Vec<T>`に対して実装することもできます。
トレイト`Summary`は、`aggregator`クレートに固有だからです。

<!--
But we can’t implement external traits on external types. For example, we can’t
implement the `Display` trait on `Vec<T>` within our `aggregator` crate,
because `Display` and `Vec<T>` are defined in the standard library and aren’t
local to our `aggregator` crate. This restriction is part of a property of
programs called *coherence*, and more specifically the *orphan rule*, so named
because the parent type is not present. This rule ensures that other people’s
code can’t break your code and vice versa. Without the rule, two crates could
implement the same trait for the same type, and Rust wouldn’t know which
implementation to use.
-->

しかし、外部のトレイトを外部の型に対して実装することはできません。例として、
`aggregator`クレート内で`Vec<T>`に対して`Display`トレイトを実装することはできません。
`Display`と`Vec<T>`は標準ライブラリで定義され、`aggregator`クレートに固有ではないからです。
この制限は、*コヒーレンス*(coherence)あるいは、具体的に*オーファンルール*(orphan rule)と呼ばれるプログラムの特性の一部で、
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

時として、全ての型の全メソッドに対して実装を必要とするのではなく、トレイトの全てあるいは一部のメソッドに対してデフォルトの振る舞いがあると有用です。
そうすれば、特定の型にトレイトを実装する際、各メソッドのデフォルト実装を保持するかオーバーライドできるわけです。

<!--
Listing 10-14 shows how to specify a default string for the `summarize` method
of the `Summary` trait instead of only defining the method signature, as we did
in Listing 10-12:
-->

リスト10-14は、リスト10-12のように、メソッドシグニチャだけを定義するのではなく、
`Summary`トレイトの`summarize`メソッドにデフォルトの文字列を指定する方法を示しています:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        // (もっと読む)
        String::from("(Read more...)")
    }
}
```

<!--
<span class="caption">Listing 10-14: Definition of a `Summary` trait with a
default implementation of the `summarize` method</span>
-->

<span class="caption">リスト10-14: `summarize`メソッドのデフォルト実装がある`Summary`トレイトの定義</span>

<!--
To use a default implementation to summarize instances of `NewsArticle` instead
of defining a custom implementation, we specify an empty `impl` block with
`impl Summary for NewsArticle {}`.
-->

独自の実装を定義するのではなく、デフォルト実装を使用して`NewsArticle`のインスタンスをまとめるには、
`impl Summary for NewsArticle {}`と空の`impl`ブロックを指定します。

<!--
Even though we’re no longer defining the `summarize` method on `NewsArticle`
directly, we’ve provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:
-->

たとえ、最早`NewsArticle`に直接`summarize`メソッドを定義することはなくても、デフォルト実装を提供し、
`NewsArticle`は`Summary`トレイトを実装すると指定しました。結果的に、それでも、
`NewsArticle`のインスタンスに対して`summarize`メソッドを呼び出すことができます。
このように:

```rust,ignore
let article = NewsArticle {
    // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    // ピッツバーグ、ペンシルベニア州、アメリカ
    location: String::from("Pittsburgh, PA, USA"),
    // アイスバーグ
    author: String::from("Iceburgh"),
    // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

// 新しい記事が利用可能です！ {}
println!("New article available! {}", article.summarize());
```

<!--
This code prints `New article available! (Read more...)`.
-->

このコードは、`New article available! (Read more...)`と出力します。

<!--
Creating a default implementation for `summarize` doesn’t require us to change
anything about the implementation of `Summary` on `Tweet` in Listing 10-13. The
reason is that the syntax for overriding a default implementation is the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.
-->

`summarize`にデフォルト実装を用意しても、リスト10-13の`Tweet`の`Summary`実装を変える必要はありません。
理由は、デフォルト実装をオーバーライドする記法がデフォルト実装のないトレイトメソッドを実装する記法と同じだからです。

<!--
Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, and then define a
`summarize` method that has a default implementation that calls the
`summarize_author` method:
-->

デフォルト実装は、他のデフォルト実装がないメソッドでも呼び出すことができます。
このように、トレイトは多くの有用な機能を提供しつつ、実装者に僅かな部分だけ指定してもらう必要しかないのです。
例えば、`Summary`トレイトを実装が必須の`summarize_author`メソッドを持つように定義し、
それから`summarize_author`メソッドを呼び出すデフォルト実装のある`summarize`メソッドを定義することもできます:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // {}さんからもっと読む
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

<!--
To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:
-->

このバージョンの`Summary`を使用するには、型にトレイトを実装する際に`summarize_author`を定義する必要しかありません:

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

<!--
After we define `summarize_author`, we can call `summarize` on instances of the
`Tweet` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we’ve provided. Because we’ve implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code.
-->

`summarize_author`定義後、`Tweet`構造体のインスタンスに対して`summarize`を呼び出せ、
`summarize`のデフォルト実装は、提供済みの`summarize_author`の定義を呼び出すでしょう。
`summarize_author`を実装したので、追加のコードを書く必要なく、`Summary`トレイトは、
`summarize`メソッドの振る舞いを与えてくれました。

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

<!--
This code prints `1 new tweet: (Read more from @horse_ebooks...)`.
-->

このコードは、`1 new tweet: (Read more from @horse_ebooks...)`と出力します。

<!--
Note that it isn’t possible to call the default implementation from an
overriding implementation of that same method.
-->

同じメソッドのオーバーライドした実装からは、デフォルト実装を呼び出すことができないことに注意してください。

<!--
### Trait Bounds
-->

### トレイト境界

<!--
Now that you know how to define traits and implement those traits on types, we
can explore how to use traits with generic type parameters. We can use *trait
bounds* to constrain generic types to ensure the type will be limited to those
that implement a particular trait and behavior.
-->

これでトレイトの定義とトレイトを型に実装する方法を知ったので、ジェネリックな型引数でトレイトを使用する方法を探究できます。
*トレイト境界*を使用してジェネリックな型を制限し、型が特定のトレイトや振る舞いを実装するものに制限されることを保証できます。

<!--
For example, in Listing 10-13, we implemented the `Summary` trait on the types
`NewsArticle` and `Tweet`. We can define a function `notify` that calls the
`summarize` method on its parameter `item`, which is of the generic type `T`.
To be able to call `summarize` on `item` without getting an error telling us
that the generic type `T` doesn’t implement the method `summarize`, we can use
trait bounds on `T` to specify that `item` must be of a type that implements
the `Summary` trait:
-->

例として、リスト10-13で、`Summary`トレイトを型`NewsArticle`と`Tweet`に実装しました。
引数`item`に対して`summarize`メソッドを呼び出す関数`notify`を定義でき、この引数はジェネリックな型`T`です。
`item`の`summarize`を呼ぶときにジェネリックな型`T`がメソッド`summarize`を実装してないというエラーが出ないように、
`T`のトレイト境界を使って`item`が`Summary`トレイトを実装する型でなければならないと指定できます。

```rust,ignore
pub fn notify<T: Summary>(item: T) {
    // 新ニュース！ {}
    println!("Breaking news! {}", item.summarize());
}
```

<!--
We place trait bounds with the declaration of the generic type parameter, after
a colon and inside angle brackets. Because of the trait bound on `T`, we can
call `notify` and pass in any instance of `NewsArticle` or `Tweet`. Code that
calls the function with any other type, like a `String` or an `i32`, won’t
compile, because those types don’t implement `Summary`.
-->

トレイト境界をジェネリックな型引数宣言とともにコロンの後、山カッコ内に配置しています。`T`に付けられたトレイト境界のため、
`notify`を呼び出して`NewsArticle`か`Tweet`のどんなインスタンスも渡すことができます。
あらゆる他の型、`String`や`i32`などでこの関数を呼び出すコードは、型が`Summary`を実装しないので、
コンパイルできません。

<!--
We can specify multiple trait bounds on a generic type using the `+` syntax.
For example, to use display formatting on the type `T` in a function as well as
the `summarize` method, we can use `T: Summary + Display` to say `T` can be any
type that implements `Summary` and `Display`.
-->

`+`記法でジェネリックな型に複数のトレイト境界を指定できます。例えば、関数で`T`に対してフォーマット表示と、
`summarize`メソッドを使用するには、`T: Summary + Display`を使用して、`T`は`Summary`と`Display`を実装するどんな型にもなると宣言できます。

<!--
However, there are downsides to using too many trait bounds. Each generic has
its own trait bounds, so functions with multiple generic type parameters can
have lots of trait bound information between a function’s name and its
parameter list, making the function signature hard to read. For this reason,
Rust has alternate syntax for specifying trait bounds inside a `where` clause
after the function signature. So instead of writing this:
-->

しかしながら、トレイト境界が多すぎると欠点もあります。各ジェネリックには、特有のトレイト境界があるので、
複数のジェネリックな型引数がある関数には、関数名と引数リストの間に多くのトレイト境界の情報が付くこともあり、
関数シグニチャが読みづらくなる原因になります。このため、Rustには関数シグニチャの後、
`where`節内にトレイト境界を指定する対立的な記法があります。従って、こう書く代わりに:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

<!--
we can use a `where` clause, like this:
-->

こんな感じに`where`節を活用できます:

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

<!--
This function’s signature is less cluttered in that the function name,
parameter list, and return type are close together, similar to a function
without lots of trait bounds.
-->

この関数シグニチャは、多くのトレイト境界のない関数のように、関数名、引数リスト、戻り値の型が一緒になって近いという点でごちゃごちゃしていません。

<!--
### Fixing the `largest` Function with Trait Bounds
-->

### トレイト境界で`largest`関数を修正する

<!--
Now that you know how to specify the behavior you want to use using the generic
type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of
the `largest` function that uses a generic type parameter! Last time we tried
to run that code, we received this error:
-->

ジェネリックな型引数の境界で使用したい振る舞いを指定する方法を知ったので、リスト10-5に戻って、
ジェネリックな型引数を使用する`largest`関数の定義を修正しましょう！最後にそのコードを実行しようとしたら、
こんなエラーが出ました:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

<!--
In the body of `largest` we wanted to compare two values of type `T` using the
greater than (`>`) operator. Because that operator is defined as a default
method on the standard library trait `std::cmp::PartialOrd`, we need to specify
`PartialOrd` in the trait bounds for `T` so the `largest` function can work on
slices of any type that we can compare. We don’t need to bring `PartialOrd`
into scope because it’s in the prelude. Change the signature of `largest` to
look like this:
-->

`largest`の本体で、大なり演算子(`>`)を使用して型`T`の2つの値を比較したかったのです。その演算子は、
標準ライブラリトレイトの`std::cmp::PartialOrd`でデフォルトメソッドとして定義されているので、
`largest`関数が、比較できるあらゆる型のスライスに対して動くように、`T`のトレイト境界に`PartialOrd`を指定する必要があります。
初期化処理に含まれているので、`PartialOrd`をスコープに導入する必要はありません。
`largest`のシグニチャを以下のような見た目に変えてください:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

<!--
This time when we compile the code, we get a different set of errors:
-->

今度コードをコンパイルすると、異なる一連のエラーが出ます:

```text
error[E0508]: cannot move out of type `[T]`, a non-copy slice
(エラー: `[T]`、コピーでないスライスからムーブできません。)
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`
                         (助言: 代わりに参照の使用を考慮してください: `&list[0]`)

error[E0507]: cannot move out of borrowed content
(エラー: 借用された内容からムーブできません)
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
            (ヒント: ムーブを避けるには、`ref item`か`ref mut item`を使用してください)
```

<!--
The key line in this error is `cannot move out of type [T], a non-copy slice`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As discussed in the “Stack-Only Data: Copy”
section in Chapter 4, types like `i32` and `char` that have a known size can be
stored on the stack, so they implement the `Copy` trait. But when we made the
`largest` function generic, it became possible for the `list` parameter to have
types in it that don’t implement the `Copy` trait. Consequently, we wouldn’t be
able to move the value out of `list[0]` and into the `largest` variable,
resulting in this error.
-->

このエラーの鍵となる行は、`cannot move out of type [T], a non-copy slice`です。
ジェネリックでないバージョンの`largest`関数では、最大の`i32`か`char`を探そうとするだけでした。
第4章の「スタックだけのデータ: コピー」節で議論したように、`i32`や`char`のような既知のサイズの型は、
スタックに格納できるので、`Copy`トレイトを実装しています。しかし、`largest`関数をジェネリックにすると、
`list`引数が`Copy`トレイトを実装しない型を含む可能性も出てきたのです。結果として、
`list[0]`から値を`largest`にムーブできず、このエラーに陥ったのです。

<!--
To call this code with only those types that implement the `Copy` trait, we can
add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into the function implement the `PartialOrd`
*and* `Copy` traits, like `i32` and `char` do.
-->

このコードを`Copy`トレイトを実装する型とだけで呼び出すには、`T`のトレイト境界に`Copy`を追加できます！
リスト10-15は、関数に渡したスライスの値の型が`i32`や`char`などのように、`PartialOrd`*と*`Copy`を実装する限り、
コンパイルできるジェネリックな`largest`関数の完全なコードを示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<!--
<span class="caption">Listing 10-15: A working definition of the `largest`
function that works on any generic type that implements the `PartialOrd` and
`Copy` traits</span>
-->

<span class="caption">リスト10-15: `PartialOrd`と`Copy`トレイトを実装するあらゆるジェネリックな型に対して動く、
`largest`関数の動く定義</span>

<!--
If we don’t want to restrict the `largest` function to the types that implement
the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
of `Copy`. Then we could clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations in the case of types that own heap
data like `String`, and heap allocations can be slow if we’re working with
large amounts of data.
-->

もし`largest`関数を`Copy`を実装する型だけに制限したくなかったら、`Copy`ではなく、
`T`が`Clone`というトレイト境界を持つと指定することもできます。そうしたら、
`largest`関数に所有権が欲しい時にスライスの各値をクローンできます。`clone`関数を使用するということは、
`String`のようなヒープデータを所有する型の場合にもっとヒープ確保が発生する可能性があることを意味し、
大きなデータを取り扱っていたら、ヒープ確保は遅いこともあります。

<!--
Another way we could implement `largest` is for the function to return a
reference to a `T` value in the slice. If we change the return type to `&T`
instead of `T`, thereby changing the body of the function to return a
reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could
avoid heap allocations. Try implementing these alternate solutions on your own!
-->

`largest`の別の実装方法は、関数がスライスの`T`値への参照を返すようにすることです。
戻り値の型を`T`ではなく`&T`に変え、それにより関数の本体を参照を返すように変更したら、
`Clone`か`Copy`トレイト境界は必要なくなり、ヒープ確保も避けられるでしょう。
試しにこれらの対立的な解決策もご自身で実装してみてください！

<!--
### Using Trait Bounds to Conditionally Implement Methods
-->

### トレイト境界を使用して、メソッド実装を条件分けする

<!--
By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-16 always implements the
`new` function. But `Pair<T>` only implements the `cmp_display` method if its
inner type `T` implements the `PartialOrd` trait that enables comparison *and*
the `Display` trait that enables printing.
-->

ジェネリックな型引数を持つ`impl`ブロックにトレイト境界を与えることで、
特定のトレイトを実装する型に対するメソッド実装を条件分けできます。例えば、
リスト10-16の型`Pair<T>`は、常に`new`関数を実装します。しかし、`Pair<T>`は、
内部の型`T`が比較を可能にする`PartialOrd`トレイト*と*出力を可能にする`Display`トレイトを実装している時のみ、
`cmp_display`メソッドを実装します。

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

<!--
<span class="caption">Listing 10-16: Conditionally implement methods on a
generic type depending on trait bounds</span>
-->

<span class="caption">リスト10-16: トレイト境界によってジェネリックな型に対するメソッド実装を条件分けする</span>

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

ブランケット実装は、「実装したもの」節のトレイトのドキュメンテーションに出現します。

<!--
Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type that the type didn’t implement. But
Rust moves these errors to compile time so we’re forced to fix the problems
before our code is even able to run. Additionally, we don’t have to write code
that checks for behavior at runtime because we’ve already checked at compile
time. Doing so improves performance without having to give up the flexibility
of generics.
-->

トレイトとトレイト境界により、ジェネリックな型引数を使用して重複を減らしつつ、コンパイラに対して、
そのジェネリックな型に特定の振る舞いが欲しいことを指定するコードを書くことができます。
それからコンパイラは、トレイト境界の情報を活用してコードに使用された具体的な型が正しい振る舞いを提供しているか確認できます。
動的型付け言語では、型が実装しない型のメソッドを呼び出せば、実行時にエラーが出るでしょう。
しかし、Rustはこの種のエラーをコンパイル時に移したので、コードが動かせるようにさえなる以前に問題を修正することを強制されるのです。
加えて、コンパイル時に既に確認したので、実行時に振る舞いがあるかどうか確認するコードを書かなくても済みます。
そうすることでジェネリクスの柔軟性を諦める必要なく、パフォーマンスを向上させます。

<!--
Another kind of generic that we’ve already been using is called *lifetimes*.
Rather than ensuring that a type has the behavior we want, lifetimes ensure
that references are valid as long as we need them to be. Let’s look at how
lifetimes do that.
-->

もう使用してきたことのある別の種のジェネリクスは、ライフタイムと呼ばれます。
型が欲しい振る舞いを保持していることを保証するのではなく、必要な間だけ参照が有効であることをライフタイムは保証します。
ライフタイムがどうやってそれを行うかを見ましょう。
