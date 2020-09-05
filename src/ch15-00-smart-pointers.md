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
data. Also, they don’t have any overhead and are the kind of pointer we use
most often.
-->

*ポインタ*は、メモリのアドレスを含む変数の一般的な概念です。このアドレスは、何らかの他のデータを参照、または「指します」。
Rustにおいて、最もありふれた種類のポインタは、参照であり、第4章で習いましたね。参照は、
`&`記号で示唆され、指している値を借用します。データを参照すること以外に特別な能力は何もありません。
また、オーバーヘッドもなく、最も頻繁に使われる種類のポインタです。

<!--
*Smart pointers*, on the other hand, are data structures that not only act like
a pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. In Rust, the different smart pointers defined in
the standard library provide functionality beyond that provided by references.
One example that we’ll explore in this chapter is the *reference counting*
smart pointer type. This pointer enables you to have multiple owners of data by
keeping track of the number of owners and, when no owners remain, cleaning up
the data.
-->

一方、*スマートポインタ*は、ポインタのように振る舞うだけでなく、追加のメタデータと能力があるデータ構造です。
スマートポインタという概念は、Rustに特有のものではありません: スマートポインタは、C++に端を発し、
他の言語にも存在しています。Rustでは、標準ライブラリに定義された色々なスマートポインタが、
参照以上の機能を提供します。この章で探究する一つの例が、*参照カウント*方式のスマートポインタ型です。
このポインタにより、所有者の数を追いかけることでデータに複数の所有者を持たせることができ、
所有者がいなくなったら、データの片付けをしてくれます。

<!--
In Rust, which uses the concept of ownership and borrowing, an additional
difference between references and smart pointers is that references are
pointers that only borrow data; in contrast, in many cases, smart pointers
*own* the data they point to.
-->

所有権と借用の概念を使うRustで、参照とスマートポインタの別の差異は、参照はデータを借用するだけのポインタであることです;
対照的に多くの場合、スマートポインタは指しているデータを*所有*します。

<!--
We’ve already encountered a few smart pointers in this book, such as `String`
and `Vec<T>` in Chapter 8, although we didn’t call them smart pointers at the
time. Both these types count as smart pointers because they own some memory and
allow you to manipulate it. They also have metadata (such as their capacity)
and extra capabilities or guarantees (such as with `String` ensuring its data
will always be valid UTF-8).
-->

その時は、スマートポインタとは呼ばなかったものの、第8章の`String`や`Vec<T>`のように、
この本の中でいくつかのスマートポインタに遭遇してきました。これらの型はどちらも、
あるメモリを所有し、それを弄ることができるので、スマートポインタに数えられます。また、
メタデータ(キャパシティなど)や追加の能力、あるいは保証(`String`ならデータが常に有効なUTF-8であると保証することなど)もあります。

<!--
Smart pointers are usually implemented using structs. The characteristic that
distinguishes a smart pointer from an ordinary struct is that smart pointers
implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance
of the smart pointer struct to behave like a reference so you can write code
that works with either references or smart pointers. The `Drop` trait allows
you to customize the code that is run when an instance of the smart pointer
goes out of scope. In this chapter, we’ll discuss both traits and demonstrate
why they’re important to smart pointers.
-->

スマートポインタは普通、構造体を使用して実装されています。スマートポインタを通常の構造体と区別する特徴は、
スマートポインタは、`Deref`と`Drop`トレイトを実装していることです。`Deref`トレイトにより、スマートポインタ構造体のインスタンスは、
参照のように振る舞うことができるので、参照あるいはスマートポインタのどちらとも動作するコードを書くことができます。
`Drop`トレイトにより、スマートポインタのインスタンスがスコープを外れた時に走るコードをカスタマイズすることができます。
この章では、どちらのトレイトについても議論し、これらのトレイトがスマートポインタにとって重要な理由を説明します。

<!--
Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:
-->

スマートポインタパターンがRustにおいてよく使われる一般的なデザインパターンだとして、この章では、全ての既存のスマートポインタを講義しません。
多くのライブラリに独自のスマートポインタがあり、自分だけのスマートポインタを書くことさえできます。
標準ライブラリの最もありふれたスマートポインタを講義します:

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

さらに、不変な型が、内部の値を可変化するAPIを晒す*内部可変性*パターンについても講義します。
また、*循環参照*についても議論します: 循環参照により、メモリがリークする方法とそれを回避する方法です。

<!--
Let’s dive in!
-->

さあ、飛び込みましょう！
