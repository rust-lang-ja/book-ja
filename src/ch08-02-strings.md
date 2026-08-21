<!--
## Storing UTF-8 Encoded Text with Strings
-->

## 文字列でUTF-8でエンコードされたテキストを保持する

<!--
We talked about strings in Chapter 4, but we’ll look at them in more depth now.
New Rustaceans commonly get stuck on strings for a combination of three
reasons: Rust’s propensity for exposing possible errors, strings being a more
complicated data structure than many programmers give them credit for, and
UTF-8. These factors combine in a way that can seem difficult when you’re
coming from other programming languages.
-->

第4章で文字列について語りましたが、今度はより掘り下げていきましょう。新参者のRustaceanは、
3つの理由の組み合わせにより、文字列でよく行き詰まります: Rustのありうるエラーを晒す性質、
多くのプログラマが思っている以上に文字列が複雑なデータ構造であること、そしてUTF-8です。
これらの要因が、他のプログラミング言語から移ってきた場合、一見困難に見えるように絡み合うわけです。

<!--
We discuss strings in the context of collections because strings are
implemented as a collection of bytes, plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we’ll
talk about the operations on `String` that every collection type has, such as
creating, updating, and reading. We’ll also discuss the ways in which `String`
is different from the other collections, namely how indexing into a `String` is
complicated by the differences between how people and computers interpret
`String` data.
-->

文字列は、バイトのコレクションに、そのバイトをテキストとして解釈するときに便利な機能を提供するメソッドを足したものとして実装されているため、コレクションの文脈で文字列を議論していきます。
この節では、生成、更新、読み込みのような全コレクションが持つ`String`の処理について語ります。
また、`String`が他のコレクションと異なる点についても議論します。具体的には、人間とコンピュータが`String`データを解釈する方法の差異により、
`String`に添え字アクセスする方法がどう複雑なのかということです。

<!--
### What Is a String?
-->

### 文字列とは？

<!--
We’ll first define what we mean by the term *string*. Rust has only one string
type in the core language, which is the string slice `str` that is usually seen
in its borrowed form `&str`. In Chapter 4, we talked about *string slices*,
which are references to some UTF-8 encoded string data stored elsewhere. String
literals, for example, are stored in the program’s binary and are therefore
string slices.
-->

まずは、*文字列*という用語の意味を定義しましょう。Rustには、言語の核として1種類しか文字列型が存在しません。
文字列スライスの`str`で、通常借用された形態`&str`で見かけます。第4章で、*文字列スライス*について語りました。
これは、別の場所に格納されたUTF-8エンコードされた文字列データへの参照です。例えば、文字列リテラルは、
プログラムのバイナリに格納されるので、文字列スライスになります。

<!--
The `String` type, which is provided by Rust’s standard library rather than
coded into the core language, is a growable, mutable, owned, UTF-8 encoded
string type. When Rustaceans refer to “strings” in Rust, they might be
referring to either the `String` or the string slice `&str` types, not just one
of those types. Although this section is largely about `String`, both types are
used heavily in Rust’s standard library, and both `String` and string slices
are UTF-8 encoded.
-->

`String`型は、言語の核として組み込まれるのではなく、Rustの標準ライブラリによって提供されますが、伸長可能、
可変、所有権のあるUTF-8エンコードされた文字列型です。RustaceanがRustにおいて「文字列」と言うとき、`String`型または文字列スライスの`&str`型のいずれかを指しているかもしれません。この節は、大方、
`String`についてですが、どちらの型もRustの標準ライブラリで重宝されており、
どちらもUTF-8エンコードされています。

<!--
### Creating a New String
-->

### 新規文字列を生成する

<!--
Many of the same operations available with `Vec<T>` are available with `String`
as well, because `String` is actually implemented as a wrapper around a vector
of bytes with some extra guarantees, restrictions, and capabilities. An example
of a function that works the same way with `Vec<T>` and `String` is the `new`
function to create an instance, shown in Listing 8-11.
-->

`String`は実際のところ、いくつか追加の保障と制限と機能を持つバイトのベクタのラッパーとして実装されているので、`Vec<T>`で使用可能な操作の多くが`String`でも使用できます。
`Vec<T>`と`String`で同様に機能する関数の例としては、それぞれのインスタンスを作成する`new`関数があります。
リスト8-11に示したようにですね。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-11: Creating a new, empty `String`</span>
-->

<span class="caption">リスト8-11: 新しい空の`String`を生成する</span>

<!--
This line creates a new empty string called `s`, which we can then load data
into. Often, we’ll have some initial data that we want to start the string
with. For that, we use the `to_string` method, which is available on any type
that implements the `Display` trait, as string literals do. Listing 8-12 shows
two examples.
-->

この行は、新しい空の`s`という文字列を生成しています。それからここにデータを読み込むことができるわけです。
だいたい、文字列の初期値を決めるデータがあるでしょう。そのために、`to_string`メソッドを使用します。
このメソッドは、文字列リテラルのように、`Display`トレイトを実装する型ならなんでも使用できます。
リスト8-12に2例、示しています。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-12: Using the `to_string` method to create a
`String` from a string literal</span>
-->

<span class="caption">リスト8-12: `to_string`メソッドを使用して文字列リテラルから`String`を生成する</span>

<!--
This code creates a string containing `initial contents`.
-->

このコードは、`initial contents`(初期値)を含む文字列を生成します。

<!--
We can also use the function `String::from` to create a `String` from a string
literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12
that uses `to_string`.
-->

さらに、`String::from`関数を使っても、文字列リテラルから`String`を生成することができます。
リスト8-13のコードは、`to_string`を使用するリスト8-12のコードと等価です。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-13: Using the `String::from` function to create
a `String` from a string literal</span>
-->

<span class="caption">リスト8-13: `String::from`関数を使って文字列リテラルから`String`を作る</span>

<!--
Because strings are used for so many things, we can use many different generic
APIs for strings, providing us with a lot of options. Some of them can seem
redundant, but they all have their place! In this case, `String::from` and
`to_string` do the same thing, so which you choose is a matter of style and
readability.
-->

文字列は、非常に多くのものに使用されるので、多くの異なる一般的なAPIを使用でき、たくさんの選択肢があるわけです。
冗長に思われるものもありますが、適材適所です！今回の場合、`String::from`と`to_string`は全く同じことをします。
従って、どちらを選ぶかは、スタイルと可読性次第です。

<!--
Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them, as shown in Listing 8-14.
-->

文字列はUTF-8エンコードされていることを覚えていますか？要するに文字列には、適切にエンコードされていればどんなものでも含めます。
リスト8-14に示したように。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-14: Storing greetings in different languages in
strings</span>
-->

<span class="caption">リスト8-14: いろんな言語の挨拶を文字列に保持する</span>

<!--
All of these are valid `String` values.
-->

これらは全て、有効な`String`の値です。

<!--
### Updating a String
-->

### 文字列を更新する

<!--
A `String` can grow in size and its contents can change, just like the contents
of a `Vec<T>`, if you push more data into it. In addition, you can conveniently
use the `+` operator or the `format!` macro to concatenate `String` values.
-->

`String`は、サイズを伸ばすことができ、`Vec<T>`の中身のように、追加のデータをプッシュすれば、中身も変化します。
付け加えると、`String`値を連結する`+`演算子や、`format!`マクロを便利に使用することができます。

<!--
#### Appending to a String with `push_str` and `push`
-->

#### `push_str`と`push`で文字列に追加する

<!--
We can grow a `String` by using the `push_str` method to append a string slice,
as shown in Listing 8-15.
-->

`push_str`メソッドで文字列スライスを追記することで、`String`を伸ばすことができます。
リスト8-15の通りです。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-15: Appending a string slice to a `String`
using the `push_str` method</span>
-->

<span class="caption">リスト8-15: `push_str`メソッドで`String`に文字列スライスを追記する</span>

<!--
After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don’t necessarily want to take ownership of the
parameter. For example, in the code in Listing 8-16, we want to be able to use
`s2` after appending its contents to `s1`.
-->

この2行の後、`s`は`foobar`を含むことになります。`push_str`メソッドは、必ずしも引数の所有権を得なくていいので、
文字列スライスを取ります。例えば、リスト8-16のコードで、中身を`s1`に追加した後も`s2`を使いたいとします。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-16: Using a string slice after appending its
contents to a `String`</span>
-->

<span class="caption">リスト8-16: 中身を`String`に追加した後に、文字列スライスを使用する</span>

<!--
If the `push_str` method took ownership of `s2`, we wouldn’t be able to print
its value on the last line. However, this code works as we’d expect!
-->

もし、`push_str`メソッドが`s2`の所有権を奪っていたら、最後の行でその値を出力することは不可能でしょう。
ところが、このコードは予想通りに動きます！

<!--
The `push` method takes a single character as a parameter and adds it to the
`String`. Listing 8-17 adds the letter “l” to a `String` using the `push`
method.
-->

`push`メソッドは、1文字を引数として取り、`String`に追加します。リスト8-15は、
`push`メソッドで“l”を`String`に追加しています。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-17: Adding one character to a `String` value
using `push`</span>
-->

<span class="caption">リスト8-17: `push`で`String`値に1文字を追加する</span>

<!--
As a result, `s` will contain `lol`.
-->

この結果、`s`は`lol`を含むことになるでしょう。

> 編者注: `lol`は`laughing out loud`(大笑いする)の頭文字からできたスラングです。
> 日本語の`www`みたいなものですね。

<!--
#### Concatenation with the `+` Operator or the `format!` Macro
-->

#### `+`演算子、または`format!`マクロで連結

<!--
Often, you’ll want to combine two existing strings. One way to do so is to use
the `+` operator, as shown in Listing 8-18.
-->

2つのすでにある文字列を組み合わせたくなることがよくあります。
これを行うための方法の一つは、リスト8-18に示したように、`+`演算子を使用することです。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-18: Using the `+` operator to combine two
`String` values into a new `String` value</span>
-->

<span class="caption">リスト8-18: `+`演算子を使用して二つの`String`値を新しい`String`値にする</span>

<!--
The string `s3` will contain `Hello, world!`. The reason `s1` is no longer
valid after the addition, and the reason we used a reference to `s2`, has to do
with the signature of the method that’s called when we use the `+` operator.
The `+` operator uses the `add` method, whose signature looks something like
this:
-->

`s3`という文字列は、`Hello, world!`を含むことになるでしょう。
追記の後、`s1`がもう有効でなくなった理由と、`s2`への参照を使用した理由は、
`+`演算子を使用した時に呼ばれるメソッドのシグニチャと関係があります。`+`演算子は、`add`メソッドを使用し、
そのシグニチャは以下のような感じです:

```rust,ignore
fn add(self, s: &str) -> String {
```

<!--
In the standard library, you'll see `add` defined using generics and associated
types. Here, we’ve substituted in concrete types, which is what happens when we
call this method with `String` values. We’ll discuss generics in Chapter 10.
This signature gives us the clues we need to understand the tricky bits of the
`+` operator.
-->

標準ライブラリでは、`add`はジェネリクスと関連型を使用して定義されているのがわかるでしょう。
ここでは具体的な型で置き換えていますが、これはこのメソッドを`String`値とともに呼び出した時に起こることです。
ジェネリクスについては、第10章で議論します。
このシグニチャが、`+`演算子の巧妙な部分を理解するのに必要な手がかりになるのです。

<!--
First, `s2` has an `&`, meaning that we’re adding a *reference* of the second
string to the first string. This is because of the `s` parameter in the `add`
function: we can only add a `&str` to a `String`; we can’t add two `String`
values together. But wait—the type of `&s2` is `&String`, not `&str`, as
specified in the second parameter to `add`. So why does Listing 8-18 compile?
-->

まず、`s2`には`&`がついてます。つまり、最初の文字列に2番目の文字列の参照を追加するということです。
これは`add`関数の`s`引数のためのものです:
`String`には`&str`を追加することしかできません。要するに2つの`String`値を追加することはできないのです。
でも待ってください。`add`の第2引数で指定されているように、`&s2`の型は、`&str`ではなく、
`&String`ではないですか。では、なぜ、リスト8-18は、コンパイルできるのでしょうか？

<!--
The reason we’re able to use `&s2` in the call to `add` is that the compiler
can *coerce* the `&String` argument into a `&str`. When we call the `add`
method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`.
We’ll discuss deref coercion in more depth in Chapter 15. Because `add` does
not take ownership of the `s` parameter, `s2` will still be a valid `String`
after this operation.
-->

`add`呼び出しで`&s2`を使える理由は、コンパイラが`&String`引数を`&str`に*型強制*してくれるためです。
`add`メソッド呼び出しの際、コンパイラは、*参照外し型強制*というものを使用し、ここでは、
`&s2`を`&s2[..]`に変えるものと考えることができます。参照外し型強制について詳しくは、第15章で議論します。
`add`が`s`引数の所有権を奪わないので、この処理後も`s2`が有効な`String`になるわけです。

<!--
Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in Listing 8-18 will be
moved into the `add` call and will no longer be valid after that. So although
`let s3 = s1 + &s2;` looks like it will copy both strings and create a new one,
this statement actually takes ownership of `s1`, appends a copy of the contents
of `s2`, and then returns ownership of the result. In other words, it looks
like it’s making a lot of copies but isn’t; the implementation is more
efficient than copying.
-->

2番目に、シグニチャから`add`は`self`の所有権をもらうことがわかります。`self`には`&`がついてい*ない*からです。
これはつまり、リスト8-18において`s1`は`add`呼び出しにムーブされ、その後は有効ではなくなるということです。
故に、`s3 = s1 + &s2;`は両文字列をコピーして新しいものを作るように見えますが、
この文は実際には`s1`の所有権を奪い、`s2`の中身のコピーを追記し、結果の所有権を返すのです。言い換えると、
たくさんのコピーをしているように見えますが、違います; 実装は、コピーよりも効率的です。

<!--
If we need to concatenate multiple strings, the behavior of the `+` operator
gets unwieldy:
-->

複数の文字列を連結する必要が出ると、`+`演算子の振る舞いは扱いにくくなります:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

<!--
At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it’s difficult to see what’s going on. For more complicated string
combining, we can instead use the `format!` macro:
-->

ここで、`s`は`tic-tac-toe`になるでしょう。`+`と`"`文字のせいで何が起きているのかわかりにくいです。
もっと複雑な文字列の連結には、代わりに`format!`マクロを使用することができます:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

<!--
This code also sets `s` to `tic-tac-toe`. The `format!` macro works like
`println!`, but instead of printing the output to the screen, it returns a
`String` with the contents. The version of the code using `format!` is much
easier to read, and the code generated by the `format!` macro uses references
so that this call doesn’t take ownership of any of its parameters.
-->

このコードでも、`s`は`tic-tac-toe`になります。`format!`マクロは、`println!`と似た動作をしますが、
出力をスクリーンに行う代わりに、中身を`String`で返すのです。`format!`を使用したコードの方がはるかに読みやすく、`format!` マクロによって生成されたコードはは参照を使用するので、この呼び出しは引数の所有権を奪いません。

<!--
### Indexing into Strings
-->

### 文字列に添え字アクセスする

<!--
In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if you try to access parts of a `String` using indexing syntax in Rust, you’ll
get an error. Consider the invalid code in Listing 8-19.
-->

他の多くのプログラミング言語では、文字列中の文字に、添え字で参照してアクセスすることは、有効なコードであり、
一般的な処理です。しかしながら、Rustにおいて、添え字記法で`String`の一部にアクセスしようとすると、
エラーが発生するでしょう。リスト8-19の非合法なコードを考えてください。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-19: Attempting to use indexing syntax with a
String</span>
-->

<span class="caption">リスト8-19: 文字列に対して添え字記法を試みる</span>

<!--
This code will result in the following error:
-->

このコードは、以下のようなエラーに落ち着きます:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

<!--
The error and the note tell the story: Rust strings don’t support indexing. But
why not? To answer that question, we need to discuss how Rust stores strings in
memory.
-->

エラーと注釈が全てを物語っています: Rustの文字列は、添え字アクセスをサポートしていないのです。
でも、なぜでしょうか？その疑問に答えるには、Rustがメモリにどのように文字列を保持しているかについて議論する必要があります。

<!--
#### Internal Representation
-->

#### 内部表現

<!--
A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly
encoded UTF-8 example strings from Listing 8-14. First, this one:
-->

`String`は`Vec<u8>`のラッパです。リスト8-14から適切にUTF-8でエンコードされた文字列の例をご覧ください。
まずは、これ:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

<!--
In this case, `len` will be 4, which means the vector storing the string “Hola”
is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8. The
following line, however, may surprise you. (Note that this string begins with
the capital Cyrillic letter Ze, not the number 3.)
-->

この場合、`len`は4になり、これは、文字列"Hola"を保持するベクタの長さが4バイトであることを意味します。
これらの各文字は、UTF-8でエンコードすると、1バイトになるのです。しかし、次の行にはびっくりするかもしれません。
(この文字列は大文字のキリル文字ゼーで始まり、数字の3では始まっていないことに注意してください)

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

<!--
Asked how long the string is, you might say 12. In fact, Rust’s answer is 24:
that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because
each Unicode scalar value in that string takes 2 bytes of storage. Therefore,
an index into the string’s bytes will not always correlate to a valid Unicode
scalar value. To demonstrate, consider this invalid Rust code:
-->

文字列の長さはと問われたら、あなたは12と答えるかもしれません。実際には、Rustの答えは、24です:
“Здравствуйте”をUTF-8でエンコードすると、この長さになります。この文字列に含まれる各Unicodeスカラー値は、2バイトの領域を取るからです。
それ故に、文字列のバイトの添え字は、必ずしも有効なUnicodeのスカラー値とは相互に関係しないのです。
デモ用に、こんな非合法なRustコードを考えてください:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

<!--
You already know that `answer` will not be `З`, the first letter. When encoded
in UTF-8, the first byte of `З` is `208` and the second is `151`, so it would
seem that `answer` should in fact be `208`, but `208` is not a valid character
on its own. Returning `208` is likely not what a user would want if they asked
for the first letter of this string; however, that’s the only data that Rust
has at byte index 0. Users generally don’t want the byte value returned, even
if the string contains only Latin letters: if `&"hello"[0]` were valid code
that returned the byte value, it would return `104`, not `h`.
-->

`answer`の値は、最初の文字の`З`にはならないことを知っているでしょう。UTF-8エンコードされた時、
`З`の最初のバイトは`208`、2番目は`151`になるので、`answer`は実際には`208`になりそうですが、
`208`は単独では有効な文字ではありません。この文字列の最初の文字を求めている場合、`208`を返すことは、
ユーザの望んでいるものではないでしょう; しかしながら、Rustには、バイト添え字0の位置には、そのデータしかないのです。
文字列がラテン文字のみを含む場合でも、ユーザは一般的にバイト値が返ることを望みません:
`&"hello"[0]`がバイト値を返す有効なコードだったら、`h`ではなく、`104`を返すでしょう。

<!--
The answer, then, is that to avoid returning an unexpected value and causing
bugs that might not be discovered immediately, Rust doesn’t compile this code
at all and prevents misunderstandings early in the development process.
-->
答えは、予期しない値を返し、すぐには判明しないバグを引き起こさないために、Rustはこのコードを全くコンパイルせず、
開発過程の早い段階で誤解を防いでくれるのです。

<!--
#### Bytes and Scalar Values and Grapheme Clusters! Oh My!
-->

#### バイトとスカラー値と書記素クラスタ！なんてこった！

<!--
Another point about UTF-8 is that there are actually three relevant ways to
look at strings from Rust’s perspective: as bytes, scalar values, and grapheme
clusters (the closest thing to what we would call *letters*).
-->

UTF-8について別の要点は、実際Rustの観点から文字列を見るには3つの関連した方法があるということです:
バイトとして、スカラー値として、そして、書記素クラスタ(人間が*文字*と呼ぶものに一番近い)としてです。

<!--
If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:
-->

ヒンディー語の単語、“नमस्ते”をデーヴァナーガリー(`訳注`: サンスクリット語とヒンディー語を書くときに使われる書記法)で表記したものを見たら、
以下のような見た目の`u8`値のベクタとして保持されます:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

<!--
That’s 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust’s `char` type is, those
bytes look like this:
-->

18バイトになり、このようにしてコンピュータは最終的にこのデータを保持しているわけです。これをUnicodeスカラー値として見たら
（Rustの`char`型はこれなのですが）このバイトは以下のような見た目になります:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

<!--
There are six `char` values here, but the fourth and sixth are not letters:
they’re diacritics that don’t make sense on their own. Finally, if we look at
them as grapheme clusters, we’d get what a person would call the four letters
that make up the Hindi word:
-->

ここでは、6つ`char`値がありますが、4番目と6番目は文字ではありません: 単独では意味をなさないダイアクリティックです。
最後に、書記素クラスタとして見たら、このヒンディー語の単語を作り上げる人間が4文字と呼ぶであろうものが得られます:

```text
["न", "म", "स्", "ते"]
```

<!--
Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.
-->

Rustには、データが表す自然言語に関わらず、各プログラムが必要な解釈方法を選択できるように、
コンピュータが保持する生の文字列データを解釈する方法がいろいろ用意されています。

<!--
A final reason Rust doesn’t allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn’t possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.
-->

Rustで文字を得るのに`String`に添え字アクセスすることが許されない最後の理由は、
添え字アクセスという処理が常に定数時間(O(1))になると期待されるからです。
しかし、`String`でそのパフォーマンスを保証することはできません。というのも、
合法な文字がいくつあるか決定するのに、最初から添え字まで中身を走査する必要があるからです。

<!--
### Slicing Strings
-->

### 文字列をスライスする

<!--
Indexing into a string is often a bad idea because it’s not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. If you really need to use
indices to create string slices, therefore, Rust asks you to be more specific.
-->

文字列に添え字アクセスするのは、しばしば悪い考えです。文字列添え字処理の戻り値の型が明瞭ではないからです:
バイト値、文字、書記素クラスタ、あるいは文字列スライスにもなります。故に、文字列スライスを生成するのに、
添え字を使う必要が本当に出た場合にコンパイラは、もっと特定するよう求めてきます。

<!--
Rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:
-->

`[]`で1つの数値により添え字アクセスするのではなく、
範囲とともに`[]`を使って、特定のバイトを含む文字列スライスを作ることができます:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

<!--
Here, `s` will be a `&str` that contains the first 4 bytes of the string.
Earlier, we mentioned that each of these characters was 2 bytes, which means
`s` will be `Зд`.
-->

ここで、`s`は文字列の最初の4バイトを含む`&str`になります。先ほど、これらの文字は各々2バイトになると指摘しましたから、
`s`は`Зд`になります。

<!--
If we were to try to slice only part of a character’s bytes with something like
`&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid
index were accessed in a vector:
-->

`&hello[0..1]`のようにして、文字を構成するバイトの一部のみをスライスしようとすると、
Rustはベクタの非合法な添え字にアクセスしたかのように実行時にパニックするでしょう:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

<!--
You should use ranges to create string slices with caution, because doing so
can crash your program.
-->

範囲を使用して文字列スライスを作る際にはプログラムをクラッシュさせることがあるので、気をつけるべきです。

<!--
### Methods for Iterating Over Strings
-->

### 文字列を走査するメソッド群

<!--
The best way to operate on pieces of strings is to be explicit about whether
you want characters or bytes. For individual Unicode scalar values, use the
`chars` method. Calling `chars` on “Зд” separates out and returns two values
of type `char`, and you can iterate over the result to access each element:
-->

文字列の部分に対して操作を行うための最良の方法は、文字に対して操作したいのかバイトに対して操作したいのかを明示することです。
個々のUnicodeスカラー値に対しては、`chars`メソッドを使用してください。
“Зд”に対して`chars`を呼び出したら、分解して2つの`char`型の値を返すので、各要素にアクセスするには、
その結果を走査すればいいわけです:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

<!--
This code will print the following:
-->

このコードは、以下のように出力します:

```text
З
д
```

<!--
Alternatively, the `bytes` method returns each raw byte, which might be
appropriate for your domain:
-->

あるいは、`bytes`メソッドは各バイトをそのまま返すので、ドメインによってはこちらが適切かもしれません:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

<!--
This code will print the four bytes that make up this string:
-->

このコードは、この文字列をなす4バイトを出力します:

```text
208
151
208
180
```

<!--
But be sure to remember that valid Unicode scalar values may be made up of more
than 1 byte.
-->

ですが、合法なUnicodeスカラー値は、2バイト以上からなる場合もあることは心得ておいてください。

<!--
Getting grapheme clusters from strings as with the Devanagari script is
complex, so this functionality is not provided by the standard library. Crates
are available on [crates.io](https://crates.io/) if this is the
functionality you need.
-->

デーヴァナーガリー文字を含むような文字列から書記素クラスタを得る方法は複雑なので、この機能は標準ライブラリでは提供されていません。
この機能が必要なら、[crates.io](https://crates.io)でクレートを入手可能です。

<!--
### Strings Are Not So Simple
-->

### 文字列はそう単純じゃない

<!--
To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which means programmers have to put more thought into
handling UTF-8 data upfront. This trade-off exposes more of the complexity of
strings than is apparent in other programming languages, but it prevents you
from having to handle errors involving non-ASCII characters later in your
development life cycle.
-->

まとめると、文字列は込み入っています。プログラミング言語ごとにこの複雑性をプログラマに提示する方法は違います。
Rustでは、`String`データを正しく扱うことが、全てのRustプログラムにとっての既定動作になっているわけであり、
これは、プログラマがUTF-8データを素直に扱う際に、よりしっかり考えないといけないことを意味します。
このトレードオフにより、他のプログラミング言語で見えるよりも文字列の複雑性がより露出していますが、
ASCII以外の文字に関するエラーを開発の後半で扱わなければならない可能性が排除されているのです。

<!--
The good news is that the standard library offers a lot of functionality built
off the `String` and `&str` types to help handle these complex situations
correctly. Be sure to check out the documentation for useful methods like
`contains` for searching in a string and `replace` for substituting parts of a
string with another string.
-->

一方で良い面としては、こうした複雑な状況に正しく対処するために、標準ライブラリが`String`と`&str`型の上に構築された多数の機能を提供していることです。
文字列内の検索のための`contains`や、文字列の一部を別の文字列で置換するための`replace`などの便利なメソッドについて、ドキュメントを確認してみてください。

<!--
Let’s switch to something a bit less complex: hash maps!
-->

もう少し複雑でないものに切り替えていきましょう: ハッシュマップです！
