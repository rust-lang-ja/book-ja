<!--
## Unsafe Rust
-->

## Unsafe Rust

<!--
All the code we’ve discussed so far has had Rust’s memory safety guarantees
enforced at compile time. However, Rust has a second language hidden inside it
that doesn’t enforce these memory safety guarantees: it’s called *unsafe Rust*
and works just like regular Rust, but gives us extra superpowers.
-->

ここまでに議論してきたコードは全て、Rust のメモリ安全保証がコンパイル時に強制されていました。しかしながら、
Rust には、これらのメモリ安全保証を強制しない第 2 の言語が中に隠されています：それは*unsafe Rust*と呼ばれ、
普通の Rust のように動きますが、おまけの強大な力を与えてくれます。

<!--
Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler tries to determine whether or not code upholds the guarantees,
it’s better for it to reject some valid programs rather than accept some
invalid programs. Although the code might be okay, as far as Rust is able to
tell, it’s not! In these cases, you can use unsafe code to tell the compiler,
“Trust me, I know what I’m doing.” The downside is that you use it at our own
risk: if you use unsafe code incorrectly, problems due to memory unsafety, such
as null pointer dereferencing, can occur.
-->

静的解析は原理的に保守的なので、unsafe Rust が存在します。コードが保証を保持しているかコンパイラが決定しようとする際、
なんらかの不正なプログラムを受け入れるよりも合法なプログラムを拒否したほうがいいのです。コードは大丈夫かもしれないけれど、
コンパイラにわかる範囲ではダメなのです！このような場合、unsafe コードを使用してコンパイラに「信じて！何をしているかわかってるよ」と教えられます。
欠点は、自らのリスクで使用することです：unsafe コードを誤って使用したら、
null ポインタ参照外しなどのメモリ非安全に起因する問題が起こることもあるのです。

<!--
Another reason Rust has an unsafe alter ego is that the underlying computer
hardware is inherently unsafe. If Rust didn’t let you do unsafe operations, you
couldn’t do certain tasks. Rust needs to allow you to do low-level systems
programming, such as directly interacting with the operating system or even
writing your own operating system. Working with low-level systems programming
is one of the goals of the language. Let’s explore what we can do with unsafe
Rust and how to do it.
-->

Rust に unsafe な分身がある別の理由は、根本にあるコンピュータのハードウェアが本質的に unsafe だからです。
Rust が unsafe な処理を行わせてくれなかったら、特定の仕事を行えないでしょう。Rust は、低レベルなシステムプログラミングを許可する必要があります。
直接 OS と相互作用したり、独自の OS を書くことさえもそうです。低レベルなシステムプログラミングに取り組むことは、
言語の目標の 1 つなのです。unsafe Rust でできることとその方法を探究しましょう。

<!--
### Unsafe Superpowers
-->

### unsafe の強大な力 (superpower)

<!--
To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take four actions in unsafe Rust, called
*unsafe superpowers*, that you can’t in safe Rust. Those superpowers include
the ability to:
-->

unsafe Rust に切り替えるには、`unsafe`キーワードを使用し、それから unsafe コードを保持する新しいブロックを開始してください。
safe Rust では行えない 4 つの行動を unsafe Rust では行え、これは*unsafe superpowers*と呼ばれます。
その superpower には、以下の能力が含まれています：

<!--
* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
-->

* 生ポインタを参照外しすること
* unsafe な関数やメソッドを呼ぶこと
* 可変で静的な変数にアクセスしたり変更すること
* unsafe なトレイトを実装すること

<!--
It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks: if you use a reference in unsafe
code, it will still be checked. The `unsafe` keyword only gives you access to
these four features that are then not checked by the compiler for memory
safety. You'll still get some degree of safety inside of an unsafe block.
-->

`unsafe`は、借用チェッカーや他の Rust の安全性チェックを無効にしないことを理解するのは重要なことです：
unsafe コードで参照を使用しても、チェックはされます。`unsafe`キーワードにより、これら 4 つの機能にアクセスできるようになり、
その場合、コンパイラによってこれらのメモリ安全性は確認されないのです。unsafe ブロック内でも、ある程度の安全性は得られます。

<!--
In addition, `unsafe` does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: the intent is
that as the programmer, you’ll ensure the code inside an `unsafe` block will
access memory in a valid way.
-->

また、unsafe は、そのブロックが必ずしも危険だったり、絶対メモリ安全上の問題を抱えていることを意味するものではありません：
その意図は、プログラマとして`unsafe`ブロック内のコードがメモリに合法的にアクセスすることを保証することです。

<!--
People are fallible, and mistakes will happen, but by requiring these four
unsafe operations to be inside blocks annotated with `unsafe` you’ll know that
any errors related to memory safety must be within an `unsafe` block. Keep
`unsafe` blocks small; you’ll be thankful later when you investigate memory
bugs.
-->

人間は失敗をするもので、間違いも起きますが、これら 4 つの unsafe な処理を`unsafe`で注釈されたブロックに入れる必要があることで、
メモリ安全性に関するどんなエラーも`unsafe`ブロック内にあるに違いないと知ります。`unsafe`ブロックは小さくしてください;
メモリのバグを調査するときに感謝することになるでしょう。

<!--
To isolate unsafe code as much as possible, it’s best to enclose unsafe code
within a safe abstraction and provide a safe API, which we’ll discuss later in
the chapter when we examine unsafe functions and methods. Parts of the standard
library are implemented as safe abstractions over unsafe code that has been
audited. Wrapping unsafe code in a safe abstraction prevents uses of `unsafe`
from leaking out into all the places that you or your users might want to use
the functionality implemented with `unsafe` code, because using a safe
abstraction is safe.
-->

unsafe なコードをできるだけ分離するために、unsafe なコードを安全な抽象の中に閉じ込め、安全な API を提供するのが最善です。
これについては、後ほど unsafe な関数とメソッドを調査する際に議論します。標準ライブラリの一部は、
検査された unsafe コードの安全な抽象として実装されています。安全な抽象に unsafe なコードを包むことで、
`unsafe`が、あなたやあなたのユーザが`unsafe`コードで実装された機能を使いたがる可能性のある箇所全部に漏れ出ることを防ぎます。
安全な抽象を使用することは、安全だからです。

<!--
Let’s look at each of the four unsafe superpowers in turn. We’ll also look at
some abstractions that provide a safe interface to unsafe code.
-->

4 つの unsafe な superpower を順に見ていきましょう。unsafe なコードへの安全なインターフェイスを提供する一部の抽象化にも目を向けます。

<!--
### Dereferencing a Raw Pointer
-->

### 生ポインタを参照外しする

<!--
In Chapter 4, in the “Dangling References” section, we mentioned that the
compiler ensures references are always valid. Unsafe Rust has two new types
called *raw pointers* that are similar to references. As with references, raw
pointers can be immutable or mutable and are written as `*const T` and `*mut
T`, respectively. The asterisk isn’t the dereference operator; it’s part of the
type name. In the context of raw pointers, *immutable* means that the pointer
can’t be directly assigned to after being dereferenced.
-->

第 4 章の「ダングリング参照」節で、コンパイラは、参照が常に有効であることを保証することに触れました。
unsafe Rust には参照に類似した*生ポインタ*と呼ばれる 2 つの新しい型があります。参照同様、
生ポインタも不変や可変になり得て、それぞれ`*const T`と`*mut T`と表記されます。このアスタリスクは、参照外し演算子ではありません;
型名の一部です。生ポインタの文脈では、*不変*は、参照外し後に直接ポインタに代入できないことを意味します。

<!--
Different from references and smart pointers, raw pointers:
-->

参照やスマートポインタと異なり、生ポインタは：

<!--
* Are allowed to ignore the borrowing rules by having both immutable and
mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup
-->

* 同じ場所への不変と可変なポインタや複数の可変なポインタが存在することで借用規則を無視できる
* 有効なメモリを指しているとは保証されない
* null の可能性がある
* 自動的な片付けは実装されていない

<!--
By opting out of having Rust enforce these guarantees, you can give up
the guaranteed safety in exchange for greater performance or the ability to
interface with another language or hardware where Rust’s guarantees don’t apply.
-->

これらの保証をコンパイラに強制させることから抜けることで、保証された安全性を諦めてパフォーマンスを向上させたり、
Rust の保証が適用されない他の言語やハードウェアとのインターフェイスの能力を得ることができます。

<!--
Listing 19-1 shows how to create an immutable and a mutable raw pointer from
references.
-->

リスト 19-1 は、参照から不変と可変な生ポインタを生成する方法を示しています。

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

<!--
<span class="caption">Listing 19-1: Creating raw pointers from references</span>
-->

<span class="caption">リスト 19-1: 参照から生ポインタを生成する</span>

<!--
Notice that we don’t include the `unsafe` keyword in this code. We can create
raw pointers in safe code; we just can’t dereference raw pointers outside an
unsafe block, as you’ll see in a bit.
-->

このコードには`unsafe`キーワードを含めていないことに注意してください。safe コードで生ポインタを生成できます;
もうすぐわかるように、unsafe ブロックの外では、生ポインタを参照外しできないだけなのです。

<!--
We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types. Because we created them
directly from references guaranteed to be valid, we know these particular raw
pointers are valid, but we can’t make that assumption about just any raw
pointer.
-->

`as`を使って不変と可変な参照を対応する生ポインタの型にキャストして生ポインタを生成しました。
有効であることが保証される参照から直接生ポインタを生成したので、これらの特定の生ポインタは有効であることがわかりますが、
その前提をあらゆる生ポインタに敷くことはできません。

<!--
Next, we’ll create a raw pointer whose validity we can’t be so certain of.
Listing 19-2 shows how to create a raw pointer to an arbitrary location in
memory. Trying to use arbitrary memory is undefined: there might be data at
that address or there might not, the compiler might optimize the code so there
is no memory access, or the program might error with a segmentation fault.
Usually, there is no good reason to write code like this, but it is possible.
-->

次に、有効であることが確信できない生ポインタを生成します。リスト 19-2 は、メモリの任意の箇所を指す生ポインタの生成法を示しています。
任意のメモリを使用しようとすることは未定義です：そのアドレスにデータがある可能性もあるし、ない可能性もあり、
コンパイラがコードを最適化してメモリアクセスがなくなる可能性もあるし、プログラムがセグメンテーションフォールトでエラーになる可能性もあります。
通常、このようなコードを書くいい理由はありませんが、可能ではあります。

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

<!--
<span class="caption">Listing 19-2: Creating a raw pointer to an arbitrary
memory address</span>
-->

<span class="caption">リスト 19-2: 任意のメモリアドレスへの生ポインタを生成する</span>

<!--
Recall that we can create raw pointers in safe code, but we can’t *dereference*
raw pointers and read the data being pointed to. In Listing 19-3, we use the
dereference operator `*` on a raw pointer that requires an `unsafe` block.
-->

safe コードで生ポインタを生成できるけれども、生ポインタを*参照外し*して指しているデータを読むことはできないことを思い出してください。
リスト 19-3 では、`unsafe`ブロックが必要になる参照外し演算子の`*`を生ポインタに使っています。

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

<!--
<span class="caption">Listing 19-3: Dereferencing raw pointers within an
`unsafe` block</span>
-->

<span class="caption">リスト 19-3: `unsafe`ブロック内で生ポインタを参照外しする</span>

<!--
Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value.
-->

ポインタの生成は害を及ぼしません; 問題が起こり得るのはポインタが指している値にアクセスしようとするときのみで、この際に無効な値を扱うことになる可能性があります。

<!--
Note also that in Listing 19-1 and 19-3, we created `*const i32` and `*mut i32`
raw pointers that both pointed to the same memory location, where `num` is
stored. If we instead tried to create an immutable and a mutable reference to
`num`, the code would not have compiled because Rust’s ownership rules don’t
allow a mutable reference at the same time as any immutable references. With
raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race. Be careful!
-->

また、リスト 19-1 とリスト 19-3 では、
`num`が格納されている同じメモリ上の場所を両方とも指す`*const i32`と`*mut i32`の生ポインタを生成したことに注目してください。
代わりに`num`への不変と可変な参照を生成しようとしたら、コードはコンパイルできなかったでしょう。
Rust の所有権規則により、不変参照と可変参照を同時に存在させられないからです。生ポインタなら、
同じ場所への可変なポインタと不変なポインタを生成でき、可変なポインタを通してデータを変更し、データ競合を引き起こす可能性があります。
気を付けてください！

<!--
With all of these dangers, why would you ever use raw pointers? One major use
case is when interfacing with C code, as you’ll see in the next section,
“Calling an Unsafe Function or Method.” Another case is when building up safe
abstractions that the borrow checker doesn’t understand. We’ll introduce unsafe
functions and then look at an example of a safe abstraction that uses unsafe
code.
-->

これらの危険がありながら、一体何故生ポインタを使うのでしょうか？主なユースケースの 1 つは、次の節「unsafe な関数やメソッドを呼ぶ」で見るように、
C コードとのインターフェイスです。別のユースケースは、借用チェッカーには理解できない安全な抽象を構成する時です。
unsafe な関数を導入し、それから unsafe コードを使用する安全な抽象の例に目を向けます。

<!--
### Calling an Unsafe Function or Method
-->

### unsafe な関数やメソッドを呼ぶ

<!--
The second type of operation that requires an unsafe block is calls to unsafe
functions. Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` before the rest of the definition. The
`unsafe` keyword in this context indicates the function has requirements we
need to uphold when we call this function, because Rust can’t guarantee we’ve
met these requirements. By calling an unsafe function within an `unsafe` block,
we’re saying that we’ve read this function’s documentation and take
responsibility for upholding the function’s contracts.
-->

unsafe ブロックが必要になる 2 番目の処理は、unsafe 関数の呼び出しです。unsafe な関数やメソッドも見た目は、
普通の関数やメソッドと全く同じですが、残りの定義の前に追加の`unsafe`があります。この文脈での`unsafe`キーワードは、
この関数を呼ぶ際に保持しておく必要のある要求が関数にあることを示唆します。コンパイラには、
この要求を満たしているか保証できないからです。`unsafe`ブロックで unsafe な関数を呼び出すことで、
この関数のドキュメンテーションを読み、関数の契約を守っているという責任を取ると宣言します。

<!--
Here is an unsafe function named `dangerous` that doesn’t do anything in its
body:
-->

こちらは、本体で何もしない`dangerous`という unsafe な関数です：

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

<!--
We must call the `dangerous` function within a separate `unsafe` block. If we
try to call `dangerous` without the `unsafe` block, we’ll get an error:
-->

個別の`unsafe`ブロックで`dangerous`関数を呼ばなければなりません。`unsafe`ブロックなしで`dangerous`を呼ぼうとすれば、
エラーになるでしょう：

```text
error[E0133]: call to unsafe function requires unsafe function or block
(エラー: unsafe 関数の呼び出しには、unsafe な関数かブロックが必要です)
 -->
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
```

<!--
By inserting the `unsafe` block around our call to `dangerous`, we’re asserting
to Rust that we’ve read the function’s documentation, we understand how to use
it properly, and we’ve verified that we’re fulfilling the contract of the
function.
-->

`dangerous`への呼び出しの周りに`unsafe`ブロックを挿入することで、コンパイラに関数のドキュメンテーションを読み、
適切に使用する方法を理解したことをアサートし、関数の契約を満たしていると実証しました。

<!--
Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other
unsafe operations within an unsafe function, we don’t need to add another
`unsafe` block.
-->

unsafe 関数の本体は、実効的に`unsafe`ブロックになるので、unsafe 関数内で unsafe な別の処理を行うのに、
別の`unsafe`ブロックは必要ないのです。

<!--
#### Creating a Safe Abstraction over Unsafe Code
-->

#### unsafe コードに安全な抽象を行う

<!--
Just because a function contains unsafe code doesn’t mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is
a common abstraction. As an example, let’s study a function from the standard
library, `split_at_mut`, that requires some unsafe code and explore how we
might implement it. This safe method is defined on mutable slices: it takes one
slice and makes it two by splitting the slice at the index given as an
argument. Listing 19-4 shows how to use `split_at_mut`.
-->

関数が unsafe なコードを含んでいるだけで関数全体を unsafe でマークする必要があることにはなりません。
事実、安全な関数で unsafe なコードをラップすることは一般的な抽象化です。例として、
なんらかの unsafe コードが必要になる標準ライブラリの関数`split_at_mut`を学び、その実装方法を探究しましょう。
この安全なメソッドは、可変なスライスに定義されています：スライスを 1 つ取り、引数で与えられた添え字でスライスを分割して 2 つにします。
リスト 19-4 は、`split_at_mut`の使用法を示しています。

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

<!--
<span class="caption">Listing 19-4: Using the safe `split_at_mut`
function</span>
-->

<span class="caption">リスト 19-4: 安全な`split_at_mut`関数を使用する</span>

<!--
We can’t implement this function using only safe Rust. An attempt might look
something like Listing 19-5, which won’t compile. For simplicity, we’ll
implement `split_at_mut` as a function rather than a method and only for slices
of `i32` values rather than for a generic type `T`.
-->

この関数を safe Rust だけを使用して実装することはできません。試みは、リスト 19-5 のようになる可能性がありますが、コンパイルできません。
簡単のため、`split_at_mut`をメソッドではなく関数として実装し、ジェネリックな型`T`ではなく、`i32`のスライス用に実装します。

```rust,ignore
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```

<!--
<span class="caption">Listing 19-5: An attempted implementation of
`split_at_mut` using only safe Rust</span>
-->

<span class="caption">リスト 19-5: safe Rust だけを使用した`split_at_mut`の未遂の実装</span>

<!--
This function first gets the total length of the slice. Then it asserts that
the index given as a parameter is within the slice by checking whether it’s
less than or equal to the length. The assertion means that if we pass an index
that is greater than the index to split the slice at, the function will panic
before it attempts to use that index.
-->

この関数はまず、スライスの全体の長さを得ます。それから引数で与えられた添え字が長さ以下であるかを確認してスライス内にあることをアサートします。
このアサートは、スライスを分割する添え字よりも大きい添え字を渡したら、その添え字を使用しようとする前に関数がパニックすることを意味します。

<!--
Then we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index and another from `mid` to the end of the
slice.
-->

そして、2 つの可変なスライスをタプルで返します：1 つは元のスライスの最初から`mid`添え字まで、
もう一方は、`mid`からスライスの終わりまでです。

<!--
When we try to compile the code in Listing 19-5, we’ll get an error.
-->

リスト 19-5 のコードのコンパイルを試みると、エラーになるでしょう。

```text
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
(エラー: 一度に 2 回以上、`*slice`を可変で借用できません)
 -->
  |
6 |     (&mut slice[..mid],
  |           ----- first mutable borrow occurs here
7 |      &mut slice[mid..])
  |           ^^^^^ second mutable borrow occurs here
8 | }
  | - first borrow ends here
```

<!--
Rust’s borrow checker can’t understand that we’re borrowing different parts of
the slice; it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two
slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
-->

Rust の借用チェッカーには、スライスの異なる部分を借用していることが理解できないのです; 
同じスライスから 2 回借用していることだけ知っています。2 つのスライスが被らないので、
スライスの異なる部分を借用することは、根本的に大丈夫なのですが、コンパイラはこれを知れるほど賢くありません。
プログラマにはコードが大丈夫とわかるのに、コンパイラにはわからないのなら、unsafe コードに手を伸ばすタイミングです。

<!--
Listing 19-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work.
-->

リスト 19-6 は`unsafe`ブロック、生ポインタ、unsafe 関数への呼び出しをして`split_at_mut`の実装が動くようにする方法を示しています。

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

<!--
<span class="caption">Listing 19-6: Using unsafe code in the implementation of
the `split_at_mut` function</span>
-->

<span class="caption">リスト 19-6: `split_at_mut`関数の実装で unsafe コードを使用する</span>

<!--
Recall from “The Slice Type” section in Chapter 4 that slices are a pointer to
some data and the length of the slice. We use the `len` method to get the
length of a slice and the `as_mut_ptr` method to access the raw pointer of a
slice. In this case, because we have a mutable slice to `i32` values,
`as_mut_ptr` returns a raw pointer with the type `*mut i32`, which we’ve stored
in the variable `ptr`.
-->

第 4 章の「スライス型」節から、スライスはなんらかのデータへのポインタとスライスの長さであることを思い出してください。
`len`メソッドを使用してスライスの長さを得て、`as_mut_ptr`メソッドを使用してスライスの生ポインタにアクセスしています。
この場合、`i32`値の可変スライスがあるので、`as_mut_ptr`は型`*mut i32`の生ポインタを返し、これを変数`ptr`に格納しました。

<!--
We keep the assertion that the `mid` index is within the slice. Then we get to
the unsafe code: the `slice::from_raw_parts_mut` function takes a raw pointer
and a length, and it creates a slice. We use this function to create a slice
that starts from `ptr` and is `mid` items long. Then we call the `offset`
method on `ptr` with `mid` as an argument to get a raw pointer that starts at
`mid`, and we create a slice using that pointer and the remaining number of
items after `mid` as the length.
-->

`mid`添え字がスライス内にあるかというアサートを残しています。そして、unsafe コードに到達します：
`slice::from_raw_parts_mut`関数は、生ポインタと長さを取り、スライスを生成します。この関数を使って、
`ptr`から始まり、`mid`の長さのスライスを生成しています。それから`ptr`に`mid`を引数として`offset`メソッドを呼び出し、
`mid`で始まる生ポインタを得て、そのポインタと`mid`の後の残りの要素数を長さとして使用してスライスを生成しています。

<!--
The function `slice::from_raw_parts_mut` is unsafe because it takes a raw
pointer and must trust that this pointer is valid. The `offset` method on raw
pointers is also unsafe, because it must trust that the offset location is also
a valid pointer. Therefore, we had to put an `unsafe` block around our calls to
`slice::from_raw_parts_mut` and `offset` so we could call them. By looking at
the code and by adding the assertion that `mid` must be less than or equal to
`len`, we can tell that all the raw pointers used within the `unsafe` block
will be valid pointers to data within the slice. This is an acceptable and
appropriate use of `unsafe`.
-->

関数`slice::from_raw_parts_mut`は、unsafe です。何故なら、生ポインタを取り、このポインタが有効であることを信用しなければならないからです。
生ポインタの`offset`メソッドも unsafe です。オフセット位置もまた有効なポインタであることを信用しなければならないからです。
故に、`slice::from_raw_parts_mut`と`offset`を呼べるように、その呼び出しの周りに`unsafe`ブロックを置かなければならなかったのです。
コードを眺めて`mid`が`len`以下でなければならないとするアサートを追加することで、
`unsafe`ブロック内で使用されている生ポインタが全てスライス内のデータへの有効なポインタであることがわかります。
これは、受け入れられ、適切な`unsafe`の使用法です。

<!--
Note that we don’t need to mark the resulting `split_at_mut` function as
`unsafe`, and we can call this function from safe Rust. We’ve created a safe
abstraction to the unsafe code with an implementation of the function that uses
`unsafe` code in a safe way, because it creates only valid pointers from the
data this function has access to.
-->

できあがった`split_at_mut`関数を`unsafe`でマークする必要はなく、この関数を safe Rust から呼び出せることに注意してください。
`unsafe`コードを安全に使用する関数の実装で、unsafe コードへの安全な抽象化を行いました。
この関数がアクセスするデータからの有効なポインタだけを生成するからです。

<!--
In contrast, the use of `slice::from_raw_parts_mut` in Listing 19-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice 10,000 items long:
-->

対照的に、リスト 19-7 の`slice::from_raw_parts_mut`の使用は、スライスが使用されるとクラッシュする可能性が高いでしょう。
このコードは任意のメモリアドレスを取り、10,000 要素の長さのスライスを生成します：

```rust
use std::slice;

let address = 0x012345usize;
let r = address as *mut i32;

let slice = unsafe {
    slice::from_raw_parts_mut(r, 10000)
};
```

<!--
<span class="caption">Listing 19-7: Creating a slice from an arbitrary memory
location</span>
-->

<span class="caption">リスト 19-7: 任意のメモリアドレスからスライスを生成する</span>

<!--
We don’t own the memory at this arbitrary location, and there is no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`slice` as though it’s a valid slice results in undefined behavior.
-->

この任意の場所のメモリは所有していなく、このコードが生成するスライスに有効な`i32`値が含まれる保証もありません。
`slice`を有効なスライスであるかのように使用しようとすると、未定義動作に陥ります。

<!--
#### Using `extern` Functions to Call External Code
-->

#### `extern`関数を使用して、外部のコードを呼び出す

<!--
Sometimes, your Rust code might need to interact with code written in another
language. For this, Rust has a keyword, `extern`, that facilitates the creation
and use of a *Foreign Function Interface (FFI)*. An FFI is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.
-->

時として、自分の Rust コードが他の言語で書かれたコードと相互作用する必要が出てくる可能性があります。このために、
Rust には`extern`というキーワードがあり、これは、
*FFI*(Foreign Function Interface: 外部関数インターフェイス) の生成と使用を容易にします。
FFI は、あるプログラミング言語に関数を定義させ、異なる (外部の) プログラミング言語にそれらの関数を呼び出すことを可能にする方法です

<!--
Listing 19-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
always unsafe to call from Rust code. The reason is that other languages don’t
enforce Rust’s rules and guarantees, and Rust can’t check them, so
responsibility falls on the programmer to ensure safety.
-->

リスト 19-8 は、C の標準ライブラリから`abs`関数を統合するセットアップ方法をデモしています。
`extern`ブロック内で宣言された関数は、常に Rust コードから呼ぶには unsafe になります。理由は、
他の言語では、Rust の規則や保証が強制されず、コンパイラもチェックできないので、
安全性を保証する責任はプログラマに降りかかるのです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // -3 の絶対値は、C によると{}
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

<!--
<span class="caption">Listing 19-8: Declaring and calling an `extern` function
defined in another language</span>
-->

<span class="caption">リスト 19-8: 他の言語で定義された`extern`関数を宣言し、呼び出す</span>

<!--
Within the `extern "C"` block, we list the names and signatures of external
functions from another language we want to call. The `"C"` part defines which
*application binary interface (ABI)* the external function uses: the ABI
defines how to call the function at the assembly level. The `"C"` ABI is the
most common and follows the C programming language’s ABI.
-->

`extern "C"`ブロック内で他の言語から呼び出した関数の名前とシグニチャを列挙します。`"C"`の部分は、
外部関数がどの*ABI*(application binary interface: アプリケーション・バイナリ・インターフェイス) を使用しているか定義します：
ABI は関数の呼び出し方法をアセンブリレベルで定義します。`"C"`ABI は最も一般的で C プログラミング言語の ABI に従っています。

<!--
> #### Calling Rust Functions from Other Languages
>
> We can also use `extern` to create an interface that allows other languages
> to call Rust functions. Instead of an `extern` block, we add the `extern`
> keyword and specify the ABI to use just before the `fn` keyword. We also need
> to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle
> the name of this function. *Mangling* is when a compiler changes the name
> we’ve given a function to a different name that contains more information for
> other parts of the compilation process to consume but is less human readable.
> Every programming language compiler mangles names slightly differently, so
> for a Rust function to be nameable by other languages, we must disable the
> Rust compiler’s name mangling.
>
> In the following example, we make the `call_from_c` function accessible from
> C code, after it’s compiled to a shared library and linked from C:
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> This usage of `extern` does not require `unsafe`.
-->

> #### 他の言語から Rust の関数を呼び出す
>
> また、`extern`を使用して他の言語に Rust の関数を呼ばせるインターフェイスを生成することもできます。
> `extern`ブロックの代わりに、`extern`キーワードを追加し、`fn`キーワードの直前に使用する ABI を指定します。
> さらに、`#[no_mangle]`注釈を追加して Rust コンパイラに関数名をマングルしないように指示する必要もあります。
> *マングル*とは、コンパイラが関数に与えた名前を他のコンパイル過程の情報をより多く含むけれども、人間に読みにくい異なる名前にすることです。
> 全ての言語のコンパイラは、少々異なる方法でマングルを行うので、Rust の関数が他の言語で名前付けできるように、
> Rust コンパイラの名前マングルをオフにしなければならないのです。
>
> 以下の例では、共有ライブラリにコンパイルし、C からリンクした後に`call_from_c`関数を C コードからアクセスできるようにしています：
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>	  // C から Rust 関数を呼び出したばかり！
>     println!("Just called a Rust function from C!");
> }
> ``` 
>
> この`extern`の使用法では、`unsafe`は必要ありません。

<!--
### Accessing or Modifying a Mutable Static Variable
-->

### 可変で静的な変数にアクセスしたり、変更する

<!--
Until now, we’ve not talked about *global variables*, which Rust does support
but can be problematic with Rust’s ownership rules. If two threads are
accessing the same mutable global variable, it can cause a data race.
-->

今までずっと、*グローバル変数*について語りませんでした。グローバル変数を Rust は確かにサポートしていますが、
Rust の所有権規則で問題になることもあります。2 つのスレッドが同じ可変なグローバル変数にアクセスしていたら、
データ競合を起こすこともあります。

<!--
In Rust, global variables are called *static* variables. Listing 19-9 shows an
example declaration and use of a static variable with a string slice as a
value.
-->

Rust では、グローバル変数は、*static*(静的) 変数と呼ばれます。リスト 19-9 は、
値として文字列スライスのある静的変数の宣言例と使用を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
	// 名前は：{}
    println!("name is: {}", HELLO_WORLD);
}
```

<!--
<span class="caption">Listing 19-9: Defining and using an immutable static
variable</span>
-->

<span class="caption">リスト 19-9: 不変で静的な変数を定義し、使用する</span>

<!--
Static variables are similar to constants, which we discussed in the
“Differences Between Variables and Constants” section in Chapter 3. The names
of static variables are in `SCREAMING_SNAKE_CASE` by convention, and we *must*
annotate the variable’s type, which is `&'static str` in this example. Static
variables can only store references with the `'static` lifetime, which means
the Rust compiler can figure out the lifetime; we don’t need to annotate it
explicitly. Accessing an immutable static variable is safe.
-->

静的変数は、定数に似ています。定数については、第 3 章の「変数と定数の違い」節で議論しました。
静的変数の名前は慣習で`SCREAMING_SNAKE_CASE`(`直訳`: 叫ぶスネークケース) になり、変数の型を注釈し*なければなりません*。
この例では`&'static str`です。静的変数は、`'static`ライフタイムの参照のみ格納でき、
これは、Rust コンパイラがライフタイムを推量できることを意味します; 明示的に注釈する必要はありません。
不変で静的な変数にアクセスすることは安全です。

<!--
Constants and immutable static variables might seem similar, but a subtle
difference is that values in a static variable have a fixed address in memory.
Using the value will always access the same data. Constants, on the other hand,
are allowed to duplicate their data whenever they’re used.
-->

定数と不変で静的な変数は、類似して見える可能性がありますが、微妙な差異は、
静的変数の値は固定されたメモリアドレスになることです。値を使用すると、常に同じデータにアクセスします。
一方、定数は使用される度にデータを複製させることができます。

<!--
Another difference between constants and static variables is that static
variables can be mutable. Accessing and modifying mutable static variables is
*unsafe*. Listing 19-10 shows how to declare, access, and modify a mutable
static variable named `COUNTER`.
-->

定数と静的変数の別の違いは、静的変数は可変にもなることです。可変で静的な変数にアクセスし変更することは、unsafe です。
リスト 19-10 は、`COUNTER`という可変で静的な変数を宣言し、アクセスし、変更する方法を表示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

<!--
<span class="caption">Listing 19-10: Reading from or writing to a mutable
static variable is unsafe</span>
-->

<span class="caption">リスト 19-10: 可変で静的な変数を読んだり、書き込むのは unsafe である</span>

<!--
As with regular variables, we specify mutability using the `mut` keyword. Any
code that reads or writes from `COUNTER` must be within an `unsafe` block. This
code compiles and prints `COUNTER: 3` as we would expect because it’s single
threaded. Having multiple threads access `COUNTER` would likely result in data
races.
-->

普通の変数同様、`mut`キーワードを使用して可変性を指定します。`COUNTER`を読み書きするコードはどれも、`unsafe`ブロックになければなりません。
シングルスレッドなので、このコードは想定通り、コンパイルでき、`COUNTER: 3`と出力します。
複数のスレッドに`COUNTER`にアクセスさせると、データ競合になる可能性が高いでしょう。

<!--
With mutable data that is globally accessible, it’s difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe. Where possible, it’s preferable to use the concurrency techniques and
thread-safe smart pointers we discussed in Chapter 16 so the compiler checks
that data accessed from different threads is done safely.
-->

グローバルにアクセス可能な可変なデータがあると、データ競合がないことを保証するのは難しくなり、そのため、
Rust は可変で静的な変数を unsafe と考えるのです。可能なら、コンパイラが、データが異なるスレッドからアクセスされることが安全に行われているかを確認するように、
第 16 章で議論した並行性テクニックとスレッド安全なスマートポインタを使用するのが望ましいです。

<!--
### Implementing an Unsafe Trait
-->

### unsafe なトレイトを実装する

<!--
The final action that works only with `unsafe` is implementing an unsafe trait.
A trait is unsafe when at least one of its methods has some invariant that the
compiler can’t verify. We can declare that a trait is `unsafe` by adding the
`unsafe` keyword before `trait` and marking the implementation of the trait as
`unsafe` too, as shown in Listing 19-11.
-->

`unsafe`でのみ動く最後の行動は、unsafe なトレイトを実装することです。少なくとも、1 つのメソッドにコンパイラが確かめられないなんらかの不変条件があると、
トレイトは unsafe になります。`trait`の前に`unsafe`キーワードを追加し、トレイトの実装も`unsafe`でマークすることで、
トレイトが`unsafe`であると宣言できます。リスト 19-11 のようにですね。

```rust
unsafe trait Foo {
    // methods go here
    // メソッドがここに来る
}

unsafe impl Foo for i32 {
    // method implementations go here
    // メソッドの実装がここに来る
}
```

<!--
<span class="caption">Listing 19-11: Defining and implementing an unsafe
trait</span>
-->

<span class="caption">リスト 19-11: unsafe なトレイトを定義して実装する</span>

<!--
By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can’t verify.
-->

`unsafe impl`を使用することで、コンパイラが確かめられない不変条件を守ることを約束しています。

<!--
As an example, recall the `Sync` and `Send` marker traits we discussed in the
“Extensible Concurrency with the `Sync` and `Send` Traits” section in Chapter
16: the compiler implements these traits automatically if our types are
composed entirely of `Send` and `Sync` types. If we implement a type that
contains a type that is not `Send` or `Sync`, such as raw pointers, and we want
to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust can’t verify
that our type upholds the guarantees that it can be safely sent across threads
or accessed from multiple threads; therefore, we need to do those checks
manually and indicate as such with `unsafe`.
-->

例として、第 16 章の「`Sync`と`Send`トレイトで拡張可能な並行性」節で議論した`Sync`と`Send`マーカートレイトを思い出してください：
型が完全に`Send`と`Sync`型だけで構成されていたら、コンパイラはこれらのトレイトを自動的に実装します。
生ポインタなどの`Send`や`Sync`でない型を含む型を実装し、その型を`Send`や`Sync`でマークしたいなら、
`unsafe`を使用しなければなりません。コンパイラは、型がスレッド間を安全に送信できたり、
複数のスレッドから安全にアクセスできるという保証を保持しているか確かめられません; 故に、そのチェックを手動で行い、
`unsafe`でそのように示唆する必要があります。

<!--
### When to Use Unsafe Code
-->

### いつ unsafe コードを使用するべきか

<!--
Using `unsafe` to take one of the four actions (superpowers) just discussed
isn’t wrong or even frowned upon. But it is trickier to get `unsafe` code
correct because the compiler can’t help uphold memory safety. When you have a
reason to use `unsafe` code, you can do so, and having the explicit `unsafe`
annotation makes it easier to track down the source of problems if they occur.
-->

`unsafe`を使って議論したばかりの 4 つの行動 (強大な力) のうちの 1 つを行うのは間違っていたり、認められさえもしないものではありません。
ですが、`unsafe`コードを正しくするのは、より巧妙なことでしょう。コンパイラがメモリ安全性を保持する手助けをできないからです。
`unsafe`コードを使用する理由があるなら、そうすることができ、明示的に`unsafe`注釈をすることで問題が起きたら、
その原因を追求するのが容易になります。
