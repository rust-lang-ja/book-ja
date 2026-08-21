<!--
# Generic Types, Traits, and Lifetimes
-->

# ジェネリック型、トレイト、ライフタイム

<!--
Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is *generics*: abstract stand-ins for
concrete types or other properties. We can express the behavior of generics or
how they relate to other generics without knowing what will be in their place
when compiling and running the code.
-->

全てのプログラミング言語には、概念の重複を効率的に扱う道具があります。
Rustにおいて、そのような道具の一つが*ジェネリクス*です。
ジェネリクスは、具体型や他のプロパティの抽象的な代役です。コンパイルやコード実行時に、
ジェネリクスの位置に何が入るかを知ることなく、ジェネリクスの振る舞いや他のジェネリクスとの関係を表現できるのです。

<!--
Functions can take parameters of some generic type, instead of a concrete type
like `i32` or `String`, in the same way a function takes parameters with
unknown values to run the same code on multiple concrete values. In fact, we’ve
already used generics in Chapter 6 with `Option<T>`, Chapter 8 with `Vec<T>`
and `HashMap<K, V>`, and Chapter 9 with `Result<T, E>`. In this chapter, you’ll
explore how to define your own types, functions, and methods with generics!
-->

関数が未知の値の引数を取り、同じコードを複数の具体的な値に対して走らせるように、
`i32`や`String`などの具体的な型の代わりに何かジェネリックな型の引数を取ることができます。
実際、第6章で`Option<T>`、第8章で`Vec<T>`と`HashMap<K, V>`、第9章で`Result<T, E>`を既に使用しました。
この章では、独自の型、関数、メソッドをジェネリクスとともに定義する方法を探究します！

<!--
First, we’ll review how to extract a function to reduce code duplication. We’ll
then use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We’ll also explain how to use
generic types in struct and enum definitions.
-->

まず、関数を抽出して、コードの重複を減らす方法を確認しましょう。次に同じテクニックを活用して、
引数の型のみが異なる2つの関数からジェネリックな関数を生成します。また、
ジェネリックな型を構造体やenum定義で使用する方法も説明します。

<!--
Then you’ll learn how to use *traits* to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to accept
only those types that have a particular behavior, as opposed to just any type.
-->

それから、トレイトを使用して、ジェネリックな方法で振る舞いを定義する方法を学びます。
ジェネリックな型にトレイトを組み合わせることで、ジェネリックな型を、単にあらゆる型に対してではなく、特定の振る舞いのある型のみを受け付けるように制限できます。

<!--
Finally, we’ll discuss *lifetimes*: a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to give the compiler enough information about borrowed values so that it can
ensure references will be valid in more situations than it could without our
help.
-->

最後に、ライフタイムを議論します。ライフタイムとは、コンパイラに参照がお互いにどう関係しているかの情報を与える一種のジェネリクスです。
ライフタイムを利用することで、借用された値についての十分な情報をコンパイラに渡すことができ、
プログラマの支援がなくてもできる範囲よりもより多くの場面で、参照が有効であることを保証できます。

<!--
## Removing Duplication by Extracting a Function
-->

## 関数を抽出することで重複を取り除く

<!--
Generics allow us to replace specific types with a placeholder that represents
multiple types to remove code duplication. Before diving into generics syntax,
then, let’s first look at how to remove duplication in a way that doesn’t
involve generic types by extracting a function that replaces specific values
with a placeholder that represents multiple values. Then we’ll apply the same
technique to extract a generic function! By looking at how to recognize
duplicated code you can extract into a function, you’ll start to recognize
duplicated code that can use generics.
-->

ジェネリクスを使用すると、複数の型を表現するプレースホルダで特定の型を置き換えることで、コードの重複を取り除くことができます。
ジェネリクスの記法に飛び込む前にまずは、複数の値を表現するプレースホルダで特定の値を置き換えて関数を抽出することで、ジェネリックな型が関わらない重複を取り除く方法を見ましょう。
そして、同じテクニックを適用してジェネリックな関数を抽出するのです！
関数に抽出できる重複したコードを認識する方法を見てみることで、ジェネリクスを使用できる重複コードも認識できるようになるでしょう。

<!--
We begin with the short program in Listing 10-1 that finds the largest number
in a list.
-->

リスト10-1の、リスト内の最大値を求める短いプログラムで始めましょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-1: Finding the largest number in a list of
numbers</span>
-->

<span class="caption">リスト10-1: 数字のリストから最大値を求める</span>

<!--
We store a list of integers in the variable `number_list` and place a reference
to the first number in the list in a variable named `largest`. We then iterate
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, replace the reference in that variable.
However, if the current number is less than or equal to the largest number seen
so far, the variable doesn’t change, and the code moves on to the next number
in the list. After considering all the numbers in the list, `largest` should
refer to the largest number, which in this case is 100.
-->

このコードは、整数のリストを変数`number_list`に格納し、リストの最初の数への参照を`largest`という変数に配置しています。
それからリストの数全部を走査し、現在の数が`largest`に格納された数値よりも大きければ、
その変数に代入されている参照を置き換えます。ですが、現在の数値が今まで見た最大値以下であれば、
変数は変わらず、コードはリストの次の数値に移っていきます。リストの数値全てを吟味した後、
`largest`は最大値を参照しているはずで、今回は100になります。

<!--
We've now been tasked with finding the largest number in two different lists of
numbers. To do so, we can choose to duplicate the code in Listing 10-1 and use
the same logic at two different places in the program, as shown in Listing 10-2.
-->

次は2つの異なる数値のリストから最大値を発見するタスクが課されました。
これを行うために、リスト10-1のコードを複製し、プログラムの異なる2箇所で同じロジックを使用するという手があります。リスト10-2のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<!--
<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>
-->

<span class="caption">リスト10-2: *2つ*の数値のリストから最大値を探すコード</span>

<!--
Although this code works, duplicating code is tedious and error prone. We also
have to remember to update the code in multiple places when we want to change
it.
-->

このコードは動くものの、コードを複製することは退屈ですし、間違いも起きやすいです。
また、コードを変更したい時には、複数箇所更新することを覚えておかなくてはなりません。

<!--
To eliminate this duplication, we’ll create an abstraction by defining a
function that operates on any list of integers passed in a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.
-->

この重複を排除するには、引数で与えられた整数のどんなリストに対しても処理が行える関数を定義して抽象化しましょう。
この解決策によりコードがより明確になり、リストの最大値を探すという概念を抽象的に表現させてくれます。

<!--
In Listing 10-3, we extract the code that finds the largest number into a
function named `largest`. Then we call the function to find the largest number
in the two lists from Listing 10-2. We could also use the function on any other
list of `i32` values we might have in the future.
-->

リスト10-3では、最大値を探すコードを`largest`という関数に抽出しています。
そして、リスト10-2の2つのリストから最大値を探すために、この関数を呼んでいます。
将来持つことになるかもしれない他のどんな`i32`値のリストに対しても、この関数を使用することができます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>
-->

<span class="caption">リスト10-3: 2つのリストから最大値を探す抽象化されたコード</span>

<!--
The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values we might pass into the function. As a result,
when we call the function, the code runs on the specific values that we pass
in.
-->

`largest`関数には`list`と呼ばれる引数があり、これは、関数に渡す可能性のある、あらゆる`i32`値の具体的なスライスを示します。
結果的に、関数呼び出しの際、コードは渡した特定の値に対して走るのです。

<!--
In summary, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:
-->

まとめとして、こちらがリスト10-2のコードからリスト10-3に変更するのに要したステップです:

<!--
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
   inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.
-->

1. 重複したコードを見分ける。
2. 重複コードを関数本体に抽出し、コードの入力と戻り値を関数シグニチャで指定する。
3. 重複したコードの2つの実体を代わりに関数を呼び出すように更新する。

<!--
Next, we’ll use these same steps with generics to reduce code duplication. In
the same way that the function body can operate on an abstract `list` instead
of specific values, generics allow code to operate on abstract types.
-->

次は、この同じ手順をジェネリクスでも踏んでコードの重複を減らします。
関数本体が特定の値ではなく抽象的な`list`に対して処理できたのと同様に、
ジェネリクスは抽象的な型に対して処理するコードを可能にしてくれます。

<!--
For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!
-->

例えば、関数が2つあるとしましょう: 1つは`i32`値のスライスから最大の要素を探し、1つは`char`値のスライスから最大要素を探します。
この重複はどう排除するのでしょうか？答えを見つけましょう！
