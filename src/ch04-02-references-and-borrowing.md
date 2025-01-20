<!--
## References and Borrowing
-->

## 参照と借用

<!--
The issue with the tuple code in Listing 4-5 is that we have to return the
`String` to the calling function so we can still use the `String` after the
call to `calculate_length`, because the `String` was moved into
`calculate_length`.
-->

リスト4-5のタプルコードの問題は、`String`型を呼び出し元の関数に戻さないと、`calculate_length`を呼び出した後に、
`String`オブジェクトが使えなくなることであり、これは`String`オブジェクトが`calculate_length`にムーブされてしまうためでした。

<!--
Here is how you would define and use a `calculate_length` function that has a
reference to an object as a parameter instead of taking ownership of the
value:
-->

ここで、値の所有権をもらう代わりに引数としてオブジェクトへの参照を取る`calculate_length`関数を定義し、
使う方法を見てみましょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

<!--
First, notice that all the tuple code in the variable declaration and the
function return value is gone. Second, note that we pass `&s1` into
`calculate_length` and, in its definition, we take `&String` rather than
`String`.
-->

まず、変数宣言と関数の戻り値にあったタプルコードは全てなくなったことに気付いてください。
2番目に、`&s1`を`calcuate_length`に渡し、その定義では、`String`型ではなく、`&String`を受け取っていることに注目してください。

<!--
These ampersands are *references*, and they allow you to refer to some value
without taking ownership of it. Figure 4-5 shows a diagram.
-->

これらのアンド記号が参照であり、これのおかげで所有権をもらうことなく値を参照することができるのです。
図4-5はその図解です。

<!--
<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />
-->

<img alt="文字列s1を指す&amp;String型のs" src="img/trpl04-05.svg" class="center" />

<!--
<span class="caption">Figure 4-5: A diagram of `&String s` pointing at `String
s1`</span>
-->

<span class="caption">図4-5: `String s1`を指す`&String s`の図表</span>

<!--
Note: The opposite of referencing by using `&` is *dereferencing*, which is
accomplished with the dereference operator, `*`. We’ll see some uses of the
dereference operator in Chapter 8 and discuss details of dereferencing in
Chapter 15.
-->

> 注釈: `&`による参照の逆は、*参照外し*であり、参照外し演算子の`*`で達成できます。
> 第8章で参照外し演算子の使用例を眺め、第15章で参照外しについて詳しく議論します。

<!--
Let’s take a closer look at the function call here:
-->

ここの関数呼び出しについて、もっと詳しく見てみましょう:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

<!--
The `&s1` syntax lets us create a reference that *refers* to the value of `s1`
but does not own it. Because it does not own it, the value it points to will
not be dropped when the reference goes out of scope.
-->

この`&s1`という記法により、`s1`の値を*参照する*参照を生成することができますが、これを所有することはありません。
所有してないということは、指している値は、参照がスコープを抜けてもドロップされないということです。

<!--
Likewise, the signature of the function uses `&` to indicate that the type of
the parameter `s` is a reference. Let’s add some explanatory annotations:
-->

同様に、関数のシグニチャでも、`&`を使用して引数`s`の型が参照であることを示しています。
説明的な注釈を加えてみましょう:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

<!--
The scope in which the variable `s` is valid is the same as any function
parameter's scope, but we don’t drop what the reference points to when it goes
out of scope because we don’t have ownership. When functions have references as
parameters instead of the actual values, we won’t need to return the values in
order to give back ownership, since we never had ownership.
-->

変数`s`が有効なスコープは、あらゆる関数の引数のものと同じですが、所有権はないので、`s`がスコープを抜けても、
参照が指しているものをドロップすることはありません。関数が実際の値の代わりに参照を引数に取ると、
所有権をもらわないので、所有権を返す目的で値を返す必要はありません。

<!--
We call having references as function parameters *borrowing*. As in real life,
if a person owns something, you can borrow it from them. When you’re done, you
have to give it back.
-->

関数の引数に参照を取ることを*借用*と呼びます。現実生活のように、誰かが何かを所有していたら、
それを借りることができます。用が済んだら、返さなきゃいけないわけです。

<!--
So what happens if we try to modify something we’re borrowing? Try the code in
Listing 4-6. Spoiler alert: it doesn’t work!
-->

では、借用した何かを変更しようとしたら、どうなるのでしょうか？リスト4-6のコードを試してください。
ネタバレ注意: 動きません！

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<!--
<span class="caption">Listing 4-6: Attempting to modify a borrowed value</span>
-->

<span class="caption">リスト4-6: 借用した値を変更しようと試みる</span>

<!--
Here’s the error:
-->

これがエラーです:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

<!--
Just as variables are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.
-->

変数が標準で不変なのと全く同様に、参照も不変なのです。参照している何かを変更することは叶わないわけです。

<!--
### Mutable References
-->

### 可変な参照

<!--
We can fix the error in the code from Listing 4-6 with just a small tweak:
-->

一捻り加えるだけでリスト4-6のコードのエラーは解決します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

<!--
First, we had to change `s` to be `mut`. Then we had to create a mutable
reference with `&mut s` and accept a mutable reference with `some_string: &mut
String`.
-->

始めに、`s`を`mut`に変えなければなりませんでした。そして、`&mut s`で可変な参照を生成し、
`some_string: &mut String`で可変な参照を受け入れなければなりませんでした。

<!--
But mutable references have one big restriction: you can have only one mutable
reference to a particular piece of data in a particular scope. This code will
fail:
-->

ところが、可変な参照には大きな制約が一つあります: 特定のスコープで、ある特定のデータに対しては、
一つしか可変な参照を持てないことです。こちらのコードは失敗します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

<!--
Here’s the error:
-->

これがエラーです:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

<!--
This restriction allows for mutation but in a very controlled fashion. It’s
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like.
-->

この制約は、可変化を許可するものの、それを非常に統制の取れた形で行えます。これは、新たなRustaceanにとっては、
壁です。なぜなら、多くの言語では、いつでも好きな時に可変化できるからです。

<!--
The benefit of having this restriction is that Rust can prevent data races at
compile time. A *data race* is similar to a race condition and happens when
these three behaviors occur:
-->

この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。
データ競合とは、競合条件と類似していて、これら3つの振る舞いが起きる時に発生します:

<!--
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.
-->

* 2つ以上のポインタが同じデータに同時にアクセスする。
* 少なくとも一つのポインタがデータに書き込みを行っている。
* データへのアクセスを同期する機構が使用されていない。

<!--
Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem
from happening because it won’t even compile code with data races!
-->

データ競合は未定義の振る舞いを引き起こし、実行時に追いかけようとした時に特定し解決するのが難しい問題です。
しかし、Rustは、データ競合が起こるコードをコンパイルさえしないので、この問題が発生しないようにしてくれるわけです。

<!--
As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not *simultaneous* ones:
-->

いつものように、波かっこを使って新しいスコープを生成し、*同時並行*なものでなく、複数の可変な参照を作ることができます。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

<!--
A similar rule exists for combining mutable and immutable references. This code
results in an error:
-->

可変と不変な参照を組み合わせることに関しても、似たような規則が存在しています。このコードはエラーになります:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

<!--
Here’s the error:
-->

これがエラーです:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

<!--
Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! However, multiple immutable references are okay because no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.
-->

ふう！*さらに*不変な参照をしている間は、可変な参照をすることはできません。不変参照の使用者は、
それ以降に値が突然変わることなんて予想してません！しかしながら、複数の不変参照をすることは可能です。
データを読み込んでいるだけの人に、他人がデータを読み込むことに対して影響を与える能力はないからです。

<!--
Note that a reference’s scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references, the `println!`,
occurs before the mutable reference is introduced:
-->

参照のスコープは代入されたときに始まり、最後に使用されたときまで続くことに注意してください。例えば次のコードは、`println!`で不変参照が使い終わったあとに可変参照に代入されるため、コンパイルできます。

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

<!--
The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don’t overlap, so this code is allowed. The ability of
the compiler to tell that a reference is no longer being used at a point before
the end of the scope is called *Non-Lexical Lifetimes* (NLL for short), and you
can read more about it in [The Edition Guide][nll].
-->

不変参照`r1`・`r2`のスコープは最後に使われた`println!`のあと、つまり可変参照`r3`が使われる直前で終わります。
これらのスコープは重複しないのでこのコードは許容されます。
参照がスコープの終了する前までに使われなくなることをコンパイラが通知する機能は*Non-Lexical Lifetimes* (NLL)と呼ばれ、詳しくは[The Edition Guide][nll]で読むことができます。


<!--
Even though these errors may be frustrating at times, remember that it’s the
Rust compiler pointing out a potential bug early (at compile time rather than
at runtime) and showing you exactly where the problem is. Then you don't have
to track down why your data isn’t what you thought it was.
-->

これらのエラーは、時としてイライラするものではありますが、Rustコンパイラがバグの可能性を早期に指摘してくれ(それも実行時ではなくコンパイル時に)、
問題の発生箇所をズバリ示してくれるのだと覚えておいてください。そうして想定通りにデータが変わらない理由を追いかける必要がなくなります。

<!--
### Dangling References
-->

### 宙に浮いた参照

<!--
In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.
-->

ポインタのある言語では、誤ってダングリングポインタを生成してしまいやすいです。ダングリングポインタとは、
他人に渡されてしまった可能性のあるメモリを指すポインタのことであり、その箇所へのポインタを保持している間に、
メモリを解放してしまうことで発生します。対照的にRustでは、コンパイラが、
参照がダングリング参照に絶対ならないよう保証してくれます: つまり、何らかのデータへの参照があったら、
コンパイラは参照がスコープを抜けるまで、データがスコープを抜けることがないよう確認してくれるわけです。

<!--
Let’s try to create a dangling reference, which Rust will prevent with a
compile-time error:
-->

ダングリング参照作りを試してみますが、コンパイラはこれをコンパイルエラーで阻止します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

<!--
Here’s the error:
-->

こちらがエラーです:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

<!--
This error message refers to a feature we haven’t covered yet: *lifetimes*. We'll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:
-->

このエラーメッセージは、まだ講義していない機能について触れています: *ライフタイム*です。
ライフタイムについては第10章で詳しく議論しますが、ライフタイムに関する部分を無視すれば、
このメッセージは、確かにこのコードが問題になる理由に関する鍵を握っています:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

<!--
Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:
-->

`dangle`コードの各段階で一体何が起きているのかを詳しく見ていきましょう:

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

<!--
Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.
-->

`s`は、`dangle`内で生成されているので、`dangle`のコードが終わったら、`s`は解放されてしまいますが、
そこへの参照を返そうとしました。つまり、この参照は無効な`String`を指していると思われるのです。
よくないことです！コンパイラは、これを阻止してくれるのです。

<!--
The solution here is to return the `String` directly:
-->

ここでの解決策は、`String`を直接返すことです:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

<!--
This works without any problems. Ownership is moved out, and nothing is
deallocated.
-->

これは何の問題もなく動きます。所有権はムーブされ、何も解放されることはありません。

<!--
### The Rules of References
-->

### 参照の規則

<!--
Let’s recap what we’ve discussed about references:
-->

参照について議論したことを再確認しましょう:

<!--
* At any given time, you can have *either* one mutable reference *or* any
number of immutable references.
* References must always be valid.
-->

* 任意のタイミングで、一つの可変参照*か*不変な参照いくつでもの*どちらか*を行える。
* 参照は常に有効でなければならない。

<!--
Next, we’ll look at a different kind of reference: slices.
-->

次は、違う種類の参照を見ていきましょう: スライスです。

[nll]: https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html
