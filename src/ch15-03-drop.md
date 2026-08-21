<!--
## Running Code on Cleanup with the `Drop` Trait
-->

## `Drop`トレイトで片付け時にコードを走らせる

<!--
The second trait important to the smart pointer pattern is `Drop`, which lets
you customize what happens when a value is about to go out of scope. You can
provide an implementation for the `Drop` trait on any type, and that code can
be used to release resources like files or network connections.
-->

スマートポインタパターンにとって重要な2番目のトレイトは、`Drop`であり、
これのおかげで値がスコープを抜けそうになった時に起こることをカスタマイズできます。
どんな型に対しても`Drop`トレイトの実装を提供することができ、
そのコードはファイルやネットワーク接続などのリソースを解放するために使用できます。

<!--
We’re introducing `Drop` in the context of smart pointers because the
functionality of the `Drop` trait is almost always used when implementing a
smart pointer. For example, when a `Box<T>` is dropped it will deallocate the
space on the heap that the box points to.
-->

`Drop`をスマートポインタの文脈で導入しています。`Drop`トレイトの機能は、ほぼ常にスマートポインタを実装する時に使われるからです。
例えば`Box<T>`はドロップされるときに、そのボックスが指しているヒープの領域を解放するでしょう。

<!--
In some languages, for some types, the programmer must call code to free memory
or resources every time they finish using an instance of those types. Examples
include file handles, sockets, or locks. If they forget, the system might
become overloaded and crash. In Rust, you can specify that a particular bit of
code be run whenever a value goes out of scope, and the compiler will insert
this code automatically. As a result, you don’t need to be careful about
placing cleanup code everywhere in a program that an instance of a particular
type is finished with—you still won’t leak resources!
-->

言語によっては、さらに型によっては、プログラマがその型のインスタンスを使い終わる度に、
メモリやリソースを解放するコードを呼ばなければならないことがあります。
そうした型の例にはファイルハンドル、ソケット、またはロックなどが含まれます。
忘れてしまったら、システムは詰め込みすぎになりクラッシュする可能性があります。Rustでは、
値がスコープを抜ける度に特定のコードが走るよう指定でき、コンパイラはこのコードを自動的に挿入します。
結果として、特定の型のインスタンスを使い終わったプログラムの箇所全部にクリーンアップコードを配置するのに配慮する必要はありません。
それでもリソースをリークすることはありません。

<!--
You specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires you to implement one method named
`drop` that takes a mutable reference to `self`. To see when Rust calls `drop`,
let’s implement `drop` with `println!` statements for now.
-->

`Drop`トレイトを実装することで、値がスコープを抜けた時に走るコードを指定します。
`Drop`トレイトは、`self`への可変参照を取る`drop`という1つのメソッドを実装する必要があります。
いつRustが`drop`を呼ぶのか確認するために、今は`println!`文のある`drop`を実装しましょう。

<!--
Listing 15-14 shows a `CustomSmartPointer` struct whose only custom
functionality is that it will print `Dropping CustomSmartPointer!` when the
instance goes out of scope, to show when Rust runs the `drop` function.
-->

リスト15-14では、コンパイラがいつ`drop`関数を走らせるかを示すために、
インスタンスがスコープを抜ける時に`Dropping CustomSmartPointer!`と出力するだけの独自の機能を持つ、
`CustomSmartPointer`構造体を示します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<!--
<span class="caption">Listing 15-14: A `CustomSmartPointer` struct that
implements the `Drop` trait where we would put our cleanup code</span>
-->

<span class="caption">リスト15-14: クリーンアップコードを配置する`Drop`トレイトを実装する`CustomSmartPointer`構造体</span>

<!--
The `Drop` trait is included in the prelude, so we don’t need to bring it into
scope. We implement the `Drop` trait on `CustomSmartPointer` and provide an
implementation for the `drop` method that calls `println!`. The body of the
`drop` function is where you would place any logic that you wanted to run when
an instance of your type goes out of scope. We’re printing some text here to
demonstrate visually when Rust will call `drop`.
-->

`Drop`トレイトは、初期化処理に含まれるので、スコープ内に持ち込む必要はありません。
`CustomSmartPointer`に`Drop`トレイトを実装し、`println!`を呼び出す`drop`メソッドの実装を提供しています。
`drop`関数の本体は、自分の型のインスタンスがスコープを抜ける時に走らせたいあらゆるロジックを配置する場所です。
コンパイラがいつ`drop`を呼ぶのかを視覚的に示すために、ここでテキストを出力しています。

<!--
In `main`, we create two instances of `CustomSmartPointer` and then print
`CustomSmartPointers created`. At the end of `main`, our instances of
`CustomSmartPointer` will go out of scope, and Rust will call the code we put
in the `drop` method, printing our final message. Note that we didn’t need to
call the `drop` method explicitly.
-->

`main`で、`CustomSmartPointer`のインスタンスを2つ作り、それから`CustomSmartPointers created.`と出力しています。
`main`の最後で、`CustomSmartPointer`のインスタンスはスコープを抜け、コンパイラは最後のメッセージを出力しながら、
`drop`メソッドに置いたコードを呼び出します。`drop`メソッドを明示的に呼び出す必要はなかったことに注意してください。

<!--
When we run this program, we’ll see the following output:
-->

このプログラムを実行すると、以下のような出力が出ます:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

<!--
Rust automatically called `drop` for us when our instances went out of scope,
calling the code we specified. Variables are dropped in the reverse order of
their creation, so `d` was dropped before `c`. This example’s purpose is to
give you a visual guide to how the `drop` method works; usually you would
specify the cleanup code that your type needs to run rather than a print
message.
-->

インスタンスがスコープを抜けた時に指定したコードを呼び出しながらコンパイラは、`drop`を自動的に呼び出してくれました。
変数は、生成されたのと逆の順序でドロップされるので、`d`は`c`より先にドロップされました。
この例の目的は`drop`メソッドがどう機能するのかを視覚的に案内することですが、通常は、メッセージ出力ではなく、
自分の型が走らせる必要のあるクリーンアップコードを指定するでしょう。

<!--
### Dropping a Value Early with `std::mem::drop`
-->

### `std::mem::drop`で早期に値をドロップする

<!--
Unfortunately, it’s not straightforward to disable the automatic `drop`
functionality. Disabling `drop` isn’t usually necessary; the whole point of the
`Drop` trait is that it’s taken care of automatically. Occasionally, however,
you might want to clean up a value early. One example is when using smart
pointers that manage locks: you might want to force the `drop` method that
releases the lock so that other code in the same scope can acquire the lock.
Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead
you have to call the `std::mem::drop` function provided by the standard library
if you want to force a value to be dropped before the end of its scope.
-->

残念ながら、自動的な`drop`機能を無効化することは、単純ではありません。通常、`drop`を無効化する必要はありません;
`Drop`トレイトの最重要な要点は、自動的に考慮されることです。ですが、時として、値を早期に片付けたくなる可能性があります。
一例は、ロックを管理するスマートポインタを使用する時です: 同じスコープの他のコードがロックを獲得できるように、
ロックを解放する`drop`メソッドを強制したくなる可能性があります。Rustは、
`Drop`トレイトの`drop`メソッドを手動で呼ばせてくれません; スコープが終わる前に値を強制的にドロップさせたいなら、
代わりに標準ライブラリが提供する`std::mem::drop`関数を呼ばなければなりません。

<!--
If we try to call the `Drop` trait’s `drop` method manually by modifying the
`main` function from Listing 15-14, as shown in Listing 15-15, we’ll get a
compiler error:
-->

リスト15-14の`main`関数を変更して手動で`Drop`トレイトの`drop`メソッドを呼び出そうとしたら、
コンパイルエラーになるでしょう。リスト15-15のようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-15: Attempting to call the `drop` method from
the `Drop` trait manually to clean up early</span>
-->

<span class="caption">リスト15-15: `Drop`トレイトから`drop`メソッドを手動で呼び出し、早期に片付けようとする</span>

<!--
When we try to compile this code, we’ll get this error:
-->

このコードをコンパイルしてみようとすると、こんなエラーが出ます:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

<!--
This error message states that we’re not allowed to explicitly call `drop`. The
error message uses the term *destructor*, which is the general programming term
for a function that cleans up an instance. A *destructor* is analogous to a
*constructor*, which creates an instance. The `drop` function in Rust is one
particular destructor.
-->

明示的に`drop`を呼び出すことは許されていないことをこのエラーメッセージは述べています。
エラーメッセージは*デストラクタ*という専門用語を使っていて、これは、
インスタンスを片付ける関数の一般的なプログラミング専門用語です。*デストラクタ*は、
*コンストラクタ*に類似していて、これはインスタンスを生成します。Rustの`drop`関数は、
1種の特定のデストラクタです。

<!--
Rust doesn’t let us call `drop` explicitly because Rust would still
automatically call `drop` on the value at the end of `main`. This would cause a
*double free* error because Rust would be trying to clean up the same value
twice.
-->

コンパイラはそれでも、`main`の終端で値に対して自動的に`drop`を呼び出すので、`drop`を明示的に呼ばせてくれません。
コンパイラが2回同じ値を片付けようとするので、これは*二重解放*エラーを発生させるでしょう。

<!--
We can’t disable the automatic insertion of `drop` when a value goes out of
scope, and we can’t call the `drop` method explicitly. So, if we need to force
a value to be cleaned up early, we use the `std::mem::drop` function.
-->

値がスコープを抜けるときに`drop`が自動的に挿入されるのを無効化できず、`drop`メソッドを明示的に呼ぶこともできません。
よって、値を早期に片付けさせる必要があるなら、`std::mem::drop`関数を使用します。

<!--
The `std::mem::drop` function is different from the `drop` method in the `Drop`
trait. We call it by passing as an argument the value we want to force drop.
The function is in the prelude, so we can modify `main` in Listing 15-15 to
call the `drop` function, as shown in Listing 15-16:
-->

`std::mem::drop`関数は、`Drop`トレイトの`drop`メソッドとは異なります。
強制ドロップさせたい値を引数として渡して呼び出します。この関数はpreludeに含まれているので、
リスト15-15の`main`を変更して`drop`関数を呼び出せます。リスト15-16のようにですね:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-16: Calling `std::mem::drop` to explicitly
drop a value before it goes out of scope</span>
-->

<span class="caption">リスト15-16: 値がスコープを抜ける前に明示的にドロップするために`std::mem::drop`を呼び出す</span>

<!--
Running this code will print the following:
-->

このコードを実行すると、以下のように出力されます:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

<!--
The text ```Dropping CustomSmartPointer with data `some data`!``` is printed
between the `CustomSmartPointer created.` and `CustomSmartPointer dropped
before the end of main.` text, showing that the `drop` method code is called to
drop `c` at that point.
-->

```Dropping CustomSmartPointer with data `some data`!```というテキストが、
`CustomSmartPointer created.`と`CustomSmartPointer dropped before the end of main.`テキストの間に出力されるので、
`drop`メソッドのコードがその時点で呼び出されて`c`をドロップしたことを示しています。

<!--
3行目のwithを...があれば、と訳している。多分辞書にも載っている
-->

<!--
You can use code specified in a `Drop` trait implementation in many ways to
make cleanup convenient and safe: for instance, you could use it to create your
own memory allocator! With the `Drop` trait and Rust’s ownership system, you
don’t have to remember to clean up because Rust does it automatically.
-->

`Drop`トレイト実装で指定されたコードをいろんな方法で使用し、片付けを便利で安全にすることができます:
例を挙げれば、これを使用して独自のメモリアロケータを作ることもできるでしょう！`Drop`トレイトとRustの所有権システムがあれば、
コンパイラが自動的に行うので、片付けを覚えておく必要はなくなります。

<!--
You also don’t have to worry about problems resulting from accidentally
cleaning up values still in use: the ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.
-->

まだ使用中の値を間違って片付けてしまうことに起因する問題を心配する必要もなくて済みます:
参照が常に有効であると確認してくれる所有権システムが、値が最早使用されなくなった時に`drop`が1回だけ呼ばれることを保証してくれるのです。

<!--
Now that we’ve examined `Box<T>` and some of the characteristics of smart
pointers, let’s look at a few other smart pointers defined in the standard
library.
-->

これで`Box<T>`とスマートポインタの特徴の一部を調査したので、標準ライブラリに定義されている他のスマートポインタをいくつか見ましょう。
