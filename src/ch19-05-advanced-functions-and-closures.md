<!--
## Advanced Functions and Closures
-->

## 高度な関数とクロージャ

<!--
This section explores some advanced features related to functions and closures,
including function pointers and returning closures.
-->

この節では関数とクロージャに関連する高度な機能の一部を探究し、
これには関数ポインタとクロージャの返却が含まれます。

<!--
### Function Pointers
-->

### 関数ポインタ

<!--
We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Functions
coerce to the type `fn` (with a lowercase f), not to be confused with the `Fn`
closure trait. The `fn` type is called a *function pointer*. Passing functions
with function pointers will allow you to use functions as arguments to other
functions.
-->

クロージャを関数に渡す方法について語りました; 普通の関数を関数に渡すこともできるのです！
新しいクロージャを定義するのではなく、既に定義した関数を渡したい時にこのテクニックは有用です。
関数は、型`fn`(小文字のfです)に型強制されます。
`Fn`クロージャトレイトと混同すべきではありません。`fn`型は、*関数ポインタ*と呼ばれます。
関数ポインタで関数を渡すことで、関数を引数として他の関数に渡して使用できます。

<!--
The syntax for specifying that a parameter is a function pointer is similar to
that of closures, as shown in Listing 19-27, where we’ve defined a function
`add_one` that adds one to its parameter. The function `do_twice` takes two
parameters: a function pointer to any function that takes an `i32` parameter
and returns an `i32`, and one `i32` value. The `do_twice` function calls the
function `f` twice, passing it the `arg` value, then adds the two function call
results together. The `main` function calls `do_twice` with the arguments
`add_one` and `5`.
-->


引数が関数ポインタであると指定する記法は、リスト19-27に示すように、クロージャのものと似ています。
ここでは引数に1を足す関数`add_one`を定義しています。関数`do_twice`は2個の引数を取ります:
`i32`引数を取り`i32`を返す任意の関数への関数ポインタと、1個の`i32`値です。
`do_twice`関数は、関数`f`に`arg`値を渡して2度呼び出し、その後2回の関数呼び出しの結果を足し合わせます。
`main`関数は`do_twice`を引数`add_one`と`5`で呼び出します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-27/src/main.rs}}
```

<!--
<span class="caption">Listing 19-27: Using the `fn` type to accept a function
pointer as an argument</span>
-->

<span class="caption">リスト19-27: `fn`型を使用して引数として関数ポインタを受け入れる</span>

<!--
This code prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.
-->

このコードは、`The answer is: 12`と出力します。`do_twice`の引数`f`は、型`i32`の1つの引数を取り、
`i32`を返す`fn`と指定しています。それから、`do_twice`の本体で`f`を呼び出すことができます。
`main`では、関数名の`add_one`を最初の引数として`do_twice`に渡せます。

<!--
Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.
-->

クロージャと異なり、`fn`はトレイトではなく型なので、トレイト境界として`Fn`トレイトの1つでジェネリックな型引数を宣言するのではなく、
直接`fn`を引数の型として指定します。

<!--
Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), meaning you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures.
-->

関数ポインタはクロージャトレイト3つ全て(`Fn`、`FnMut`、`FnOnce`)を実装するので、
つまり、クロージャを期待する関数には常に関数ポインタを引数として渡すことができます。
関数が関数とクロージャどちらも受け入れられるように、
ジェネリックな型とクロージャトレイトの1つを使用して関数を書くのが最善です。

<!--
That said, one example of where you would want to only accept `fn` and not
closures is when interfacing with external code that doesn’t have closures: C
functions can accept functions as arguments, but C doesn’t have closures.
-->

とはいえ、クロージャではなく`fn`だけを受け入れたくなる箇所の一例は、
クロージャのない外部コードとのインターフェイスです:
C関数は引数として関数を受け入れられますが、Cにはクロージャがありません。

<!--
couldだが、でしょうでは文を続けられないので、できるであろうにしている
-->

<!--
As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of the `map` method provided by the `Iterator`
trait in the standard library. To use the `map` function to turn a vector of
numbers into a vector of strings, we could use a closure, like this:
-->

インラインでクロージャが定義されるか、名前付きの関数を使用できるであろう箇所の例として、
標準ライブラリの`Iterator`トレイトによって提供される`map`メソッドの使用に目を向けましょう。
`map`関数を使用して数字のベクタを文字列のベクタに変換するには、このようにクロージャを使用できるでしょう:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

<!--
Or we could name a function as the argument to `map` instead of the closure,
like this:
-->

あるいは、このようにクロージャの代わりに`map`に引数として関数を名指しできるでしょう:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

<!--
Note that we must use the fully qualified syntax that we talked about earlier
in the [“Advanced Traits”][advanced-traits] section because
there are multiple functions available named `to_string`. Here, we’re using the
`to_string` function defined in the `ToString` trait, which the standard
library has implemented for any type that implements `Display`.
-->

先ほど[「高度なトレイト」][advanced-traits]節で語ったフルパス記法を使わなければならないことに注意してください。
というのも、`to_string`という利用可能な関数は複数あるからです。ここでは、
`ToString`トレイトで定義された`to_string`関数を使用していて、このトレイトは標準ライブラリが、
`Display`を実装するあらゆる型に実装しています。

<!--
Recall from the [“Enum values”][enum-values] section of Chapter
6 that the name of each enum variant that we define also becomes an initializer
function. We can use these initializer functions as function pointers that
implement the closure traits, which means we can specify the initializer
functions as arguments for methods that take closures, like so:
-->

第6章の[「Enumの値」][enum-values]節で学んだように、
各enum列挙子の名前は初期化子関数にもなることを思い出してください。
これらの初期化子関数は、クロージャトレイトを実装する関数ポインタとして使用することができます。
つまり、次のように、クロージャを取るメソッドの引数として初期化子関数を指定することができるのです:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

<!--
Here we create `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style, and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.
-->

ここでは、`map`が呼ばれる範囲内の各`u32`値を使用して、
`Status::Value`の初期化子関数を使用することで、`Status::Value`インスタンスを作成しています。
このスタイルを好む方もいますし、クロージャを使うのを好む方もいます。
どちらも結果的に同じコードにコンパイルされるので、どちらでも、自分にとって明確な方を使用してください。

<!--
### Returning Closures
-->

### クロージャを返却する

<!--
Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type, for example.
-->

クロージャはトレイトによって表現されます。つまり、クロージャを直接は返却できないのです。
トレイトを返却したい可能性のあるほとんどの場合、代わりにトレイトを実装する具体的な型を関数の戻り値として使用できます。
ですが、クロージャではそれはできません。返却可能な具体的な型がないからです; 例えば、
関数ポインタの`fn`を戻り値の型として使うことは許容されていません。

<!--
The following code tries to return a closure directly, but it won’t compile:
-->

以下のコードは、クロージャを直接返そうとしていますが、コンパイルできません:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

<!--
The compiler error is as follows:
-->

コンパイルエラーは以下の通りです:

```console
{{#include ../listings/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```

<!--
The error references the `Sized` trait again! Rust doesn’t know how much space
it will need to store the closure. We saw a solution to this problem earlier.
We can use a trait object:
-->

エラーは、再度`Sized`トレイトを参照しています！コンパイラには、クロージャを格納するのに必要なスペースがどれくらいかわからないのです。
この問題の解決策は先ほど見かけました。トレイトオブジェクトを使えます:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

<!--
This code will compile just fine. For more about trait objects, refer to the
section [“Using Trait Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]
in Chapter 17.
-->

このコードは、問題なくコンパイルできます。トレイトオブジェクトについて詳しくは、
第17章の[「トレイトオブジェクトで異なる型の値を許容する」][using-trait-objects-that-allow-for-values-of-different-types]節を参照してください。

<!--
Next, let’s look at macros!
-->

次は、マクロを見てみましょう！

<!--
[advanced-traits]:
ch19-03-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
-->

[advanced-traits]:
ch19-03-advanced-traits.html#高度なトレイト
[enum-values]: ch06-01-defining-an-enum.html#enumの値
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#トレイトオブジェクトで異なる型の値を許容する
