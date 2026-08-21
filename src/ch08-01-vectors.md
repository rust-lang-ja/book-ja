<!--
## Storing Lists of Values with Vectors
-->

## ベクタで値のリストを保持する

<!--
The first collection type we’ll look at is `Vec<T>`, also known as a *vector*.
Vectors allow you to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.
-->

最初に見るコレクション型は`Vec<T>`です。*ベクタ*とも呼ばれます。
ベクタを使用することで、複数の値をメモリ上で互いに隣り合うように単一のデータ構造に保持することができます。
ベクタは同じ型の値のみ保持することができます。
テキストファイルの各行や、ショッピングカートのアイテムの価格などのような、要素のリストがある場合にベクタは有用です。

<!--
### Creating a New Vector
-->

### 新しいベクタを生成する

<!--
To create a new empty vector, we call the `Vec::new` function, as shown in
Listing 8-1.
-->

空のベクタを新たに作成するには、リスト8-1に示すように`Vec::new`関数を呼びます。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type `i32`</span>
-->

<span class="caption">リスト8-1：`i32`型の値を保持する新しい空のベクタを作成する</span>

<!--
Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type.
When we create a vector to hold a specific type, we can specify the type within
angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.
-->

ここで型注釈を付けていることに注目してください。
このベクタには何も値を挿入していないので、コンパイラには私たちがどんなデータを保持させるつもりか分かりません。
これは重要な点です。
ベクタはジェネリクスを使用して実装されています; ジェネリクスを使用する自身の型をどう使用するかついては第10章で解説します。
今のところは、標準ライブラリで提供される`Vec<T>`型は、どんな型でも保持できると理解しておいてください。
特定の型を保持するベクタを作成するときには、その型を山かっこ内に指定すればよいです。
リスト8-1では、コンパイラに`v`の`Vec<T>`は`i32`型の要素を保持すると指示していたのです。

<!--
More often, you’ll create a `Vec<T>` with initial values and Rust will infer
the type of value you want to store, so you rarely need to do this type
annotation. Rust conveniently provides the `vec!` macro, which will create a
new vector that holds the values you give it. Listing 8-2 creates a new
`Vec<i32>` that holds the values `1`, `2`, and `3`. The integer type is `i32`
because that’s the default integer type, as we discussed in the [“Data
Types”][data-types] section of Chapter 3.
-->

`Vec<T>`を作成する場合は初期値を持たせることのほうが多いでしょう。
その場合コンパイラは保持したい値の型を推論するでしょうから、このように型注釈を付ける必要はほとんどありません。
Rustには`vec!`という便利なマクロが用意されていて、これは与えた値を保持する新しいベクタを作成します。
リスト8-2では、`1`、`2`、`3`という値を持つ新しい`Vec<i32>`を作成しています。
整数の型が`i32`になっているのは、3章の[「データ型」][data-types]節で学んだように、これがデフォルトの整数型だからです。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-2: Creating a new vector containing
values</span>
-->

<span class="caption">リスト8-2: 値を含む新しいベクタを作成する</span>

<!--
Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.
-->

初期値の`i32`値を与えたので、コンパイラは`v`の型が`Vec<i32>`であると推論でき、型注釈は不要になりました。
次はベクタを変更する方法を見ましょう。

<!--
### Updating a Vector
-->

### ベクタを更新する

<!--
To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.
-->

ベクタを作成し、それから要素を追加するには、リスト8-3に示すように`push`メソッドを使います。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-3: Using the `push` method to add values to a
vector</span>
-->

<span class="caption">リスト8-3：`push`メソッドを使用してベクタに値を追加する</span>

<!--
As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.
-->

第3章で説明したとおり、どんな変数でも、その値を変更したかったら`mut`キーワードで可変にする必要があります。
中に配置する数値は全て`i32`型であり、Rustはこのことをデータから推論するので、`Vec<i32>`という注釈は不要です。

<!--
### Reading Elements of Vectors
-->

### ベクタの要素を読む

<!--
There are two ways to reference a value stored in a vector: via indexing or
using the `get` method. In the following examples, we’ve annotated the types of
the values that are returned from these functions for extra clarity.
-->

ベクタに保持された値を参照する方法は2つあります: 添え字を使用する方法と、`get`メソッドを使用する方法です。
これから示す例では、理解を助けるために、それらの関数からの戻り値型を注釈しています。

<!--
Listing 8-4 shows both methods of accessing a value in a vector, with indexing
syntax and the `get` method.
-->

リスト8-4はベクタの値にアクセスする両方の方法として、添え字記法と`get`メソッドが示されています。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-4: Using indexing syntax or the `get` method to
access an item in a vector</span>
-->

<span class="caption">リスト8-4：添え字記法か`get`メソッドを使用してベクタの要素にアクセスする</span>

<!--
Note a few details here. We use the index value of `2` to get the third element
because vectors are indexed by number, starting at zero. Using `&` and `[]`
gives us a reference to the element at the index value. When we use the `get`
method with the index passed as an argument, we get an `Option<&T>` that we can
use with `match`.
-->

ここでいくつか注意があります。
ベクタは0から始まる番号で添え字アクセスされるので、3番目の要素を得るのに`2`という添え字の値を使用しています。
`&`と`[]`を使用することで、その添え字の値にある要素への参照が得られます。
`get`メソッドに引数として添え字を渡して使用すると、`Option<&T>`が得られ、これは`match`で使用することができます。

<!--
The reason Rust provides these two ways to reference an element is so you can
choose how the program behaves when you try to use an index value outside the
range of existing elements. As an example, let’s see what happens when we have
a vector of five elements and then we try to access an element at index 100
with each technique, as shown in Listing 8-5.
-->

Rustが要素を参照するためにこれらの2通りの方法を提供している理由は、存在する要素の範囲外の添え字の値を使おうとしたときのプログラムの振る舞いを選択できるようにするためです。
例として、5要素のベクタがあるとして、それぞれの手法で添え字100の要素にアクセスを試みた場合、何が起こるのか確認しましょう。
リスト8-5に示します。

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-5: Attempting to access the element at index
100 in a vector containing five elements</span>
-->

<span class="caption">リスト8-5：5つの要素を含むベクタの添え字100の要素にアクセスしようとする</span>

<!--
When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there’s an attempt to access an element past the
end of the vector.
-->

このコードを走らせると、最初の`[]`メソッドはプログラムをパニックさせます。
なぜなら存在しない要素を参照しているからです。
このメソッドは、ベクタの終端を超えて要素にアクセスしようとしたときにプログラムをクラッシュさせたい場合に最適です。

<!--
When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector may happen occasionally under normal
circumstances. Your code will then have logic to handle having either
`Some(&element)` or `None`, as discussed in Chapter 6. For example, the index
could be coming from a person entering a number. If they accidentally enter a
number that’s too large and the program gets a `None` value, you could tell the
user how many items are in the current vector and give them another chance to
enter a valid value. That would be more user-friendly than crashing the program
due to a typo!
-->

`get`メソッドにベクタ外の添え字を渡すと、パニックすることなく`None`を返します。
普通の状況でもベクタの範囲外にアクセスする可能性があるなら、このメソッドを使用することになるでしょう。
その場合、第6章で説明したように、コードは`Some(&element)`か`None`を扱うロジックを持つことになります。
例えば、誰かが入力した数値が添え字になるかもしれません。
もし誤って大きすぎる値を入力し、プログラムが`None`値を得たなら、いまベクタに何要素あるかをユーザに教え、正しい値を再入力してもらうこともできます。
その方が、ただのタイプミスでプログラムをクラッシュさせるより、ユーザに優しいといえそうです。

<!--
When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states you can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-6, where we hold an immutable reference
to the first element in a vector and try to add an element to the end. This
program won’t work if we also try to refer to that element later in the
function:
-->

プログラムに有効な参照がある場合、借用チェッカー (borrow checker) は、（第4章で解説しましたが）所有権と借用規則を強制し、ベクタの中身へのこの参照や他のいかなる参照も有効であり続けることを保証してくれます。
同一スコープ上では、可変と不変な参照を同時には存在させられないというルールを思い出してください。
このルールはリスト8-6でも適用されています。
リスト8-6ではベクタの最初の要素への不変参照を保持しつつ、終端に要素を追加しようとしています。
このプログラムは、関数内のここ以降で、この要素（訳注：`first`のこと）を参照しようとすると失敗します。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-6: Attempting to add an element to a vector
while holding a reference to an item</span>
-->

<span class="caption">リスト8-6：要素への参照を保持しつつ、ベクタに要素を追加しようとする</span>

<!--
Compiling this code will result in this error:
-->

このコードをコンパイルすると、こんなエラーになります。

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

<!--
The code in Listing 8-6 might look like it should work: why should a reference
to the first element care about changes at the end of the vector? This error is
due to the way vectors work: because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require
allocating new memory and copying the old elements to the new space, if there
isn’t enough room to put all the elements next to each other where the vector
is currently stored. In that case, the reference to the first element would be
pointing to deallocated memory. The borrowing rules prevent programs from
ending up in that situation.
-->

リスト8-6のコードは、一見動きそうに見えるかもしれません。
なぜ最初の要素への参照が、ベクタの終端への変更を気にかける必要があるのでしょうか？
このエラーはベクタが動作するしくみによるものです: 
ベクタはメモリ上に値同士を隣り合うように配置するため、新たな要素をベクタの終端に追加するとき、いまベクタが置かれている場所に全要素を隣り合わせに配置するだけのスペースがないなら、新しいメモリを割り当て、古い要素を新しいスペースにコピーする必要があります。
その場合、最初の要素を指す参照は、解放されたメモリを指すことになるでしょう。
借用規則がそのような状況に陥らないよう防いでくれるのです。

<!--
> Note: For more on the implementation details of the `Vec<T>` type, see [“The
> Rustonomicon”][nomicon].
-->

> 注釈: `Vec<T>`の実装に関する詳細については、[“The Rustonomicon”][nomicon]を参照してください （訳注：日本語版は[こちら][nomicon-ja-vec]です）。

[nomicon-ja-vec]: https://doc.rust-jp.rs/rust-nomicon-ja/vec.html

<!--
### Iterating over the Values in a Vector
-->

### ベクタ内の値を順に処理する

<!--
To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time. Listing 8-7 shows how
to use a `for` loop to get immutable references to each element in a vector of
`i32` values and print them.
-->

ベクタの要素に順番にアクセスするには、添え字で1要素ごとにアクセスするのではなく、全要素を走査することになるでしょう。
リスト8-7で`for`ループを使い、`i32`のベクタの各要素に対する不変な参照を得て、それらを表示する方法を示します。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-7: Printing each element in a vector by
iterating over the elements using a `for` loop</span>
-->

<span class="caption">リスト8-7：`for`ループで要素を走査し、ベクタの各要素を表示する</span>

<!--
We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-8
will add `50` to each element.
-->

また、全要素に変更を加えるために、可変なベクタの各要素への可変な参照を走査することもできます。
リスト8-9の`for`ループでは各要素に`50`を足しています。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-8: Iterating over mutable references to
elements in a vector</span>
-->

<span class="caption">リスト8-8：ベクタの要素への可変な参照を走査する</span>

<!--
To change the value that the mutable reference refers to, we have to use the
`*` dereference operator to get to the value in `i` before we can use the `+=`
operator. We’ll talk more about the dereference operator in the [“Following the
Pointer to the Value with the Dereference Operator”][deref]
section of Chapter 15.
-->

可変参照が参照している値を変更するには、`+=`演算子を使用する前に、`*`参照外し演算子を使用して`i`の値にたどり着かないといけません。
参照外し演算子については、第15章の[「参照外し演算子で値までポインタを追いかける」][deref]節でより詳しく扱います。

<!--
Iterating over a vector, whether immutably or mutably, is safe because of the
borrow checker's rules. If we attempted to insert or remove items in the `for`
loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error
similar to the one we got with the code in Listing 8-6. The reference to the
vector that the `for` loop holds prevents simultaneous modification of the
whole vector.
-->

借用チェッカーの規則のおかげで、ベクタを走査するのは不変であっても可変であっても安全です。
もしリスト8-7やリスト8-8の`for`ループ本体の中で要素を挿入または削除しようとすると、リスト8-6のコードで発生したのと同じようなコンパイルエラーになるでしょう。
`for`ループがベクタへの参照をつかんでいるために、それと同時にベクタ全体の変更が起こらないようになっています。

<!--
### Using an Enum to Store Multiple Types
-->

### Enumを使って複数の型を保持する

<!--
Vectors can only store values that are the same type. This can be inconvenient;
there are definitely use cases for needing to store a list of items of
different types. Fortunately, the variants of an enum are defined under the
same enum type, so when we need one type to represent elements of different
types, we can define and use an enum!
-->

ベクタは同じ型の値しか保持できません。これは不便なこともあります。
異なる型の要素を保持する必要のあるユースケースは必ず存在します。
幸運なことに、enumの列挙子は同じenumの型の中に定義されるので、異なる型の要素を表現する単一の型が必要になったら、enumを定義して使用すればよいのです！

<!--
For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and all the enum variants will be considered the same type: that
of the enum. Then we can create a vector to hold that enum and so, ultimately,
holds different types. We’ve demonstrated this in Listing 8-9.
-->

例えば、スプレッドシートのある行から値を得ることを考えます。
ここで、その行の中の列には、整数を含むもの、浮動小数点数を含むもの、文字列を含むものがあるとします。
列挙子ごとに異なる値の型を保持するenumが定義できます。
そして、このenumの列挙子は全て同じ型、つまりenumの型、と考えられるわけです。
ですから、そのenumを保持するためのベクタを作成でき、結果的に異なる型を保持できるようになるわけです。
リスト8-9でこれを実演しています。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-9: Defining an `enum` to store values of
different types in one vector</span>
-->

<span class="caption">リスト8-9：`enum`を定義して、一つのベクタに異なる型の値を保持する</span>

<!--
Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. We
must also be explicit about what types are allowed in this vector. If Rust
allowed a vector to hold any type, there would be a chance that one or more of
the types would cause errors with the operations performed on the elements of
the vector. Using an enum plus a `match` expression means that Rust will ensure
at compile time that every possible case is handled, as discussed in Chapter 6.
-->

個々の要素を格納するのにヒープ上で必要となるメモリの量を正確に把握するめに、Rustコンパイラはコンパイル時にベクタに入る型を知る必要があります。
また、このベクタではどんな型が許容されるのか明示しなくてはなりません。
もしRustが、ベクタにどんな型でも保持できることを許していたら、ベクタの要素に対して行われる処理に対して、いくつかの型がエラーを引き起こすかもしれません。
enumに加えて`match`式を使うことで、第6章で説明したとおり、あらゆるケースが処理できることを、Rustがコンパイル時に保証することになります。

<!--
If you don’t know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won’t work. Instead, you can use a trait
object, which we’ll cover in Chapter 17.
-->

プログラムが実行時に取得し、ベクタに格納し得る全ての型を網羅できない場合には、このenumを使ったテクニックはうまくいかないでしょう。
代わりにトレイトオブジェクトを使用できます。
こちらは第17章で取り上げます。

<!--
Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api] for all the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element.
-->

これまでにベクタの代表的な使い方をいくつか紹介しました。
標準ライブラリで`Vec<T>`に定義されている多くの有益なメソッドについて、[APIドキュメント][vec-api]を必ず確認するようにしてください。
例えば、`push`に加えて、`pop`というメソッドがあり、これは最後の要素を削除して返します。

<!--
### Dropping a Vector Drops Its Elements
-->

### ベクタをドロップすれば、要素もドロップする

<!--
Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-10.
-->

他のあらゆる`struct`（構造体）と同様に、ベクタもスコープを抜ければ解放されます。
その様子をリスト8-10に示します。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-10: Showing where the vector and its elements
are dropped</span>
-->

<span class="caption">リスト8-10：ベクタとその要素がドロップされる箇所を示す</span>

<!--
When the vector gets dropped, all of its contents are also dropped, meaning the
integers it holds will be cleaned up. The borrow checker ensures that any
references to contents of a vector are only used while the vector itself is
valid.
-->

ベクタがドロップされると、その中身もドロップされます。
つまり、保持されていた整数値が片付けられるということです。
借用チェッカーは、ベクタの内容へのいかなる参照も、ベクタ自体が有効な間のみ使用されることを保証します。

<!--
Let’s move on to the next collection type: `String`!
-->

それでは次のコレクション型である`String`に移りましょう！

<!--
[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
-->

[data-types]: ch03-02-data-types.html#データ型
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#値までポインタを追いかける
