<!--
## Data Types
-->

## データ型

<!--
Every value in Rust is of a certain *data type*, which tells Rust what kind of
data is being specified so it knows how to work with that data. We'll look at
two data type subsets: scalar and compound.
-->

Rust における値は全て、何らかの*データ型*になり、コンパイラがどんなデータが指定されているか知れるので、
そのデータの取り扱い方も把握できるというわけです。2 種のデータ型のサブセットを見ましょう：スカラー型と複合型です。

<!--
Keep in mind that Rust is a *statically typed* language, which means that it
must know the types of all variables at compile time. The compiler can usually
infer what type we want to use based on the value and how we use it. In cases
when many types are possible, such as when we converted a `String` to a numeric
type using `parse` in the “Comparing the Guess to the Secret Number” section in
Chapter 2, we must add a type annotation, like this:
-->

Rust は*静的型付き*言語であることを弁えておいてください。つまり、
コンパイル時に全ての変数の型が判明している必要があるということです。コンパイラは通常、値と使用方法に基づいて、
使用したい型を推論してくれます。複数の型が推論される可能性がある場合、例えば、
第 2 章の「予想と秘密の数字を比較する」節で`parse`メソッドを使って`String`型を数値型に変換した時のように、
複数の型が可能な場合には、型注釈をつけなければいけません。以下のようにですね：

```rust
let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
```

<!--
If we don’t add the type annotation here, Rust will display the following
error, which means the compiler needs more information from us to know which
type we want to use:
-->

ここで型注釈を付けなければ、コンパイラは以下のエラーを表示し、これは可能性のある型のうち、
どの型を使用したいのかを知るのに、コンパイラがプログラマからもっと情報を得る必要があることを意味します：

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

<!--
You’ll see different type annotations for other data types.
-->

他のデータ型についても、様々な型注釈を目にすることになるでしょう。

<!--
### Scalar Types
-->

### スカラー型

<!--
A *scalar* type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters. You may recognize
these from other programming languages. Let’s jump into how they work in Rust.
-->

スカラー型は、単独の値を表します。Rust には主に 4 つのスカラー型があります：
整数、浮動小数点数、論理値、最後に文字です。他のプログラミング言語でも、これらの型を見かけたことはあるでしょう。
Rust での動作方法に飛び込みましょう。

<!--
#### Integer Types
-->

#### 整数型

<!--
An *integer* is a number without a fractional component. We used one integer
type in Chapter 2, the `u32` type. This type declaration indicates that the
value it’s associated with should be an unsigned integer (signed integer types
start with `i` instead of `u`) that takes up 32 bits of space. Table 3-1 shows
the built-in integer types in Rust. Each variant in the Signed and Unsigned
columns (for example, `i16`) can be used to declare the type of an integer
value.
-->

整数とは、小数部分のない数値のことです。第 2 章で一つの整数型を使用しましたね。`u32`型です。
この型定義は、紐付けられる値が、符号なし整数 (符号付き整数は`u`ではなく、`i`で始まります) になり、
これは、32 ビット分のサイズを取ります。表 3-1 は、Rust の組み込み整数型を表示しています。
符号付きと符号なし欄の各バリアント (例：`i16`) を使用して、整数値の型を宣言することができます。

<!--
<span class="caption">Table 3-1: Integer Types in Rust</span>
-->

<span class="caption">表 3-1: Rust の整数型</span>

<!--
| Length | Signed  | Unsigned |
|--------|---------|----------|
| 8-bit  | `i8`    | `u8`     |
| 16-bit | `i16`   | `u16`    |
| 32-bit | `i32`   | `u32`    |
| 64-bit | `i64`   | `u64`    |
| arch   | `isize` | `usize`  |
-->

| 大きさ  | 符号付き | 符号なし |
|--------|---------|---------|
| 8-bit  | `i8`    | `u8`    |
| 16-bit | `i16`   | `u16`   |
| 32-bit | `i32`   | `u32`   |
| 64-bit | `i64`   | `u64`   |
| arch   | `isize` | `usize` |

<!--
Each variant can be either signed or unsigned and has an explicit size.
*Signed* and *unsigned* refers to whether it’s possible for the number to be
negative or positive-in other words, whether the number needs to have a sign
with it (signed) or whether it will only ever be positive and can therefore be
represented without a sign (unsigned). It’s like writing numbers on paper: when
the sign matters, a number is shown with a plus sign or a minus sign; however,
when it’s safe to assume the number is positive, it’s shown with no sign.
Signed numbers are stored using two’s complement representation (if you’re
unsure what this is, you can search for it online; an explanation is outside
the scope of this book).
-->

各バリアントは、符号付きか符号なしかを選べ、明示的なサイズを持ちます。*符号付き*と*符号なし*は、
数値が正負を持つかどうかを示します。つまり、数値が符号を持つ必要があるかどうか (符号付き)、または、
絶対に正数にしかならず符号なしで表現できるかどうか (符号なし) です。これは、数値を紙に書き下すのと似ています：
符号が問題になるなら、数値はプラス記号、またはマイナス記号とともに表示されます; しかしながら、
その数値が正数であると仮定することが安全なら、符号なしで表示できるわけです。符号付き数値は、
2 の補数表現で保持されます (これが何なのか確信を持てないのであれば、ネットで検索することができます。
まあ要するに、この解説は、この本の範疇外というわけです)。

<!--
Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where *n* is the number of bits that variant uses. So an
`i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals
-128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1,
so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.
-->

各符号付きバリアントは、-(2<sup>n - 1</sup>) 以上 2<sup>n - 1</sup> - 1 以下の数値を保持でき、
ここで*n*はこのバリアントが使用するビット数です。以上から、`i8`型は-(2<sup>7</sup>) から 2<sup>7</sup> - 1 まで、
つまり、-128 から 127 までを保持できます。符号なしバリアントは、0 以上 2<sup>n</sup> - 1 以下を保持できるので、
`u8`型は、0 から 2<sup>8</sup> - 1 までの値、つまり、0 から 255 までを保持できることになります。

<!--
Additionally, the `isize` and `usize` types depend on the kind of computer your
program is running on: 64-bits if you’re on a 64-bit architecture and 32-bits
if you’re on a 32-bit architecture.
-->

加えて、`isize`と`usize`型は、プログラムが動作しているコンピュータの種類に依存します：
64 ビットアーキテクチャなら、64 ビットですし、32 ビットアーキテクチャなら、32 ビットになります。

<!--
You can write integer literals in any of the forms shown in Table 3-2. Note
that all number literals except the byte literal allow a type suffix, such as
`57u8`, and `_` as a visual separator, such as `1_000`.
-->

整数リテラル (`訳注`: リテラルとは、見たままの値ということ) は、表 3-2 に示すどの形式でも記述することができます。
バイトリテラルを除く数値リテラルは全て、
型接尾辞 (例えば、`57u8`) と`_`を見た目の区切り記号 (例えば、`1_000`) に付加することができます。

<!--
<span class="caption">Table 3-2: Integer Literals in Rust</span>
-->

<span class="caption">表 3-2: Rust の整数リテラル</span>

<!--
| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |
-->

| 数値リテラル       | 例            |
|------------------|---------------|
| 10 進数            | `98_222`      |
| 16 進数            | `0xff`        |
| 8 進数             | `0o77`        |
| 2 進数             | `0b1111_0000` |
| バイト (`u8`だけ)  | `b'A'`        |

<!--
So how do you know which type of integer to use? If you’re unsure, Rust’s
defaults are generally good choices, and integer types default to `i32`: this
type is generally the fastest, even on 64-bit systems. The primary situation in
which you’d use `isize` or `usize` is when indexing some sort of collection.
-->

では、どの整数型を使うべきかはどう把握すればいいのでしょうか？もし確信が持てないのならば、
Rust の基準型は一般的にいい選択肢になります。整数型の基準は`i32`型です：64 ビットシステム上でも、
この型が普通最速になります。`isize`と`usize`を使う主な状況は、何らかのコレクションにアクセスすることです。

<!--
#### Floating-Point Types
-->

#### 浮動小数点型

<!--
Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust’s floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because on modern CPUs it’s roughly the same speed as `f32` but is capable of
more precision.
-->

Rust にはさらに、*浮動小数点数*に対しても、2 種類の基本型があり、浮動小数点数とは数値に小数点がついたもののことです。
Rust の浮動小数点型は、`f32`と`f64`で、それぞれ 32 ビットと 64 ビットサイズです。基準型は`f64`です。
なぜなら、現代の CPU では、`f32`とほぼ同スピードにもかかわらず、より精度が高くなるからです。

<!--
Here’s an example that shows floating-point numbers in action:
-->

実際に動作している浮動小数点数の例をご覧ください：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

<!--
Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, and `f64` has double precision.
-->

浮動小数点数は、IEEE-754 規格に従って表現されています。`f32`が単精度浮動小数点数、
`f64`が倍精度浮動小数点数です。

<!--
#### Numeric Operations
-->

#### 数値演算

<!--
Rust supports the usual basic mathematical operations you’d expect for all of the
number types: addition, subtraction, multiplication, division, and remainder.
The following code shows how you’d use each one in a `let` statement:
-->

Rust にも全数値型に期待されうる標準的な数学演算が用意されています：足し算、引き算、掛け算、割り算、余りです。
以下の例では、`let`文での各演算の使用方法をご覧になれます：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

<!--
Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. Appendix B contains a
list of all operators that Rust provides.
-->

これらの文の各式は、数学演算子を使用しており、一つの値に評価され、そして、変数に束縛されます。
付録 B に Rust で使える演算子の一覧が載っています。

<!--
#### The Boolean Type
-->

#### 論理値型

<!--
As in most other programming languages, a boolean type in Rust has two possible
values: `true` and `false`. The boolean type in Rust is specified using `bool`.
For example:
-->

他の多くの言語同様、Rust の論理値型も取りうる値は二つしかありません：`true`と`false`です。
Rust の論理値型は、`bool`と指定されます。
例です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

<!--
The main way to use Boolean values is through conditionals, such as an `if`
expression. We’ll cover how `if` expressions work in Rust in the “Control Flow”
section.
-->

論理値を使う主な手段は、条件式です。例えば、`if`式などですね。`if`式の Rust での動作方法については、
「制御フロー」節で講義します。

<!--
#### The Character Type
-->

#### 文字型

<!--
So far we’ve worked only with numbers, but Rust supports letters too. Rust’s
`char` type is the language’s most primitive alphabetic type, and the following
code shows one way to use it. (Note that the `char` type is specified with
single quotes, as opposed to strings, which use double quotes.)
-->

ここまで、数値型のみ扱ってきましたが、Rust には文字も用意されています。Rust の`char`型は、
言語の最も基本的なアルファベット型であり、以下のコードでその使用方法の一例を見ることができます。
(`char`は、ダブルクォーテーションマークを使用する文字列に対して、シングルクォートで指定されることに注意してください。)

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

<!--
Rust’s `char` type represents a Unicode Scalar Value, which means it can
represent a lot more than just ASCII. Accented letters; Chinese, Japanese and
Korean characters; emoji; and zero-width spaces are all valid `char` types in
Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to
`U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode,
so your human intuition for what a “character” is may not match up with what a
`char` is in Rust. We’ll discuss this topic in detail in the “Strings” in Chapter 8.
-->

Rust の`char`型は、ユニコードのスカラー値を表します。これはつまり、アスキーよりもずっとたくさんのものを表せるということです。
アクセント文字; 中国語、日本語、韓国語文字;
絵文字; ゼロ幅スペースは、全て Rust では、有効な`char`型になります。ユニコードスカラー値は、
`U+0000`から`U+D7FF`までと`U+E000`から`U+10FFFF`までの範囲になります。
ところが、「文字」は実はユニコードの概念ではないので、文字とは何かという人間としての直観は、
Rust における`char`値が何かとは合致しない可能性があります。この話題については、第 8 章の「文字列」で詳しく議論しましょう。

<!--
### Compound Types
-->

### 複合型

<!--
*Compound types* can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.
-->

*複合型*により、複数の値を一つの型にまとめることができます。Rust には、
2 種類の基本的な複合型があります：タプルと配列です。

<!--
#### The Tuple type
-->

#### タプル型

<!--
A tuple is a general way of grouping together some number of other values with
a variety of types into one compound type.
-->

タプルは、複数の型の何らかの値を一つの複合型にまとめ上げる一般的な手段です。

<!--
We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:
-->

タプルは、丸かっこの中にカンマ区切りの値リストを書くことで生成します。タプルの位置ごとに型があり、
タプル内の値はそれぞれ全てが同じ型である必要はありません。今回の例では、型注釈をあえて追加しました：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

<!--
The variable `tup` binds to the entire tuple, because a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:
-->

変数`tup`は、タプル全体に束縛されています。なぜなら、タプルは、一つの複合要素と考えられるからです。
タプルから個々の値を取り出すには、パターンマッチングを使用して分解することができます。以下のように：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

<!--
This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.
-->

このプログラムは、まずタプルを生成し、それを変数`tup`に束縛しています。
それから`let`とパターンを使って`tup`変数の中身を 3 つの個別の変数 (`x`、`y`、`z`ですね) に変換しています。
この過程は、*分配*と呼ばれます。単独のタプルを破壊して三分割しているからです。最後に、
プログラムは`y`変数の値を出力し、`6.4`と表示されます。

<!--
In addition to destructuring through pattern matching, we can also access a tuple
element directly by using a period (`.`) followed by the index of the value we
want to access. For example:
-->

パターンマッチングを通しての分配の他にも、アクセスしたい値の番号をピリオド (`.`) に続けて書くことで、
タプルの要素に直接アクセスすることもできます。例です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

<!--
This program creates a tuple, `x`, and then makes new variables for each
element by using their index. As with most programming languages, the first
index in a tuple is 0.
-->

このプログラムは、新しいタプル`x`を作成し、添え字アクセスで各要素に対して新しい変数も作成しています。
多くのプログラミング言語同様、タプルの最初の添え字は 0 です。

<!--
#### The Array Type
-->

#### 配列型

<!--
Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Arrays in Rust are
different than arrays in some other languages because arrays in Rust have a
fixed length: once declared, they cannot grow or shrink in size.
-->

*配列*によっても、複数の値のコレクションを得ることができます。タプルと異なり、配列の全要素は、
同じ型でなければなりません。Rust の配列は、他の言語と異なっています。Rust の配列は、
固定長なのです：一度宣言されたら、サイズを伸ばすことも縮めることもできません。

<!--
In Rust, the values going into an array are written as a comma-separated list
inside square brackets:
-->

Rust では、配列に入れる要素は、角かっこ内にカンマ区切りリストとして記述します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

<!--
Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in Chapter 4) or when
you want to ensure you always have a fixed number of elements. An array isn't
as flexible as the vector type, though. A vector is a similar collection type
provided by the standard library that *is* allowed to grow or shrink in size.
If you’re unsure whether to use an array or a vector, you should probably use a
vector. Chapter 8 discusses vectors in more detail.
-->

配列は、ヒープよりもスタック (スタックとヒープについては第 4 章で<ruby>詳<rp>(</rp><rt>つまび</rt><rp>)</rp></ruby>らかに議論します) にデータのメモリを確保したい時、
または、常に固定長の要素があることを確認したい時に有効です。
ただ、配列は、ベクタ型ほど柔軟ではありません。ベクタは、標準ライブラリによって提供されている配列と似たようなコレクション型で、
こちらは、サイズを伸縮させることが*できます*。配列とベクタ型、どちらを使うべきか確信が持てない時は、
おそらくベクタ型を使うべきです。第 8 章でベクタについて詳細に議論します。

<!--
An example of when you might want to use an array rather than a vector is in a
program that needs to know the names of the months of the year. It’s very
unlikely that such a program will need to add or remove months, so you can use
an array because you know it will always contain 12 items:
-->

ベクタ型よりも配列を使いたくなるかもしれない例は、1 年の月の名前を扱うプログラムです。そのようなプログラムで、
月を追加したり削除したりすることまずないので、配列を使用できます。常に 12 個要素があることもわかってますからね：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

<!--
You write an array’s type using square brackets with the type of each element,
a semicolon, and then the number of elements in the array, like so:
-->

例えば次のように、配列の型は角かっこの中に要素の型とセミコロン、そして配列の要素数を与えます。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

<!--
Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the array contains five elements.
-->

ここでの`i32`は要素の型です。セミコロンのあとの`5`という数字は配列の要素が 5 つあることを表しています。

<!--
You can also initialize an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets, as shown here:
-->

次のように、角かっこの中に初期値とセミコロン、そして配列の長さを与えることで、各要素に同じ値を持つように配列を初期化することができます。

```rust
let a = [3; 5];
```

<!--
The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a
more concise way.
-->

この`a`という名前の配列は`3`という値が 5 つあるものです。これは`let a = [3, 3, 3, 3, 3];`と書くのと同じですが、より簡潔になります。

<!--
##### Accessing Array Elements
-->

##### 配列の要素にアクセスする

<!--
An array is a single chunk of memory allocated on the stack. You can access
elements of an array using indexing, like this:
-->

配列は、スタック上に確保される一塊のメモリです。添え字によって、
配列の要素にこのようにアクセスすることができます：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

<!--
In this example, the variable named `first` will get the value `1`, because
that is the value at index `[0]` in the array. The variable named `second` will
get the value `2` from index `[1]` in the array.
-->

この例では、`first`という名前の変数には`1`という値が格納されます。配列の`[0]`番目にある値が、
それだからですね。`second`という名前の変数には、配列の`[1]`番目の値`2`が格納されます。

<!--
##### Invalid Array Element Access
-->

##### 配列要素への無効なアクセス

<!--
What happens if you try to access an element of an array that is past the end
of the array? Say you change the example to the following code, which will
compile but exit with an error when it runs:
-->

配列の終端を越えて要素にアクセスしようとしたら、どうなるでしょうか？
先ほどの例を以下のように変えたとすると、コンパイルは通りますが、実行するとエラーで終了します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

<!--
This code compiles successfully. If you run this code using `cargo run` and
enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at
that index in the array. If you instead enter a number past the end of the
array, such as 10, you’ll see output like this:
-->

このコードはコンパイルされます。`cargo run`で走らせ、0, 1, 2, 3, または 4 をこのプログラムに入力すると配列の対応する値を出力します。もし配列の末尾を超えるような、例えば 10 などの数字を与えると、次のような出力が表示されます。

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
スレッド'main'は'範囲外アクセス: 長さは5ですが、添え字は10でした', src/main.rs:19:19
でパニックしました
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!--
The compilation didn’t produce any errors, but the program results in a
*runtime* error and didn’t exit successfully. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than the length, Rust will
*panic*, which is the term Rust uses when a program exits with an error.
-->

コンパイルでは何もエラーが出なかったものの、プログラムは*実行時*エラーに陥り、
正常終了しませんでした。要素に添え字アクセスを試みると、言語は、
指定されたその添え字が配列長よりも小さいかを確認してくれます。添え字が配列長よりも大きければ、言語は*パニック*します。
パニックとは、プログラムがエラーで終了したことを表す Rust 用語です。

<!--
This is the first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling.
-->

これは、実際に稼働している Rust の安全機構の最初の例になります。低レベル言語の多くでは、
この種のチェックは行われないため、間違った添え字を与えると、無効なメモリにアクセスできてしまいます。
Rust では、メモリアクセスを許可し、処理を継続する代わりに即座にプログラムを終了することで、
この種のエラーからプログラマを保護しています。Rust のエラー処理については、第 9 章でもっと議論します。
