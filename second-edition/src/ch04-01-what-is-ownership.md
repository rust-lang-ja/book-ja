<!-- ## What Is Ownership? -->

## 所有権とは？

<!-- Rust’s central feature is *ownership*. Although the feature is straightforward -->
<!-- to explain, it has deep implications for the rest of the language. -->

Rustの中心的な機能は、*所有権*です。機能は説明するのに単純なのですが、言語の残りの機能全てにかかるほど
深い裏の意味を含んでいるのです。

<!-- All programs have to manage the way they use a computer’s memory while running. -->
<!-- Some languages have garbage collection that constantly looks for no longer used -->
<!-- memory as the program runs; in other languages, the programmer must explicitly -->
<!-- allocate and free the memory. Rust uses a third approach: memory is managed -->
<!-- through a system of ownership with a set of rules that the compiler checks at -->
<!-- compile time. None of the ownership features slow down your proram while it's -->
<!-- running. -->

全てのプログラムは、実行中にコンピュータのメモリの使用方法を管理する必要があります。プログラムが動作するにつれて、
定期的に使用されていないメモリを検索するガベージコレクションを持つ言語もありますが、他の言語では、
プログラマが明示的にメモリを確保したり、解放したりしなければなりません。Rustでは第3の選択肢を取っています: 
メモリは、コンパイラがコンパイル時にチェックする一定の規則とともに所有権システムを通じて管理されています。
どの所有権機能も、実行中にプログラムの動作を遅くすることはありません。

<!-- Because ownership is a new concept for many programmers, it does take some time -->
<!-- to get used to. The good news is that the more experienced you become with Rust -->
<!-- and the rules of the ownership system, the more you’ll be able to naturally -->
<!-- develop code that is safe and efficient. Keep at it!  -->

所有権は多くのプログラマにとって新しい概念なので、慣れるまでに時間がかかります。
Rustと所有権システムの規則と経験を積むにつれて、自然に安全かつ効率的なコードを構築できるようになることは、
素晴らしいお知らせです。その調子でいきましょう！

<!-- When you understand ownership, you’ll have a solid foundation for understanding -->
<!-- the features that make Rust unique. In this chapter, you’ll learn ownership by -->
<!-- working through some examples that focus on a very common data structure: -->
<!-- strings. -->

所有権を理解した時、Rustを際立たせる機能の理解に対する強固な礎を得ることになるでしょう。この章では、
非常に一般的なデータ構造に着目した例を取り扱うことで所有権を学んでいくでしょう: 文字列です。

<!-- PROD: START BOX -->

<!-- 引用符付きの行は、日本語と英語を交互に書くとmdbookに正しく解析してもらえないので、英語、日本語の順にまとめて配置します -->
<!-- ### The Stack and the Heap -->

<!-- In many programming languages, you don’t have to think about the stack and -->
<!-- the heap very often. But in a systems programming language like Rust, whether -->
<!-- a value is on the stack or the heap has more of an effect on how the language -->
<!-- behaves and why you have to make certain decisions. Parts of ownership will -->
<!-- be described in relation to the stack and the heap later in this chapter, so -->
<!-- here is a brief explanation in preparation. -->

<!-- Both the stack and the heap are parts of memory that is available to your code -->
<!-- to use at runtime, but they are structured in different ways. The stack stores -->
<!-- values in the order it gets them and removes the values in the opposite order. -->
<!-- This is referred to as *last in, first out*. Think of a stack of plates: when -->
<!-- you add more plates, you put them on top of the pile, and when you need a -->
<!-- plate, you take one off the top. Adding or removing plates from the middle or -->
<!-- bottom wouldn’t work as well! Adding data is called *pushing onto the stack*, -->
<!-- and removing data is called *popping off the stack*. -->

<!-- The stack is fast because of the way it accesses the data: it never has to -->
<!-- search for a place to put new data or a place to get data from because that -->
<!-- place is always the top. Another property that makes the stack fast is that all -->
<!-- data on the stack must take up a known, fixed size. -->

<!-- > Data with a size unknown at compile time or a size that might change can be -->
<!-- > stored on the heap instead. The heap is less organized: when you put data on -->
<!-- > the heap, you ask for some amount of space. The operating system finds an -->
<!-- > empty spot somewhere in the heap that is big enough, marks it as being in -->
<!-- > use, and returns a *pointer*, which is the address of that location. This -->
<!-- > process is called *allocating on the heap*, sometimes abbreviated as just -->
<!-- > “allocating.” Pushing values onto the stack is not considered allocating. -->
<!-- > Because the pointer is a known, fixed size, you can store the pointer on the -->
<!-- > stack, but when you want the actual data, you have to follow the pointer. -->

<!-- Think of being seated at a restaurant. When you enter, you state the number of -->
<!-- people in your group, and the staff finds an empty table that fits everyone and -->
<!-- leads you there. If someone in your group comes late, they can ask where -->
<!-- you've been seated to find you. -->

<!-- Accessing data in the heap is slower than accessing data on the stack because -->
<!-- you have to follow a pointer to get there. Contemporary processors are faster -->
<!-- if they jump around less in memory. Continuing the analogy, consider a server -->
<!-- at a restaurant taking orders from many tables. It’s most efficient to get -->
<!-- all the orders at one table before moving on to the next table. Taking an -->
<!-- order from table A, then an order from table B, then one from A again, and -->
<!-- then one from B again would be a much slower process. By the same token, a -->
<!-- processor can do its job better if it works on data that’s close to other -->
<!-- data (as it is on the stack) rather than farther away (as it can be on the -->
<!-- heap). Allocating a large amount of space on the heap can also take time. -->

<!-- When your code calls a function, the values passed into the function -->
<!-- (including, potentially, pointers to data on the heap) and the function’s -->
<!-- local variables get pushed onto the stack. When the function is over, those -->
<!-- values get popped off the stack. -->

<!-- Keeping track of what parts of code are using what data on the heap, -->
<!-- minimizing the amount of duplicate data on the heap, and cleaning up unused -->
<!-- data on the heap so we don’t run out of space are all problems that ownership -->
<!-- addresses. Once you understand ownership, you won’t need to think about the -->
<!-- stack and the heap very often, but knowing that managing heap data is why -->
<!-- ownership exists can help explain why it works the way it does. -->

> ### スタックとヒープ
>
> 多くのプログラミング言語において、スタックとヒープについて考える機会はそう多くないでしょう。
> しかし、Rustのようなシステムプログラミング言語においては、値がスタックに載るかヒープに載るかは、
> 言語の振る舞い方や、特定の決断を下す理由などに影響以上のものを与えるのです。
> この章の後半でスタックとヒープを絡めて所有権に一部は解説されるので、ここでちょっと予行演習をしておきましょう。
>
> スタックもヒープも、実行時にコードが使用できるメモリの一部になりますが、異なる手段で構成されています。
> スタックは、得た順番に値を並べ、逆の順で値を取り除いていきます。これは、
> *last in, first out*(`脚注`: あえて日本語にするなら、けつ入れ頭出しといったところでしょうか)と呼ばれます。
> お皿の山を思い浮かべてください: お皿を追加する時には、山の一番上に置き、お皿が必要になったら、一番上から1枚を取り去りますよね。
> 途中や一番下に追加したり、取り除いたりすることは同じようにはできません。データを追加することは、
> *スタックにpushする*といい、データを取り除くことは、*スタックからpopする*と表現します(`脚注`:
> これらの動作に対する画一的な日本語訳を見かけたことはありません)。
>
> データへのアクセス方法のおかげで、スタックは高速です: 新データを置いたり、
> データを取得する場所を探す必要が絶対にないわけです。というのも、その場所は常に一番上だからですね。スタックを高速にする特性は、
> 他にもあり、それはスタック上のデータは全て既知の固定サイズにならなければならないということです。
>
> コンパイル時にサイズがわからなかったり、サイズが可変のデータについては、代わりにヒープに格納することができます。
> ヒープは、もっとごちゃごちゃしています: ヒープにデータを置く時、あるサイズのスペースを求めます。
> OSはヒープ上に十分な大きさの空の領域を見つけ、使用中にし、*ポインタ*を返してきます。ポインタとは、その場所へのアドレスです。
> この過程は、*ヒープに領域を確保する*と呼ばれ、時としてそのフレーズを単に*allocateする*などと省略したりします。
> (`脚注`: こちらもこなれた日本語訳はないでしょう。allocateはメモリを確保すると訳したいところですが)
> スタックに値を載せることは、メモリ確保とは考えられません。ポインタは、既知の固定サイズなので、
> スタックに保管することができますが、実データが必要になったら、ポインタを追いかける必要があります。
>
> レストランで席を確保することを考えましょう。入店したら、グループの人数を告げ、
> 店員が全員座れる空いている席を探し、そこまで誘導します。もしグループの誰かが遅れて来るのなら、
> 着いた席の場所を尋ねてあなたを発見することができます。
>
> ヒープへのデータアクセスは、スタックのデータへのアクセスよりも低速です。
> ポインタを追って目的の場所に到達しなければならないからです。現代のプロセッサは、メモリをあちこち行き来しなければ、
> より速くなります。似た例えを続けましょう。レストランで多くのテーブルから注文を受ける給仕人を考えましょう。最も効率的なのは、
> 次のテーブルに移らずに、一つのテーブルで全部の注文を受け付けてしまうことです。テーブルAで注文を受け、
> それからテーブルBの注文、さらにまたA、それからまたBと渡り歩くのは、かなり低速な過程になってしまうでしょう。
> 同じ意味で、プロセッサは、
> データが隔離されている(ヒープではそうなっている可能性がある)よりも近くにある(スタックではこうなる)ほうが、
> 仕事をうまくこなせるのです。ヒープに大きな領域を確保する行為も時間がかかることがあります。
>
> コードが関数を呼び出すと、関数に渡された値(ヒープのデータへのポインタも含まれる可能性あり)と、
> 関数のローカル変数がスタックに載ります。関数の実行が終了すると、それらの値はスタックから取り除かれます。
>
> どの部分のコードがどのヒープ上のデータを使用しているか把握すること、ヒープ上の重複するデータを最小化すること、
> メモリ不足にならないようにヒープ上の未使用のデータを掃除することは全て、所有権が解決する問題です。
> 一度所有権を理解したら、あまり頻繁にスタックとヒープに関して考える必要はなくなるでしょうが、
> ヒープデータを管理することが所有権の存在する理由だと知っていると、所有権がありのままで動作する理由を
> 説明するのに役立つこともあります。
>
<!-- PROD: END BOX -->

<!-- ### Ownership Rules -->

### 所有権規則

<!-- First, let’s take a look at the ownership rules. Keep these rules in mind as we -->
<!-- work through the examples that illustrate them: -->

まず、所有権のルールについて見ていきましょう。この規則を具体化する例を
扱っていく間もこれらのルールを肝に命じておいてください:

<!-- * Each value in Rust has a variable that’s called its *owner*. -->
<!-- * There can only be one owner at a time. -->
<!-- * When the owner goes out of scope, the value will be dropped. -->

* Rustの各値は、*所有者*と呼ばれる変数と対応している。
* いかなる時も所有者は一つである。
* 所有者がスコープから外れたら、値は破棄される。

<!-- ### Variable Scope -->

### 変数スコープ

<!-- We’ve walked through an example of a Rust program already in Chapter 2. Now -->
<!-- that we’re past basic syntax, we won’t include all the `fn main() {` code in -->
<!-- examples, so if you’re following along, you’ll have to put the following -->
<!-- examples inside a `main` function manually. As a result, our examples will be a -->
<!-- bit more concise, letting us focus on the actual details rather than -->
<!-- boilerplate code. -->

第2章で、Rustプログラムの例はすでに見ています。もう基本的な記法は通り過ぎたので、
`fn main() {`というコードはもう例に含みません。従って、例をなぞっているなら、
これからの例は`main`関数に手動で入れ込まなければいけなくなるでしょう。結果的に、例は少々簡潔になり、
定型コードよりも具体的な詳細に集中しやすくなります。

<!-- As a first example of ownership, we’ll look at the *scope* of some variables. A -->
<!-- scope is the range within a program for which an item is valid. Let’s say we -->
<!-- have a variable that looks like this: -->

所有権の最初の例として、何らかの変数の*スコープ*について見ていきましょう。スコープとは、
要素が有効になるプログラム内の範囲のことです。以下のような変数があるとしましょう:

```rust
let s = "hello";
```

<!-- The variable `s` refers to a string literal, where the value of the string is -->
<!-- hardcoded into the text of our program. The variable is valid from the point at -->
<!-- which it’s declared until the end of the current *scope*. Listing 4-1 has -->
<!-- comments annotating where the variable `s` is valid. -->

変数`s`は、文字列リテラルを参照し、ここでは、文字列の値はプログラムのテキストとしてハードコードされています。
この変数は、宣言された地点から、現在の*スコープ*の終わりまで有効になります。リスト4-1には、
変数`s`が有効な場所に関する注釈がコメントで付記されています。

<!-- ```rust -->
<!-- {                      // s is not valid here, it’s not yet declared -->
<!--    let s = "hello";   // s is valid from this point forward -->

<!--    // do stuff with s -->
<!-- }                      // this scope is now over, and s is no longer valid -->
<!-- ``` -->

```rust
{                      // sは、ここでは有効ではない。まだ宣言されていない
    let s = "hello";   // sは、ここから有効になる

    // sで作業をする
}                      // このスコープは終わり。もうsは有効ではない
```

<!-- <span class="caption">Listing 4-1: A variable and the scope in which it is
valid</span> -->

<span class="caption">リスト4-1: 変数と有効なスコープ</span>

<!-- In other words, there are two important points in time here: -->

言い換えると、ここまでに重要な点は二つあります:

<!-- * When `s` comes *into scope*, it is valid. -->
<!-- * It remains valid until it goes *out of scope*. -->

1. `s`が*スコープに入る*と、有効になる
1. *スコープを抜ける*まで、有効なまま

<!-- At this point, the relationship between scopes and when variables are valid is -->
<!-- similar to other programming languages. Now we’ll build on top of this -->
<!-- understanding by introducing the `String` type. -->

ここで、スコープと変数が有効になる期間の関係は、他の言語に類似しています。さて、この理解のもとに、
`String`型を導入して構築していきましょう。

<!-- ### The `String` Type -->

### `String`型

<!-- To illustrate the rules of ownership, we need a data type that is more complex -->
<!-- than the ones we covered in the "Data Types" section of Chapter 3. The types -->
<!-- covered previously are all stored on the stack and popped off the stack when -->
<!-- their scope is over, but we want to look at data that is stored on the heap and -->
<!-- explore how Rust knows when to clean up that data. -->

所有権の規則を具体化するには、第3章の「データ型」節で講義したものよりも、より複雑なデータ型が必要になります。
以前講義した型は全てスタックに保管され、スコープが終わるとスタックから取り除かれますが、
ヒープに確保されるデータ型を観察して、
コンパイラがどうそのデータを掃除すべきタイミングを把握しているかを掘り下げていきたいです。

<!-- We’ll use `String` as the example here and concentrate on the parts of `String` -->
<!-- that relate to ownership. These aspects also apply to other complex data types -->
<!-- provided by the standard library and that you create. We’ll discuss `String` in -->
<!-- more depth in Chapter 8. -->

ここでは、例として`String`型を使用し、`String`型の所有権にまつわる部分に着目しましょう。
また、この観点は、標準ライブラリや自分で生成する他の複雑なデータ型にも適用されます。
`String`型については、第8章でより深く議論します。

<!-- We’ve already seen string literals, where a string value is hardcoded into our -->
<!-- program. String literals are convenient, but they aren’t suitable for every -->
<!-- situation in which we may want to use text. One reason is that they’re -->
<!-- immutable. Another is that not every string value can be known when we write -->
<!-- our code: for example, what if we want to take user input and store it? For -->
<!-- these situations, Rust has a second string type, `String`. This type is -->
<!-- allocated on the heap and as such is able to store an amount of text that is -->
<!-- unknown to us at compile time. You can create a `String` from a string literal -->
<!-- using the `from` function, like so: -->

すでに文字列リテラルは見かけましたね。文字列リテラルでは、文字列の値はプログラムにハードコードされます。
文字列リテラルは便利ですが、テキストを使いたいかもしれない場面全てに最適なわけではありません。一因は、
文字列リテラルが不変であることに起因します。別の原因は、コードを書く際に、全ての文字列値が判明するわけではないからです:
例えば、ユーザ入力を受け付け、それを保持したいとしたらどうでしょうか？このような場面用に、Rustには、
2種類目の文字列型、`String`型があります。この型はヒープにメモリを確保するので、
コンパイル時にはサイズが不明なテキストも保持することができるのです。`from`関数を使用して、
文字列リテラルから`String`型を生成できます。以下のように:

```rust
let s = String::from("hello");
```

<!-- The double colon (`::`) is an operator that allows us to namespace this -->
<!-- particular `from` function under the `String` type rather than using some sort -->
<!-- of name like `string_from`. We’ll discuss this syntax more in the “Method -->
<!-- Syntax” section of Chapter 5 and when we talk about namespacing with modules in -->
<!-- "Module DEfinitions" in Chapter 7. -->

この二重コロンは、`string_from`などの名前を使うのではなく、
`String`型直下の`from`関数を特定する働きをする演算子です。この記法について詳しくは、
第5章の「メソッド記法」節と、第7章の「モジュール定義」でモジュールを使った名前空間分けについて話をするときに議論します。

<!-- This kind of string *can* be mutated: -->

この種の文字列は、可変化することが*できます*:

<!-- ```rust -->
<!-- let mut s = String::from("hello"); -->

<!-- s.push_str(", world!"); // push_str() appends a literal to a String -->

<!-- println!("{}", s); // This will print `hello, world!` -->
<!-- ``` -->

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

println!("{}", s); // これは`hello, world!`と出力する
```

<!-- So, what’s the difference here? Why can `String` be mutated but literals -->
<!-- cannot? The difference is how these two types deal with memory. -->

では、ここでの違いは何でしょうか？なぜ、`String`型は可変化できるのに、リテラルはできないのでしょうか？
違いは、これら二つの型がメモリを扱う方法にあります。

<!-- ### Memory and Allocation -->

### メモリと確保

<!-- In the case of a string literal, we know the contents at compile time, so the -->
<!-- text is hardcoded directly into the final executable. This is why string -->
<!-- literals are fast and efficient. But these properties only come from the string -->
<!-- literal's immutability. Unfortunately, we can’t put a blob of memory into the -->
<!-- binary for each piece of text whose size is unknown at compile time and whose -->
<!-- size might change while running the program. -->

文字列リテラルの場合、中身はコンパイル時に判明しているので、テキストは最終的なバイナリファイルに直接ハードコードされます。
このため、文字列リテラルは、高速で効率的になるのです。しかし、これらの特性は、
その文字列リテラルの不変性にのみ端を発するものです。残念なことに、コンパイル時にサイズが不明だったり、
プログラム実行に合わせてサイズが可変なテキスト片用に一塊のメモリをバイナリに確保しておくことは不可能です。

<!-- With the `String` type, in order to support a mutable, growable piece of text, -->
<!-- we need to allocate an amount of memory on the heap, unknown at compile time, -->
<!-- to hold the contents. This means: -->

`String`型では、可変かつ伸長可能なテキスト破片をサポートするために、コンパイル時には不明な量のメモリを
ヒープに確保して内容を保持します。つまり:

<!-- * The memory must be requested from the operating system at runtime. -->
<!-- * We need a way of returning this memory to the operating system when we’re -->
<!-- done with our `String`. -->

* メモリは、実行時にOSに要求される。
* `String`型を使用し終わったら、OSにこのメモリを返還する方法が必要である。

<!-- That first part is done by us: when we call `String::from`, its implementation -->
<!-- requests the memory it needs. This is pretty much universal in programming -->
<!-- languages. -->

この最初の部分は、すでにしています: `String::from`関数を呼んだら、その実装が必要なメモリを要求するのです。
これは、プログラミング言語において、極めて普遍的です。

<!-- However, the second part is different. In languages with a *garbage collector -->
<!-- (GC)*, the GC keeps track and cleans up memory that isn’t being used anymore, -->
<!-- and we don’t need to think about it. Without a GC, it’s our responsibility to -->
<!-- identify when memory is no longer being used and call code to explicitly return -->
<!-- it, just as we did to request it. Doing this correctly has historically been a -->
<!-- difficult programming problem. If we forget, we’ll waste memory. If we do it -->
<!-- too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. -->
<!-- We need to pair exactly one `allocate` with exactly one `free`. -->

<!-- かっこがあると、*が機能しないようなので、(GC)の部分には指定していません -->

しかしながら、2番目の部分は異なります。*ガベージコレクタ*(GC)付きの言語では、GCがこれ以上、
使用されないメモリを検知して片付けるため、プログラマは、そのことを考慮する必要はありません。
GCがないなら、メモリがもう使用されないことを見計らって、明示的に返還するコードを呼び出すのは、
プログラマの責任になります。ちょうど要求の際にしたようにですね。これを正確にすることは、
歴史的にも難しいプログラミング問題の一つであり続けています。もし、忘れていたら、メモリを無駄にします。
タイミングが早すぎたら、無効な変数を作ってしまいます。2回解放してしまっても、バグになるわけです。
`allocate`と`free`は完璧に1対1対応にしなければならないのです。

<!-- Rust takes a different path: the memory is automatically returned once the -->
<!-- variable that owns it goes out of scope. Here’s a version of our scope example -->
<!-- from Listing 4-1 using a `String` instead of a string literal: -->

Rustは、異なる道を歩んでいます: ひとたび、メモリを所有している変数がスコープを抜けたら、
メモリは自動的に返還されます。こちらの例は、
リスト4-1のスコープ例を文字列リテラルから`String`型を使うものに変更したバージョンになります:

<!-- ```rust -->
<!-- { -->
<!--    let s = String::from("hello"); // s is valid from this point forward -->

<!--    // do stuff with s -->
<!--}                                  // this scope is now over, and s is no -->
<!--                                   // longer valid -->
<!-- ``` -->

```rust
{
    let s = String::from("hello"); // sはここから有効になる

    // sで作業をする
}                                  // このスコープはここでおしまい。sは
                                   // もう有効ではない
```

<!-- There is a natural point at which we can return the memory our `String` needs -->
<!-- to the operating system: when `s` goes out of scope. When a variable goes out -->
<!-- of scope, Rust calls a special function for us. This function is called `drop`, -->
<!-- and it’s where the author of `String` can put the code to return the memory. -->
<!-- Rust calls `drop` automatically at the closing curly bracket. -->

`String`型が必要とするメモリをOSに返還することが自然な地点があります: `s`変数がスコープを抜ける時です。
変数がスコープを抜ける時、Rustは特別な関数を呼んでくれます。この関数は、`drop`と呼ばれ、
ここに`String`型の書き手はメモリ返還するコードを配置することができます。Rustは、閉じ波括弧で自動的に`drop`関数を呼び出します。

<!-- Note: In C++, this pattern of deallocating resources at the end of an item's -->
<!-- lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*. -->
<!-- The `drop` function in Rust will be familiar to you if you’ve used RAII -->
<!-- patterns. -->

> 注釈: C++では、要素の生存期間の終了地点でリソースを解放するこのパターンを時に、
> *RAII*(Resource Aquisition Is Initialization: リソースの獲得は、初期化である)と呼んだりします。
> Rustの`drop`関数は、あなたがRAIIパターンを使ったことがあれば、馴染み深いものでしょう。

<!-- This pattern has a profound impact on the way Rust code is written. It may seem -->
<!-- simple right now, but the behavior of code can be unexpected in more -->
<!-- complicated situations when we want to have multiple variables use the data -->
<!-- we’ve allocated on the heap. Let’s explore some of those situations now. -->

このパターンは、Rustコードの書かれ方に甚大な影響をもたらします。現状は簡単そうに見えるかもしれませんが、
ヒープ上に確保されたデータを複数の変数に使用させるようなもっと複雑な場面では、コードの振る舞いは、
予期しないものになる可能性もあります。これから、そのような場面を掘り下げてみましょう。

<!-- #### Ways Variables and Data Interact: Move -->

#### 変数とデータの相互作用法: ムーブ

<!-- Multiple variables can interact with the same data in different ways in Rust. -->
<!-- Let’s look at an example using an integer in Listing 4-2. -->

Rustにおいては、複数の変数が同じデータに対して異なる手段で相互作用することができます。
整数を使用したリスト4-2の例を見てみましょう。

```rust
let x = 5;
let y = x;
```

<!-- <span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span> -->

<span class="caption">リスト4-2: 変数`x`の整数値を`y`に代入する</span>

<!-- We can probably guess what this is doing: “bind the value `5` to `x`; then make -->
<!-- a copy of the value in `x` and bind it to `y`.” We now have two variables, `x` -->
<!-- and `y`, and both equal `5`. This is indeed what is happening, because integers -->
<!-- are simple values with a known, fixed size, and these two `5` values are pushed -->
<!-- onto the stack. -->

もしかしたら、何をしているのか予想することができるでしょう: 
「値`5`を`x`に束縛する; それから`x`の値をコピーして`y`に束縛する。」これで、
二つの変数(`x`と`y`)が存在し、両方、値は`5`になりました。これは確かに起こっている現象を説明しています。
なぜなら、整数は既知の固定サイズの単純な値で、これら二つの`5`という値は、スタックに積まれるからです。

<!-- Now let’s look at the `String` version: -->

では、`String`バージョンを見ていきましょう:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

<!-- This looks very similar to the previous code, so we might assume that the way -->
<!-- it works would be the same: that is, the second line would make a copy of the -->
<!-- value in `s1` and bind it to `s2`. But this isn’t quite what happens. -->

このコードは先ほどのコードに酷似していますので、動作方法も同じだと思い込んでしまうかもしれません:
要するに、2行目で`s1`の値をコピーし、`s2`に束縛するということです。ところが、
これは全く起こることを言い当てていません。

<!-- Take a look at Figure 4-1 to see what is happening to `String` under the -->
<!-- covers. A `String` is made up of three parts, shown on the left: a pointer to -->
<!-- the memory that holds the contents of the string, a length, and a capacity. -->
<!-- This group of data is stored on the stack. On the right is the memory on the -->
<!-- heap that holds the contents. -->

図4-1を見て、ベールの下で`String`に何が起きているかを確かめてください。
`String`型は、左側に示されているように、3つの部品でできています: 
文字列の中身を保持するメモリへのポインタと長さ、そして、許容量です。この種のデータは、スタックに保持されます。
右側には、中身を保持したヒープ上のメモリがあります。 

<!-- <img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" /> -->

<img alt="メモリ上の文字列" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<!-- <span class="caption">Figure 4-1: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span> -->

<span class="caption">図4-1: `s1`に束縛された`"hello"`という値を保持する`String`のメモリ上の表現</span>

<!-- The length is how much memory, in bytes, the contents of the `String` is -->
<!-- currently using. The capacity is the total amount of memory, in bytes, that the -->
<!-- `String` has received from the operating system. The difference between length -->
<!-- and capacity matters, but not in this context, so for now, it’s fine to ignore -->
<!-- the capacity. -->

長さは、`String`型の中身が現在使用しているメモリ量をバイトで表したものです。許容量は、
`String`型がOSから受け取った全メモリ量をバイトで表したものです。長さと許容量の違いは問題になることですが、
この文脈では違うので、とりあえずは、許容量を無視しても構わないでしょう。 

<!-- When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the -->
<!-- pointer, the length, and the capacity that are on the stack. We do not copy the -->
<!-- data on the heap that the pointer refers to. In other words, the data -->
<!-- representation in memory looks like Figure 4-2. -->

`s1`を`s2`に代入すると、`String`型のデータがコピーされます。つまり、スタックにあるポインタ、長さ、
許容量をコピーするということです。ポインタが指すヒープ上のデータはコピーしません。言い換えると、
メモリ上のデータ表現は図4-4のようになるということです。

<!-- <img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" /> -->

<img alt="同じ値を指すs1とs2" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<!-- <span class="caption">Figure 4-2: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span> -->

<span class="caption">図4-2: `s1`のポインタ、長さ、許容量のコピーを保持する変数`s2`のメモリ上での表現</span>

<!-- The representation does *not* look like Figure 4-3, which is what memory would -->
<!-- look like if Rust instead copied the heap data as well. If Rust did this, the -->
<!-- operation `s2 = s1` could be very expensive in terms of runtime performance if -->
<!-- the data on the heap ware large. -->

メモリ上の表現は、図4-5のようにはなり*ません*。これは、
Rustが代わりにヒープデータもコピーするという選択をしていた場合のメモリ表現ですね。Rustがこれをしていたら、
ヒープ上のデータが大きい時に`s2 = s1`という処理の実行時性能がとても悪くなっていた可能性があるでしょう。

<!-- <img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" /> -->

<img alt="2箇所へのs1とs2" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<!-- <span class="caption">Figure 4-3: Another possibility of what `s2 = s1` might
do if Rust copied the heap data as well</span> -->

<span class="caption">図4-3: Rustがヒープデータもコピーしていた場合に`s2 = s1`という処理が行なった可能性のあること</span>

<!-- Earlier, we said that when a variable goes out of scope, Rust automatically -->
<!-- calls the `drop` function and cleans up the heap memory for that variable. But -->
<!-- Figure 4-2 shows both data pointers pointing to the same location. This is a -->
<!-- problem: when `s2` and `s1` go out of scope, they will both try to free the -->
<!-- same memory. This is known as a *double free* error and is one of the memory -->
<!-- safety bugs we mentioned previously. Freeing memory twice can lead to memory -->
<!-- corruption, which can potentially lead to security vulnerabilities. -->

先ほど、変数がスコープを抜けたら、Rustは自動的に`drop`関数を呼び出し、
その変数が使っていたヒープメモリを片付けると述べました。しかし、図4-4は、
両方のデータポインタが同じ場所を指していることを示しています。これは問題です: `s2`と`s1`がスコープを抜けたら、
両方とも同じメモリを解放しようとします。これは*二重解放*エラーとして知られ、以前触れたメモリ安全性上のバグの一つになります。
メモリを2回解放することは、メモリの退廃につながり、さらにセキュリティ上の脆弱性を生む可能性があります。

<!-- To ensure memory safety, there’s one more detail to what happens in this -->
<!-- situation in Rust. Instead of trying to copy the allocated memory, Rust -->
<!-- considers `s1` to no longer be valid and, therefore, Rust doesn’t need to free -->
<!-- anything when `s1` goes out of scope. Check out what happens when you try to -->
<!-- use `s1` after `s2` is created, it won't work: -->

<!-- この最初の文は、こなれた日本語にしにくい -->

メモリ安全性を保証するために、Rustにおいてこの場面で知っておきたい起こる事の詳細がもう一つあります。
確保されたメモリをコピーしようとする代わりに、コンパイラは、`s1`が最早有効ではないと考え、
故に`s1`がスコープを抜けた際に何も解放する必要がなくなるわけです。`s2`の生成後に`s1`を使用しようとしたら、
どうなるかを確認してみましょう。動かないでしょう:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

<!-- You’ll get an error like this because Rust prevents you from using the -->
<!-- invalidated reference: -->

コンパイラが無効化された参照は使用させてくれないので、以下のようなエラーが出るでしょう:

```text
error[E0382]: use of moved value: `s1`
              (ムーブされた値の使用: `s1`)
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |                               (ムーブ後にここで使用されています)
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
    (注釈: ムーブが起きたのは、`s1`が`std::string::String`という
    `Copy`トレイトを実装していない型だからです)
```

<!-- If you’ve heard the terms “shallow copy” and “deep copy” while working with -->
<!-- other languages, the concept of copying the pointer, length, and capacity -->
<!-- without copying the data probably sounds like a shallow copy. But -->
<!-- because Rust also invalidates the first variable, instead of being called a -->
<!-- shallow copy, it’s known as a *move*. In this example, we would say that `s1` -->
<!-- was *moved* into `s2`. So what actually happens is shown in Figure 4-4. -->

他の言語を触っている間に"shallow copy"と"deep copy"という用語を耳にしたことがあるなら、
データのコピーなしにポインタと長さ、許容量をコピーするという概念は、shallow copyのように思えるかもしれません。
ですが、コンパイラは最初の変数をも無効化するので、shallow copyと呼ばれる代わりに、
ムーブとして知られているわけです。この例では、`s1`は`s2`に*ムーブ*されたと表現するでしょう。
以上より、実際に起きることを図4-4に示してみました。

<!-- <img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" /> -->

<img alt="s2にムーブされたs1" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<!-- <span class="caption">Figure 4-4: Representation in memory after `s1` has been
invalidated</span> -->

<span class="caption">図4-4: `s1`が無効化された後のメモリ表現</span>

<!-- That solves our problem! With only `s2` valid, when it goes out of scope, it -->
<!-- alone will free the memory, and we’re done. -->

これにて一件落着です。`s2`だけが有効なので、スコープを抜けたら、それだけがメモリを解放して、
終わりになります。

<!-- In addition, there’s a design choice that’s implied by this: Rust will never -->
<!-- automatically create “deep” copies of your data. Therefore, any *automatic* -->
<!-- copying can be assumed to be inexpensive in terms of runtime performance. -->

付け加えると、これにより暗示される設計上の選択があります: Rustでは、
自動的にデータの"deep copy"が行われることは絶対にないわけです。それ故に、あらゆる*自動*コピーは、実行時性能の観点で言うと、
悪くないと考えてよいことになります。

<!-- #### Ways Variables and Data Interact: Clone -->

#### 変数とデータの相互作用法: クローン

<!-- If we *do* want to deeply copy the heap data of the `String`, not just the -->
<!-- stack data, we can use a common method called `clone`. We’ll discuss method -->
<!-- syntax in Chapter 5, but because methods are a common feature in many -->
<!-- programming languages, you’ve probably seen them before. -->

仮に、スタック上のデータだけでなく、本当に`String`型のヒープデータのdeep copyが必要ならば、
`clone`と呼ばれるよくあるメソッドを使うことができます。メソッド記法については第5章で議論しますが、
メソッドは多くのプログラミング言語に見られる機能なので、以前に見かけたこともあるんじゃないでしょうか。

<!-- Here’s an example of the `clone` method in action: -->

こちらで、`clone`メソッドの稼働例をご覧ください:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

<!-- This works just fine and explicitly produces the behavior shown in Figure 4-3, -->
<!-- where the heap data *does* get copied. -->

これは単純にうまく動き、図4-3で示した動作を明示的に生み出します。ここでは、
ヒープデータが*実際に*コピーされています。

<!-- When you see a call to `clone`, you know that some arbitrary code is being -->
<!-- executed and that code may be expensive. It’s a visual indicator that something -->
<!-- different is going on. -->

`clone`メソッドの呼び出しを見かけたら、何らかの任意のコードが実行され、その実行コストは高いと把握できます。
何か違うことが起こっているなと見た目でわかるわけです。

<!-- #### Stack-Only Data: Copy -->

#### スタックのみのデータ: コピー

<!-- There’s another wrinkle we haven’t talked about yet. This code using integers, -->
<!-- part of which was shown in Listing 4-2, works and is valid: -->

まだ話題にしていない別のしわ(`脚注`: 「気になるもの」程度の意味と思われる)の話があります。
この整数を使用したコードは、一部をリスト4-2で示しましたが、うまく動作し、有効です:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

<!-- But this code seems to contradict what we just learned: we don’t have a call to -->
<!-- `clone`, but `x` is still valid and wasn’t moved into `y`. -->

ですが、このコードは一見、今学んだことと矛盾しているように見えます:
`clone`メソッドの呼び出しがないのに、`x`は有効で、`y`にムーブされませんでした。

<!-- The reason is that types such as integers that have a known size at compile -->
<!-- time are stored entirely on the stack, so copies of the actual values are quick -->
<!-- to make. That means there’s no reason we would want to prevent `x` from being -->
<!-- valid after we create the variable `y`. In other words, there’s no difference -->
<!-- between deep and shallow copying here, so calling `clone` wouldn’t do anything -->
<!-- differently from the usual shallow copying and we can leave it out. -->

その理由は、整数のようなコンパイル時に既知のサイズを持つ型は、スタック上にすっぽり保持されるので、
実際の値をコピーするのも高速だからです。これは、変数`y`を生成した後にも`x`を無効化したくなる理由がないことを意味します。
換言すると、ここでは、shallow copyとdeep copyの違いがないことになり、
`clone`メソッドを呼び出しても、一般的なshallow copy以上のことをしなくなり、
そのまま放置しておけるということです。

<!-- Rust has a special annotation called the `Copy` trait that we can place on -->
<!-- types like integers that are stored on the stack (we’ll talk more about traits -->
<!-- in Chapter 10). If a type has the `Copy` trait, an older variable is still -->
<!-- usable after assignment. Rust won’t let us annotate a type with the `Copy` -->
<!-- trait if the type, or any of its parts, has implemented the `Drop` trait. If -->
<!-- the type needs something special to happen when the value goes out of scope and -->
<!-- we add the `Copy` annotation to that type, we’ll get a compile-time error. To -->
<!-- learn about how to add the `Copy` annotation to your type, see "Derivable -->
<!-- Traits" in Appendix C. -->

Rustには`Copy`トレイトと呼ばれる特別な注釈があり、
整数のようなスタックに保持される型に対して配置することができます(トレイトについては第10章でもっと詳しく話します)。
型が`Copy`トレイトに適合していれば、代入後も古い変数が使用可能になります。コンパイラは、
型やその一部分でも`Drop`トレイトを実装している場合、`Copy`トレイトによる注釈をさせてくれません。
型の値がスコープを外れた時に何か特別なことを起こす必要がある場合に、`Copy`注釈を追加すると、コンパイルエラーが出ます。
型に`Copy`注釈をつける方法について学ぶには、付録Cの「継承可能トレイト」をご覧ください。

<!-- So what types are `Copy`? You can check the documentation for the given type to -->
<!-- be sure, but as a general rule, any group of simple scalar values can be -->
<!-- `Copy`, and nothing that requires allocation or is some form of resource is -->
<!-- `Copy`. Here are some of the types that are `Copy`: -->

では、どの型が`Copy`なのでしょうか？ある型について、ドキュメントをチェックすればいいのですが、
一般規則として、単純なスカラー値の集合は何でも`Copy`であり、メモリ確保が必要だったり、
何らかの形態のリソースだったりするものは`Copy`ではありません。ここに`Copy`の型を並べておきます。

<!-- * All the integer types, like `u32`. -->
<!-- * The Boolean type, `bool`, with values `true` and `false`. -->
<!-- * All the floating point types, like `f64`. -->
<!-- * The character type, `char`. -->
<!-- * Tuples, but only if they contain types that are also `Copy`. For example -->
<!-- `(i32, i32)` is `Copy`, but `(i32, String)` is not. -->

* あらゆる整数型。`u32`など。
* 論理値型、`bool`、`true`と`false`という値がある。
* あらゆる浮動小数点型、`f64`など。
* 文字型、`char`。
* タプル。ただ、`Copy`の型だけを含む場合。例えば、`(i32, i32)`は`Copy`だが、
`(i32, String)`は違う。

<!-- ### Ownership and Functions -->

### 所有権と関数

<!-- The semantics for passing a value to a function are similar to those for -->
<!-- assigning a value to a variable. Passing a variable to a function will move or -->
<!-- copy, just as assignment does. Listing 4-3 has an example with some annotations -->
<!-- showing where variables go into and out of scope. -->

意味論的に、関数に値を渡すことと、値を変数に代入することは似ています。関数に変数を渡すと、
代入のようにムーブやコピーされます。リスト4-7は変数がスコープに入ったり、
抜けたりする地点について注釈してある例です。

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

<!-- ```rust -->
<!-- fn main() { -->
<!--    let s = String::from("hello");  // s comes into scope -->

<!--    takes_ownership(s);             // s's value moves into the function... -->
<!--                                    // ... and so is no longer valid here -->

<!--    let x = 5;                      // x comes into scope -->

<!--    makes_copy(x);                  // x would move into the function, -->
<!--                                    // but i32 is Copy, so it’s okay to still -->
<!--                                    // use x afterward -->

<!-- } // Here, x goes out of scope, then s. But since s's value was moved, nothing -->
<!--  // special happens. -->

<!-- fn takes_ownership(some_string: String) { // some_string comes into scope. -->
<!--    println!("{}", some_string); -->
<!-- } // Here, some_string goes out of scope and `drop` is called. The backing -->
<!--  // memory is freed. -->

<!-- fn makes_copy(some_integer: i32) { // some_integer comes into scope -->
<!--    println!("{}", some_integer); -->
<!-- } // Here, some_integer goes out of scope. Nothing special happens. -->
<!-- ``` -->

```rust
fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // ここでxがスコープを抜け、sも。だけど、sの値はムーブされてるので、何も特別なことはない。
  //

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
```

<!-- <span class="caption">Listing 4-3: Functions with ownership and scope
annotated</span> -->

<span class="caption">リスト4-3: 所有権とスコープが注釈された関数群</span>

<!-- If we tried to use `s` after the call to `takes_ownership`, Rust would throw a -->
<!-- compile-time error. These static checks protect us from mistakes. Try adding -->
<!-- code to `main` that uses `s` and `x` to see where you can use them and where -->
<!-- the ownership rules prevent you from doing so. -->

`takes_ownership`の呼び出し後に`s`を呼び出そうとすると、コンパイラは、コンパイルエラーを投げるでしょう。
これらの静的チェックにより、ミスを犯さないでいられます。`s`や`x`を使用するコードを`main`に追加してみて、
どこで使えて、そして、所有権規則により、どこで使えないかを確認してください。

<!-- ### Return Values and Scope -->

### 戻り値とスコープ

<!-- Returning values can also transfer ownership. Listing 4-4 is an example with -->
<!-- similar annotations to those in Listing 4-3. -->

値を返すことでも、所有権は移動します。リスト4-4は、リスト4-3と似た注釈のついた例です。

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

<!-- ```rust -->
<!-- fn main() { -->
<!--    let s1 = gives_ownership();         // gives_ownership moves its return -->
<!--                                        // value into s1 -->

<!--    let s2 = String::from("hello");     // s2 comes into scope -->

<!--    let s3 = takes_and_gives_back(s2);  // s2 is moved into -->
<!--                                        // takes_and_gives_back, which also -->
<!--                                        // moves its return value into s3 -->
<!-- } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was -->
<!--  // moved, so nothing happens. s1 goes out of scope and is dropped. -->

<!-- fn gives_ownership() -> String {             // gives_ownership will move its -->
<!--                                             // return value into the function -->
<!--                                             // that calls it -->

<!--    let some_string = String::from("hello"); // some_string comes into scope -->

<!--    some_string                              // some_string is returned and -->
<!--                                             // moves out to the calling -->
<!--                                             // function -->
<!-- } -->

<!-- // takes_and_gives_back will take a String and return one. -->
<!-- fn takes_and_gives_back(a_string: String) -> String { // a_string comes into -->
<!--                                                       // scope -->

<!--    a_string  // a_string is returned and moves out to the calling function -->
<!-- } -->
<!-- ``` -->

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
```

<!-- <span class="caption">Listing 4-4: Transferring ownership of return -->
<!-- values</span> -->

<span class="caption">リスト4-4: 戻り値の所有権を移動する</span>

<!-- The ownership of a variable follows the same pattern every time: assigning a -->
<!-- value to another variable moves it. When a variable that includes data on the -->
<!-- heap goes out of scope, the value will be cleaned up by `drop` unless the data -->
<!-- has been moved to be owned by another variable. -->

変数の所有権は、毎回同じパターンを辿っています: 別の変数に値を代入すると、ムーブされます。
ヒープにデータを含む変数がスコープを抜けると、データが別の変数に所有されるようムーブされていない限り、
`drop`により片付けられるでしょう。

<!-- Taking ownership and then returning ownership with every function is a bit -->
<!-- tedious. What if we want to let a function use a value but not take ownership? -->
<!-- It’s quite annoying that anything we pass in also needs to be passed back if we -->
<!-- want to use it again, in addition to any data resulting from the body of the -->
<!-- function that we might want to return as well. -->

所有権を得ては返すを全ての関数でしていたら、ちょっとめんどくさいですね。関数に値を使わせたいけど、
所有権は保持させたくない場合はどうすればいいのでしょうか？
返したいと思うかもしれない関数本体で発生したあらゆるデータとともに再利用したかったら、渡されたものをまた返さなきゃいけないのは、
非常に煩わしいことです。

<!-- It’s possible to return multiple values using a tuple, as shown in Listing 4-5. -->

タプルで、複数の値を返すことは可能です。リスト4-5のようにね。

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

<!-- ```rust -->
<!-- fn main() { -->
<!--    let s1 = String::from("hello"); -->

<!--    let (s2, len) = calculate_length(s1); -->

<!--    println!("The length of '{}' is {}.", s2, len); -->
<!-- } -->

<!-- fn calculate_length(s: String) -> (String, usize) { -->
<!--    let length = s.len(); // len() returns the length of a String -->

<!--    (s, length) -->
<!-- } -->
<!-- ``` -->

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}
```

<!-- <span class="caption">Listing 4-5: Returning ownership of parameters</span> -->

<span class="caption">リスト4-5: 引数の所有権を返す</span>

<!-- But this is too much ceremony and a lot of work for a concept that should be -->
<!-- common. Luckily for us, Rust has a feature for this concept, called -->
<!-- *references*. -->

でも、これでは、大袈裟すぎますし、ありふれているはずの概念に対して、作業量が多すぎます。
私たちにとって幸運なことに、Rustにはこの概念に対する機能があり、*参照*と呼ばれます。
