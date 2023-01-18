<!--
## Using Threads to Run Code Simultaneously
-->

## スレッドを使用してコードを同時に走らせる

<!--
In most current operating systems, an executed program’s code is run in a
*process*, and the operating system manages multiple processes at once. Within
your program, you can also have independent parts that run simultaneously. The
features that run these independent parts are called *threads*.
-->

多くの現代の OS では、実行中のプログラムのコードは*プロセス*で走り、OS は同時に複数のプロセスを管理します。
自分のプログラム内で、独立した部分を同時に実行できます。これらの独立した部分を走らせる機能を*スレッド*と呼びます。

<!--
Splitting the computation in your program into multiple threads can improve
performance because the program does multiple tasks at the same time, but it
also adds complexity. Because threads can run simultaneously, there’s no
inherent guarantee about the order in which parts of your code on different
threads will run. This can lead to problems, such as:
-->

プログラム内の計算を複数のスレッドに分けると、パフォーマンスが改善します。プログラムが同時に複数の作業をするからですが、
複雑度も増します。スレッドは同時に走らせることができるので、異なるスレッドのコードが走る順番に関して、
本来的に保証はありません。これは例えば以下のような問題を招きます：

<!--
* Race conditions, where threads are accessing data or resources in an
inconsistent order
* Deadlocks, where two threads are waiting for each other to finish using a
resource the other thread has, preventing both threads from continuing
* Bugs that happen only in certain situations and are hard to reproduce and fix
reliably
-->

* スレッドがデータやリソースに矛盾した順番でアクセスする競合状態
* 2 つのスレッドがお互いにもう一方が持っているリソースを使用し終わるのを待ち、両者が継続するのを防ぐデッドロック
* 特定の状況でのみ起き、確実な再現や修正が困難なバグ

<!--
Rust attempts to mitigate the negative effects of using threads, but
programming in a multithreaded context still takes careful thought and requires
a code structure that is different from programs that run in a single
thread.
-->

Rust は、スレッドを使用する際の悪影響を軽減しようとしていますが、それでも、マルチスレッドの文脈でのプログラミングでは、
注意深い思考とシングルスレッドで走るプログラムとは異なるコード構造が必要です。

<!--
Programming languages implement threads in a few different ways. Many operating
systems provide an API for creating new threads. This model where a language
calls the operating system APIs to create threads is sometimes called *1:1*,
meaning one operating system thread per one language thread.
-->

プログラミング言語によってスレッドはいくつかの方法で実装されています。多くの OS で、新規スレッドを生成する API が提供されています。
言語が OS の API を呼び出してスレッドを生成するこのモデルを時に*1:1*と呼び、1 つの OS スレッドに対して 1 つの言語スレッドを意味します。

<!--
Many programming languages provide their own special implementation of threads.
Programming language-provided threads are known as *green* threads, and
languages that use these green threads will execute them in the context of a
different number of operating system threads. For this reason, the
green-threaded model is called the *M:N* model: there are `M` green threads per
`N` operating system threads, where `M` and `N` are not necessarily the same
number.
-->

多くのプログラミング言語がスレッドの独自の特別な実装を提供しています。プログラミング言語が提供するスレッドは、
*グリーン*スレッドとして知られ、このグリーンスレッドを使用する言語は、それを異なる数の OS スレッドの文脈で実行します。
このため、グリーンスレッドのモデルは*M:N*モデルと呼ばれます：`M`個のグリーンスレッドに対して、
`N`個の OS スレッドがあり、`M`と`N`は必ずしも同じ数字ではありません。

<!--
Each model has its own advantages and trade-offs, and the trade-off most
important to Rust is runtime support. *Runtime* is a confusing term and can
have different meanings in different contexts.
-->

各モデルには、それだけの利点と代償があり、Rust にとって最も重要な代償は、ランタイムのサポートです。
*ランタイム*は、混乱しやすい用語で文脈によって意味も変わります。

<!--
In this context, by *runtime* we mean code that is included by the language in
every binary. This code can be large or small depending on the language, but
every non-assembly language will have some amount of runtime code. For that
reason, colloquially when people say a language has “no runtime,” they often
mean “small runtime.” Smaller runtimes have fewer features but have the
advantage of resulting in smaller binaries, which make it easier to combine the
language with other languages in more contexts. Although many languages are
okay with increasing the runtime size in exchange for more features, Rust needs
to have nearly no runtime and cannot compromise on being able to call into C to
maintain performance.
-->

この文脈での*ランタイム*とは、言語によって全てのバイナリに含まれるコードのことを意味します。
言語によってこのコードの大小は決まりますが、非アセンブリ言語は全てある量の実行時コードを含みます。
そのため、口語的に誰かが「ノーランタイム」と言ったら、「小さいランタイム」のことを意味することがしばしばあります。
ランタイムが小さいと機能も少ないですが、バイナリのサイズも小さくなるという利点があり、
その言語を他の言語とより多くの文脈で組み合わせることが容易になります。多くの言語では、
より多くの機能と引き換えにランタイムのサイズが膨れ上がるのは、受け入れられることですが、
Rust にはほとんどゼロのランタイムが必要でパフォーマンスを維持するために C コードを呼び出せることを妥協できないのです。

<!--
The green-threading M:N model requires a larger language runtime to manage
threads. As such, the Rust standard library only provides an implementation of
1:1 threading. Because Rust is such a low-level language, there are crates that
implement M:N threading if you would rather trade overhead for aspects such as
more control over which threads run when and lower costs of context switching,
for example.
-->

M:N のグリーンスレッドモデルは、スレッドを管理するのにより大きな言語ランタイムが必要です。よって、
Rust の標準ライブラリは、1:1 スレッドの実装のみを提供しています。Rust はそのような低級言語なので、
例えば、むしろどのスレッドがいつ走るかのより詳細な制御や、より低コストの文脈切り替えなどの一面をオーバーヘッドと引き換えるなら、
M:N スレッドの実装をしたクレートもあります。

<!--
Now that we’ve defined threads in Rust, let’s explore how to use the
thread-related API provided by the standard library.
-->

今や Rust におけるスレッドを定義したので、標準ライブラリで提供されているスレッド関連の API の使用法を探究しましょう。

<!--
### Creating a New Thread with `spawn`
-->

### `spawn`で新規スレッドを生成する

<!--
To create a new thread, we call the `thread::spawn` function and pass it a
closure (we talked about closures in Chapter 13) containing the code we want to
run in the new thread. The example in Listing 16-1 prints some text from a main
thread and other text from a new thread:
-->

新規スレッドを生成するには、`thread::spawn`関数を呼び出し、
新規スレッドで走らせたいコードを含むクロージャ (クロージャについては第 13 章で語りました) を渡します。
リスト 16-1 の例は、メインスレッドと新規スレッドからテキストを出力します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

<!--
<span class="caption">Listing 16-1: Creating a new thread to print one thing
while the main thread prints something else</span>
-->

<span class="caption">リスト 16-1: メインスレッドが別のものを出力する間に新規スレッドを生成して何かを出力する</span>

<!--
Note that with this function, the new thread will be stopped when the main
thread ends, whether or not it has finished running. The output from this
program might be a little different every time, but it will look similar to the
following:
-->

この関数では、新しいスレッドは、実行が終わったかどうかにかかわらず、メインスレッドが終了したら停止することに注意してください。
このプログラムからの出力は毎回少々異なる可能性がありますが、だいたい以下のような感じでしょう：

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

<!--
The calls to `thread::sleep` force a thread to stop its execution for a short
duration, allowing a different thread to run. The threads will probably take
turns, but that isn’t guaranteed: it depends on how your operating system
schedules the threads. In this run, the main thread printed first, even though
the print statement from the spawned thread appears first in the code. And even
though we told the spawned thread to print until `i` is 9, it only got to 5
before the main thread shut down.
-->

`thread::sleep`を呼び出すと、少々の間、スレッドの実行を止め、違うスレッドを走らせることができます。
スレッドはおそらく切り替わるでしょうが、保証はありません：OS がスレッドのスケジュールを行う方法によります。
この実行では、コード上では立ち上げられたスレッドの print 文が先に現れているのに、メインスレッドが先に出力しています。また、
立ち上げたスレッドには`i`が 9 になるまで出力するよう指示しているのに、メインスレッドが終了する前の 5 までしか到達していません。

<!--
If you run this code and only see output from the main thread, or don’t see any
overlap, try increasing the numbers in the ranges to create more opportunities
for the operating system to switch between the threads.
-->

このコードを実行してメインスレッドの出力しか目の当たりにできなかったり、オーバーラップがなければ、
範囲の値を増やして OS がスレッド切り替えを行う機会を増やしてみてください。

<!--
### Waiting for All Threads to Finish Using `join` Handles
-->

### `join`ハンドルで全スレッドの終了を待つ

<!--
The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time due to the main thread ending, but also can’t guarantee that the
spawned thread will get to run at all. The reason is that there is no guarantee
on the order in which threads run!
-->

リスト 16-1 のコードは、メインスレッドが終了するためにほとんどの場合、立ち上げたスレッドがすべて実行されないだけでなく、
立ち上げたスレッドが実行されるかどうかも保証できません。原因は、スレッドの実行順に保証がないからです。

<!--
We can fix the problem of the spawned thread not getting to run, or not getting
to run completely, by saving the return value of `thread::spawn` in a variable.
The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to
finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created
in Listing 16-1 and call `join` to make sure the spawned thread finishes before
`main` exits:
-->

`thread::spawn`の戻り値を変数に保存することで、立ち上げたスレッドが実行されなかったり、
完全には実行されなかったりする問題を修正することができます。`thread::spawn`の戻り値の型は`JoinHandle`です。
`JoinHandle`は、その`join`メソッドを呼び出したときにスレッドの終了を待つ所有された値です。
リスト 16-2 は、リスト 16-1 で生成したスレッドの`JoinHandle`を使用し、`join`を呼び出して、
`main`が終了する前に、立ち上げたスレッドが確実に完了する方法を示しています：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

<!--
<span class="caption">Listing 16-2: Saving a `JoinHandle` from `thread::spawn`
to guarantee the thread is run to completion</span>
-->

<span class="caption">リスト 16-2: `thread::spawn`の`JoinHandle`を保存してスレッドが完了するのを保証する</span>

<!--
Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running Listing 16-2 should
produce output similar to this:
-->

ハンドルに対して`join`を呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックします。
スレッドを*ブロック*するとは、そのスレッドが動いたり、終了したりすることを防ぐことです。
`join`の呼び出しをメインスレッドの`for`ループの後に配置したので、リスト 16-2 を実行すると、
以下のように出力されるはずです：

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

<!--
The two threads continue alternating, but the main thread waits because of the
call to `handle.join()` and does not end until the spawned thread is finished.
-->

2 つのスレッドが代わる代わる実行されていますが、`handle.join()`呼び出しのためにメインスレッドは待機し、
立ち上げたスレッドが終了するまで終わりません。

<!--
But let’s see what happens when we instead move `handle.join()` before the
`for` loop in `main`, like this:
-->

ですが、代わりに`handle.join()`を`for`ループの前に移動したらどうなるのか確認しましょう。こんな感じに：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

<!--
The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won’t be interleaved anymore, as shown here:
-->

メインスレッドは、立ち上げたスレッドが終了するまで待ち、それから`for`ループを実行するので、
以下のように出力はもう混ざらないでしょう：

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

<!--
Small details, such as where `join` is called, can affect whether or not your
threads run at the same time.
-->

どこで`join`を呼ぶかといったほんの些細なことが、スレッドが同時に走るかどうかに影響することもあります。

<!--
### Using `move` Closures with Threads
-->

### スレッドで`move`クロージャを使用する

<!--
The `move` closure is often used alongside `thread::spawn` because it allows
you to use data from one thread in another thread.
-->

`move`クロージャは、`thread::spawn`とともによく使用されます。
あるスレッドのデータを別のスレッドで使用できるようになるからです。

<!--
In Chapter 13, we mentioned we can use the `move` keywrod before the parameter
list of a closure to force the closure to take ownership of the values it uses
in the environment. This technique is especially useful when creating new
threads in order to transfer ownership of values from one thread to another.
-->

第 13 章で、クロージャの引数リストの前に`move`キーワードを使用して、
クロージャに環境で使用している値の所有権を強制的に奪わせることができると述べました。
このテクニックは、あるスレッドから別のスレッドに値の所有権を移すために新しいスレッドを生成する際に特に有用です。

<!--
Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. To use data from the main thread in the spawned thread, the
spawned thread’s closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won’t yet work, as you’ll see in a moment:
-->

リスト 16-1 において、`thread::spawn`に渡したクロージャには引数がなかったことに注目してください：
立ち上げたスレッドのコードでメインスレッドからのデータは何も使用していないのです。
立ち上げたスレッドでメインスレッドのデータを使用するには、立ち上げるスレッドのクロージャは、
必要な値をキャプチャしなければなりません。リスト 16-3 は、メインスレッドでベクタを生成し、
立ち上げたスレッドで使用する試みを示しています。しかしながら、すぐにわかるように、これはまだ動きません：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // こちらがベクタ：{:?}
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<!--
<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread in another thread</span>
-->

<span class="caption">リスト 16-3: 別のスレッドでメインスレッドが生成したベクタを使用しようとする</span>

<!--
The closure uses `v`, so it will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:
-->

クロージャは`v`を使用しているので、`v`をキャプチャし、クロージャの環境の一部にしています。
`thread::spawn`はこのクロージャを新しいスレッドで走らせるので、
その新しいスレッド内で`v`にアクセスできるはずです。しかし、このコードをコンパイルすると、
以下のようなエラーが出ます：

```text
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
(エラー: クロージャは現在の関数よりも長生きするかもしれませんが、現在の関数が所有している
`v`を借用しています)
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
(助言：`v`(や他の参照されている変数) の所有権をクロージャに奪わせるには、`move`キーワードを使用してください)
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

<!--
Rust *infers* how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t
tell how long the spawned thread will run, so it doesn’t know if the reference
to `v` will always be valid.
-->

Rust は`v`のキャプチャ方法を*推論*し、`println!`は`v`への参照のみを必要とするので、クロージャは、
`v`を借用しようとします。ですが、問題があります：コンパイラには、立ち上げたスレッドがどのくらいの期間走るのかわからないので、
`v`への参照が常に有効であるか把握できないのです。

<!--
Listing 16-4 provides a scenario that’s more likely to have a reference to `v`
that won’t be valid:
-->

リスト 16-4 は、`v`への参照がより有効でなさそうな筋書きです：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    // いや〜！
    drop(v); // oh no!

    handle.join().unwrap();
}
```

<!--
<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>
-->

<span class="caption">リスト 16-4: `v`をドロップするメインスレッドから`v`への参照をキャプチャしようとするクロージャを伴うスレッド</span>

<!--
If we were allowed to run this code, there’s a possibility the spawned thread
would be immediately put in the background without running at all. The spawned
thread has a reference to `v` inside, but the main thread immediately drops
`v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!
-->

このコードを実行できてしまうなら、立ち上げたスレッドはまったく実行されることなく即座にバックグラウンドに置かれる可能性があります。
立ち上げたスレッドは内部に`v`への参照を保持していますが、メインスレッドは、第 15 章で議論した`drop`関数を使用して、
即座に`v`をドロップしています。そして、立ち上げたスレッドが実行を開始する時には、`v`はもう有効ではなく、
参照も不正になるのです。あちゃー！

<!--
To fix the compiler error in Listing 16-3, we can use the error message’s
advice:
-->

リスト 16-3 のコンパイルエラーを修正するには、エラーメッセージのアドバイスを活用できます：

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

<!--
By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend:
-->

クロージャの前に`move`キーワードを付することで、コンパイラに値を借用すべきと推論させるのではなく、
クロージャに使用している値の所有権を強制的に奪わせます。リスト 16-5 に示したリスト 16-3 に対する変更は、
コンパイルでき、意図通りに動きます：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<!--
<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>
-->

<span class="caption">リスト 16-5: `move`キーワードを使用してクロージャに使用している値の所有権を強制的に奪わせる</span>

<!--
What would happen to the code in Listing 16-4 where the main thread called
`drop` if we use a `move` closure? Would `move` fix that case? Unfortunately,
no; we would get a different error because what Listing 16-4 is trying to do
isn’t allowed for a different reason. If we added `move` to the closure, we
would move `v` into the closure’s environment, and we could no longer call
`drop` on it in the main thread. We would get this compiler error instead:
-->

`move`クロージャを使用していたら、メインスレッドが`drop`を呼び出すリスト 16-4 のコードはどうなるのでしょうか？
`move`で解決するのでしょうか？残念ながら、違います; リスト 16-4 が試みていることは別の理由によりできないので、
違うエラーが出ます。クロージャに`move`を付与したら、`v`をクロージャの環境にムーブするので、
最早メインスレッドで`drop`を呼び出すことは叶わなくなるでしょう。代わりにこのようなコンパイルエラーが出るでしょう：

```text
error[E0382]: use of moved value: `v`
(エラー: ムーブされた値の使用：`v`)
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
   (注釈：`v`の型が`std::vec::Vec<i32>`のためムーブが起きました。この型は、`Copy`トレイトを実装していません)
```

<!--
Rust’s ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread’s reference. By telling Rust to move ownership of `v` to the spawned
thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If
we change Listing 16-4 in the same way, we’re then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust’s conservative default of borrowing; it doesn’t let us violate the
ownership rules.
-->

再三 Rust の所有権規則が救ってくれました！リスト 16-3 のコードはエラーになりました。
コンパイラが一時的に保守的になり、スレッドに対して`v`を借用しただけだったからで、
これは、メインスレッドは理論上、立ち上げたスレッドの参照を不正化する可能性があることを意味します。
`v`の所有権を立ち上げたスレッドに移動するとコンパイラに指示することで、
メインスレッドはもう`v`を使用しないとコンパイラに保証しているのです。リスト 16-4 も同様に変更したら、
メインスレッドで`v`を使用しようとする際に所有権の規則に違反することになります。
`move`キーワードにより、Rust の保守的な借用のデフォルトが上書きされるのです; 
所有権の規則を侵害させてくれないのです。

<!--
With a basic understanding of threads and the thread API, let’s look at what we
can *do* with threads.
-->

スレッドとスレッド API の基礎知識を得たので、スレッドで*できる*ことを見ていきましょう。
