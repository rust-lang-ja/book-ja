<!--
# Smart Pointers
-->

# スマートポインタ

<!--
A *pointer* is a general concept for a variable that contains an address in
memory. This address refers to, or “points at,” some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don’t have any special capabilities other than referring to
data, and have no overhead.
-->

*ポインタ*は、メモリのアドレスを含む変数の一般的な概念です。このアドレスは、何らかの他のデータを参照、または「指します」。
Rustにおいて最もありふれた種類のポインタは参照です。参照については第4章で習いましたね。参照は`&`記号で示唆され、指している値を借用します。
データを参照すること以外に特別な能力は何もなく、オーバーヘッドもありません。

<!--
*Smart pointers*, on the other hand, are data structures that act like a
pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. Rust has a variety of smart pointers defined in the
standard library that provide functionality beyond that provided by references.
To explore the general concept, we’ll look at a couple of different examples of
smart pointers, including a *reference counting* smart pointer type. This
pointer enables you to allow data to have multiple owners by keeping track of
the number of owners and, when no owners remain, cleaning up the data.
-->

一方、*スマートポインタ*は、ポインタのように振る舞いますが、追加のメタデータと能力があるデータ構造です。
スマートポインタという概念は、Rustに特有のものではありません。スマートポインタは、C++に端を発し、
他の言語にも存在しています。Rustには、参照以上の機能を提供する色々なスマートポインタが標準ライブラリに定義されています。
一般的な概念を探索するために、いくつかのスマートポインタの例を見ていきますが、その一つが*参照カウント*方式のスマートポインタ型です。
このポインタのおかげでデータに複数の所有者を持たせることができます。
所有者の数を追いかけ、所有者がいなくなったらデータの片付けをしてくれるからです。

<!--
Rust, with its concept of ownership and borrowing, has an additional difference
between references and smart pointers: while references only borrow data, in
many cases, smart pointers *own* the data they point to.
-->

所有権と借用の概念を持つRustでは、参照とスマートポインタにはもう1つ違いがあります:
参照はデータを借用するだけである一方で、スマートポインタは、多くの場合、指しているデータを*所有*します。

<!--
Though we didn’t call them as such at the time, we’ve already encountered a few
smart pointers in this book, including `String` and `Vec<T>` in Chapter 8. Both
these types count as smart pointers because they own some memory and allow you
to manipulate it. They also have metadata and extra capabilities or guarantees.
`String`, for example, stores its capacity as metadata and has the extra
ability to ensure its data will always be valid UTF-8.
-->

そのときはそう呼ばなかったものの、私達はすでに、この本の中でいくつかのスマートポインタに遭遇してきました。
例えば第8章の`String`や`Vec<T>`です。これらの型はどちらも、あるメモリを所有しそれを弄ることができるので、
スマートポインタと見なされます。また、メタデータや追加の能力、あるいは保証もあります。
例えば`String`は、その許容量をメタデータとして保持し、データが常に有効なUTF-8であると保証する追加の能力を持ちます。

<!--
Smart pointers are usually implemented using structs. Unlike an ordinary
struct, smart pointers implement the `Deref` and `Drop` traits. The `Deref`
trait allows an instance of the smart pointer struct to behave like a reference
so you can write your code to work with either references or smart pointers.
The `Drop` trait allows you to customize the code that’s run when an instance
of the smart pointer goes out of scope. In this chapter, we’ll discuss both
traits and demonstrate why they’re important to smart pointers.
-->

スマートポインタは普通、構造体を使用して実装されています。通常の構造体とは異なり、
スマートポインタが`Deref`と`Drop`トレイトを実装します。`Deref`トレイトにより、スマートポインタ構造体のインスタンスは、
参照のように振る舞うことができるので、参照あるいはスマートポインタのどちらとも動作するようにコードを書くことができます。
`Drop`トレイトにより、スマートポインタのインスタンスがスコープを外れた時に走るコードをカスタマイズすることができます。
この章では、どちらのトレイトについても議論し、これらのトレイトがスマートポインタにとって重要な理由を説明します。

<!--
Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:
-->

スマートポインタパターンがRustにおいてよく使われる一般的なデザインパターンであることを考えれば、この章で既存のスマートポインタを全て取り扱うことなどできません。
多くのライブラリに独自のスマートポインタがあり、自分だけのスマートポインタを書くことさえできるのです。
ここでは標準ライブラリの最もありふれたスマートポインタを取り扱っていきます。

<!--
* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time
-->

* ヒープに値を確保する`Box<T>`
* 複数の所有権を可能にする参照カウント型の`Rc<T>`
* `RefCell<T>`を通してアクセスされ、コンパイル時ではなく実行時に借用規則を強制する型の`Ref<T>`と`RefMut<T>`

<!--
In addition, we’ll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
*reference cycles*: how they can leak memory and how to prevent them.
-->

さらに、*内部可変性*パターンも扱います。そこでは不変な型が、内部の値を変更するためのAPIを公開するのです。
また、*循環参照*についても議論します。つまり、循環参照によっていかにしてメモリがリークするのか、そしてどうやってそれを回避するのかを議論します。

<!--
Let’s dive in!
-->

さあ、飛び込みましょう！
