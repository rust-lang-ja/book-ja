<!--
## Using `Box<T>` to Point to Data on the Heap
-->

## ヒープのデータを指す`Box<T>`を使用する

<!--
The most straightforward smart pointer is a *box*, whose type is written
`Box<T>`. Boxes allow you to store data on the heap rather than the stack. What
remains on the stack is the pointer to the heap data. Refer to Chapter 4 to
review the difference between the stack and the heap.
-->

最も素直なスマートポインタは*ボックス*であり、その型は`Box<T>`と記述されます。
ボックスにより、スタックではなくヒープにデータを格納することができます。スタックに残るのは、
ヒープデータへのポインタです。スタックとヒープの違いを再確認するには、第4章を参照されたし。

<!--
Boxes don’t have performance overhead, other than storing their data on the
heap instead of on the stack. But they don’t have many extra capabilities
either. You’ll use them most often in these situations:
-->

ボックスは、データをスタックの代わりにヒープに格納する以外は、パフォーマンスのオーバーヘッドはありません。
しかし、特別な能力がたくさんあるわけでもありません。以下のような場面で最もよく使われるでしょう。

<!--
* When you have a type whose size can’t be known at compile time and you want
  to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but
  ensure the data won’t be copied when you do so
* When you want to own a value and you care only that it’s a type that
  implements a particular trait rather than being of a specific type
-->

* コンパイル時にはサイズを知ることができない型があり、正確なサイズを要求する文脈でその型の値を使用する時
* 多くのデータがあり、その所有権を移したいが、その際にデータがコピーされないようにしたい時
* 値を所有する必要があり、特定の型であることではなく、特定のトレイトを実装する型であることのみ気にかけている時

<!--
最後の行がよくわからない。what ... (that) you'll apply ... なのか？
辞書には文の先頭に用いて強調する用法があると書かれているので、これのことと思われる
-->

<!--
We’ll demonstrate the first situation in the [“Enabling Recursive Types with
Boxes”](#enabling-recursive-types-with-boxes) section. In the
second case, transferring ownership of a large amount of data can take a long
time because the data is copied around on the stack. To improve performance in
this situation, we can store the large amount of data on the heap in a box.
Then, only the small amount of pointer data is copied around on the stack,
while the data it references stays in one place on the heap. The third case is
known as a *trait object*, and Chapter 17 devotes an entire section, [“Using
Trait Objects That Allow for Values of Different Types,”][trait-objects]
just to that topic. So what you learn here you’ll apply again in
Chapter 17!
-->

[「ボックスで再帰的な型を可能にする」](#ボックスで再帰的な型を可能にする)節で1つ目の場合について実際に説明します。
2番目の場合、多くのデータの所有権を転送するには、データがスタック上でコピーされるので、長い時間がかかり得ます。
この場面でパフォーマンスを向上させるために、多くのデータをヒープ上にボックスとして格納することができます。
そして、小さなポインタのデータのみがスタック上でコピーされる一方、それが参照しているデータはヒープ上の1箇所に留まります。
3番目のケースは*トレイトオブジェクト*として知られています。第17章の[「トレイトオブジェクトで異なる型の値を許容する」][trait-objects]の節は、
すべてその話題に捧げられています。
従って、ここで学ぶことは第17章でもまた使うことになります！

<!--
### Using a `Box<T>` to Store Data on the Heap
-->

### `Box<T>`を使ってヒープにデータを格納する

<!--
Before we discuss the heap storage use case for `Box<T>`, we’ll cover the
syntax and how to interact with values stored within a `Box<T>`.
-->

`Box<T>`のヒープストレージのユースケースを議論する前に、`Box<T>`の記法と、`Box<T>`内に格納された値を読み書きする方法について講義しましょう。

<!--
Listing 15-1 shows how to use a box to store an `i32` value on the heap:
-->

リスト15-1は、ボックスを使用してヒープに`i32`の値を格納する方法を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

<!--
<span class="caption">Listing 15-1: Storing an `i32` value on the heap using a
box</span>
-->

<span class="caption">リスト15-1: ボックスを使用して`i32`の値をヒープに格納する</span>

<!--
We define the variable `b` to have the value of a `Box` that points to the
value `5`, which is allocated on the heap. This program will print `b = 5`; in
this case, we can access the data in the box similar to how we would if this
data were on the stack. Just like any owned value, when a box goes out of
scope, as `b` does at the end of `main`, it will be deallocated. The
deallocation happens both for the box (stored on the stack) and the data it
points to (stored on the heap).
-->

変数`b`を定義して`Box`の値を保持します。`Box`は値`5`を指し、値`5`はヒープに確保されています。このプログラムは、`b = 5`と出力するでしょう。つまりこの場合、このデータがスタックにあるのと同じような方法でボックスのデータにアクセスできます。
所有された値と全く同じでスコープを抜けるとき、実際`b`は`main`の終わりで抜けるのですが、
ボックスはメモリから解放されます。メモリの解放は（スタックに格納されている）ボックスと（ヒープに格納されている）指しているデータの両方に対して起きます。

<!--
Putting a single value on the heap isn’t very useful, so you won’t use boxes by
themselves in this way very often. Having values like a single `i32` on the
stack, where they’re stored by default, is more appropriate in the majority of
situations. Let’s look at a case where boxes allow us to define types that we
wouldn’t be allowed to if we didn’t have boxes.
-->

ヒープに単独の値を置いても嬉しいことはほとんどないので、このように単独でボックスを使用することはあまりありません。
単独の`i32`のような値はデフォルトではスタックに置かれます。ほとんどの場合ではその方が適切です。
ボックスのおかげで定義できるようになる型を見てみましょう。ボックスがなければそれらの型は定義できません。

<!--
### Enabling Recursive Types with Boxes
-->

### ボックスで再帰的な型を可能にする

<!--
A value of *recursive type* can have another value of the same type as part of
itself. Recursive types pose an issue because at compile time Rust needs to
know how much space a type takes up. However, the nesting of values of
recursive types could theoretically continue infinitely, so Rust can’t know how
much space the value needs. Because boxes have a known size, we can enable
recursive types by inserting a box in the recursive type definition.
-->

*再帰的な型*の値は、値の一部として同じ型の他の値を持つことができます。再帰的な型はある問題を提起します。
コンパイラは、コンパイル時にある型が占有する領域の大きさを知っておかなくてはならないからです。
しかし、再帰的な型の値のネストは、理論的には無限に続く可能性があるので、コンパイラはその値が必要とする領域の大きさを知ることができないのです。
ボックスのサイズはわかっているので、再帰的な型の定義にボックスを挟むことで、再帰的な型を作ることができます。

<!--
As an example of a recursive type, let’s explore the *cons list*. This is a data
type commonly found in functional programming languages. The cons list type
we’ll define is straightforward except for the recursion; therefore, the
concepts in the example we’ll work with will be useful any time you get into
more complex situations involving recursive types.
-->

再帰的な型の例として、*コンスリスト*を探究しましょう。これは関数型プログラミング言語でよく見られるデータ型です。
我々が定義するコンスリストは、再帰を除けば素直です。故に、これから取り掛かる例に現れる概念は、
再帰的な型が関わるもっと複雑な場面に遭遇したときには必ず役に立つでしょう。

<!--
#### More Information About the Cons List
-->

#### コンスリストについてもっと詳しく

<!--
A *cons list* is a data structure that comes from the Lisp programming language
and its dialects and is made up of nested pairs, and is the Lisp version of a
linked list. Its name comes from the `cons` function (short for “construct
function”) in Lisp that constructs a new pair from its two arguments. By
calling `cons` on a pair consisting of a value and another pair, we can
construct cons lists made up of recursive pairs.
-->

コンスリストは、Lispプログラミング言語とその方言に由来するデータ構造で、ネストされたペアによって構成される、
Lisp版の連結リストです。その名前は、2つの引数から新しいペアを構築するLispの`cons`関数（"construct function"の省略形です）に由来します。
`cons`を、値と別のペアからなるペアに対して呼び出すことで、再帰的なペアからなるコンスリストを構築することができます。

<!--
For example, here’s a pseudocode representation of a cons list containing the
list 1, 2, 3 with each pair in parentheses:
-->

例えば、リスト 1, 2, 3 を含むコンスリストを、丸かっこを各ペアする擬似コードで表現すると、次のようになります:

```text
(1, (2, (3, Nil)))
```

<!--
Each item in a cons list contains two elements: the value of the current item
and the next item. The last item in the list contains only a value called `Nil`
without a next item. A cons list is produced by recursively calling the `cons`
function. The canonical name to denote the base case of the recursion is `Nil`.
Note that this is not the same as the “null” or “nil” concept in Chapter 6,
which is an invalid or absent value.
-->

コンスリストの各要素は、2つの要素を含みます。現在の要素の値と次の要素です。リストの最後の要素は、
`Nil`と呼ばれる値だけを含み、次の要素を持ちません。コンスリストは、繰り返し`cons`関数を呼び出すことで生成されます。
繰り返しの基底ケースを示すのに標準的に使われる名前は`Nil`です。これは第6章の"null"や"nil"の概念とは異なることに注意してください。
"null"や"nil"は、無効だったり存在しない値です。

<!--
The cons list isn’t a commonly used data structure in Rust. Most of the time
when you have a list of items in Rust, `Vec<T>` is a better choice to use.
Other, more complex recursive data types *are* useful in various situations,
but by starting with the cons list in this chapter, we can explore how boxes
let us define a recursive data type without much distraction.
-->

コンスリストはRustではあまり使用されないデータ構造です。
Rustで要素のリストがあるときはほとんど、`Vec<T>`を使用するのがよりよい選択になります。
より複雑な他の再帰的なデータ型は様々な場面で役に立ち*ます*。しかしこの章ではコンスリストから始めることで、
ボックスのおかげで再帰的なデータ型を定義できるわけを、あまり気を散らすことなく調べることができるのです。

<!--
Listing 15-2 contains an enum definition for a cons list. Note that this code
won’t compile yet because the `List` type doesn’t have a known size, which
we’ll demonstrate.
-->

リスト15-2には、コンスリストのenum定義が含まれています。このコードはまだコンパイルできないことに注意してください。
`List`型のサイズが分からないからです。
これについてはこの後説明します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-2: The first attempt at defining an enum to
represent a cons list data structure of `i32` values</span>
-->

<span class="caption">リスト15-2: `i32`値のコンスリストデータ構造を表すenumを定義する最初の試行</span>

<!--
> Note: We’re implementing a cons list that holds only `i32` values for the
> purposes of this example. We could have implemented it using generics, as we
> discussed in Chapter 10, to define a cons list type that could store values of
> any type.
-->

> 注釈: この例のために`i32`値だけを保持するコンスリストを実装します。第10章で議論したように、
> ジェネリクスを使用してどんな型の値も格納できるコンスリストを定義して実装することもできたでしょう。

<!--
Using the `List` type to store the list `1, 2, 3` would look like the code in
Listing 15-3:
-->

この`List`型を使用してリスト`1, 2, 3`を格納すると、リスト15-3のコードのような見た目になるでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-3: Using the `List` enum to store the list `1,
2, 3`</span>
-->

<span class="caption">リスト15-3: `List` enumを使用してリスト`1, 2, 3`を格納する</span>

<!--
The first `Cons` value holds `1` and another `List` value. This `List` value is
another `Cons` value that holds `2` and another `List` value. This `List` value
is one more `Cons` value that holds `3` and a `List` value, which is finally
`Nil`, the non-recursive variant that signals the end of the list.
-->

最初の`Cons`値は、`1`と別の`List`値を保持しています。この`List`値は別の`Cons`値で、
`2`とまた別の`List`値を保持しています。この`List`値はまたまた別の`Cons`値で、
`3`と`List`値を保持していますが、この`List`値でついに`Nil`になります。`Nil`はリストの終端を通知する非再帰的な列挙子です。

<!--
If we try to compile the code in Listing 15-3, we get the error shown in
Listing 15-4:
-->

リスト15-3のコードをコンパイルしようとすると、リスト15-4に示したエラーが出ます。

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

<!--
<span class="caption">Listing 15-4: The error we get when attempting to define
a recursive enum</span>
-->

<span class="caption">リスト15-4: 再帰的なenumを定義しようとすると得られるエラー</span>

<!--
The error shows this type “has infinite size.” The reason is that we’ve defined
`List` with a variant that is recursive: it holds another value of itself
directly. As a result, Rust can’t figure out how much space it needs to store a
`List` value. Let’s break down why we get this error. First, we’ll look at how
Rust decides how much space it needs to store a value of a non-recursive type.
-->

エラーは、この型は「無限のサイズである」と表示しています。理由は、再帰的な列挙子を含む`List`を定義したからです。
つまり、`List`は自身の別の値を直接保持しているのです。結果として、コンパイラは`List`値を格納するのに必要な領域が計算できません。
このエラーが出た理由を噛み砕きましょう。まず、非再帰的な型の値を格納するのに必要な領域をどうコンパイラが決定しているかを見ましょう。

<!--
#### Computing the Size of a Non-Recursive Type
-->

#### 非再帰的な型のサイズを計算する

<!--
Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:
-->

第6章でenum定義を議論した時にリスト6-2で定義した`Message` enumを思い出してください。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<!--
To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see which variant needs the most space. Rust
sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Because only one variant will be
used, the most space a `Message` value will need is the space it would take to
store the largest of its variants.
-->

`Message`値一つにメモリを確保するために必要な領域を決定するために、コンパイラは、
各列挙子を見てどの列挙子が最も領域を必要とするかを確認します。コンパイラは、
`Message::Quit`は全く領域を必要とせず、`Message::Move`は`i32`値を2つ格納するのに十分な領域が必要、などと確かめます。
ただ1つの列挙子しか使用されないので、`Message`値一つが必要とする最大の領域は、
最大の列挙子を格納するのに必要になる領域です。

<!--
Contrast this with what happens when Rust tries to determine how much space a
recursive type like the `List` enum in Listing 15-2 needs. The compiler starts
by looking at the `Cons` variant, which holds a value of type `i32` and a value
of type `List`. Therefore, `Cons` needs an amount of space equal to the size of
an `i32` plus the size of a `List`. To figure out how much memory the `List`
type needs, the compiler looks at the variants, starting with the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, and this process continues infinitely, as shown in Figure 15-1.
-->

これをコンパイラがリスト15-2の`List` enumのような再帰的な型が必要とする領域を決定しようとする時に起こることと比較してください。
コンパイラは`Cons`列挙子を見ることから始めます。この列挙子には、型`i32`値が一つと型`List`の値が一つ保持されます。
故に、`Cons`は1つの`i32`と`List`のサイズに等しい領域を必要とします。`List`が必要とするメモリ量を計算するのに、
コンパイラは`Cons`列挙子から列挙子を観察します。`Cons`列挙子は型`i32`を1つと型`List`の値1つを保持し、
この過程は無限に続きます。図15-1のようにですね。

<!--
<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />
-->

<img alt="無限のコンスリスト" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 15-1: An infinite `List` consisting of infinite
`Cons` variants</span>
-->

<span class="caption">図15-1: 無限の`Cons`列挙子からなる無限の`List`</span>

<!--
#### Using `Box<T>` to Get a Recursive Type with a Known Size
-->

#### `Box<T>`で既知のサイズの再帰的な型を得る

<!--
Because Rust can’t figure out how much space to allocate for recursively
defined types, the compiler gives an error with this helpful suggestion:
-->

コンパイラは、再帰的に定義された型に必要なメモリ量を計算できないので、以下の役に立つ提案を含むエラーを返します:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
(ヘルプ: 何らかの間接参照(例: `Box`、`Rc`、あるいは`&`)を挿入して、循環を断ち切ってください)
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

<!--
In this suggestion, “indirection” means that instead of storing a value
directly, we should change the data structure to store the value indirectly by
storing a pointer to the value instead.
-->

この提案において「間接参照」は、値を直接格納するのではなく、データ構造を変更して値を間接的に格納すべきであるという意味です。
これは値の代わりに値へのポインタを格納することによって可能になります。

<!--
Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>`
needs: a pointer’s size doesn’t change based on the amount of data it’s
pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead
of another `List` value directly. The `Box<T>` will point to the next `List`
value that will be on the heap rather than inside the `Cons` variant.
Conceptually, we still have a list, created with lists holding other lists, but
this implementation is now more like placing the items next to one another
rather than inside one another.
-->

`Box<T>`はポインタなので、コンパイラには`Box<T>`が必要とする領域が必ずわかります。すなわち、ポインタのサイズは指しているデータの量に左右されません。つまり、別の`List`値を直接置く代わりに、
`Cons`列挙子の中に`Box<T>`を配置することができます。`Box<T>`は、
`Cons`列挙子の中ではなく、ヒープに置かれる次の`List`値を指します。概念的には、
依然として我々のリストは他のリストを保持するリストによって作られたものです。
しかし、今やこの実装は、要素をお互いの中に配置するというより、隣り合うように配置するような感じになります。

<!--
We can change the definition of the `List` enum in Listing 15-2 and the usage
of the `List` in Listing 15-3 to the code in Listing 15-5, which will compile:
-->

リスト15-2の`List` enumの定義とリスト15-3の`List`の使用をリスト15-5のコードに変更することができ、
これはコンパイルが通ります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

<!--
<span class="caption">Listing 15-5: Definition of `List` that uses `Box<T>` in
order to have a known size</span>
-->

<span class="caption">リスト15-5: 既知のサイズにするために`Box<T>`を使用する`List`の定義</span>

<!--
The `Cons` variant needs the size of an `i32` plus the space to store the
box’s pointer data. The `Nil` variant stores no values, so it needs less space
than the `Cons` variant. We now know that any `List` value will take up the
size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve
broken the infinite, recursive chain, so the compiler can figure out the size
it needs to store a `List` value. Figure 15-2 shows what the `Cons` variant
looks like now.
-->

`Cons`列挙子は、1つの`i32`のサイズに加えてボックスのポインタデータを格納する領域を必要とします。
`Nil`列挙子は値を格納しないので、`Cons`列挙子よりも必要な領域は小さいです。これで、
どんな`List`値も`i32`1つのサイズに加えてボックスのポインタデータのサイズを必要とすることがわかりました。
ボックスを使うことで無限に続く再帰の連鎖を断ち切ったので、コンパイラは`List`値を格納するのに必要なサイズを計算できます。
図15-2は、`Cons`列挙子の今の見た目を示しています。

<!--
<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />
-->

<img alt="有限のコンスリスト" src="img/trpl15-02.svg" class="center" />

<!--
<span class="caption">Figure 15-2: A `List` that is not infinitely sized
because `Cons` holds a `Box`</span>
-->

<span class="caption">図15-2: `Cons`が`Box`を保持しているので、無限にサイズがあるわけではない`List`</span>

<!--
Boxes provide only the indirection and heap allocation; they don’t have any
other special capabilities, like those we’ll see with the other smart pointer
types. They also don’t have the performance overhead that these special
capabilities incur, so they can be useful in cases like the cons list where the
indirection is the only feature we need. We’ll look at more use cases for boxes
in Chapter 17, too.
-->

ボックスは、間接参照とヒープメモリ確保だけを提供します。他のスマートポインタ型に見られるような別の特別な能力は何もありません。
これらの特別な能力が招くパフォーマンスのオーバーヘッドもないので、
コンスリストのように間接参照だけが必要な機能である場合には便利でしょう。
より多くのボックスのユースケースは第17章でもお見かけするでしょう。

<!--
The `Box<T>` type is a smart pointer because it implements the `Deref` trait,
which allows `Box<T>` values to be treated like references. When a `Box<T>`
value goes out of scope, the heap data that the box is pointing to is cleaned
up as well because of the `Drop` trait implementation. These two traits will be
even more important to the functionality provided by the other smart pointer
types we’ll discuss in the rest of this chapter. Let’s explore these two traits
in more detail.
-->

`Box<T>`型がスマートポインタなのは、`Deref`トレイトを実装しているからです。
このトレイトにより`Box<T>`の値を参照のように扱うことができます。
`Box<T>`値がスコープを抜けると、ボックスが参照しているヒープデータも片付けられます。これは`Drop`トレイト実装のおかげです。
これら2つのトレイトは、他のスマートポインタ型が提供する機能にとってさらに重要なものです。それらついてはこの章の残りで議論します。
これら2つのトレイトをより詳しく探究しましょう。

<!--
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
-->

[trait-objects]: ch17-02-trait-objects.html#トレイトオブジェクトで異なる型の値を許容する

