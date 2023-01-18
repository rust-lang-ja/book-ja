<!--
## Validating References with Lifetimes
-->

## ライフタイムで参照を検証する

<!--
One detail we didn’t discuss in the [“References and
Borrowing”][references-and-borrowing] section in Chapter 4 is
that every reference in Rust has a *lifetime*, which is the scope for which
that reference is valid. Most of the time, lifetimes are implicit and
inferred, just like most of the time, types are inferred. We must annotate
types when multiple types are possible. In a similar way, we must annotate
lifetimes when the lifetimes of references could be related in a few different
ways. Rust requires us to annotate the relationships using generic lifetime
parameters to ensure the actual references used at runtime will definitely be
valid.
-->

第 4 章の[「参照と借用」][references-and-borrowing]節で議論しなかった詳細の一つに、Rustにおいて参照は全てライフタイムを保持するということがあります。
ライフタイムとは、その参照が有効になるスコープのことです。多くの場合、型が推論されるように、
大体の場合、ライフタイムも暗黙的に推論されます。複数の型の可能性があるときには、型を注釈しなければなりません。
同様に、参照のライフタイムがいくつか異なる方法で関係することがある場合には注釈しなければなりません。
コンパイラは、ジェネリックライフタイム引数を使用して関係を注釈し、実行時に実際の参照が確かに有効であることを保証することを要求するのです。

<!--
The concept of lifetimes is somewhat different from tools in other programming
languages, arguably making lifetimes Rust’s most distinctive feature. Although
we won’t cover lifetimes in their entirety in this chapter, we’ll discuss
common ways you might encounter lifetime syntax so you can become familiar with
the concepts.
-->

ライフタイムの概念は、他のプログラミング言語の道具とはどこか異なり、間違いなく Rust で一番際立った機能になっています。
この章では、ライフタイムの全体を解説することはしませんが、
ライフタイム記法が必要となる最も一般的な場合について議論しますので、ライフタイムの概念について馴染むことができるでしょう。

<!--
### Preventing Dangling References with Lifetimes
-->

### ライフタイムでダングリング参照を回避する

<!--
The main aim of lifetimes is to prevent dangling references, which cause a
program to reference data other than the data it’s intended to reference.
Consider the program in Listing 10-17, which has an outer scope and an inner
scope.
-->

ライフタイムの主な目的は、ダングリング参照を回避することです。ダングリング参照によりプログラムは、
参照するつもりだったデータ以外のデータを参照してしまいます。リスト 10-17 のプログラムを考えてください。
これには、外側のスコープと内側のスコープが含まれています。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-17: An attempt to use a reference whose value
has gone out of scope</span>
-->

<span class="caption">リスト 10-17: 値がスコープを抜けてしまった参照を使用しようとする</span>

<!--
> Note: The examples in Listings 10-17, 10-18, and 10-24 declare variables
> without giving them an initial value, so the variable name exists in the
> outer scope. At first glance, this might appear to be in conflict with Rust’s
> having no null values. However, if we try to use a variable before giving it
> a value, we’ll get a compile-time error, which shows that Rust indeed does
> not allow null values.
-->

> 注釈：リスト 10-17 や 10-18、10-24 では、変数に初期値を与えずに宣言しているので、変数名は外側のスコープに存在します。
> 初見では、これは Rust には null 値が存在しないということと衝突しているように見えるかもしれません。
> しかしながら、値を与える前に変数を使用しようとすれば、コンパイルエラーになり、
> 確かに Rust では null 値は許可されていないことがわかります。

<!--
The outer scope declares a variable named `r` with no initial value, and the
inner scope declares a variable named `x` with the initial value of 5. Inside
the inner scope, we attempt to set the value of `r` as a reference to `x`. Then
the inner scope ends, and we attempt to print the value in `r`. This code won’t
compile because the value `r` is referring to has gone out of scope before we
try to use it. Here is the error message:
-->

外側のスコープで初期値なしの`r`という変数を宣言し、内側のスコープで初期値 5 の`x`という変数を宣言しています。
内側のスコープ内で、`r`の値を`x`への参照にセットしようとしています。それから内側のスコープが終わり、
`r`の値を出力しようとしています。`r`が参照している値が使おうとする前にスコープを抜けるので、
このコードはコンパイルできません。こちらがエラーメッセージです：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/output.txt}}
```

<!--
The variable `x` doesn’t “live long enough.” The reason is that `x` will be out
of scope when the inner scope ends on line 7. But `r` is still valid for the
outer scope; because its scope is larger, we say that it “lives longer.” If
Rust allowed this code to work, `r` would be referencing memory that was
deallocated when `x` went out of scope, and anything we tried to do with `r`
wouldn’t work correctly. So how does Rust determine that this code is invalid?
It uses a borrow checker.
-->

変数`x`の「生存期間が短すぎます」。原因は、内側のスコープが 7 行目で終わった時点で`x`がスコープを抜けるからです。
ですが、`r`はまだ、外側のスコープに対して有効です; スコープが大きいので、「長生きする」と言います。
Rust で、このコードが動くことを許可していたら、`r`は`x`がスコープを抜けた時に解放されるメモリを参照していることになり、
`r`で行おうとするいかなることもちゃんと動作しないでしょう。では、どうやってコンパイラはこのコードが無効であると決定しているのでしょうか？
それは、借用チェッカーを使用しているのです。

<!--
### The Borrow Checker
-->

### 借用精査機

<!--
The Rust compiler has a *borrow checker* that compares scopes to determine
whether all borrows are valid. Listing 10-18 shows the same code as Listing
10-17 but with annotations showing the lifetimes of the variables.
-->

Rust コンパイラには、スコープを比較して全ての借用が有効であるかを決定する*借用チェッカー*があります。
リスト 10-18 は、リスト 10-17 と同じコードを示していますが、変数のライフタイムを表示する注釈が付いています。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-18: Annotations of the lifetimes of `r` and
`x`, named `'a` and `'b`, respectively</span>
-->

<span class="caption">リスト 10-18: それぞれ`'a`と`'b`と名付けられた`r`と`x`のライフタイムの注釈</span>

<!--
Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x`
with `'b`. As you can see, the inner `'b` block is much smaller than the outer
`'a` lifetime block. At compile time, Rust compares the size of the two
lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory
with a lifetime of `'b`. The program is rejected because `'b` is shorter than
`'a`: the subject of the reference doesn’t live as long as the reference.
-->

ここで、`r`のライフタイムは`'a`、`x`のライフタイムは`'b`で注釈しました。ご覧の通り、
内側の`'b`ブロックの方が、外側の`'a`ライフタイムブロックよりはるかに小さいです。
コンパイル時に、コンパイラは 2 つのライフタイムのサイズを比較し、`r`は`'a`のライフタイムだけれども、
`'b`のライフタイムのメモリを参照していると確認します。`'b`は`'a`よりも短いので、プログラムは拒否されます：
参照の対象が参照ほど長生きしないのです。

<!--
Listing 10-19 fixes the code so it doesn’t have a dangling reference and
compiles without any errors.
-->

リスト 10-19 でコードを修正したので、ダングリング参照はなくなり、エラーなくコンパイルできます。

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-19: A valid reference because the data has a
longer lifetime than the reference</span>
-->

<span class="caption">リスト 10-19: データのライフタイムが参照より長いので、有効な参照</span>

<!--
Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x` because Rust knows that the reference in `r` will
always be valid while `x` is valid.
-->

ここで`x`のライフタイムは`'b`になり、今回の場合`'a`よりも大きいです。つまり、
コンパイラは`x`が有効な間、`r`の参照も常に有効になることを把握しているので、`r`は`x`を参照できます。

<!--
Now that you know where the lifetimes of references are and how Rust analyzes
lifetimes to ensure references will always be valid, let’s explore generic
lifetimes of parameters and return values in the context of functions.
-->

今や、参照のライフタイムがどれだけであるかと、コンパイラがライフタイムを解析して参照が常に有効であることを保証する仕組みがわかったので、
関数における引数と戻り値のジェネリックなライフタイムを探究しましょう。

<!--
### Generic Lifetimes in Functions
-->

### 関数のジェネリックなライフタイム

<!--
Let’s write a function that returns the longer of two string slices. This
function will take two string slices and return a string slice. After we’ve
implemented the `longest` function, the code in Listing 10-20 should print `The
longest string is abcd`.
-->

2 つの文字列スライスのうち、長い方を返す関数を書きましょう。この関数は、
2 つの文字列スライスを取り、1 つの文字列スライスを返します。`longest`関数の実装完了後、
リスト 10-20 のコードは、`The longest string is abcd`と出力するはずです。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs}}
```

<!--
<span class="caption">Listing 10-20: A `main` function that calls the `longest`
function to find the longer of two string slices</span>
-->

<span class="caption">リスト 10-20: `longest`関数を呼び出して 2 つの文字列スライスのうち長い方を探す`main`関数</span>

<!--
Note that we want the function to take string slices, which are references,
because we don’t want the `longest` function to take ownership of its
parameters. Refer to the [“String Slices as
Parameters”][string-slices-as-parameters] section in Chapter 4
for more discussion about why the parameters we use in Listing 10-20 are the
ones we want.
-->

関数に取ってほしい引数が文字列スライス、つまり参照であることに注意してください。
何故なら、`longest`関数に引数の所有権を奪ってほしくないからです。
リスト 10-20 で使用している引数が、我々が必要としているものである理由についてもっと詳しい議論は、
第 4 章の[「引数としての文字列スライス」][string-slices-as-parameters]節をご参照ください。

<!--
If we try to implement the `longest` function as shown in Listing 10-21, it
won’t compile.
-->

リスト 10-21 に示すように`longest`関数を実装しようとしたら、コンパイルできないでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-21: An implementation of the `longest`
function that returns the longer of two string slices but does not yet
compile</span>
-->

<span class="caption">リスト 10-21: 2 つの文字列スライスのうち長い方を返すけれども、コンパイルできない`longest`関数の実装</span>

<!--
Instead, we get the following error that talks about lifetimes:
-->

代わりに、以下のようなライフタイムに言及するエラーが出ます：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/output.txt}}
```

<!--
The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can’t tell whether the reference being returned refers to
`x` or `y`. Actually, we don’t know either, because the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!
-->

助言テキストが、戻り値の型はジェネリックなライフタイム引数である必要があると明かしています。
というのも、返している参照が`x`か`y`のどちらを参照しているか、コンパイラにはわからないからです。
実際のところ、この関数の本体の`if`ブロックは`x`への参照を返し、`else`ブロックは`y`への参照を返すので、
どちらなのか私たちにもわかりません！

<!--
When we’re defining this function, we don’t know the concrete values that will
be passed into this function, so we don’t know whether the `if` case or the
`else` case will execute. We also don’t know the concrete lifetimes of the
references that will be passed in, so we can’t look at the scopes as we did in
Listings 10-18 and 10-19 to determine whether the reference we return will
always be valid. The borrow checker can’t determine this either, because it
doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the
return value. To fix this error, we’ll add generic lifetime parameters that
define the relationship between the references so the borrow checker can
perform its analysis.
-->

この関数を定義する際、この関数に渡される具体的な値がわからないので、`if`ケースと`else`ケースのどちらが実行されるかわからないのです。
また、リスト 10-18 と 10-19 で、返す参照が常に有効であるかを決定したときのようにスコープを見ることも、渡される参照の具体的なライフタイムがわからないのでできないのです。
借用チェッカーもこれを決定することはできません。`x`と`y`のライフタイムがどう戻り値のライフタイムと関係するかわからないからです。
このエラーを修正するために、借用チェッカーが解析を実行できるように、参照間の関係を定義するジェネリックなライフタイム引数を追加しましょう。

<!--
### Lifetime Annotation Syntax
-->

### ライフタイム注釈記法

<!--
Lifetime annotations don’t change how long any of the references live. Just
as functions can accept any type when the signature specifies a generic type
parameter, functions can accept references with any lifetime by specifying a
generic lifetime parameter. Lifetime annotations describe the relationships of
the lifetimes of multiple references to each other without affecting the
lifetimes.
-->

ライフタイム注釈は、いかなる参照の生存期間も変えることはありません。シグニチャにジェネリックな型引数を指定された
関数が、あらゆる型を受け取ることができるのと同様に、ジェネリックなライフタイム引数を指定された関数は、
あらゆるライフタイムの参照を受け取ることができます。ライフタイム注釈は、ライフタイムに影響することなく、
複数の参照のライフタイムのお互いの関係を記述します。

<!--
Lifetime annotations have a slightly unusual syntax: the names of lifetime
parameters must start with an apostrophe (`'`) and are usually all lowercase and
very short, like generic types. Most people use the name `'a`. We place
lifetime parameter annotations after the `&` of a reference, using a space to
separate the annotation from the reference’s type.
-->

ライフタイム注釈は、少し不自然な記法です：ライフタイム引数の名前はアポストロフィー(`'`) で始まらなければならず、
通常全部小文字で、ジェネリック型のようにとても短いです。多くの人は、`'a`という名前を使います。
ライフタイム引数注釈は、参照の`&`の後に配置し、注釈と参照の型を区別するために空白を 1 つ使用します。

<!--
Here are some examples: a reference to an `i32` without a lifetime parameter, a
reference to an `i32` that has a lifetime parameter named `'a`, and a mutable
reference to an `i32` that also has the lifetime `'a`.
-->

例を挙げましょう：ライフタイム引数なしの`i32`への参照、`'a`というライフタイム引数付きの`i32`への参照、
そして同じくライフタイム`'a`を持つ`i32`への可変参照です。

```rust,ignore
&i32        // a reference
            // (ただの) 参照
&'a i32     // a reference with an explicit lifetime
            // 明示的なライフタイム付きの参照
&'a mut i32 // a mutable reference with an explicit lifetime
            // 明示的なライフタイム付きの可変参照
```

<!--
One lifetime annotation by itself doesn’t have much meaning, because the
annotations are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other. For example, let’s say we have a function with
the parameter `first` that is a reference to an `i32` with lifetime `'a`. The
function also has another parameter named `second` that is another reference to
an `i32` that also has the lifetime `'a`. The lifetime annotations indicate
that the references `first` and `second` must both live as long as that generic
lifetime.
-->

1 つのライフタイム注釈それだけでは、大して意味はありません。注釈は、複数の参照のジェネリックなライフタイム引数が、
お互いにどう関係するかをコンパイラに指示することを意図しているからです。例えば、
ライフタイム`'a`付きの`i32`への参照となる引数`first`のある関数があるとしましょう。
この関数にはさらに、`'a`のライフタイム付きの`i32`への別の参照となる`second`という別の引数もあります。
ライフタイム注釈は、`first`と`second`の参照がどちらもこのジェネリックなライフタイムと同じだけ生きることを示唆します。

<!--
### Lifetime Annotations in Function Signatures
-->

### 関数シグニチャにおけるライフタイム注釈

<!--
Now let’s examine lifetime annotations in the context of the `longest`
function. As with generic type parameters, we need to declare generic lifetime
parameters inside angle brackets between the function name and the parameter
list. The constraint we want to express in this signature is that all the
references in the parameters and the return value must have the same lifetime.
We’ll name the lifetime `'a` and then add it to each reference, as shown in
Listing 10-22.
-->

さて、`longest`関数を例にライフタイム注釈を詳しく見ていきましょう。ジェネリックな型引数同様、
関数名と引数リストの間の山カッコの中にジェネリックなライフタイム引数を宣言します。
このシグニチャで表現したい制約は、引数の全ての参照と戻り値が同じライフタイムを持つことです。
リスト 10-22 に示すように、ライフタイムを`'a`と名付け、それを各参照に付与します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-22: The `longest` function definition
specifying that all the references in the signature must have the same lifetime
`'a`</span>
-->

<span class="caption">リスト 10-22: シグニチャの全参照が同じライフタイム`'a`を持つと指定した`longest`関数の定義</span>

<!--
This code should compile and produce the result we want when we use it with the
`main` function in Listing 10-20.
-->

このコードはコンパイルでき、リスト 10-20 の`main`関数とともに使用したら、欲しい結果になるはずです。

<!--
The function signature now tells Rust that for some lifetime `'a`, the function
takes two parameters, both of which are string slices that live at least as
long as lifetime `'a`. The function signature also tells Rust that the string
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the
`longest` function is the same as the smaller of the lifetimes of the
references passed in. These constraints are what we want Rust to enforce.
Remember, when we specify the lifetime parameters in this function signature,
we’re not changing the lifetimes of any values passed in or returned. Rather,
we’re specifying that the borrow checker should reject any values that don’t
adhere to these constraints. Note that the `longest` function doesn’t need to
know exactly how long `x` and `y` will live, only that some scope can be
substituted for `'a` that will satisfy this signature.
-->

これで関数シグニチャは、何らかのライフタイム`'a`に対して、関数は 2 つの引数を取り、
どちらも少なくともライフタイム`'a`と同じだけ生きる文字列スライスであるとコンパイラに教えるようになりました。
また、この関数シグニチャは、関数から返る文字列スライスも少なくともライフタイム`'a`と同じだけ生きると、
コンパイラに教えています。
実際には、`longest`関数が返す参照のライフタイムは、渡された参照のうち、小さい方のライフタイムと同じであるという事です。
これらの制約は、まさに私たちがコンパイラに保証してほしかったものです。

この関数シグニチャでライフタイム引数を指定する時、渡されたり、返したりした、いかなる値のライフタイムも変更していないことを思い出してください。
むしろ、借用チェッカーは、これらの制約を守らない値全てを拒否するべきと指定しています。
`longest`関数は、`x`と`y`の正確な生存期間を知っている必要はなく、
このシグニチャを満たすようなスコープを`'a`に代入できることを知っているだけであることに注意してください。

<!--
When annotating lifetimes in functions, the annotations go in the function
signature, not in the function body. Rust can analyze the code within the
function without any help. However, when a function has references to or from
code outside that function, it becomes almost impossible for Rust to figure out
the lifetimes of the parameters or return values on its own. The lifetimes
might be different each time the function is called. This is why we need to
annotate the lifetimes manually.
-->

関数にライフタイムを注釈するときは、注釈は関数の本体ではなくシグニチャに付与します。
コンパイラは注釈がなくとも関数内のコードを解析できます。しかしながら、
関数に関数外からの参照や関数外への参照がある場合、コンパイラが引数や戻り値のライフタイムを自力で解決することはほとんど不可能になります。
そのライフタイムは、関数が呼び出される度に異なる可能性があります。このために、手動でライフタイムを注釈する必要があるのです。

<!--
When we pass concrete references to `longest`, the concrete lifetime that is
substituted for `'a` is the part of the scope of `x` that overlaps with the
scope of `y`. In other words, the generic lifetime `'a` will get the concrete
lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because
we’ve annotated the returned reference with the same lifetime parameter `'a`,
the returned reference will also be valid for the length of the smaller of the
lifetimes of `x` and `y`.
-->

具体的な参照を`longest`に渡すと、`'a`に代入される具体的なライフタイムは、`x`のスコープの一部であって`y`のスコープと重なる部分となります。
言い換えると、ジェネリックなライフタイム`'a`は、`x`と`y`のライフタイムのうち、小さい方に等しい具体的なライフタイムになるのです。
返却される参照を同じライフタイム引数`'a`で注釈したので、返却される参照も`x`か`y`のライフタイムの小さい方と同じだけ有効になるでしょう。

<!--
Let’s look at how the lifetime annotations restrict the `longest` function by
passing in references that have different concrete lifetimes. Listing 10-23 is
a straightforward example.
-->

ライフタイム注釈が異なる具体的なライフタイムを持つ参照を渡すことで`longest`関数を制限する方法を見ましょう。
リスト 10-23 はそのシンプルな例です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-23: Using the `longest` function with
references to `String` values that have different concrete lifetimes</span>
-->

<span class="caption">リスト 10-23: 異なる具体的なライフタイムを持つ`String`値への参照で`longest`関数を使用する</span>

<!--
In this example, `string1` is valid until the end of the outer scope, `string2`
is valid until the end of the inner scope, and `result` references something
that is valid until the end of the inner scope. Run this code, and you’ll see
that the borrow checker approves of this code; it will compile and print `The
longest string is long string is long`.
-->

この例において、`string1`は外側のスコープの終わりまで有効で、`string2`は内側のスコープの終わりまで有効、
そして`result`は内側のスコープの終わりまで有効な何かを参照しています。このコードを実行すると、
借用チェッカーがこのコードを良しとするのがわかるでしょう。要するに、コンパイルでき、
`The longest string is long string is long`と出力するのです。

<!--
Next, let’s try an example that shows that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Then we’ll move the `println!` that uses `result` outside the inner
scope, after the inner scope has ended. The code in Listing 10-24 will not
compile.
-->

次に、`result`の参照のライフタイムが 2 つの引数の小さい方のライフタイムになることを示す例を試しましょう。
`result`変数の宣言を内側のスコープの外に移すものの、`result`変数への代入は`string2`のスコープ内に残したままにします。
それから`result`を使用する`println!`を内側のスコープの外、内側のスコープが終わった後に移動します。
リスト 10-24 のコードはコンパイルできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-24: Attempting to use `result` after `string2`
has gone out of scope</span>
-->

<span class="caption">リスト 10-24: `string2`がスコープを抜けてから`result`を使用しようとする</span>

<!--
When we try to compile this code, we’ll get this error:
-->

このコードのコンパイルを試みると、こんなエラーになります：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/output.txt}}
```

<!--
The error shows that for `result` to be valid for the `println!` statement,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values using the same lifetime parameter `'a`.
-->

このエラーは、`result`が`println!`文に対して有効であるためには、`string2`が外側のスコープの終わりまで有効である必要があることを示しています。
関数引数と戻り値のライフタイムを同じライフタイム引数`'a`で注釈したので、コンパイラはこのことを知っています。

<!--
As humans, we can look at this code and see that `string1` is longer than
`string2` and therefore `result` will contain a reference to `string1`.
Because `string1` has not gone out of scope yet, a reference to `string1` will
still be valid for the `println!` statement. However, the compiler can’t see
that the reference is valid in this case. We’ve told Rust that the lifetime of
the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the references passed in. Therefore, the borrow checker
disallows the code in Listing 10-24 as possibly having an invalid reference.
-->

人間からしたら、`string1`は`string2`よりも長く、それ故に`result`が`string1`への参照を含んでいることは
コードから明らかです。まだ`string1`はスコープを抜けていないので、
`string1`への参照は`println!`にとって有効でしょう。ですが、コンパイラはこの場合、
参照が有効であると見なせません。`longest`関数から返ってくる参照のライフタイムは、
渡した参照のうちの小さい方と同じだとコンパイラに指示しました。したがって、
借用チェッカーは、リスト 10-24 のコードを無効な参照がある可能性があるとして許可しないのです。

<!--
Try designing more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether or not your experiments will pass the
borrow checker before you compile; then check to see if you’re right!
-->

試しに、値や、`longest`関数に渡される参照のライフタイムや、返される参照の使われかたが異なる実験をもっとしてみてください。
コンパイル前に、その実験が借用チェッカーを通るかどうか仮説を立ててください; そして、正しいか確かめてください！

<!--
### Thinking in Terms of Lifetimes
-->

### ライフタイムの観点で思考する

<!--
The way in which you need to specify lifetime parameters depends on what your
function is doing. For example, if we changed the implementation of the
`longest` function to always return the first parameter rather than the longest
string slice, we wouldn’t need to specify a lifetime on the `y` parameter. The
following code will compile:
-->

何にライフタイム引数を指定する必要があるかは、関数が行っていることに依存します。例えば、
`longest`関数の実装を最長の文字列スライスではなく、常に最初の引数を返すように変更したら、
`y`引数に対してライフタイムを指定する必要はなくなるでしょう。以下のコードはコンパイルできます：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

<!--
In this example, we’ve specified a lifetime parameter `'a` for the parameter
`x` and the return type, but not for the parameter `y`, because the lifetime of
`y` does not have any relationship with the lifetime of `x` or the return value.
-->

この例では、引数`x`と戻り値に対してライフタイム引数`'a`を指定しましたが、引数`y`には指定していません。
`y`のライフタイムは`x`や戻り値のライフタイムとは何の関係もないからです。

<!--
When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for one of the parameters. If
the reference returned does *not* refer to one of the parameters, it must refer
to a value created within this function, which would be a dangling reference
because the value will go out of scope at the end of the function. Consider
this attempted implementation of the `longest` function that won’t compile:
-->

関数から参照を返す際、戻り値型のライフタイム引数は、引数のうちどれかのライフタイム引数と一致する必要があります。
返される参照が引数のどれかを参照*していない*ならば、この関数内で生成された値を参照しているはずです。
すると、その値は関数の末端でスコープを抜けるので、これはダングリング参照になるでしょう。
以下に示す、コンパイルできない`longest`関数の未完成の実装を考えてください：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

<!--
Here, even though we’ve specified a lifetime parameter `'a` for the return
type, this implementation will fail to compile because the return value
lifetime is not related to the lifetime of the parameters at all. Here is the
error message we get:
-->

ここでは、たとえ、戻り値型にライフタイム引数`'a`を指定していても、戻り値のライフタイムは、
引数のライフタイムと全く関係がないので、この実装はコンパイルできないでしょう。
こちらが、得られるエラーメッセージです：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

<!--
The problem is that `result` goes out of scope and gets cleaned up at the end
of the `longest` function. We’re also trying to return a reference to `result`
from the function. There is no way we can specify lifetime parameters that
would change the dangling reference, and Rust won’t let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so the calling function is then responsible for
cleaning up the value.
-->

問題は、`result`が`longest`関数の末端でスコープを抜け、片付けられてしまうことです。
かつ、関数から`result`への参照を返そうともしています。ダングリング参照を変えてくれるようなライフタイム引数を指定する手段はなく、
コンパイラは、ダングリング参照を生成させてくれません。今回の場合、最善の修正案は、
（呼び出し先ではなく）呼び出し元の関数に値の片付けをさせるために、参照ではなく所有されたデータ型を返すことでしょう。

<!--
Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they’re connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.
-->

究極的にライフタイム記法は、関数のいろんな引数と戻り値のライフタイムを接続することに関するものです。
一旦それらが繋がれば、メモリ安全な処理を許可し、ダングリングポインタを生成したりメモリ安全性を侵害したりする処理を禁止するのに十分な情報をコンパイラは得たことになります。

<!--
### Lifetime Annotations in Struct Definitions
-->

### 構造体定義のライフタイム注釈

<!--
So far, we’ve only defined structs to hold owned types. It’s possible for
structs to hold references, but in that case we would need to add a lifetime
annotation on every reference in the struct’s definition. Listing 10-25 has a
struct named `ImportantExcerpt` that holds a string slice.
-->

ここまで、所有された型を保持する構造体だけを定義してきました。構造体に参照を保持させることもできますが、
その場合、構造体定義の全参照にライフタイム注釈を付け加える必要があるでしょう。
リスト 10-25 には、文字列スライスを保持する`ImportantExcerpt`(重要な一節) という構造体があります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs}}
```

<!--
<span class="caption">Listing 10-25: A struct that holds a reference, so its
definition needs a lifetime annotation</span>
-->

<span class="caption">リスト 10-25: 参照を含む構造体なので、定義にライフタイム注釈が必要</span>

<!--
This struct has one field, `part`, that holds a string slice, which is a
reference. As with generic data types, we declare the name of the generic
lifetime parameter inside angle brackets after the name of the struct so we can
use the lifetime parameter in the body of the struct definition. This
annotation means an instance of `ImportantExcerpt` can’t outlive the reference
it holds in its `part` field.
-->

この構造体には文字列スライスを保持する 1 つのフィールド、`part`があり、これは参照です。
ジェネリックなデータ型同様、構造体名の後、山カッコの中にジェネリックなライフタイム引数の名前を宣言するので、
構造体定義の本体でライフタイム引数を使用できます。この注釈は、`ImportantExcerpt`のインスタンスが、
`part`フィールドに保持している参照よりも長生きしないことを意味します。

<!--
The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`. The data in `novel` exists before the `ImportantExcerpt`
instance is created. In addition, `novel` doesn’t go out of scope until after
the `ImportantExcerpt` goes out of scope, so the reference in the
`ImportantExcerpt` instance is valid.
-->

ここの`main`関数は、変数`novel`に所有される`String`の、最初の文への参照を保持する`ImportantExcerpt`インスタンスを生成しています。
`novel`のデータは、`ImportantExcerpt`インスタンスが作られる前に存在しています。
加えて、`ImportantExcerpt`がスコープを抜けるまで`novel`はスコープを抜けないので、
`ImportantExcerpt`インスタンスの参照は有効なのです。

<!--
### Lifetime Elision
-->

### ライフタイム省略

<!--
You’ve learned that every reference has a lifetime and that you need to specify
lifetime parameters for functions or structs that use references. However, in
Chapter 4 we had a function in Listing 4-9, which is shown again in Listing
10-26, that compiled without lifetime annotations.
-->

全参照にはライフタイムがあり、参照を使用する関数や構造体にはライフタイム引数を指定する必要があることを学びました。
しかし、リスト 4-9 にあった関数（リスト 10-26 に再度示しました）はライフタイム注釈なしでコンパイルできました。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-26: A function we defined in Listing 4-9 that
compiled without lifetime annotations, even though the parameter and return
type are references</span>
-->

<span class="caption">リスト 10-26: リスト 4-9 で定義した、引数と戻り値型が参照であるにも関わらず、ライフタイム注釈なしでコンパイルできた関数</span>

<!--
The reason this function compiles without lifetime annotations is historical:
in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because
every reference needed an explicit lifetime. At that time, the function
signature would have been written like this:
-->

この関数がライフタイム注釈なしでコンパイルできる理由には、Rust の歴史が関わっています：昔のバージョンの Rust(1.0 以前) では、
全参照に明示的なライフタイムが必要だったので、このコードはコンパイルできませんでした。
その頃、関数シグニチャはこのように記述されていたのです：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

<!--
After writing a lot of Rust code, the Rust team found that Rust programmers
were entering the same lifetime annotations over and over in particular
situations. These situations were predictable and followed a few deterministic
patterns. The developers programmed these patterns into the compiler’s code so
the borrow checker could infer the lifetimes in these situations and wouldn’t
need explicit annotations.
-->

多くの Rust コードを書いた後、Rust チームは、Rust プログラマが、
特定の場面で何度も同じライフタイム注釈を入力していることを発見しました。これらの場面は予測可能で、
いくつかの決まりきったパターンに従っていました。開発者はこのパターンをコンパイラのコードに落とし込んだので、
このような場面には借用チェッカーがライフタイムを推論できるようになり、明示的な注釈を必要としなくなったのです。

<!--
This piece of Rust history is relevant because it’s possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.
-->

ここで、この Rust の歴史話が関係しているのは、他にも決まりきったパターンが出現し、コンパイラに追加されることもあり得るからです。
将来的に、さらに少数のライフタイム注釈しか必要にならない可能性もあります。

<!--
The patterns programmed into Rust’s analysis of references are called the
*lifetime elision rules*. These aren’t rules for programmers to follow; they’re
a set of particular cases that the compiler will consider, and if your code
fits these cases, you don’t need to write the lifetimes explicitly.
-->

コンパイラの参照解析に落とし込まれたパターンは、*ライフタイム省略規則*と呼ばれます。
これらはプログラマが従う規則ではありません; コンパイラが考慮する一連の特定のケースであり、
自分のコードがこのケースに当てはまれば、ライフタイムを明示的に書く必要はなくなります。

<!--
The elision rules don’t provide full inference. If Rust deterministically
applies the rules but there is still ambiguity as to what lifetimes the
references have, the compiler won’t guess what the lifetime of the remaining
references should be. In this case, instead of guessing, the compiler will give
you an error that you can resolve by adding the lifetime annotations that
specify how the references relate to each other.
-->

省略規則は、完全な推論を提供しません。コンパイラが決定的に規則を適用できるけれども、
参照が保持するライフタイムに関してそれでも曖昧性があるなら、コンパイラは、残りの参照がなるべきライフタイムを推測しません。
この場合コンパイラは、それらを推測するのではなくエラーを与えます。
これらは、参照がお互いにどう関係するかを指定するライフタイム注釈を追記することで解決できます。

<!--
Lifetimes on function or method parameters are called *input lifetimes*, and
lifetimes on return values are called *output lifetimes*.
-->

関数やメソッドの引数のライフタイムは、*入力ライフタイム*と呼ばれ、
戻り値のライフタイムは*出力ライフタイム*と称されます。

<!--
The compiler uses three rules to figure out what lifetimes references have when
there aren’t explicit annotations. The first rule applies to input lifetimes,
and the second and third rules apply to output lifetimes. If the compiler gets
to the end of the three rules and there are still references for which it can’t
figure out lifetimes, the compiler will stop with an error. These rules apply
to `fn` definitions as well as `impl` blocks.
-->

コンパイラは 3 つの規則を活用し、明示的な注釈がない時に、参照がどんなライフタイムになるかを計算します。
最初の規則は入力ライフタイムに適用され、2 番目と 3 番目の規則は出力ライフタイムに適用されます。
コンパイラが 3 つの規則の最後まで到達し、それでもライフタイムを割り出せない参照があったら、
コンパイラはエラーで停止します。
これらのルールは`fn`の定義にも`impl`ブロックにも適用されます。

<!--
The first rule is that each parameter that is a reference gets its own lifetime
parameter. In other words, a function with one parameter gets one lifetime
parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two
separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so
on.
-->

最初の規則は、参照である各引数は、独自のライフタイム引数を得るというものです。換言すれば、
1 引数の関数は、1 つのライフタイム引数を得るということです：`fn foo<'a>(x: &'a i32)`;
2 つ引数のある関数は、2 つの個別のライフタイム引数を得ます：`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`;
以下同様。

<!--
The second rule is if there is exactly one input lifetime parameter, that
lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32)
-> &'a i32`.
-->

2 番目の規則は、1 つだけ入力ライフタイム引数があるなら、そのライフタイムが全ての出力ライフタイム引数に代入されるというものです：
`fn foo<'a>(x: &'a i32) -> &'a i32`。

<!--
The third rule is if there are multiple input lifetime parameters, but one of
them is `&self` or `&mut self` because this is a method, the lifetime of `self`
is assigned to all output lifetime parameters. This third rule makes methods
much nicer to read and write because fewer symbols are necessary.
-->

3 番目の規則は、複数の入力ライフタイム引数があるけれども、メソッドなのでそのうちの一つが`&self`や`&mut self`だったら、
`self`のライフタイムが全出力ライフタイム引数に代入されるというものです。
この 3 番目の規則により、必要なシンボルの数が減るので、メソッドが遥かに読み書きしやすくなります。

<!--
Let’s pretend we’re the compiler. We’ll apply these rules to figure out what
the lifetimes of the references in the signature of the `first_word` function
in Listing 10-26 are. The signature starts without any lifetimes associated
with the references:
-->

コンパイラの立場になってみましょう。これらの規則を適用して、リスト 10-26 の`first_word`関数のシグニチャの参照のライフタイムが何か計算します。
シグニチャは、参照に紐づけられるライフタイムがない状態から始まります：

```rust,ignore
fn first_word(s: &str) -> &str {
```

<!--
Then the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We’ll call it `'a` as usual, so now the signature is
this:
-->

そうして、コンパイラは最初の規則を適用し、各引数が独自のライフタイムを得ると指定します。
それを通常通り`'a`と呼ぶので、シグニチャはこうなります：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

<!--
The second rule applies because there is exactly one input lifetime. The second
rule specifies that the lifetime of the one input parameter gets assigned to
the output lifetime, so the signature is now this:
-->

1 つだけ入力ライフタイムがあるので、2 番目の規則を適用します。2 番目の規則は、1 つの入力引数のライフタイムが、
出力引数に代入されると指定するので、シグニチャはこうなります：

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

<!--
Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.
-->

もうこの関数シグニチャの全ての参照にライフタイムが付いたので、コンパイラは、
プログラマにこの関数シグニチャのライフタイムを注釈してもらう必要なく、解析を続行できます。

<!--
Let’s look at another example, this time using the `longest` function that had
no lifetime parameters when we started working with it in Listing 10-21:
-->

別の例に目を向けましょう。今回は、リスト 10-21 で取り掛かったときにはライフタイム引数がなかった`longest`関数です：

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

<!--
Let’s apply the first rule: each parameter gets its own lifetime. This time we
have two parameters instead of one, so we have two lifetimes:
-->

最初の規則を適用しましょう：各引数が独自のライフタイムを得るのです。今回は、
1 つではなく 2 つ引数があるので、ライフタイムも 2 つです：

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

<!--
You can see that the second rule doesn’t apply because there is more than one
input lifetime. The third rule doesn’t apply either, because `longest` is a
function rather than a method, so none of the parameters are `self`. After
working through all three rules, we still haven’t figured out what the return
type’s lifetime is. This is why we got an error trying to compile the code in
Listing 10-21: the compiler worked through the lifetime elision rules but still
couldn’t figure out all the lifetimes of the references in the signature.
-->

2 つ以上入力ライフタイムがあるので、2 番目の規則は適用されないとわかります。また 3 番目の規則も適用されません。
`longest`はメソッドではなく関数なので、どの引数も`self`ではないのです。3 つの規則全部を適用した後でも、
まだ戻り値型のライフタイムが判明していません。このために、リスト 10-21 でこのコードをコンパイルしようとしてエラーになったのです：
コンパイラは、ライフタイム省略規則全てを適用したけれども、シグニチャの参照全部のライフタイムを計算できなかったのです。

<!--
Because the third rule really only applies in method signatures, we’ll look at
lifetimes in that context next to see why the third rule means we don’t have to
annotate lifetimes in method signatures very often.
-->

実際のところ、3 番目の規則はメソッドのシグニチャにしか適用されません。ですので、次はその文脈においてライフタイムを観察し、
3 番目の規則のおかげで、メソッドシグニチャであまり頻繁にライフタイムを注釈しなくても済む理由を確認します。

<!--
### Lifetime Annotations in Method Definitions
-->

### メソッド定義におけるライフタイム注釈

<!--
When we implement methods on a struct with lifetimes, we use the same syntax as
that of generic type parameters shown in Listing 10-11. Where we declare and
use the lifetime parameters depends on whether they’re related to the struct
fields or the method parameters and return values.
-->

構造体にライフタイムのあるメソッドを実装する際、リスト 10-11 で示したジェネリックな型引数と同じ記法を使用します。
ライフタイム引数を宣言し使用する場所は、構造体フィールドかメソッド引数と戻り値に関係するかによります。

<!--
Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, because those lifetimes are part
of the struct’s type.
-->

構造体のフィールド用のライフタイム名は、`impl`キーワードの後に宣言する必要があり、
それから構造体名の後に使用されます。そのようなライフタイムは構造体の型の一部になるからです。

<!--
In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-25.
-->

`impl`ブロック内のメソッドシグニチャでは、参照は構造体のフィールドの参照のライフタイムに紐づいている可能性と、
独立している可能性があります。加えて、ライフタイム省略規則により、メソッドシグニチャでライフタイム注釈が必要なくなることがよくあります。
リスト 10-25 で定義した`ImportantExcerpt`という構造体を使用した例をいくつか見てみましょう。

<!--
First, we’ll use a method named `level` whose only parameter is a reference to
`self` and whose return value is an `i32`, which is not a reference to anything:
-->

まず、唯一の引数が`self`への参照で戻り値が`i32`という何かへの参照ではない`level`というメソッドを使用します：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

<!--
The lifetime parameter declaration after `impl` and its use after the type name
are required, but we’re not required to annotate the lifetime of the reference
to `self` because of the first elision rule.
-->

`impl`後のライフタイム引数宣言と型名の後にそれを使用するのは必須ですが、最初の省略規則のため、
`self`への参照のライフタイムを注釈する必要はありません。

<!--
Here is an example where the third lifetime elision rule applies:
-->

3 番目のライフタイム省略規則が適用される例はこちらです：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

<!--
There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.
-->

2 つ入力ライフタイムがあるので、コンパイラは最初のライフタイム省略規則を適用し、
`&self`と`announcement`に独自のライフタイムを与えます。それから、
引数の 1 つが`&self`なので、戻り値型は`&self`のライフタイムを得て、
全てのライフタイムが説明されました。

<!--
### The Static Lifetime
-->

###  静的ライフタイム

<!--
One special lifetime we need to discuss is `'static`, which means that this
reference *can* live for the entire duration of the program. All string
literals have the `'static` lifetime, which we can annotate as follows:
-->

議論する必要のある 1 種の特殊なライフタイムが、`'static`であり、これは、この参照がプログラムの全期間生存*できる*事を意味します。
文字列リテラルは全て`'static`ライフタイムになり、次のように注釈できます：

```rust
// 僕は静的ライフタイムを持ってるよ
let s: &'static str = "I have a static lifetime.";
```

<!--
The text of this string is stored directly in the program’s binary, which
is always available. Therefore, the lifetime of all string literals is
`'static`.
-->

この文字列のテキストは、プログラムのバイナリに直接格納され、常に利用可能です。故に、全文字列リテラルのライフタイムは、
`'static`なのです。

<!--
You might see suggestions to use the `'static` lifetime in error messages. But
before specifying `'static` as the lifetime for a reference, think about
whether the reference you have actually lives the entire lifetime of your
program or not. You might consider whether you want it to live that long, even
if it could. Most of the time, the problem results from attempting to create a
dangling reference or a mismatch of the available lifetimes. In such cases, the
solution is fixing those problems, not specifying the `'static` lifetime.
-->

エラーメッセージで、`'static`ライフタイムを使用するよう勧める提言を見かける可能性があります。
ですが、参照に対してライフタイムとして`'static`を指定する前に、今ある参照が本当にプログラムの全期間生きるかどうか考えてください。
それが可能であったとしても、参照がそれだけの期間生きてほしいのかどうか考慮するのも良いでしょう。
ほとんどの場合、問題は、ダングリング参照を生成しようとしているか、利用可能なライフタイムの不一致が原因です。
そのような場合、解決策はその問題を修正することであり、`'static`ライフタイムを指定することではありません。

<!--
## Generic Type Parameters, Trait Bounds, and Lifetimes Together
-->

## ジェネリックな型引数、トレイト境界、ライフタイムを一度に

<!--
Let’s briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!
-->

ジェネリックな型引数、トレイト境界、ライフタイム指定の構文のすべてを 1 つの関数で簡単に見てみましょう！

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

<!--
This is the `longest` function from Listing 10-22 that returns the longer of
two string slices. But now it has an extra parameter named `ann` of the generic
type `T`, which can be filled in by any type that implements the `Display`
trait as specified by the `where` clause. This extra parameter will be printed
before the function compares the lengths of the string slices, which is why the
`Display` trait bound is necessary. Because lifetimes are a type of generic,
the declarations of the lifetime parameter `'a` and the generic type parameter
`T` go in the same list inside the angle brackets after the function name.
-->

これがリスト 10-22 からの 2 つの文字列のうち長い方を返す`longest`関数ですが、
ジェネリックな型`T`の`ann`という追加の引数があり、これは`where`節で指定されているように、
`Display`トレイトを実装するあらゆる型で埋めることができます。
この追加の引数は、関数が文字列スライスの長さを比較する前に出力されるので、
`Display`トレイト境界が必要なのです。ライフタイムは一種のジェネリックなので、
ライフタイム引数`'a`とジェネリックな型引数`T`が関数名の後、山カッコ内の同じリストに収まっています。

<!--
## Summary
-->

## まとめ

<!--
We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you’re
ready to write code without repetition that works in many different situations.
Generic type parameters let you apply the code to different types. Traits and
trait bounds ensure that even though the types are generic, they’ll have the
behavior the code needs. You learned how to use lifetime annotations to ensure
that this flexible code won’t have any dangling references. And all of this
analysis happens at compile time, which doesn’t affect runtime performance!
-->

たくさんのことをこの章では講義しましたね！今やジェネリックな型引数、トレイトとトレイト境界、そしてジェネリックなライフタイム引数を知ったので、
多くの異なる場面で動くコードを繰り返すことなく書く準備ができました。ジェネリックな型引数により、
コードを異なる型に適用させてくれます。トレイトとトレイト境界は、型がジェネリックであっても、
コードが必要とする振る舞いを持つことを保証します。ライフタイム注釈を活用して、
この柔軟なコードにダングリング参照が存在しないことを保証する方法を学びました。
さらにこの解析は全てコンパイル時に起こり、実行時のパフォーマンスには影響しません！

<!--
Believe it or not, there is much more to learn on the topics we discussed in
this chapter: Chapter 17 discusses trait objects, which are another way to use
traits. There are also more complex scenarios involving lifetime annotations
that you will only need in very advanced scenarios; for those, you should read
the [Rust Reference][reference]. But next, you’ll learn how to write tests in
Rust so you can make sure your code is working the way it should.
-->

信じられないかもしれませんが、この章で議論した話題にはもっともっと学ぶべきことがあります：
第 17 章ではトレイトオブジェクトを議論します。これはトレイトを使用する別の手段です。
非常に高度な筋書きの場合でのみ必要になる、ライフタイム注釈が関わる、もっと複雑な筋書きもあります。
それらについては、[Rust Reference][reference]をお読みください。 
ですが次は、コードがあるべき通りに動いていることを確かめられるように、Rust でテストを書く方法を学びます。

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#参照と借用
[string-slices-as-parameters]:
ch04-03-slices.html#引数としての文字列スライス
[reference]: https://doc.rust-lang.org/reference/
