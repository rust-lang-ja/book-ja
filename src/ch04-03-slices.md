<!--
## The Slice Type
-->

## スライス型

<!--
*Slices* let you reference a contiguous sequence of elements in a collection
rather than the whole collection. A slice is a kind of reference, so it does
not have ownership.
-->

*スライス*により、コレクション全体ではなく、その内の連続した要素の列を参照することができます。
スライスは参照の一種であり、そのため所有権を持っていません。

<!--
Here’s a small programming problem: write a function that takes a string of
words separated by spaces and returns the first word it finds in that string.
If the function doesn’t find a space in the string, the whole string must be
one word, so the entire string should be returned.
-->

ちょっとしたプログラミングの問題を考えてみましょう: スペースで区切られた複数の単語からなる文字列を受け取って、その文字列中の最初の単語を返す関数を書いてください。
関数が文字列中に空白を見つけられなかったら、文字列全体が一つの単語に違いないので、文字列全体が返されるべきです。

<!--
Let’s work through how we’d write the signature of this function without using
slices, to understand the problem that slices will solve:
-->

スライスが解決しようとしている問題を理解するために、スライスを使わずにこの関数のシグネチャをどう書くかという問題を通して、
考えてみましょう。

```rust,ignore
fn first_word(s: &String) -> ?
```

<!--
The `first_word` function has a `&String` as a parameter. We don’t want
ownership, so this is fine. But what should we return? We don’t really have a
way to talk about *part* of a string. However, we could return the index of the
end of the word, indicated by a space. Let’s try that, as shown in Listing 4-7.
-->

`first_word`関数は引数に`&String`をとります。所有権はいらないので、これで十分です。
ですが、何を返すべきでしょうか？文字列の*一部*について語る方法が全くありません。しかし、
空白によって示される単語の終端の添え字を返すことができますね。リスト4-7に示したように、その方法を試してみましょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<!--
<span class="caption">Listing 4-7: The `first_word` function that returns a
byte index value into the `String` parameter</span>
-->

<span class="caption">リスト4-7: `String`引数へのバイト数で表された添え字を返す`first_word`関数</span>

<!--
Because we need to go through the `String` element by element and check whether
a value is a space, we’ll convert our `String` to an array of bytes using the
`as_bytes` method.
-->

`String`の値を要素ごとに見て、空白かどうかを確かめる必要があるので、
`as_bytes`メソッドを使って、`String`オブジェクトをバイト配列に変換しています。

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

<!--
Next, we create an iterator over the array of bytes using the `iter` method:
-->

次に、そのバイト配列に対して、`iter`メソッドを使用してイテレータを生成しています:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

<!--
We’ll discuss iterators in more detail in [Chapter 13][ch13].
For now, know that `iter` is a method that returns each element in a collection
and that `enumerate` wraps the result of `iter` and returns each element as
part of a tuple instead. The first element of the tuple returned from
`enumerate` is the index, and the second element is a reference to the element.
This is a bit more convenient than calculating the index ourselves.
-->

イテレータについて詳しくは、[第13章][ch13]で議論します。今は、`iter`は、コレクション内の各要素を返すメソッドであること、
`enumerate`が`iter`の結果をラップして、（結果をそのまま返す）代わりにタプルの一部として各要素を返すことを知っておいてください。
`enumerate`から返ってくるタプルの第1要素は、添え字であり、2番目の要素は、(コレクションの）要素への参照になります。
これは、手動で添え字を計算するよりも少しだけ便利です。

<!--
Because the `enumerate` method returns a tuple, we can use patterns to
destructure that tuple. We’ll be discussing patterns more in [Chapter
6][ch6]. In the `for` loop, we specify a pattern that has `i`
for the index in the tuple and `&item` for the single byte in the tuple.
Because we get a reference to the element from `.iter().enumerate()`, we use
`&` in the pattern.
-->

`enumerate`メソッドがタプルを返すので、パターンを使って、そのタプルを分配できます。
パターンについては[第6章][ch6]でさらに議論します。
`for`ループ内で、タプルの添え字に対する`i`とタプルの1バイトに対応する`&item`を含むパターンを指定しています。
`.iter().enumerate()`から要素への参照を取得するので、パターンに`&`を使っています。

<!--
Inside the `for` loop, we search for the byte that represents the space by
using the byte literal syntax. If we find a space, we return the position.
Otherwise, we return the length of the string by using `s.len()`.
-->

`for`ループ内で、バイトリテラル表記を使用して空白を表すバイトを検索しています。空白が見つかったら、その位置を返します。
それ以外の場合、`s.len()`を使って文字列の長さを返します。

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

<!--
We now have a way to find out the index of the end of the first word in the
string, but there’s a problem. We’re returning a `usize` on its own, but it’s
only a meaningful number in the context of the `&String`. In other words,
because it’s a separate value from the `String`, there’s no guarantee that it
will still be valid in the future. Consider the program in Listing 4-8 that
uses the `first_word` function from Listing 4-7.
-->

さて、文字列内の最初の単語の終端の添え字を見つけ出せるようになりましたが、問題があります。
`usize`型を単独で返していますが、これは`&String`の文脈でのみ意味を持つ数値です。
言い換えると、`String`から切り離された値なので、将来的にも有効である保証がないのです。
リスト4-7の`first_word`関数を使用するリスト4-8のプログラムを考えてください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 4-8: Storing the result from calling the
`first_word` function and then changing the `String` contents</span>
-->

<span class="caption">リスト4-8: `first_word`関数の呼び出し結果を保持し、`String`の中身を変更する</span>

<!--
This program compiles without any errors and would also do so if we used `word`
after calling `s.clear()`. Because `word` isn’t connected to the state of `s`
at all, `word` still contains the value `5`. We could use that value `5` with
the variable `s` to try to extract the first word out, but this would be a bug
because the contents of `s` have changed since we saved `5` in `word`.
-->

このプログラムは何のエラーもなくコンパイルが通り、`word`を`s.clear()`の呼び出し後に使用しても、
コンパイルが通ります。`word`は`s`の状態に全く関連づけられていないので、その中身はまだ値`5`のままです。
その値`5`を変数`s`に使用し、最初の単語を取り出そうとすることはできますが、これはバグでしょう。
というのも、`s`の中身は、`5`を`word`に保存した後変わってしまったからです。

<!--
Having to worry about the index in `word` getting out of sync with the data in
`s` is tedious and error prone! Managing these indices is even more brittle if
we write a `second_word` function. Its signature would have to look like this:
-->

`word`内の添え字が`s`に格納されたデータと同期されなくなるのを心配することは、面倒ですし間違いになりやすいです！
これらの添え字の管理は、`second_word`関数を書いたら、さらに難しくなります。
そのシグニチャは以下のようになるはずです:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

<!--
Now we’re tracking a starting *and* an ending index, and we have even more
values that were calculated from data in a particular state but aren’t tied to
that state at all. We have three unrelated variables floating around that need
to be kept in sync.
-->

今、私たちは開始*と*終端の添え字を追うようになりました。特定の状態のデータから計算されたが、
その状態に全く紐付けられていない値がさらに増えました。いつの間にか変わってしまうので、同期を取る必要のある、関連性のない変数が3つになってしまいました。

<!--
Luckily, Rust has a solution to this problem: string slices.
-->

運のいいことに、Rustにはこの問題への解決策が用意されています: 文字列スライスです。

<!--
### String Slices
-->

### 文字列スライス

<!--
A *string slice* is a reference to part of a `String`, and it looks like this:
-->

*文字列スライス*とは、`String`の一部への参照で、こんな見た目をしています:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

<!--
Rather than a reference to the entire `String`, `hello` is a reference to a
portion of the `String`, specified in the extra `[0..5]` bit. We create slices
using a range within brackets by specifying `[starting_index..ending_index]`,
where `starting_index` is the first position in the slice and `ending_index` is
one more than the last position in the slice. Internally, the slice data
structure stores the starting position and the length of the slice, which
corresponds to `ending_index` minus `starting_index`. So, in the case of `let
world = &s[6..11];`, `world` would be a slice that contains a pointer to the
byte at index 6 of `s` with a length value of `5`.
-->

`hello`は`String`全体への参照ではなく、追加の`[0..5]`という部分で指定された、`String`の一部への参照です。
`[starting_index..ending_index]`と指定することで、角かっこに範囲を使い、スライスを生成します。
ここで、`starting_index`はスライスの最初の位置、`ending_index`はスライスの終端位置よりも、
1大きい値です。内部的には、スライスデータ構造は、開始地点とスライスの長さを保持しており、
スライスの長さは`ending_index`から`starting_index`を引いたものに対応します。以上より、
`let world = &s[6..11];`の場合には、`world`は`s`の添え字6のバイトへのポインタと`5`という長さを持つスライスになるでしょう。

<!--
Figure 4-6 shows this in a diagram.
-->

図4-6は、これを図解しています。

<!--
<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table rep-resents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-06.svg" class="center" style="width: 50%;" />
-->

<img alt="3個の表: sのスタックデータを表現するテーブルは、ヒープ上の文字列データ&quot;hello world&quot;のテーブル内の添え字0のバイトを指している。
3個目の表はスライスworldのスタックデータを表現していて、長さの値5を持ち、ヒープデータの表のバイト6を指している。"
src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-6: String slice referring to part of a
`String`</span>
-->

<span class="caption">図4-6: `String`オブジェクトの一部を参照する文字列スライス</span>

<!--
With Rust’s `..` range syntax, if you want to start at index 0, you can drop
the value before the two periods. In other words, these are equal:
-->

Rustの`..`という範囲記法で、添え字0から始めたければ、2連ピリオドの前に値を書かなければいいです。
換言すれば、これらは等価です:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

<!--
By the same token, if your slice includes the last byte of the `String`, you
can drop the trailing number. That means these are equal:
-->

同様の意味で、`String`の最後のバイトをスライスが含むのならば、末尾の数値を書かなければいいです。
つまり、これらは等価になります:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

<!--
You can also drop both values to take a slice of the entire string. So these
are equal:
-->

さらに、両方の値を省略すると、文字列全体のスライスを得られます。故に、これらは等価です:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

<!--
> Note: String slice range indices must occur at valid UTF-8 character
> boundaries. If you attempt to create a string slice in the middle of a
> multibyte character, your program will exit with an error. For the purposes
> of introducing string slices, we are assuming ASCII only in this section; a
> more thorough discussion of UTF-8 handling is in the [“Storing UTF-8 Encoded
> Text with Strings”][strings] section of Chapter 8.
-->

> 注釈: 文字列スライスの範囲添え字は、有効なUTF-8文字境界に置かなければなりません。
> マルチバイト文字の真ん中で文字列スライスを生成しようとしたら、エラーでプログラムは落ちるでしょう。
> この節では文字列スライスを導入することが目的なので、ASCIIのみを想定しています; UTF-8に関するより徹底した議論は、
> 第8章の[「文字列でUTF-8エンコードされたテキストを格納する」][strings]節で行います。

<!--
With all this information in mind, let’s rewrite `first_word` to return a
slice. The type that signifies “string slice” is written as `&str`:
-->

これらの情報を念頭に、`first_word`を書き直してスライスを返すようにしましょう。
文字列スライスを意味する型は、`&str`と記述します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

<!--
We get the index for the end of the word the same way we did in Listing 4-7, by
looking for the first occurrence of a space. When we find a space, we return a
string slice using the start of the string and the index of the space as the
starting and ending indices.
-->

リスト4-7で取った方法と同じように、最初の空白を探すことで単語の終端の添え字を取得しています。
空白を発見したら、文字列の最初を開始地点、空白の添え字を終了地点として使用して文字列スライスを返しています。

<!--
Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.
-->

これで、`first_word`を呼び出すと、元のデータに紐付けられた単独の値を得られるようになりました。
この値は、スライスの開始地点への参照とスライス中の要素数から構成されています。

<!--
Returning a slice would also work for a `second_word` function:
-->

`second_word`関数についても、スライスを返すことでうまくいくでしょう:

```rust,ignore
fn second_word(s: &String) -> &str {
```

<!--
We now have a straightforward API that’s much harder to mess up because the
compiler will ensure the references into the `String` remain valid. Remember
the bug in the program in Listing 4-8, when we got the index to the end of the
first word but then cleared the string so our index was invalid? That code was
logically incorrect but didn’t show any immediate errors. The problems would
show up later if we kept trying to use the first word index with an emptied
string. Slices make this bug impossible and let us know we have a problem with
our code much sooner. Using the slice version of `first_word` will throw a
compile-time error:
-->

これで、ずっと混乱しにくい素直なAPIになりました。なぜなら、`String`への参照が有効なままであることをコンパイラが、
保証してくれるからです。最初の単語の終端添え字を得た時に、
文字列を空っぽにして先ほどの添え字が無効になってしまったリスト4-8のプログラムのバグを覚えていますか？
そのコードは、論理的に正しくないのですが、即座にエラーにはなりませんでした。問題は後になってから発生し、
それは空の文字列に対して、最初の単語の添え字を使用し続けようとした時でした。スライスならこんなバグはあり得ず、
コードに問題があるなら、もっと迅速に判明します。スライスバージョンの`first_word`を使用すると、
コンパイルエラーが発生します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

<!--
Here’s the compiler error:
-->

こちらがコンパイルエラーです:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

<!--
Recall from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Because `clear` needs to
truncate the `String`, it needs to get a mutable reference. The `println!`
after the call to `clear` uses the reference in `word`, so the immutable
reference must still be active at that point. Rust disallows the mutable
reference in `clear` and the immutable reference in `word` from existing at the
same time, and compilation fails. Not only has Rust made our API easier to use,
but it has also eliminated an entire class of errors at compile time!
-->

借用規則から、何かへの不変な参照がある時、さらに可変な参照を得ることはできないことを思い出してください。
`clear`は`String`を切り詰める必要があるので、可変な参照を得る必要があります。
`clear`の呼び出しの後の`println!`は`word`中の参照を使用するので、不変参照はその時点でもまだ有効でなくてはいけません。
Rustは`clear`中の可変参照と`word`中の不変参照が同時に存在することを認めないので、コンパイルが失敗します。
RustのおかげでAPIが使いやすくなるだけでなく、ある種のエラー全てを完全にコンパイル時に排除してくれるのです！

<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="string-literals-are-slices"></a>
-->

<!--
#### String Literals as Slices
-->

#### スライスとしての文字列リテラル

<!--
Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:
-->

文字列は、バイナリに埋め込まれると話したことを思い出してください。今やスライスのことを知ったので、
文字列リテラルを正しく理解することができます。

```rust
let s = "Hello, world!";
```

<!--
The type of `s` here is `&str`: it’s a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.
-->

ここでの`s`の型は、`&str`です: バイナリのその特定の位置を指すスライスです。
これは、文字列が不変である理由にもなっています。要するに、`&str`は不変な参照なのです。

<!--
#### String Slices as Parameters
-->

#### 引数としての文字列スライス

<!--
Knowing that you can take slices of literals and `String` values leads us to
one more improvement on `first_word`, and that’s its signature:
-->

リテラルや`String`値のスライスを得ることができると知ると、`first_word`に対して、もう一つ改善点を見出すことができます。
シグニチャです:

```rust,ignore
fn first_word(s: &String) -> &str {
```

<!--
A more experienced Rustacean would write the signature shown in Listing 4-9
instead because it allows us to use the same function on both `&String` values
and `&str` values.
-->

もっと経験を積んだRustaceanなら、代わりにリスト4-9のようなシグニチャを書くでしょう。というのも、こうすると、
同じ関数を`&String`値と`&str`値両方に使えるようになるからです。

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 4-9: Improving the `first_word` function by using
a string slice for the type of the `s` parameter</span>
-->

<span class="caption">リスト4-9: `s`引数の型に文字列スライスを使用して`first_word`関数を改善する</span>

<!--
If we have a string slice, we can pass that directly. If we have a `String`, we
can pass a slice of the `String` or a reference to the `String`. This
flexibility takes advantage of *deref coercions*, a feature we will cover in
[“Implicit Deref Coercions with Functions and
Methods”][deref-coercions] section of Chapter 15.
-->

もし、文字列スライスがあるなら、それを直接渡せます。`String`があるなら、
その`String`のスライスか、`String`への参照を渡せます。この柔軟性は、第15章の[「関数やメソッドで暗黙的な参照外し型強制」][deref-coercions]で扱う機能、
*参照外し型強制*の利点を活用して実現されています。

<!--
Defining a function to take a string slice instead of a reference to a `String`
makes our API more general and useful without losing any functionality:
-->

`String`への参照の代わりに文字列スライスを取るよう関数を定義すると、
何も機能を失うことなくAPIをより一般的で有益なものにできるのです。

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

<!--
### Other Slices
-->

### 他のスライス

<!--
String slices, as you might imagine, are specific to strings. But there’s a
more general slice type too. Consider this array:
-->

文字列リテラルは、ご想像通り、文字列に特化したものです。ですが、もっと一般的なスライス型も存在します。
この配列を考えてください:

```rust
let a = [1, 2, 3, 4, 5];
```

<!--
Just as we might want to refer to part of a string, we might want to refer to
part of an array. We’d do so like this:
-->

文字列の一部を参照したくなる可能性があるのと同様、配列の一部を参照したくなる可能性もあります。
以下のようにすれば、参照することができます:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

<!--
This slice has the type `&[i32]`. It works the same way as string slices do, by
storing a reference to the first element and a length. You’ll use this kind of
slice for all sorts of other collections. We’ll discuss these collections in
detail when we talk about vectors in Chapter 8.
-->

このスライスは、`&[i32]`という型になります。これも文字列スライスと同じように動作します。
つまり、最初の要素への参照と長さを保持するのです。
この種のスライスは、他のすべての種類のコレクションに対して使用することになるでしょう。
それらのコレクションについて、詳しくは、第8章でベクタについて話すときに議論します。

<!--
## Summary
-->

## まとめ

<!--
The concepts of ownership, borrowing, and slices ensure memory safety in Rust
programs at compile time. The Rust language gives you control over your memory
usage in the same way as other systems programming languages, but having the
owner of data automatically clean up that data when the owner goes out of scope
means you don’t have to write and debug extra code to get this control.
-->

所有権、借用、スライスの概念は、Rustプログラムにおいて、コンパイル時にメモリ安全性を保証します。
Rust言語も他のシステムプログラミング言語と同じように、メモリの使用法について制御させてくれるわけですが、
データの所有者がスコープを抜けたときに、所有者に自動的にデータを片付けさせることは、この制御をするために、
余計なコードを書いたりデバッグしたりする必要がないことを意味します。

<!--
Ownership affects how lots of other parts of Rust work, so we’ll talk about
these concepts further throughout the rest of the book. Let’s move on to
Chapter 5 and look at grouping pieces of data together in a `struct`.
-->

所有権は、Rustの他のいろんな部分が動作する方法に影響を与えるので、これ以降もこれらの概念についてさらに語っていく予定です。
第5章に移って、`struct`でデータをグループ化することについて見ていきましょう。

<!--
[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
-->

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#値に束縛されるパターン
[strings]: ch08-02-strings.html#文字列でutf-8でエンコードされたテキストを保持する
[deref-coercions]: ch15-02-deref.html#関数やメソッドで暗黙的な参照外し型強制
