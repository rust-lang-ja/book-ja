<!--
## Using Threads to Run Code Simultaneously
-->

## スレッドを使用してコードを同時に走らせる

<!--
In most current operating systems, an executed program’s code is run in a
*process*, and the operating system will manage multiple processes at once.
Within a program, you can also have independent parts that run simultaneously.
The features that run these independent parts are called *threads*. For
example, a web server could have multiple threads so that it could respond to
more than one request at the same time.
-->

多くの現代のOSでは、実行中のプログラムのコードは*プロセス*で走り、OSは同時に複数のプロセスを管理するでしょう。
プログラム内では、独立した部分を同時に実行することもできます。これらの独立した部分を走らせる機能を*スレッド*と呼びます。
例えばwebサーバは、同時に複数のリクエストに応答できるように、複数のスレッドを使用することができます。

<!--
Splitting the computation in your program into multiple threads to run multiple
tasks at the same time can improve performance, but it also adds complexity.
Because threads can run simultaneously, there’s no inherent guarantee about the
order in which parts of your code on different threads will run. This can lead
to problems, such as:
-->

同時に複数のタスクを実行するために、プログラム内の計算を複数のスレッドに分けることで、パフォーマンスを改善することができますが、
複雑度も増します。スレッドは同時に走らせることができるので、異なるスレッドのコードが走る順番に関して、
本来的に保証はありません。これは例えば以下のような問題を招きます:

<!--
* Race conditions, where threads are accessing data or resources in an
  inconsistent order
* Deadlocks, where two threads are waiting for each other, preventing both
  threads from continuing
* Bugs that happen only in certain situations and are hard to reproduce and fix
  reliably
-->

* スレッドがデータやリソースに矛盾した順番でアクセスする競合状態
* 2つのスレッドがお互いを待ち、両者が継続するのを防ぐデッドロック
* 特定の状況でのみ起き、確実な再現や修正が困難なバグ

<!--
Rust attempts to mitigate the negative effects of using threads, but
programming in a multithreaded context still takes careful thought and requires
a code structure that is different from that in programs running in a single
thread.
-->

Rustは、スレッドを使用する際の悪影響を軽減しようとしていますが、それでも、マルチスレッドの文脈でのプログラミングでは、
注意深い思考と、シングルスレッドで走るプログラムでのそれとは異なるコード構造が必要です。

<!--
Programming languages implement threads in a few different ways, and many
operating systems provide an API the language can call for creating new
threads. The Rust standard library uses a *1:1* model of thread implementation,
whereby a program uses one operating system thread per one language thread.
There are crates that implement other models of threading that make different
tradeoffs to the 1:1 model.
-->

プログラミング言語によってスレッドはいくつかの方法で実装されており、多くのOSは、言語が呼び出すことができる、
新規スレッドを生成するためのAPIを提供しています。
Rust標準ライブラリは*1:1*モデルのスレッド実装を使用しており、1つの言語スレッドに対して1つのOSスレッドを使用します。
1:1モデルとは異なるトレードオフを選択して、他のモデルのスレッドを実装するクレートもあります。

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
新規スレッドで走らせたいコードを含むクロージャ(クロージャについては第13章で語りました)を渡します。
リスト16-1の例は、メインスレッドと新規スレッドからテキストを出力します:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<!--
<span class="caption">Listing 16-1: Creating a new thread to print one thing
while the main thread prints something else</span>
-->

<span class="caption">リスト16-1: メインスレッドが別のものを出力する間に新規スレッドを生成して何かを出力する</span>

<!--
Note that when the main thread of a Rust program completes, all spawned threads
are shut down, whether or not they have finished running. The output from this
program might be a little different every time, but it will look similar to the
following:
-->

Rustプログラムのメインスレッドが完了するときには、立ち上げられたすべてのスレッドは、その実行が完了したかどうかにかかわらず、
停止されることに注意してください。このプログラムからの出力は毎回少々異なる可能性がありますが、だいたい以下のような感じでしょう:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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
スレッドはおそらく切り替わるでしょうが、保証はありません: OSがスレッドのスケジュールを行う方法によります。
この実行では、コード上では立ち上げられたスレッドのprint文が先に現れているのに、メインスレッドが先に出力しています。また、
立ち上げたスレッドには`i`が9になるまで出力するよう指示しているのに、メインスレッドが終了する前の5までしか到達していません。

<!--
If you run this code and only see output from the main thread, or don’t see any
overlap, try increasing the numbers in the ranges to create more opportunities
for the operating system to switch between the threads.
-->

このコードを実行してメインスレッドの出力しか目の当たりにできなかったり、オーバーラップがなければ、
範囲の値を増やしてOSがスレッド切り替えを行う機会を増やしてみてください。

<!--
### Waiting for All Threads to Finish Using `join` Handles
-->

### `join`ハンドルで全スレッドの終了を待つ

<!--
The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time due to the main thread ending, but because there is no guarantee on
the order in which threads run, we also can’t guarantee that the spawned thread
will get to run at all!
-->

リスト16-1のコードはほとんどの場合、メインスレッドが終了することで、立ち上げたスレッドが完了する前に停止されるだけでなく、
スレッドの実行順に保証がないことから、立ち上げたスレッドがそもそも実行されるかどうかも保証できません。

<!--
We can fix the problem of the spawned thread not running or ending prematurely
by saving the return value of `thread::spawn` in a variable. The return type of
`thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we
call the `join` method on it, will wait for its thread to finish. Listing 16-2
shows how to use the `JoinHandle` of the thread we created in Listing 16-1 and
call `join` to make sure the spawned thread finishes before `main` exits:
-->

`thread::spawn`の戻り値を変数に保存することで、立ち上げたスレッドが実行されなかったり、
完了する前に終了してしまったりする問題を修正することができます。`thread::spawn`の戻り値の型は`JoinHandle`です。
`JoinHandle`は、その`join`メソッドを呼び出したときにスレッドの終了を待つ所有された値です。
リスト16-2は、リスト16-1で生成したスレッドの`JoinHandle`を使用し、`join`を呼び出して、
`main`が終了する前に、立ち上げたスレッドが確実に完了する方法を示しています:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<!--
<span class="caption">Listing 16-2: Saving a `JoinHandle` from `thread::spawn`
to guarantee the thread is run to completion</span>
-->

<span class="caption">リスト16-2: `thread::spawn`の`JoinHandle`を保存してスレッドが完了するのを保証する</span>

<!--
Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running Listing 16-2 should
produce output similar to this:
-->

ハンドルに対して`join`を呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックします。
スレッドを*ブロック*するとは、そのスレッドが動いたり、終了したりすることを防ぐことです。
`join`の呼び出しをメインスレッドの`for`ループの後に配置したので、リスト16-2を実行すると、
以下のように出力されるはずです:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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

2つのスレッドが代わる代わる実行されていますが、`handle.join()`呼び出しのためにメインスレッドは待機し、
立ち上げたスレッドが終了するまで終わりません。

<!--
But let’s see what happens when we instead move `handle.join()` before the
`for` loop in `main`, like this:
-->

ですが、代わりに`handle.join()`を`for`ループの前に移動したらどうなるのか確認しましょう。こんな感じに:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

<!--
The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won’t be interleaved anymore, as shown here:
-->

メインスレッドは、立ち上げたスレッドが終了するまで待ち、それから`for`ループを実行するので、
以下のように出力はもう混ざらないでしょう:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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
We'll often use the `move` keyword with closures passed to `thread::spawn`
because the closure will then take ownership of the values it uses from the
environment, thus transferring ownership of those values from one thread to
another. In the [“Capturing References or Moving Ownership”][capture]
section of Chapter 13, we discussed `move` in the context of closures. Now,
we’ll concentrate more on the interaction between `move` and `thread::spawn`.
-->

`thread::spawn`に渡されるクロージャでは、`move`キーワードを多用することになるでしょう。
そうすることで、クロージャは環境から使用する値の所有権を奪い、あるスレッドから別のスレッドに値の所有権を移すからです。
第13章の[「参照をキャプチャするか、所有権を移動するか」][capture]節では、クロージャの文脈で`move`について議論しました。
それでは、`move`と`thread::spawn`の相互作用により集中していきましょう。

<!--
Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. To use data from the main thread in the spawned thread, the
spawned thread’s closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won’t yet work, as you’ll see in a moment.
-->

リスト16-1において、`thread::spawn`に渡したクロージャには引数がなかったことに注目してください:
立ち上げたスレッドのコードでメインスレッドからのデータは何も使用していないのです。
立ち上げたスレッドでメインスレッドのデータを使用するには、立ち上げるスレッドのクロージャは、
必要な値をキャプチャしなければなりません。リスト16-3は、メインスレッドでベクタを生成し、
立ち上げたスレッドで使用する試みを示しています。しかしながら、すぐにわかるように、これはまだ動きません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<!--
<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread in another thread</span>
-->

<span class="caption">リスト16-3: 別のスレッドでメインスレッドが生成したベクタを使用しようとする</span>

<!--
The closure uses `v`, so it will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:
-->

クロージャは`v`を使用しているので、`v`をキャプチャし、クロージャの環境の一部にしています。
`thread::spawn`はこのクロージャを新しいスレッドで走らせるので、
その新しいスレッド内で`v`にアクセスできるはずです。しかし、このコードをコンパイルすると、
以下のようなエラーが出ます:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

<!--
Rust *infers* how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t
tell how long the spawned thread will run, so it doesn’t know if the reference
to `v` will always be valid.
-->

Rustは`v`のキャプチャ方法を*推論*し、`println!`は`v`への参照のみを必要とするので、クロージャは、
`v`を借用しようとします。ですが、問題があります: コンパイラには、立ち上げたスレッドがどのくらいの期間走るのかわからないので、
`v`への参照が常に有効であるか把握できないのです。

<!--
Listing 16-4 provides a scenario that’s more likely to have a reference to `v`
that won’t be valid:
-->

リスト16-4は、`v`への参照がより有効でなさそうな筋書きです:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}

```

<!--
<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>
-->

<span class="caption">リスト16-4: `v`をドロップするメインスレッドから`v`への参照をキャプチャしようとするクロージャを伴うスレッド</span>

<!--
If Rust allowed us to run this code, there’s a possibility the spawned thread
would be immediately put in the background without running at all. The spawned
thread has a reference to `v` inside, but the main thread immediately drops
`v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!
-->

このコードを実行できてしまうなら、立ち上げたスレッドはまったく実行されることなく即座にバックグラウンドに置かれる可能性があります。
立ち上げたスレッドは内部に`v`への参照を保持していますが、メインスレッドは、第15章で議論した`drop`関数を使用して、
即座に`v`をドロップしています。そして、立ち上げたスレッドが実行を開始する時には、`v`はもう有効ではなく、
参照も不正になるのです。あちゃー！

<!--
To fix the compiler error in Listing 16-3, we can use the error message’s
advice:
-->

リスト16-3のコンパイルエラーを修正するには、エラーメッセージのアドバイスを活用できます:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
(ヘルプ: `v`(や他の参照されている変数)の所有権をクロージャに奪わせるには、`move`キーワードを使用してください)
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

<!--
By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend:
-->

クロージャの前に`move`キーワードを付することで、コンパイラに値を借用すべきと推論させるのではなく、
クロージャに使用している値の所有権を強制的に奪わせます。リスト16-5に示したリスト16-3に対する変更は、
コンパイルでき、意図通りに動きます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<!--
<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>
-->

<span class="caption">リスト16-5: `move`キーワードを使用してクロージャに使用している値の所有権を強制的に奪わせる</span>

<!--
We might be tempted to try the same thing to fix the code in Listing 16-4 where
the main thread called `drop` by using a `move` closure. However, this fix will
not work because what Listing 16-4 is trying to do is disallowed for a
different reason. If we added `move` to the closure, we would move `v` into the
closure’s environment, and we could no longer call `drop` on it in the main
thread. We would get this compiler error instead:
-->

リスト16-4の、メインスレッドが`drop`を呼び出すコードを修正するためにも、
`move`クロージャを使用して同じことを試したくなるかもしれません。しかしながら、
リスト16-4が試みていることは別の理由によりできないので、この修正はうまくいきません。
クロージャに`move`を付与したら、`v`をクロージャの環境にムーブするので、
最早メインスレッドで`drop`を呼び出すことは叶わなくなるでしょう。代わりにこのようなコンパイルエラーが出るでしょう:


```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
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

再三Rustの所有権規則が救ってくれました！リスト16-3のコードはエラーになりました。
コンパイラが一時的に保守的になり、スレッドに対して`v`を借用しただけだったからで、
これは、メインスレッドは理論上、立ち上げたスレッドの参照を不正化する可能性があることを意味します。
`v`の所有権を立ち上げたスレッドに移動するとコンパイラに指示することで、
メインスレッドはもう`v`を使用しないとコンパイラに保証しているのです。リスト16-4も同様に変更したら、
メインスレッドで`v`を使用しようとする際に所有権の規則に違反することになります。
`move`キーワードにより、Rustの保守的な借用のデフォルトが上書きされるのです; 
所有権の規則を侵害させてくれないのです。

<!--
With a basic understanding of threads and the thread API, let’s look at what we
can *do* with threads.
-->

スレッドとスレッドAPIの基礎知識を得たので、スレッドで*できる*ことを見ていきましょう。

<!--
[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
-->

[capture]: ch13-01-closures.html#参照をキャプチャするか所有権を移動するか
