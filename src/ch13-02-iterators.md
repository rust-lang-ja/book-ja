<!--
## Processing a Series of Items with Iterators
-->

## 一連の要素をイテレータで処理する

<!--
The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself.
-->

イテレータパターンにより、一連の要素に順番に何らかの作業を行うことができます。イテレータは、
各要素を繰り返し、シーケンスが終わったことを決定するロジックの責任を負います。イテレータを使用すると、
自身でそのロジックを再実装する必要がなくなるのです。

<!--
In Rust, iterators are *lazy*, meaning they have no effect until you call
methods that consume the iterator to use it up. For example, the code in
Listing 13-10 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything
useful.
-->

Rustにおいて、イテレータは*怠惰*です。つまり、イテレータを使い込んで消費するメソッドを呼ぶまで何の効果もないということです。
例えば、リスト13-10のコードは、`Vec<T>`に定義された`iter`メソッドを呼ぶことで`v1`ベクタの要素に対するイテレータを生成しています。
このコード単独では、何も有用なことはしません。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-10: Creating an iterator</span>
-->

<span class="caption">リスト13-10: イテレータを生成する</span>

<!--
The iterator is stored in the `v1_iter` variable. Once we’ve created an
iterator, we can use it in a variety of ways. In Listing 3-5 in Chapter 3, we
iterated over an array using a `for` loop to execute some code on each of its
items. Under the hood this implicitly created and then consumed an iterator,
but we glossed over how exactly that works until now.
-->

イテレータは`v1_iter`変数に保存されます。一旦イテレータを生成したら、いろんな手段で使用することができます。
第3章のリスト3-5では、`for`ループを使用して配列を走査し、各要素に対して何らかのコードを実行しました。
今までそれが正確にはどう機能するのかごまかしてきましたが、裏で暗黙にイテレータを作成して消費していたのです。

<!--
In the example in Listing 13-11, we separate the creation of the iterator from
the use of the iterator in the `for` loop. When the `for` loop is called using
the iterator in `v1_iter`, each element in the iterator is used in one
iteration of the loop, which prints out each value.
-->

リスト13-11の例では、イテレータの生成と`for`ループでイテレータを使用することを区別しています。
`v1_iter`のイテレータで`for`ループが呼び出された時に、イテレータの各要素がループの繰り返しで使用され、各値が出力されます。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-11: Using an iterator in a `for` loop</span>
-->

<span class="caption">リスト13-11: `for`ループでイテレータを使用する</span>

<!--
In languages that don’t have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
items in the vector.
-->

標準ライブラリにより提供されるイテレータが存在しない言語では、変数を添え字0から始め、
その変数でベクタに添え字アクセスして値を得て、ベクタの総要素数に到達するまでループでその変数の値をインクリメントすることで、
この同じ機能を書く可能性が高いでしょう。

<!--
Iterators handle all that logic for you, cutting down on repetitive code you
could potentially mess up. Iterators give you more flexibility to use the same
logic with many different kinds of sequences, not just data structures you can
index into, like vectors. Let’s examine how iterators do that.
-->

イテレータはそのロジック全てを処理してくれるので、めちゃくちゃにしてしまう可能性のあるコードの繰り返しを減らしてくれます。
イテレータにより、添え字を使えるデータ構造、ベクタなどだけではなく、多くの異なるシーケンスに対して同じロジックを使う柔軟性も得られます。
イテレータがそれをする方法を調査しましょう。

<!--
### The `Iterator` Trait and the `next` Method
-->

### `Iterator`トレイトと`next`メソッド

<!--
All iterators implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:
-->

全てのイテレータは、標準ライブラリで定義されている`Iterator`というトレイトを実装しています。
このトレイトの定義は、以下のようになっています:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // デフォルト実装のあるメソッドは省略
    // methods with default implementations elided
}
```

<!--
Notice this definition uses some new syntax: `type Item` and `Self::Item`,
which are defining an *associated type* with this trait. We’ll talk about
associated types in depth in Chapter 19. For now, all you need to know is that
this code says implementing the `Iterator` trait requires that you also define
an `Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type returned from the
iterator.
-->

この定義は新しい記法を使用していることに注目してください: `type Item`と`Self::Item`で、
これらはこのトレイトとの*関連型*(associated type)を定義しています。関連型についての詳細は、第19章で語ります。
とりあえず、知っておく必要があることは、このコードが`Iterator`トレイトを実装するには、`Item`型も定義する必要があり、
そして、この`Item`型が`next`メソッドの戻り値の型に使われていると述べていることです。換言すれば、
`Item`型がイテレータから返ってくる型になるだろうということです。

<!--
The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`.
-->

`Iterator`トレイトは、一つのメソッドを定義することを実装者に要求することだけします: `next`メソッドで、
これは1度に`Some`に包まれたイテレータの1要素を返し、繰り返しが終わったら、`None`を返します。

<!--
We can call the `next` method on iterators directly; Listing 13-12 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.
-->

イテレータに対して直接`next`メソッドを呼び出すこともできます; リスト13-12は、
ベクタから生成されたイテレータの`next`を繰り返し呼び出した時にどんな値が返るかを模擬しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-12: Calling the `next` method on an
iterator</span>
-->

<span class="caption">リスト13-12: イテレータに対して`next`メソッドを呼び出す</span>

<!--
Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.
-->

`v1_iter`を可変にする必要があったことに注目してください: イテレータの`next`メソッドを呼び出すと、
今シーケンスのどこにいるかを追いかけるためにイテレータが使用している内部の状態が変わります。
つまり、このコードはイテレータを*消費*、または使い込むのです。
`next`の各呼び出しは、イテレータの要素を一つ、食います。`for`ループを使用した時には、
`v1_iter`を可変にする必要はありませんでした。というのも、ループが`v1_iter`の所有権を奪い、
陰で可変にしていたからです。

<!--
Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.
-->

また、`next`の呼び出しで得られる値は、ベクタの値への不変な参照であることにも注目してください。
`iter`メソッドは、不変参照へのイテレータを生成します。`v1`の所有権を奪い、所有された値を返すイテレータを生成したいなら、
`iter`ではなく`into_iter`を呼び出すことができます。同様に、可変参照を繰り返したいなら、
`iter`ではなく`iter_mut`を呼び出せます。

<!--
### Methods that Consume the Iterator
-->

### イテレータを消費するメソッド

<!--
The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you’re required to implement the `next` method when implementing the
`Iterator` trait.
-->

`Iterator`トレイトには、標準ライブラリが提供してくれているデフォルト実装のある多くの異なるメソッドがあります;
`Iterator`トレイトの標準ライブラリのAPIドキュメントを検索することで、これらのメソッドについて知ることができます。
これらのメソッドの中には、定義内で`next`メソッドを呼ぶものもあり、故に`Iterator`トレイトを実装する際には、
`next`メソッドを実装する必要があるのです。

<!--
Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-13 has a
test illustrating a use of the `sum` method:
-->

`next`を呼び出すメソッドは、*消費アダプタ*(consuming adaptors)と呼ばれます。呼び出しがイテレータの使い込みになるからです。
一例は、`sum`メソッドで、これはイテレータの所有権を奪い、`next`を繰り返し呼び出すことで要素を繰り返し、
故にイテレータを消費するのです。繰り返しが進むごとに、各要素を一時的な合計に追加し、
繰り返しが完了したら、その合計を返します。リスト13-13は、`sum`の使用を説明したテストです:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-13: Calling the `sum` method to get the total
of all items in the iterator</span>
-->

<span class="caption">リスト13-13: `sum`メソッドを呼び出してイテレータの全要素の合計を得る</span>

<!--
We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.
-->

`sum`は呼び出し対象のイテレータの所有権を奪うので、`sum`呼び出し後に`v1_iter`を使用することはできません。

<!--
### Methods that Produce Other Iterators
-->

### 他のイテレータを生成するメソッド

<!--
*Iterator adaptors* are methods defined on the `Iterator` trait that don’t
consume the iterator. Instead, they produce different iterators by changing
some aspect of the original iterator.
-->

*イテレータアダプタ*(iterator adaptors)は、`Iterator`トレイト上で定義されたメソッドのうち、イテレータを消費しないものです。
これらはイテレータを消費しない代わりに、元のイテレータの一部の様相を変更することによって、別のイテレータを生成します。

<!--
Listing 13-14 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item as the items are iterated through.
The `map` method returns a new iterator that produces the modified items. The
closure here creates a new iterator in which each item from the vector will be
incremented by 1:
-->

リスト13-14は、イテレータアダプタメソッドの`map`の呼び出し例を示しています。
`map`は、各要素が反復処理されるときに、その要素に対して呼び出すクロージャを取ります。
`map`メソッドは、修正された要素を生成する新しいイテレータを返します。
`map`新しいイテレータを生成します。ここのクロージャは、ベクタの各要素が1インクリメントされる新しいイテレータを作成します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-14: Calling the iterator adaptor `map` to
create a new iterator</span>
-->

<span class="caption">リスト13-14: イテレータアダプタの`map`を呼び出して新規イテレータを作成する</span>

<!--
However, this code produces a warning:
-->

ところが、このコードは警告を発します:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

<!--
The code in Listing 13-14 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.
-->

リスト13-14のコードは何もしません; 指定したクロージャは、決して呼ばれないのです。警告が理由を思い出させてくれています:
イテレータアダプタは怠惰で、ここでイテレータを消費する必要があるのです。

<!--
To fix this warning and consume the iterator, we’ll use the `collect` method,
which we used in Chapter 12 with `env::args` in Listing 12-1. This method
consumes the iterator and collects the resulting values into a collection data
type.
-->

この警告を修正し、イテレータを消費するには、`collect`メソッドを使用しますが、これは第12章のリスト12-1で`env::args`とともに使用しました。
このメソッドはイテレータを消費し、結果の値をコレクションデータ型に集結させます。

<!--
In Listing 13-15, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.
-->

リスト13-15において、`map`呼び出しから返ってきたイテレータを繰り返した結果をベクタに集結させています。
このベクタは、最終的に元のベクタの各要素に1を足したものが含まれます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-15: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>
-->

<span class="caption">リスト13-15: `map`メソッドを呼び出して新規イテレータを作成し、
それから`collect`メソッドを呼び出してその新規イテレータを消費し、ベクタを生成する</span>

<!--
Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.
-->

`map`はクロージャを取るので、各要素に対して行いたいどんな処理も指定することができます。
これは、`Iterator`トレイトが提供する繰り返し動作を再利用しつつ、
クロージャにより一部の動作をカスタマイズできる好例になっています。

<!--
You can chain multiple calls to iterator adaptors to perform complex actions in
a readable way. But because all iterators are lazy, you have to call one of the
consuming adaptor methods to get results from calls to iterator adaptors.
-->

複雑な操作を可読性の高い方法で行うために、イテレータアダプタの呼び出しを複数チェーンすることができます。
ですが、すべてのイテレータは怠惰なので、イテレータアダプタの呼び出しから結果を得るには、消費アダプタメソッドのうちいずれかを呼ぶ必要があります。

<!--
### Using Closures that Capture Their Environment
-->

### 環境をキャプチャするクロージャを使用する

<!--
Many iterator adapters take closures as arguments, and commonly the closures
we’ll specify as arguments to iterator adapters will be closures that capture
their environment.
-->

多くのイテレータアダプタは引数としてクロージャを取り、また、イテレータアダプタに引数として指定するクロージャは、
環境をキャプチャするクロージャであることは、よくあることでしょう。

<!--
For this example, we’ll use the `filter` method that takes a closure. The
closure gets an item from the iterator and returns a `bool`. If the closure
returns `true`, the value will be included in the iteration produced by
`filter`. If the closure returns `false`, the value won’t be included.
-->

この例では、クロージャを取る`filter`メソッドを使用します。クロージャは、イテレータからの要素を取り、`bool`を返すクロージャです。
このクロージャが`true`を返せば、`filter`が生成する繰り返し処理にその値が含まれます。クロージャが`false`を返したら、
その値は含まれません。

<!--
In Listing 13-16, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.
-->

リスト13-16では、環境から`shoe_size`変数をキャプチャするクロージャで`filter`を使って、
`Shoe`構造体インスタンスのコレクションを繰り返しています。指定したサイズの靴だけを返すわけです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

<!--
<span class="caption">Listing 13-16: Using the `filter` method with a closure
that captures `shoe_size`</span>
-->

<span class="caption">リスト13-16: `shoe_size`をキャプチャするクロージャで`filter`メソッドを使用する</span>

<!--
The `shoes_in_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.
-->

`shoes_in_size`関数は、引数として靴のベクタとサイズの所有権を奪います。指定されたサイズの靴だけを含むベクタを返します。

<!--
In the body of `shoes_in_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.
-->

`shoes_in_size`の本体で、`into_iter`を呼び出してベクタの所有権を奪うイテレータを作成しています。
そして、`filter`を呼び出してそのイテレータをクロージャが`true`を返した要素だけを含む新しいイテレータに適合させます。

<!--
The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.
-->

クロージャは、環境から`shoe_size`引数をキャプチャし、指定されたサイズの靴だけを保持しながら、
その値を各靴のサイズと比較します。最後に、`collect`を呼び出すと、
関数により返ってきたベクタに適合させたイテレータから返ってきた値が集まるのです。

<!--
The test shows that when we call `shoes_in_size`, we get back only shoes
that have the same size as the value we specified.
-->

`shoes_in_size`を呼び出した時に、指定した値と同じサイズの靴だけが得られることをテストは示しています。
