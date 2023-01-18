<!--
## Treating Smart Pointers Like Regular References with the `Deref` Trait
-->

## `Deref`トレイトでスマートポインタを普通の参照のように扱う

<!--
Implementing the `Deref` trait allows you to customize the behavior of the
*dereference operator*, `*` (as opposed to the multiplication or glob
operator). By implementing `Deref` in such a way that a smart pointer can be
treated like a regular reference, you can write code that operates on
references and use that code with smart pointers too.
-->

`Deref`トレイトを実装することで、*参照外し演算子*の`*`（掛け算やグロブ演算子とは違います）の振る舞いをカスタマイズできます。
`Deref`を実装してスマートポインタを普通の参照みたいに扱えるようにすれば、
参照に対して処理を行うコードを書いて、そのコードをスマートポインタに対しても使うことができるのです。

<!--
Let’s first look at how dereference operator works with regular references.
Then we'll try to define a custom type that behaves like `Box<T>`, and see why
the dereference operator doesn't work like a reference on our newly defined
type. We’ll explore how implementing the `Deref` trait makes it possible for
smart pointers to work in a similar way as references. Then we’ll look at
Rust’s *deref coercion* feature and how it lets us work with either references
or smart pointers.
-->

まずは、参照外し演算子が普通の参照に対して動作するところを見ましょう。それから、`Box<T>`のように振る舞う独自の型を定義してみましょう。
参照とは異なり、新しく定義した型には参照外し演算子を使えません。その理由を確認します。
`Deref`トレイトを実装すればスマートポインタは参照と同じように機能するので、そのやり方を調べましょう。
そして、Rust には*参照外し型強制*という機能があり、その機能のおかげで参照やスマートポインタをうまく使うことができるので、それに目を向けてみましょう。

<!--
### Following the Pointer to the Value with the Dereference Operator
-->

### 参照外し演算子で値までポインタを追いかける

<!--
A regular reference is a type of pointer, and one way to think of a pointer is
as an arrow to a value stored somewhere else. In Listing 15-6, we create a
reference to an `i32` value and then use the dereference operator to follow the
reference to the data:
-->

普通の参照は 1 種のポインタであり、ポインタはどこか他の場所に格納された値への矢印と見なすことができます。
リスト 15-6 では、`i32`値への参照を生成してから参照外し演算子を使ってデータまで参照を辿ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<!--
<span class="caption">Listing 15-6: Using the dereference operator to follow a
reference to an `i32` value</span>
-->

<span class="caption">リスト 15-6: 参照外し演算子を使用して参照を`i32`値まで追いかける</span>

<!--
The variable `x` holds an `i32` value, `5`. We set `y` equal to a reference to
`x`. We can assert that `x` is equal to `5`. However, if we want to make an
assertion about the value in `y`, we have to use `*y` to follow the reference
to the value it’s pointing to (hence *dereference*). Once we dereference `y`,
we have access to the integer value `y` is pointing to that we can compare with
`5`.
-->

変数`x`は`i32`値の`5`を保持しています。`y`は`x`への参照として設定します。`x`は`5`に等しいとアサートできます。
しかしながら、`y`の値に関するアサートを行いたい場合、`*y`を使用して参照が指している値まで追いかけなければなりません（そのため*参照外し*です）。
一旦`y`の参照を外せば、`y`が指している整数値にアクセスできます。これは`5`と比較可能です。

<!--
If we tried to write `assert_eq!(5, y);` instead, we would get this compilation
error:
-->

代わりに`assert_eq!(5, y);`と書こうとしたら、こんなコンパイルエラーが出るでしょう。

```text
error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<&{integer}>` is
not satisfied
(エラー: トレイト境界`{integer}: std::cmp::PartialEq<&{integer}>`は満たされていません)
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ can't compare `{integer}` with `&{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
  (助言：トレイト`std::cmp::PartialEq<&{integer}>`は`{integer}`に対して実装されていません)
```

<!--
Comparing a number and a reference to a number isn’t allowed because they’re
different types. We must use the dereference operator to follow the reference
to the value it's pointing to.
-->

数値と数値への参照の比較は許されていません。これらは異なる型だからです。参照外し演算子を使用して、
参照が指している値まで追いかけなければならないのです。

<!--
### Using `Box<T>` Like a Reference
-->

### `Box<T>`を参照のように使う

<!--
We can rewrite the code in Listing 15-6 to use a `Box<T>` instead of a
reference; the dereference operator will work as shown in Listing 15-7:
-->

リスト 15-6 のコードを、参照の代わりに`Box<T>`を使うように書き直すことができます。
参照外し演算子は、リスト 15-7 に示したように動くでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<!--
<span class="caption">Listing 15-7: Using the dereference operator on a
`Box<i32>`</span>
-->

<span class="caption">リスト 15-7: `Box<i32>`に対して参照外し演算子を使用する</span>

<!--
The only difference between Listing 15-7 and Listing 15-6 is that here we set
`y` to be an instance of a box pointing to the value in `x` rather than a
reference pointing to the value of `x`. In the last assertion, we can use the
dereference operator to follow the box’s pointer in the same way that we did
when `y` was a reference. Next, we’ll explore what is special about `Box<T>`
that enables us to use the dereference operator by defining our own box type.
-->

リスト 15-7 とリスト 15-6 の唯一の違いは、ここでは`y`が、`x`の値を指す参照ではなく、
`x`の値を指すボックスのインスタンスとして設定されている点にあります。
最後のアサートでは、参照外し演算子を使ってボックスのポインタを辿ることができます。これは`y`が参照だった時と同じやり方です。
参照外し演算子が使える以上`Box<T>`には特別な何かがあるので、次はそれについて調べることにします。そのために、独自にボックス型を定義します。

<!--
### Defining Our Own Smart Pointer
-->

### 独自のスマートポインタを定義する

<!--
Let’s build a smart pointer similar to the `Box<T>` type provided by the
standard library to experience how smart pointers behave differently than
references by default. Then we’ll look at how to add the ability to use the
dereference operator.
-->

標準ライブラリが提供している`Box<T>`型に似たスマートポインタを作りましょう。そうすれば、スマートポインタがそのままだと
参照と同じ様には振る舞わないことがわかります。それから、どうすれば参照外し演算子を使えるようになるのか見てみましょう。

<!--
The `Box<T>` type is ultimately defined as a tuple struct with one element, so
Listing 15-8 defines a `MyBox<T>` type in the same way. We’ll also define a
`new` function to match the `new` function defined on `Box<T>`.
-->

`Box<T>`型は突き詰めると（訳註：データがヒープに置かれることを無視すると）1 要素のタプル構造体のような定義になります。なのでリスト 15-8 ではそのように`MyBox<T>`型を定義しています。
また、`Box<T>`に定義された`new`関数に対応する`new`関数も定義しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

<!--
<span class="caption">Listing 15-8: Defining a `MyBox<T>` type</span>
-->

<span class="caption">リスト 15-8: `MyBox<T>`型を定義する</span>

<!--
We define a struct named `MyBox` and declare a generic parameter `T`, because
we want our type to hold values of any type. The `MyBox` type is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.
-->

`MyBox`という構造体を定義し、ジェネリック引数の`T`を宣言しています。この型にどんな型の値も持たせたいからです。
`MyBox`型は型`T`の要素を 1 つ持つタプル構造体です。`MyBox::new`関数は型`T`の引数を 1 つ取り、
渡した値を持つ`MyBox`のインスタンスを返します。

<!--
Let’s try adding the `main` function in Listing 15-7 to Listing 15-8 and
changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The
code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference
`MyBox`.
-->

試しにリスト 15-7 の`main`関数をリスト 15-8 に追加し、定義した`MyBox<T>`型を`Box<T>`の代わりに使うよう変更してみてください。
コンパイラは`MyBox`を参照外しする方法がわからないので、リスト 15-9 のコードはコンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<!--
<span class="caption">Listing 15-9: Attempting to use `MyBox<T>` in the same
way we used references and `Box<T>`</span>
-->

<span class="caption">リスト 15-9: 参照と`Box<T>`を使ったのと同じように`MyBox<T>`を使おうとする</span>

<!--
Here’s the resulting compilation error:
-->

こちらが結果として出るコンパイルエラーです。

```text
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
(エラー: 型`MyBox<{integer}>`は参照外しできません)
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
```

<!--
Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that
ability on our type. To enable dereferencing with the `*` operator, we
implement the `Deref` trait.
-->

`MyBox<T>`の参照を外すことはできません。そのための実装を与えていないからです。`*`演算子で参照外しできるようにするには、
`Deref`トレイトを実装します。

<!--
### Treating a Type Like a Reference by Implementing the `Deref` Trait
-->

### `Deref`トレイトを実装して型を参照のように扱う

<!--
As discussed in Chapter 10, to implement a trait, we need to provide
implementations for the trait’s required methods. The `Deref` trait, provided
by the standard library, requires us to implement one method named `deref` that
borrows `self` and returns a reference to the inner data. Listing 15-10
contains an implementation of `Deref` to add to the definition of `MyBox`:
-->

第 10 章で議論したように、トレイトを実装するにはトレイトの必須メソッドに実装を与える必要があります。
`Deref`トレイトは標準ライブラリで提供されており、`deref`という 1 つのメソッドの実装を要求します。`deref`は`self`を借用し、
内部のデータへの参照を返すメソッドです。
リスト 15-10 には、`MyBox`の定義に付け足す`Deref`の実装が含まれています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

<!--
<span class="caption">Listing 15-10: Implementing `Deref` on `MyBox<T>`</span>
-->

<span class="caption">リスト 15-10: `MyBox<T>`に`Deref`を実装する</span>

<!--
The `type Target = T;` syntax defines an associated type for the `Deref` trait
to use. Associated types are a slightly different way of declaring a generic
parameter, but you don’t need to worry about them for now; we’ll cover them in
more detail in Chapter 19.
-->

`type Target = T;`という記法は、`Deref`トレイトが使用する関連型を定義しています。関連型はまた少し違ったやり方でジェネリック引数を宣言するためのものですが、今は気にする必要はありません。第 19 章でより詳しく扱います。

<!--
We fill in the body of the `deref` method with `&self.0` so `deref` returns a
reference to the value we want to access with the `*` operator. The `main`
function in Listing 15-9 that calls `*` on the `MyBox<T>` value now compiles,
and the assertions pass!
-->

`deref`メソッドの本体は`&self.0`だけなので、`deref`が返すのは私達が`*`演算子でアクセスしたい値への参照なわけです。
リスト 15-9 の`MyBox<T>`に`*`を呼び出す`main`関数はこれでコンパイルでき、アサートも通ります！

<!--
Without the `Deref` trait, the compiler can only dereference `&` references.
The `deref` method gives the compiler the ability to take a value of any type
that implements `Deref` and call the `deref` method to get a `&` reference that
it knows how to dereference.
-->

`Deref`トレイトがないと、コンパイラは`&`参照しか参照外しできません。
`deref`メソッドのおかげで、コンパイラは`Deref`を実装している型の値を取り、`deref`メソッドを呼ぶことで、参照外しが可能な`&`参照を得られるようになります。

<!--
When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this
code:
-->

リスト 15-9 に`*y`を入力した時、水面下で Rust は実際にはこのようなコードを走らせていました。

```rust,ignore
*(y.deref())
```


<!--
Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so we don’t have to think about whether or not we need to
call the `deref` method. This Rust feature lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.
-->

Rust が`*`演算子を`deref`メソッドの呼び出しと普通の参照外しへと置き換えてくれるので、
私達は`deref`メソッドを呼び出す必要があるかどうかを考えなくて済むわけです。この Rust の機能により、
普通の参照か`Deref`を実装した型であるかどうかに関わらず、等しく機能するコードを書くことができます。

<!--
The reason the `deref` method returns a reference to a value and that the plain
dereference outside the parentheses in `*(y.deref())` is still necessary is the
ownership system. If the `deref` method returned the value directly instead of
a reference to the value, the value would be moved out of `self`. We don't want
to take ownership of the inner value inside `MyBox<T>` in this case or in most
cases where we use the dereference operator.
-->

`deref`メソッドが値への参照を返し、`*(y.deref())`のかっこの外にある普通の参照外しがそれでも必要になるのは、
所有権システムがあるからです。`deref`メソッドが値への参照ではなく値を直接返したら、値は`self`から外にムーブされてしまいます。
今回もそうですが、参照外し演算子を使用するときはほとんどの場合、`MyBox<T>`の中の値の所有権を奪いたくはありません。


<!--
Note that the `*` operator is replaced with a call to the `deref` method and
then a call to `*` operator just once, each time we type a `*` in our code.
Because the substitution of the `*` operator does not recurse infinitely, we
end up with data of type `i32`, which matches the `5` in `assert_eq!` in
Listing 15-9.
-->

`*`演算子が`deref`メソッドの呼び出しと`*`演算子の呼び出しに置き換えられるのは、コード内で`*`を打つ毎にただ 1 回だけ、という点に注意して下さい。
`*`演算子の置き換えは無限に繰り返されないので、型`i32`のデータに行き着きます。これはリスト 15-9 で`assert_eq!`の`5`と合致します。

<!--
### Implicit Deref Coercions with Functions and Methods
-->

### 関数やメソッドで暗黙的な参照外し型強制

<!--
*Deref coercion* is a convenience that Rust performs on arguments to functions
and methods. Deref coercion converts a reference to a type that implements
`Deref` into a reference to a type that `Deref` can convert the original type
into. Deref coercion happens automatically when we pass a reference to a
particular type’s value as an argument to a function or method that doesn’t
match the parameter type in the function or method definition. A sequence of
calls to the `deref` method converts the type we provided into the type the
parameter needs.
-->

*参照外し型強制*は、コンパイラが関数やメソッドの実引数に行う便利なものです。参照外し型強制は、
`Deref`を実装する型への参照を`Deref`が元の型を変換できる型への参照に変換します。参照外し型強制は、
特定の型の値への参照を関数やメソッド定義の引数型と一致しない引数として関数やメソッドに渡すときに自動的に発生します。
一連の`deref`メソッドの呼び出しが、提供した型を引数が必要とする型に変換します。

<!--
2 行目、add as many ... as のようにも見えるが、add [as many ...] ということと思われる
-->

<!--
Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.
-->

参照外し型強制は、関数やメソッド呼び出しを書くプログラマが`&`や`*`を多くの明示的な参照や参照外しとして追記する必要がないように、
Rust に追加されました。また、参照外し型強制のおかげで参照あるいはスマートポインタのどちらかで動くコードをもっと書くことができます。

<!--
To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter:
-->

参照外し型強制が実際に動いていることを確認するため、リスト 15-8 で定義した`MyBox<T>`と、
リスト 15-10 で追加した`Deref`の実装を使用しましょう。リスト 15-11 は、
文字列スライス引数のある関数の定義を示しています：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

<!--
<span class="caption">Listing 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>
-->

<span class="caption">リスト 15-11: 型`&str`の引数`name`のある`hello`関数</span>

<!--
We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12:
-->

`hello`関数は、文字列スライスを引数として呼び出すことができます。例えば、`hello("Rust")`などです。
参照外し型強制により、`hello`を型`MyBox<String>`の値への参照とともに呼び出すことができます。リスト 15-12 のようにですね：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

<!--
<span class="caption">Listing 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>
-->

<span class="caption">リスト 15-12: `hello`を`MyBox<String>`値とともに呼び出し、参照外し型強制のおかげで動く</span>

<!--
Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, and this is in the API documentation
for `Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function’s definition.
-->

ここで、`hello`関数を引数`&m`とともに呼び出しています。この引数は、`MyBox<String>`値への参照です。
リスト 15-10 で`MyBox<T>`に`Deref`トレイトを実装したので、コンパイラは`deref`を呼び出すことで、
`&MyBox<String>`を`&String`に変換できるのです。標準ライブラリは、`String`に文字列スライスを返す`Deref`の実装を提供していて、
この実装は、`Deref`の API ドキュメンテーションに載っています。コンパイラはさらに`deref`を呼び出して、
`&String`を`&str`に変換し、これは`hello`関数の定義と合致します。

<!--
If Rust didn’t implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`.
-->

Rust に参照外し型強制が実装されていなかったら、リスト 15-12 のコードの代わりにリスト 15-13 のコードを書き、
型`&MyBox<String>`の値で`hello`を呼び出さなければならなかったでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

<!--
<span class="caption">Listing 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>
-->

<span class="caption">リスト 15-13: Rust に参照外し型強制がなかった場合に書かなければならないであろうコード</span>

<!--
The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. The code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.
-->

`(*m)`が`MyBox<String>`を`String`に参照外ししています。そして、`&`と`[..]`により、
文字列全体と等しい`String`の文字列スライスを取り、`hello`のシグニチャと一致するわけです。
参照外し型強制のないコードは、これらの記号が関係するので、読むのも書くのも理解するのもより難しくなります。
参照外し型強制により、コンパイラはこれらの変換を自動的に扱えるのです。

<!--
When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter’s type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!
-->

`Deref`トレイトが関係する型に定義されていると、コンパイラは、型を分析し必要なだけ`Deref::deref`を使用して、
参照を得、引数の型と一致させます。`Deref::deref`が挿入される必要のある回数は、コンパイル時に解決されるので、
参照外し型強制を活用するための実行時の代償は何もありません。

<!--
### How Deref Coercion Interacts with Mutability
-->

### 参照外し型強制が可変性と相互作用する方法

<!--
Similar to how we use the `Deref` trait to override `*` operator on
immutable references, you can use the `DerefMut` trait to override the `*`
operator on mutable references.
-->

`Deref`トレイトを使用して不変参照に対して`*`をオーバーライドするように、
`DerefMut`トレイトを使用して可変参照の`*`演算子をオーバーライドできます。

<!--
Rust does deref coercion when it finds types and trait implementations in three
cases:
-->

以下の 3 つの場合に型やトレイト実装を見つけた時にコンパイラは、参照外し型強制を行います：

<!--
* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`
-->

* `T: Deref<Target=U>`の時、`&T`から`&U`
* `T: DerefMut<Target=U>`の時、`&mut T`から`&mut U`
* `T: Deref<Target=U>`の時、`&mut T`から`&U`

<!--
The first two cases are the same except for mutability. The first case states
that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can
get a `&U` transparently. The second case states that the same deref coercion
happens for mutable references.
-->

前者 2 つは、可変性を除いて一緒です。最初のケースは、`&T`があり、`T`が何らかの型`U`への`Deref`を実装しているなら、
透過的に`&U`を得られると述べています。2 番目のケースは、同じ参照外し型強制が可変参照についても起こることを述べています。

<!--
8 行目後半、and だが、逆説で訳した
-->

<!--
The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that
there is only one immutable reference to that data, and the borrowing rules
don’t guarantee that. Therefore, Rust can’t make the assumption that converting
an immutable reference to a mutable reference is possible.
-->

3 番目のケースはもっと巧妙です：Rust はさらに、可変参照を不変参照にも型強制するのです。ですが、逆はできま*せん*:
不変参照は、絶対に可変参照に型強制されないのです。借用規則により、可変参照があるなら、
その可変参照がそのデータへの唯一の参照に違いありません (でなければ、プログラムはコンパイルできません)。
1 つの可変参照を 1 つの不変参照に変換することは、借用規則を絶対に破壊しません。
不変参照を可変参照にするには、そのデータへの不変参照がたった 1 つしかないことが必要ですが、
借用規則はそれを保証してくれません。故に、不変参照を可変参照に変換することが可能であるという前提を敷けません。
