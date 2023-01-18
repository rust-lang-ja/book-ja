<!--
## To `panic!` or Not to `panic!`
-->

## `panic!`すべきかするまいか

<!--
So how do you decide when you should `panic!` and when you should return
`Result`? When code panics, there’s no way to recover. You could call `panic!`
for any error situation, whether there’s a possible way to recover or not, but
then you’re making the decision on behalf of the code calling your code that a
situation is unrecoverable. When you choose to return a `Result` value, you
give the calling code options rather than making the decision for it. The
calling code could choose to attempt to recover in a way that’s appropriate for
its situation, or it could decide that an `Err` value in this case is
unrecoverable, so it can call `panic!` and turn your recoverable error into an
unrecoverable one. Therefore, returning `Result` is a good default choice when
you’re defining a function that might fail.
-->

では、`panic!`すべき時と`Result`を返すべき時はどう決定すればいいのでしょうか？コードがパニックしたら、
回復する手段はありません。回復する可能性のある手段の有る無しに関わらず、どんなエラー場面でも`panic!`を呼ぶことはできますが、
そうすると、呼び出す側のコードの立場に立ってこの場面は回復不能だという決定を下すことになります。
`Result`値を返す決定をすると、決断を下すのではなく、呼び出し側に選択肢を与えることになります。
呼び出し側は、場面に合わせて回復を試みることを決定したり、この場合の`Err`値は回復不能と断定して、
`panic!`を呼び出し、回復可能だったエラーを回復不能に変換することもできます。故に、`Result`を返却することは、
失敗する可能性のある関数を定義する際には、いい第一選択肢になります。

<!--
In rare situations, it’s more appropriate to write code that panics instead of
returning a `Result`. Let’s explore why it’s appropriate to panic in examples,
prototype code, and tests. Then we'll discuss situations in which the compiler
can’t tell that failure is impossible, but you as a human can. The chpater will
conclude with some general guidelines on how to decide whether to panic in
library code.
-->

稀な場面では、`Result`を返すよりもパニックするコードを書く方がより適切になることもあります。
例やプロトタイプコード、テストでパニックするのが適切な理由を探ってみましょう。
それからコンパイラではありえない失敗だと気づけなくとも、人間なら気づける場面を議論しましょう。
そして、ライブラリコードでパニックするか決定する方法についての一般的なガイドラインで結論づけましょう。

<!--
### Examples, Prototype Code, and Tests
-->

### 例、プロトタイプコード、テスト

<!--
When you’re writing an example to illustrate some concept, having robust
error-handling code in the example as well can make the example less clear. In
examples, it’s understood that a call to a method like `unwrap` that could
panic is meant as a placeholder for the way that you’d want your application to
handle errors, which can differ based on what the rest of your code is doing.
-->

例を記述して何らかの概念を具体化している時、頑健なエラー処理コードも例に含むことは、例の明瞭さを欠くことになりかねません。
例において、`unwrap`などのパニックする可能性のあるメソッド呼び出しは、
アプリケーションにエラーを処理してほしい方法へのプレースホルダーを意味していると理解され、
これは残りのコードがしていることによって異なる可能性があります。

<!--
Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
before you’re ready to decide how to handle errors. They leave clear markers in
your code for when you’re ready to make your program more robust.
-->

同様に、`unwrap`や`expect`メソッドは、エラーの処理法を決定する準備ができる前、プロトタイプの段階では、
非常に便利です。それらにより、コードにプログラムをより頑健にする時の明らかなマーカーが残されるわけです。

<!--
If a method call fails in a test, we’d want the whole test to fail, even if
that method isn’t the functionality under test. Because `panic!` is how a test
is marked as a failure, calling `unwrap` or `expect` is exactly what should
happen.
-->

メソッド呼び出しがテスト内で失敗したら、そのメソッドがテスト下に置かれた機能ではなかったとしても、
テスト全体が失敗してほしいでしょう。`panic!`が、テストが失敗と印づけられる手段なので、
`unwrap`や`expect`の呼び出しはズバリ起こるべきことです。

<!--
### Cases in Which You Have More Information Than the Compiler
-->

### コンパイラよりもプログラマがより情報を持っている場合

<!--
It would also be appropriate to call `unwrap` when you have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn’t
something the compiler understands. You’ll still have a `Result` value that you
need to handle: whatever operation you’re calling still has the possibility of
failing in general, even though it’s logically impossible in your particular
situation. If you can ensure by manually inspecting the code that you’ll never
have an `Err` variant, it’s perfectly acceptable to call `unwrap`. Here’s an
example:
-->

`Result`が`Ok`値であると確認する何らかの別のロジックがある場合、`unwrap`を呼び出すことは適切でしょうが、
コンパイラは、そのロジックを理解はしません。それでも、処理する必要のある`Result`は存在するでしょう：
呼び出している処理が何であれ、自分の特定の場面では論理的に起こり得なくても、一般的にまだ失敗する可能性はあるわけです。
手動でコードを調査して`Err`列挙子は存在しないと確認できたら、`unwrap`を呼び出すことは完全に受容できることです。
こちらが例です：

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

<!--
We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `127.0.0.1` is a valid IP address, so it’s acceptable to use `unwrap`
here. However, having a hardcoded, valid string doesn’t change the return type
of the `parse` method: we still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is still a possibility
because the compiler isn’t smart enough to see that this string is always a
valid IP address. If the IP address string came from a user rather than being
hardcoded into the program, and therefore *did* have a possibility of failure,
we’d definitely want to handle the `Result` in a more robust way instead.
-->

ハードコードされた文字列を構文解析することで`IpAddr`インスタンスを生成しています。
プログラマには`127.0.0.1`が合法な IP アドレスであることがわかるので、ここで`unwrap`を使用することは、
受容可能なことです。しかしながら、ハードコードされた合法な文字列が存在することは、
`parse`メソッドの戻り値型を変えることにはなりません：それでも得られるのは、`Result`値であり、
コンパイラはまだ`Err`列挙子になる可能性があるかのように`Result`を処理することを強制してきます。
コンパイラは、この文字列が常に合法な IP アドレスであると把握できるほど利口ではないからです。
プログラムにハードコードされるのではなく、IP アドレス文字列がユーザ起源でそれ故に*確かに*失敗する可能性がある場合、
`Result`をもっと頑健な方法で処理したほうが絶対にいいでしょう。

<!--
### Guidelines for Error Handling
-->

### エラー処理のガイドライン

<!--
It’s advisable to have your code panic when it’s possible that your code
could end up in a bad state. In this context, a *bad state* is when some
assumption, guarantee, contract, or invariant has been broken, such as when
invalid values, contradictory values, or missing values are passed to your
code—plus one or more of the following:
-->

コードが悪い状態に陥る可能性があるときにパニックさせるのは、推奨されることです。この文脈において、
*悪い状態*とは、何らかの前提、保証、契約、不変性が破られたことを言い、例を挙げれば、無効な値、
矛盾する値、行方不明な値がコードに渡されることと、さらに以下のいずれか一つ以上の状態であります：

<!--
* The bad state is not something that’s *expected* to happen occasionally.
* Your code after this point needs to rely on not being in this bad state.
* There’s not a good way to encode this information in the types you use.
-->

* 悪い状態がときに起こるとは*予想*されないとき。
* この時点以降、この悪い状態にないことを頼りにコードが書かれているとき。
* 使用している型にこの情報をコード化するいい手段がないとき。

<!--
If someone calls your code and passes in values that don’t make sense, the best
choice might be to `panic!` and alert the person using your library to the
bug in their code so they can fix it during development. Similarly, `panic!` is
often appropriate if you’re calling external code that is out of your control
and it returns an invalid state that you have no way of fixing.
-->

誰かが自分のコードを呼び出して筋の通らない値を渡してきたら、最善の選択肢は`panic!`し、
開発段階で修正できるように自分たちのコードにバグがあることをライブラリ使用者に通知することかもしれません。
同様に自分の制御下にない外部コードを呼び出し、修正しようのない無効な状態を返すときに`panic!`はしばしば適切です。

<!--
When a bad state is reached, but it’s expected to happen no matter how well you
write your code, it’s still more appropriate to return a `Result` rather than
making a `panic!` call. Examples include a parser being given malformed data
or an HTTP request returning a status that indicates you have hit a rate limit.
In these cases, you should indicate that failure is an expected possibility by
returning a `Result` to propagate these bad states upwards so the calling code
can decide how to handle the problem. To call `panic!` wouldn’t be the best way
to handle these cases.
-->

しかし、どんなにコードをうまく書いても起こると予想されますが、悪い状態に達したとき、それでも`panic!`呼び出しをするよりも、
`Result`を返すほうがより適切です。例には、不正なデータを渡されたパーサとか、
訪問制限に引っかかったことを示唆するステータスを返す HTTP リクエストなどが挙げられます。
このような場合には、呼び出し側が問題の処理方法を決定できるように`Result`を返してこの悪い状態を委譲して、
失敗が予想される可能性であることを示唆するべきです。`panic!`を呼び出すことは、
これらのケースでは最善策ではないでしょう。

<!--
When your code performs operations on values, your code should verify the
values are valid first and panic if the values aren’t valid. This is mostly for
safety reasons: attempting to operate on invalid data can expose your code to
vulnerabilities. This is the main reason the standard library will call
`panic!` if you attempt an out-of-bounds memory access: trying to access memory
that doesn’t belong to the current data structure is a common security problem.
Functions often have *contracts*: their behavior is only guaranteed if the
inputs meet particular requirements. Panicking when the contract is violated
makes sense because a contract violation always indicates a caller-side bug,
and it’s not a kind of error you want the calling code to have to explicitly
handle. In fact, there’s no reasonable way for calling code to recover; the
calling *programmers* need to fix the code. Contracts for a function,
especially when a violation will cause a panic, should be explained in the API
documentation for the function.
-->

コードが値に対して処理を行う場合、コードはまず値が合法であることを確認し、
値が合法でなければパニックするべきです。これはほぼ安全性上の理由によるものです：不正なデータの処理を試みると、
コードを脆弱性に晒す可能性があります。これが、境界外へのメモリアクセスを試みたときに標準ライブラリが`panic!`を呼び出す主な理由です：
現在のデータ構造に属しないメモリにアクセスを試みることは、ありふれたセキュリティ問題なのです。
関数にはしばしば*契約*が伴います：入力が特定の条件を満たすときのみ、振る舞いが保証されるのです。
契約が侵されたときにパニックすることは、道理が通っています。なぜなら、契約侵害は常に呼び出し側のバグを示唆し、
呼び出し側に明示的に処理してもらう必要のある種類のエラーではないからです。実際に、
呼び出し側が回復する合理的な手段はありません; 呼び出し側の*プログラマ*がコードを修正する必要があるのです。
関数の契約は、特に侵害がパニックを引き起こす際には、関数の API ドキュメント内で説明されているべきです。

<!--
However, having lots of error checks in all of your functions would be verbose
and annoying. Fortunately, you can use Rust’s type system (and thus the type
checking the compiler does) to do many of the checks for you. If your function
has a particular type as a parameter, you can proceed with your code’s logic
knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*. Your code then doesn’t have to handle
two cases for the `Some` and `None` variants: it will only have one case for
definitely having a value. Code trying to pass nothing to your function won’t
even compile, so your function doesn’t have to check for that case at runtime.
Another example is using an unsigned integer type like `u32`, which ensures
the parameter is never negative.
-->

ですが、全ての関数でたくさんのエラーチェックを行うことは冗長で煩わしいことでしょう。幸運にも、
Rust の型システム (故にコンパイラが行う型精査) を使用して多くの検査を行ってもらうことができます。
関数の引数に特定の型があるなら、合法な値があるとコンパイラがすでに確認していることを把握して、
コードのロジックに進むことができます。例えば、`Option`以外の型がある場合、プログラムは、
*何もない*ではなく*何かある*と想定します。そうしたらコードは、
`Some`と`None`列挙子の 2 つの場合を処理する必要がなくなるわけです：
確実に値があるという可能性しかありません。関数に何もないことを渡そうとしてくるコードは、
コンパイルが通りもしませんので、その場合を実行時に検査する必要はないわけです。
別の例は、`u32`のような符号なし整数を使うことであり、この場合、引数は負には絶対にならないことが確認されます。

<!--
### Creating Custom Types for Validation
-->

### 検証のために独自の型を作る

<!--
Let’s take the idea of using Rust’s type system to ensure we have a valid value
one step further and look at creating a custom type for validation. Recall the
guessing game in Chapter 2 in which our code asked the user to guess a number
between 1 and 100. We never validated that the user’s guess was between those
numbers before checking it against our secret number; we only validated that
the guess was positive. In this case, the consequences were not very dire: our
output of “Too high” or “Too low” would still be correct. It would be a
useful enhancement to guide the user toward valid guesses and have different
behavior when a user guesses a number that’s out of range versus when a user
types, for example, letters instead.
-->

Rust の型システムを使用して合法な値があると確認するというアイディアを一歩先に進め、
検証のために独自の型を作ることに目を向けましょう。第 2 章の数当てゲームで、
コードがユーザに 1 から 100 までの数字を推測するよう求めたことを思い出してください。
秘密の数字と照合する前にユーザの推測がそれらの値の範囲にあることを全く確認しませんでした;
推測が正であることしか確認しませんでした。この場合、結果はそれほど悲惨なものではありませんでした：
「大きすぎ」、「小さすぎ」という出力は、それでも正しかったでしょう。ユーザを合法な推測に導き、
ユーザが範囲外の数字を推測したり、例えばユーザが文字を代わりに入力したりしたときに別の挙動をするようにしたら、
有益な改善になるでしょう。

<!--
One way to do this would be to parse the guess as an `i32` instead of only a
`u32` to allow potentially negative numbers, and then add a check for the
number being in range, like so:
-->

これをする一つの方法は、ただの`u32`の代わりに`i32`として推測をパースし、負の数になる可能性を許可し、
それから数字が範囲に収まっているというチェックを追加することでしょう。そう、以下のように：

```rust,ignore
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}
```

<!--
The `if` expression checks whether our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.
-->

この`if`式が、値が範囲外かどうかをチェックし、ユーザに問題を告知し、`continue`を呼び出してループの次の繰り返しを始め、
別の推測を求めます。`if`式の後、`guess`は 1 から 100 の範囲にあると把握して、`guess`と秘密の数字の比較に進むことができます。

<!--
However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).
-->

ところが、これは理想的な解決策ではありません：プログラムが 1 から 100 の範囲の値しか処理しないことが間違いなく、
肝要であり、この要求がある関数の数が多ければ、このようなチェックを全関数で行うことは、
面倒でパフォーマンスにも影響を及ぼす可能性があるでしょう。

<!--
Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-9 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100.
-->

代わりに、新しい型を作って検証を関数内に閉じ込め、検証を全箇所で繰り返すのではなく、
その型のインスタンスを生成することができます。そうすれば、関数がその新しい型をシグニチャに用い、
受け取った値を自信を持って使用することは安全になります。リスト 9-9 に、`new`関数が 1 から 100 までの値を受け取った時のみ、
`Guess`のインスタンスを生成する`Guess`型を定義する一つの方法を示しました。

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            // 予想の値は 1 から 100 の範囲でなければなりませんが、{}でした
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

<!--
<span class="caption">Listing 9-9: A `Guess` type that will only continue with
values between 1 and 100</span>
-->

<span class="caption">リスト 9-9: 値が 1 から 100 の場合のみ処理を継続する`Guess`型</span>

<!--
First, we define a struct named `Guess` that has a field named `value` that
holds a `u32`. This is where the number will be stored.
-->

まず、`u32`型の`value`をフィールドに持つ`Guess`という名前の構造体を定義しています。
ここに数値が保管されます。

<!--
Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `u32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it’s between 1 and 100.
If `value` doesn’t pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we’ll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.
-->

それから`Guess`に`Guess`値のインスタンスを生成する`new`という名前の関連関数を実装しています。
`new`関数は、`u32`型の`value`という引数を取り、`Guess`を返すように定義されています。
`new`関数の本体のコードは、`value`をふるいにかけ、1 から 100 の範囲であることを確かめます。
`value`がふるいに引っかかったら、`panic!`呼び出しを行います。これにより、呼び出しコードを書いているプログラマに、
修正すべきバグがあると警告します。というのも、この範囲外の`value`で`Guess`を生成することは、
`Guess::new`が頼りにしている契約を侵害するからです。`Guess::new`がパニックするかもしれない条件は、
公開されている API ドキュメントで議論されるべきでしょう; あなたが作成する API ドキュメントで`panic!`の可能性を示唆する、
ドキュメントの規約は、第 14 章で講義します。`value`が確かにふるいを通ったら、
`value`フィールドが`value`引数にセットされた新しい`Guess`を作成して返します。

<!--
Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns a `u32`. This kind of method is sometimes called
a *getter*, because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It’s important that the `value` field is private so code
using the `Guess` struct is not allowed to set `value` directly: code outside
the module *must* use the `Guess::new` function to create an instance of
`Guess`, thereby ensuring there’s no way for a `Guess` to have a `value` that
hasn’t been checked by the conditions in the `Guess::new` function.
-->

次に、`self`を借用し、他に引数はなく、`u32`を返す`value`というメソッドを実装します。
この類のメソッドは時に*ゲッター*と呼ばれます。目的がフィールドから何らかのデータを得て返すことだからです。
この公開メソッドは、`Guess`構造体の`value`フィールドが非公開なので、必要になります。
`value`フィールドが非公開なことは重要であり、そのために`Guess`構造体を使用するコードは、
直接`value`をセットすることが叶わないのです：モジュール外のコードは、
`Guess::new`関数を使用して`Guess`のインスタンスを生成し*なければならず*、
それにより、`Guess::new`関数の条件式でチェックされていない`value`が`Guess`に存在する手段はないことが保証されるわけです。

<!--
A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than a
`u32` and wouldn’t need to do any additional checks in its body.
-->

そうしたら、引数を一つ持つか、1 から 100 の範囲の数値のみを返す関数は、シグニチャで`u32`ではなく、
`Guess`を取るか返し、本体内で追加の確認を行う必要はなくなると宣言できるでしょう。

<!--
## Summary
-->

## まとめ

<!--
Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.
-->

Rust のエラー処理機能は、プログラマがより頑健なコードを書く手助けをするように設計されています。
`panic!`マクロは、プログラムが処理できない状態にあり、無効だったり不正な値で処理を継続するのではなく、
プロセスに処理を中止するよう指示することを通知します。`Result` enum は、Rust の型システムを使用して、
コードが回復可能な方法で処理が失敗するかもしれないことを示唆します。`Result`を使用して、
呼び出し側のコードに成功や失敗する可能性を処理する必要があることも教えます。
適切な場面で`panic!`や`Result`を使用することで、必然的な問題の眼前でコードの信頼性を上げてくれます。

<!--
Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.
-->

今や、標準ライブラリが`Option`や`Result` enum などでジェネリクスを有効活用するところを目の当たりにしたので、
ジェネリクスの動作法と自分のコードでの使用方法について語りましょう。
