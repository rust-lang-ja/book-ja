<!--
## Advanced Functions and Closures
-->

## 高度な関数とクロージャ

<!--
Finally, we’ll explore some advanced features related to functions and
closures, which include function pointers and returning closures.
-->

最後に関数とクロージャに関連する高度な機能の一部を探究し、これには関数ポインタとクロージャの返却が含まれます。

<!--
### Function Pointers
-->

### 関数ポインタ

<!--
We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Doing this
with function pointers will allow you to use functions as arguments to other
functions. Functions coerce to the type `fn` (with a lowercase f), not to be
confused with the `Fn` closure trait. The `fn` type is called a *function
pointer*. The syntax for specifying that a parameter is a function pointer is
similar to that of closures, as shown in Listing 19-35.
-->

クロージャを関数に渡す方法について語りました; 普通の関数を関数に渡すこともできるのです！
新しいクロージャを定義するのではなく、既に定義した関数を渡したい時にこのテクニックは有用です。
これを関数ポインタで行うと、関数を引数として他の関数に渡して使用できます。関数は、型`fn`(小文字の f です) に型強制されます。
`Fn`クロージャトレイトと混同すべきではありません。`fn`型は、*関数ポインタ*と呼ばれます。
引数が関数ポインタであると指定する記法は、クロージャのものと似ています。リスト 19-35 のように。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    // 答えは{}
    println!("The answer is: {}", answer);
}
```

<!--
<span class="caption">Listing 19-35: Using the `fn` type to accept a function
pointer as an argument</span>
-->

<span class="caption">リスト 19-35: `fn`型を使用して引数として関数ポインタを受け入れる</span>

<!--
This code prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.
-->

このコードは、`The answer is: 12`と出力します。`do_twice`の引数`f`は、型`i32`の 1 つの引数を取り、
`i32`を返す`fn`と指定しています。それから、`do_twice`の本体で`f`を呼び出すことができます。
`main`では、関数名の`add_one`を最初の引数として`do_twice`に渡せます。

<!--
Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.
-->

クロージャと異なり、`fn`はトレイトではなく型なので、トレイト境界として`Fn`トレイトの 1 つでジェネリックな型引数を宣言するのではなく、
直接`fn`を引数の型として指定します。

<!--
Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures.
-->

関数ポインタは、クロージャトレイト 3 つ全て (`Fn`、`FnMut`、`FnOnce`) を実装するので、常に関数ポインタを引数として、
クロージャを期待する関数に渡すことができます。関数が関数とクロージャどちらも受け入れられるように、
ジェネリックな型とクロージャトレイトの 1 つを使用して関数を書くのが最善です。

<!--
An example of where you would want to only accept `fn` and not closures is when
interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures.
-->

クロージャではなく`fn`だけを受け入れたくなる箇所の一例は、クロージャのない外部コードとのインターフェイスです：
C 関数は引数として関数を受け入れられますが、C にはクロージャがありません。

<!--
could だが、でしょうでは文を続けられないので、できるであろうにしている
-->

<!--
As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of `map`. To use the `map` function to turn a
vector of numbers into a vector of strings, we could use a closure, like this:
-->

インラインでクロージャが定義されるか、名前付きの関数を使用できるであろう箇所の例として、`map`の使用に目を向けましょう。
`map`関数を使用して数字のベクタを文字列のベクタに変換するには、このようにクロージャを使用できるでしょう：

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

<!--
Or we could name a function as the argument to `map` instead of the closure,
like this:
-->

あるいは、このようにクロージャの代わりに`map`に引数として関数を名指しできるでしょう：

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

<!--
Note that we must use the fully qualified syntax that we talked about earlier
in the “Advanced Traits” section because there are multiple functions available
named `to_string`. Here, we’re using the `to_string` function defined in the
`ToString` trait, which the standard library has implemented for any type that
implements `Display`.
-->

先ほど「高度なトレイト」節で語ったフルパス記法を使わなければならないことに注意してください。
というのも、`to_string`という利用可能な関数は複数あるからです。ここでは、
`ToString`トレイトで定義された`to_string`関数を使用していて、このトレイトは標準ライブラリが、
`Display`を実装するあらゆる型に実装しています。

<!--
Some people prefer this style, and some people prefer to use closures. They end
up compiling to the same code, so use whichever style is clearer to you.
-->

このスタイルを好む方もいますし、クロージャを使うのを好む方もいます。どちらも結果的に同じコードにコンパイルされるので、
どちらでも、自分にとって明確な方を使用してください。

<!--
### Returning Closures
-->

### クロージャを返却する

<!--
Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. But you can’t do that with closures because they don’t have a
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

以下のコードは、クロージャを直接返そうとしていますが、コンパイルできません：

```rust,ignore
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

<!--
The compiler error is as follows:
-->

コンパイルエラーは以下の通りです：

```text
error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static:
std::marker::Sized` is not satisfied
 -->
  |
1 | fn returns_closure() -> Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^ `std::ops::Fn(i32) -> i32 + 'static`
  does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for
  `std::ops::Fn(i32) -> i32 + 'static`
  = note: the return type of a function must have a statically known size
```

<!--
The error references the `Sized` trait again! Rust doesn’t know how much space
it will need to store the closure. We saw a solution to this problem earlier.
We can use a trait object:
-->

エラーは、再度`Sized`トレイトを参照しています！コンパイラには、クロージャを格納するのに必要なスペースがどれくらいかわからないのです。
この問題の解決策は先ほど見かけました。トレイトオブジェクトを使えます：

```rust
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

<!--
This code will compile just fine. For more about trait objects, refer to the
“Using Trait Objects That Allow for Values of Different Types” section in
Chapter 17.
-->

このコードは、問題なくコンパイルできます。トレイトオブジェクトについて詳しくは、
第 17 章の「トレイトオブジェクトで異なる型の値を許容する」節を参照してください。

<!--
Next, let’s look at macros!
-->

次は、マクロを見てみましょう！
