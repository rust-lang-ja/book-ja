<!--
## Data Types
-->

## データ型

<!--
Every value in Rust is of a certain *data type*, which tells Rust what kind of
data is being specified so it knows how to work with that data. We’ll look at
two data type subsets: scalar and compound.
-->

Rustにおける値は全て、何らかの*データ型*になり、コンパイラがどんなデータが指定されているか知れるので、
そのデータの取り扱い方も把握できるというわけです。2種のデータ型のサブセットを見ましょう: スカラー型と複合型です。

<!--
Keep in mind that Rust is a *statically typed* language, which means that it
must know the types of all variables at compile time. The compiler can usually
infer what type we want to use based on the value and how we use it. In cases
when many types are possible, such as when we converted a `String` to a numeric
type using `parse` in the [“Comparing the Guess to the Secret
Number”][comparing-the-guess-to-the-secret-number] section in
Chapter 2, we must add a type annotation, like this:
-->

Rustは*静的型付き*言語であることを弁えておいてください。つまり、
コンパイル時に全ての変数の型が判明している必要があるということです。コンパイラは通常、値と使用方法に基づいて、
使用したい型を推論してくれます。複数の型が推論される可能性がある場合、例えば、
第2章の[「予想と秘密の数字を比較する」][comparing-the-guess-to-the-secret-number]節で`parse`メソッドを使って`String`型を数値型に変換した時のように、
複数の型が可能な場合には、型注釈をつけなければいけません。以下のようにですね:

```rust
let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
```

<!--
If we don’t add the `: u32` type annotation shown in the preceding code, Rust
will display the following error, which means the compiler needs more
information from us to know which type we want to use:
-->

上のコード中に示す`: u32`型注釈を付けなければ、コンパイラは以下のエラーを表示し、これは可能性のある型のうち、
どの型を使用したいのかを知るのに、コンパイラがプログラマからもっと情報を得る必要があることを意味します:

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

*スカラー*型は、単独の値を表します。Rustには主に4つのスカラー型があります:
整数、浮動小数点数、論理値、最後に文字です。他のプログラミング言語でも、これらの型を見かけたことはあるでしょう。
Rustでの動作方法に飛び込みましょう。

<!--
#### Integer Types
-->

#### 整数型

<!--
An *integer* is a number without a fractional component. We used one integer
type in Chapter 2, the `u32` type. This type declaration indicates that the
value it’s associated with should be an unsigned integer (signed integer types
start with `i` instead of `u`) that takes up 32 bits of space. Table 3-1 shows
the built-in integer types in Rust. We can use any of these variants to declare
the type of an integer value.
-->

*整数*とは、小数部分のない数値のことです。第2章で一つの整数型を使用しましたね。`u32`型です。
この型定義は、紐付けられる値が、符号なし整数(符号付き整数は`u`ではなく、`i`で始まります)になり、
これは、32ビット分のサイズを取ります。表3-1は、Rustの組み込み整数型を表示しています。
これらのバリアントを使用して、整数値の型を宣言することができます。

<!--
<span class="caption">Table 3-1: Integer Types in Rust</span>
-->

<span class="caption">表3-1: Rustの整数型</span>

<!--
| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |
-->

| 大きさ  | 符号付き | 符号なし |
|---------|----------|----------|
| 8-bit   | `i8`     | `u8`     |
| 16-bit  | `i16`    | `u16`    |
| 32-bit  | `i32`    | `u32`    |
| 64-bit  | `i64`    | `u64`    |
| 128-bit | `i128`   | `u128`   |
| arch    | `isize`  | `usize`  |

<!--
Each variant can be either signed or unsigned and has an explicit size.
*Signed* and *unsigned* refer to whether it’s possible for the number to be
negative—in other words, whether the number needs to have a sign with it
(signed) or whether it will only ever be positive and can therefore be
represented without a sign (unsigned). It’s like writing numbers on paper: when
the sign matters, a number is shown with a plus sign or a minus sign; however,
when it’s safe to assume the number is positive, it’s shown with no sign.
Signed numbers are stored using [two’s complement][twos-complement]
representation.
-->

各バリアントは、符号付きか符号なしかを選べ、明示的なサイズを持ちます。*符号付き*と*符号なし*は、
数値が負の数になり得るかどうかを示します。つまり、数値が符号を持つ必要があるかどうか(符号付き)、または、
絶対に正数にしかならず符号なしで表現できるかどうか(符号なし)です。これは、数値を紙に書き下すのと似ています:
符号が問題になるなら、数値はプラス記号、またはマイナス記号とともに表示されます; しかしながら、
その数値が正数であると仮定することが安全なら、符号なしで表示できるわけです。符号付き数値は、
[2の補数][twos-complement]表現で保持されます。

<!--
Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where *n* is the number of bits that variant uses. So an
`i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals
-128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1,
so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.
-->

各符号付きバリアントは、-(2<sup>n - 1</sup>)以上2<sup>n - 1</sup> - 1以下の数値を保持でき、
ここで*n*はこのバリアントが使用するビット数です。以上から、`i8`型は-(2<sup>7</sup>)から2<sup>7</sup> - 1まで、
つまり、-128から127までを保持できます。符号なしバリアントは、0以上2<sup>n</sup> - 1以下を保持できるので、
`u8`型は、0から2<sup>8</sup> - 1までの値、つまり、0から255までを保持できることになります。

<!--
Additionally, the `isize` and `usize` types depend on the architecture of the
computer your program is running on, which is denoted in the table as “arch”:
64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit
architecture.
-->

加えて、`isize`と`usize`型は、表では「arch」と表記していますが、プログラムが動作しているコンピュータのアーキテクチャに依存します:
64ビットアーキテクチャなら、64ビットですし、32ビットアーキテクチャなら、32ビットになります。

<!--
You can write integer literals in any of the forms shown in Table 3-2. Note
that number literals that can be multiple numeric types allow a type suffix,
such as `57u8`, to designate the type. Number literals can also use `_` as a
visual separator to make the number easier to read, such as `1_000`, which will
have the same value as if you had specified `1000`.
-->

整数リテラル(`訳注`: リテラルとは、見たままの値ということ)は、表3-2に示すどの形式でも記述することができます。
複数の数値型になることができる数値リテラルは、型を指示するために型接尾辞をつけて、`57u8`のように書くことができます。
数値リテラルはさらに、数値を読みやすくするために見た目の区切り記号として`_`をつけて、`1_000`のように書くこともできます。
これは`1000`と指定した場合とまったく同じ値となるでしょう。

<!--
<span class="caption">Table 3-2: Integer Literals in Rust</span>
-->

<span class="caption">表3-2: Rustの整数リテラル</span>

<!--
| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |
-->

| 数値リテラル      | 例            |
|-------------------|---------------|
| 10進数            | `98_222`      |
| 16進数            | `0xff`        |
| 8進数             | `0o77`        |
| 2進数             | `0b1111_0000` |
| バイト (`u8`だけ) | `b'A'`        |

<!--
So how do you know which type of integer to use? If you’re unsure, Rust’s
defaults are generally good places to start: integer types default to `i32`.
The primary situation in which you’d use `isize` or `usize` is when indexing
some sort of collection.
-->

では、どの整数型を使うべきかはどう把握すればいいのでしょうか？もし確信が持てないのならば、
Rustの基準型は一般的にいい開始地点になります: 整数型の基準は`i32`型です。
`isize`と`usize`を使う主な状況は、何らかのコレクションにアクセスすることです。

<!--
> ##### Integer Overflow
>
> Let’s say you have a variable of type `u8` that can hold values between 0 and
> 255. If you try to change the variable to a value outside that range, such as
> 256, *integer overflow* will occur, which can result in one of two behaviors.
> When you’re compiling in debug mode, Rust includes checks for integer overflow
> that cause your program to *panic* at runtime if this behavior occurs. Rust
> uses the term *panicking* when a program exits with an error; we’ll discuss
> panics in more depth in the [“Unrecoverable Errors with
> `panic!`”][unrecoverable-errors-with-panic] section in Chapter
> 9.
>
> When you’re compiling in release mode with the `--release` flag, Rust does
> *not* include checks for integer overflow that cause panics. Instead, if
> overflow occurs, Rust performs *two’s complement wrapping*. In short, values
> greater than the maximum value the type can hold “wrap around” to the minimum
> of the values the type can hold. In the case of a `u8`, the value 256 becomes
> 0, the value 257 becomes 1, and so on. The program won’t panic, but the
> variable will have a value that probably isn’t what you were expecting it to
> have. Relying on integer overflow’s wrapping behavior is considered an error.
>
> To explicitly handle the possibility of overflow, you can use these families
> of methods provided by the standard library for primitive numeric types:
>
> * Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
> * Return the `None` value if there is overflow with the `checked_*` methods.
> * Return the value and a boolean indicating whether there was overflow with
>   the `overflowing_*` methods.
> * Saturate at the value’s minimum or maximum values with the `saturating_*`
>   methods.
-->

> ##### 整数オーバーフロー
>
> `u8`型の変数があるとしましょう。`u8`は0から255までの間の値を取ることができます。
> この変数を範囲外の値、例えば256に変更しようとすると、*整数オーバーフロー (integer overflow)* が発生し、次の2つのうちのどちらかの挙動になります。
> デバッグモードでコンパイルしているときは、もしこの挙動が発生したときは実行時にプログラムを*パニック (panic)* させるような、整数オーバーフローのチェックをコンパイラが入れ込みます。
> プログラムがエラーとともに終了するとき、Rustは*パニック*という用語を使用します;
> パニックについては第9章の[「`panic!`で回復不能なエラー」][unrecoverable-errors-with-panic]でより深く議論します。
>
> `--release`フラグを付けてリリースモードでコンパイルしているときは、コンパイラはパニックを引き起こす整数オーバーフローチェックを入れ込み*ません*。
> 代わりに、オーバーフローが起きたときは、プログラムは*2の補数ラップアラウンド (two's complement wrapping)* を行います。
> 一言で言うと、その型が取ることができる最大値よりも大きい値は、その型が取ることができる最小値に「回り込む」 (“wrap around”) のです。
> `u8`の場合は、値256は0になり、値257は1になり、という感じです。
> プログラムはパニックはしなくなるでしょうが、変数が持っている値はおそらくプログラマが期待していたものではないでしょう。
> 整数オーバーフローのラップアラウンドの挙動に依存するのは、エラーと考えられます。
>
> オーバーフローが発生する可能性を明示的に取り扱うためには、プリミティブ数値型に関して標準ライブラリが提供する、以下のメソッド群を使うことができます:
>
> * `wrapping_*`メソッド（`wrapping_add`等）で、モードを問わずラップアラウンドさせる。
> * `checked_*`メソッドで、オーバーフローが発生する場合には`None`値を返す。
> * `overflowing_*`メソッドで、値と、オーバーフローが発生したかどうかを示す論理値を返す。
> * `saturating_*`メソッドで、値の最小値または最大値で飽和させる。（訳注: 結果が最大値を上回る場合は最大値に、最小値を下回る場合は最小値にするという意味です）

<!--
#### Floating-Point Types
-->

#### 浮動小数点型

<!--
Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust’s floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because on modern CPUs, it’s roughly the same speed as `f32` but is capable of
more precision. All floating-point types are signed.
-->

Rustにはさらに、*浮動小数点数*に対しても、2種類の基本型があり、浮動小数点数とは数値に小数点がついたもののことです。
Rustの浮動小数点型は、`f32`と`f64`で、それぞれ32ビットと64ビットサイズです。基準型は`f64`です。
なぜなら、現代のCPUでは、`f32`とほぼ同スピードにもかかわらず、より精度が高くなるからです。
すべての浮動小数点型は符号付きです。

<!--
Here’s an example that shows floating-point numbers in action:
-->

実際に動作している浮動小数点数の例をご覧ください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

<!--
Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, and `f64` has double precision.
-->

浮動小数点数は、IEEE-754規格に従って表現されています。`f32`が単精度浮動小数点数、
`f64`が倍精度浮動小数点数です。

<!--
#### Numeric Operations
-->

#### 数値演算

<!--
Rust supports the basic mathematical operations you’d expect for all the number
types: addition, subtraction, multiplication, division, and remainder. Integer
division truncates toward zero to the nearest integer. The following code shows
how you’d use each numeric operation in a `let` statement:
-->

Rustにも全数値型に期待されうる標準的な数学演算が用意されています: 足し算、引き算、掛け算、割り算、余りです。
整数の割り算では、0に近い方の最も近い整数に切り捨てられます。
以下の例では、`let`文での各数学演算の使用方法をご覧になれます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

<!--
Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. [Appendix
B][appendix_b] contains a list of all operators that Rust
provides.
-->

これらの文の各式は、数学演算子を使用しており、一つの値に評価され、そして、変数に束縛されます。
[付録B][appendix_b]にRustで使える演算子の一覧が載っています。

<!--
#### The Boolean Type
-->

#### 論理値型

<!--
As in most other programming languages, a Boolean type in Rust has two possible
values: `true` and `false`. Booleans are one byte in size. The Boolean type in
Rust is specified using `bool`. For example:
-->

他の多くの言語同様、Rustの論理値型も取りうる値は二つしかありません: `true`と`false`です。
論理値のサイズは1バイトです。
Rustの論理値型は、`bool`と指定されます。
例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

<!--
The main way to use Boolean values is through conditionals, such as an `if`
expression. We’ll cover how `if` expressions work in Rust in the [“Control
Flow”][control-flow] section.
-->

論理値を使う主な手段は、条件式です。例えば、`if`式などですね。`if`式のRustでの動作方法については、
[「制御フロー」][control-flow]節で講義します。

<!--
#### The Character Type
-->

#### 文字型

<!--
Rust’s `char` type is the language’s most primitive alphabetic type. Here are
some examples of declaring `char` values:
-->

Rustの`char`型は、言語の最も基本的なアルファベット型です。以下は`char`値を宣言するいくつかの例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

<!--
Note that we specify `char` literals with single quotes, as opposed to string
literals, which use double quotes. Rust’s `char` type is four bytes in size and
represents a Unicode Scalar Value, which means it can represent a lot more than
just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji;
and zero-width spaces are all valid `char` values in Rust. Unicode Scalar
Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.
However, a “character” isn’t really a concept in Unicode, so your human
intuition for what a “character” is may not match up with what a `char` is in
Rust. We’ll discuss this topic in detail in [“Storing UTF-8 Encoded Text with
Strings”][strings] in Chapter 8.
-->

`char`リテラルは、ダブルクォーテーションマークを使用する文字列リテラルに対して、シングルクォートで指定することに注意してください。
Rustの`char`型は4バイトのサイズを持ち、ユニコードのスカラー値を表します。これはつまり、アスキーよりもずっとたくさんのものを表せるということです。
アクセント記号付き文字; 中国語、日本語、韓国語の文字;
絵文字; ゼロ幅スペースはすべて、Rustでは有効な`char`値です。ユニコードスカラー値は、
`U+0000`から`U+D7FF`までと`U+E000`から`U+10FFFF`までの範囲になります。
ところが、「文字」は実はユニコードの概念ではないので、文字とは何かという人間としての直観は、
Rustにおける`char`値が何かとは合致しない可能性があります。この話題については、第8章の[「文字列でUTF-8でエンコードされたテキストを保持する」][strings]で詳しく議論しましょう。

<!--
### Compound Types
-->

### 複合型

<!--
*Compound types* can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.
-->

*複合型*により、複数の値を一つの型にまとめることができます。Rustには、
2種類の基本的な複合型があります: タプルと配列です。

<!--
#### The Tuple Type
-->

#### タプル型

<!--
A *tuple* is a general way of grouping together a number of values with a
variety of types into one compound type. Tuples have a fixed length: once
declared, they cannot grow or shrink in size.
-->

*タプル*は、様々な型の複数の値を一つの複合型にまとめ上げる汎用的な手段です。
タプルの長さは固定です: 一度宣言されたらサイズは伸縮できません。

<!--
We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:
-->

タプルは、丸かっこの中にカンマ区切りの値リストを書くことで生成します。タプルの位置ごとに型があり、
タプル内の値はそれぞれ全てが同じ型である必要はありません。今回の例では、型注釈をあえて追加しました:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

<!--
The variable `tup` binds to the entire tuple because a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:
-->

変数`tup`は、タプル全体に束縛されています。なぜなら、タプルは、一つの複合要素と考えられるからです。
タプルから個々の値を取り出すには、パターンマッチングを使用して分解することができます。以下のように:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

<!--
This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring* because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.
-->

このプログラムは、まずタプルを生成し、それを変数`tup`に束縛しています。
それから`let`とパターンを使って`tup`変数の中身を3つの個別の変数(`x`、`y`、`z`ですね)に変換しています。
この過程は、*分配*と呼ばれます。単独のタプルを破壊して三分割しているからです。最後に、
プログラムは`y`変数の値を出力し、`6.4`と表示されます。

<!--
We can also access a tuple element directly by using a period (`.`) followed by
the index of the value we want to access. For example:
-->

アクセスしたい値の番号をピリオド(`.`)に続けて書くことで、タプルの要素に直接アクセスすることもできます。例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

<!--
This program creates the tuple `x` and then accesses each element of the tuple
using their respective indices. As with most programming languages, the first
index in a tuple is 0.
-->

このプログラムは、新しいタプル`x`を作成し、タプルの各要素にそれぞれの添え字を使ってアクセスしています。
多くのプログラミング言語同様、タプルの最初の添え字は0です。

<!--
The tuple without any values has a special name, *unit*. This value and its
corresponding type are both written `()` and represent an empty value or an
empty return type. Expressions implicitly return the unit value if they don’t
return any other value.
-->

値をひとつも持たないタプルは*ユニット*という特別な名前を持っています。
この値と、それに対応する型はともに`()`と書き表され、空の値や空の戻り値型を表現します。
式は、特に値を返さなければ、暗黙的にユニット値を返します。

<!--
#### The Array Type
-->

#### 配列型

<!--
Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Unlike arrays in
some other languages, arrays in Rust have a fixed length.
-->

*配列*によっても、複数の値のコレクションを得ることができます。タプルと異なり、配列の全要素は、
同じ型でなければなりません。一部の他の言語の配列と異なり、Rustの配列は固定長です。

<!--
We write the values in an array as a comma-separated list inside square
brackets:
-->

配列内の要素は、角かっこ内にカンマ区切りリストとして記述します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

<!--
Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in [Chapter
4][stack-and-heap]) or when you want to ensure you always have a
fixed number of elements. An array isn’t as flexible as the vector type,
though. A *vector* is a similar collection type provided by the standard
library that *is* allowed to grow or shrink in size. If you’re unsure whether
to use an array or a vector, chances are you should use a vector. [Chapter
8][vectors] discusses vectors in more detail.
-->

配列は、ヒープよりもスタック(スタックとヒープについては[第4章][stack-and-heap]で<ruby>詳<rp>(</rp><rt>つまび</rt><rp>)</rp></ruby>らかに議論します)にデータのメモリを確保したい時、
または、常に固定長の要素があることを確認したい時に有効です。
ただ、配列は、ベクタ型ほど柔軟ではありません。*ベクタ*は、標準ライブラリによって提供されている配列と似たようなコレクション型で、
こちらは、サイズを伸縮させることが*できます*。配列とベクタ型、どちらを使うべきか確信が持てない時は、
おそらくベクタ型を使うべきです。[第8章][vectors]でベクタについて詳細に議論します。

<!--
However, arrays are more useful when you know the number of elements will not
need to change. For example, if you were using the names of the month in a
program, you would probably use an array rather than a vector because you know
it will always contain 12 elements:
-->

しかしながら、要素数を変えられる必要はないだろうと分かっている場合は、配列のほうが便利です。
例えば、プログラム中で月の名前を使おうとしているなら、おそらくベクタよりも配列を使うのが良いでしょう。
常に12個要素があることもわかってますからね:

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

ここでの`i32`は要素の型です。セミコロンのあとの`5`という数字は配列の要素が5つあることを表しています。

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

この`a`という名前の配列は`3`という値が5つあるものです。これは`let a = [3, 3, 3, 3, 3];`と書くのと同じですが、より簡潔になります。

<!--
##### Accessing Array Elements
-->

##### 配列の要素にアクセスする

<!--
An array is a single chunk of memory of a known, fixed size that can be 
allocated on the stack. You can access elements of an array using indexing,
like this:
-->

配列は、あらかじめ知られた固定サイズを持ち、スタック上に確保することができる一塊のメモリです。
添え字によって、配列の要素にこのようにアクセスすることができます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

<!--
In this example, the variable named `first` will get the value `1` because that
is the value at index `[0]` in the array. The variable named `second` will get
the value `2` from index `[1]` in the array.
-->

この例では、`first`という名前の変数には`1`という値が格納されます。配列の`[0]`番目にある値が、
それだからですね。`second`という名前の変数には、配列の`[1]`番目の値`2`が格納されます。

<!--
##### Invalid Array Element Access
-->

##### 配列要素への無効なアクセス

<!--
Let’s see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game in
Chapter 2, to get an array index from the user:
-->

配列の終端を越えて要素にアクセスしようとしたらどうなるか、見てみましょう。
第2章の数当てゲームと同じようにユーザから配列の添え字を受け取る、次のコードを実行する場合を考えてみてください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

<!--
This code compiles successfully. If you run this code using `cargo run` and
enter `0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding
value at that index in the array. If you instead enter a number past the end of
the array, such as `10`, you’ll see output like this:
-->

このコードはコンパイルされます。`cargo run`で走らせ、`0`, `1`, `2`, `3`, または`4`をこのプログラムに入力すると配列の対応する値を出力します。
もし配列の末尾を超えるような、例えば`10`などの数字を与えると、次のような出力が表示されます。

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
(スレッド'main'はsrc/main.rs:19:19でパニックしました:
範囲外アクセス: 長さは5ですが、添え字は10でした)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!--
The program resulted in a *runtime* error at the point of using an invalid
value in the indexing operation. The program exited with an error message and
didn’t execute the final `println!` statement. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than or equal to the length,
Rust will panic. This check has to happen at runtime, especially in this case,
because the compiler can’t possibly know what value a user will enter when they
run the code later.
-->

プログラムは、添え字アクセスで無効な値を使用した時点で*実行時*エラーに陥りました。
プログラムはエラーメッセージとともに終了し、最後の`println!`文を実行しませんでした。
要素に添え字アクセスを試みると、言語は、指定されたその添え字が配列長よりも小さいかを確認してくれます。
添え字が配列長と等しいかより大きければ、言語は*パニック*します。
この例の場合は特にそうですが、このチェックは実行時に行われなくてはなりません。
なぜならコンパイラは、ユーザが後でコードを実行したときに、ユーザがどんな値を入力するか知りようがないからです。

<!--
This is an example of Rust’s memory safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling and how you can
write readable, safe code that neither panics nor allows invalid memory access.
-->

これは、実際に稼働しているRustのメモリ安全機構の例のひとつになります。低レベル言語の多くでは、
この種のチェックは行われないため、間違った添え字を与えると、無効なメモリにアクセスできてしまいます。
Rustでは、メモリアクセスを許可し、処理を継続する代わりに即座にプログラムを終了することで、
この種のエラーからプログラマを保護しています。第9章ではRustのエラー処理について、そして、
可読性が高く安全で、パニックもしなければ不正なメモリアクセスも許さないコードをどうすれば書けるのか、もっと議論します。

<!--
[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
-->

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#予想と秘密の数字を比較する
[twos-complement]: https://ja.wikipedia.org/wiki/2の補数
[control-flow]: ch03-05-control-flow.html#制御フロー
[strings]: ch08-02-strings.html#文字列でutf-8でエンコードされたテキストを保持する
[stack-and-heap]: ch04-01-what-is-ownership.html#スタックとヒープ
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
