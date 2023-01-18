<!--
## Closures: Anonymous Functions that Can Capture Their Environment
-->

## クロージャ：環境をキャプチャできる匿名関数

<!--
Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure to evaluate it in a different context. Unlike functions,
closures can capture values from the scope in which they’re called. We’ll
demonstrate how these closure features allow for code reuse and behavior
customization.
-->

Rust のクロージャは、変数に保存したり、引数として他の関数に渡すことのできる匿名関数です。
ある場所でクロージャを生成し、それから別の文脈でクロージャを呼び出して評価することができます。
関数と異なり、呼び出されたスコープの値をクロージャは、キャプチャすることができます。
これらのクロージャの機能がコードの再利用や、動作のカスタマイズを行わせてくれる方法を模擬しましょう。

<!--
### Creating an Abstraction of Behavior with Closures
-->

### クロージャで動作の抽象化を行う

<!--
Let’s work on an example of a situation in which it’s useful to store a closure
to be executed later. Along the way, we’ll talk about the syntax of closures,
type inference, and traits.
-->

クロージャを保存して後々使用できるようにするのが有用な場面の例に取り掛かりましょう。その過程で、
クロージャの記法、型推論、トレイトについて語ります。

<!--
Consider this hypothetical situation: we work at a startup that’s making an app
to generate custom exercise workout plans. The backend is written in Rust, and
the algorithm that generates the workout plan takes into account many factors,
such as the app user’s age, body mass index, exercise preferences, recent
workouts, and an intensity number they specify. The actual algorithm used isn’t
important in this example; what’s important is that this calculation takes a
few seconds. We want to call this algorithm only when we need to and only call
it once so we don’t make the user wait more than necessary.
-->

以下のような架空の場面を考えてください：カスタマイズされたエクササイズのトレーニングプランを生成するアプリを作るスタートアップで働くことになりました。
バックエンドは Rust で記述され、トレーニングプランを生成するアルゴリズムは、アプリユーザの年齢や、
BMI、運動の好み、最近のトレーニング、指定された強弱値などの多くの要因を考慮します。
実際に使用されるアルゴリズムは、この例では重要ではありません; 重要なのは、この計算が数秒要することです。
必要なときだけこのアルゴリズムを呼び出し、1 回だけ呼び出したいので、必要以上にユーザを待たせないことになります。

<!--
We’ll simulate calling this hypothetical algorithm with the
`simulated_expensive_calculation` shown in Listing 13-1, which will print
`calculating slowly...`, wait for two seconds, and then return whatever number
we passed in.
-->

リスト 13-1 に示した`simulated_expensive_calculation`関数でこの仮定のアルゴリズムを呼び出すことをシミュレートし、
この関数は`calculating slowly`と出力し、2 秒待ってから、渡した数値をなんでも返します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // ゆっくり計算します
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

<!--
<span class="caption">Listing 13-1: A function to stand in for a hypothetical
calculation that takes about 2 seconds to run</span>
-->

<span class="caption">リスト 13-1: 実行に約 2 秒かかる架空の計算の代役を務める関数</span>

<!--
Next is the `main` function that contains the parts of the workout app
important for this example. This function represents the code that the app will
call when a user asks for a workout plan. Because the interaction with the
app’s frontend isn’t relevant to the use of closures, we’ll hardcode values
representing inputs to our program and print the outputs.
-->

次は、この例で重要なトレーニングアプリの部分を含む`main`関数です。この関数は、
ユーザがトレーニングプランを要求した時にアプリが呼び出すコードを表します。
アプリのフロントエンドと相互作用する部分は、クロージャの使用と関係ないので、プログラムへの入力を表す値をハードコードし、
その出力を表示します。

<!--
The required inputs are these:
-->

必要な入力は以下の通りです：

<!--
* An intensity number from the user, which is specified when they request
a workout to indicate whether they want a low-intensity workout or a
high-intensity workout.
* A random number that will generate some variety in the workout plans.
-->

* ユーザの強弱値、これはユーザがトレーニングを要求して、低強度のトレーニングか、
高強度のトレーニングがしたいかを示したときに指定されます。
* 乱数、これはトレーニングプランにバリエーションを起こします。

<!--
The output will be the recommended workout plan. Listing 13-2 shows the `main`
function we’ll use.
-->

出力は、推奨されるトレーニングプランになります。リスト 13-2 は使用する`main`関数を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
# fn generate_workout(intensity: u32, random_number: u32) {}
```

<!--
<span class="caption">Listing 13-2: A `main` function with hardcoded values to
simulate user input and random number generation</span>
-->

<span class="caption">リスト 13-2: ユーザ入力や乱数生成をシミュレートするハードコードされた値がある`main`関数</span>

<!--
We’ve hardcoded the variable `simulated_user_specified_value` as 10 and the
variable `simulated_random_number` as 7 for simplicity’s sake; in an actual
program, we’d get the intensity number from the app frontend, and we’d use the
`rand` crate to generate a random number, as we did in the Guessing Game
example in Chapter 2. The `main` function calls a `generate_workout` function
with the simulated input values.
-->

簡潔性のために、変数`simulated_user_specified_value`は 10、変数`simulated_random_number`は 7 とハードコードしました;
実際のプログラムにおいては、強弱値はアプリのフロントエンドから取得し、乱数の生成には、第 2 章の数当てゲームの例のように、`rand`クレートを使用するでしょう。
`main`関数は、シミュレートされた入力値とともに`generate_workout`関数を呼び出します。

<!--
Now that we have the context, let’s get to the algorithm. The
`generate_workout` function in Listing 13-3 contains the business logic of the
app that we’re most concerned with in this example. The rest of the code
changes in this example will be made to this function.
-->

今や文脈ができたので、アルゴリズムに取り掛かりましょう。リスト 13-3 の`generate_workout`関数は、
この例で最も気にかかるアプリのビジネスロジックを含んでいます。この例での残りの変更は、
この関数に対して行われるでしょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {

        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

<!--
<span class="caption">Listing 13-3: The business logic that prints the workout
plans based on the inputs and calls to the `simulated_expensive_calculation`
function</span>
-->

<span class="caption">リスト 13-3: 入力に基づいてトレーニングプランを出力するビジネスロジックと、
`simulated_expensive_calculation`関数の呼び出し</span>

<!--
The code in Listing 13-3 has multiple calls to the slow calculation function.
The first `if` block calls `simulated_expensive_calculation` twice, the `if`
inside the outer `else` doesn’t call it at all, and the code inside the
second `else` case calls it once.
-->

リスト 13-3 のコードには、遅い計算を行う関数への呼び出しが複数あります。最初の`if`ブロックが、
`simulated_expensive_calculation`を 2 回呼び出し、外側の`else`内の`if`は全く呼び出さず、
2 番目の`else`ケースの内側にあるコードは 1 回呼び出しています。

<!--
NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199
-->

<!--
The desired behavior of the `generate_workout` function is to first check
whether the user wants a low-intensity workout (indicated by a number less
than 25) or a high-intensity workout (a number of 25 or greater).
-->

`generate_workout`関数の期待される振る舞いは、まずユーザが低強度のトレーニング (25 より小さい数値で表される) か、
高強度のトレーニング (25 以上の数値) を欲しているか確認することです。

<!--
Low-intensity workout plans will recommend a number of push-ups and sit-ups
based on the complex algorithm we’re simulating.
-->

低強度のトレーニングプランは、シミュレーションしている複雑なアルゴリズムに基づいて、
多くの腕立て伏せや腹筋運動を推奨してきます。

<!--
If the user wants a high-intensity workout, there’s some additional logic: if
the value of the random number generated by the app happens to be 3, the app
will recommend a break and hydration. If not, the user will get a number of
minutes of running based on the complex algorithm.
-->

ユーザが高強度のトレーニングを欲していれば、追加のロジックがあります：アプリが生成した乱数がたまたま 3 なら、
アプリは休憩と水分補給を勧めます。そうでなければ、ユーザは複雑なアルゴリズムに基づいて数分間のランニングをします。

<!--
This code works the way the business wants it to now, but let's say the data
science team decides that we need to make some changes to the way we call the
`simulated_expensive_calculation` function in the future. To simplify the
update when those changes happen, we want to refactor this code so it calls the
`simulated_expensive_calculation` function only once. We also want to cut the
place where we’re currently unnecessarily calling the function twice without
adding any other calls to that function in the process. That is, we don’t want
to call it if the result isn’t needed, and we still want to call it only once.
-->

このコードは現在、ビジネスのほしいままに動くでしょうが、データサイエンスチームが、
`simulated_expensive_calculation`関数を呼び出す方法に何らかの変更を加える必要があると決定したとしましょう。
そのような変更が起きた時に更新を簡略化するため、`simulated_expensive_calculation`関数を 1 回だけ呼び出すように、
このコードをリファクタリングしたいです。また、その過程でその関数への呼び出しを増やすことなく無駄に 2 回、
この関数を現時点で呼んでいるところを切り捨てたくもあります。要するに、結果が必要なければ関数を呼び出したくなく、
それでも 1 回だけ呼び出したいのです。

<!--
#### Refactoring Using Functions
-->

#### 関数でリファクタリング

<!--
We could restructure the workout program in many ways. First, we’ll try
extracting the duplicated call to the `simulated_expensive_calculation`
function into a variable, as shown in Listing 13-4.
-->

多くの方法でトレーニングプログラムを再構築することもできます。
1 番目に`simulated_expensive_calculation`関数への重複した呼び出しを変数に抽出しようとしましょう。リスト 13-4 に示したように。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

<!--
<span class="caption">Listing 13-4: Extracting the calls to
`simulated_expensive_calculation` to one place and storing the result in the
`expensive_result` variable</span>
-->

<span class="caption">リスト 13-4: 複数の`simulated_expensive_calculation`の呼び出しを 1 箇所に抽出し、
結果を`expensive_result`変数に保存する</span>

<!--
This change unifies all the calls to `simulated_expensive_calculation` and
solves the problem of the first `if` block unnecessarily calling the function
twice. Unfortunately, we’re now calling this function and waiting for the
result in all cases, which includes the inner `if` block that doesn’t use the
result value at all.
-->

この変更により`simulated_expensive_calculation`の呼び出しが単一化され、
最初の`if`ブロックが無駄に関数を 2 回呼んでいた問題を解決します。不幸なことに、これでは、
あらゆる場合にこの関数を呼び出し、その結果を待つことになり、結果値を全く使用しない内側の`if`ブロックでもそうしてしまいます。

<!--
We want to define code in one place in our program, but only *execute* that
code where we actually need the result. This is a use case for closures!
-->

プログラムの 1 箇所でコードを定義したいですが、結果が本当に必要なところでだけコードを*実行*します。
これは、クロージャのユースケースです！

<!--
#### Refactoring with Closures to Store Code
-->

#### クロージャでリファクタリングして、コードを保存する

<!--
Instead of always calling the `simulated_expensive_calculation` function before
the `if` blocks, we can define a closure and store the *closure* in a variable
rather than storing the result of the function call, as shown in Listing 13-5.
We can actually move the whole body of `simulated_expensive_calculation` within
the closure we’re introducing here.
-->

`if`ブロックの前にいつも`simulated_expensive_calculation`関数を呼び出す代わりに、
クロージャを定義し、関数呼び出しの結果を保存するのではなく、その*クロージャ*を変数に保存できます。リスト 13-5 のようにですね。
`simulated_expensive_calculation`の本体全体を実際に、ここで導入しているクロージャ内に移すことができます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);
```

<!--
<span class="caption">Listing 13-5: Defining a closure and storing it in the
`expensive_closure` variable</span>
-->

<span class="caption">リスト 13-5: クロージャを定義し、`expensive_closure`変数に保存する</span>

<!--
The closure definition comes after the `=` to assign it to the variable
`expensive_closure`. To define a closure, we start with a pair of vertical
pipes (`|`), inside which we specify the parameters to the closure; this syntax
was chosen because of its similarity to closure definitions in Smalltalk and
Ruby. This closure has one parameter named `num`: if we had more than one
parameter, we would separate them with commas, like `|param1, param2|`.
-->

クロージャ定義が`=`に続き、変数`expensive_closure`に代入されています。クロージャを定義するには、
1 組の縦棒から始め、その内部にクロージャの仮引数を指定します; この記法は、Smalltalk や Ruby のクロージャ定義と類似していることから、
選択されました。このクロージャには、`num`という引数が 1 つあります：2 つ以上引数があるなら、
`|param1, param2|`のように、カンマで区切ります。

<!--
After the parameters, we place curly brackets that hold the body of the
closure—these are optional if the closure body is a single expression. The end
of the closure, after the curly brackets, needs a semicolon to complete the
`let` statement. The value returned from the last line in the closure body
(`num`) will be the value returned from the closure when it’s called, because
that line doesn’t end in a semicolon; just as in function bodies.
-->

引数の後に、クロージャの本体を保持する波括弧を配置します (これはクロージャ本体が式一つなら省略可能です)。
波括弧の後、クロージャのお尻には、セミコロンが必要で、`let`文を完成させます。クロージャ本体の最後の行から返る値 (`num`) が、
呼び出された時にクロージャから返る値になります。その行がセミコロンで終わっていないからです;
ちょうど関数の本体みたいですね。

<!--
Note that this `let` statement means `expensive_closure` contains the
*definition* of an anonymous function, not the *resulting value* of calling the
anonymous function. Recall that we’re using a closure because we want to define
the code to call at one point, store that code, and call it at a later point;
the code we want to call is now stored in `expensive_closure`.
-->

この`let`文は、`expensive_closure`が、匿名関数を呼び出した*結果の値*ではなく、
匿名関数の*定義*を含むことを意味することに注意してください。コードを定義して、
1 箇所で呼び出し、そのコードを保存し、後々、それを呼び出したいがためにクロージャを使用していることを思い出してください;
呼び出したいコードは、現在、`expensive_closure`に保存されています。

<!--
この冒頭の with も順接の理由にしている。やはり強すぎるか？
-->

<!--
With the closure defined, we can change the code in the `if` blocks to call the
closure to execute the code and get the resulting value. We call a closure like
we do a function: we specify the variable name that holds the closure
definition and follow it with parentheses containing the argument values we
want to use, as shown in Listing 13-6.
-->

クロージャが定義されたので、`if`ブロックのコードを変更して、そのコードを実行するクロージャを呼び出し、結果値を得ることができます。
クロージャは、関数のように呼び出せます：クロージャ定義を含む変数名を指定し、使用したい引数値を含むかっこを続けます。
リスト 13-6 に示したようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

<!--
<span class="caption">Listing 13-6: Calling the `expensive_closure` we’ve
defined</span>
-->

<span class="caption">リスト 13-6: 定義した`expensive_closure`を呼び出す</span>

<!--
2 行目最後は、今の通りにも (at) where のようにも取れるか？
-->

<!--
Now the expensive calculation is called in only one place, and we’re only
executing that code where we need the results.
-->

今では、重い計算はたった 1 箇所でのみ呼び出され、その結果が必要なコードを実行するだけになりました。

<!--
However, we’ve reintroduced one of the problems from Listing 13-3: we’re still
calling the closure twice in the first `if` block, which will call the
expensive code twice and make the user wait twice as long as they need to. We
could fix this problem by creating a variable local to that `if` block to hold
the result of calling the closure, but closures provide us with another
solution. We’ll talk about that solution in a bit. But first let’s talk about
why there aren’t type annotations in the closure definition and the traits
involved with closures.
-->

ところが、リスト 13-3 の問題の一つを再浮上させてしまいました：それでも、最初の`if`ブロックでクロージャを 2 回呼んでいて、
そうすると、重いコードを 2 回呼び出し、必要な分の 2 倍ユーザを待たせてしまいます。その`if`ブロックのみに属する変数を生成して、
クロージャの呼び出し結果を保持するその`if`ブロックに固有の変数を生成することでこの問題を解消することもできますが、
クロージャは他の解決法も用意してくれます。その解決策については、もう少し先で語りましょう。でもまずは、
クロージャ定義に型注釈がない理由とクロージャに関わるトレイトについて話しましょう。

<!--
### Closure Type Inference and Annotation
-->

### クロージャの型推論と注釈

<!--
Closures don’t require you to annotate the types of the parameters or the
return value like `fn` functions do. Type annotations are required on functions
because they’re part of an explicit interface exposed to your users. Defining
this interface rigidly is important for ensuring that everyone agrees on what
types of values a function uses and returns. But closures aren’t used in an
exposed interface like this: they’re stored in variables and used without
naming them and exposing them to users of our library.
-->

クロージャでは、`fn`関数のように引数の型や戻り値の型を注釈する必要はありません。関数では、
型注釈は必要です。ユーザに露出する明示的なインターフェイスの一部だからです。このインターフェイスを堅実に定義することは、
関数が使用したり、返したりする値の型についてみんなが合意していることを保証するために重要なのです。
しかし、クロージャはこのような露出するインターフェイスには使用されません：変数に保存され、
名前付けしたり、ライブラリの使用者に晒されることなく、使用されます。

<!--
Closures are usually short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler is
reliably able to infer the types of the parameters and the return type, similar
to how it’s able to infer the types of most variables.
-->

クロージャは通常短く、あらゆる任意の筋書きではなく、狭い文脈でのみ関係します。
このような限定された文脈内では、コンパイラは、多くの変数の型を推論できるのと似たように、
引数や戻り値の型を頼もしく推論することができます。

<!--
Making programmers annotate the types in these small, anonymous functions would
be annoying and largely redundant with the information the compiler already has
available.
-->

このような小さく匿名の関数で型をプログラマに注釈させることは煩わしいし、コンパイラがすでに利用可能な情報と大きく被っています。

<!--
As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for the closure we defined in Listing 13-5
would look like the definition shown in Listing 13-7.
-->

本当に必要な以上に冗長になることと引き換えに、明示性と明瞭性を向上させたいなら、変数に型注釈を加えることもできます;
リスト 13-5 で定義したクロージャに型を注釈するなら、リスト 13-7 に示した定義のようになるでしょう。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

<!--
<span class="caption">Listing 13-7: Adding optional type annotations of the
parameter and return value types in the closure</span>
-->

<span class="caption">リスト 13-7: クロージャの引数と戻り値の省略可能な型注釈を追加する</span>

<!--
With type annotations added, the syntax of closures looks more similar to the
syntax of functions. The following is a vertical comparison of the syntax for
the definition of a function that adds 1 to its parameter and a closure that
has the same behavior. We’ve added some spaces to line up the relevant parts.
This illustrates how closure syntax is similar to function syntax except for
the use of pipes and the amount of syntax that is optional:
-->

型注釈を付け加えると、クロージャの記法は、関数の記法により酷似して見えます。以下が、引数に 1 を加える関数の定義と、
同じ振る舞いをするクロージャの定義の記法を縦に比べたものです。
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
annotated closure definition. The third line removes the type annotations from
the closure definition, and the fourth line removes the brackets, which are
optional because the closure body has only one expression. These are all valid
definitions that will produce the same behavior when they’re called.
-->

1 行目が関数定義を示し、2 行目がフルに注釈したクロージャ定義を示しています。
3 行目は、クロージャ定義から型注釈を取り除き、4 行目は、かっこを取り除いていて、
かっこはクロージャの本体がただ 1 つの式からなるので、省略可能です。これらは全て、
呼び出された時に同じ振る舞いになる合法な定義です。

<!--
Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, Listing 13-8 shows the
definition of a short closure that just returns the value it receives as a
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition: if
we then try to call the closure twice, using a `String` as an argument the
first time and a `u32` the second time, we’ll get an error.
-->

クロージャ定義には、引数それぞれと戻り値に対して推論される具体的な型が一つあります。例えば、
リスト 13-8 に引数として受け取った値を返すだけの短いクロージャの定義を示しました。
このクロージャは、この例での目的以外には有用ではありません。この定義には、
何も型注釈を加えていないことに注意してください：それから 1 回目に`String`を引数に、
2 回目に`u32`を引数に使用してこのクロージャを 2 回呼び出そうとしたら、エラーになります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

<!--
<span class="caption">Listing 13-8: Attempting to call a closure whose types
are inferred with two different types</span>
-->

<span class="caption">リスト 13-8: 2 つの異なる型で型が推論されるクロージャの呼び出しを試みる</span>

<!--
The compiler gives us this error:
-->

コンパイラは、次のエラーを返します：

```text
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```

<!--
The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked in to the closure in `example_closure`, and we get a type
error if we try to use a different type with the same closure.
-->

`String`値で`example_closure`を呼び出した最初の時点で、コンパイラは`x`とクロージャの戻り値の型を`String`と推論します。
そして、その型が`example_closure`のクロージャに閉じ込められ、同じクロージャを異なる型で使用しようとすると、
型エラーが出るのです。

<!--
### Storing Closures Using Generic Parameters and the `Fn` Traits
-->

### ジェネリック引数と`Fn`トレイトを使用してクロージャを保存する

<!--
Let’s return to our workout generation app. In Listing 13-6, our code was still
calling the expensive calculation closure more times than it needed to. One
option to solve this issue is to save the result of the expensive closure in a
variable for reuse and use the variable in each place we need the result,
instead of calling the closure again. However, this method could result in a
lot of repeated code.
-->

トレーニング生成アプリに戻りましょう。リスト 13-6 において、まだコードは必要以上の回数、重い計算のクロージャを呼んでいました。
この問題を解決する一つの選択肢は、重いクロージャの結果を再利用できるように変数に保存し、クロージャを再度呼ぶ代わりに、
結果が必要になる箇所それぞれでその変数を使用することです。しかしながら、この方法は同じコードを大量に繰り返す可能性があります。

<!--
Fortunately, another solution is available to us. We can create a struct that
will hold the closure and the resulting value of calling the closure. The
struct will execute the closure only if we need the resulting value, and it
will cache the resulting value so the rest of our code doesn’t have to be
responsible for saving and reusing the result. You may know this pattern as
*memoization* or *lazy evaluation*.
-->

運のいいことに、別の解決策もあります。クロージャやクロージャの呼び出し結果の値を保持する構造体を作れるのです。
結果の値が必要な場合のみにその構造体はクロージャを実行し、その結果の値をキャッシュするので、残りのコードは、
結果を保存し、再利用する責任を負わなくて済むのです。このパターンは、*メモ化*(memoization) または、
*遅延評価*(lazy evaluation) として知っているかもしれません。

<!--
5 行目、structs, enums に that がかかるか曖昧だが、この訳の方が自然と思われる
-->

<!--
To make a struct that holds a closure, we need to specify the type of the
closure, because a struct definition needs to know the types of each of its
fields. Each closure instance has its own unique anonymous type: that is, even
if two closures have the same signature, their types are still considered
different. To define structs, enums, or function parameters that use closures,
we use generics and trait bounds, as we discussed in Chapter 10.
-->

クロージャを保持する構造体を作成するために、クロージャの型を指定する必要があります。
構造体定義は、各フィールドの型を把握しておく必要がありますからね。各クロージャインスタンスには、
独自の匿名の型があります：つまり、たとえ 2 つのクロージャが全く同じシグニチャでも、その型はそれでも違うものと考えられるということです。
クロージャを使用する構造体、enum、関数引数を定義するには、第 10 章で議論したように、
ジェネリクスとトレイト境界を使用します。

<!--
The `Fn` traits are provided by the standard library. All closures implement
one of the traits: `Fn`, `FnMut`, or `FnOnce`. We’ll discuss the
difference between these traits in the "Capturing the Environment with
Closures" section; in this example, we can use the `Fn` trait.
-->

`Fn`トレイトは、標準ライブラリで用意されています。全てのクロージャは、以下のいずれかのトレイトを実装しています：
`Fn`、`FnMut`または、`FnOnce`です。「クロージャで環境をキャプチャする」節で、これらのトレイト間の差異を議論します;
この例では、`Fn`トレイトを使えます。

<!--
We add types to the `Fn` trait bound to represent the types of the parameters
and return values the closures must have to match this trait bound. In this
case, our closure has a parameter of type `u32` and returns a `u32`, so the
trait bound we specify is `Fn(u32) -> u32`.
-->

`Fn`トレイト境界にいくつかの型を追加することで、このトレイト境界に合致するクロージャが持つべき引数と戻り値の型を示します。
今回のクロージャは`u32`型の引数を一つ取り、`u32`を返すので、指定するトレイト境界は`Fn(u32) -> u32`になります。

<!--
Listing 13-9 shows the definition of the `Cacher` struct that holds a closure
and an optional result value.
-->

リスト 13-9 は、クロージャとオプションの結果値を保持する`Cacher`構造体の定義を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

<!--
<span class="caption">Listing 13-9: Defining a `Cacher` struct that holds a
closure in `calculation` and an optional result in `value`</span>
-->

<span class="caption">リスト 13-9: クロージャを`calculation`に、オプションの結果値を`value`に保持する`Cacher`構造体を定義する</span>

<!--
The `Cacher` struct has a `calculation` field of the generic type `T`. The
trait bounds on `T` specify that it’s a closure by using the `Fn` trait. Any
closure we want to store in the `calculation` field must have one `u32`
parameter (specified within the parentheses after `Fn`) and must return a
`u32` (specified after the `->`).
-->

`Cacher`構造体は、ジェネリックな型`T`の`calculation`フィールドを持ちます。`T`のトレイト境界は、
`Fn`トレイトを使うことでクロージャであると指定しています。`calculation`フィールドに保存したいクロージャは全て、
1 つの`u32`引数 (`Fn`の後の括弧内で指定されている) を取り、`u32`(`->`の後に指定されている) を返さなければなりません。

<!--
> Note: Functions implement all three of the `Fn` traits too. If what we want
> to do doesn’t require capturing a value from the environment, we can use a
> function rather than a closure where we need something that implements an `Fn`
> trait.
-->

> 注釈：関数も 3 つの`Fn`トレイト全部を実装します。もし環境から値をキャプチャする必要がなければ、
> `Fn`トレイトを実装する何かが必要になるクロージャではなく、関数を使用できます。

<!--
The `value` field is of type `Option<u32>`. Before we execute the closure,
`value` will be `None`. When code using a `Cacher` asks for the *result* of the
closure, the `Cacher` will execute the closure at that time and store the
result within a `Some` variant in the `value` field. Then if the code asks for
the result of the closure again, instead of executing the closure again, the
`Cacher` will return the result held in the `Some` variant.
-->

`value`フィールドの型は、`Option<u32>`です。クロージャを実行する前に、`value`は`None`になるでしょう。
`Cacher`を使用するコードがクロージャの*結果*を求めてきたら、その時点で`Cacher`はクロージャを実行し、
その結果を`value`フィールドの`Some`列挙子に保存します。それから、コードが再度クロージャの結果を求めたら、
クロージャを再実行するのではなく、`Cacher`は`Some`列挙子に保持された結果を返すでしょう。

<!--
The logic around the `value` field we’ve just described is defined in Listing
13-10.
-->

たった今解説した`value`フィールド周りのロジックは、リスト 13-10 で定義されています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

<!--
<span class="caption">Listing 13-10: The caching logic of `Cacher`</span>
-->

<span class="caption">リスト 13-10: `Cacher`のキャッシュ機構</span>

<!--
We want `Cacher` to manage the struct fields’ values rather than letting the
calling code potentially change the values in these fields directly, so these
fields are private.
-->

呼び出し元のコードにこれらのフィールドの値を直接変えてもらうのではなく、`Cacher`に構造体のフィールドの値を管理してほしいので、
これらのフィールドは非公開になっています。

<!--
The `Cacher::new` function takes a generic parameter `T`, which we’ve defined
as having the same trait bound as the `Cacher` struct. Then `Cacher::new`
returns a `Cacher` instance that holds the closure specified in the
`calculation` field and a `None` value in the `value` field, because we haven’t
executed the closure yet.
-->

`Cacher::new`関数はジェネリックな引数の`T`を取り、`Cacher`構造体と同じトレイト境界を持つよう定義しました。
それから`calculation`フィールドに指定されたクロージャと、
`value`フィールドに`None`値を保持する`Cacher`インスタンスを`Cacher::new`は返します。
まだクロージャを実行していないからですね。

<!--
When the calling code needs the result of evaluating the closure, instead of
calling the closure directly, it will call the `value` method. This method
checks whether we already have a resulting value in `self.value` in a `Some`;
if we do, it returns the value within the `Some` without executing the closure
again.
-->

呼び出し元のコードがクロージャの評価結果を必要としたら、クロージャを直接呼ぶ代わりに、`value`メソッドを呼びます。
このメソッドは、結果の値が`self.value`の`Some`に既にあるかどうか確認します; そうなら、
クロージャを再度実行することなく`Some`内の値を返します。

<!--
If `self.value` is `None`, the code calls the closure stored in
`self.calculation`, saves the result in `self.value` for future use, and
returns the value as well.
-->

`self.value`が`None`なら、コードは`self.calculation`に保存されたクロージャを呼び出し、
結果を将来使えるように`self.value`に保存し、その値を返しもします。

<!--
Listing 13-11 shows how we can use this `Cacher` struct in the function
`generate_workout` from Listing 13-6.
-->

リスト 13-11 は、リスト 13-6 の関数`generate_workout`でこの`Cacher`構造体を使用する方法を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
# impl<T> Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     fn new(calculation: T) -> Cacher<T> {
#         Cacher {
#             calculation,
#             value: None,
#         }
#     }
#
#     fn value(&mut self, arg: u32) -> u32 {
#         match self.value {
#             Some(v) => v,
#             None => {
#                 let v = (self.calculation)(arg);
#                 self.value = Some(v);
#                 v
#             },
#         }
#     }
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

<!--
<span class="caption">Listing 13-11: Using `Cacher` in the `generate_workout`
function to abstract away the caching logic</span>
-->

<span class="caption">リスト 13-11: `generate_workout`関数内で`Cacher`を使用し、キャッシュ機構を抽象化する</span>

<!--
Instead of saving the closure in a variable directly, we save a new instance of
`Cacher` that holds the closure. Then, in each place we want the result, we
call the `value` method on the `Cacher` instance. We can call the `value`
method as many times as we want, or not call it at all, and the expensive
calculation will be run a maximum of once.
-->

クロージャを変数に直接保存する代わりに、クロージャを保持する`Cacher`の新規インスタンスを保存しています。
そして、結果が必要な場所それぞれで、その`Cacher`インスタンスに対して`value`メソッドを呼び出しています。
必要なだけ`value`メソッドを呼び出したり、全く呼び出さないこともでき、重い計算は最大でも 1 回しか走りません。

<!--
Try running this program with the `main` function from Listing 13-2. Change the
values in the `simulated_user_specified_value` and `simulated_random_number`
variables to verify that in all the cases in the various `if` and `else`
blocks, `calculating slowly...` appears only once and only when needed. The
`Cacher` takes care of the logic necessary to ensure we aren’t calling the
expensive calculation more than we need to so `generate_workout` can focus on
the business logic.
-->

リスト 13-2 の`main`関数とともにこのプログラムを走らせてみてください。
`simulated_user_specified_value`と`simulated_random_number`変数の値を変えて、
いろんな`if`や`else`ブロックの場合全てで、`calculating slowly`は 1 回だけ、必要な時にのみ出現することを実証してください。
必要以上に重い計算を呼び出さないことを保証するのに必要なロジックの面倒を`Cacher`は見るので、
`generate_workout`はビジネスロジックに集中できるのです。

<!--
### Limitations of the `Cacher` Implementation
-->

### `Cacher`実装の限界

<!--
Caching values is a generally useful behavior that we might want to use in
other parts of our code with different closures. However, there are two
problems with the current implementation of `Cacher` that would make reusing it
in different contexts difficult.
-->

値をキャッシュすることは、コードの他の部分でも異なるクロージャで行いたくなる可能性のある一般的に有用な振る舞いです。
しかし、現在の`Cacher`の実装には、他の文脈で再利用することを困難にしてしまう問題が 2 つあります。

<!--
The first problem is that a `Cacher` instance assumes it will always get the
same value for the parameter `arg` to the `value` method. That is, this test of
`Cacher` will fail:
-->

1 番目の問題は、`Cacher`インスタンスが、常に`value`メソッドの引数`arg`に対して同じ値になると想定していることです。
言い換えると、`Cacher`のこのテストは、失敗するでしょう：

```rust,ignore
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

<!--
This test creates a new `Cacher` instance with a closure that returns the value
passed into it. We call the `value` method on this `Cacher` instance with an
`arg` value of 1 and then an `arg` value of 2, and we expect the call to
`value` with the `arg` value of 2 to return 2.
-->

このテストは、渡された値を返すクロージャを伴う`Cacher`インスタンスを新しく生成しています。
この`Cacher`インスタンスに対して 1 という`arg`値で呼び出し、それから 2 という`arg`値で呼び出し、
2 という`arg`値の`value`呼び出しは 2 を返すべきと期待しています。

<!--
Run this test with the `Cacher` implementation in Listing 13-9 and Listing
13-10, and the test will fail on the `assert_eq!` with this message:
-->

このテストをリスト 13-9 とリスト 13-10 の`Cacher`実装で動かすと、`assert_eq`からこんなメッセージが出て、
テストは失敗します：

```text
thread 'call_with_different_values' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs
```

<!--
The problem is that the first time we called `c.value` with 1, the `Cacher`
instance saved `Some(1)` in `self.value`. Thereafter, no matter what we pass in
to the `value` method, it will always return 1.
-->

問題は、初めて`c.value`を 1 で呼び出した時に、`Cacher`インスタンスは`self.value`に`Some(1)`を保存したことです。
その後`value`メソッドに何を渡しても、常に 1 を返すわけです。

<!--
Try modifying `Cacher` to hold a hash map rather than a single value. The keys
of the hash map will be the `arg` values that are passed in, and the values of
the hash map will be the result of calling the closure on that key. Instead of
looking at whether `self.value` directly has a `Some` or a `None` value, the
`value` function will look up the `arg` in the hash map and return the value if
it’s present. If it’s not present, the `Cacher` will call the closure and save
the resulting value in the hash map associated with its `arg` value.
-->

単独の値ではなく、ハッシュマップを保持するように`Cacher`を改変してみてください。ハッシュマップのキーは、
渡される`arg`値になり、ハッシュマップの値は、そのキーでクロージャを呼び出した結果になるでしょう。
`self.value`が直接`Some`か`None`値であることを調べる代わりに、`value`関数はハッシュマップの`arg`を調べ、
存在するならその値を返します。存在しないなら、`Cacher`はクロージャを呼び出し、
`arg`値に紐づけてハッシュマップに結果の値を保存します。

<!--
The second problem with the current `Cacher` implementation is that it only
accepts closures that take one parameter of type `u32` and return a `u32`. We
might want to cache the results of closures that take a string slice and return
`usize` values, for example. To fix this issue, try introducing more generic
parameters to increase the flexibility of the `Cacher` functionality.
-->

現在の`Cacher`実装の 2 番目の問題は、引数の型に`u32`を一つ取り、`u32`を返すクロージャしか受け付けないことです。
例えば、文字列スライスを取り、`usize`を返すクロージャの結果をキャッシュしたくなるかもしれません。
この問題を修正するには、`Cacher`機能の柔軟性を向上させるためによりジェネリックな引数を導入してみてください。

<!--
### Capturing the Environment with Closures
-->

### クロージャで環境をキャプチャする

<!--
In the workout generator example, we only used closures as inline anonymous
functions. However, closures have an additional capability that functions don’t
have: they can capture their environment and access variables from the scope in
which they’re defined.
-->

トレーニング生成の例においては、クロージャをインラインの匿名関数として使っただけでした。しかし、
クロージャには、関数にはない追加の能力があります：環境をキャプチャし、
自分が定義されたスコープの変数にアクセスできるのです。

<!--
Listing 13-12 has an example of a closure stored in the `equal_to_x` variable
that uses the `x` variable from the closure’s surrounding environment.
-->

リスト 13-12 は、`equal_to_x`変数に保持されたクロージャを囲む環境から`x`変数を使用するクロージャの例です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

<!--
<span class="caption">Listing 13-12: Example of a closure that refers to a
variable in its enclosing scope</span>
-->

<span class="caption">リスト 13-12: 内包するスコープの変数を参照するクロージャの例</span>

<!--
Here, even though `x` is not one of the parameters of `equal_to_x`, the
`equal_to_x` closure is allowed to use the `x` variable that’s defined in the
same scope that `equal_to_x` is defined in.
-->

ここで、`x`は`equal_to_x`の引数でもないのに、
`equal_to_x`が定義されているのと同じスコープで定義されている`x`変数を`equal_to_x`クロージャは使用できています。

<!--
We can’t do the same with functions; if we try with the following example, our
code won’t compile:
-->

同じことを関数では行うことができません; 以下の例で試したら、コードはコンパイルできません：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

<!--
We get an error:
-->

エラーが出ます：

```text
error[E0434]: can't capture dynamic environment in a fn item; use the || { ...
} closure form instead
(エラー: fn 要素では動的な環境をキャプチャできません; 代わりに|| { ... }のクロージャ形式を
使用してください)
 --> src/main.rs
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

<!--
The compiler even reminds us that this only works with closures!
-->

コンパイラは、この形式はクロージャでのみ動作することさえも思い出させてくれています！

<!--
When a closure captures a value from its environment, it uses memory to store
the values for use in the closure body. This use of memory is overhead that we
don’t want to pay in more common cases where we want to execute code that
doesn’t capture its environment. Because functions are never allowed to capture
their environment, defining and using functions will never incur this overhead.
-->

クロージャが環境から値をキャプチャすると、メモリを使用してクロージャ本体で使用できるようにその値を保存します。
このメモリ使用は、環境をキャプチャしないコードを実行するようなもっと一般的な場合には払いたくないオーバーヘッドです。
関数は、絶対に環境をキャプチャすることが許可されていないので、関数を定義して使えば、このオーバーヘッドを招くことは絶対にありません。

<!--
Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: taking
ownership, borrowing mutably, and borrowing immutably. These are encoded in the
three `Fn` traits as follows:
-->

クロージャは、3 つの方法で環境から値をキャプチャでき、この方法は関数が引数を取れる 3 つの方法に直に対応します：
所有権を奪う、可変で借用する、不変で借用するです。これらは、以下のように 3 つの`Fn`トレイトでコード化されています：

<!--
* `FnOnce` consumes the variables it captures from its enclosing scope, known
as the closure’s *environment*. To consume the captured variables, the
closure must take ownership of these variables and move them into the closure
when it is defined. The `Once` part of the name represents the fact that the
closure can’t take ownership of the same variables more than once, so it can
only be called one time.
* `FnMut` can change the environment because it mutably borrows values.
* `Fn` borrows values from the environment immutably.
-->

* `FnOnce`は、クロージャの*環境*として知られている内包されたスコープからキャプチャした変数を消費します。
キャプチャした変数を消費するために、定義された際にクロージャはこれらの変数の所有権を奪い、
自身にムーブするのです。名前のうち、`Once`の部分は、
このクロージャは同じ変数の所有権を 2 回以上奪うことができないという事実を表しているので、1 回しか呼ぶことができないのです。
* `FnMut`は、可変で値を借用するので、環境を変更することができます。
* `Fn`は、環境から値を不変で借用します。

<!--
When you create a closure, Rust infers which trait to use based on how the
closure uses the values from the environment. All closures implement `FnOnce`,
because they can all be called at least once. Closures that don't move the
captured variables also implement `FnMut`, and closures that don't need mutable
access to the captured variables also implement `Fn`. In Listing 13-12, the
`equal_to_x` closure borrows `x` immutably (so `equal_to_x` has the `Fn` trait)
because the body of the closure only needs to read the value in `x`.
-->

クロージャを生成する時、クロージャが環境を使用する方法に基づいて、コンパイラはどのトレイトを使用するか推論します。
少なくとも 1 回は呼び出されるので、全てのクロージャは`FnOnce`を実装しています。キャプチャした変数をムーブしないクロージャは、
`FnMut`も実装し、キャプチャした変数に可変でアクセスする必要のないクロージャは、`Fn`も実装しています。
リスト 13-12 では、`equal_to_x`クロージャは`x`を不変で借用しています (ゆえに`equal_to_x`は`Fn`トレイトです)。
クロージャの本体は、`x`を読む必要しかないからです。

<!--
If you want to force the closure to take ownership of the values it uses in the
environment, we can use the `move` keyword before the parameter list. This
technique is mostly useful when passing a closure to a new thread to move the
data so it’s owned by the new thread.
-->

環境でクロージャが使用している値の所有権を奪うことをクロージャに強制したいなら、引数リストの前に`move`キーワードを使用できます。
このテクニックは、新しいスレッドにデータが所有されるように、クロージャを新しいスレッドに渡して、
データをムーブする際に大概は有用です。

<!--
We’ll have more examples of `move` closures in Chapter 16 when we talk about
concurrency. For now, here’s the code from Listing 13-12 with the `move`
keyword added to the closure definition and using vectors instead of integers,
because integers can be copied rather than moved; note that this code will not
yet compile.
-->

並行性について語る第 16 章で、`move`クロージャの例はもっと多く出てきます。とりあえず、
こちらが`move`キーワードがクロージャ定義に追加され、整数の代わりにベクタを使用するリスト 13-12 からのコードです。
整数はムーブではなく、コピーされてしまいますからね; このコードはまだコンパイルできないことに注意してください。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // ここでは、x を使用できません：{:?}
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

<!--
We receive the following error:
-->

以下のようなエラーを受けます：

```text
error[E0382]: use of moved value: `x`
(エラー: ムーブされた値の使用：`x`)
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
                                  (値はここで(クロージャに)ムーブされた)
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
                                             (ムーブ後、値はここで使用された)
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
  implement the `Copy` trait
  (注釈：`x`が`std::vec::Vec<i32>`という`Copy`トレイトを実装しない型のため、ムーブが起きました)
```

<!--
The `x` value is moved into the closure when the closure is defined, because we
added the `move` keyword. The closure then has ownership of `x`, and `main`
isn’t allowed to use `x` anymore in the `println!` statement. Removing
`println!` will fix this example.
-->

クロージャが定義された際に、クロージャに`x`の値はムーブされています。`move`キーワードを追加したからです。
そして、クロージャは`x`の所有権を持ち、`main`が`println!`で`x`を使うことはもう叶わないのです。
`println!`を取り除けば、この例は修正されます。

<!--
Most of the time when specifying one of the `Fn` trait bounds, you can start
with `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based
on what happens in the closure body.
-->

`Fn`トレイトのどれかを指定するほとんどの場合、`Fn`から始めると、コンパイラがクロージャ本体内で起こっていることにより、
`FnMut`や`FnOnce`が必要な場合、教えてくれるでしょう。

<!--
To illustrate situations where closures that can capture their environment are
useful as function parameters, let’s move on to our next topic: iterators.
-->

環境をキャプチャできるクロージャが関数の引数として有用な場面を説明するために、次のトピックに移りましょう：イテレータです。
