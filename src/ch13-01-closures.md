<!-- Old heading. Do not remove or links may break. -->
<!--
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
-->

<!--
## Closures: Anonymous Functions that Capture Their Environment
-->

## クロージャ: 環境をキャプチャする匿名関数

<!--
Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure elsewhere to evaluate it in a different context. Unlike
functions, closures can capture values from the scope in which they’re defined.
We’ll demonstrate how these closure features allow for code reuse and behavior
customization.
-->

Rustのクロージャは、変数に保存したり、引数として他の関数に渡すことのできる匿名関数です。
ある場所でクロージャを生成した後、他の場所でクロージャを呼び出して、別の文脈で評価することができます。
クロージャは関数と異なり、定義されたスコープから値をキャプチャすることができます。
これらのクロージャの特性を使用して、どのようにコードの再利用や振る舞いのカスタマイズができるか、実際に試してみましょう。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
-->

<!--
### Capturing the Environment with Closures
-->

### クロージャで環境をキャプチャする

<!--
We’ll first examine how we can use closures to capture values from the
environment they’re defined in for later use. Here’s the scenario: Every so
often, our t-shirt company gives away an exclusive, limited-edition shirt to
someone on our mailing list as a promotion. People on the mailing list can
optionally add their favorite color to their profile. If the person chosen for
a free shirt has their favorite color set, they get that color shirt. If the
person hasn’t specified a favorite color, they get whatever color the company
currently has the most of.
-->

まず、クロージャから定義された環境から後で使用するために値をキャプチャするために、
どのようにクロージャを使用することができるのか、確かめてみましょう。
このようなシナリオを考えます: 私たちのTシャツ会社は時折プロモーションとして、
メーリングリスト上の誰かに、その人専用の限定版のシャツを配ります。
メーリングリスト上の人たちは、自身のプロファイルにお気に入りの色を任意で追加することができます。
シャツ無料対象に選ばれた人がお気に入りの色を設定している場合は、その色のシャツをもらえます。
その人がお気に入りの色を指定していない場合は、会社が現在最も多く持っている色をもらえます。

<!--
There are many ways to implement this. For this example, we’re going to use an
enum called `ShirtColor` that has the variants `Red` and `Blue` (limiting the
number of colors available for simplicity). We represent the company’s
inventory with an `Inventory` struct that has a field named `shirts` that
contains a `Vec<ShirtColor>` representing the shirt colors currently in stock.
The method `giveaway` defined on `Inventory` gets the optional shirt
color preference of the free shirt winner, and returns the shirt color the
person will get. This setup is shown in Listing 13-1:
-->

これを実装する方法はたくさんあります。この例では、`Red`と`Blue`の列挙子を持つ、
`ShirtColor`というenumを使用します(簡潔さのために入手可能な色の数は制限しています)。
会社の棚卸資産は、`Vec<ShirtColor>`を含み現在在庫にあるシャツの色を表現する`shirts`というフィールドを持つ、
`Inventory`構造体で表現します。`Inventory`上に定義された`giveaway`メソッドは、
シャツ無料対象者の任意選択のシャツの色の好みを取得し、その人がもらえるシャツの色を返します。
この構築をリスト13-1に示します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<!--
<span class="caption">Listing 13-1: Shirt company giveaway situation</span>
-->

<span class="caption">リスト13-1: シャツ会社の無料プレゼントのシチュエーション</span>

<!--
The `store` defined in `main` has two blue shirts and one red shirt remaining
to distribute for this limited-edition promotion. We call the `giveaway` method
for a user with a preference for a red shirt and a user without any preference.
-->

`main`で定義されている`store`には、この限定版プロモーションのために残された、
青いシャツが2着と赤いシャツが1着あります。赤いシャツが好みのユーザと、好みがないユーザを対象として、
`giveaway`メソッドを呼び出しています。

<!--
Again, this code could be implemented in many ways, and here, to focus on
closures, we’ve stuck to concepts you’ve already learned except for the body of
the `giveaway` method that uses a closure. In the `giveaway` method, we get the
user preference as a parameter of type `Option<ShirtColor>` and call the
`unwrap_or_else` method on `user_preference`. The [`unwrap_or_else` method on
`Option<T>`][unwrap-or-else] is defined by the standard library.
It takes one argument: a closure without any arguments that returns a value `T`
(the same type stored in the `Some` variant of the `Option<T>`, in this case
`ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else`
returns the value from within the `Some`. If the `Option<T>` is the `None`
variant, `unwrap_or_else` calls the closure and returns the value returned by
the closure.
-->

繰り返しになりますが、このコードは多くの方法で実装することができます。
ここではクロージャに着目するために、すでに学習した概念に固執しています。
ただし、`giveaway`メソッドの本体はクロージャを使用していて、例外です。
`giveaway`メソッドでは、ユーザの好みを`Option<ShirtColor>`型の引数として取り、
`user_preference`に対して`unwrap_or_else`メソッドを呼び出します。
[`Option<T>`に対する`unwrap_or_else`メソッド][unwrap-or-else]は標準ライブラリで定義されています。
このメソッドは引数を1つ取ります: 値`T`(`Option<T>`の`Some`列挙子内に保存されているのと同じ型、
この場合は`ShirtColor`)を返す、引数のないクロージャです。
もし`Option<T>`が`Some`列挙子の場合、`unwrap_or_else`は`Some`の内側の値を返します。
もし`Option<T>`が`None`列挙子の場合、`unwrap_or_else`はクロージャを呼び出し、
そのクロージャによって返された値を返します。

<!--
We specify the closure expression `|| self.most_stocked()` as the argument to
`unwrap_or_else`. This is a closure that takes no parameters itself (if the
closure had parameters, they would appear between the two vertical bars). The
body of the closure calls `self.most_stocked()`. We’re defining the closure
here, and the implementation of `unwrap_or_else` will evaluate the closure
later if the result is needed.
-->

`unwrap_or_else`への引数としてクロージャ式`|| self.most_stocked()`を指定しています。
これは引数を取らないクロージャです(クロージャが引数を持つ場合、それらは2本の縦線の間に現れます)。
クロージャの本体は`self.most_stocked()`を呼び出します。
クロージャはここで定義され、`unwrap_or_else`の実装詳細がクロージャの結果を必要とするなら、
このクロージャを呼び出すでしょう。

<!--
Running this code prints:
-->

このコードを実行すると、以下が出力されます:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

<!--
One interesting aspect here is that we’ve passed a closure that calls
`self.most_stocked()` on the current `Inventory` instance. The standard library
didn’t need to know anything about the `Inventory` or `ShirtColor` types we
defined, or the logic we want to use in this scenario. The closure captures an
immutable reference to the `self` `Inventory` instance and passes it with the
code we specify to the `unwrap_or_else` method. Functions, on the other hand,
are not able to capture their environment in this way.
-->

ここで興味深い側面は、現在の`Inventory`インスタンスに対して`self.most_stocked()`を呼び出すクロージャを渡していることです。
標準ライブラリは、プログラマが定義した`Inventory`や`ShirtColor`型や、
このシナリオで使用したいロジックについては何も知りません。
クロージャは`self` `Inventory`インスタンスへの不変参照をキャプチャし、
この不変参照をプログラマが指定したコードとともに`unwrap_or_else`メソッドに渡します。
一方で関数は、このように環境をキャプチャすることができません。

<!--
### Closure Type Inference and Annotation
-->

### クロージャの型推論と注釈

<!--
There are more differences between functions and closures. Closures don’t
usually require you to annotate the types of the parameters or the return value
like `fn` functions do. Type annotations are required on functions because the
types are part of an explicit interface exposed to your users. Defining this
interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. Closures, on the other hand, aren’t used
in an exposed interface like this: they’re stored in variables and used without
naming them and exposing them to users of our library.
-->

関数とクロージャの違いは他にもあります。
クロージャでは通常、`fn`関数のように引数の型や戻り値の型を注釈する必要はありません。
関数では、型注釈は必要です。型はユーザに露出する明示的なインターフェイスの一部だからです。
このインターフェイスを堅実に定義することは、関数が使用したり、
返したりする値の型についてみんなが合意していることを保証するために重要なのです。
一方で、クロージャはこのような露出するインターフェイスには使用されません: 変数に保存され、
名前付けしたり、ライブラリの使用者に晒されることなく、使用されます。

<!--
Closures are typically short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler can
infer the types of the parameters and the return type, similar to how it’s able
to infer the types of most variables (there are rare cases where the compiler
needs closure type annotations too).
-->

クロージャは典型的には短く、あらゆる任意の筋書きではなく、狭い文脈でのみ関係します。
このような限定された文脈内では、コンパイラは、多くの変数の型を推論できるのと似たように、
引数や戻り値の型を推論することができます (コンパイラがクロージャ型注釈を要求するレアケースもあります)。

<!--
As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for a closure would look like the definition
shown in Listing 13-2. In this example, we’re defining a closure and storing it
in a variable rather than defining the closure in the spot we pass it as an
argument as we did in Listing 13-1.
-->

本当に必要な以上に冗長になることと引き換えに、明示性と明瞭性を向上させたいなら、
変数に型注釈を加えることもできます;
クロージャに対する型注釈は、リスト13-2に示した定義のようになるでしょう。
この例では、リスト13-1でやったようにクロージャを定義したその場で引数として渡すのではなく、
クロージャを定義して変数に保存しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-2: Adding optional type annotations of the
parameter and return value types in the closure</span>
-->

<span class="caption">リスト13-2: クロージャの引数と戻り値の省略可能な型注釈を追加する</span>

<!--
With type annotations added, the syntax of closures looks more similar to the
syntax of functions. Here we define a function that adds 1 to its parameter and
a closure that has the same behavior, for comparison. We’ve added some spaces
to line up the relevant parts. This illustrates how closure syntax is similar
to function syntax except for the use of pipes and the amount of syntax that is
optional:
-->

型注釈を付け加えると、クロージャの記法は関数の記法とより似た見た目になっていきます。
以下は比較のために、引数に1を加える関数と、同じ振る舞いをするクロージャを定義しています。
空白を追加して、関連のある部分を並べています。これにより、縦棒の使用と省略可能な記法の量を除いて、
クロージャ記法が関数記法に似ているところを説明しています。

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

<!--
The first line shows a function definition, and the second line shows a fully
annotated closure definition. In the third line, we remove the type annotations
from the closure definition. In the fourth line, we remove the brackets, which
are optional because the closure body has only one expression. These are all
valid definitions that will produce the same behavior when they’re called. The
`add_one_v3` and `add_one_v4` lines require the closures to be evaluated to be
able to compile because the types will be inferred from their usage. This is
similar to `let v = Vec::new();` needing either type annotations or values of
some type to be inserted into the `Vec` for Rust to be able to infer the type.
-->

1行目が関数定義を示し、2行目がフルに注釈したクロージャ定義を示しています。
3行目ではクロージャ定義から型注釈を取り除いています。
4行目ではかっこを取り除いてます。クロージャの本体がただ1つの式からなるので、このかっこは省略可能です。
これらは全て、呼び出された時に同じ振る舞いになる合法な定義です。
型はその使い方から推論されるので、`add_one_v3`と`add_one_v4`行がコンパイルできるようになるためには、
それを評価する必要があります。
これは、コンパイラが型を推論できるようにするために、型注釈か`Vec`の中に挿入する何らかの型の値を必要とする、
`let v = Vec::new();`と似ています。

<!--
For closure definitions, the compiler will infer one concrete type for each of
their parameters and for their return value. For instance, Listing 13-3 shows
the definition of a short closure that just returns the value it receives as a
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition.
Because there are no type annotations, we can call the closure with any type,
which we’ve done here with `String` the first time. If we then try to call
`example_closure` with an integer, we’ll get an error.
-->

クロージャ定義では、コンパイラは引数それぞれと戻り値に対して、一つの具体的な型を推論します。
例えば、リスト13-3に引数として受け取った値を返すだけの短いクロージャの定義を示しました。
このクロージャは、この例での目的以外には有用ではありません。
この定義には、何も型注釈を加えていないことに注意してください。
型注釈が無いので、任意の型に対してこのクロージャを呼び出すことができ、
ここでは最初は`String`に対して呼び出しています。
その後整数に対して`example_closure`を呼び出そうとすると、エラーになります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 13-3: Attempting to call a closure whose types
are inferred with two different types</span>
-->

<span class="caption">リスト13-3: 2つの異なる型で型が推論されるクロージャの呼び出しを試みる</span>

<!--
The compiler gives us this error:
-->

コンパイラは、次のエラーを返します:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

<!--
The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error when we next try to use a different type with the same closure.
-->

`String`値で`example_closure`を呼び出した最初の時点で、
コンパイラは`x`とクロージャの戻り値の型を`String`と推論します。
そして、その型が`example_closure`のクロージャに閉じ込められ、
次に同じクロージャを異なる型で使用しようとすると、型エラーが出るのです。

<!--
### Capturing References or Moving Ownership
-->

### 参照をキャプチャするか、所有権を移動するか

<!--
Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership. The closure will decide
which of these to use based on what the body of the function does with the
captured values.
-->

クロージャは、3つの方法で環境から値をキャプチャでき、この方法は関数が引数を取れる3つの方法に直に対応します:
不変で借用する、可変で借用する、所有権を奪うです。クロージャは、
関数本体がキャプチャされた値を使って何をするかに基づいて、これらのうちどれを使用するかを決定します。

<!--
In Listing 13-4, we define a closure that captures an immutable reference to
the vector named `list` because it only needs an immutable reference to print
the value:
-->

リスト13-4では、`list`というベクタへの不変参照をキャプチャするクロージャが定義されます。
値を出力するためには不変参照しか必要としないからです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<!--
<span class="caption">Listing 13-4: Defining and calling a closure that
captures an immutable reference</span>
-->

<span class="caption">リスト13-4: 不変参照をキャプチャするクロージャを定義し、呼び出す</span>

<!--
This example also illustrates that a variable can bind to a closure definition,
and we can later call the closure by using the variable name and parentheses as
if the variable name were a function name.
-->

この例は、変数をクロージャ定義に束縛することができ、
その変数名が関数名であるかのように変数名と丸かっこを使用することで、
後でそのクロージャを呼び出すことができることも示しています。

<!--
Because we can have multiple immutable references to `list` at the same time,
`list` is still accessible from the code before the closure definition, after
the closure definition but before the closure is called, and after the closure
is called. This code compiles, runs, and prints:
-->

`list`への不変参照は複数同時に存在することができるので、`list`には、
クロージャ定義より前のコードからでも、クロージャ定義からクロージャ呼び出しまでの間のコードからでも、
クロージャ呼び出し後のコードからでも、アクセスすることができます。
このコードはコンパイルでき、実行でき、以下を出力します:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

<!--
Next, in Listing 13-5, we change the closure body so that it adds an element to
the `list` vector. The closure now captures a mutable reference:
-->

次にリスト13-5では、`list`ベクタへ要素を追加するようにクロージャ本体を変更します。
クロージャは可変参照をキャプチャするようになります:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<!--
<span class="caption">Listing 13-5: Defining and calling a closure that
captures a mutable reference</span>
-->

<span class="caption">リスト13-5: 可変参照をキャプチャするクロージャを定義し、呼び出す</span>

<!--
This code compiles, runs, and prints:
-->

このコードはコンパイルでき、実行でき、以下を出力します:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

<!--
Note that there’s no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: when `borrows_mutably` is defined, it captures a
mutable reference to `list`. We don’t use the closure again after the closure
is called, so the mutable borrow ends. Between the closure definition and the
closure call, an immutable borrow to print isn’t allowed because no other
borrows are allowed when there’s a mutable borrow. Try adding a `println!`
there to see what error message you get!
-->

`borrows_mutably`クロージャの定義と呼び出しの間の`println!`を消したことに注意してください:
`borrows_mutably`が定義されるときに、`list`への可変参照がキャプチャされます。
クロージャを呼び出した後は、クロージャを再度使用していないので、可変借用は終了します。
可変借用が存在するときに他の借用は許可されないので、
クロージャ定義とクロージャ呼び出しの間に、出力するための不変借用は許可されません。
`println!`を追加してみて、どんなエラーメッセージが得られるか確認してみてください！

<!--
If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn’t strictly need
ownership, you can use the `move` keyword before the parameter list.
-->

クロージャの本体が厳密には所有権を必要としない場合であっても、
クロージャにそれが使用する値の所有権を環境から奪うように強制したい場合は、
引数リストの前で`move`キーワードを使用することができます。

<!--
This technique is mostly useful when passing a closure to a new thread to move
the data so that it’s owned by the new thread. We’ll discuss threads and why
you would want to use them in detail in Chapter 16 when we talk about
concurrency, but for now, let’s briefly explore spawning a new thread using a
closure that needs the `move` keyword. Listing 13-6 shows Listing 13-4 modified
to print the vector in a new thread rather than in the main thread:
-->

この手法は主に、新しいスレッドにクロージャを渡すときに、
データが新しいスレッドに所有されるようにムーブするために有用です。
スレッドについて、そしてなぜスレッドを使用するのかについては、
第16章で並行性について語るときに詳しく議論しますが、今のところは、
`move`キーワードが必要なクロージャを使って新しいスレッドを生成することについて簡単に探検してみましょう。
リスト13-6に、メインスレッド内ではなく新しいスレッド内でベクタを出力するようにリスト13-4を変更したものを示します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<!--
<span class="caption">Listing 13-6: Using `move` to force the closure for the
thread to take ownership of `list`</span>
-->

<span class="caption">リスト13-6: `move`を使用して、スレッドのためのクロージャに`list`の所有権を奪わせる</span>

<!--
We spawn a new thread, giving the thread a closure to run as an argument. The
closure body prints out the list. In Listing 13-4, the closure only captured
`list` using an immutable reference because that's the least amount of access
to `list` needed to print it. In this example, even though the closure body
still only needs an immutable reference, we need to specify that `list` should
be moved into the closure by putting the `move` keyword at the beginning of the
closure definition. The new thread might finish before the rest of the main
thread finishes, or the main thread might finish first. If the main thread
maintained ownership of `list` but ended before the new thread did and dropped
`list`, the immutable reference in the thread would be invalid. Therefore, the
compiler requires that `list` be moved into the closure given to the new thread
so the reference will be valid. Try removing the `move` keyword or using `list`
in the main thread after the closure is defined to see what compiler errors you
get!
-->

実行するクロージャを引数としてスレッドに与えて、新しいスレッドを生成します。
クロージャ本体はリストを表示します。
リスト13-4では、クロージャは不変参照を使って`list`をキャプチャしているだけでした。
それが`list`を出力するために必要な最小のアクセスだからです。
この例では、クロージャ本体はやはり不変参照のみを要求しますが、
クロージャ定義の先頭に`move`キーワードを置くことで、
`list`をクロージャ内にムーブすべきだと指定する必要があります。
新しいスレッドはメインスレッドの残りの部分が終了するより前に終了するかもしれませんし、
メインスレッドが先に終了するかもしれません。
もしメインスレッドが`list`の所有権を持ち続け、新しいスレッドが終了する前に終了して`list`をドロップしてしまうと、
スレッド内の不変参照は無効になるでしょう。そのためコンパイラは、参照が有効になるように、
新しいスレッドに渡されるクロージャに`list`がムーブされることを要求します。
`move`キーワードを削除したり、メインスレッド内でクロージャが定義された後に`list`を使用したりしてみて、
どんなコンパイルエラーが得られるか確認してみてください！

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
-->

<!--
### Moving Captured Values Out of Closures and the `Fn` Traits
-->

### キャプチャされた値のクロージャからのムーブと、`Fn`系トレイト

<!--
Once a closure has captured a reference or captured ownership of a value from
the environment where the closure is defined (thus affecting what, if anything,
is moved *into* the closure), the code in the body of the closure defines what
happens to the references or values when the closure is evaluated later (thus
affecting what, if anything, is moved *out of* the closure). A closure body can
do any of the following: move a captured value out of the closure, mutate the
captured value, neither move nor mutate the value, or capture nothing from the
environment to begin with.
-->

クロージャが参照をキャプチャしたり、クロージャが定義されている環境から値の所有権をキャプチャする
(つまりクロージャの*中に*何をムーブするかに影響を与えます)と、クロージャの本体内のコードは、
後でクロージャが評価されたときにその参照または値に何が起こるかを定義します
(つまりクロージャの*外に*何をムーブするかに影響を与えます)。
クロージャ本体は以下のいずれかを行うことができます: キャプチャされた値をクロージャの外にムーブしたり、
キャプチャされた値を変更したり、値をムーブも変更もしないでいたり、そもそも環境から何もキャプチャしないかです。

<!--
The way a closure captures and handles values from the environment affects
which traits the closure implements, and traits are how functions and structs
can specify what kinds of closures they can use. Closures will automatically
implement one, two, or all three of these `Fn` traits, in an additive fashion,
depending on how the closure’s body handles the values:
-->

クロージャがどのように環境から値をキャプチャして使用するかは、クロージャがどのトレイトを実装するかに影響を及ぼします。
トレイトは、関数や構造体がどんな種類のクロージャを使用できるかを指定するための方法として使用されます。
クロージャは、その本体はどのように値を使用するかに応じて、以下の`Fn`系トレイトのうち1つ、2つ、または3つすべてを、付加的に自動的に実装します:

<!--
1. `FnOnce` applies to closures that can be called once. All closures implement
   at least this trait, because all closures can be called. A closure that
   moves captured values out of its body will only implement `FnOnce` and none
   of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body
   and that don’t mutate captured values, as well as closures that capture
   nothing from their environment. These closures can be called more than once
   without mutating their environment, which is important in cases such as
   calling a closure multiple times concurrently.
-->

1. `FnOnce`は、一度呼び出すことができるクロージャに適用されます。
   すべてのクロージャは、呼び出すことができるので、最低でもこのトレイトを実装します。
   キャプチャされた値を本体の外にムーブするクロージャは、一度しか呼び出すことができないので、
   `FnOnce`のみを実装し、他の`Fn`系トレイト群を実装しないことになるでしょう。
2. `FnMut`は、キャプチャされた値を本体の外にムーブしないが、キャプチャされた値を変更するかもしれないクロージャに適用されます。
   これらのクロージャは複数回呼び出すことができます。
3. `Fn`は、キャプチャされた値を本体の外にムーブせず、キャプチャされた値を変更しないクロージャと、
   環境から何もキャプチャしないクロージャに適用されます。これらのクロージャは環境を変更することなく複数回呼び出すことができ、
   クロージャを並行して複数回呼び出す場合などに重要です。


<!--
Let’s look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-1:
-->

リスト13-1で使用した、`Option<T>`に対する`unwrap_or_else`メソッドの定義を見てみましょう:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

<!--
Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.
-->

`T`は`Option`の`Some`列挙子内の値の型を表現するジェネリック型であることを思い出してください。
型`T`は`unwrap_or_else`関数の戻り値型でもあります: 例えば`Option<String>`に対して`unwrap_or_else`を呼び出すコードは、`String`を得るでしょう。

<!--
Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.
-->

次に、`unwrap_or_else`関数は追加のジェネリック型引数`F`を持つことに注目してください。
型`F`は引数`f`の型で、`f`は`unwrap_or_else`を呼び出すときにこちらから提供するクロージャです。

<!--
The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called once, take no arguments, and return a `T`.
Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts the most different kinds
of closures and is as flexible as it can be.
-->

`FnOnce() -> T`はジェネリック型`F`に対して指定されているトレイト境界で、これは、
`F`は一度呼び出すことができ、引数を取らず、`T`を返すことを意味します。
トレイト境界で`FnOnce`を使用することは、`unwrap_or_else`は`f`を多くとも1回しか呼び出さないという制約を表現します。
`unwrap_or_else`の本体の中では、`Option`が`Some`の場合は`f`は呼ばれないことがわかるでしょう。
`Option`が`None`の場合は、`f`は一度呼び出されるでしょう。すべてのクロージャは`FnOnce`を実装するので、
`unwrap_or_else`は最も多くの種類のクロージャを受け入れ、最も柔軟です。

<!--
> Note: Functions can implement all three of the `Fn` traits too. If what we
> want to do doesn’t require capturing a value from the environment, we can use
> the name of a function rather than a closure where we need something that
> implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value,
> we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the
> value is `None`.
-->

> 注釈: 関数も3つの`Fn`系トレイト全部を実装することができます。もし環境から値をキャプチャする必要がなければ、
> `Fn`系トレイトのいずれかを実装する何かが必要になるクロージャではなく、関数の名前を使用できます。
> 例えば、`Option<Vec<T>>`値に対して、その値が`None`である場合には新しい空のベクタを得るためには、
> `unwrap_or_else(Vec::new)`と呼び出すことができます。

<!--
Now let’s look at the standard library method `sort_by_key` defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound. The closure gets one argument
in the form of a reference to the current item in the slice being considered,
and returns a value of type `K` that can be ordered. This function is useful
when you want to sort a slice by a particular attribute of each item. In
Listing 13-7, we have a list of `Rectangle` instances and we use `sort_by_key`
to order them by their `width` attribute from low to high:
-->

それでは、スライスに対して定義されている標準ライブラリメソッド`sort_by_key`を見てみて、
`unwrap_or_else`とどう違うのか、`sort_by_key`はなぜトレイト境界に`FnOnce`ではなく`FnMut`を使用するのか、
確認してみましょう。このクロージャは、処理対象のスライス内の現在の要素への参照という形で1つの引数を取り、
順序付けることができる`K`型の値を返します。この関数は、
スライスを各要素の特定の属性によってソートしたいときに便利です。
リスト13-7では、`Rectangle`インスタンスからなるリストがあり、
`width`属性によってその各要素を昇順に並び替えるために、`sort_by_key`を使用しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<!--
<span class="caption">Listing 13-7: Using `sort_by_key` to order rectangles by
width</span>
-->

<span class="caption">リスト13-7: `sort_by_key`を使用して、幅で長方形を順序付ける</span>

<!--
This code prints:
-->

このコードは以下を出力します:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

<!--
The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move out anything from its environment, so
it meets the trait bound requirements.
-->

`sort_by_key`が`FnMut`クロージャを取るように定義されている理由は、クロージャを複数回呼び出すからです:
スライス内の各要素ごとに一度呼び出されます。クロージャ`|r| r.width` は、環境から何もキャプチャ、
変更、ムーブしないので、トレイト境界の要求を満たしています。

<!--
In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won’t let us use this closure with `sort_by_key`:
-->

対照的にリスト13-8では、環境から値をムーブすることから、
`FnOnce`トレイトだけを実装するクロージャの例を示しています。
コンパイラは、`sort_by_key`でこのクロージャを使わせてくれないでしょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<!--
<span class="caption">Listing 13-8: Attempting to use an `FnOnce` closure with
`sort_by_key`</span>
-->

<span class="caption">リスト13-8: `sort_by_key`で`FnOnce`クロージャを使用しようとする</span>

<!--
This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` gets called when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure’s
environment—into the `sort_operations` vector. The closure captures `value`
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:
-->

これは、`list`をソートするときに`sort_by_key`が呼ばれる回数を数えようとする、
人為的で複雑な(しかも動かない)方法です。このコードは、
`sort_operations`ベクタに`value`—クロージャの環境からの`String`—をプッシュすることで、
このカウントを行おうとします。このクロージャは`value`をキャプチャし、
`value`の所有権を`sort_operations`ベクタに移転することで、`value`をクロージャの外にムーブします。
このクロージャは一度呼び出すことができます; 2回目の呼び出しをしようとしても、
`sort_operations`にもう一度プッシュするための`value`はもう環境にないので、うまくいかないでしょう！
そのため、このクロージャは`FnOnce`のみを実装します。
このコードをコンパイルしようとすると、クロージャが`FnMut`を実装していなくてはならないので、
`value`はクロージャの外にムーブすることができない、というエラーが得られます:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

<!--
The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. To count the number of times `sort_by_key`
is called, keeping a counter in the environment and incrementing its value in
the closure body is a more straightforward way to calculate that. The closure
in Listing 13-9 works with `sort_by_key` because it is only capturing a mutable
reference to the `num_sort_operations` counter and can therefore be called more
than once:
-->

エラーは、クロージャ本体内で、`value`を環境からムーブする行を指しています。
これを修正するためには、値を環境からムーブしないようにクロージャ本体を変更する必要があります。
`sort_by_key`が呼ばれる回数を数えるためには、環境内でカウンタを保持し、
クロージャ本体の中でその値をインクリメントするのが、より自然な方法です。
リスト13-9のクロージャは、`num_sort_operations`カウンタへの可変参照をキャプチャするだけで、
そのため複数回呼び出すことができるので、`sort_by_key`で使うことができます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<!--
<span class="caption">Listing 13-9: Using an `FnMut` closure with `sort_by_key`
is allowed</span>
-->

<span class="caption">リスト13-9: `sort_by_key`で`FnMut`クロージャを使用することは許可される</span>

<!--
The `Fn` traits are important when defining or using functions or types that
make use of closures. In the next section, we’ll discuss iterators. Many
iterator methods take closure arguments, so keep these closure details in mind
as we continue!
-->

クロージャを使用する関数や型を定義または使用するときに、`Fn`系トレイトは重要です。
次の節では、イテレータについて議論します。多くのイテレータメソッドはクロージャ引数を取るので、
続けるにあたって、これらのクロージャの詳細のことを覚えておいてください！

<!--
[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
-->

[unwrap-or-else]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
