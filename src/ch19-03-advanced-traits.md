<!--
## Advanced Traits
-->

## 高度なトレイト

<!--
We first covered traits in the “Traits: Defining Shared Behavior” section of
Chapter 10, but as with lifetimes, we didn’t discuss the more advanced details.
Now that you know more about Rust, we can get into the nitty-gritty.
-->

最初にトレイトについて講義したのは、第 10 章の「トレイト：共通の振る舞いを定義する」節でしたが、
ライフタイム同様、より高度な詳細は議論しませんでした。今や、Rust に詳しくなったので、核心に迫れるでしょう。

<!--
### Specifying Placeholder Types in Trait Definitions with Associated Types
-->

### 関連型でトレイト定義においてプレースホルダーの型を指定する

<!--
*Associated types* connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures. The
implementor of a trait will specify the concrete type to be used in this type’s
place for the particular implementation. That way, we can define a trait that
uses some types without needing to know exactly what those types are until the
trait is implemented.
-->

*関連型*は、トレイトのメソッド定義がシグニチャでプレースホルダーの型を使用できるように、トレイトと型のプレースホルダーを結び付けます。
トレイトを実装するものがこの特定の実装で型の位置に使用される具体的な型を指定します。そうすることで、
なんらかの型を使用するトレイトをトレイトを実装するまでその型が一体なんであるかを知る必要なく定義できます。

<!--
3 行目、the rest of the book とあるが、これ以降の章という意味での残りではなく、ここまでに見かけてきた章のことと思われるため
-->

<!--
We’ve described most of the advanced features in this chapter as being rarely
needed. Associated types are somewhere in the middle: they’re used more rarely
than features explained in the rest of the book but more commonly than many of
the other features discussed in this chapter.
-->

この章のほとんどの高度な機能は、稀にしか必要にならないと解説しました。関連型はその中間にあります：
本の他の部分で説明される機能よりは使用されるのが稀ですが、この章で議論される他の多くの機能よりは頻繁に使用されます。

<!--
One example of a trait with an associated type is the `Iterator` trait that the
standard library provides. The associated type is named `Item` and stands in
for the type of the values the type implementing the `Iterator` trait is
iterating over. In “The `Iterator` Trait and the `next` Method” section of
Chapter 13, we mentioned that the definition of the `Iterator` trait is as
shown in Listing 19-20.
-->

関連型があるトレイトの一例は、標準ライブラリが提供する`Iterator`トレイトです。その関連型は`Item`と名付けられ、
`Iterator`トレイトを実装している型が走査している値の型の代役を務めます。第 13 章の「`Iterator`トレイトと`next`メソッド」節で、
`Iterator`トレイトの定義は、リスト 19-20 に示したようなものであることに触れました。

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

<!--
<span class="caption">Listing 19-20: The definition of the `Iterator` trait
that has an associated type `Item`</span>
-->

<span class="caption">リスト 19-20: 関連型`Item`がある`Iterator`トレイトの定義</span>

<!--
The type `Item` is a placeholder type, and the `next` method’s definition shows
that it will return values of type `Option<Self::Item>`. Implementors of the
`Iterator` trait will specify the concrete type for `Item`, and the `next`
method will return an `Option` containing a value of that concrete type.
-->

型`Item`はプレースホルダー型で`next`メソッドの定義は、型`Option<Self::Item>`の値を返すことを示しています。
`Iterator`トレイトを実装するものは、`Item`の具体的な型を指定し、`next`メソッドは、
その具体的な型の値を含む`Option`を返します。

<!--
ジェネリクスはこうという話をしているのに、似ていると言っているのがどうも引っかかる
1 行目終わり、the latter allows と思われるが、直っていない
-->

<!--
Associated types might seem like a similar concept to generics, in that the
latter allow us to define a function without specifying what types it can
handle. So why use associated types?
-->

関連型は、ジェネリクスにより扱う型を指定せずに関数を定義できるという点でジェネリクスに似た概念のように思える可能性があります。
では、何故関連型を使用するのでしょうか？

<!--
Let’s examine the difference between the two concepts with an example from
Chapter 13 that implements the `Iterator` trait on the `Counter` struct. In
Listing 13-21, we specified that the `Item` type was `u32`:
-->

2 つの概念の違いを第 13 章から`Counter`構造体に`Iterator`トレイトを実装する例で調査しましょう。
リスト 13-21 で、`Item`型は`u32`だと指定しました：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

<!--
This syntax seems comparable to that of generics. So why not just define the
`Iterator` trait with generics, as shown in Listing 19-21?
-->

この記法は、ジェネリクスと比較可能に思えます。では、何故単純にリスト 19-21 のように、
`Iterator`トレイトをジェネリクスで定義しないのでしょうか？

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<!--
<span class="caption">Listing 19-21: A hypothetical definition of the
`Iterator` trait using generics</span>
-->

<span class="caption">リスト 19-21: ジェネリクスを使用した架空の`Iterator`トレイトの定義</span>

<!--
The difference is that when using generics, as in Listing 19-21, we must
annotate the types in each implementation; because we can also implement
`Iterator<String> for Counter` or any other type, we could have multiple
implementations of `Iterator` for `Counter`. In other words, when a trait has a
generic parameter, it can be implemented for a type multiple times, changing
the concrete types of the generic type parameters each time. When we use the
`next` method on `Counter`, we would have to provide type annotations to
indicate which implementation of `Iterator` we want to use.
-->

差異は、リスト 19-21 のようにジェネリクスを使用すると、各実装で型を注釈しなければならないことです;
`Iterator<String> for Counter`や他のどんな型にも実装することができるので、
`Counter`の`Iterator`の実装が複数できるでしょう。換言すれば、トレイトにジェネリックな引数があると、
毎回ジェネリックな型引数の具体的な型を変更してある型に対して複数回実装できるということです。
`Counter`に対して`next`メソッドを使用する際に、どの`Iterator`の実装を使用したいか型注釈をつけなければならないでしょう。

<!--
With associated types, we don’t need to annotate types because we can’t
implement a trait on a type multiple times. In Listing 19-20 with the
definition that uses associated types, we can only choose what the type of
`Item` will be once, because there can only be one `impl Iterator for Counter`.
We don’t have to specify that we want an iterator of `u32` values everywhere
that we call `next` on `Counter`.
-->

関連型なら、同じ型に対してトレイトを複数回実装できないので、型を注釈する必要はありません。
関連型を使用する定義があるリスト 19-20 では、`Item`の型は 1 回しか選択できませんでした。
1 つしか`impl Iterator for Counter`がないからです。`Counter`に`next`を呼び出す度に、
`u32`値のイテレータが欲しいと指定しなくてもよいわけです。

<!--
### Default Generic Type Parameters and Operator Overloading
-->

### デフォルトのジェネリック型引数と演算子オーバーロード

<!--
When we use generic type parameters, we can specify a default concrete type for
the generic type. This eliminates the need for implementors of the trait to
specify a concrete type if the default type works. The syntax for specifying a
default type for a generic type is `<PlaceholderType=ConcreteType>` when
declaring the generic type.
-->

ジェネリックな型引数を使用する際、ジェネリックな型に対して既定の具体的な型を指定できます。これにより、
既定の型が動くのなら、トレイトを実装する側が具体的な型を指定する必要を排除します。ジェネリックな型に既定の型を指定する記法は、
ジェネリックな型を宣言する際に`<PlaceholderType=ConcreteType>`です。

<!--
A great example of a situation where this technique is useful is with operator
overloading. *Operator overloading* is customizing the behavior of an operator
(such as `+`) in particular situations.
-->

このテクニックが有用になる場面の好例が、演算子オーバーロードです。*演算子オーバーロード*とは、
特定の状況で演算子 (`+`など) の振る舞いをカスタマイズすることです。

<!--
Rust doesn’t allow you to create your own operators or overload arbitrary
operators. But you can overload the operations and corresponding traits listed
in `std::ops` by implementing the traits associated with the operator. For
example, in Listing 19-22 we overload the `+` operator to add two `Point`
instances together. We do this by implementing the `Add` trait on a `Point`
struct:
-->

Rust では、独自の演算子を作ったり、任意の演算子をオーバーロードすることはできません。しかし、
演算子に紐づいたトレイトを実装することで`std::ops`に列挙された処理と対応するトレイトをオーバーロードできます。
例えば、リスト 19-22 で`+`演算子をオーバーロードして 2 つの`Point`インスタンスを足し合わせています。
`Point`構造体に`Add`トレイトを実装することでこれを行なっています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

<!--
<span class="caption">Listing 19-22: Implementing the `Add` trait to overload
the `+` operator for `Point` instances</span>
-->

<span class="caption">リスト 19-22: `Add`トレイトを実装して`Point`インスタンス用に`+`演算子をオーバーロードする</span>

<!--
The `add` method adds the `x` values of two `Point` instances and the `y`
values of two `Point` instances to create a new `Point`. The `Add` trait has an
associated type named `Output` that determines the type returned from the `add`
method.
-->

`add`メソッドは 2 つの`Point`インスタンスの`x`値と 2 つの`Point`インスタンスの`y`値を足します。
`Add`トレイトには、`add`メソッドから返却される型を決定する`Output`という関連型があります。

<!--
The default generic type in this code is within the `Add` trait. Here is its
definition:
-->

このコードの既定のジェネリック型は、`Add`トレイト内にあります。こちらがその定義です：

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

<!--
This code should look generally familiar: a trait with one method and an
associated type. The new part is `RHS=Self`: this syntax is called *default
type parameters*. The `RHS` generic type parameter (short for “right hand
side”) defines the type of the `rhs` parameter in the `add` method. If we don’t
specify a concrete type for `RHS` when we implement the `Add` trait, the type
of `RHS` will default to `Self`, which will be the type we’re implementing
`Add` on.
-->

このコードは一般的に馴染みがあるはずです：1 つのメソッドと関連型が 1 つあるトレイトです。
新しい部分は、`RHS=Self`です：この記法は、*デフォルト型引数*と呼ばれます。
RHS というジェネリックな型引数 ("right hand side": 右辺の省略形) が、`add`メソッドの`rhs`引数の型を定義しています。
`Add`トレイトを実装する際に`RHS`の具体的な型を指定しなければ、`RHS`の型は標準で`Self`になり、
これは`Add`を実装している型になります。

<!--
When we implemented `Add` for `Point`, we used the default for `RHS` because we
wanted to add two `Point` instances. Let’s look at an example of implementing
the `Add` trait where we want to customize the `RHS` type rather than using the
default.
-->

`Point`に`Add`を実装する際、2 つの`Point`インスタンスを足したかったので、`RHS`の規定を使用しました。
既定を使用するのではなく、`RHS`の型をカスタマイズしたくなる`Add`トレイトの実装例に目を向けましょう。

<!--
We have two structs `Millimeters` and `Meters`, holding values in different
units. We want to add values in millimeters to values in meters and have the
implementation of `Add` do the conversion correctly. We can implement `Add` for
`Millimeters` with `Meters` as the `RHS`, as shown in Listing 19-23.
-->

異なる単位で値を保持する構造体、`Millimeters`と`Meters`(それぞれ`ミリメートル`と`メートル`) が 2 つあります。
ミリメートルの値をメートルの値に足し、`Add`の実装に変換を正しくしてほしいです。
`Add`を`RHS`に`Meters`のある`Millimeters`に実装することができます。リスト 19-23 のように：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<!--
<span class="caption">Listing 19-23: Implementing the `Add` trait on
`Millimeters` to add `Millimeters` to `Meters`</span>
-->

<span class="caption">リスト 19-23: `Millimeters`に`Add`トレイトを実装して、`Meters`に`Millimeters`を足す</span>

<!--
To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the
value of the `RHS` type parameter instead of using the default of `Self`.
-->

`Millimeters`を`Meters`に足すため、`Self`という既定を使う代わりに`impl Add<Meters>`を指定して、
`RHS`型引数の値をセットしています。

<!--
You'll use default type parameters in two main ways:
-->

主に 2 通りの方法でデフォルト型引数を使用します：

<!--
* To extend a type without breaking existing code
* To allow customization in specific cases most users won’t need
-->

* 既存のコードを破壊せずに型を拡張する
* ほとんどのユーザは必要としない特定の場合でカスタマイズを可能にする

<!--
The standard library’s `Add` trait is an example of the second purpose:
usually, you’ll add two like types, but the `Add` trait provides the ability to
customize beyond that. Using a default type parameter in the `Add` trait
definition means you don’t have to specify the extra parameter most of the
time. In other words, a bit of implementation boilerplate isn’t needed, making
it easier to use the trait.
-->

標準ライブラリの`Add`トレイトは、2 番目の目的の例です：通常、2 つの似た型を足しますが、
`Add`トレイトはそれ以上にカスタマイズする能力を提供します。`Add`トレイト定義でデフォルト型引数を使用することは、
ほとんどの場合、追加の引数を指定しなくてもよいことを意味します。つまり、トレイトを使いやすくして、
ちょっとだけ実装の定型コードが必要なくなるのです。

<!--
The first purpose is similar to the second but in reverse: if you want to add a
type parameter to an existing trait, you can give it a default to allow
extension of the functionality of the trait without breaking the existing
implementation code.
-->

最初の目的は 2 番目に似ていますが、逆です：既存のトレイトに型引数を追加したいなら、既定を与えて、
既存の実装コードを破壊せずにトレイトの機能を拡張できるのです。

<!--
### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
-->

### 明確化のためのフルパス記法：同じ名前のメソッドを呼ぶ

<!--
Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor does Rust prevent you from implementing both traits
on one type. It’s also possible to implement a method directly on the type with
the same name as methods from traits.
-->

Rust において、別のトレイトのメソッドと同じ名前のメソッドがトレイトにあったり、両方のトレイトを 1 つの型に実装することを妨げるものは何もありません。
トレイトのメソッドと同じ名前のメソッドを直接型に実装することも可能です。

<!--
When calling methods with the same name, you'll need to tell Rust which one you
want to use. Consider the code in Listing 19-24 where we’ve defined two traits,
`Pilot` and `Wizard`, that both have a method called `fly`. We then implement
both traits on a type `Human` that already has a method named `fly` implemented
on it. Each `fly` method does something different.
-->

同じ名前のメソッドを呼ぶ際、コンパイラにどれを使用したいのか教える必要があるでしょう。両方とも`fly`というメソッドがある 2 つのトレイト、
`Pilot`と`Wizard`(`訳注`: パイロットと魔法使い) を定義したリスト 19-24 のコードを考えてください。
それから両方のトレイトを既に`fly`というメソッドが実装されている型`Human`(`訳注`: 人間) に実装します。
各`fly`メソッドは異なることをします。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        // キャプテンのお言葉
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        // 上がれ！
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        // *激しく腕を振る*
        println!("*waving arms furiously*");
    }
}
```

<!--
<span class="caption">Listing 19-24: Two traits are defined to have a `fly`
method and are implemented on the `Human` type, and a `fly` method is
implemented on `Human` directly</span>
-->

<span class="caption">リスト 19-24: 2 つのトレイトに`fly`があるように定義され、`Human`に実装されつつ、
    `fly`メソッドは`Human`に直接にも実装されている</span>

<!--
When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 19-25.
-->

`Human`のインスタンスに対して`fly`を呼び出すと、コンパイラは型に直接実装されたメソッドを標準で呼び出します。
リスト 19-25 のようにですね：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    person.fly();
}
```

<!--
<span class="caption">Listing 19-25: Calling `fly` on an instance of
`Human`</span>
-->

<span class="caption">リスト 19-25: `Human`のインスタンスに対して`fly`を呼び出す</span>

<!--
Running this code will print `*waving arms furiously*`, showing that Rust
called the `fly` method implemented on `Human` directly.
-->

このコードを実行すると、`*waving arms furiously*`と出力され、コンパイラが`Human`に直接実装された`fly`メソッドを呼んでいることを示しています。

<!--
To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait,
we need to use more explicit syntax to specify which `fly` method we mean.
Listing 19-26 demonstrates this syntax.
-->

`Pilot`トレイトか、`Wizard`トレイトの`fly`メソッドを呼ぶためには、
より明示的な記法を使用して、どの`fly`メソッドを意図しているか指定する必要があります。
リスト 19-26 は、この記法をデモしています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

<!--
<span class="caption">Listing 19-26: Specifying which trait’s `fly` method we
want to call</span>
-->

<span class="caption">リスト 19-26: どのトレイトの`fly`メソッドを呼び出したいか指定する</span>

<!--
Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also write
`Human::fly(&person)`, which is equivalent to the `person.fly()` that we used
in Listing 19-26, but this is a bit longer to write if we don’t need to
disambiguate.
-->

メソッド名の前にトレイト名を指定すると、コンパイラにどの`fly`の実装を呼び出したいか明確化できます。
また、`Human::fly(&person)`と書くこともでき、リスト 19-26 で使用した`person.fly()`と等価ですが、
こちらの方は明確化する必要がないなら、ちょっと記述量が増えます。

<!--
Running this code prints the following:
-->

このコードを実行すると、こんな出力がされます：

```text
This is your captain speaking.
Up!
*waving arms furiously*
```

<!--
Because the `fly` method takes a `self` parameter, if we had two *types* that
both implement one *trait*, Rust could figure out which implementation of a
trait to use based on the type of `self`.
-->

`fly`メソッドは`self`引数を取るので、1 つの*トレイト*を両方実装する*型*が 2 つあれば、
コンパイラには、`self`の型に基づいてどのトレイトの実装を使うべきかわかるでしょう。

<!--
However, associated functions that are part of traits don’t have a `self`
parameter. When two types in the same scope implement that trait, Rust can’t
figure out which type you mean unless you use *fully qualified syntax*. For
example, the `Animal` trait in Listing 19-27 has the associated function
`baby_name`, the implementation of `Animal` for the struct `Dog`, and the
associated function `baby_name` defined on `Dog` directly.
-->

しかしながら、トレイトの一部になる関連関数には`self`引数がありません。同じスコープの 2 つの型がそのトレイトを実装する場合、
*フルパス記法*(fully qualified syntax) を使用しない限り、どの型を意図しているかコンパイラは推論できません。例えば、
リスト 19-27 の`Animal`トレイトには、関連関数`baby_name`、構造体`Dog`の`Animal`の実装、
`Dog`に直接定義された関連関数`baby_name`があります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット (Wikipedia によると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

fn main() {
    // 赤ちゃん犬は{}と呼ばれる
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

<!--
<span class="caption">Listing 19-27: A trait with an associated function and a
type with an associated function of the same name that also implements the
trait</span>
-->

<span class="caption">リスト 19-27: 関連関数のあるトレイトとそのトレイトも実装し、同じ名前の関連関数がある型</span>

<!--
This code is for an animal shelter that wants to name all puppies Spot, which
is implemented in the `baby_name` associated function that is defined on `Dog`.
The `Dog` type also implements the trait `Animal`, which describes
characteristics that all animals have. Baby dogs are called puppies, and that
is expressed in the implementation of the `Animal` trait on `Dog` in the
`baby_name` function associated with the `Animal` trait.
-->

このコードは、全ての子犬をスポットと名付けたいアニマル・シェルター(`訳注`: 身寄りのないペットを保護する保健所みたいなところ) 用で、
`Dog`に定義された`baby_name`関連関数で実装されています。`Dog`型は、トレイト`Animal`も実装し、
このトレイトは全ての動物が持つ特徴を記述します。赤ちゃん犬は子犬と呼ばれ、
それが`Dog`の`Animal`トレイトの実装の`Animal`トレイトと紐づいた`base_name`関数で表現されています。

<!--
In `main`, we call the `Dog::baby_name` function, which calls the associated
function defined on `Dog` directly. This code prints the following:
-->

`main`で、`Dog::baby_name`関数を呼び出し、直接`Dog`に定義された関連関数を呼び出しています。
このコードは以下のような出力をします：

```text
A baby dog is called a Spot
```

<!--
This output isn’t what we wanted. We want to call the `baby_name` function that
is part of the `Animal` trait that we implemented on `Dog` so the code prints
`A baby dog is called a puppy`. The technique of specifying the trait name that
we used in Listing 19-26 doesn’t help here; if we change `main` to the code in
Listing 19-28, we’ll get a compilation error.
-->

この出力は、欲しかったものではありません。`Dog`に実装した`Animal`トレイトの一部の`baby_name`関数を呼び出したいので、
コードは`A baby dog is called a puppy`と出力します。リスト 19-26 で使用したトレイト名を指定するテクニックは、
ここでは役に立ちません; `main`をリスト 19-28 のようなコードに変更したら、コンパイルエラーになるでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

<!--
<span class="caption">Listing 19-28: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>
-->

<span class="caption">リスト 19-28: `Animal`トレイトの`baby_name`関数を呼び出そうとするも、コンパイラにはどの実装を使うべきかわからない</span>

<!--
Because `Animal::baby_name` is an associated function rather than a method, and
thus doesn’t have a `self` parameter, Rust can’t figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:
-->

`Animal::baby_name`はメソッドではなく関連関数であり、故に`self`引数がないので、どの`Animal::baby_name`が欲しいのか、
コンパイラには推論できません。こんなコンパイルエラーが出るでしょう：

```text
error[E0283]: type annotations required: cannot resolve `_: Animal`
(エラー: 型注釈が必要です：`_: Animal`を解決できません)
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^
   |
   = note: required by `Animal::baby_name`
   (注釈：`Animal::baby_name`に必要です)
```

<!--
To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog`, we need to use fully qualified syntax. Listing 19-29
demonstrates how to use fully qualified syntax.
-->

`Dog`に対して`Animal`実装を使用したいと明確化し、コンパイラに指示するには、フルパス記法を使う必要があります。
リスト 19-29 は、フルパス記法を使用する方法をデモしています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# trait Animal {
#     fn baby_name() -> String;
# }
#
# struct Dog;
#
# impl Dog {
#     fn baby_name() -> String {
#         String::from("Spot")
#     }
# }
#
# impl Animal for Dog {
#     fn baby_name() -> String {
#         String::from("puppy")
#     }
# }
#
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

<!--
<span class="caption">Listing 19-29: Using fully qualified syntax to specify
that we want to call the `baby_name` function from the `Animal` trait as
implemented on `Dog`</span>
-->

<span class="caption">リスト 19-29: フルパス記法を使って`Dog`に実装されているように、
    `Animal`トレイトからの`baby_name`関数を呼び出したいと指定する</span>

<!--
We’re providing Rust with a type annotation within the angle brackets, which
indicates we want to call the `baby_name` method from the `Animal` trait as
implemented on `Dog` by saying that we want to treat the `Dog` type as an
`Animal` for this function call. This code will now print what we want:
-->

コンパイラに山カッコ内で型注釈を提供し、これは、この関数呼び出しでは`Dog`型を`Animal`として扱いたいと宣言することで、
`Dog`に実装されたように、`Animal`トレイトの`baby_name`メソッドを呼び出したいと示唆しています。
もうこのコードは、望み通りの出力をします：

```text
A baby dog is called a puppy
```

<!--
In general, fully qualified syntax is defined as follows:
-->

一般的に、フルパス記法は、以下のように定義されています：

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

<!--
For associated functions, there would not be a `receiver`: there would only be
the list of other arguments. You could use fully qualified syntax everywhere
that you call functions or methods. However, you’re allowed to omit any part of
this syntax that Rust can figure out from other information in the program. You
only need to use this more verbose syntax in cases where there are multiple
implementations that use the same name and Rust needs help to identify which
implementation you want to call.
-->

関連関数では、`receiver`がないでしょう：他の引数のリストがあるだけでしょう。関数やメソッドを呼び出す箇所全部で、
フルパス記法を使用することもできるでしょうが、プログラムの他の情報からコンパイラが推論できるこの記法のどの部分も省略することが許容されています。
同じ名前を使用する実装が複数あり、どの実装を呼び出したいかコンパイラが特定するのに助けが必要な場合だけにこのより冗長な記法を使用する必要があるのです。

<!--
### Using Supertraits to Require One Trait’s Functionality Within Another Trait
-->

### スーパートレイトを使用して別のトレイト内で、あるトレイトの機能を必要とする

<!--
Sometimes, you might need one trait to use another trait’s functionality. In
this case, you need to rely on the dependent trait's also being implemented.
The trait you rely on is a *supertrait* of the trait you’re implementing.
-->

時として、あるトレイトに別のトレイトの機能を使用させる必要がある可能性があります。この場合、
依存するトレイトも実装されることを信用する必要があります。信用するトレイトは、実装しているトレイトの*スーパートレイト*です。

<!--
For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a value framed in asterisks. That is,
given a `Point` struct that implements `Display` to result in `(x, y)`, when we
call `outline_print` on a `Point` instance that has `1` for `x` and `3` for
`y`, it should print the following:
-->

例えば、アスタリスクをフレームにする値を出力する`outline_print`メソッドがある`OutlinePrint`トレイトを作りたくなったとしましょう。
つまり、`Display`を実装し、`(x, y)`という結果になる`Point`構造体が与えられて、
`x`が`1`、`y`が`3`の`Point`インスタンスに対して`outline_print`を呼び出すと、以下のような出力をするはずです：

```text
**********
*        *
* (1, 3) *
*        *
**********
```

<!--
In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. Therefore, we need to specify that the `OutlinePrint` trait will
work only for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This technique is similar to adding a trait bound to
the trait. Listing 19-30 shows an implementation of the `OutlinePrint` trait.
-->

`outline_print`の実装では、`Display`トレイトの機能を使用したいです。故に、`Display`も実装する型に対してだけ`OutlinePrint`が動くと指定し、
`OutlinePrint`が必要とする機能を提供する必要があるわけです。トレイト定義で`OutlinePrint: Display`と指定することで、
そうすることができます。このテクニックは、トレイトにトレイト境界を追加することに似ています。
リスト 19-30 は、`OutlinePrint`トレイトの実装を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

<!--
<span class="caption">Listing 19-30: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>
-->

<span class="caption">リスト 19-30: `Display`からの機能を必要とする`OutlinePrint`トレイトを実装する</span>

<!--
Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding a
colon and specifying `Display` trait after the trait name, we’d get an
error saying that no method named `to_string` was found for the type `&Self` in
the current scope.
-->

`OutlinePrint`は`Display`トレイトを必要とすると指定したので、`Display`を実装するどんな型にも自動的に実装される`to_string`関数を使えます。
トレイト名の後にコロンと`Display`トレイトを追加せずに`to_string`を使おうとしたら、
現在のスコープで型`&Self`に`to_string`というメソッドは存在しないというエラーが出るでしょう。

<!--
Let’s see what happens when we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:
-->

`Display`を実装しない型、`Point`構造体などに`OutlinePrint`を実装しようとしたら、何が起きるか確認しましょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

<!--
We get an error saying that `Display` is required but not implemented:
-->

`Display`が必要だけれども、実装されていないというエラーが出ます：

```text
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter;
try using `:?` instead if you are using a format string
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
```

<!--
To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:
-->

これを修正するために、`Point`に`Display`を実装し、`OutlinePrint`が必要とする制限を満たします。
こんな感じで：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

<!--
Then implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.
-->

そうすれば、`Point`に`OutlinePrint`トレイトを実装してもコンパイルは成功し、
`Point`インスタンスに対して`outline_print`を呼び出し、アスタリスクのふちの中に表示することができます。

<!--
### Using the Newtype Pattern to Implement External Traits on External Types
-->

### ニュータイプパターンを使用して外部の型に外部のトレイトを実装する

<!--
In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the
orphan rule that states we’re allowed to implement a trait on a type as long as
either the trait or the type are local to our crate. It’s possible to get
around this restriction using the *newtype pattern*, which involves creating a
new type in a tuple struct. (We covered tuple structs in the “Using Tuple
Structs without Named Fields to Create Different Types” section of Chapter 5.)
The tuple struct will have one field and be a thin wrapper around the type we
want to implement a trait for. Then the wrapper type is local to our crate, and
we can implement the trait on the wrapper. *Newtype* is a term that originates
from the Haskell programming language. There is no runtime performance penalty
for using this pattern, and the wrapper type is elided at compile time.
-->

第 10 章の「型にトレイトを実装する」節で、トレイトか型がクレートにローカルな限り、型にトレイトを実装できると述べるオーファンルールについて触れました。
*ニュータイプパターン*を使用してこの制限を回避することができ、タプル構造体に新しい型を作成することになります。
(タプル構造体については、第 5 章の「異なる型を生成する名前付きフィールドのないタプル構造体を使用する」節で講義しました。)
タプル構造体は 1 つのフィールドを持ち、トレイトを実装したい型の薄いラッパになるでしょう。そして、
ラッパの型はクレートにローカルなので、トレイトをラッパに実装できます。*ニュータイプ*という用語は、
Haskell プログラミング言語に端を発しています。このパターンを使用するのに実行時のパフォーマンスを犠牲にすることはなく、
ラッパ型はコンパイル時に省かれます。

<!--
As an example, let’s say we want to implement `Display` on `Vec<T>`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct
that holds an instance of `Vec<T>`; then we can implement `Display` on
`Wrapper` and use the `Vec<T>` value, as shown in Listing 19-31.
-->

例として、`Vec<T>`に`Display`を実装したいとしましょう。`Display`トレイトも`Vec<T>`型もクレートの外で定義されているので、
直接それを行うことはオーファンルールにより妨げられます。`Vec<T>`のインスタンスを保持する`Wrapper`構造体を作成できます;
そして、`Wrapper`に`Display`を実装し、`Vec<T>`値を使用できます。リスト 19-31 のように。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

<!--
<span class="caption">Listing 19-31: Creating a `Wrapper` type around
`Vec<String>` to implement `Display`</span>
-->

<span class="caption">リスト 19-31: `Vec<String>`の周りに`Wrapper`を作成して`Display`を実装する</span>

<!--
The implementation of `Display` uses `self.0` to access the inner `Vec<T>`,
because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.
-->

`Display`の実装は、`self.0`で中身の`Vec<T>`にアクセスしています。`Wrapper`はタプル構造体で、
`Vec<T>`がタプルの添え字 0 の要素だからです。それから、`Wrapper`に対して`Display`型の機能を使用できます。

<!--
The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding. We would have to implement
all the methods of `Vec<T>` directly on `Wrapper` such that the methods
delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a
`Vec<T>`. If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the “Treating Smart
Pointers like Regular References with the `Deref` Trait” section) on the
`Wrapper` to return the inner type would be a solution. If we don’t want the
`Wrapper` type to have all the methods of the inner type—for example, to
restrict the `Wrapper` type’s behavior—we would have to implement just the
methods we do want manually.
-->

このテクニックを使用する欠点は、`Wrapper`が新しい型なので、保持している値のメソッドがないことです。
`self.0`に委譲して、`Wrapper`を`Vec<T>`と全く同様に扱えるように、`Wrapper`に直接`Vec<T>`の全てのメソッドを実装しなければならないでしょう。
内部の型が持つ全てのメソッドを新しい型に持たせたいなら、
`Deref`トレイト (第 15 章の「`Deref`トレイトでスマートポインタを普通の参照のように扱う」節で議論しました) を`Wrapper`に実装して、
内部の型を返すことは解決策の 1 つでしょう。内部の型のメソッド全部を`Wrapper`型に持たせたくない (例えば、`Wrapper`型の機能を制限するなど) なら、
本当に欲しいメソッドだけを手動で実装しなければならないでしょう。

<!--
Now you know how the newtype pattern is used in relation to traits; it’s also a
useful pattern even when traits are not involved. Let’s switch focus and look
at some advanced ways to interact with Rust’s type system.
-->

もう、トレイトに関してニュータイプパターンが使用される方法を知りました; トレイトが関連しなくても、
有用なパターンでもあります。焦点を変更して、Rust の型システムと相互作用する一部の高度な方法を見ましょう。
