<!--
## Treating Smart Pointers Like Regular References with the `Deref` Trait
-->

## `Deref`トレイトでスマートポインタを普通の参照のように扱う

<!--
Implementing the `Deref` trait allows you to customize the behavior of the
*dereference operator* `*` (not to be confused with the multiplication or glob
operator). By implementing `Deref` in such a way that a smart pointer can be
treated like a regular reference, you can write code that operates on
references and use that code with smart pointers too.
-->

`Deref`トレイトを実装することで、*参照外し演算子*の`*`（掛け算やグロブ演算子と混同しないてください）の振る舞いをカスタマイズできます。
`Deref`を実装してスマートポインタを普通の参照みたいに扱えるようにすれば、
参照に対して処理を行うコードを書いて、そのコードをスマートポインタに対しても使うことができるのです。

<!--
Let’s first look at how the dereference operator works with regular references.
Then we’ll try to define a custom type that behaves like `Box<T>`, and see why
the dereference operator doesn’t work like a reference on our newly defined
type. We’ll explore how implementing the `Deref` trait makes it possible for
smart pointers to work in ways similar to references. Then we’ll look at
Rust’s *deref coercion* feature and how it lets us work with either references
or smart pointers.
-->

まずは、参照外し演算子が普通の参照に対して動作するところを見ましょう。それから、`Box<T>`のように振る舞う独自の型を定義してみましょう。
参照とは異なり、新しく定義した型には参照外し演算子を使えません。その理由を確認します。
`Deref`トレイトを実装すればスマートポインタは参照と同じように機能するので、そのやり方を調べましょう。
そして、Rustには*参照外し型強制*という機能があり、その機能のおかげで参照やスマートポインタをうまく使うことができるので、それに目を向けてみましょう。

<!--
> Note: there’s one big difference between the `MyBox<T>` type we’re about to
> build and the real `Box<T>`: our version will not store its data on the heap.
> We are focusing this example on `Deref`, so where the data is actually stored
> is less important than the pointer-like behavior.
-->

> 注釈: 私たちが構築しようとしている`MyBox<T>`型と本物の`Box<T>`との間には、1つ大きな違いがあります:
> 私たちのバージョンはヒープ上にデータを保存しません。
> 私たちは`Deref`の例として注目しているので、データが実際にどこに保存されるかは、
> ポインタのように振る舞うかどうかに比べて、重要ではありません。

<!-- Old link, do not remove -->
<!--
<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
-->

<!--
### Following the Pointer to the Value
-->

### 値までポインタを追いかける

<!--
A regular reference is a type of pointer, and one way to think of a pointer is
as an arrow to a value stored somewhere else. In Listing 15-6, we create a
reference to an `i32` value and then use the dereference operator to follow the
reference to the value:
-->

普通の参照は1種のポインタであり、ポインタはどこか他の場所に格納された値への矢印と見なすことができます。
リスト15-6では、`i32`値への参照を生成してから参照外し演算子を使って値まで参照を辿ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<!--
<span class="caption">Listing 15-6: Using the dereference operator to follow a
reference to an `i32` value</span>
-->

<span class="caption">リスト15-6: 参照外し演算子を使用して参照を`i32`値まで追いかける</span>

<!--
The variable `x` holds an `i32` value `5`. We set `y` equal to a reference to
`x`. We can assert that `x` is equal to `5`. However, if we want to make an
assertion about the value in `y`, we have to use `*y` to follow the reference
to the value it’s pointing to (hence *dereference*) so the compiler can compare
the actual value. Once we dereference `y`, we have access to the integer value
`y` is pointing to that we can compare with `5`.
-->

変数`x`は`i32`値の`5`を保持しています。`y`は`x`への参照として設定します。`x`は`5`に等しいとアサートできます。
しかしながら、`y`の値に関するアサートを行いたい場合、コンパイラが実際の値と比較できるように、
`*y`を使用して参照が指している値まで追いかけなければなりません（そのため*参照外し*です）。
一旦`y`の参照を外せば、`y`が指している整数値にアクセスできます。これは`5`と比較可能です。

<!--
If we tried to write `assert_eq!(5, y);` instead, we would get this compilation
error:
-->

代わりに`assert_eq!(5, y);`と書こうとしたら、こんなコンパイルエラーが出るでしょう。

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

<!--
Comparing a number and a reference to a number isn’t allowed because they’re
different types. We must use the dereference operator to follow the reference
to the value it’s pointing to.
-->

数値と数値への参照の比較は許されていません。これらは異なる型だからです。参照外し演算子を使用して、
参照が指している値まで追いかけなければならないのです。

<!--
### Using `Box<T>` Like a Reference
-->

### `Box<T>`を参照のように使う

<!--
We can rewrite the code in Listing 15-6 to use a `Box<T>` instead of a
reference; the dereference operator used on the `Box<T>` in Listing 15-7
functions in the same way as the dereference operator used on the reference in
Listing 15-6:
-->

リスト15-6のコードを、参照の代わりに`Box<T>`を使うように書き直すことができます。
リスト15-7に`Box<T>`に対して使用されている参照外し演算子は、
リスト15-6で参照に対して使用されている参照外し演算子と同じように機能するでしょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<!--
<span class="caption">Listing 15-7: Using the dereference operator on a
`Box<i32>`</span>
-->

<span class="caption">リスト15-7: `Box<i32>`に対して参照外し演算子を使用する</span>

<!--
The main difference between Listing 15-7 and Listing 15-6 is that here we set
`y` to be an instance of a `Box<T>` pointing to a copied value of `x` rather
than a reference pointing to the value of `x`. In the last assertion, we can
use the dereference operator to follow the pointer of the `Box<T>` in the same
way that we did when `y` was a reference. Next, we’ll explore what is special
about `Box<T>` that enables us to use the dereference operator by defining our
own type.
-->

リスト15-7とリスト15-6の主な違いは、ここでは`y`が、`x`の値を指す参照ではなく、
`x`からコピーされた値を指す`Box<T>`のインスタンスとして設定されている点にあります。
最後のアサートでは、参照外し演算子を使って`Box<T>`のポインタを辿ることができます。これは`y`が参照だった時と同じやり方です。
参照外し演算子が使える以上`Box<T>`には特別な何かがあるので、次はそれについて調べることにします。そのために、独自の型を定義します。

<!--
### Defining Our Own Smart Pointer
-->

### 独自のスマートポインタを定義する

<!--
Let’s build a smart pointer similar to the `Box<T>` type provided by the
standard library to experience how smart pointers behave differently from
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

`Box<T>`型は突き詰めると（訳註：データがヒープに置かれることを無視すると）1要素のタプル構造体のような定義になります。なのでリスト15-8ではそのように`MyBox<T>`型を定義しています。
また、`Box<T>`に定義された`new`関数に対応する`new`関数も定義しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-8: Defining a `MyBox<T>` type</span>
-->

<span class="caption">リスト15-8: `MyBox<T>`型を定義する</span>

<!--
We define a struct named `MyBox` and declare a generic parameter `T`, because
we want our type to hold values of any type. The `MyBox` type is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.
-->

`MyBox`という構造体を定義し、ジェネリック引数の`T`を宣言しています。この型にどんな型の値も持たせたいからです。
`MyBox`型は型`T`の要素を1つ持つタプル構造体です。`MyBox::new`関数は型`T`の引数を1つ取り、
渡した値を持つ`MyBox`のインスタンスを返します。

<!--
Let’s try adding the `main` function in Listing 15-7 to Listing 15-8 and
changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The
code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference
`MyBox`.
-->

試しにリスト15-7の`main`関数をリスト15-8に追加し、定義した`MyBox<T>`型を`Box<T>`の代わりに使うよう変更してみてください。
コンパイラは`MyBox`を参照外しする方法がわからないので、リスト15-9のコードはコンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-9: Attempting to use `MyBox<T>` in the same
way we used references and `Box<T>`</span>
-->

<span class="caption">リスト15-9: 参照と`Box<T>`を使ったのと同じように`MyBox<T>`を使おうとする</span>

<!--
Here’s the resulting compilation error:
-->

こちらが結果として出るコンパイルエラーです。

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
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
As discussed in the [“Implementing a Trait on a Type”][impl-trait]
section of Chapter 10, to implement a trait, we need to provide
implementations for the trait’s required methods. The `Deref` trait, provided
by the standard library, requires us to implement one method named `deref` that
borrows `self` and returns a reference to the inner data. Listing 15-10
contains an implementation of `Deref` to add to the definition of `MyBox`:
-->

第10章の[「トレイトを型に実装する」][impl-trait]節で議論したように、トレイトを実装するにはトレイトの必須メソッドに実装を与える必要があります。
`Deref`トレイトは標準ライブラリで提供されており、`deref`という1つのメソッドの実装を要求します。`deref`は`self`を借用し、
内部のデータへの参照を返すメソッドです。
リスト15-10には、`MyBox`の定義に付け足す`Deref`の実装が含まれています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-10: Implementing `Deref` on `MyBox<T>`</span>
-->

<span class="caption">リスト15-10: `MyBox<T>`に`Deref`を実装する</span>

<!--
The `type Target = T;` syntax defines an associated type for the `Deref`
trait to use. Associated types are a slightly different way of declaring a
generic parameter, but you don’t need to worry about them for now; we’ll cover
them in more detail in Chapter 19.
-->

`type Target = T;`という記法は、`Deref`トレイトが使用する関連型を定義しています。
関連型はまた少し違ったやり方でジェネリック引数を宣言するためのものですが、今は気にする必要はありません。
第19章でより詳しく扱います。

<!--
We fill in the body of the `deref` method with `&self.0` so `deref` returns a
reference to the value we want to access with the `*` operator; recall from the
[“Using Tuple Structs without Named Fields to Create Different
Types”][tuple-structs] section of Chapter 5 that `.0` accesses
the first value in a tuple struct. The `main` function in Listing 15-9 that
calls `*` on the `MyBox<T>` value now compiles, and the assertions pass!
-->

`deref`が`*`演算子でアクセスしたい値への参照を返すように、`deref`メソッドの本体は`&self.0`で埋めました;
第5章の[「異なる型を生成する名前付きフィールドのないタプル構造体を使用する」][tuple-structs]節で触れたように、
`.0`はタプル構造体の最初の値にアクセスするということを思い出してください。
リスト15-9の`MyBox<T>`に`*`を呼び出す`main`関数はこれでコンパイルでき、アサートも通ります！

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

リスト15-9に`*y`を入力した時、水面下でRustは実際にはこのようなコードを走らせていました。

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

Rustが`*`演算子を`deref`メソッドの呼び出しと普通の参照外しへと置き換えてくれるので、
私達は`deref`メソッドを呼び出す必要があるかどうかを考えなくて済むわけです。このRustの機能により、
普通の参照か`Deref`を実装した型であるかどうかに関わらず、等しく機能するコードを書くことができます。

<!--
The reason the `deref` method returns a reference to a value, and that the
plain dereference outside the parentheses in `*(y.deref())` is still necessary,
is to do with the ownership system. If the `deref` method returned the value
directly instead of a reference to the value, the value would be moved out of
`self`. We don’t want to take ownership of the inner value inside `MyBox<T>` in
this case or in most cases where we use the dereference operator.
-->

`deref`メソッドが値への参照を返し、`*(y.deref())`のかっこの外にある普通の参照外しがそれでも必要になるのは、
所有権システムに関係します。`deref`メソッドが値への参照ではなく値を直接返したら、値は`self`から外にムーブされてしまいます。
今回もそうですが、参照外し演算子を使用するときはほとんどの場合、`MyBox<T>`の中の値の所有権を奪いたくはありません。


<!--
Note that the `*` operator is replaced with a call to the `deref` method and
then a call to the `*` operator just once, each time we use a `*` in our code.
Because the substitution of the `*` operator does not recurse infinitely, we
end up with data of type `i32`, which matches the `5` in `assert_eq!` in
Listing 15-9.
-->

`*`演算子が`deref`メソッドの呼び出しと`*`演算子の呼び出しに置き換えられるのは、コード内で`*`を使用する毎にただ1回だけ、という点に注意して下さい。
`*`演算子の置き換えは無限に繰り返されないので、型`i32`のデータに行き着きます。これはリスト15-9で`assert_eq!`の`5`と合致します。

<!--
### Implicit Deref Coercions with Functions and Methods
-->

### 関数やメソッドで暗黙的な参照外し型強制

<!--
*Deref coercion* converts a reference to a type that implements the `Deref`
trait into a reference to another type. For example, deref coercion can convert
`&String` to `&str` because `String` implements the `Deref` trait such that it
returns `&str`. Deref coercion is a convenience Rust performs on arguments to
functions and methods, and works only on types that implement the `Deref`
trait. It happens automatically when we pass a reference to a particular type’s
value as an argument to a function or method that doesn’t match the parameter
type in the function or method definition. A sequence of calls to the `deref`
method converts the type we provided into the type the parameter needs.
-->

*参照外し型強制*は、`Deref`トレイトを実装する型への参照を、別の型への参照に変換します。
例えば、`String`は`&str`を返すように`Deref`トレイトを実装しているので、参照外し型強制で`&String`を`&str`に変換することができます。
参照外し型強制は、コンパイラが関数やメソッドの実引数に行う便利なもので、
`Deref`トレイトを実装する型に対してのみ機能します。参照外し型強制は、
特定の型の値への参照を関数やメソッド定義の引数型と一致しない引数として関数やメソッドに渡すときに自動的に発生します。
一連の`deref`メソッドの呼び出しが、提供した型を引数が必要とする型に変換します。

<!--
2行目、add as many ... asのようにも見えるが、add [as many ...]ということと思われる
-->

<!--
Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.
-->

参照外し型強制は、関数やメソッド呼び出しを書くプログラマが`&`や`*`を多くの明示的な参照や参照外しとして追記する必要がないように、
Rustに追加されました。また、参照外し型強制のおかげで参照あるいはスマートポインタのどちらかで動くコードをもっと書くことができます。

<!--
To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter:
-->

参照外し型強制が実際に動いていることを確認するため、リスト15-8で定義した`MyBox<T>`と、
リスト15-10で追加した`Deref`の実装を使用しましょう。リスト15-11は、
文字列スライス引数のある関数の定義を示しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>
-->

<span class="caption">リスト15-11: 型`&str`の引数`name`のある`hello`関数</span>

<!--
We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12:
-->

`hello`関数は、文字列スライスを引数として呼び出すことができます。例えば、`hello("Rust")`などです。
参照外し型強制により、`hello`を型`MyBox<String>`の値への参照とともに呼び出すことができます。リスト15-12のようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>
-->

<span class="caption">リスト15-12: `hello`を`MyBox<String>`値とともに呼び出し、参照外し型強制のおかげで動く</span>

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
リスト15-10で`MyBox<T>`に`Deref`トレイトを実装したので、コンパイラは`deref`を呼び出すことで、
`&MyBox<String>`を`&String`に変換できるのです。標準ライブラリは、`String`に文字列スライスを返す`Deref`の実装を提供していて、
この実装は、`Deref`のAPIドキュメンテーションに載っています。コンパイラはさらに`deref`を呼び出して、
`&String`を`&str`に変換し、これは`hello`関数の定義と合致します。

<!--
If Rust didn’t implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`.
-->

Rustに参照外し型強制が実装されていなかったら、リスト15-12のコードの代わりにリスト15-13のコードを書き、
型`&MyBox<String>`の値で`hello`を呼び出さなければならなかったでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>
-->

<span class="caption">リスト15-13: Rustに参照外し型強制がなかった場合に書かなければならないであろうコード</span>

<!--
The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. This code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.
-->

`(*m)`が`MyBox<String>`を`String`に参照外ししています。そして、`&`と`[..]`により、
文字列全体と等しい`String`の文字列スライスを取り、`hello`のシグニチャと一致するわけです。
参照外し型強制のないこのコードは、これらの記号が関係するので、読むのも書くのも理解するのもより難しくなります。
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
Similar to how you use the `Deref` trait to override the `*` operator on
immutable references, you can use the `DerefMut` trait to override the `*`
operator on mutable references.
-->

`Deref`トレイトを使用して不変参照に対して`*`をオーバーライドするように、
`DerefMut`トレイトを使用して可変参照の`*`演算子をオーバーライドできます。

<!--
Rust does deref coercion when it finds types and trait implementations in three
cases:
-->

以下の3つの場合に型やトレイト実装を見つけた時にコンパイラは、参照外し型強制を行います:

<!--
* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`
-->

* `T: Deref<Target=U>`の時、`&T`から`&U`
* `T: DerefMut<Target=U>`の時、`&mut T`から`&mut U`
* `T: Deref<Target=U>`の時、`&mut T`から`&U`

<!--
The first two cases are the same as each other except that the second
implements mutability. The first case states that if you have a `&T`, and `T`
implements `Deref` to some type `U`, you can get a `&U` transparently. The
second case states that the same deref coercion happens for mutable references.
-->

最初の2つは、2番目は可変性を実装している点を除いて、一緒です。最初のケースは、`&T`があり、`T`が何らかの型`U`への`Deref`を実装しているなら、
透過的に`&U`を得られると述べています。2番目のケースは、同じ参照外し型強制が可変参照についても起こることを述べています。

<!--
The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that the
initial immutable reference is the only immutable reference to that data, but
the borrowing rules don’t guarantee that. Therefore, Rust can’t make the
assumption that converting an immutable reference to a mutable reference is
possible.
-->

3番目のケースはもっと巧妙です: Rustはさらに、可変参照を不変参照にも型強制するのです。ですが、逆はできま*せん*:
不変参照は、絶対に可変参照に型強制されないのです。借用規則により、可変参照があるなら、
その可変参照がそのデータへの唯一の参照に違いありません(でなければ、プログラムはコンパイルできません)。
1つの可変参照を1つの不変参照に変換することは、借用規則を絶対に破壊しません。
不変参照を可変参照にするには、変換元の不変参照がそのデータへの唯一の不変参照であることが必要ですが、
借用規則はそれを保証してくれません。故に、不変参照を可変参照に変換することが可能であるという前提を敷けません。

<!--
[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
-->

[impl-trait]: ch10-02-traits.html#トレイトを型に実装する
[tuple-structs]: ch05-01-defining-structs.html#異なる型を生成する名前付きフィールドのないタプル構造体を使用する
