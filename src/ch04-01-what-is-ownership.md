<!--
## What Is Ownership?
-->

## 所有権とは？

<!--
*Ownership* is a set of rules that govern how a Rust program manages memory.
-->

*所有権*とは、Rustプログラムがメモリをどのように管理するかを支配している、規則の集合です。

<!--
All programs have to manage the way they use a computer’s memory while running.
Some languages have garbage collection that regularly looks for no-longer-used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free the memory. Rust uses a third approach: memory is managed
through a system of ownership with a set of rules that the compiler checks. If
any of the rules are violated, the program won’t compile. None of the features
of ownership will slow down your program while it’s running.
-->

全てのプログラムは、実行中にコンピュータのメモリの使用方法を管理する必要があります。プログラムが動作するにつれて、
定期的に使用されていないメモリを検索するガベージコレクションを持つ言語もありますが、他の言語では、
プログラマが明示的にメモリを確保したり、解放したりしなければなりません。Rustでは第3の選択肢を取っています: 
メモリは、コンパイラがチェックする一定の規則とともに所有権システムを通じて管理されています。
規則のうちどれかひとつにでも違反すれば、プログラムはコンパイルできないでしょう。
どの所有権機能も、実行中にプログラムの動作を遅くすることはないでしょう。

<!--
Because ownership is a new concept for many programmers, it does take some time
to get used to. The good news is that the more experienced you become with Rust
and the rules of the ownership system, the easier you’ll find it to naturally
develop code that is safe and efficient. Keep at it!
-->

所有権は多くのプログラマにとって新しい概念なので、慣れるまでに時間がかかります。
嬉しいことに、Rustと、所有権システムの規則の経験を積んでいくにつれて、
自然に安全かつ効率的なコードを構築するのはだんだん簡単だと思えるようになるでしょう。
その調子でいきましょう！

<!--
When you understand ownership, you’ll have a solid foundation for understanding
the features that make Rust unique. In this chapter, you’ll learn ownership by
working through some examples that focus on a very common data structure:
strings.
-->

所有権を理解した時、Rustを際立たせる機能の理解に対する強固な礎を得ることになるでしょう。この章では、
非常に一般的なデータ構造に着目した例を取り扱うことで所有権を学んでいきます: 文字列です。

<!--
引用符付きの行は、日本語と英語を交互に書くとmdbookに正しく解析してもらえないので、英語、日本語の順にまとめて配置します
> ### The Stack and the Heap
>
> Many programming languages don’t require you to think about the stack and the
> heap very often. But in a systems programming language like Rust, whether a
> value is on the stack or the heap affects how the language behaves and why
> you have to make certain decisions. Parts of ownership will be described in
> relation to the stack and the heap later in this chapter, so here is a brief
> explanation in preparation.
>
> Both the stack and the heap are parts of memory available to your code to use
> at runtime, but they are structured in different ways. The stack stores
> values in the order it gets them and removes the values in the opposite
> order. This is referred to as *last in, first out*. Think of a stack of
> plates: when you add more plates, you put them on top of the pile, and when
> you need a plate, you take one off the top. Adding or removing plates from
> the middle or bottom wouldn’t work as well! Adding data is called *pushing
> onto the stack*, and removing data is called *popping off the stack*. All
> data stored on the stack must have a known, fixed size. Data with an unknown
> size at compile time or a size that might change must be stored on the heap
> instead.
>
> The heap is less organized: when you put data on the heap, you request a
> certain amount of space. The memory allocator finds an empty spot in the heap
> that is big enough, marks it as being in use, and returns a *pointer*, which
> is the address of that location. This process is called *allocating on the
> heap* and is sometimes abbreviated as just *allocating* (pushing values onto
> the stack is not considered allocating). Because the pointer to the heap is a
> known, fixed size, you can store the pointer on the stack, but when you want
> the actual data, you must follow the pointer. Think of being seated at a
> restaurant. When you enter, you state the number of people in your group, and
> the host finds an empty table that fits everyone and leads you there. If
> someone in your group comes late, they can ask where you’ve been seated to
> find you.
>
> Pushing to the stack is faster than allocating on the heap because the
> allocator never has to search for a place to store new data; that location is
> always at the top of the stack. Comparatively, allocating space on the heap
> requires more work because the allocator must first find a big enough space
> to hold the data and then perform bookkeeping to prepare for the next
> allocation.
>
> Accessing data in the heap is slower than accessing data on the stack because
> you have to follow a pointer to get there. Contemporary processors are faster
> if they jump around less in memory. Continuing the analogy, consider a server
> at a restaurant taking orders from many tables. It’s most efficient to get
> all the orders at one table before moving on to the next table. Taking an
> order from table A, then an order from table B, then one from A again, and
> then one from B again would be a much slower process. By the same token, a
> processor can do its job better if it works on data that’s close to other
> data (as it is on the stack) rather than farther away (as it can be on the
> heap).
>
> When your code calls a function, the values passed into the function
> (including, potentially, pointers to data on the heap) and the function’s
> local variables get pushed onto the stack. When the function is over, those
> values get popped off the stack.
>
> Keeping track of what parts of code are using what data on the heap,
> minimizing the amount of duplicate data on the heap, and cleaning up unused
> data on the heap so you don’t run out of space are all problems that ownership
> addresses. Once you understand ownership, you won’t need to think about the
> stack and the heap very often, but knowing that the main purpose of ownership
> is to manage heap data can help explain why it works the way it does.
-->

> ### スタックとヒープ
>
> 多くのプログラミング言語において、スタックとヒープについて考える機会はそう多くないでしょう。
> しかし、Rustのようなシステムプログラミング言語においては、値がスタックに積まれるかヒープに置かれるかは、
> 言語の振る舞いや、特定の決断を下す理由などに影響を与えるのです。
> この章の後半でスタックとヒープを交えて所有権の一部が解説されるので、ここでちょっと予行演習をしておきましょう。
>
> スタックもヒープも、実行時にコードが使用できるメモリの一部になりますが、異なる手段で構成されています。
> スタックは、得た順番に値を並べ、逆の順で値を取り除いていきます。これは、
> *last in, first out*(`訳注`: あえて日本語にするなら、「最後に入れたものが最初に出てくる」といったところでしょうか)と呼ばれます。
> お皿の山を思い浮かべてください: お皿を追加する時には、山の一番上に置き、お皿が必要になったら、一番上から1枚を取り去りますよね。
> 途中や一番下に追加したり、取り除いたりすることもできません。データを追加することは、
> *スタックにpushする*といい、データを取り除くことは、*スタックからpopする*と表現します(`訳注`:
> 日本語では単純に英語をそのまま活用してプッシュ、ポップと表現するでしょう)。
> スタック上のデータは全て既知の固定サイズでなければなりません。
> コンパイル時にサイズがわからなかったり、サイズが可変のデータについては、代わりにヒープに格納しなければなりません。
>
> ヒープは、もっとごちゃごちゃしています: ヒープにデータを置く時、あるサイズのスペースを求めます。
> メモリアロケータはヒープ上に十分な大きさの空の領域を見つけ、使用中にし、*ポインタ*を返します。ポインタとは、その場所へのアドレスです。
> この過程は、*ヒープに領域を確保する (allocating on the heap)* と呼ばれ、時としてそのフレーズを単に*allocateする*などと省略したりします
> (`訳注`: こちらもこなれた日本語訳はないでしょう。allocateは「メモリを確保する」と訳したいところですが)
> (スタックに値を積むことは、メモリ確保とは考えられません)。ヒープへのポインタは、既知の固定サイズなので、
> スタックに保管することができますが、実データが必要になったら、ポインタを追いかける必要があります。
> レストランで席を確保することを考えましょう。入店したら、グループの人数を告げ、
> 店員が全員座れる空いている席を探し、そこまで誘導します。もしグループの誰かが遅れて来るのなら、
> 着いた席の場所を尋ねてあなたを発見することができます。
>
> スタックにデータを積むほうが、ヒープ上にメモリを確保するより高速です。
> アロケータが新しいデータを置くための場所を探さなくていいからです。というのも、その場所は常にスタックの一番上だからですね。
> それと比べて、ヒープ上にメモリを確保するのはより手間の多い仕事です。
> アロケータはまずデータを入れるのに充分な大きさの場所を探して、そして次のメモリ確保に備えて予約をしないといけないからです。
>
> ヒープへのデータアクセスは、スタックのデータへのアクセスよりも低速です。
> ポインタを追って目的の場所に到達しなければならないからです。現代のプロセッサは、メモリをあちこち行き来しなければ、
> より速くなります。似た例えを続けましょう。レストランで多くのテーブルから注文を受ける給仕人を考えましょう。最も効率的なのは、
> 次のテーブルに移らずに、一つのテーブルで全部の注文を受け付けてしまうことです。テーブルAで注文を受け、
> それからテーブルBの注文、さらにまたA、それからまたBと渡り歩くのは、かなり低速な過程になってしまうでしょう。
> 同じ意味で、プロセッサは、
> データが隔離されている(ヒープではそうなっている可能性がある)よりも近くにある(スタックではこうなる)ほうが、
> 仕事をうまくこなせるのです。
>
> コードが関数を呼び出すと、関数に渡された値(ヒープのデータへのポインタも含まれる可能性あり)と、
> 関数のローカル変数がスタックに載ります。関数の実行が終了すると、それらの値はスタックから取り除かれます。
>
> どの部分のコードがどのヒープ上のデータを使用しているか把握すること、ヒープ上の重複するデータを最小化すること、
> メモリ不足にならないようにヒープ上の未使用のデータを掃除することは全て、所有権が解決する問題です。
> 一度所有権を理解したら、あまり頻繁にスタックとヒープに関して考える必要はなくなるでしょうが、
> 所有権の主な目的はヒープデータを管理することだと知っていると、所有権がどうしてこうなっているのか説明するのに役立つこともあります。
>

<!--
### Ownership Rules
-->

### 所有権規則

<!--
First, let’s take a look at the ownership rules. Keep these rules in mind as we
work through the examples that illustrate them:
-->

まず、所有権の規則について見ていきましょう。
この規則を具体化する例を扱っていく間もこれらの規則を肝に銘じておいてください:

<!--
* Each value in Rust has an *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.
-->

* Rustの各値には、*所有者*が存在する。
* いかなる時も所有者は一つである。
* 所有者がスコープから外れたら、値は破棄される。

<!--
### Variable Scope
-->

### 変数スコープ

<!--
Now that we’re past basic Rust syntax, we won’t include all the `fn main() {`
code in examples, so if you’re following along, make sure to put the following
examples inside a `main` function manually. As a result, our examples will be a
bit more concise, letting us focus on the actual details rather than
boilerplate code.
-->

もう基本的なRustの文法は通り過ぎたので、`fn main() {`というコードはもう例に含みません。
従って、例をなぞっているなら、これからの例は`main`関数に手動で入れ込むようにしてください。
結果的に、例は少々簡潔になり、定型コードよりも具体的な詳細に集中しやすくなります。

<!--
As a first example of ownership, we’ll look at the *scope* of some variables. A
scope is the range within a program for which an item is valid. Take the
following variable:
-->

所有権の最初の例として、何らかの変数の*スコープ*について見ていきましょう。スコープとは、
要素が有効になるプログラム内の範囲のことです。以下の変数を例に取ります:

```rust
let s = "hello";
```

<!--
The variable `s` refers to a string literal, where the value of the string is
hardcoded into the text of our program. The variable is valid from the point at
which it’s declared until the end of the current *scope*. Listing 4-1 shows a
program with comments annotating where the variable `s` would be valid.
-->

変数`s`は、文字列リテラルを参照し、ここでは、文字列の値はプログラムのテキストとしてハードコードされています。
この変数は、宣言された地点から、現在の*スコープ*の終わりまで有効になります。リスト4-1には、
変数`s`が有効な場所に関する注釈がコメントで付記されたプログラムを示します。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<!--
<span class="caption">Listing 4-1: A variable and the scope in which it is
valid</span>
-->

<span class="caption">リスト4-1: 変数と有効なスコープ</span>

<!--
In other words, there are two important points in time here:
-->

言い換えると、ここまでに重要な点は二つあります:

<!--
* When `s` comes *into* scope, it is valid.
* It remains valid until it goes *out of* scope.
-->

* `s`がスコープに*入る*と、有効になる。
* スコープを*抜ける*まで、有効なまま。

<!--
At this point, the relationship between scopes and when variables are valid is
similar to that in other programming languages. Now we’ll build on top of this
understanding by introducing the `String` type.
-->

今のところ、スコープと、変数が有効である期間の関係は、他の言語のそれに類似しています。さて、この理解のもとに、
`String`型を導入して構築していきましょう。

<!--
### The `String` Type
-->

### `String`型

<!--
To illustrate the rules of ownership, we need a data type that is more complex
than those we covered in the [“Data Types”][data-types] section
of Chapter 3. The types covered previously are of a known size, can be stored
on the stack and popped off the stack when their scope is over, and can be
quickly and trivially copied to make a new, independent instance if another
part of code needs to use the same value in a different scope. But we want to
look at data that is stored on the heap and explore how Rust knows when to
clean up that data, and the `String` type is a great example.
-->

所有権の規則を具体化するには、第3章の[「データ型」][data-types]節で講義したものよりも、より複雑なデータ型が必要になります。
以前講義した型は、既知のサイズを持ち、スタックに置いておいてスコープが終わるとスタックから取り除かれるという扱いが可能でした。
さらに、コードの別の場所で同じ値を異なるスコープを持たせて使う必要があるなら、
新しい、独立したインスタンスに、高速に、そして自明にコピーすることができました。
が、ヒープに確保されるデータ型を観察して、
コンパイラがどうそのデータを掃除すべきタイミングを把握しているかを掘り下げていきたいと思います。
`String`型はその好例です。

<!--
We’ll concentrate on the parts of `String` that relate to ownership. These
aspects also apply to other complex data types, whether they are provided by
the standard library or created by you. We’ll discuss `String` in more depth in
Chapter 8.
-->

`String`型の所有権にまつわる部分に着目しましょう。
またこの観点は、標準ライブラリから提供されたか自分で作成したかを問わず、他の複雑なデータ型にも適用されます。
`String`型については、[第8章][ch8]でより深く議論します。

<!--
We’ve already seen string literals, where a string value is hardcoded into our
program. String literals are convenient, but they aren’t suitable for every
situation in which we may want to use text. One reason is that they’re
immutable. Another is that not every string value can be known when we write
our code: for example, what if we want to take user input and store it? For
these situations, Rust has a second string type, `String`. This type manages
data allocated on the heap and as such is able to store an amount of text that
is unknown to us at compile time. You can create a `String` from a string
literal using the `from` function, like so:
-->

既に文字列リテラルは見かけましたね。文字列リテラルでは、文字列の値はプログラムにハードコードされます。
文字列リテラルは便利ですが、テキストを使いたいかもしれない場面全てに最適なわけではありません。一因は、
文字列リテラルが不変であることに起因します。別の原因は、コードを書く際に、全ての文字列値が判明するわけではないからです:
例えば、ユーザ入力を受け付け、それを保持したいとしたらどうでしょうか？このような場面用に、Rustには、
2種類目の文字列型、`String`型があります。この型はヒープにメモリを確保するので、
コンパイル時にはサイズが不明なテキストも保持することができるのです。`from`関数を使用して、
文字列リテラルから`String`型を生成できます。以下のように:

```rust
let s = String::from("hello");
```

<!--
The double colon `::` operator allows us to namespace this particular `from`
function under the `String` type rather than using some sort of name like
`string_from`. We’ll discuss this syntax more in the [“Method
Syntax”][method-syntax] section of Chapter 5, and when we talk
about namespacing with modules in [“Paths for Referring to an Item in the
Module Tree”][paths-module-tree] in Chapter 7.
-->

この二重コロン`::`演算子は、`string_from`などの名前を使うのではなく、
`String`型直下の`from`関数を特定する働きをします。この記法について詳しくは、
第5章の[「メソッド記法」][method-syntax]節と、第7章の[「モジュールツリーの要素を示すためのパス」][paths-module-tree]でモジュールを使った名前空間分けについて話をするときに議論します。

<!--
This kind of string *can* be mutated:
-->

この種の文字列は、変更することが*できます*:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```


<!--
So, what’s the difference here? Why can `String` be mutated but literals
cannot? The difference is in how these two types deal with memory.
-->

では、ここでの違いは何でしょうか？なぜ、`String`型は変更できるのに、リテラルはできないのでしょうか？
違いは、これら二つの型がメモリを扱う方法にあります。

<!--
### Memory and Allocation
-->

### メモリと確保

<!--
In the case of a string literal, we know the contents at compile time, so the
text is hardcoded directly into the final executable. This is why string
literals are fast and efficient. But these properties only come from the string
literal’s immutability. Unfortunately, we can’t put a blob of memory into the
binary for each piece of text whose size is unknown at compile time and whose
size might change while running the program.
-->

文字列リテラルの場合、中身はコンパイル時に判明しているので、テキストは最終的なバイナリファイルに直接ハードコードされます。
このため、文字列リテラルは、高速で効率的になるのです。しかし、これらの特性は、
その文字列リテラルの不変性にのみ端を発するものです。残念なことに、コンパイル時にサイズが不明だったり、
プログラム実行に合わせてサイズが可変なテキスト片用に一塊のメモリをバイナリに確保しておくことは不可能です。

<!--
With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:
-->

`String`型では、可変かつ伸長可能なテキスト破片をサポートするために、コンパイル時には不明な量のメモリを
ヒープに確保して内容を保持します。つまり:

<!--
* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we’re done with
  our `String`.
-->

* メモリは、実行時にメモリアロケータに要求される。
* `String`型を使用し終わったら、アロケータにこのメモリを返還する方法が必要である。

<!--
That first part is done by us: when we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.
-->

この最初の部分は、既にしています: `String::from`関数を呼んだら、その実装が必要なメモリを要求するのです。
これは、プログラミング言語において、極めて普遍的です。

<!--
However, the second part is different. In languages with a *garbage collector
(GC)*, the GC keeps track of and cleans up memory that isn’t being used
anymore, and we don’t need to think about it. In most languages without a GC,
it’s our responsibility to identify when memory is no longer being used and to
call code to explicitly free it, just as we did to request it. Doing this
correctly has historically been a difficult programming problem. If we forget,
we’ll waste memory. If we do it too early, we’ll have an invalid variable. If
we do it twice, that’s a bug too. We need to pair exactly one `allocate` with
exactly one `free`.
-->

<!--
かっこがあると、*が機能しないようなので、(GC)の部分には指定していません
-->

しかしながら、2番目の部分は異なります。*ガベージコレクタ*(GC)付きの言語では、GCがこれ以上、
使用されないメモリを検知して片付けるため、プログラマは、そのことを考慮する必要はありません。
GCを持たない言語の多くでは、メモリがもう使用されないことを見計らって、明示的に解放するコードを呼び出すのは、
プログラマの責任になります。ちょうど要求の際にしたようにですね。これを正確にすることは、
歴史的にも難しいプログラミング問題の一つであり続けています。もし、忘れていたら、メモリを無駄にします。
タイミングが早すぎたら、無効な変数を作ってしまいます。2回解放してしまっても、バグになるわけです。
`allocate`と`free`は完璧に1対1対応にしなければならないのです。

<!--
Rust takes a different path: the memory is automatically returned once the
variable that owns it goes out of scope. Here’s a version of our scope example
from Listing 4-1 using a `String` instead of a string literal:
-->

Rustは、異なる道を歩んでいます: ひとたび、メモリを所有している変数がスコープを抜けたら、
メモリは自動的に返還されます。こちらの例は、
リスト4-1のスコープ例を文字列リテラルから`String`型を使うものに変更したバージョンになります:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

<!--
There is a natural point at which we can return the memory our `String` needs
to the allocator: when `s` goes out of scope. When a variable goes out of
scope, Rust calls a special function for us. This function is called
[`drop`][drop], and it’s where the author of `String` can put
the code to return the memory. Rust calls `drop` automatically at the closing
curly bracket.
-->

`String`型が必要とするメモリをアロケータに返還することが自然な地点があります: `s`変数がスコープを抜ける時です。
変数がスコープを抜ける時、Rustは特別な関数を呼んでくれます。この関数は、[`drop`][drop]と呼ばれ、
ここに`String`型の書き手はメモリを返還するコードを配置することができます。Rustは、閉じ波括弧で自動的に`drop`関数を呼び出します。

<!--
> Note: In C++, this pattern of deallocating resources at the end of an item’s
> lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*.
> The `drop` function in Rust will be familiar to you if you’ve used RAII
> patterns.
-->

> 注釈: C++では、要素の生存期間の終了地点でリソースを解放するこのパターンを時に、
> *RAII*(Resource Aquisition Is Initialization: リソースの獲得は、初期化である)と呼んだりします。
> Rustの`drop`関数は、あなたがRAIIパターンを使ったことがあれば、馴染み深いものでしょう。

<!--
This pattern has a profound impact on the way Rust code is written. It may seem
simple right now, but the behavior of code can be unexpected in more
complicated situations when we want to have multiple variables use the data
we’ve allocated on the heap. Let’s explore some of those situations now.
-->

このパターンは、Rustコードの書かれ方に甚大な影響をもたらします。現状は簡単そうに見えるかもしれませんが、
ヒープ上に確保されたデータを複数の変数に使用させるようなもっと複雑な場面では、コードの振る舞いは、
予期しないものになる可能性もあります。これから、そのような場面を掘り下げてみましょう。

<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="ways-variables-and-data-interact-move"></a>
-->

<!--
#### Variables and Data Interacting with Move
-->

#### ムーブによる変数とデータの相互作用

<!--
Multiple variables can interact with the same data in different ways in Rust.
Let’s look at an example using an integer in Listing 4-2.
-->

Rustにおいては、複数の変数が同じデータに対して異なる手段で相互作用することができます。
整数を使用したリスト4-2の例を見てみましょう。

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span>
-->

<span class="caption">リスト4-2: 変数`x`の整数値を`y`に代入する</span>

<!--
We can probably guess what this is doing: “bind the value `5` to `x`; then make
a copy of the value in `x` and bind it to `y`.” We now have two variables, `x`
and `y`, and both equal `5`. This is indeed what is happening, because integers
are simple values with a known, fixed size, and these two `5` values are pushed
onto the stack.
-->

もしかしたら、何をしているのか予想することができるでしょう: 
「値`5`を`x`に束縛する; それから`x`の値をコピーして`y`に束縛する。」これで、
二つの変数(`x`と`y`)が存在し、両方、値は`5`になりました。これは確かに起こっている現象を説明しています。
なぜなら、整数は既知の固定サイズの単純な値で、これら二つの`5`という値は、スタックに積まれるからです。

<!--
Now let’s look at the `String` version:
-->

では、`String`バージョンを見ていきましょう:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

<!--
This looks very similar, so we might assume that the way it works would be the
same: that is, the second line would make a copy of the value in `s1` and bind
it to `s2`. But this isn’t quite what happens.
-->

このコードは酷似していますので、動作方法も同じだと思い込んでしまうかもしれません:
要するに、2行目で`s1`の値をコピーし、`s2`に束縛するということです。ところが、
これは全く起こることを言い当てていません。

<!--
Take a look at Figure 4-1 to see what is happening to `String` under the
covers. A `String` is made up of three parts, shown on the left: a pointer to
the memory that holds the contents of the string, a length, and a capacity.
This group of data is stored on the stack. On the right is the memory on the
heap that holds the contents.
-->

図4-1を見て、ベールの下で`String`に何が起きているかを確かめてください。
`String`型は、左側に示されているように、3つの部品でできています: 
文字列の中身を保持するメモリへのポインタと長さ、そして、許容量です。この種のデータは、スタックに保持されます。
右側には、中身を保持したヒープ上のメモリがあります。 

<!--
<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />
-->

<img alt="2個の表: 1個目の表はスタック上のs1の表現を含み、表はs1の長さ(5)、許容量(5)、
そして2個目の表の最初の値へのポインタからなる。2個目の表はヒープ上の文字列データのバイト単位の表現を含む。"
src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-1: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>
-->

<span class="caption">図4-1: `s1`に束縛された`"hello"`という値を保持する`String`のメモリ上の表現</span>

<!--
The length is how much memory, in bytes, the contents of the `String` are
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the allocator. The difference between length and
capacity matters, but not in this context, so for now, it’s fine to ignore the
capacity.
-->

長さは、`String`型の中身が現在使用しているメモリ量をバイトで表したものです。許容量は、
`String`型がアロケータから受け取った全メモリ量をバイトで表したものです。長さと許容量の違いは問題になることですが、
この文脈では違うので、とりあえずは、許容量を無視しても構わないでしょう。 

<!--
When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-2.
-->

`s1`を`s2`に代入すると、`String`型のデータがコピーされます。つまり、スタックにあるポインタ、長さ、
許容量をコピーするということです。ポインタが指すヒープ上のデータはコピーしません。言い換えると、
メモリ上のデータ表現は図4-2のようになるということです。

<!--
<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />
-->

<img alt="3個の表: 表s1とs2はスタック上の文字列s1とs2それぞれを表現し、どちらもヒープ上の同じ文字列データを指している。"
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-2: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span>
-->

<span class="caption">図4-2: `s1`のポインタ、長さ、許容量のコピーを保持する変数`s2`のメモリ上での表現</span>

<!--
The representation does *not* look like Figure 4-3, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could be very expensive in terms of runtime performance if
the data on the heap were large.
-->

メモリ上の表現は、図4-3のようにはなり*ません*。これは、
Rustが代わりにヒープデータもコピーするという選択をしていた場合のメモリ表現ですね。Rustがこれをしていたら、
ヒープ上のデータが大きい時に`s2 = s1`という処理の実行時性能がとても悪くなっていた可能性があるでしょう。

<!--
<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />
-->

<img alt="4個の表: 2個の表はs1とs2のスタックデータを表現し、それぞれがヒープ上の文字列データのコピーを指している。"
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-3: Another possibility for what `s2 = s1` might
do if Rust copied the heap data as well</span>
-->

<span class="caption">図4-3: Rustがヒープデータもコピーしていた場合に`s2 = s1`という処理が行なった可能性のあること</span>

<!--
Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-2 shows both data pointers pointing to the same location. This is a
problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.
-->

先ほど、変数がスコープを抜けたら、Rustは自動的に`drop`関数を呼び出し、
その変数が使っていたヒープメモリを片付けると述べました。しかし、図4-2は、
両方のデータポインタが同じ場所を指していることを示しています。これは問題です: `s2`と`s1`がスコープを抜けたら、
両方とも同じメモリを解放しようとします。これは*二重解放*エラーとして知られ、以前触れたメモリ安全性上のバグの一つになります。
メモリを2回解放することは、memory corruption (`訳注`: メモリの崩壊。意図せぬメモリの書き換え) につながり、
セキュリティ上の脆弱性を生む可能性があります。

<!--
To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as
no longer valid. Therefore, Rust doesn’t need to free anything when `s1` goes
out of scope. Check out what happens when you try to use `s1` after `s2` is
created; it won’t work:
-->

メモリ安全性を保証するために、`let s2 = s1;`の行の後、コンパイラは`s1`が最早有効ではないとみなします。
故に`s1`がスコープを抜けた際に何も解放する必要がなくなるわけです。`s2`の生成後に`s1`を使用しようとしたら、
どうなるかを確認してみましょう。動かないでしょう:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

<!--
You’ll get an error like this because Rust prevents you from using the
invalidated reference:
-->

コンパイラが無効化された参照は使用させてくれないので、以下のようなエラーが出るでしょう:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

<!--
If you’ve heard the terms *shallow copy* and *deep copy* while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like making a shallow copy. But
because Rust also invalidates the first variable, instead of being called a
shallow copy, it’s known as a *move*. In this example, we would say that `s1`
was *moved* into `s2`. So, what actually happens is shown in Figure 4-4.
-->

他の言語を触っている間に*shallow copy*と*deep copy*という用語を耳にしたことがあるなら、
データのコピーなしにポインタと長さ、許容量をコピーするという概念は、shallow copyすることのように思えるかもしれません。
ですが、コンパイラは最初の変数をも無効化するので、shallow copyと呼ばれる代わりに、
*ムーブ*として知られているわけです。この例では、`s1`は`s2`に*ムーブ*されたと表現するでしょう。
以上より、実際に起きることを図4-4に示してみました。

<!--
<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />
-->

<img alt="3個の表: 表s1とs2はスタック上の文字列s1とs2それぞれを表現し、どちらもヒープ上の同じ文字列データを指している。
s1はもう有効ではないので、表s1はグレー表示されている。
s2だけがヒープデータにアクセスするために使用できる。" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-4: Representation in memory after `s1` has been
invalidated</span>
-->

<span class="caption">図4-4: `s1`が無効化された後のメモリ表現</span>

<!--
That solves our problem! With only `s2` valid, when it goes out of scope it
alone will free the memory, and we’re done.
-->

これにて一件落着です。`s2`だけが有効なので、スコープを抜けたら、それだけがメモリを解放して、
終わりになります。

<!--
In addition, there’s a design choice that’s implied by this: Rust will never
automatically create “deep” copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.
-->

付け加えると、これにより暗示される設計上の選択があります: Rustでは、
自動的にデータの"deep copy"が行われることは絶対にないわけです。それ故に、あらゆる*自動*コピーは、実行時性能の観点で言うと、
悪くないと考えてよいことになります。

<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="ways-variables-and-data-interact-clone"></a>
-->

<!--
#### Variables and Data Interacting with Clone
-->

#### クローンによる変数とデータの相互作用

<!--
If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.
-->

仮に、スタック上のデータだけでなく、*本当に*`String`型のヒープデータのdeep copyが必要ならば、
`clone`と呼ばれるよくあるメソッドを使うことができます。メソッド記法については第5章で議論しますが、
メソッドは多くのプログラミング言語に見られる機能なので、以前に見かけたこともあるんじゃないでしょうか。

<!--
Here’s an example of the `clone` method in action:
-->

これは、`clone`メソッドの動作例です:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

<!--
This works just fine and explicitly produces the behavior shown in Figure 4-3,
where the heap data *does* get copied.
-->

これは問題なく動作し、図4-3で示した動作を明示的に生み出します。ここでは、
ヒープデータが*実際に*コピーされています。

<!--
When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.
-->

`clone`メソッドの呼び出しを見かけたら、何らかの任意のコードが実行され、その実行コストは高いと把握できます。
何か違うことが起こっているなと見た目でわかるわけです。

<!--
#### Stack-Only Data: Copy
-->

#### スタックのみのデータ: コピー

<!--
There’s another wrinkle we haven’t talked about yet. This code using
integers—part of which was shown in Listing 4-2—works and is valid:
-->

まだ話題にしていない別の問題があります。
この整数を使用したコードは、一部をリスト4-2で示しましたが、うまく動作する有効なものです:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

<!--
But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.
-->

ですが、このコードは一見、今学んだことと矛盾しているように見えます:
`clone`メソッドの呼び出しがないのに、`x`は有効で、`y`にムーブされませんでした。

<!--
The reason is that types such as integers that have a known size at compile
time are stored entirely on the stack, so copies of the actual values are quick
to make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
different from the usual shallow copying, and we can leave it out.
-->

その理由は、整数のようなコンパイル時に既知のサイズを持つ型は、スタック上にすっぽり保持されるので、
実際の値をコピーするのも高速だからです。これは、変数`y`を生成した後にも`x`を無効化したくなる理由がないことを意味します。
換言すると、ここでは、shallow copyとdeep copyの違いがないことになり、
`clone`メソッドを呼び出しても、一般的なshallow copy以上のことをしなくなり、
そのまま放置しておけるということです。

<!--
Rust has a special annotation called the `Copy` trait that we can place on
types that are stored on the stack, as integers are (we’ll talk more about
traits in [Chapter 10][traits]). If a type implements the `Copy`
trait, variables that use it do not move, but rather are trivially copied,
making them still valid after assignment to another variable.

-->

Rustには`Copy`トレイトと呼ばれる特別な注釈があり、
整数のようなスタックに保持される型に対して配置することができます(トレイトについては[第10章][traits]でもっと詳しく話します)。
型が`Copy`トレイトを実装していれば、その型を持つ変数はムーブされず、代わりに自明な方法でにコピーされます。
そのため、他の変数に代入した後でも、元の変数は有効なままです。

<!--
Rust won’t let us annotate a type with `Copy` if the type, or any of its parts,
has implemented the `Drop` trait. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we’ll get a compile-time error. To learn about how to add the `Copy` annotation
to your type to implement the trait, see [“Derivable
Traits”][derivable-traits] in Appendix C.
-->

コンパイラは、型やその一部分でも`Drop`トレイトを実装している場合、`Copy`による注釈をさせてくれません。
型の値がスコープを外れた時に何か特別なことを起こす必要がある場合に、`Copy`注釈を追加すると、コンパイルエラーが出ます。
型に`Copy`注釈をつけてトレイトを実装する方法について学ぶには、付録Cの[「導出可能なトレイト」][derivable-traits]をご覧ください。

<!--
So, what types implement the `Copy` trait? You can check the documentation for
the given type to be sure, but as a general rule, any group of simple scalar
values can implement `Copy`, and nothing that requires allocation or is some
form of resource can implement `Copy`. Here are some of the types that
implement `Copy`:
-->

では、どの型が`Copy`トレイトを実装しているのでしょうか？
確実に知るためには、調べたい型のドキュメントをチェックすればいいのですが、
一般規則として、単純なスカラー値の集合は何でも`Copy`を実装でき、メモリ確保が必要だったり、
何らかの形態のリソースだったりするものは`Copy`を実装できません。ここに`Copy`を実装する型の一部を並べておきます。

<!--
* All the integer types, such as `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* All the floating-point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that also implement `Copy`. For example,
  `(i32, i32)` implements `Copy`, but `(i32, String)` does not.
-->

* あらゆる整数型。`u32`など。
* 論理値型である`bool`。`true`と`false`という値がある。
* あらゆる浮動小数点型、`f64`など。
* 文字型である`char`。
* タプル。ただ、`Copy`の型だけを含む場合。例えば、`(i32, i32)`は`Copy`だが、
`(i32, String)`は違う。

<!--
### Ownership and Functions
-->

### 所有権と関数

<!--
The mechanics of passing a value to a function are similar to those when
assigning a value to a variable. Passing a variable to a function will move or
copy, just as assignment does. Listing 4-3 has an example with some annotations
showing where variables go into and out of scope.
-->

関数に値を渡すことと、値を変数に代入することの仕組みは似ています。関数に変数を渡すと、
代入のようにムーブやコピーされます。リスト4-3は変数がスコープに入ったり、
抜けたりする地点について注釈してある例です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<!--
<span class="caption">Listing 4-3: Functions with ownership and scope
annotated</span>
-->

<span class="caption">リスト4-3: 所有権とスコープが注釈された関数群</span>

<!--
If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile-time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.
-->

`takes_ownership`の呼び出し後に`s`を呼び出そうとすると、コンパイラは、コンパイルエラーを投げるでしょう。
これらの静的チェックにより、ミスを犯さないでいられます。`s`や`x`を使用するコードを`main`に追加してみて、
どこで使えて、そして、所有権規則により、どこで使えないかを確認してください。

<!--
### Return Values and Scope
-->

### 戻り値とスコープ

<!--
Returning values can also transfer ownership. Listing 4-4 shows an example of a
function that returns some value, with similar annotations as those in Listing
4-3.
-->

値を返すことでも、所有権は移動します。リスト4-4に値を返す関数の例を、リスト4-3と似た注釈を付けて示します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<!--
<span class="caption">Listing 4-4: Transferring ownership of return
values</span>
-->

<span class="caption">リスト4-4: 戻り値の所有権を移動する</span>

<!--
The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.
-->

変数の所有権は、毎回同じパターンを辿っています: 別の変数に値を代入すると、ムーブされます。
ヒープにデータを含む変数がスコープを抜けると、データの所有権が別の変数にムーブされていない限り、
`drop`により片付けられるでしょう。

<!--
While this works, taking ownership and then returning ownership with every
function is a bit tedious. What if we want to let a function use a value but
not take ownership? It’s quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting
from the body of the function that we might want to return as well.
-->

これでもいいのですが、所有権を取り、またその所有権を戻す、ということを全ての関数でしていたら、ちょっとめんどくさいですね。
関数に値は使わせるものの所有権を取らないようにさせるにはどうするべきでしょうか。
返したいと思うかもしれない関数本体で発生したあらゆるデータとともに、再利用したかったら、渡されたものをまた返さなきゃいけないのは、
非常に煩わしいことです。

<!--
Rust does let us return multiple values using a tuple, as shown in Listing 4-5.
-->

Rustでは、タプルを使って複数の値を返すことができます。リスト4-5のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<!--
<span class="caption">Listing 4-5: Returning ownership of parameters</span>
-->

<span class="caption">リスト4-5: 引数の所有権を返す</span>

<!--
But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for using a value without
transferring ownership, called *references*.
-->

でも、これでは、大袈裟すぎますし、ありふれているはずの概念に対して、作業量が多すぎます。
私たちにとって幸運なことに、Rustには所有権を移動することなく値を使うための機能があり、*参照*と呼ばれます。

<!--
[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop
-->

[data-types]: ch03-02-data-types.html#データ型
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#メソッド記法
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
