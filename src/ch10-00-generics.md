<!--
# Generic Types, Traits, and Lifetimes
-->

# ジェネリック型、トレイト、ライフタイム

<!--
Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is *generics*. Generics are abstract
stand-ins for concrete types or other properties. When we’re writing code, we
can express the behavior of generics or how they relate to other generics
without knowing what will be in their place when compiling and running the code.
-->

全てのプログラミング言語には、概念の重複を効率的に扱う道具があります。Rust において、そのような道具の一つが*ジェネリクス*です。
ジェネリクスは、具体型や他のプロパティの抽象的な代役です。コード記述の際、コンパイルやコード実行時に、
ジェネリクスの位置に何が入るかを知ることなく、ジェネリクスの振る舞いや他のジェネリクスとの関係を表現できるのです。

<!--
Similar to the way a function takes parameters with unknown values to run the
same code on multiple concrete values, functions can take parameters of some
generic type instead of a concrete type, like `i32` or `String`. In fact, we’ve
already used generics in Chapter 6 with `Option<T>`, Chapter 8 with `Vec<T>`
and `HashMap<K, V>`, and Chapter 9 with `Result<T, E>`. In this chapter, you’ll
explore how to define your own types, functions, and methods with generics!
-->

関数が未知の値の引数を取り、同じコードを複数の具体的な値に対して走らせるように、
`i32`や`String`などの具体的な型の代わりに何かジェネリックな型の引数を取ることができます。
実際、第 6 章で`Option<T>`、第 8 章で`Vec<T>`と`HashMap<K, V>`、第 9 章で`Result<T, E>`を既に使用しました。
この章では、独自の型、関数、メソッドをジェネリクスとともに定義する方法を探究します！

<!--
First, we’ll review how to extract a function to reduce code duplication. Next,
we’ll use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We’ll also explain how to use
generic types in struct and enum definitions.
-->

まず、関数を抽出して、コードの重複を減らす方法を確認しましょう。次に同じテクニックを活用して、
引数の型のみが異なる 2 つの関数からジェネリックな関数を生成します。また、
ジェネリックな型を構造体や enum 定義で使用する方法も説明します。

<!--
Then you’ll learn how to use *traits* to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to only
those types that have a particular behavior, as opposed to just any type.
-->

それから、トレイトを使用して、ジェネリックな方法で振る舞いを定義する方法を学びます。
ジェネリックな型にトレイトを組み合わせることで、ジェネリックな型を、単にあらゆる型に対してではなく、特定の振る舞いのある型のみに制限できます。

<!--
Finally, we’ll discuss *lifetimes*, a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to borrow values in many situations while still enabling the compiler to
check that the references are valid.
-->

最後に、ライフタイムを議論します。ライフタイムとは、コンパイラに参照がお互いにどう関係しているかの情報を与える一種のジェネリクスです。
ライフタイムのおかげでコンパイラに参照が有効であることを確認してもらうことを可能にしつつ、多くの場面で値を借用できます。

<!--
## Removing Duplication by Extracting a Function
-->

## 関数を抽出することで重複を取り除く

<!--
Before diving into generics syntax, let’s first look at how to remove
duplication that doesn’t involve generic types by extracting a function. Then
we’ll apply this technique to extract a generic function! In the same way that
you recognize duplicated code to extract into a function, you’ll start to
recognize duplicated code that can use generics.
-->

ジェネリクスの記法に飛び込む前にまずは、関数を抽出することでジェネリックな型が関わらない重複を取り除く方法を見ましょう。
そして、このテクニックを適用してジェネリックな関数を抽出するのです！重複したコードを認識して関数に抽出できるのと同じように、
ジェネリクスを使用できる重複コードも認識し始めるでしょう。

<!--
Consider a short program that finds the largest number in a list, as shown in
Listing 10-1.
-->

リスト 10-1 に示したように、リスト内の最大値を求める短いプログラムを考えてください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    // 最大値は{}です
    println!("The largest number is {}", largest);
#  assert_eq!(largest, 100);
}
```

<!--
<span class="caption">Listing 10-1: Code to find the largest number in a list
of numbers</span>
-->

<span class="caption">リスト 10-1: 数字のリストから最大値を求めるコード</span>

<!--
This code stores a list of integers in the variable `number_list` and places
the first number in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, it replaces the number in that variable.
However, if the current number is less than the largest number seen so far, the
variable doesn’t change, and the code moves on to the next number in the list.
After considering all the numbers in the list, `largest` should hold the
largest number, which in this case is 100.
-->

このコードは、整数のリストを変数`number_list`に格納し、リストの最初の数字を`largest`という変数に配置しています。
それからリストの数字全部を走査し、現在の数字が`largest`に格納された数値よりも大きければ、
その変数の値を置き換えます。ですが、現在の数値が今まで見た最大値よりも小さければ、
変数は変わらず、コードはリストの次の数値に移っていきます。リストの数値全てを吟味した後、
`largest`は最大値を保持しているはずで、今回は 100 になります。

<!--
To find the largest number in two different lists of numbers, we can duplicate
the code in Listing 10-1 and use the same logic at two different places in the
program, as shown in Listing 10-2.
-->

2 つの異なる数値のリストから最大値を発見するには、リスト 10-1 のコードを複製し、
プログラムの異なる 2 箇所で同じロジックを使用できます。リスト 10-2 のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

<!--
<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>
-->

<span class="caption">リスト 10-2: *2 つ*の数値のリストから最大値を探すコード</span>

<!--
Although this code works, duplicating code is tedious and error prone. We also
have to update the code in multiple places when we want to change it.
-->

このコードは動くものの、コードを複製することは退屈ですし、間違いも起きやすいです。また、
コードを変更したい時に複数箇所、更新しなければなりません。

<!--
To eliminate this duplication, we can create an abstraction by defining a
function that operates on any list of integers given to it in a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.
-->

この重複を排除するには、引数で与えられた整数のどんなリストに対しても処理が行える関数を定義して抽象化できます。
この解決策によりコードがより明確になり、リストの最大値を探すという概念を抽象的に表現させてくれます。

<!--
In Listing 10-3, we extracted the code that finds the largest number into a
function named `largest`. Unlike the code in Listing 10-1, which can find the
largest number in only one particular list, this program can find the largest
number in two different lists.
-->

リスト 10-3 では、最大値を探すコードを`largest`という関数に抽出しました。リスト 10-1 のコードは、
たった 1 つの特定のリストからだけ最大値を探せますが、それとは異なり、このプログラムは 2 つの異なるリストから最大値を探せます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn largest(list: &[i32]) -> i32 {
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
#    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
#    assert_eq!(result, 6000);
}
```

<!--
<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>
-->

<span class="caption">リスト 10-3: 2 つのリストから最大値を探す抽象化されたコード</span>

<!--
The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values that we might pass into the function. As a
result, when we call the function, the code runs on the specific values that we
pass in.
-->

`largest`関数には`list`と呼ばれる引数があり、これは、関数に渡す可能性のある、あらゆる`i32`値の具体的なスライスを示します。
結果的に、関数呼び出しの際、コードは渡した特定の値に対して走るのです。

<!--
In sum, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:
-->

まとめとして、こちらがリスト 10-2 のコードからリスト 10-3 に変更するのに要したステップです：

<!--
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.
-->

1. 重複したコードを見分ける。
2. 重複コードを関数本体に抽出し、コードの入力と戻り値を関数シグニチャで指定する。
3. 重複したコードの 2 つの実体を代わりに関数を呼び出すように更新する。

<!--
Next, we’ll use these same steps with generics to reduce code duplication in
different ways. In the same way that the function body can operate on an
abstract `list` instead of specific values, generics allow code to operate on
abstract types.
-->

次は、この同じ手順をジェネリクスでも踏んで異なる方法でコードの重複を減らします。
関数本体が特定の値ではなく抽象的な`list`に対して処理できたのと同様に、
ジェネリクスは抽象的な型に対して処理するコードを可能にしてくれます。

<!--
For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!
-->

例えば、関数が 2 つあるとしましょう：1 つは`i32`値のスライスから最大の要素を探し、1 つは`char`値のスライスから最大要素を探します。
この重複はどう排除するのでしょうか？答えを見つけましょう！
