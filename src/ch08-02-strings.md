<!--
## Storing UTF-8 Encoded Text with Strings
-->

## 文字列で UTF-8 でエンコードされたテキストを保持する

<!--
We talked about strings in Chapter 4, but we’ll look at them in more depth now.
New Rustaceans commonly get stuck on strings for a combination of three
concepts: Rust’s propensity for exposing possible errors, strings being a more
complicated data structure than many programmers give them credit for, and
UTF-8. These factors combine in a way that can seem difficult when you’re
coming from other programming languages.
-->

第 4 章で文字列について語りましたが、今度はより掘り下げていきましょう。新参者の Rustacean は、
3 つの概念の組み合わせにより、文字列でよく行き詰まります：Rust のありうるエラーを晒す性質、
多くのプログラマが思っている以上に文字列が複雑なデータ構造であること、そして UTF-8 です。
これらの要因が、他のプログラミング言語から移ってきた場合、一見困難に見えるように絡み合うわけです。

<!--
It's useful to discuss strings in the context of collections because strings
are implemented as a collection of bytes, plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we’ll
talk about the operations on `String` that every collection type has, such as
creating, updating, and reading. We’ll also discuss the ways in which `String`
is different from the other collections, namely how indexing into a `String` is
complicated by the differences between how people and computers interpret
`String` data.
-->

コレクションの文脈で文字列を議論することは、有用なことです。なぜなら、文字列はテキストとして解釈された時に有用になる機能を提供するメソッドと、
バイトのコレクションで実装されているからです。この節では、生成、更新、読み込みのような全コレクションが持つ`String`の処理について語ります。
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
literals, for example, are stored in the binary output of the program and are
therefore string slices.
-->

まずは、*文字列*という用語の意味を定義しましょう。Rust には、言語の核として 1 種類しか文字列型が存在しません。
文字列スライスの`str`で、通常借用された形態`&str`で見かけます。第 4 章で、*文字列スライス*について語りました。
これは、別の場所に格納された UTF-8 エンコードされた文字列データへの参照です。例えば、文字列リテラルは、
プログラムのバイナリ出力に格納されるので、文字列スライスになります。

<!--
The `String` type, which is provided in Rust’s standard library rather than
coded into the core language, is a growable, mutable, owned, UTF-8 encoded
string type. When Rustaceans refer to “strings” in Rust, they usually mean the
`String` and the string slice `&str` types, not just one of those types.
Although this section is largely about `String`, both types are used heavily in
Rust's standard library, and both `String` and string slices are UTF-8 encoded.
-->

`String`型は、言語の核として組み込まれるのではなく、Rust の標準ライブラリで提供されますが、伸長可能、
可変、所有権のある UTF-8 エンコードされた文字列型です。Rustacean が Rust において「文字列」を指したら、
どちらかではなく、`String`と文字列スライスの`&str`のことを通常意味します。この節は、大方、
`String`についてですが、どちらの型も Rust の標準ライブラリで重宝されており、
どちらも UTF-8 エンコードされています。

<!--
Rust’s standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates can provide even
more options for storing string data. See how those names all end in `String`
or `Str`? They refer to owned and borrowed variant, just like the `String` and
`str` types you've seen previously. These string types can store text in
different encodings or be represented in memory in a different way, for
example. We won’t discuss these other string types in this chapter; see their
API documentation for more about how to use them and when each is appropriate.
-->

また、Rust の標準ライブラリには、他の文字列型も含まれています。`OsString`、`OsStr`、`CString`、`CStr`などです。
ライブラリクレートにより、文字列データを格納する選択肢はさらに増えます。
それらの名前が全て`String`か`Str`で終わっているのがわかりますか？所有権ありと借用されたバージョンを指しているのです。
ちょうど以前見かけた`String`と`&str`のようですね。例えば、これらの文字列型は、異なるエンコード方法でテキストを格納していたり、
メモリ上の表現が異なったりします。この章では、これらの他の種類の文字列については議論しません;
使用方法やどれが最適かについては、API ドキュメントを参照してください。

<!--
### Creating a New String
-->

### 新規文字列を生成する

<!--
Many of the same operations available with `Vec<T>` are available with `String`
as well, starting with the `new` function to create a string, shown in Listing
8-11.
-->

`Vec<T>`で使用可能な処理の多くが`String`でも使用できます。文字列を生成する`new`関数から始めましょうか。
リスト 8-11 に示したようにですね。

```rust
let mut s = String::new();
```

<!--
<span class="caption">Listing 8-11: Creating a new, empty `String`</span>
-->

<span class="caption">リスト 8-11: 新しい空の`String`を生成する</span>

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
リスト 8-12 に 2 例、示しています。

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

<!--
<span class="caption">Listing 8-12: Using the `to_string` method to create a
`String` from a string literal</span>
-->

<span class="caption">リスト 8-12: `to_string`メソッドを使用して文字列リテラルから`String`を生成する</span>

<!--
This code creates a string containing `initial contents`.
-->

このコードは、`initial contents`(初期値) を含む文字列を生成します。

<!--
We can also use the function `String::from` to create a `String` from a string
literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12
that uses `to_string`.
-->

さらに、`String::from`関数を使っても、文字列リテラルから`String`を生成することができます。
リスト 8-13 のコードは、`to_string`を使用するリスト 8-12 のコードと等価です。

```rust
let s = String::from("initial contents");
```

<!--
<span class="caption">Listing 8-13: Using the `String::from` function to create
a `String` from a string literal</span>
-->

<span class="caption">リスト 8-13: `String::from`関数を使って文字列リテラルから`String`を作る</span>

<!--
Because strings are used for so many things, we can use many different generic
APIs for strings, providing us with a lot of options. Some of them can seem
redundant, but they all have their place! In this case, `String::from` and
`to_string` do the same thing, so which you choose is a matter of style.
-->

文字列は、非常に多くのものに使用されるので、多くの異なる一般的な API を使用でき、たくさんの選択肢があるわけです。
冗長に思われるものもありますが、適材適所です！今回の場合、`String::from`と`to_string`は全く同じことをします。
従って、どちらを選ぶかは、スタイル次第です。

<!--
Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them, as shown in Listing 8-14.
-->

文字列は UTF-8 エンコードされていることを覚えていますか？要するに文字列には、適切にエンコードされていればどんなものでも含めます。
リスト 8-14 に示したように。

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

<!--
<span class="caption">Listing 8-14: Storing greetings in different languages in
strings</span>
-->

<span class="caption">リスト 8-14: いろんな言語の挨拶を文字列に保持する</span>

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
of a `Vec<T>`, if you push more data into it. In addition, we can conveniently
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
リスト 8-15 の通りです。

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

<!--
<span class="caption">Listing 8-15: Appending a string slice to a `String`
using the `push_str` method</span>
-->

<span class="caption">リスト 8-15: `push_str`メソッドで`String`に文字列スライスを追記する</span>

<!--
After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don’t necessarily want to take ownership of the
parameter. For example, the code in Listing 8-16 shows that it would be
unfortunate if we weren’t able to use `s2` after appending its contents to `s1`.
-->

この 2 行の後、`s`は`foobar`を含むことになります。`push_str`メソッドは、必ずしも引数の所有権を得なくていいので、
文字列スライスを取ります。例えば、リスト 8-16 のコードは、中身を`s1`に追加した後、
`s2`を使えなかったら残念だということを示しています。

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```

<!--
<span class="caption">Listing 8-16: Using a string slice after appending its
contents to a `String`</span>
-->

<span class="caption">リスト 8-16: 中身を`String`に追加した後に、文字列スライスを使用する</span>

<!--
If the `push_str` method took ownership of `s2`, we wouldn’t be able to print
its value on the last line. However, this code works as we’d expect!
-->

もし、`push_str`メソッドが`s2`の所有権を奪っていたら、最後の行でその値を出力することは不可能でしょう。
ところが、このコードは予想通りに動きます！

<!--
The `push` method takes a single character as a parameter and adds it to the
`String`. Listing 8-17 shows code that adds the letter *l* to a `String` using
the `push` method.
-->

`push`メソッドは、1 文字を引数として取り、`String`に追加します。リスト 8-15 は、
`push`メソッドで*l*を`String`に追加するコードを呈示しています。

```rust
let mut s = String::from("lo");
s.push('l');
```

<!--
<span class="caption">Listing 8-17: Adding one character to a `String` value
using `push`</span>
-->

<span class="caption">リスト 8-17: `push`で`String`値に 1 文字を追加する</span>

<!--
As a result of this code, `s` will contain `lol`.
-->

このコードの結果、`s`は`lol`を含むことになるでしょう。

> 編者注：`lol`は`laughing out loud`(大笑いする) の頭文字からできたスラングです。
> 日本語の`www`みたいなものですね。

<!--
#### Concatenation with the `+` Operator or the `format!` Macro
-->

#### `+`演算子、または`format!`マクロで連結

<!--
Often, you’ll want to combine two existing strings. One way is to use the `+`
operator, as shown in Listing 8-18.
-->

2 つのすでにある文字列を組み合わせたくなることがよくあります。リスト 8-18 に示したように、
一つ目の方法は、`+`演算子を使用することです。

<!--
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note that s1 has been moved here and can no longer be used
```
-->

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 はムーブされ、もう使用できないことに注意
```

<!--
<span class="caption">Listing 8-18: Using the `+` operator to combine two
`String` values into a new `String` value</span>
-->

<span class="caption">リスト 8-18: `+`演算子を使用して二つの`String`値を新しい`String`値にする</span>

<!--
The string `s3` will contain `Hello, world!` as a result of this code. The
reason `s1` is no longer valid after the addition and the reason we used a
reference to `s2` has to do with the signature of the method that gets called
when we use the `+` operator. The `+` operator uses the `add` method, whose
signature looks something like this:
-->

このコードの結果、`s3`という文字列は、`Hello, world!`を含むことになるでしょう。
追記の後、`s1`がもう有効でなくなった理由と、`s2`への参照を使用した理由は、
`+`演算子を使用した時に呼ばれるメソッドのシグニチャと関係があります。`+`演算子は、`add`メソッドを使用し、
そのシグニチャは以下のような感じです：

```rust,ignore
fn add(self, s: &str) -> String {
```

<!--
This isn’t the exact signature that’s in the standard library: in the standard
library, `add` is defined using generics. Here, we’re looking at the signature
of `add` with concrete types substituted for the generic ones, which is what
happens when we call this method with `String` values. We’ll discuss generics
in Chapter 10. This signature gives us the clues we need to understand the
tricky bits of the `+` operator.
-->

これは、標準ライブラリにあるシグニチャそのものではありません：標準ライブラリでは、`add`はジェネリクスで定義されています。
ここでは、ジェネリックな型を具体的な型に置き換えた`add`のシグニチャを見ており、これは、
このメソッドを`String`値とともに呼び出した時に起こることです。ジェネリクスについては、第 10 章で議論します。
このシグニチャが、`+`演算子の巧妙な部分を理解するのに必要な手がかりになるのです。

<!--
First, `s2` has an `&`, meaning that we’re adding a *reference* of the second
string to the first string because of the `s` parameter in the `add` function:
we can only add a `&str` to a `String`; we can’t add two `String` values
together. But wait-the type of `&s2` is `&String`, not `&str`, as specified in
the second parameter to `add`. So why does Listing 8-18 compile?
-->

まず、`s2`には`&`がついてます。つまり、`add`関数の`s`引数のために最初の文字列に 2 番目の文字列の参照を追加するということです：
`String`には`&str`を追加することしかできません。要するに 2 つの`String`値を追加することはできないのです。
でも待ってください。`add`の第 2 引数で指定されているように、`&s2`の型は、`&str`ではなく、
`&String`ではないですか。では、なぜ、リスト 8-18 は、コンパイルできるのでしょうか？

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
`&s2`を`&s2[..]`に変えるものと考えることができます。参照外し型強制について詳しくは、第 15 章で議論します。
`add`が`s`引数の所有権を奪わないので、この処理後も`s2`が有効な`String`になるわけです。

<!--
Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in Listing 8-18 will be
moved into the `add` call and no longer be valid after that. So although `let
s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this
statement actually takes ownership of `s1`, appends a copy of the contents of
`s2`, and then returns ownership of the result. In other words, it looks like
it’s making a lot of copies but isn’t; the implementation is more efficient
than copying.
-->

2 番目に、シグニチャから`add`は`self`の所有権をもらうことがわかります。`self`には`&`がついてい*ない*からです。
これはつまり、リスト 8-18 において`s1`は`add`呼び出しにムーブされ、その後は有効ではなくなるということです。
故に、`s3 = s1 + &s2;`は両文字列をコピーして新しいものを作るように見えますが、
この文は実際には`s1`の所有権を奪い、`s2`の中身のコピーを追記し、結果の所有権を返すのです。言い換えると、
たくさんのコピーをしているように見えますが、違います; 実装は、コピーよりも効率的です。

<!--
If we need to concatenate multiple strings, the behavior of `+` operator
gets unwieldy:
-->

複数の文字列を連結する必要が出ると、`+`演算子の振る舞いは扱いにくくなります：

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

<!--
At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it’s difficult to see what’s going on. For more complicated string
combining, we can use the `format!` macro:
-->

ここで、`s`は`tic-tac-toe`になるでしょう。`+`と`"`文字のせいで何が起きているのかわかりにくいです。
もっと複雑な文字列の連結には、`format!`マクロを使用することができます：

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

<!--
This code also sets `s` to `tic-tac-toe`. The `format!` macro works in the same
way as `println!`, but instead of printing the output to the screen, it returns
a `String` with the contents. The version of the code using `format!` is much
easier to read and doesn’t take ownership of any of its parameters.
-->

このコードでも、`s`は`tic-tac-toe`になります。`format!`マクロは、`println!`と同様の動作をしますが、
出力をスクリーンに行う代わりに、中身を`String`で返すのです。`format!`を使用したコードの方がはるかに読みやすく、
引数の所有権を奪いません。

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
一般的な処理です。しかしながら、Rust において、添え字記法で`String`の一部にアクセスしようとすると、
エラーが発生するでしょう。リスト 8-19 の非合法なコードを考えてください。

```rust,ignore
let s1 = String::from("hello");
let h = s1[0];
```

<!--
<span class="caption">Listing 8-19: Attempting to use indexing syntax with a
String</span>
-->

<span class="caption">リスト 8-19: 文字列に対して添え字記法を試みる</span>

<!--
This code will result in the following error:
-->

このコードは、以下のようなエラーに落ち着きます：

```text
error[E0277]: the trait bound `std::string::String: std::ops::Index<{Integer}>` is not satisfied
(エラー: トレイト境界`std::string::String: std::ops::Index<{Integer}>`が満たされていません)
  |>
3 |>     let h = s1[0];
  |>             ^^^^^ the type `std::string::String` cannot be indexed by `{Integer}`
  |>                   (型`std::string::String`は`{Integer}`で添え字アクセスできません)
  = help: the trait `std::ops::Index<{Integer}>` is not implemented for `std::string::String`
  (ヘルプ：`std::ops::Index<{Integer}>`というトレイトが`std::string::String`に対して実装されていません)
```

<!--
The error and the note tell the story: Rust strings don’t support indexing. But
why not? To answer that question, we need to discuss how Rust stores strings in
memory.
-->

エラーと注釈が全てを物語っています：Rust の文字列は、添え字アクセスをサポートしていないのです。
でも、なぜでしょうか？その疑問に答えるには、Rust がメモリにどのように文字列を保持しているかについて議論する必要があります。

<!--
#### Internal Representation
-->

#### 内部表現

<!--
A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly
encoded UTF-8 example strings from Listing 8-14. First, this one:
-->

`String`は`Vec<u8>`のラッパです。リスト 8-14 から適切に UTF-8 でエンコードされた文字列の例をご覧ください。
まずは、これ：

```rust
let len = String::from("Hola").len();
```

<!--
In this case, `len` will be 4, which means the vector storing the string “Hola”
is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8. But
what about the following line? (Note that this string begins with the capital
Cyrillic letter Ze, not the Arabic number 3.)
-->

この場合、`len`は 4 になり、これは、文字列"Hola"を保持するベクタの長さが 4 バイトであることを意味します。
これらの各文字は、UTF-8 でエンコードすると、1 バイトになるのです。しかし、以下の行ではどうでしょうか？
(この文字列は大文字のキリル文字 Ze で始まり、アラビア数字の 3 では始まっていないことに注意してください)

```rust
let len = String::from("Здравствуйте").len();
```

<!--
Asked how long the string is, you might say 12. However, Rust’s answer is 24:
that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because
each Unicode scalar value takes two bytes of storage. Therefore, an index into
the string’s bytes will not always correlate to a valid Unicode scalar value.
To demonstrate, consider this invalid Rust code:
-->

文字列の長さはと問われたら、あなたは 12 と答えるかもしれません。ところが、Rust の答えは、24 です：
“Здравствуйте”を UTF-8 でエンコードすると、この長さになります。各 Unicode スカラー値は、2 バイトの領域を取るからです。
それ故に、文字列のバイトの添え字は、必ずしも有効な Unicode のスカラー値とは相互に関係しないのです。
デモ用に、こんな非合法な Rust コードを考えてください：

```rust,ignore
let hello = "Здравствуйте";
let answer = &hello[0];
```

<!--
What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208`, and the second is `151`, so
`answer` should in fact be `208`, but `208` is not a valid character on its
own. Returning `208` is likely not what a user would want if they asked for the
first letter of this string; however, that’s the only data that Rust has at
byte index 0. Users generally don't want the byte value returned, even if
the string contains only Latin letters: if `&"hello"[0]` was valid code that
returned the byte value, it would return `104`, not `h`. To avoid returning an
unexpected value and causing bugs that might not be discovered immediately,
Rust doesn’t compile this code at all and prevents misunderstandings early in
the development process.
-->

`answer`の値は何になるべきでしょうか？最初の文字の`З`になるべきでしょうか？UTF-8 エンコードされた時、
`З`の最初のバイトは`208`、2 番目は`151`になるので、`answer`は実際、`208`になるべきですが、
`208`は単独では有効な文字ではありません。この文字列の最初の文字を求めている場合、`208`を返すことは、
ユーザの望んでいるものではないでしょう; しかしながら、Rust には、バイト添え字 0 の位置には、そのデータしかないのです。
文字列がラテン文字のみを含む場合でも、ユーザは一般的にバイト値が返ることを望みません：
`&"hello"[0]`がバイト値を返す有効なコードだったら、`h`ではなく、`104`を返すでしょう。
予期しない値を返し、すぐには判明しないバグを引き起こさないために、Rust はこのコードを全くコンパイルせず、
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

UTF-8 について別の要点は、実際 Rust の観点から文字列を見るには 3 つの関連した方法があるということです：
バイトとして、スカラー値として、そして、書記素クラスタ (人間が*文字*と呼ぶものに一番近い) としてです。

<!--
If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:
-->

ヒンディー語の単語、“नमस्ते”をデーヴァナーガリー(`訳注`: サンスクリット語とヒンディー語を書くときに使われる書記法) で表記したものを見たら、
以下のような見た目の`u8`値のベクタとして保持されます：

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

<!--
That’s 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust’s `char` type is, those
bytes look like this:
-->

18 バイトになり、このようにしてコンピュータは最終的にこのデータを保持しているわけです。これを Unicode スカラー値として見たら
（Rust の`char`型はこれなのですが）このバイトは以下のような見た目になります：

```text
['न', 'म', 'स', '्', 'त', 'े']
```

<!--
There are six `char` values here, but the fourth and sixth are not letters:
they’re diacritics that don’t make sense on their own. Finally, if we look at
them as grapheme clusters, we’d get what a person would call the four letters
that make up the Hindi word:
-->

ここでは、6 つ`char`値がありますが、4 番目と 6 番目は文字ではありません：単独では意味をなさないダイアクリティックです。
最後に、書記素クラスタとして見たら、このヒンディー語の単語を作り上げる人間が 4 文字と呼ぶであろうものが得られます：

```text
["न", "म", "स्", "ते"]
```

<!--
Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.
-->

Rust には、データが表す自然言語に関わらず、各プログラムが必要な解釈方法を選択できるように、
コンピュータが保持する生の文字列データを解釈する方法がいろいろ用意されています。

<!--
A final reason Rust doesn’t allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn’t possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.
-->

Rust で文字を得るのに`String`に添え字アクセスすることが許されない最後の理由は、
添え字アクセスという処理が常に定数時間 (O(1)) になると期待されるからです。
しかし、`String`でそのパフォーマンスを保証することはできません。というのも、
合法な文字がいくつあるか決定するのに、最初から添え字まで中身を走査する必要があるからです。

<!--
### Slicing Strings
-->

### 文字列をスライスする

<!--
Indexing into a string is often a bad idea because it’s not clear what the
return type of the string indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. Therefore, Rust asks you to
be more specific if you really need to use indices to create string slices. To
be more specific in your indexing and indicate that you want a string slice,
rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:
-->

文字列に添え字アクセスするのは、しばしば悪い考えです。文字列添え字処理の戻り値の型が明瞭ではないからです：
バイト値、文字、書記素クラスタ、あるいは文字列スライスにもなります。故に、文字列スライスを生成するのに、
添え字を使う必要が本当に出た場合にコンパイラは、もっと特定するよう求めてきます。添え字アクセスを特定し、
文字列スライスが欲しいと示唆するためには、`[]`で 1 つの数値により添え字アクセスするのではなく、
範囲とともに`[]`を使って、特定のバイトを含む文字列スライスを作ることができます：

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

<!--
Here, `s` will be a `&str` that contains the first 4 bytes of the string.
Earlier, we mentioned that each of these characters was 2 bytes, which means
`s` will be `Зд`.
-->

ここで、`s`は文字列の最初の 4 バイトを含む`&str`になります。先ほど、これらの文字は各々 2 バイトになると指摘しましたから、
`s`は`Зд`になります。

<!--
What would happen if we used `&hello[0..1]`? The answer: Rust would panic at
runtime in the same way as if an invalid index were accessed in a vector:
-->

`&hello[0..1]`と使用したら、何が起きるでしょうか？答え：Rust はベクタの非合法な添え字にアクセスしたかのように、
実行時にパニックするでしょう：

```text
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
('main'スレッドは「バイト添え字 1 は文字の境界ではありません; `Здравствуйте`の'З'(バイト番号 0 から 2) の中です」でパニックしました)
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
Fortunately, you can access elements in a string in other ways.
-->

幸いなことに、他の方法でも文字列の要素にアクセスすることができます。

<!--
If you need to perform operations on individual Unicode scalar values, the best
way to do so is to use the `chars` method. Calling `chars` on “नमस्ते” separates
out and returns six values of type `char`, and you can iterate over the result
to access each element:
-->

もし、個々の Unicode スカラー値に対して処理を行う必要があったら、最適な方法は`chars`メソッドを使用するものです。
“नमस्ते”に対して`chars`を呼び出したら、分解して 6 つの`char`型の値を返すので、各要素にアクセスするには、
その結果を走査すればいいわけです：

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

<!--
This code will print the following:
-->

このコードは、以下のように出力します：

```text
न
म
स
्
त
े
```

<!--
The `bytes` method returns each raw byte, which might be appropriate for your
domain:
-->

`bytes`メソッドは、各バイトをそのまま返すので、最適になることもあるかもしれません：

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

<!--
This code will print the 18 bytes that make up this `String`:
-->

このコードは、`String`をなす 18 バイトを出力します：

```text
224
164
// --snip--
165
135
```

<!--
But be sure to remember that valid Unicode scalar values may be made up of more
than 1 byte.
-->

ですが、合法な Unicode スカラー値は、2 バイト以上からなる場合もあることは心得ておいてください。

<!--
Getting grapheme clusters from strings is complex, so this functionality is not
provided by the standard library. Crates are available on
[crates.io](https://crates.io) if this is the functionality you need.
-->

書記素クラスタを文字列から得る方法は複雑なので、この機能は標準ライブラリでは提供されていません。
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
Rust では、`String`データを正しく扱うことが、全ての Rust プログラムにとっての既定動作になっているわけであり、
これは、プログラマが UTF-8 データを素直に扱う際に、よりしっかり考えないといけないことを意味します。
このトレードオフにより、他のプログラミング言語で見えるよりも文字列の複雑性がより露出していますが、
ASCII 以外の文字に関するエラーを開発の後半で扱わなければならない可能性が排除されているのです。

<!--
Let’s switch to something a bit less complex: hash maps!
-->

もう少し複雑でないものに切り替えていきましょう：ハッシュマップです！
