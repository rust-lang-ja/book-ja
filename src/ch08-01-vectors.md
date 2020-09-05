<!--
## Storing Lists of Values with Vectors
-->

## ベクタで一連の値を保持する

<!--
The first collection type we’ll look at is `Vec<T>`, also known as a *vector*.
Vectors allow us to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.
-->

最初に見るコレクションは、`Vec<T>`であり、*ベクタ*としても知られています。ベクタは、
メモリ上に値を隣り合わせに並べる単独のデータ構造に2つ以上の値を保持させてくれます。
ベクタには、同じ型の値しか保持できません。要素のリストがある場合に有用です。
例えば、テキストファイルの各行とか、ショッピングカートのアイテムの価格などです。

<!--
### Creating a New Vector
-->

### 新しいベクタを生成する

<!--
To create a new, empty vector, we can call the `Vec::new` function, as shown in
Listing 8-1.
-->

新しい空のベクタを作るには、リスト8-1に示されたように、`Vec::new`関数を呼べばよいです。

```rust
let v: Vec<i32> = Vec::new();
```

<!--
<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type `i32`</span>
-->

<span class="caption">リスト8-1: 新しい空のベクタを生成して`i32`型の値を保持する</span>

<!--
Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type,
and when a specific vector holds a specific type, the type is specified within
angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.
-->

ここでは、型注釈を付け足したことに注目してください。このベクタに対して、何も値を挿入していないので、
コンパイラには、どんなデータを保持させるつもりなのかわからないのです。これは重要な点です。ベクタは、
ジェネリクスを使用して実装されているのです; 独自の型でジェネリクスを使用する方法については、
第10章で解説します。今は、標準ライブラリにより提供されている`Vec<T>`型は、どんな型でも保持でき、
特定のベクタが特定の型を保持するとき、その型は山かっこ内に指定されることを知っておいてください。
リスト8-1では、コンパイラに`v`の`Vec<T>`は、`i32`型の要素を保持すると指示しました。

<!--
In more realistic code, Rust can often infer the type of value you want to
store once you insert values, so you rarely need to do this type annotation.
It's more common to create a `Vec<T>` that has initial values, and Rust
provides the `vec!` macro for convenience. The macro will create a new vector
that holds the values we give it. Listing 8-2 creates a new `Vec<i32>` that
holds the values `1`, `2`, and `3`.
-->

より現実的なコードでは、一旦値を挿入したら、コンパイラは保持させたい値の型をしばしば推論できるので、
この型注釈をすることは滅多にありません。初期値のある`Vec<T>`を生成する方が一般的ですし、
Rustには、利便性のために`vec!`というマクロも用意されています。このマクロは、
与えた値を保持する新しいベクタ型を生成します。リスト8-2では、`1`、`2`、`3`という値を持つ新しい`Vec<i32>`を生成しています。

```rust
let v = vec![1, 2, 3];
```

<!--
<span class="caption">Listing 8-2: Creating a new vector containing
values</span>
-->

<span class="caption">リスト8-2: 値を含む新しいベクタを生成する</span>

<!--
Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.
-->

初期値の`i32`値を与えたので、コンパイラは、`v`の型が`Vec<i32>`であると推論でき、型注釈は必要なくなりました。
次は、ベクタを変更する方法を見ましょう。

<!--
### Updating a Vector
-->

### ベクタを更新する

<!--
To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.
-->

ベクタを生成し、それから要素を追加するには、リスト8-3に示したように、`push`メソッドを使用できます。

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

<!--
<span class="caption">Listing 8-3: Using the `push` method to add values to a
vector</span>
-->

<span class="caption">リスト8-3: `push`メソッドを使用してベクタ型に値を追加する</span>

<!--
As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.
-->

あらゆる変数同様、第3章で議論したように、値を変化させたかったら、`mut`キーワードで可変にする必要があります。
中に配置する数値は全て`i32`型であり、コンパイラはこのことをデータから推論するので、
`Vec<i32>`という注釈は必要なくなります。

<!--
### Dropping a Vector Drops Its Elements
-->

### ベクタをドロップすれば、要素もドロップする

<!--
Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-4.
-->

他のあらゆる`構造体`同様、ベクタもスコープを抜ければ、解放されます。リスト8-4に注釈したようにですね。

<!--
```rust
{
let v = vec![1, 2, 3, 4];
-->

<!--
// do stuff with v
-->

<!--
} // <- v goes out of scope and is freed here
```
-->

```rust
{
    let v = vec![1, 2, 3, 4];

    // vで作業をする

} // <- vはここでスコープを抜け、解放される
```

<!--
<span class="caption">Listing 8-4: Showing where the vector and its elements
are dropped</span>
-->

<span class="caption">リスト8-4: ベクタとその要素がドロップされる箇所を示す</span>

<!--
When the vector gets dropped, all of its contents will also be dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point but can get a bit more complicated when you start to
introduce references to the elements of the vector. Let’s tackle that next!
-->

ベクタがドロップされると、その中身もドロップされます。つまり、保持されていた整数値が、
片付けられるということです。これは一見単純な点に見えるかもしれませんが、ベクタの要素への参照を導入した途端、
もうちょっと複雑になる可能性を秘めています。次は、それに挑んでいきましょう！

<!--
### Reading Elements of Vectors
-->

### ベクタの要素を読む

<!--
Now that you know how to create, update, and destroy vectors, knowing how to
read their contents is a good next step. There are two ways to reference a
value stored in a vector. In the examples, we’ve annotated the types of the
values that are returned from these functions for extra clarity.
-->

もうベクタを生成し、更新し、破壊する方法を知ったので、中身を読む方法を知るのはいいステップアップです。
ベクタに保持された値を参照する方法は2つあります。例では、さらなる明瞭性を求めて、
これらの関数から返る値の型を注釈しました。

<!--
Listing 8-5 shows both methods of accessing a value in a vector, either with
indexing syntax or the `get` method.
-->

リスト8-5に示したのは、両メソッドがベクタの値に対して、添字記法と`get`メソッドによりアクセスするところです。

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

<!--
<span class="caption">Listing 8-5: Using indexing syntax or the `get` method to
access an item in a vector</span>
-->

<span class="caption">リスト8-5: 添字記法か`get`メソッドを使用してベクタの要素にアクセスする</span>

<!--
Note two details here. First, we use the index value of `2` to get the third
element: vectors are indexed by number, starting at zero. Second, the two ways
to get the third element are by using `&` and `[]`, which gives us a reference,
or by using the `get` method with the index passed as an argument, which gives
us an `Option<&T>`.
-->

ここでは、2つのことに注目してください。まず、3番目の要素を得るのに`2`という添え字の値を使用していることです:
ベクタは、数値により順序付けされ、添え字は0から始まります。2番目に、3番目の要素を得る2つの方法は、
`&`と`[]`を使用して参照を得るものと、番号を引数として`get`メソッドに渡して、`Option<&T>`を得るものということです。

<!--
Rust has two ways to reference an element so you can choose how the program
behaves when you try to use an index value that the vector doesn’t have an
element for. As an example, let’s see what a program will do if it has a vector
that holds five elements and then tries to access an element at index 100, as
shown in Listing 8-6.
-->

Rustには要素を参照する方法が2通りあるので、ベクタに要素が含まれない番号の値を使用しようとした時に、
プログラムの振る舞いを選択できます。例として、ベクタに5つ要素があり、添え字100の要素にアクセスを試みた場合、
プログラムがすることを確認しましょう。リスト8-6に示したようにですね。

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

<!--
<span class="caption">Listing 8-6: Attempting to access the element at index
100 in a vector containing five elements</span>
-->

<span class="caption">リスト8-6: 5つの要素を含むベクタの添え字100の要素にアクセスしようとする</span>

<!--
When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there's an attempt to access an element past the
end of the vector.
-->

このコードを走らせると、最初の`[]`メソッドはプログラムをパニックさせます。存在しない要素を参照しているからです。
このメソッドは、ベクタの終端を超えて要素にアクセスしようとした時にプログラムをクラッシュさせたい場合に最適です。

<!--
When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector happens occasionally under normal circumstances.
Your code will then have logic to handle having either `Some(&element)` or
`None`, as discussed in Chapter 6. For example, the index could be coming from
a person entering a number. If they accidentally enter a number that’s too
large and the program gets a `None` value, you could tell the user how many
items are in the current vector and give them another chance to enter a valid
value. That would be more user-friendly than crashing the program due to a typo!
-->

`get`メソッドがベクタ外の添え字を渡されると、パニックすることなく`None`を返します。
普通の状態でも、ベクタの範囲外にアクセスする可能性がある場合に、このメソッドを使用することになるでしょう。
そうしたら、コードには`Some(&element)`か`None`を扱うロジックが存在することになります。そう、
第6章で議論したように。例えば、添え字は人間に数値を入力してもらうことで得ることもできます。
もし大きすぎる値を誤って入力し、プログラムが`None`値を得てしまったら、現在ベクタに幾つ要素があるかをユーザに教え、
再度正しい値を入力してもらうことができるでしょう。その方が、タイプミスでプログラムをクラッシュさせるより、
ユーザに優しくなるでしょう。

<!--
When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states we can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-7, where we hold an immutable reference to
the first element in a vector and try to add an element to the end, which won't
work.
-->

プログラムに有効な参照がある場合、借用チェッカー(borrow checker)は(第4章で解説しましたが)、
所有権と借用規則を強制し、ベクタの中身へのこの参照や他のいかなる参照も有効であり続けることを保証してくれます。
同一スコープ上では、可変と不変な参照を同時には存在させられないというルールを思い出してください。
このルールはリスト8-7にも適用され、リスト8-7ではベクタの最初の要素への不変参照を保持し、
終端に要素を追加しようとしていますが、動きません。

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

<!--
<span class="caption">Listing 8-7: Attempting to add an element to a vector
while holding a reference to an item</span>
-->

<span class="caption">リスト8-7: 要素への参照を保持しつつ、ベクタに要素を追加しようとする</span>

<!--
Compiling this code will result in this error:
-->

このコードをコンパイルすると、こんなエラーになります:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
(エラー: 不変としても借用されているので、`v`を可変で借用できません)
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
  |                  (不変借用はここで発生しています)
5 |
6 |     v.push(6);
  |     ^ mutable borrow occurs here
  |      (可変借用は、ここで発生しています)
7 | }
  | - immutable borrow ends here
  |   (不変借用はここで終了しています)
```

<!--
The code in Listing 8-7 might look like it should work: why should a reference
to the first element care about what changes at the end of the vector? This
error is due to the way vectors work: adding a new element onto the end of the
vector might require allocating new memory and copying the old elements to the
new space if there isn’t enough room to put all the elements next to each
other where the vector currently is. In that case, the reference to the first
element would be pointing to deallocated memory. The borrowing rules prevent
programs from ending up in that situation.
-->

リスト8-7のコードは、一見動くはずのように見えるかもしれません: なぜ、最初の要素への参照が、
ベクタの終端への変更を気にかける必要があるのでしょうか？このエラーは、ベクタの動作法のせいです:
新規要素をベクタの終端に追加すると、ベクタが現在存在する位置に隣り合って要素を入れるだけの領域がなかった場合に、
メモリの新規確保をして古い要素を新しいスペースにコピーする必要があるかもしれないからです。
その場合、最初の要素を指す参照は、解放されたメモリを指すことになるでしょう。借用規則により、
そのような場面に陥らないよう回避されるのです。

<!--
> Note: For more on the implementation details of the `Vec<T>` type, see “The
> Rustonomicon” at https://doc.rust-lang.org/stable/nomicon/vec.html.
-->

> 注釈: `Vec<T>`の実装に関する詳細については、[“The Rustonomicon”](https://doc.rust-lang.org/stable/nomicon/vec.html)を参照してください。

> 訳注: 日本語版のThe Rustonomiconは[こちら][nomicon-ja-vec]です。

[nomicon-ja-vec]: https://doc.rust-jp.rs/rust-nomicon-ja/vec.html

<!--
### Iterating Over the Values in a Vector
-->

### ベクタの値を走査する

<!--
If we want to access each element in a vector in turn, we can iterate through
all of the elements rather than use indexes to access one at a time. Listing
8-8 shows how to use a `for` loop to get immutable references to each element
in a vector of `i32` values and print them.
-->

ベクタの要素に順番にアクセスしたいなら、添え字で1回に1要素にアクセスするのではなく、全要素を走査することができます。
リスト8-8で`for`ループを使い、`i32`のベクタの各要素に対する不変な参照を得て、それらを出力する方法を示しています。

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

<!--
<span class="caption">Listing 8-8: Printing each element in a vector by
iterating over the elements using a `for` loop</span>
-->

<span class="caption">リスト8-8: `for`ループで要素を走査し、ベクタの各要素を出力する</span>

<!--
We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-9
will add `50` to each element.
-->

全要素に変更を加える目的で、可変なベクタの各要素への可変な参照を走査することもできます。
リスト8-9の`for`ループでは、各要素に`50`を足しています。

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

<!--
<span class="caption">Listing 8-9: Iterating over mutable references to
elements in a vector</span>
-->

<span class="caption">リスト8-9: ベクタの要素への可変な参照を走査する</span>

<!--
To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator.
-->

可変参照が参照している値を変更するには、`+=`演算子を使用する前に、
参照外し演算子(`*`)を使用して`i`の値に辿り着かないといけません。

<!--
### Using an Enum to Store Multiple Types
-->

### Enumを使って複数の型を保持する

<!--
At the beginning of this chapter, we said that vectors can only store values
that are the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of items of different types. Fortunately, the
variants of an enum are defined under the same enum type, so when we need to
store elements of a different type in a vector, we can define and use an enum!
-->

この章の冒頭で、ベクタは同じ型の値しか保持できないと述べました。これは不便なこともあります;
異なる型の要素を保持する必要性が出てくるユースケースも確かにあるわけです。幸運なことに、
enumの列挙子は、同じenumの型の元に定義されるので、ベクタに異なる型の要素を保持する必要が出たら、
enumを定義して使用することができます！

<!--
For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and then all the enum variants will be considered the same type:
that of the enum. Then we can create a vector that holds that enum and so,
ultimately, holds different types. We’ve demonstrated this in Listing 8-10.
-->

例えば、スプレッドシートの行から値を得たくなったとしましょう。ここで行の列には、整数を含むものや、
浮動小数点数を含むもの、文字列を含むものがあります。列挙子が異なる値の型を保持するenumを定義できます。
そして、このenumの列挙子は全て同じ型: enumの型と考えられるわけです。それからそのenumを保持するベクタを生成でき、
結果的に異なる型を保持できるようになるわけです。リスト8-10でこれを模擬しています。

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

<!--
<span class="caption">Listing 8-10: Defining an `enum` to store values of
different types in one vector</span>
-->

<span class="caption">リスト8-10: `enum`を定義して、一つのベクタに異なる型の値を保持する</span>

<!--
Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. A
secondary advantage is that we can be explicit about what types are allowed in
this vector. If Rust allowed a vector to hold any type, there would be a chance
that one or more of the types would cause errors with the operations performed
on the elements of the vector. Using an enum plus a `match` expression means
that Rust will ensure at compile time that every possible case is handled, as
discussed in Chapter 6.
-->

各要素を格納するのにヒープ上でズバリどれくらいのメモリが必要になるかをわかるように、
コンパイラがコンパイル時にベクタに入る型を知る必要があります。副次的な利点は、
このベクタではどんな型が許容されるのか明示できることです。もしRustでベクタがどんな型でも保持できたら、
ベクタの要素に対して行われる処理に対して一つ以上の型がエラーを引き起こす可能性があったでしょう。
enumに加えて`match`式を使うことは、第6章で議論した通り、コンパイル時にありうる場合全てに対処していることをコンパイラが、
保証できることを意味します。

<!--
When you're writing a program, if you don’t know the exhaustive set of types
the program will get at runtime to store in a vector, the enum technique won’t
work. Instead, you can use a trait object, which we’ll cover in Chapter 17.
-->

プログラム記述時にプログラムがベクタに実行時に保持するありとあらゆる一連の型をプログラマが知らない場合、
このenumテクニックはうまく動かないでしょう。代わりにトレイトオブジェクトを使用することができ、こちらは第17章で講義します。

<!--
Now that we’ve discussed some of the most common ways to use vectors, be sure
to review the API documentation for all the many useful methods defined on
`Vec<T>` by the standard library. For example, in addition to `push`, a `pop`
method removes and returns the last element. Let’s move on to the next
collection type: `String`!
-->

<!--
1行目、discussed some of を「について触れ、議論した」と訳した
-->

今や、ベクタを使用するべき最も一般的な方法について触れ、議論したので、標準ライブラリで`Vec<T>`に定義されている多くの有益なメソッドについては、
APIドキュメントを確認することを心得てください。例として、`push`に加えて、`pop`メソッドは最後の要素を削除して返します。
次のコレクション型に移りましょう: `String`です！
