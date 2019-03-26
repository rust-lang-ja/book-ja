<!-- ## Advanced Lifetimes -->

## 高度なライフタイム

<!-- In Chapter 10 in the “Validating References with Lifetimes” section, you -->
<!-- learned how to annotate references with lifetime parameters to tell Rust how -->
<!-- lifetimes of different references relate. You saw how every reference has a -->
<!-- lifetime, but most of the time, Rust will let you elide lifetimes. Now we’ll -->
<!-- look at three advanced features of lifetimes that we haven’t covered yet: -->

第10章の「ライフタイムで参照を有効化する」節で、参照をライフタイム引数で注釈し、
コンパイラに異なる参照のライフタイムがどう関連しているかを指示する方法を学びました。全ての参照にはライフタイムがあるものの、
ほとんどの場合、コンパイラがライフタイムを省略させてくれることも見ました。ここでは、
まだ講義していないライフタイムの高度な機能を3つ見ていきます:

<!-- * Lifetime subtyping: ensures that one lifetime outlives another lifetime -->
<!-- * Lifetime bounds: specifies a lifetime for a reference to a generic type -->
<!-- * Inference of trait object lifetimes: allows the compiler to infer trait -->
<!--   object lifetimes and when they need to be specified -->

* ライフタイム・サブタイピング: あるライフタイムが他のライフタイムより長生きすることを保証する
* ライフタイム境界: ジェネリックな型への参照のライフタイムを指定する
* トレイトオブジェクトのライフタイムの推論: コンパイラにトレイトオブジェクトのライフタイムを推論させることと指定する必要があるタイミング

<!-- ### Ensuring One Lifetime Outlives Another with Lifetime Subtyping -->

### ライフタイム・サブタイピングにより、あるライフタイムが他よりも長生きすることを保証する

<!-- *Lifetime subtyping* specifies that one lifetime should outlive another -->
<!-- lifetime. To explore lifetime subtyping, imagine we want to write a parser. -->
<!-- We’ll use a structure called `Context` that holds a reference to the string -->
<!-- we’re parsing. We’ll write a parser that will parse this string and return -->
<!-- success or failure. The parser will need to borrow the `Context` to do the -->
<!-- parsing. Listing 19-12 implements this parser code, except the code doesn’t -->
<!-- have the required lifetime annotations, so it won’t compile. -->

*ライフタイム・サブタイピング*(lifetime subtyping; `訳注`: あえて訳すなら、ライフタイムの継承)は、
あるライフタイムが他のライフタイムよりも長生きすべきであることを指定します。
ライフタイム・サブタイピングを探求するために、パーサを書きたいところを想像してください。
パース(`訳注`: parse; 構文解析)中の文字列への参照を保持する`Context`と呼ばれる構造を使用します。この文字列をパースし、
成功か失敗を返すパーサを書きます。パーサは構文解析を行うために`Context`を借用する必要があるでしょう。
リスト19-12は、コードに必要なライフタイム注釈がないことを除いてこのパーサのコードを実装しているので、コンパイルはできません。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<!-- <span class="caption">Listing 19-12: Defining a parser without lifetime -->
<!-- annotations</span> -->

<span class="caption">リスト19-12: ライフタイム注釈なしでパーサを定義する</span>

<!-- Compiling the code results in errors because Rust expects lifetime parameters -->
<!-- on the string slice in `Context` and the reference to a `Context` in `Parser`. -->

コンパイラは`Context`の文字列スライスと`Parser`の`Context`への参照にライフタイム引数を期待するので、
このコードをコンパイルすると、エラーに落ち着きます。

<!-- For simplicity’s sake, the `parse` function returns `Result<(), &str>`. That -->
<!-- is, the function will do nothing on success and, on failure, will return the -->
<!-- part of the string slice that didn’t parse correctly. A real implementation -->
<!-- would provide more error information and would return a structured data type -->
<!-- when parsing succeeds. We won’t be discussing those details because they aren’t -->
<!-- relevant to the lifetimes part of this example. -->

簡単のため、`parse`関数は、`Result<(), &str>`を返します。つまり、関数は成功時には何もせず、
失敗時には、正しくパースできなかった文字列スライスの一部を返すということです。本物の実装は、
もっとエラーの情報を提供し、パースが成功したら、構造化されたデータ型を返すでしょう。そのような詳細を議論するつもりはありません。
この例のライフタイムの部分に関係ないからです。

<!-- To keep this code simple, we won’t write any parsing logic. However, it’s very -->
<!-- likely that somewhere in the parsing logic we would handle invalid input by -->
<!-- returning an error that references the part of the input that is invalid; this -->
<!-- reference is what makes the code example interesting in regard to lifetimes. -->
<!-- Let’s pretend that the logic of our parser is that the input is invalid after -->
<!-- the first byte. Note that this code might panic if the first byte is not on a -->
<!-- valid character boundary; again, we’re simplifying the example to focus on the -->
<!-- lifetimes involved. -->

このコードを単純に保つため、構文解析のロジックは何も書きません。ですが、構文解析ロジックのどこかで、
非合法な入力の一部を参照するエラーを返すことで非合法な入力を扱う可能性が非常に高いでしょう; この参照が、
ライフタイムに関連してこのコード例を面白くしてくれます。パーサのロジックが、最初のバイトの後で入力が不正だった振りをしましょう。
最初のバイトが合法な文字境界になければ、このコードはパニックする可能性があることに注意してください;
ここでも、例を簡略化して関連するライフタイムに集中しています。

<!-- To get this code to compile, we need to fill in the lifetime parameters for the -->
<!-- string slice in `Context` and the reference to the `Context` in `Parser`. The -->
<!-- most straightforward way to do this is to use the same lifetime name -->
<!-- everywhere, as shown in Listing 19-13. Recall from the “Lifetime Annotations in -->
<!-- Struct Definitions” section in Chapter 10 that each of `struct Context<'a>`, -->
<!-- `struct Parser<'a>`, and `impl<'a>` is declaring a new lifetime parameter. -->
<!-- While their names happen to all be the same, the three lifetime parameters -->
<!-- declared in this example aren’t related. -->

このコードをコンパイルできるようにするには、`Context`の文字列スライスと`Parser`の`Context`への参照のライフタイム引数を埋める必要があります。
最も率直な方法は、リスト19-13のように、全ての箇所で同じライフタイム名を使用することです。
第10章の「構造体定義のライフタイム注釈」節から、`struct Context<'a>`、`struct Parser<'a>`、
`impl<'a>`それぞれが新しいライフタイム引数を宣言することを思い出してください。全部の名前が偶然一致しましたが、
この例で宣言された3つのライフタイム引数は、関連していません。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<!-- <span class="caption">Listing 19-13: Annotating all references in `Context` and -->
<!-- `Parser` with lifetime parameters</span> -->

<span class="caption">リスト19-13: `Context`と`Parser`の全参照をライフタイム引数で注釈する</span>

<!-- This code compiles just fine. It tells Rust that a `Parser` holds a reference -->
<!-- to a `Context` with lifetime `'a` and that `Context` holds a string slice that -->
<!-- also lives as long as the reference to the `Context` in `Parser`. Rust’s -->
<!-- compiler error message stated that lifetime parameters were required for these -->
<!-- references, and we’ve now added lifetime parameters. -->

このコードは、単純にうまくコンパイルできます。コンパイラに`Parser`はライフタイム`'a`の`Context`への参照を保持し、
`Context`は`Parser`の`Context`への参照と同じ期間生きる文字列スライスを保持していると指示しています。
Rustコンパイラのエラーメッセージは、これらの参照にライフタイム引数が必要であることを述べていて、
今ではライフタイム引数を追加しました。

<!-- Next, in Listing 19-14, we’ll add a function that takes an instance of -->
<!-- `Context`, uses a `Parser` to parse that context, and returns what `parse` -->
<!-- returns. This code doesn’t quite work. -->

次にリスト19-14では、`Context`のインスタンスを1つ取り、`Parser`を使ってその文脈をパースし、
`parse`が返すものを返す関数を追加します。このコードはあまり動きません。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<!-- <span class="caption">Listing 19-14: An attempt to add a `parse_context` -->
<!-- function that takes a `Context` and uses a `Parser`</span> -->

<span class="caption">リスト19-14: `Context`を取り、`Parser`を使用する`parse_context`関数を追加する試み</span>

<!-- We get two verbose errors when we try to compile the code with the addition of -->
<!-- the `parse_context` function: -->

`parse_context`関数を追加してコードをコンパイルしようとすると、2つ冗長なエラーが出ます:

```text
error[E0597]: borrowed value does not live long enough
(エラー: 借用された値は十分長生きしません)
  --> src/lib.rs:14:5
   |
14 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
15 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
(注釈: 借用された値は、13:1の関数本体で定義された1番目の匿名のライフタイムに有効でなければなりません)
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^

error[E0597]: `context` does not live long enough
  --> src/lib.rs:14:24
   |
14 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
15 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^
```

<!-- These errors state that the `Parser` instance that is created and the `context` -->
<!-- parameter live only until the end of the `parse_context` function. But they -->
<!-- both need to live for the entire lifetime of the function. -->

これらのエラーは、生成された`Parser`インスタンスと`context`引数が`parse_context`関数の最後までしか生きないと述べています。
しかし、どちらも関数全体のライフタイムだけ生きる必要があります。

<!-- In other words, `Parser` and `context` need to *outlive* the entire function -->
<!-- and be valid before the function starts as well as after it ends for all the -->
<!-- references in this code to always be valid. The `Parser` we’re creating and the -->
<!-- `context` parameter go out of scope at the end of the function, because -->
<!-- `parse_context` takes ownership of `context`. -->

言い換えると、`Parser`と`context`は関数全体より*長生きし*、このコードの全参照が常に有効であるためには、
関数が始まる前や、終わった後も有効である必要があります。生成している`Parser`と`context`引数は、
関数の終わりでスコープを抜けます。`parse_context`が`context`の所有権を奪っているからです。

<!-- To figure out why these errors occur, let’s look at the definitions in Listing -->
<!-- 19-13 again, specifically the references in the signature of the `parse` method: -->

これらのエラーが起こる理由を理解するため、再度リスト19-13の定義、特に`parse`メソッドのシグニチャの参照を観察しましょう:

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

<!-- Remember the elision rules? If we annotate the lifetimes of the references -->
<!-- rather than eliding, the signature would be as follows: -->

省略規則を覚えていますか？省略するのではなく、参照のライフタイムを注釈するなら、シグニチャは以下のようになるでしょう:

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

<!-- That is, the error part of the return value of `parse` has a lifetime that is -->
<!-- tied to the lifetime of the `Parser` instance (that of `&self` in the `parse` -->
<!-- method signature). That makes sense: the returned string slice references the -->
<!-- string slice in the `Context` instance held by the `Parser`, and the definition -->
<!-- of the `Parser` struct specifies that the lifetime of the reference to -->
<!-- `Context` and the lifetime of the string slice that `Context` holds should be -->
<!-- the same. -->

要するに、`parse`の戻り値のエラー部分は、`Parser`インスタンスのライフタイムと紐づいたライフタイムになるのです
(`parse`メソッドシグニチャの`&self`のライフタイム)。それは、理に適っています: 返却される文字列スライスは、
`Parser`に保持された`Context`インスタンスの文字列スライスを参照していて、`Parser`構造体の定義は、
`Context`への参照のライフタイムと`Context`が保持する文字列スライスのライフタイムは同じになるべきと指定しています。

<!-- The problem is that the `parse_context` function returns the value returned -->
<!-- from `parse`, so the lifetime of the return value of `parse_context` is tied to -->
<!-- the lifetime of the `Parser` as well. But the `Parser` instance created in the -->
<!-- `parse_context` function won’t live past the end of the function (it’s -->
<!-- temporary), and `context` will go out of scope at the end of the function -->
<!-- (`parse_context` takes ownership of it). -->

問題は、`parse_context`関数は、`parse`から返却される値を返すので、`parse_context`の戻り値のライフタイムも、
`Parser`のライフタイムに紐づくことです。しかし、`parse_context`関数で生成された`Parser`インスタンスは、
関数の終端を超えて生きることはなく(一時的なのです)、`context`も関数の終端でスコープを抜けるのです(`parse_context`が所有権を奪っています)。

<!-- Rust thinks we’re trying to return a reference to a value that goes out of -->
<!-- scope at the end of the function, because we annotated all the lifetimes with -->
<!-- the same lifetime parameter. The annotations told Rust the lifetime of the -->
<!-- string slice that `Context` holds is the same as that of the lifetime of the -->
<!-- reference to `Context` that `Parser` holds. -->

コンパイラは、私たちが、関数の終端でスコープを抜ける値への参照を返そうとしていると考えます。
全ライフタイムを同じライフタイム引数で注釈したからです。注釈は、コンパイラに`Context`が保持する文字列スライスのライフタイムは、
`Parser`が保持する`Context`への参照のライフタイムと一致すると指示しました。

<!-- The `parse_context` function can’t see that within the `parse` function, the -->
<!-- string slice returned will outlive `Context` and `Parser` and that the -->
<!-- reference `parse_context` returns refers to the string slice, not to `Context` -->
<!-- or `Parser`. -->

`parse_context`関数には、`parse`関数内で返却される文字列スライスが`Context`と`Parser`より長生きし、
`parse_context`が返す参照が`Context`や`Parser`ではなく、文字列スライスを参照することはわかりません。

<!-- By knowing what the implementation of `parse` does, we know that the only -->
<!-- reason the return value of `parse` is tied to the `Parser` instance is that -->
<!-- it’s referencing the `Parser` instance’s `Context`, which is referencing the -->
<!-- string slice. So, it’s really the lifetime of the string slice that -->
<!-- `parse_context` needs to care about. We need a way to tell Rust that the string -->
<!-- slice in `Context` and the reference to the `Context` in `Parser` have -->
<!-- different lifetimes and that the return value of `parse_context` is tied to the -->
<!-- lifetime of the string slice in `Context`. -->

`parse`の実装が何をするか知ることで、`parse`の戻り値が`Parser`インスタンスに紐づく唯一の理由が、`Parser`インスタンスの`Context`、
引いては文字列スライスを参照していることであることを把握します。従って、`parse_context`が気にする必要があるのは、
本当は文字列スライスのライフタイムなのです。`Context`の文字列スライスと`Parser`の`Context`への参照が異なるライフタイムになり、
`parse_context`の戻り値が`Context`の文字列スライスのライフタイムに紐づくことをコンパイラに教える方法が必要です。

<!-- First, we’ll try giving `Parser` and `Context` different lifetime parameters, -->
<!-- as shown in Listing 19-15. We’ll use `'s` and `'c` as lifetime parameter names -->
<!-- to clarify which lifetime goes with the string slice in `Context` and which -->
<!-- goes with the reference to `Context` in `Parser`. Note that this solution won’t -->
<!-- completely fix the problem, but it’s a start. We’ll look at why this fix isn’t -->
<!-- sufficient when we try to compile. -->

まず、試しに`Parser`と`Context`に異なるライフタイム引数を与えてみましょう。リスト19-15のようにですね。
ライフタイム引数の名前として`'s`と`'c`を使用して、どのライフタイムが`Context`の文字列スライスに当てはまり、
どれが`Parser`の`Context`への参照に当てはまるかを明確化します。この解決策は、完全には問題を修正しませんが、
スタート地点です。コンパイルしようとする時にこの修正で十分でない理由に目を向けます。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<!-- <span class="caption">Listing 19-15: Specifying different lifetime parameters -->
<!-- for the references to the string slice and to `Context`</span> -->

<span class="caption">リスト19-15: 文字列スライスと`Context`への参照に異なるライフタイム引数を指定する</span>

<!-- We’ve annotated the lifetimes of the references in all the same places that we -->
<!-- annotated them in Listing 19-13. But this time we used different parameters -->
<!-- depending on whether the reference goes with the string slice or with -->
<!-- `Context`. We’ve also added an annotation to the string slice part of the -->
<!-- return value of `parse` to indicate that it goes with the lifetime of the -->
<!-- string slice in `Context`. -->

参照のライフタイム全部をリスト19-13で注釈したのと同じ箇所に注釈しました。ですが今回は、
参照が文字列スライスか`Context`に当てはまるかによって異なる引数を使用しました。また、
`parse`の戻り値の文字列スライス部分にも注釈を追加して、`Context`の文字列スライスのライフタイムに当てはまることを示唆しました。

<!-- When we try to compile now, we get the following error: -->

今コンパイルを試みると、以下のようなエラーになります:

```text
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
(エラー: 型`&'c Cotnext<'s>`において、参照のライフタイムが参照先のデータよりも長くなっています)
 --> src/lib.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:1
(注釈: ポインタは3:1の構造体で定義されたように、ライフタイム'cの間有効です)
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:1
(注釈: しかし、参照されたデータは、3:1の構造体で定義されたように、ライフタイム'sの間だけ有効です)
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

<!-- Rust doesn’t know of any relationship between `'c` and `'s`. To be valid, the -->
<!-- referenced data in `Context` with lifetime `'s` needs to be constrained to -->
<!-- guarantee that it lives longer than the reference with lifetime `'c`. If `'s` -->
<!-- is not longer than `'c`, the reference to `Context` might not be valid. -->

コンパイラは、`'c`と`'s`の間になんの関連性も知りません。合法であるために、`Context`でライフタイム`'s`と参照されたデータは、
制限され、ライフタイム`'c`の参照よりも長生きすることを保証する必要があります。`'s`が`'c`より長くないと、
`Context`への参照は合法ではない可能性があるのです。

<!-- Now we get to the point of this section: the Rust feature *lifetime -->
<!-- subtyping* specifies that one lifetime parameter lives at least as long as -->
<!-- another one. In the angle brackets where we declare lifetime parameters, we can -->
<!-- declare a lifetime `'a` as usual and declare a lifetime `'b` that lives at -->
<!-- least as long as `'a` by declaring `'b` using the syntax `'b: 'a`. -->

さて、この節の要点に到達しました: Rustの機能、*ライフタイム・サブタイピング*は、あるライフタイム引数が、
少なくとも他のライフタイムと同じだけ生きることを指定します。ライフタイム引数を宣言する山カッコ内で、
通常通りライフタイム`'a`を宣言し、`'b`を`'b: 'a`記法を使用して宣言することで、
`'a`と少なくとも同じ期間生きるライフタイム`'b`を宣言できます。

<!-- In our definition of `Parser`, to say that `'s` (the lifetime of the string -->
<!-- slice) is guaranteed to live at least as long as `'c` (the lifetime of the -->
<!-- reference to `Context`), we change the lifetime declarations to look like this: -->

`Parser`の定義で、`'s`(文字列スライスのライフタイム)が少なくとも`'c`(`Context`への参照のライフタイム)と同じ期間だけ生きると、
保証することを宣言するには、ライフタイム宣言を以下のように変更します:

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
# struct Context<'a>(&'a str);
#
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

<!-- Now the reference to `Context` in the `Parser` and the reference to the string -->
<!-- slice in the `Context` have different lifetimes; we’ve ensured that the -->
<!-- lifetime of the string slice is longer than the reference to the `Context`. -->

これで`Parser`の`Context`への参照と`Context`の文字列スライスへの参照のライフタイムは、違うものになりました;
文字列スライスのライフタイムが`Context`への参照よりも長いことを保証したのです。

<!-- That was a very long-winded example, but as we mentioned at the start of this -->
<!-- chapter, Rust’s advanced features are very specific. You won’t often need the -->
<!-- syntax we described in this example, but in such situations, you’ll know how to -->
<!-- refer to something and give it the necessary lifetime. -->

非常に長くぐにゃぐにゃした例でしたが、この章の冒頭で触れたように、Rustの高度な機能は、非常に限定的です。
この例で解説した記法は、あまり必要になりませんが、そのような場面では、何かを参照し、それに必要なライフタイムを与える方法を知っているでしょう。

<!-- ### Lifetime Bounds on References to Generic Types -->

### ジェネリックな型への参照に対するライフタイム境界

<!-- In the “Trait Bounds” section in Chapter 10, we discussed using trait bounds on -->
<!-- generic types. We can also add lifetime parameters as constraints on generic -->
<!-- types; these are called *lifetime bounds*. Lifetime bounds help Rust verify -->
<!-- that references in generic types won’t outlive the data they’re referencing. -->

第10章の「トレイト境界」節で、ジェネリックな型にトレイト境界を使用することを議論しました。
また、ジェネリックな型への制限としてライフタイム引数を追加することもできます; これは*ライフタイム境界*と呼ばれます。
ライフタイム境界は、コンパイラが、ジェネリックな型の中の参照が参照先のデータよりも長生きしないことを確かめる手助けをします。

<!-- As an example, consider a type that is a wrapper over references. Recall the -->
<!-- `RefCell<T>` type from the “`RefCell<T>` and the Interior Mutability Pattern” -->
<!-- section in Chapter 15: its `borrow` and `borrow_mut` methods return the types -->
<!-- `Ref` and `RefMut`, respectively. These types are wrappers over references that -->
<!-- keep track of the borrowing rules at runtime. The definition of the `Ref` -->
<!-- struct is shown in Listing 19-16, without lifetime bounds for now. -->

例として、参照のラッパの型を考えてください。第15章の「`RefCell<T>`と内部可変性パターン」節から`RefCell<T>`型を思い出してください:
`borrow`と`borrow_mut`メソッドがそれぞれ、`Ref`と`RefMut`を返します。これらの型は、
実行時に借用規則を追いかける参照に対するラッパです。`Ref`構造体の定義をリスト19-16に今はライフタイム境界なしで示しました。

<!-- <span class="filename">Filename: src/lib.rs</span> -->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
struct Ref<'a, T>(&'a T);
```

<!-- <span class="caption">Listing 19-16: Defining a struct to wrap a reference to a -->
<!-- generic type, without lifetime bounds</span> -->

<span class="caption">リスト19-16: ライフタイム境界なしでジェネリックな型への参照をラップする構造体を定義する</span>

<!-- Without explicitly constraining the lifetime `'a` in relation to the generic -->
<!-- parameter `T`, Rust will error because it doesn’t know how long the generic -->
<!-- type `T` will live: -->

明示的にジェネリック引数`T`と関連してライフタイム`'a`を制限することがないと、ジェネリックな型`T`がどれだけ生きるのかわからないので、
コンパイラはエラーにします:

```text
error[E0309]: the parameter type `T` may not live long enough
(エラー: パラメータ型の`T`は十分長生きしないかもしれません)
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
  (助言: 明示的なライフタイム境界`T: 'a`を追加することを考慮してください)
note: ...so that the reference type `&'a T` does not outlive the data it points at
(注釈: そうすれば、参照型の`&'a T`が、指しているデータよりも長生きしません)
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

<!-- Because `T` can be any type, `T` could be a reference or a type that holds one -->
<!-- or more references, each of which could have their own lifetimes. Rust can’t be -->
<!-- sure `T` will live as long as `'a`. -->

`T`はどんな型にもなるので、`T`が参照や1つ以上の参照を保持する型になることもあり、その個々の参照が独自のライフタイムになることもあるでしょう。
コンパイラは、`T`が`'a`と同じだけ生きることを確信できません。

<!-- Fortunately, the error provides helpful advice on how to specify the lifetime -->
<!-- bound in this case: -->

幸運なことに、この場合、エラーがライフタイム境界を指定する方法について役に立つアドバイスをくれています:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at
```

<!-- Listing 19-17 shows how to apply this advice by specifying the lifetime bound -->
<!-- when we declare the generic type `T`. -->

リスト19-17は、ジェネリックな型`T`を宣言する時にライフタイム境界を指定することで、
このアドバイスを適用する方法を示しています。

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<!-- <span class="caption">Listing 19-17: Adding lifetime bounds on `T` to specify -->
<!-- that any references in `T` live at least as long as `'a`</span> -->

<span class="caption">リスト19-17: `T`にライフタイム境界を追加して`T`のどんな参照も少なくとも、`'a`と同じだけ生きると指定する</span>

<!-- This code now compiles because the `T: 'a` syntax specifies that `T` can be any -->
<!-- type, but if it contains any references, the references must live at least as -->
<!-- long as `'a`. -->

このコードはもうコンパイルできます。`T: 'a`記法により、`T`はどんな型にもなり得ますが、何か参照を含んでいるのなら、
その参照は少なくとも、`'a`と同じだけ生きなければならないと指定しているからです。

<!-- We could solve this problem in a different way, as shown in the definition of a -->
<!-- `StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on -->
<!-- `T`. This means if `T` contains any references, they must have the `'static` -->
<!-- lifetime. -->

この問題をリスト19-18の`StaticRef`構造体の定義で示したように、`T`に`'static`ライフタイム境界を追加し、異なる方法で解決することもできます。
これは、`T`に何か参照が含まれるなら、`'static`ライフタイムでなければならないことを意味します。

```rust
struct StaticRef<T: 'static>(&'static T);
```

<!-- <span class="caption">Listing 19-18: Adding a `'static` lifetime bound to `T` -->
<!-- to constrain `T` to types that have only `'static` references or no -->
<!-- references</span> -->

<span class="caption">リスト19-18: `T`に`'static`ライフタイム境界を追加して`T`を`'static`参照だけがあるか参照なしの型に制限する</span>

<!-- Because `'static` means the reference must live as long as the entire program, -->
<!-- a type that contains no references meets the criteria of all references living -->
<!-- as long as the entire program (because there are no references). For the borrow -->
<!-- checker concerned about references living long enough, there is no real -->
<!-- distinction between a type that has no references and a type that has -->
<!-- references that live forever: both are the same for determining whether or not -->
<!-- a reference has a shorter lifetime than what it refers to. -->

`'static`は、参照がプログラム全体と同じだけ生きなければならないことを意味するので、何も参照を含まない型は、
全ての参照がプログラム全体と同じだけ生きるという基準を満たします(参照がないからです)。借用チェッカーが、
参照が十分長生きしないと心配することに関しては、参照が何もない型と永久に生きる参照がある型を現実的に区別できません:
どちらも、参照が参照先のライフタイムよりも短いか決定することに関しては同じです。

<!-- ### Inference of Trait Object Lifetimes -->

### トレイトオブジェクトライフタイムの推論

<!-- In Chapter 17 in the “Using Trait Objects that Allow for Values of Different -->
<!-- Types” section, we discussed trait objects, consisting of a trait behind a -->
<!-- reference, that allow us to use dynamic dispatch. We haven’t yet discussed what -->
<!-- happens if the type implementing the trait in the trait object has a lifetime -->
<!-- of its own. Consider Listing 19-19 where we have a trait `Red` and a struct -->
<!-- `Ball`. The `Ball` struct holds a reference (and thus has a lifetime parameter) -->
<!-- and also implements trait `Red`. We want to use an instance of `Ball` as the -->
<!-- trait object `Box<Red>`. -->

第17章の「異なる型の値を許容するトレイトオブジェクトを使用する」節で、参照の背後のトレイトから構成され、
ダイナミック・ディスパッチを使用できるトレイトオブジェクトを議論しました。まだ、トレイトオブジェクトのトレイトを実装する型が、
独自のライフタイムだった時に何が起きるか議論していません。トレイト`Red`と構造体`Ball`があるリスト19-19を考えてください。
`Ball`構造体は参照を保持し(故にライフタイム引数があり)、トレイト`Red`を実装もしています。
`Ball`のインスタンスを`Box<Red>`として使用したいです。

<!-- <span class="filename">Filename: src/main.rs</span> -->

<span class="filename">ファイル名: src/main.rs</span>

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
```

<!-- <span class="caption">Listing 19-19: Using a type that has a lifetime parameter -->
<!-- with a trait object</span> -->

<span class="caption">リスト19-19: トレイトオブジェクトでライフタイム引数のある型を使用する</span>

<!-- This code compiles without any errors, even though we haven’t explicitly -->
<!-- annotated the lifetimes involved in `obj`. This code works because there are -->
<!-- rules for working with lifetimes and trait objects: -->

明示的に`obj`に関連するライフタイムを注釈していないものの、このコードはエラーなくコンパイルできます。
ライフタイムとトレイトオブジェクトと共に働く規則があるので、このコードは動くのです:

<!-- * The default lifetime of a trait object is `'static`. -->
<!-- * With `&'a Trait` or `&'a mut Trait`, the default lifetime of the trait object -->
<!--   is `'a`. -->
<!-- * With a single `T: 'a` clause, the default lifetime of the trait object is -->
<!--   `'a`. -->
<!-- * With multiple clauses like `T: 'a`, there is no default lifetime; we must be -->
<!--   explicit. -->

* トレイトオブジェクトのデフォルトのライフタイムは、`'static`。
* `&'a Trait`や`&'a mut Trait`に関して、トレイトオブジェクトのデフォルトのライフタイムは、`'a`。
* 単独の`T: 'a`節について、トレイトオブジェクトのデフォルトのライフタイムは、`'a`。
* 複数の`T: 'a`のような節について、デフォルトのライフタイムはない; 明示しなければならない。

<!-- When we must be explicit, we can add a lifetime bound on a trait object like -->
<!-- `Box<Red>` using the syntax `Box<Red + 'static>` or `Box<Red + 'a>`, depending -->
<!-- on whether the reference lives for the entire program or not. As with the other -->
<!-- bounds, the syntax adding a lifetime bound means that any implementor of the -->
<!-- `Red` trait that has references inside the type must have the same lifetime -->
<!-- specified in the trait object bounds as those references. -->

明示しなければならない時、`Box<Red>`のようなトレイトオブジェクトに対して、参照がプログラム全体で生きるかどうかにより、
記法`Box<Red + 'static>`か`Box<Red + 'a>`を使用してライフタイム境界を追加できます。他の境界同様、
ライフタイム境界を追記する記法は、型の内部に参照がある`Red`トレイトを実装しているものは全て、
トレイト境界に指定されるライフタイムがそれらの参照と同じにならなければならないことを意味します。

<!-- Next, let’s look at some other advanced features that manage traits. -->

次は、トレイトを管理する他の一部の高度な機能に目を向けましょう。
