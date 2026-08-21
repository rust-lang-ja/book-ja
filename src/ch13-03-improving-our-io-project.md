<!--
## Improving Our I/O Project
-->

## 入出力プロジェクトを改善する

<!--
ここでは、withを条件のように訳している。(今まではなかったのに、)今はある状態で -> ...があればという意訳である
やはりwithは状態を表すだけなので、強すぎる気がしなくもない
-->

<!--
With this new knowledge about iterators, we can improve the I/O project in
Chapter 12 by using iterators to make places in the code clearer and more
concise. Let’s look at how iterators can improve our implementation of the
`Config::build` function and the `search` function.
-->

このイテレータに関する新しい知識があれば、イテレータを使用してコードのいろんな場所をより明確で簡潔にすることで、
第12章の入出力プロジェクトを改善することができます。イテレータが`Config::build`関数と`search`関数の実装を改善する方法に目を向けましょう。

<!--
### Removing a `clone` Using an Iterator
-->

### イテレータを使用して`clone`を取り除く

<!--
In Listing 12-6, we added code that took a slice of `String` values and created
an instance of the `Config` struct by indexing into the slice and cloning the
values, allowing the `Config` struct to own those values. In Listing 13-17,
we’ve reproduced the implementation of the `Config::build` function as it was
in Listing 12-23:
-->

リスト12-6において、スライスに添え字アクセスして値をクローンすることで、`Config`構造体に値を所有させながら、
`String`値のスライスを取り、`Config`構造体のインスタンスを作るコードを追記しました。リスト13-17では、
リスト12-23のような`Config::build`の実装を再現しました:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

<!--
<span class="caption">Listing 13-17: Reproduction of the `Config::build`
function from Listing 12-23</span>
-->

<span class="caption">リスト13-17: リスト12-23から`Config::build`関数の再現</span>

<!--
At the time, we said not to worry about the inefficient `clone` calls because
we would remove them in the future. Well, that time is now!
-->

その際、将来的に除去する予定なので、非効率的な`clone`呼び出しを憂慮するなと述べました。
えっと、その時は今です！

<!--
We needed `clone` here because we have a slice with `String` elements in the
parameter `args`, but the `build` function doesn’t own `args`. To return
ownership of a `Config` instance, we had to clone the values from the `query`
and `file_path` fields of `Config` so the `Config` instance can own its values.
-->

引数`args`に`String`要素のスライスがあるためにここで`clone`が必要だったのですが、
`build`関数は`args`を所有していません。`Config`インスタンスの所有権を返すためには、
`Config`インスタンスがその値を所有できるように、`Config`の`query`と`file_path`フィールドから値をクローンしなければなりませんでした。

<!--
ここも節冒頭と同様。やはり強すぎるか？
-->

<!--
With our new knowledge about iterators, we can change the `build` function to
take ownership of an iterator as its argument instead of borrowing a slice.
We’ll use the iterator functionality instead of the code that checks the length
of the slice and indexes into specific locations. This will clarify what the
`Config::build` function is doing because the iterator will access the values.
-->

イテレータについての新しい知識があれば、`build`関数をスライスを借用する代わりに、
引数としてイテレータの所有権を奪うように変更することができます。スライスの長さを確認し、
特定の場所に添え字アクセスするコードの代わりにイテレータの機能を使います。これにより、
イテレータは値にアクセスするので、`Config::build`関数がすることが明確化します。

<!--
Once `Config::build` takes ownership of the iterator and stops using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.
-->

ひとたび、`Config::build`がイテレータの所有権を奪い、借用する添え字アクセス処理をやめたら、
`clone`を呼び出して新しくメモリ確保するのではなく、イテレータからの`String`値を`Config`にムーブできます。

<!--
#### Using the Returned Iterator Directly
-->

#### 返却されるイテレータを直接使う

<!--
Open your I/O project’s *src/main.rs* file, which should look like this:
-->

入出力プロジェクトの*src/main.rs*ファイルを開いてください。こんな見た目のはずです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

<!--
We’ll first change the start of the `main` function that we had in Listing
12-24 to the code in Listing 13-18, which this time uses an iterator. This
won’t compile until we update `Config::build` as well.
-->

まずはリスト12-24のような`main`関数の冒頭を、今回はイテレータを使用するリスト13-18のコードに変更します。
これは、`Config::build`も更新するまでコンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-18: Passing the return value of `env::args` to
`Config::build`</span>
-->

<span class="caption">リスト13-18: `env::args`の戻り値を`Config::build`に渡す</span>

<!--
The `env::args` function returns an iterator! Rather than collecting the
iterator values into a vector and then passing a slice to `Config::build`, now
we’re passing ownership of the iterator returned from `env::args` to
`Config::build` directly.
-->

`env::args`関数は、イテレータを返します！イテレータの値をベクタに集結させ、それからスライスを`Config::build`に渡すのではなく、
今では`env::args`から返ってくるイテレータの所有権を直接`Config::build`に渡しています。

<!--
Next, we need to update the definition of `Config::build`. In your I/O
project’s *src/lib.rs* file, let’s change the signature of `Config::build` to
look like Listing 13-19. This still won’t compile because we need to update the
function body.
-->

次に、`Config::build`の定義を更新する必要があります。入出力プロジェクトの*src/lib.rs*ファイルで、
`Config::build`のシグニチャをリスト13-19のように変えましょう。関数本体を更新する必要があるので、
それでもコンパイルはできません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-19: Updating the signature of `Config::build`
to expect an iterator</span>
-->

<span class="caption">リスト13-19: `Config::build`のシグニチャをイテレータを期待するように更新する</span>

<!--
The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`, and that type implements
the `Iterator` trait and returns `String` values.
-->

`env::args`関数の標準ライブラリドキュメントは、この関数が返すイテレータの型は`std::env::Args`であること、
そしてこの型は`String`値を返す`Iterator`トレイトを実装していることを示しています。

<!--
We’ve updated the signature of the `Config::build` function so the parameter
`args` has a generic type with the trait bounds `impl Iterator<Item = String>`
instead of `&[String]`. This usage of the `impl Trait` syntax we discussed in
the [“Traits as Parameters”][impl-trait] section of Chapter 10
means that `args` can be any type that implements the `Iterator` type and
returns `String` items.
-->

引数`args`の型が`&[String]`ではなく、トレイト境界`impl Iterator<Item = String>`を持つジェネリック型を持つように、
`Config::build`関数のシグニチャを更新しています。第10章の[「引数としてのトレイト」][impl-trait]節で議論した`impl Trait`構文のここでの使用は、
`args`は`Iterator`型を実装し`String`要素を返す任意の型でよいことを意味します。


<!--
Because we’re taking ownership of `args` and we’ll be mutating `args` by
iterating over it, we can add the `mut` keyword into the specification of the
`args` parameter to make it mutable.
-->

`args`の所有権を奪い、繰り返しを行うことで`args`を可変化する予定なので、
`args`引数の仕様に`mut`キーワードを追記でき、可変にします。

<!--
#### Using `Iterator` Trait Methods Instead of Indexing
-->

#### 添え字の代わりに`Iterator`トレイトのメソッドを使用する

<!--
Next, we’ll fix the body of `Config::build`. Because `args` implements the
`Iterator` trait, we know we can call the `next` method on it! Listing 13-20
updates the code from Listing 12-23 to use the `next` method:
-->

次に、`Config::build`の本体を修正しましょう。`args`は`Iterator`トレイトを実装しているので、
それに対して`next`メソッドを呼び出せることがわかります！リスト13-20は、
リスト12-23のコードを`next`メソッドを使用するように更新したものです:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-20: Changing the body of `Config::build` to use
iterator methods</span>
-->

<span class="caption">リスト13-20: `Config::build`の本体をイテレータメソッドを使うように変更する</span>

<!--
6行目真ん中のandを順接の理由で訳している。
-->

<!--
Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Second, we call `next` to get the
value we want to put in the `query` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value. We do
the same thing for the `file_path` value.
-->

`env::args`の戻り値の1番目の値は、プログラム名であることを思い出してください。それは無視し、
次の値を取得したいので、まず`next`を呼び出し、戻り値に対して何もしません。2番目に、
`next`を呼び出して`Config`の`query`フィールドに置きたい値を得ます。`next`が`Some`を返したら、
`match`を使用してその値を抜き出します。`None`を返したら、十分な引数が与えられなかったということなので、
`Err`値で早期リターンします。`file_path`値に対しても同じことをします。

<!--
### Making Code Clearer with Iterator Adaptors
-->

### イテレータアダプタでコードをより明確にする

<!--
We can also take advantage of iterators in the `search` function in our I/O
project, which is reproduced here in Listing 13-21 as it was in Listing 12-19:
-->

入出力プロジェクトの`search`関数でも、イテレータを活用することができます。その関数はリスト12-19に示していますが、以下のリスト13-28に再掲します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<!--
<span class="caption">Listing 13-21: The implementation of the `search`
function from Listing 12-19</span>
-->

<span class="caption">リスト13-21: リスト12-19の`search`関数の実装</span>

<!--
We can write this code in a more concise way using iterator adaptor methods.
Doing so also lets us avoid having a mutable intermediate `results` vector. The
functional programming style prefers to minimize the amount of mutable state to
make code clearer. Removing the mutable state might enable a future enhancement
to make searching happen in parallel, because we wouldn’t have to manage
concurrent access to the `results` vector. Listing 13-22 shows this change:
-->

イテレータアダプタメソッドを使用して、このコードをもっと簡潔に書くことができます。そうすれば、
可変な中間の`results`ベクタをなくすこともできます。関数型プログラミングスタイルは、可変な状態の量を最小化することを好み、
コードを明瞭化します。可変な状態を除去すると、検索を同時並行に行うという将来的な改善をするのが、
可能になる可能性があります。なぜなら、`results`ベクタへの同時アクセスを管理する必要がなくなるからです。
リスト13-22は、この変更を示しています:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 13-22: Using iterator adaptor methods in the
implementation of the `search` function</span>
-->

<span class="caption">リスト13-22: `search`関数の実装でイテレータアダプタのメソッドを使用する</span>

<!--
Recall that the purpose of the `search` function is to return all lines in
`contents` that contain the `query`. Similar to the `filter` example in Listing
13-16, this code uses the `filter` adaptor to keep only the lines that
`line.contains(query)` returns `true` for. We then collect the matching lines
into another vector with `collect`. Much simpler! Feel free to make the same
change to use iterator methods in the `search_case_insensitive` function as
well.
-->

`search`関数の目的は、`query`を含む`contents`の行全てを返すことであることを思い出してください。
リスト13-16の`filter`例に酷似して、このコードは`filter`アダプタを使用して`line.contains(query)`が`true`を返す行だけを残すことができます。
それから、合致した行を別のベクタに`collect`で集結させます。ずっと単純です！ご自由に、
同じ変更を行い、`search_case_insensitive`関数でもイテレータメソッドを使うようにしてください。

<!--
### Choosing Between Loops or Iterators
-->

### ループかイテレータかの選択

<!--
The next logical question is which style you should choose in your own code and
why: the original implementation in Listing 13-21 or the version using
iterators in Listing 13-22. Most Rust programmers prefer to use the iterator
style. It’s a bit tougher to get the hang of at first, but once you get a feel
for the various iterator adaptors and what they do, iterators can be easier to
understand. Instead of fiddling with the various bits of looping and building
new vectors, the code focuses on the high-level objective of the loop. This
abstracts away some of the commonplace code so it’s easier to see the concepts
that are unique to this code, such as the filtering condition each element in
the iterator must pass.
-->

次の論理的な疑問は、自身のコードでどちらのスタイルを選ぶかと理由です: リスト13-21の元の実装とリスト13-22のイテレータを使用するバージョンです。
多くのRustプログラマは、イテレータスタイルを好みます。とっかかりが少し困難ですが、
いろんなイテレータアダプタとそれがすることの感覚を一度掴めれば、イテレータの方が理解しやすいこともあります。
いろんなループを少しずつもてあそんだり、新しいベクタを構築する代わりに、コードは、ループの高難度の目的に集中できるのです。
これは、ありふれたコードの一部を抽象化するので、イテレータの各要素が通過しなければならないふるい条件など、
このコードに独特の概念を理解しやすくなります。

<!--
But are the two implementations truly equivalent? The intuitive assumption
might be that the more low-level loop will be faster. Let’s talk about
performance.
-->

ですが、本当に2つの実装は等価なのでしょうか？直観的な仮説は、より低レベルのループの方がより高速ということかもしれません。
パフォーマンスに触れましょう。

<!--
[impl-trait]: ch10-02-traits.html#traits-as-parameters
-->

[impl-trait]: ch10-02-traits.html#引数としてのトレイト
