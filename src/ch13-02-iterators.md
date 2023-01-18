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
Listing 13-13 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything
useful.
-->

Rust において、イテレータは*怠惰*です。つまり、イテレータを使い込んで消費するメソッドを呼ぶまで何の効果もないということです。
例えば、リスト 13-13 のコードは、`Vec<T>`に定義された`iter`メソッドを呼ぶことで`v1`ベクタの要素に対するイテレータを生成しています。
このコード単独では、何も有用なことはしません。

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<!--
<span class="caption">Listing 13-13: Creating an iterator</span>
-->

<span class="caption">リスト 13-13: イテレータを生成する</span>

<!--
Once we’ve created an iterator, we can use it in a variety of ways. In Listing
3-5 in Chapter 3, we used iterators with `for` loops to execute some code on
each item, although we glossed over what the call to `iter` did until now.
-->

一旦イテレータを生成したら、いろんな手段で使用することができます。第 3 章のリスト 3-5 では、
ここまで`iter`の呼び出しが何をするかごまかしてきましたが、`for`ループでイテレータを使い、
各要素に何かコードを実行しています。

<!--
The example in Listing 13-14 separates the creation of the iterator from the
use of the iterator in the `for` loop. The iterator is stored in the `v1_iter`
variable, and no iteration takes place at that time. When the `for` loop is
called using the iterator in `v1_iter`, each element in the iterator is used in
one iteration of the loop, which prints out each value.
-->

リスト 13-14 の例は、イテレータの生成と`for`ループでイテレータを使用することを区別しています。
イテレータは、`v1_iter`変数に保存され、その時には繰り返しは起きていません。`v1_iter`のイテレータで、
`for`ループが呼び出された時に、イテレータの各要素がループの繰り返しで使用され、各値が出力されます。

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    // {}でした
    println!("Got: {}", val);
}
```

<!--
<span class="caption">Listing 13-14: Using an iterator in a `for` loop
-->

<span class="caption">リスト 13-14: `for`ループでイテレータを使用する</span>

<!--
In languages that don’t have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
item in the vector.
-->

標準ライブラリにより提供されるイテレータが存在しない言語では、変数を添え字 0 から始め、
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
このトレイトの定義は、以下のようになっています：

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

この定義は新しい記法を使用していることに注目してください：`type Item`と`Self::Item`で、
これらはこのトレイトとの*関連型*(associated type) を定義しています。関連型についての詳細は、第 19 章で語ります。
とりあえず、知っておく必要があることは、このコードが`Iterator`トレイトを実装するには、`Item`型も定義する必要があり、
そして、この`Item`型が`next`メソッドの戻り値の型に使われていると述べていることです。換言すれば、
`Item`型がイテレータから返ってくる型になるだろうということです。

<!--
The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`.
-->

`Iterator`トレイトは、一つのメソッドを定義することを実装者に要求することだけします：`next`メソッドで、
これは 1 度に`Some`に包まれたイテレータの 1 要素を返し、繰り返しが終わったら、`None`を返します。

<!--
We can call the `next` method on iterators directly; Listing 13-15 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.
-->

イテレータに対して直接`next`メソッドを呼び出すこともできます; リスト 13-15 は、
ベクタから生成されたイテレータの`next`を繰り返し呼び出した時にどんな値が返るかを模擬しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

<!--
<span class="caption">Listing 13-15: Calling the `next` method on an
iterator</span>
-->

<span class="caption">リスト 13-15: イテレータに対して`next`メソッドを呼び出す</span>

<!--
Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.
-->

`v1_iter`を可変にする必要があったことに注目してください：イテレータの`next`メソッドを呼び出すと、
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
is why we’re required to implement the `next` method when implementing the
`Iterator` trait.
-->

`Iterator`トレイトには、標準ライブラリが提供してくれているデフォルト実装のある多くの異なるメソッドがあります;
`Iterator`トレイトの標準ライブラリの API ドキュメントを検索することで、これらのメソッドについて知ることができます。
これらのメソッドの中には、定義内で`next`メソッドを呼ぶものもあり、故に`Iterator`トレイトを実装する際には、
`next`メソッドを実装する必要があるのです。

<!--
Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-16 has a
test illustrating a use of the `sum` method:
-->

`next`を呼び出すメソッドは、*消費アダプタ*(consuming adaptors) と呼ばれます。呼び出しがイテレータの使い込みになるからです。
一例は、`sum`メソッドで、これはイテレータの所有権を奪い、`next`を繰り返し呼び出すことで要素を繰り返し、
故にイテレータを消費するのです。繰り返しが進むごとに、各要素を一時的な合計に追加し、
繰り返しが完了したら、その合計を返します。リスト 13-16 は、`sum`の使用を説明したテストです：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

<!--
<span class="caption">Listing 13-16: Calling the `sum` method to get the total
of all items in the iterator</span>
-->

<span class="caption">リスト 13-16: `sum`メソッドを呼び出してイテレータの全要素の合計を得る</span>

<!--
We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.
-->

`sum`は呼び出し対象のイテレータの所有権を奪うので、`sum`呼び出し後に`v1_iter`を使用することはできません。

<!--
### Methods that Produce Other Iterators
-->

## 他のイテレータを生成するメソッド

<!--
Other methods defined on the `Iterator` trait, known as *iterator adaptors*,
allow us to change iterators into different kind of iterators. You can chain
multiple calls to iterator adaptors to perform complex actions in a readable
way. But because all iterators are lazy, we have to call one of the consuming
adaptor methods to get results from calls to iterator adaptors.
-->

`Iterator`トレイトに定義された他のメソッドは、*イテレータアダプタ*(iterator adaptors) として知られていますが、
イテレータを別の種類のイテレータに変えさせてくれます。イテレータアダプタを複数回呼ぶ呼び出しを連結して、
複雑な動作を読みやすい形で行うことができます。ですが、全てのイテレータは怠惰なので、消費アダプタメソッドのどれかを呼び出し、
イテレータアダプタの呼び出しから結果を得なければなりません。

<!--
Listing 13-17 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item to produce a new iterator. The
closure here creates a new iterator in which each item from the vector has been
incremented by 1. However, this code produces a warning:
-->

リスト 13-17 は、イテレータアダプタメソッドの`map`の呼び出し例を示し、各要素に対して呼び出すクロージャを取り、
新しいイテレータを生成します。ここのクロージャは、ベクタの各要素が 1 インクリメントされる新しいイテレータを作成します。
ところが、このコードは警告を発します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

<!--
<span class="caption">Listing 13-17: Calling the iterator adaptor `map` to
create a new iterator</span>
-->

<span class="caption">リスト 13-17: イテレータアダプタの`map`を呼び出して新規イテレータを作成する</span>

<!--
The warning we get is this:
-->

出る警告は以下の通りです：

```text
warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy
and do nothing unless consumed
(警告：使用されねばならない`std::iter::Map`が未使用です：イテレータアダプタは怠惰で、
消費されるまで何もしません)
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```

<!--
The code in Listing 13-17 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.
-->

リスト 13-17 のコードは何もしません; 指定したクロージャは、決して呼ばれないのです。警告が理由を思い出させてくれています：
イテレータアダプタは怠惰で、ここでイテレータを消費する必要があるのです。

<!--
To fix this and consume the iterator, we’ll use the `collect` method, which we
used in Chapter 12 with `env::args` in Listing 12-1. This method consumes the
iterator and collects the resulting values into a collection data type.
-->

これを修正し、イテレータを消費するには、`collect`メソッドを使用しますが、これは第 12 章のリスト 12-1 で`env::args`とともに使用しました。
このメソッドはイテレータを消費し、結果の値をコレクションデータ型に集結させます。

<!--
In Listing 13-18, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.
-->

リスト 13-18 において、`map`呼び出しから返ってきたイテレータを繰り返した結果をベクタに集結させています。
このベクタは、最終的に元のベクタの各要素に 1 を足したものが含まれます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<!--
<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>
-->

<span class="caption">リスト 13-18: `map`メソッドを呼び出して新規イテレータを作成し、
それから`collect`メソッドを呼び出してその新規イテレータを消費し、ベクタを生成する</span>

<!--
Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let us customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.
-->

`map`はクロージャを取るので、各要素に対して行いたいどんな処理も指定することができます。
これは、`Iterator`トレイトが提供する繰り返し動作を再利用しつつ、
クロージャにより一部の動作をカスタマイズできる好例になっています。

<!--
### Using Closures that Capture Their Environment
-->

### 環境をキャプチャするクロージャを使用する

<!--
Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adaptor.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a Boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator.
-->

イテレータが出てきたので、`filter`イテレータアダプタを使って環境をキャプチャするクロージャの一般的な使用をデモすることができます。
イテレータの`filter`メソッドは、イテレータの各要素を取り、論理値を返すクロージャを取ります。
このクロージャが`true`を返せば、`filter`が生成するイテレータにその値が含まれます。クロージャが`false`を返したら、
結果のイテレータにその値は含まれません。

<!--
In Listing 13-19 we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.
-->

リスト 13-19 では、環境から`shoe_size`変数をキャプチャするクロージャで`filter`を使って、
`Shoe`構造体インスタンスのコレクションを繰り返しています。指定したサイズの靴だけを返すわけです。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

<!--
<span class="caption">Listing 13-19: Using the `filter` method with a closure
that captures `shoe_size`</span>
-->

<span class="caption">リスト 13-19: `shoe_size`をキャプチャするクロージャで`filter`メソッドを使用する</span>

<!--
The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.
-->

`shoes_in_my_size`関数は、引数として靴のベクタとサイズの所有権を奪います。指定されたサイズの靴だけを含むベクタを返します。

<!--
In the body of `shoes_in_my_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.
-->

`shoes_in_my_size`の本体で、`into_iter`を呼び出してベクタの所有権を奪うイテレータを作成しています。
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
The test shows that when we call `shoes_in_my_size`, we get back only shoes
that have the same size as the value we specified.
-->

`shoes_in_my_size`を呼び出した時に、指定した値と同じサイズの靴だけが得られることをテストは示しています。

<!--
### Creating Our Own Iterators with `Iterator` Trait
-->

### `Iterator`トレイトで独自のイテレータを作成する

<!--
We’ve shown that you can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. You can create iterators from the other collection
types in the standard library, such as hash map. You can also create iterators
that do anything you want by implementing the `Iterator` trait on your own
types. As previously mentioned, the only method you’re required to provide a
definition for is the `next` method. Once you’ve done that, you can use all
other methods that have default implementations provided by the `Iterator`
trait!
-->

ベクタに対し、`iter`、`into_iter`、`iter_mut`を呼び出すことでイテレータを作成できることを示してきました。
ハッシュマップなどの標準ライブラリの他のコレクション型からもイテレータを作成できます。
`Iterator`トレイトを自分で実装することで、したいことを何でもするイテレータを作成することもできます。
前述の通り、定義を提供する必要のある唯一のメソッドは、`next`メソッドなのです。一旦、そうしてしまえば、
`Iterator`トレイトが用意しているデフォルト実装のある他の全てのメソッドを使うことができるのです！

<!--
To demonstrate, let’s create an iterator that will only ever count from 1 to 5.
First, we’ll create a struct to hold some values. Then we’ll make this struct
into an iterator by implementing the `Iterator` trait and use the values in
that implementation.
-->

デモ用に、絶対に 1 から 5 をカウントするだけのイテレータを作成しましょう。まず、値を保持する構造体を生成し、
`Iterator`トレイトを実装することでこの構造体をイテレータにし、その実装内の値を使用します。

<!--
Listing 13-20 has the definition of the `Counter` struct and an associated
`new` function to create instances of `Counter`:
-->

リスト 13-20 は、`Counter`構造体と`Counter`のインスタンスを作る`new`関連関数の定義です：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

<!--
<span class="caption">Listing 13-20: Defining the `Counter` struct and a `new`
function that creates instances of `Counter` with an initial value of 0 for
`count`</span>
-->

<span class="caption">リスト 13-20: `Counter`構造体と`count`に対して 0 という初期値で`Counter`のインスタンスを作る`new`関数を定義する</span>

<!--
The `Counter` struct has one field named `count`. This field holds a `u32`
value that will keep track of where we are in the process of iterating from 1
to 5. The `count` field is private because we want the implementation of
`Counter` to manage its value. The `new` function enforces the behavior of
always starting new instances with a value of 0 in the `count` field.
-->

`Counter`構造体には、`count`というフィールドがあります。このフィールドは、
1 から 5 までの繰り返しのどこにいるかを追いかける`u32`値を保持しています。`Counter`の実装にその値を管理してほしいので、
`count`フィールドは非公開です。`count`フィールドは常に 0 という値から新規インスタンスを開始するという動作を`new`関数は強要します。

<!--
Next, we’ll implement the `Iterator` trait for our `Counter` type by defining
the body of the `next` method to specify what we want to happen when this
iterator is used, as shown in Listing 13-21:
-->

次に、`next`メソッドの本体をこのイテレータが使用された際に起きてほしいことを指定するように定義して、
`Counter`型に対して`Iterator`トレイトを実装します。リスト 13-21 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<!--
<span class="caption">Listing 13-21: Implementing the `Iterator` trait on our
`Counter` struct</span>
-->

<span class="caption">リスト 13-21: `Counter`構造体に`Iterator`トレイトを実装する</span>

<!--
We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll cover them in Chapter 19.
-->

イテレータの`Item`関連型を`u32`に設定しました。つまり、イテレータは、`u32`の値を返します。
ここでも、まだ関連型について心配しないでください。第 19 章で講義します。

<!--
We want our iterator to add 1 to the current state, so we initialized `count`
to 0 so it would return 1 first. If the value of `count` is less than 6, `next`
will return the current value wrapped in `Some`, but if `count` is 6 or higher,
our iterator will return `None`.
-->

イテレータに現在の状態に 1 を足してほしいので、まず 1 を返すように`count`を 0 に初期化しました。
`count`の値が 5 以下なら、`next`は`Some`に包まれた現在の値を返しますが、
`count`が 6 以上なら、イテレータは`None`を返します。

<!--
#### Using Our `Counter` Iterator’s `next` Method
-->

#### `Counter`イテレータの`next`メソッドを使用する

<!--
Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality of our
`Counter` struct by calling the `next` method on it directly, just as we did
with the iterator created from a vector in Listing 13-15.
-->

一旦`Iterator`トレイトを実装し終わったら、イテレータの出来上がりです！リスト 13-22 は、
リスト 13-15 のベクタから生成したイテレータと全く同様に、直接`next`メソッドを呼び出すことで、
`Counter`構造体のイテレータ機能を使用できることをデモするテストを示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Iterator for Counter {
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         self.count += 1;
#
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

<!--
<span class="caption">Listing 13-22: Testing the functionality of the `next`
method implementation</span>
-->

<span class="caption">リスト 13-22: `next`メソッド実装の機能をテストする</span>

<!--
This test creates a new `Counter` instance in the `counter` variable and then
calls `next` repeatedly, verifying that we have implemented the behavior we
want this iterator to have: returning the values from 1 to 5.
-->

このテストは、`counter`変数に新しい`Counter`インスタンスを生成し、
それからイテレータにほしい動作が実装し終わっていることを実証しながら、`next`を繰り返し呼び出しています：
1 から 5 の値を返すことです。

<!--
#### Using Other `Iterator` Trait Methods
-->

#### 他の`Iterator`トレイトメソッドを使用する

<!--
We implemented the `Iterator` trait by defining the `next` method, so we
can now use any `Iterator` trait method’s default implementations as defined in
the standard library, because they all use the `next` method’s functionality.
-->

`next`メソッドを定義して`Iterator`トレイトを実装したので、今では、標準ライブラリで定義されているように、
どんな`Iterator`トレイトメソッドのデフォルト実装も使えるようになりました。全て`next`メソッドの機能を使っているからです。

<!--
For example, if for some reason we wanted to take the values produced by an
instance of `Counter`, pair them with values produced by another `Counter`
instance after skipping the first value, multiply each pair together, keep only
those results that are divisible by 3, and add all the resulting values
together, we could do so, as shown in the test in Listing 13-23:
-->

例えば、何らかの理由で、`Counter`インスタンスが生成する値を取り、最初の値を飛ばしてから、
別の`Counter`インスタンスが生成する値と一組にし、各ペアを掛け算し、3 で割り切れる結果だけを残し、
全結果の値を足し合わせたくなったら、リスト 13-23 のテストに示したように、そうすることができます：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

<!--
check to see は畳語だが、いい訳はあるだろうか
-->

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Counter {
#     fn new() -> Counter {
#         Counter { count: 0 }
#     }
# }
#
# impl Iterator for Counter {
#     // このイテレータは u32 を生成します
#     // Our iterator will produce u32s
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         // カウントをインクリメントする。故に 0 から始まる
#         // increment our count. This is why we started at zero.
#         self.count += 1;
#
#         // カウントが終わったかどうか確認する
#         // check to see if we've finished counting or not.
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

<!--
<span class="caption">Listing 13-23: Using a variety of `Iterator` trait
methods on our `Counter` iterator</span>
-->

<span class="caption">リスト 13-23: `Counter`イテレータに対していろんな`Iterator`トレイトのメソッドを使用する</span>

<!--
Note that `zip` produces only four pairs; the theoretical fifth pair `(5,
None)` is never produced because `zip` returns `None` when either of its input
iterators return `None`.
-->

`zip`は 4 組しか生成しないことに注意してください; 理論的な 5 番目の組の`(5, None)`は、
入力イテレータのどちらかが`None`を返したら、`zip`は`None`を返却するため、決して生成されることはありません。

<!--
All of these method calls are possible because we specified how the `next`
method works, and the standard library provides default implementations for
other methods that call `next`.
-->

`next`メソッドの動作方法を指定し、標準ライブラリが`next`を呼び出す他のメソッドにデフォルト実装を提供しているので、
これらのメソッド呼び出しは全て可能です。
