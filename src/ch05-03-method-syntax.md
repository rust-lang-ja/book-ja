<!--
## Method Syntax
-->

## メソッド記法

<!--
*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can have parameters and a return value, and they contain some
code that is run when they’re called from somewhere else. However, methods are
different from functions in that they’re defined within the context of a struct
(or an enum or a trait object, which we cover in Chapters 6 and 17,
respectively), and their first parameter is always `self`, which represents the
instance of the struct the method is being called on.
-->

*メソッド*は関数に似ています：`fn`キーワードと名前で宣言されるし、引数と返り値があるし、
どこか別の場所で呼び出された時に実行されるコードを含みます。ところが、
メソッドは構造体の文脈 (あるいは enum かトレイトオブジェクトの。これらについては各々第 6 章と 17 章で解説します) で定義されるという点で、
関数とは異なり、最初の引数は必ず`self`になり、これはメソッドが呼び出されている構造体インスタンスを表します。

<!--
### Defining Methods
-->

### メソッドを定義する

<!--
Let’s change the `area` function that has a `Rectangle` instance as a parameter
and instead make an `area` method defined on the `Rectangle` struct, as shown
in Listing 5-13.
-->

`Rectangle`インスタンスを引数に取る`area`関数を変え、代わりに`Rectangle`構造体上に`area`メソッドを作りましょう。
リスト 5-13 に示した通りですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

<!--
<span class="caption">Listing 5-13: Defining an `area` method on the
`Rectangle` struct</span>
-->

<span class="caption">リスト 5-13: `Rectangle`構造体上に`area`メソッドを定義する</span>

<!--
To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block. Then we move the `area` function within the `impl`
curly brackets and change the first (and in this case, only) parameter to be
`self` in the signature and everywhere within the body. In `main`, where we
called the `area` function and passed `rect1` as an argument, we can instead
use *method syntax* to call the `area` method on our `Rectangle` instance.
The method syntax goes after an instance: we add a dot followed by the method
name, parentheses, and any arguments.
-->

`Rectangle`の文脈内で関数を定義するには、`impl`(implementation; 実装) ブロックを始めます。
それから`area`関数を`impl`の波かっこ内に移動させ、最初の (今回は唯一の) 引数をシグニチャ内と本体内全てで`self`に変えます。
`area`関数を呼び出し、`rect1`を引数として渡す`main`では、代替としてメソッド記法を使用して、
`Rectangle`インスタンスの`area`メソッドを呼び出せます。メソッド記法は、インスタンスの後に続きます：
ドット、メソッド名、かっこ、そして引数と続くわけです。

<!--
In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`
because Rust knows the type of `self` is `Rectangle` due to this method being
inside the `impl Rectangle` context. Note that we still need to use the `&`
before `self`, just like we did in `&Rectangle`. Methods can take ownership of
`self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably,
just as they can any other parameter.
-->

`area`のシグニチャでは、`rectangle: &Rectangle`の代わりに`&self`を使用しています。
というのも、コンパイラは、このメソッドが`impl Rectangle`という文脈内に存在するために、
`self`の型が`Rectangle`であると把握しているからです。`&Rectangle`と同様に、
`self`の直前に`&`を使用していることに注意してください。メソッドは、`self`の所有権を奪ったり、
ここでしているように不変で`self`を借用したり、可変で`self`を借用したりできるのです。
他の引数と全く同じですね。

<!--
We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to read the
data in the struct, not write to it. If we wanted to change the instance that
we’ve called the method on as part of what the method does, we’d use `&mut
self` as the first parameter. Having a method that takes ownership of the
instance by using just `self` as the first parameter is rare; this technique is
usually used when the method transforms `self` into something else and we want
to prevent the caller from using the original instance after the transformation.
-->

ここで`&self`を選んでいるのは、関数バージョンで`&Rectangle`を使用していたのと同様の理由です：
所有権はいらず、構造体のデータを読み込みたいだけで、書き込む必要はないわけです。
メソッドの一部でメソッドを呼び出したインスタンスを変更したかったら、第 1 引数に`&mut self`を使用するでしょう。
`self`だけを第 1 引数にしてインスタンスの所有権を奪うメソッドを定義することは稀です; このテクニックは通常、
メソッドが`self`を何か別のものに変形し、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合に使用されます。

<!--
The main benefit of using methods instead of functions, in addition to using
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type in one `impl` block rather than making future users of our
code search for capabilities of `Rectangle` in various places in the library we
provide.
-->

関数の代替としてメソッドを使う主な利点は、メソッド記法を使用して全メソッドのシグニチャで`self`の型を繰り返す必要がなくなる以外だと、
体系化です。コードの将来的な利用者に`Rectangle`の機能を提供しているライブラリ内の各所でその機能を探させるのではなく、
この型のインスタンスでできることを一つの`impl`ブロックにまとめあげています。

<!--
例によって、以下の節では、引用ブロックの後に和訳を示します
-->

<!--
### Where’s the `->` Operator?

In C and C++, two different operators are used for calling methods: you use
`.` if you’re calling a method on the object directly and `->` if you're
calling the method on a pointer to the object and need to dereference the
pointer first. In other words, if `object` is a pointer,
`object->something()` is similar to `(*object).something()`.

Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a
feature called *automatic referencing and dereferencing*. Calling methods is
one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with `object.something()`, Rust
automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of
the method. In other words, the following are the same:

```rust
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
#
# impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
#
#        f64::sqrt(x_squared + y_squared)
#    }
# }
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much cleaner. This automatic referencing behavior works
because methods have a clear receiver—the type of `self`. Given the receiver
and name of a method, Rust can figure out definitively whether the method is
reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
that Rust makes borrowing implicit for method receivers is a big part of
making ownership ergonomic in practice.
-->

> ### `->`演算子はどこに行ったの？
>
> C と C++では、メソッド呼び出しには 2 種類の異なる演算子が使用されます：
> オブジェクトに対して直接メソッドを呼び出すのなら、`.`を使用するし、オブジェクトのポインタに対してメソッドを呼び出し、
> 先にポインタを参照外しする必要があるなら、`->`を使用するわけです。
> 言い換えると、`object`がポインタなら、`object->something()`は、`(*object).something()`と同等なのです。
>
> Rust には`->`演算子の代わりとなるようなものはありません; その代わり、Rust には、
> *自動参照および参照外し*という機能があります。Rust においてメソッド呼び出しは、
> この動作が行われる数少ない箇所なのです。
>
> 動作方法はこうです：`object.something()`とメソッドを呼び出すと、
> コンパイラは`object`がメソッドのシグニチャと合致するように、自動で`&`か`&mut`、`*`を付与するのです。
> 要するに、以下のコードは同じものです：
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 前者の方がずっと明確です。メソッドには自明な受け手 (`self`の型) がいるので、この自動参照機能は動作するのです。
> 受け手とメソッド名が与えられれば、コンパイラは確実にメソッドが読み込み専用 (`&self`) か、書き込みもする (`&mut self`) のか、
> 所有権を奪う (`self`) のか判断できるわけです。メソッドの受け手に関して借用が明示されないというのが、
> 所有権を実際に使うのが Rust において簡単である大きな理由です。

<!--
### Methods with More Parameters
-->

### より引数の多いメソッド

<!--
Let’s practice using methods by implementing a second method on the `Rectangle`
struct. This time, we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self`; otherwise it should return `false`. That is, we want to be able
to write the program shown in Listing 5-14, once we’ve defined the `can_hold`
method.
-->

`Rectangle`構造体に 2 番目のメソッドを実装して、メソッドを使う鍛錬をしましょう。今回は、`Rectangle`のインスタンスに、
別の`Rectangle`のインスタンスを取らせ、2 番目の`Rectangle`が`self`に完全にはめ込まれたら、`true`を返すようにしたいのです;
そうでなければ、`false`を返すべきです。つまり、一旦`can_hold`メソッドを定義したら、
リスト 5-14 のようなプログラムを書けるようになりたいのです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // rect1 に rect2 ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

<!--
<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>
-->

<span class="caption">リスト 5-14: まだ書いていない`can_hold`メソッドを使用する</span>

<!--
And the expected output would look like the following, because both dimensions
of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider than
`rect1`:
-->

そして、予期される出力は以下のようになります。なぜなら、`rect2`の各寸法は`rect1`よりも小さいものの、
`rect3`は`rect1`より幅が広いからです：

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

<!--
We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are both greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.
-->

メソッドを定義したいことはわかっているので、`impl Rectangle`ブロック内での話になります。
メソッド名は、`can_hold`になり、引数として別の`Rectangle`を不変借用で取るでしょう。
メソッドを呼び出すコードを見れば、引数の型が何になるかわかります：`rect1.can_hold(&rect2)`は、
`&rect2`、`Rectangle`のインスタンスである`rect2`への不変借用を渡しています。
これは道理が通っています。なぜなら、`rect2`を読み込む (書き込みではなく。この場合、可変借用が必要になります) だけでよく、
`can_hold`メソッドを呼び出した後にも`rect2`が使えるよう、所有権を`main`に残したままにしたいからです。
`can_hold`の返り値は、boolean になり、メソッドの中身は、`self`の幅と高さがもう一つの`Rectangle`の幅と高さよりも、
それぞれ大きいことを確認します。リスト 5-13 の`impl`ブロックに新しい`can_hold`メソッドを追記しましょう。
リスト 5-15 に示した通りです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<!--
<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>
-->

<span class="caption">リスト 5-15: 別の`Rectangle`のインスタンスを引数として取る`can_hold`メソッドを、
`Rectangle`に実装する</span>

<!--
When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.
-->

このコードをリスト 5-14 の`main`関数と合わせて実行すると、望み通りの出力が得られます。
メソッドは、`self`引数の後にシグニチャに追加した引数を複数取ることができ、
その引数は、関数の引数と同様に動作するのです。

<!--
### Associated Functions
-->

### 関連関数

<!--
Another useful feature of `impl` blocks is that we’re allowed to define
functions within `impl` blocks that *don’t* take `self` as a parameter. These
are called *associated functions* because they’re associated with the struct.
They’re still functions, not methods, because they don’t have an instance of
the struct to work with. You’ve already used the `String::from` associated
function.
-->

`impl`ブロックの別の有益な機能は、`impl`ブロック内に`self`を引数に取ら*ない*関数を定義できることです。
これは、構造体に関連付けられているので、*関連関数*と呼ばれます。それでも、関連関数は関数であり、メソッドではありません。
というのも、対象となる構造体のインスタンスが存在しないからです。もう`String::from`という関連関数を使用したことがありますね。

<!--
Associated functions are often used for constructors that will return a new
instance of the struct. For example, we could provide an associated function
that would have one dimension parameter and use that as both width and height,
thus making it easier to create a square `Rectangle` rather than having to
specify the same value twice:
-->

関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用されます。例えば、一つの寸法を引数に取り、
長さと幅両方に使用する関連関数を提供することができ、その結果、同じ値を 2 回指定する必要なく、
正方形の`Rectangle`を生成しやすくすることができます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

<!--
To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in Chapter 7.
-->

この関連関数を呼び出すために、構造体名と一緒に`::`記法を使用します; 一例は`let sq = Rectangle::square(3);`です。
この関数は、構造体によって名前空間分けされています：`::`という記法は、関連関数とモジュールによって作り出される名前空間両方に使用されます。
モジュールについては第 7 章で議論します。

<!--
### Multiple `impl` Blocks
-->

### 複数の`impl`ブロック

<!--
Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method
in its own `impl` block.
-->

各構造体には、複数の`impl`ブロックを存在させることができます。例えば、リスト 5-15 はリスト 5-16 に示したコードと等価で、
リスト 5-16 では、各メソッドごとに`impl`ブロックを用意しています。

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<!--
<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>
-->

<span class="caption">リスト 5-16: 複数の`impl`ブロックを使用してリスト 5-15 を書き直す</span>

<!--
There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We'll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.
-->

ここでこれらのメソッドを個々の`impl`ブロックに分ける理由はないのですが、合法な書き方です。
複数の`impl`ブロックが有用になるケースは第 10 章で見ますが、そこではジェネリック型と、トレイトについて議論します。

<!--
## Summary
-->

## まとめ

<!--
Structs let us create custom types that are meaningful for your domain. By
using structs, we can keep associated pieces of data connected to each other
and name each piece to make our code clear. Methods let us specify the
behavior that instances of our structs have, and associated functions let you
namespace functionality that is particular to our struct without having an
instance available.
-->

構造体により、自分の領域で意味のある独自の型を作成することができます。構造体を使用することで、
関連のあるデータ片を相互に結合させたままにし、各部品に名前を付け、コードを明確にすることができます。
メソッドにより、構造体のインスタンスが行う動作を指定することができ、関連関数により、
構造体に特有の機能をインスタンスを利用することなく、名前空間分けすることができます。

<!--
But structs aren’t the only way we can create custom types: let’s turn to
Rust’s enum feature to add another tool to our toolbox.
-->

しかし、構造体だけが独自の型を作成する手段ではありません：Rust の enum 機能に目を向けて、
別の道具を道具箱に追加しましょう。
