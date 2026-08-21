<!--
## Method Syntax
-->

## メソッド記法

<!--

*Methods* are similar to functions: we declare them with the `fn` keyword and a
name, they can have parameters and a return value, and they contain some code
that’s run when the method is called from somewhere else. Unlike functions,
methods are defined within the context of a struct (or an enum or a trait
object, which we cover in [Chapter 6][enums] and [Chapter
17][trait-objects], respectively), and their first parameter is
always `self`, which represents the instance of the struct the method is being
called on.
-->

*メソッド*は関数に似ています: `fn`キーワードと名前を使って宣言され、引数と戻り値を持つことができ、
メソッドがどこか別の場所で呼び出された時に実行されるコードを含みます。
関数とは異なり、メソッドは構造体の文脈の中で定義されます（enumの文脈やトレイトオブジェクトの文脈の中でも定義されますが、これらについてはそれぞれ[6章][enums]と[17章][trait-objects]で解説します）。
最初の引数は必ず`self`になり、これはメソッドが呼び出されている構造体インスタンスを表します。

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
リスト5-13に示した通りですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<!--
<span class="caption">Listing 5-13: Defining an `area` method on the
`Rectangle` struct</span>
-->

<span class="caption">リスト5-13: `Rectangle`構造体上に`area`メソッドを定義する</span>

<!--
To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block for `Rectangle`. Everything within this `impl` block
will be associated with the `Rectangle` type. Then we move the `area` function
within the `impl` curly brackets and change the first (and in this case, only)
parameter to be `self` in the signature and everywhere within the body. In
`main`, where we called the `area` function and passed `rect1` as an argument,
we can instead use *method syntax* to call the `area` method on our `Rectangle`
instance. The method syntax goes after an instance: we add a dot followed by
the method name, parentheses, and any arguments.
-->

`Rectangle`の文脈内で関数を定義するには、`Rectangle`のための`impl`(implementation; 実装)ブロックを始めます。
この`impl`ブロック内のあらゆる定義は`Rectangle`に関連付けられたものとなるでしょう。
それから`area`関数を`impl`の波かっこ内に移動させ、最初の(今回は唯一の)引数をシグニチャ内と本体内全てで`self`に変えます。
`area`関数を呼び出し、`rect1`を引数として渡す`main`では、代替としてメソッド記法を使用して、
`Rectangle`インスタンスの`area`メソッドを呼び出せます。メソッド記法は、インスタンスの後に続きます:
ドット、メソッド名、かっこ、そして引数と続くわけです。

<!--
In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`.
The `&self` is actually short for `self: &Self`. Within an `impl` block, the
type `Self` is an alias for the type that the `impl` block is for. Methods must
have a parameter named `self` of type `Self` for their first parameter, so Rust
lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthand to
indicate that this method borrows the `Self` instance, just as we did in
`rectangle: &Rectangle`. Methods can take ownership of `self`, borrow `self`
immutably, as we’ve done here, or borrow `self` mutably, just as they can any
other parameter.
-->

`area`のシグニチャでは、`rectangle: &Rectangle`の代わりに`&self`を使用しています。
`&self`は実際のところ`self: &Self`の省略記法です。
`impl`ブロック内では、型`Self`はその`impl`ブロックの対象である型に対するエイリアスとして使えます。
メソッドはその第1引数として`self`という名前の`Self`型引数を持たなくてはならないので、
Rust処理系は第1引数の位置では`self`という名前のみに省略することを許可しているのです。
ただし、`self`省略記法の前の`&`は依然として必要であることに注意してください。
これは、`rectangle: &Rectangle`と書いたときと同様に、このメソッドが`Self`のインスタンスを借用することを示しています。
メソッドは、`self`の所有権を奪ったり、ここでしているように不変で`self`を借用したり、可変で`self`を借用したりできるのです。
他の引数と全く同じですね。

<!--
We chose `&self` here for the same reason we used `&Rectangle` in the function
version: we don’t want to take ownership, and we just want to read the data in
the struct, not write to it. If we wanted to change the instance that we’ve
called the method on as part of what the method does, we’d use `&mut self` as
the first parameter. Having a method that takes ownership of the instance by
using just `self` as the first parameter is rare; this technique is usually
used when the method transforms `self` into something else and you want to
prevent the caller from using the original instance after the transformation.
-->

ここで`&self`を選んでいるのは、関数バージョンで`&Rectangle`を使用していたのと同様の理由です:
所有権はいらず、構造体のデータを読み込みたいだけで、書き込む必要はないわけです。
メソッドの一部でメソッドを呼び出したインスタンスを変更したかったら、第1引数に`&mut self`を使用するでしょう。
`self`だけを第1引数にしてインスタンスの所有権を奪うメソッドを定義することは稀です; このテクニックは通常、
メソッドが`self`を何か別のものに変形し、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合に使用されます。

<!--
The main reason for using methods instead of functions, in addition to
providing method syntax and not having to repeat the type of `self` in every
method’s signature, is for organization. We’ve put all the things we can do
with an instance of a type in one `impl` block rather than making future users
of our code search for capabilities of `Rectangle` in various places in the
library we provide.
-->

関数の代替としてメソッドを使う主な理由は、メソッド記法を使用できるようにすることと、
すべてのメソッドのシグニチャでいちいち`self`の型を繰り返す必要がなくなることに加えて、体系化のためです。
コードの将来的な利用者に、私たちが提供するライブラリ内のあらゆる箇所から`Rectangle`の機能を探させるのではなく、
この型のインスタンスでできることを一つの`impl`ブロックにまとめあげています。

<!--
Note that we can choose to give a method the same name as one of the struct’s
fields. For example, we can define a method on `Rectangle` that is also named
`width`:
-->

メソッドには、構造体のフィールドと同じ名前を与えることもできることに注意してください。
例えば、`Rectangle`に`width`という名前のメソッドを定義することができます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

<!--
Here, we’re choosing to make the `width` method return `true` if the value in
the instance’s `width` field is greater than `0` and `false` if the value is
`0`: we can use a field within a method of the same name for any purpose. In
`main`, when we follow `rect1.width` with parentheses, Rust knows we mean the
method `width`. When we don’t use parentheses, Rust knows we mean the field
`width`.
-->

ここで`width`メソッドは、インスタンスの`width`フィールドの値が`0`より大きい場合に`true`を返し、値が`0`の場合は`false`を返すようにしています:
同名のメソッド内でも、フィールドをあらゆる目的のために使用することができます。
`main`では`rect1.width`の後に丸かっこを続けているので、処理系はメソッドの`width`を意図したものと認識します。
丸かっこを使わなければ、処理系はフィールドの`width`を意図したものと認識します。

<!--
Often, but not always, when we give a method the same name as a field we want
it to only return the value in the field and do nothing else. Methods like this
are called *getters*, and Rust does not implement them automatically for struct
fields as some other languages do. Getters are useful because you can make the
field private but the method public, and thus enable read-only access to that
field as part of the type’s public API. We will discuss what public and private
are and how to designate a field or method as public or private in [Chapter
7][public].
-->

フィールドの値を返すだけで他に何もしないメソッドに、フィールドと同じ名前を与えることが、常にではありませんが、よくあります。
このようなメソッドは*ゲッター*と呼ばれます。
他のいくつかの言語で行われているような自動的なゲッターの実装を、Rustが構造体フィールドに対して行うことはありません。
フィールドを非公開にしながらメソッドは公開するということができ、そうすることで、型の公開APIの一部としてフィールドへの読み込み専用アクセスを提供できるので、ゲッターは有用です。
公開と非公開とはなにか、そしてフィールドやメソッドを公開または非公開として指定する方法については、[7章][public]で議論します。

<!--
例によって、以下の節では、引用ブロックの後に和訳を示します
-->

<!--
> ### Where’s the `->` Operator?
>
> In C and C++, two different operators are used for calling methods: you use
> `.` if you’re calling a method on the object directly and `->` if you’re
> calling the method on a pointer to the object and need to dereference the
> pointer first. In other words, if `object` is a pointer,
> `object->something()` is similar to `(*object).something()`.
>
> Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a
> feature called *automatic referencing and dereferencing*. Calling methods is
> one of the few places in Rust that has this behavior.
>
> Here’s how it works: when you call a method with `object.something()`, Rust
> automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of
> the method. In other words, the following are the same:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 --
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
> The first one looks much cleaner. This automatic referencing behavior works
> because methods have a clear receiver—the type of `self`. Given the receiver
> and name of a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
> that Rust makes borrowing implicit for method receivers is a big part of
> making ownership ergonomic in practice.
-->

> ### `->`演算子はどこに行ったの？
>
> CとC++では、メソッド呼び出しには2種類の異なる演算子が使用されます:
> オブジェクトに対して直接メソッドを呼び出すのなら、`.`を使用するし、オブジェクトのポインタに対してメソッドを呼び出し、
> 先にポインタを参照外しする必要があるなら、`->`を使用するわけです。
> 言い換えると、`object`がポインタなら、`object->something()`は、`(*object).something()`と同等なのです。
>
> Rustには`->`演算子の代わりとなるようなものはありません; その代わり、Rustには、
> *自動参照および参照外し*という機能があります。Rustにおいてメソッド呼び出しは、
> この動作が行われる数少ない箇所なのです。
>
> 動作方法はこうです: `object.something()`とメソッドを呼び出すと、
> コンパイラは`object`がメソッドのシグニチャと合致するように、自動で`&`か`&mut`、`*`を付与するのです。
> 要するに、以下のコードは同じものです:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
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
> 前者の方がずっと明確です。メソッドには自明な受け手(`self`の型)がいるので、この自動参照機能は動作するのです。
> 受け手とメソッド名が与えられれば、コンパイラは確実にメソッドが読み込み専用(`&self`)か、書き込みもする(`&mut self`)のか、
> 所有権を奪う(`self`)のか判断できるわけです。メソッドの受け手に関して借用が明示されないというのが、
> 所有権を実際に使うのがRustにおいて簡単である大きな理由です。

<!--
### Methods with More Parameters
-->

### より引数の多いメソッド

<!--
Let’s practice using methods by implementing a second method on the `Rectangle`
struct. This time we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self` (the first `Rectangle`); otherwise, it should return `false`.
That is, once we’ve defined the `can_hold` method, we want to be able to write
the program shown in Listing 5-14.
-->

`Rectangle`構造体に2番目のメソッドを実装して、メソッドを使う練習をしましょう。
今回は`Rectangle`のインスタンスに別の`Rectangle`のインスタンスを受け取らせ、2番目の`Rectangle`が`self`（1番目の`Rectangle`）に完全に収まるなら、`true`を返すようにしたいとしましょう;
そうでなければ、`false`を返すべきです。つまり、一旦`can_hold`メソッドを定義したら、リスト5-14のようなプログラムを書けるようにしたいのです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<!--
<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>
-->

<span class="caption">リスト5-14: まだ書いていない`can_hold`メソッドを使用する</span>

<!--
The expected output would look like the following because both dimensions of
`rect2` are smaller than the dimensions of `rect1`, but `rect3` is wider than
`rect1`:
-->

予期される出力は以下のようになります。なぜなら、`rect2`の各寸法は`rect1`よりも小さいものの、
`rect3`は`rect1`より幅が広いからです:

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
`self` are greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.
-->

メソッドを定義したいことはわかっているので、`impl Rectangle`ブロック内での話になります。
メソッド名は、`can_hold`になり、引数として別の`Rectangle`を不変借用で取るでしょう。
メソッドを呼び出すコードを見れば、引数の型が何になるかわかります: `rect1.can_hold(&rect2)`は、
`&rect2`、`Rectangle`のインスタンスである`rect2`への不変借用を渡しています。
これは道理が通っています。なぜなら、`rect2`を読み込む(書き込みではなく。この場合、可変借用が必要になります)だけでよく、
`can_hold`メソッドを呼び出した後にも`rect2`が使えるよう、所有権を`main`に残したままにしたいからです。
`can_hold`の返り値は、booleanになり、メソッドの中身は、`self`の幅と高さがもう一つの`Rectangle`の幅と高さよりも、
それぞれ大きいことを確認します。リスト5-13の`impl`ブロックに新しい`can_hold`メソッドを追記しましょう。
リスト5-15に示した通りです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>
-->

<span class="caption">リスト5-15: 別の`Rectangle`のインスタンスを引数として取る`can_hold`メソッドを、
`Rectangle`に実装する</span>

<!--
When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.
-->

このコードをリスト5-14の`main`関数と合わせて実行すると、望み通りの出力が得られます。
メソッドは、`self`引数の後にシグニチャに追加した引数を複数取ることができ、
その引数は、関数の引数と同様に動作するのです。

<!--
### Associated Functions
-->

### 関連関数

<!--
All functions defined within an `impl` block are called *associated functions*
because they’re associated with the type named after the `impl`. We can define
associated functions that don’t have `self` as their first parameter (and thus
are not methods) because they don’t need an instance of the type to work with.
We’ve already used one function like this: the `String::from` function that’s
defined on the `String` type.
-->

`impl`ブロック内で定義されたすべての関数は、`impl`の後に書かれた型に関連付けられているので、*関連関数*と呼ばれます。
対象の型のインスタンスを必要としないために、`self`を第1引数として持たない（つまりメソッドではない）関連関数を定義することもできます。
このような関数は、すでにひとつ使用しています: `String::from`関数は`String`型の上に定義された関数です。

<!--
Associated functions that aren’t methods are often used for constructors that
will return a new instance of the struct. These are often called `new`, but
`new` isn’t a special name and isn’t built into the language. For example, we
could choose to provide an associated function named `square` that would have
one dimension parameter and use that as both width and height, thus making it
easier to create a square `Rectangle` rather than having to specify the same
value twice:
-->

メソッドでない関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用されます。
これらにはしばしば`new`という名前が付けられますが、`new`は特別な名前ではなく、言語に組み込まれたものでもありません。
例えば、一つの寸法を引数に取り、それを高さと幅の両方として使用する関連関数`square`を提供してもよいでしょう。
そうすれば同じ値を2回指定する必要なく、正方形の`Rectangle`を生成しやすくすることができます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

<!--
The `Self` keywords in the return type and in the body of the function are
aliases for the type that appears after the `impl` keyword, which in this case
is `Rectangle`.
-->

戻り値型と関数本体内のの`Self` キーワードは`impl` キーワードの後に現れる型に対するエイリアスで、今回の場合は`Rectangle`です。

<!--
To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules. We’ll discuss modules in [Chapter
7][modules].
-->

この関連関数を呼び出すために、構造体名と一緒に`::`記法を使用します; 一例は`let sq = Rectangle::square(3);`です。
この関数は、構造体によって名前空間分けされています: `::`という記法は、関連関数とモジュールによって作り出される名前空間両方に使用されます。
モジュールについては[第7章][modules]で議論します。

<!--
### Multiple `impl` Blocks
-->

### 複数の`impl`ブロック

<!--
Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method in
its own `impl` block.
-->

各構造体には、複数の`impl`ブロックを存在させることができます。例えば、リスト5-15はリスト5-16に示したコードと等価で、
リスト5-16では、各メソッドごとに`impl`ブロックを用意しています。

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>
-->

<span class="caption">リスト5-16: 複数の`impl`ブロックを使用してリスト5-15を書き直す</span>

<!--
There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.
-->

ここでこれらのメソッドを個々の`impl`ブロックに分ける理由はないのですが、合法な書き方です。
複数の`impl`ブロックが有用になるケースは第10章で見ますが、そこではジェネリック型と、トレイトについて議論します。

<!--
## Summary
-->

## まとめ

<!--
Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.
-->

構造体により、ドメインにとって意味のある独自の型を作成することができます。構造体を使用することで、
関連のあるデータ片を相互に結合させたままにし、各部品に名前を付け、コードを明確にすることができます。
`impl`ブロック内では型に関連付けられた関数を定義することができ、メソッドは構造体のインスタンスが持つ振る舞いを規定することができる関連関数の一種です。

<!--
But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.
-->

しかし、構造体だけが独自の型を作成する手段ではありません: Rustのenum機能に目を向けて、
別の道具を道具箱に追加しましょう。

<!--
[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
-->

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#パスをpubキーワードで公開する
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
