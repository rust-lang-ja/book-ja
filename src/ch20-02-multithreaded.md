<!--
## Turning Our Single-Threaded Server into a Multithreaded Server
-->

## シングルスレッドサーバをマルチスレッド化する

<!--
Right now, the server will process each request in turn, meaning it won’t
process a second connection until the first is finished processing. If the
server received more and more requests, this serial execution would be less and
less optimal. If the server receives a request that takes a long time to
process, subsequent requests will have to wait until the long request is
finished, even if the new requests can be processed quickly. We’ll need to fix
this, but first, we’ll look at the problem in action.
-->

現状、サーバはリクエストを順番に処理します。つまり、最初の接続が処理し終わるまで、2番目の接続は処理しないということです。
サーバが受け付けるリクエストの量が増えるほど、この連続的な実行は、最適ではなくなるでしょう。
サーバが処理するのに長い時間がかかるリクエストを受け付けたら、新しいリクエストは迅速に処理できても、
続くリクエストは長いリクエストが完了するまで待たなければならなくなるでしょう。これを修正する必要がありますが、
まずは、実際に問題が起こっているところを見ます。

<!--
### Simulating a Slow Request in the Current Server Implementation
-->

### 現在のサーバの実装で遅いリクエストをシミュレーションする

<!--
We’ll look at how a slow-processing request can affect other requests made to
our current server implementation. Listing 20-10 implements handling a request
to */sleep* with a simulated slow response that will cause the server to sleep
for 5 seconds before responding.
-->

処理が遅いリクエストが現在のサーバ実装に対して行われる他のリクエストにどう影響するかに目を向けます。
リスト20-10は、応答する前に5秒サーバをスリープさせる遅いレスポンスをシミュレーションした */sleep*へのリクエストを扱う実装です。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-10: Simulating a slow request by sleeping for
5 seconds</span>
-->

<span class="caption">リスト20-10: 5秒間スリープすることで遅いリクエストをシミュレーションする</span>

<!--
We switched from `if` to `match` now that we have three cases. We need to
explicitly match on a slice of `request_line` to pattern match against the
string literal values; `match` doesn’t do automatic referencing and
dereferencing like the equality method does.
-->

3通りの場合分けを持つようになったので、`if`から`match`に切り替えました。
文字列リテラル値を使ってパターンマッチするには、`request_line`のスライスにマッチすることを明示する必要があります;
`match`は、等値性メソッドが行うような自動的な参照および参照外しを行いません。

<!--
The first arm is the same as the `if` block from Listing 20-9. The second arm
matches a request to */sleep*. When that request is received, the server will
sleep for 5 seconds before rendering the successful HTML page. The third arm is
the same as the `else` block from Listing 20-9.

-->

最初のアームは、リスト20-9の`if`ブロックと同じです。
2番目のアームは、*/sleep* へのリクエストにマッチします。
そのリクエストが受け付けられると、サーバは成功のHTMLページを描画する前に5秒間スリープします。
3番目のアームは、リスト20-9の`else`ブロックと同じです。

<!--
You can see how primitive our server is: real libraries would handle the
recognition of multiple requests in a much less verbose way!
-->

我々のサーバがどれだけ基礎的か見て取れます: 本物のライブラリは、もっと冗長でない方法で複数のリクエストの認識を扱うでしょう！

<!--
Start the server using `cargo run`. Then open two browser windows: one for
*http://127.0.0.1:7878/* and the other for *http://127.0.0.1:7878/sleep*. If
you enter the */* URI a few times, as before, you’ll see it respond quickly.
But if you enter */sleep* and then load */*, you’ll see that */* waits until
`sleep` has slept for its full 5 seconds before loading.
-->

`cargo run`でサーバを開始してください。それから2つブラウザのウインドウを開いてください: 1つは、
*http://127.0.0.1:7878/* 用、そしてもう1つは*http://127.0.0.1:7878/sleep* 用です。
以前のように */* URIを数回入力したら、素早く応答するでしょう。しかし、*/sleep*を入力し、それから */* をロードしたら、
`sleep`がロードする前にきっかり5秒スリープし終わるまで、*/* は待機するのを目撃するでしょう。

<!--
There are multiple techniques we could use to avoid requests backing up behind
a slow request; the one we’ll implement is a thread pool.
-->

遅いリクエストの後ろにリクエストが積み重なってしまうのを回避するための技術は、複数あります;
これから実装するのは、スレッドプールです。

<!--
### Improving Throughput with a Thread Pool
-->

### スレッドプールでスループットを向上させる

<!--
A *thread pool* is a group of spawned threads that are waiting and ready to
handle a task. When the program receives a new task, it assigns one of the
threads in the pool to the task, and that thread will process the task. The
remaining threads in the pool are available to handle any other tasks that come
in while the first thread is processing. When the first thread is done
processing its task, it’s returned to the pool of idle threads, ready to handle
a new task. A thread pool allows you to process connections concurrently,
increasing the throughput of your server.
-->

*スレッドプール*は、待機し、タスクを処理する準備のできた一塊りの大量に生成されたスレッドです。
プログラムが新しいタスクを受け取ったら、プールのスレッドのどれかをタスクにあてがい、
そのスレッドがそのタスクを処理します。
プールの残りのスレッドは、最初のスレッドが処理中にやってくる他のあらゆるタスクを扱うために利用可能です。
最初のスレッドがタスクの処理を完了したら、アイドル状態のスレッドプールに戻り、新しいタスクを処理する準備ができます。
スレッドプールにより、並行で接続を処理でき、サーバのスループットを向上させます。

<!--
We’ll limit the number of threads in the pool to a small number to protect us
from Denial of Service (DoS) attacks; if we had our program create a new thread
for each request as it came in, someone making 10 million requests to our
server could create havoc by using up all our server’s resources and grinding
the processing of requests to a halt.
-->

プール内のスレッド数は、小さい数字に制限し、DoS(Denial of Service; サービスの拒否)攻撃から保護します; リクエストが来た度に新しいスレッドをプログラムに生成させたら、
1000万リクエストをサーバに行う誰かが、サーバのリソースを使い尽くし、リクエストの処理を停止に追い込むことで、
大混乱を招くことができてしまうでしょう。

<!--
Rather than spawning unlimited threads, then, we’ll have a fixed number of
threads waiting in the pool. Requests that come in are sent to the pool for
processing. The pool will maintain a queue of incoming requests. Each of the
threads in the pool will pop off a request from this queue, handle the request,
and then ask the queue for another request. With this design, we can process up
to `N` requests concurrently, where `N` is the number of threads. If each
thread is responding to a long-running request, subsequent requests can still
back up in the queue, but we’ve increased the number of long-running requests
we can handle before reaching that point.
-->

無制限にスレッドを大量生産するのではなく、プールに固定された数のスレッドを待機させます。
やってきたリクエストは、処理するためにプールに送られます。プールは、やって来るリクエストのキューを管理します。
プールの各スレッドがこのキューからリクエストを取り出し、リクエストを処理し、そして、別のリクエストをキューに要求します。
この設計により、`N`個までのリクエストを並行して処理でき、ここで`N`はスレッド数です。各スレッドが実行に時間のかかるリクエストに応答していたら、
続くリクエストはそれでも、キュー内で待機させられてしまうこともありますが、その地点に到達する前に扱える時間のかかるリクエスト数を増加させました。

<!--
This technique is just one of many ways to improve the throughput of a web
server. Other options you might explore are the *fork/join model*, the
*single-threaded async I/O model*, or the *multi-threaded async I/O model*. If
you’re interested in this topic, you can read more about other solutions and
try to implement them; with a low-level language like Rust, all of these
options are possible.
-->

このテクニックは、Webサーバのスループットを向上させる多くの方法の1つに過ぎません。探究する可能性のある他の選択肢は、
*fork/joinモデル*、*シングルスレッドの非同期I/Oモデル*、または*マルチスレッドの非同期I/Oモデル*です。
この話題に興味があれば、他の解決策についてもっと読み、実装してみるのもよいでしょう;
Rustのような低レベル言語であれば、これらの選択肢全部が可能なのです。

<!--
Before we begin implementing a thread pool, let’s talk about what using the
pool should look like. When you’re trying to design code, writing the client
interface first can help guide your design. Write the API of the code so it’s
structured in the way you want to call it; then implement the functionality
within that structure rather than implementing the functionality and then
designing the public API.
-->

スレッドプールを実装し始める前に、プールを使うのはどんな感じになるはずなのかについて語りましょう。コードの設計を試みる際、
クライアントのインターフェイスをまず書くことは、設計を導く手助けになることがあります。呼び出したいように構成されるよう、
コードのAPIを記述してください; そして、機能を実装してから公開APIの設計をするのではなく、その構造内で機能を実装してください。

<!--
Similar to how we used test-driven development in the project in Chapter 12,
we’ll use compiler-driven development here. We’ll write the code that calls the
functions we want, and then we’ll look at errors from the compiler to determine
what we should change next to get the code to work. Before we do that, however,
we’ll explore the technique we’re not going to use as a starting point.
-->

第12章のプロジェクトでTDDを使用したように、ここではCompiler Driven Development(コンパイラ駆動開発)を使用します。
欲しい関数を呼び出すコードを書き、それからコンパイラの出すエラーを見てコードが動くように次に何を変更すべきかを決定します。
ですがその前に開始点として、使用しない技法について探索してみましょう。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>
-->

<!--
#### Spawning a Thread for Each Request
-->

#### 各リクエストに対してスレッドを立ち上げる

<!--
First, let’s explore how our code might look if it did create a new thread for
every connection. As mentioned earlier, this isn’t our final plan due to the
problems with potentially spawning an unlimited number of threads, but it is a
starting point to get a working multithreaded server first. Then we’ll add the
thread pool as an improvement, and contrasting the two solutions will be
easier. Listing 20-11 shows the changes to make to `main` to spawn a new thread
to handle each stream within the `for` loop.
-->

まず、全接続に対して新しいスレッドを確かに生成した場合にコードがどんな見た目になるかを探究しましょう。
先ほど述べたように、無制限にスレッドを大量生産する可能性があるという問題のため、これは最終的な計画ではありませんが、
まず機能するマルチスレッドサーバを得るための開始点です。
その後、改善としてスレッドプールを追加します。そうすることで、2つの解決策の対比がより簡単になるでしょう。
リスト20-11は、新しいスレッドを立ち上げて`for`ループ内で各ストリームを扱うために`main`に行う変更を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-11: Spawning a new thread for each
stream</span>
-->

<span class="caption">リスト20-11: 各ストリームに対して新しいスレッドを立ち上げる</span>

<!--
As you learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in the new thread. If you run this code and load
*/sleep* in your browser, then */* in two more browser tabs, you’ll indeed see
that the requests to */* don’t have to wait for */sleep* to finish. However, as
we mentioned, this will eventually overwhelm the system because you’d be making
new threads without any limit.
-->

第16章で学んだように、`thread::spawn`は新しいスレッドを生成し、それからクロージャ内のコードを新しいスレッドで実行します。
このコードを実行してブラウザで */sleep*をロードし、それからもう2つのブラウザのタブで */* をロードしたら、
確かに */* へのリクエストは、*/sleep*が完了するのを待機しなくても済むことがわかるでしょう。
ですが、前述したように、無制限にスレッドを生成することになるので、これは最終的にシステムを参らせてしまうでしょう。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>
-->

<!--
#### Creating a Finite Number of Threads
-->

#### 有限個のスレッドを作成する

<!--
We want our thread pool to work in a similar, familiar way so switching from
threads to a thread pool doesn’t require large changes to the code that uses
our API. Listing 20-12 shows the hypothetical interface for a `ThreadPool`
struct we want to use instead of `thread::spawn`.
-->

スレッドからスレッドプールへの変更にAPIを使用するコードへの大きな変更が必要ないように、
スレッドプールには似た、馴染み深い方法で動作してほしいです。リスト20-12は、
`thread::spawn`の代わりに使用したい`ThreadPool`構造体の架空のインターフェイスを表示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-12: Our ideal `ThreadPool` interface</span>
-->

<span class="caption">リスト20-12: `ThreadPool`の理想的なインターフェイス</span>

<!--
We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` has a
similar interface as `thread::spawn` in that it takes a closure the pool should
run for each stream. We need to implement `pool.execute` so it takes the
closure and gives it to a thread in the pool to run. This code won’t yet
compile, but we’ll try so the compiler can guide us in how to fix it.
-->

`ThreadPool::new`を使用して設定可能なスレッド数で新しいスレッドプールを作成し、今回の場合は4です。
それから`for`ループ内で、`pool.execute`は、プールが各ストリームに対して実行すべきクロージャを受け取るという点で、
`thread::spawn`と似たインターフェイスです。`pool.execute`を実装する必要があるので、
これはクロージャを取り、実行するためにプール内のスレッドに与えます。このコードはまだコンパイルできませんが、
コンパイラがどう修正したらいいかガイドできるように試してみます。

<!-- Old headings. Do not remove or links may break. -->
<!--
<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>
-->

<!--
#### Building `ThreadPool` Using Compiler Driven Development
-->

#### コンパイラ駆動開発で`ThreadPool`を構築する

<!--
Make the changes in Listing 20-12 to *src/main.rs*, and then let’s use the
compiler errors from `cargo check` to drive our development. Here is the first
error we get:
-->

リスト20-12の変更を*src/main.rs*に行い、それから開発を駆動するために`cargo check`からのコンパイラエラーを活用しましょう。
こちらが得られる最初のエラーです:

```console
{{#include ../listings/ch20-web-server/listing-20-12/output.txt}}
```

<!--
Great! This error tells us we need a `ThreadPool` type or module, so we’ll
build one now. Our `ThreadPool` implementation will be independent of the kind
of work our web server is doing. So, let’s switch the `hello` crate from a
binary crate to a library crate to hold our `ThreadPool` implementation. After
we change to a library crate, we could also use the separate thread pool
library for any work we want to do using a thread pool, not just for serving
web requests.
-->

よろしい!このエラーは`ThreadPool`型かモジュールが必要なことを教えてくれているので、今構築します。
`ThreadPool`の実装は、Webサーバが行う仕事の種類とは独立しています。従って、`hello`クレートをバイナリクレートからライブラリクレートに切り替え、
`ThreadPool`の実装を保持させましょう。ライブラリクレートに変更後、
個別のスレッドプールライブラリをWebリクエストを提供するためだけではなく、スレッドプールでしたいあらゆる作業にも使用できます。

<!--
Create a *src/lib.rs* that contains the following, which is the simplest
definition of a `ThreadPool` struct that we can have for now:
-->

以下を含む*src/lib.rs*を生成してください。これは、現状存在できる最も単純な`ThreadPool`の定義です:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

<!--
Then edit *main.rs* file to bring `ThreadPool` into scope from the library
crate by adding the following code to the top of *src/main.rs*:
-->

それから*main.rs*ファイルを編集し、以下のコードを*src/main.rs*の先頭に追記して、
ライブラリクレートから`ThreadPool`をスコープに導入してください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

<!--
This code still won’t work, but let’s check it again to get the next error that
we need to address:
-->

このコードはまだ動きませんが、再度それを確認して扱う必要のある次のエラーを手に入れましょう:

```console
{{#include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

<!--
This error indicates that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument and should return a `ThreadPool` instance.
Let’s implement the simplest `new` function that will have those
characteristics:
-->

このエラーは、次に、`ThreadPool`に対して`new`という関連関数を作成する必要があることを示唆しています。
また、`new`には`4`を引数として受け入れる引数1つがあり、`ThreadPool`インスタンスを返すべきということも知っています。
それらの特徴を持つ最も単純な`new`関数を実装しましょう:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

<!--
We chose `usize` as the type of the `size` parameter, because we know that a
negative number of threads doesn’t make any sense. We also know we’ll use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the [“Integer Types”][integer-types]
section of Chapter 3.
-->

`size`引数の型として、`usize`を選択しました。何故なら、マイナスのスレッド数は、何も筋が通らないことを知っているからです。
また、この4をスレッドのコレクションの要素数として使用し、第3章の[「整数型」][integer-types]節で議論したように、これは`usize`のあるべき姿であることも知っています。

<!--
Let’s check the code again:
-->

コードを再度確認しましょう:

```console
{{#include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

<!--
Now the error occurs because we don’t have an `execute` method on `ThreadPool`.
Recall from the [“Creating a Finite Number of
Threads”](#creating-a-finite-number-of-threads) section that we
decided our thread pool should have an interface similar to `thread::spawn`. In
addition, we’ll implement the `execute` function so it takes the closure it’s
given and gives it to an idle thread in the pool to run.
-->

今度は、`ThreadPool`に`execute`メソッドがないためにエラーが発生しました。
[「有限個のスレッドを作成する」](#有限個のスレッドを作成する)節で我々のスレッドプールは、
`thread::spawn`と似たインターフェイスにするべきと決定したことを思い出してください。
さらに、`execute`関数を実装するので、与えられたクロージャを取り、実行するようにプールの待機中のスレッドに渡します。

<!--
We’ll define the `execute` method on `ThreadPool` to take a closure as a
parameter. Recall from the [“Moving Captured Values Out of the Closure and the
`Fn` Traits”][fn-traits] section in Chapter 13 that we can take
closures as parameters with three different traits: `Fn`, `FnMut`, and
`FnOnce`. We need to decide which kind of closure to use here. We know we’ll
end up doing something similar to the standard library `thread::spawn`
implementation, so we can look at what bounds the signature of `thread::spawn`
has on its parameter. The documentation shows us the following:
-->

`ThreadPool`に`execute`メソッドをクロージャを引数として受け取るように定義します。
第13章の[「キャプチャされた値のクロージャからのムーブと、`Fn`系トレイト」][fn-traits]節から、
3つの異なるトレイトでクロージャを引数として取ることができることを思い出してください: `Fn`、`FnMut`、`FnOnce`です。
ここでは、どの種類のクロージャを使用するか決定する必要があります。最終的には、
標準ライブラリの`thread::spawn`実装に似たことをすることがわかっているので、
`thread::spawn`のシグニチャで引数にどんな境界があるか見ることができます。ドキュメンテーションは、以下のものを示しています:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `F` type parameter is the one we’re concerned with here; the `T` type
parameter is related to the return value, and we’re not concerned with that. We
can see that `spawn` uses `FnOnce` as the trait bound on `F`. This is probably
what we want as well, because we’ll eventually pass the argument we get in
`execute` to `spawn`. We can be further confident that `FnOnce` is the trait we
want to use because the thread for running a request will only execute that
request’s closure one time, which matches the `Once` in `FnOnce`.
-->

`F`型引数がここで関心のあるものです; `T`型引数は戻り値と関係があり、関心はありません。`spawn`は、
`F`のトレイト境界として`FnOnce`を使用していることが確認できます。これはおそらく、我々が欲しているものでもあるでしょう。
というのも、最終的には`execute`で得た引数を`spawn`に渡すからです。さらに`FnOnce`は使用したいトレイトであると自信を持つことができます。
リクエストを実行するスレッドは、そのリクエストのクロージャを1回だけ実行し、これは`FnOnce`の`Once`に合致するからです。

<!--
The `F` type parameter also has the trait bound `Send` and the lifetime bound
`'static`, which are useful in our situation: we need `Send` to transfer the
closure from one thread to another and `'static` because we don’t know how long
the thread will take to execute. Let’s create an `execute` method on
`ThreadPool` that will take a generic parameter of type `F` with these bounds:
-->

`F`型引数にはまた、トレイト境界の`Send`とライフタイム境界の`'static`もあり、この状況では有用です:
あるスレッドから別のスレッドにクロージャを移動するのに`Send`が必要で、スレッドの実行にどれくらいかかるかわからないので、
`'static`も必要です。`ThreadPool`にこれらの境界のジェネリックな型`F`の引数を取る`execute`メソッドを生成しましょう:

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

<!--
We still use the `()` after `FnOnce` because this `FnOnce` represents a closure
that takes no parameters and returns the unit type `()`. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.
-->

それでも、`FnOnce`の後に`()`を使用しています。この`FnOnce`は引数を取らず、ユニット型`()`を返すクロージャを表すからです。
関数定義同様に、戻り値の型はシグニチャから省略できますが、引数がなくても、カッコは必要です。

<!--
Again, this is the simplest implementation of the `execute` method: it does
nothing, but we’re trying only to make our code compile. Let’s check it again:
-->

またもや、これが`execute`メソッドの最も単純な実装です: 何もしませんが、
コードがコンパイルできるようにしようとしているだけです。再確認しましょう:

```console
{{#include ../listings/ch20-web-server/no-listing-03-define-execute/output.txt}}
```

<!--
It compiles! But note that if you try `cargo run` and make a request in the
browser, you’ll see the errors in the browser that we saw at the beginning of
the chapter. Our library isn’t actually calling the closure passed to `execute`
yet!
-->

コンパイルできました！しかし、`cargo run`を試して、
ブラウザでリクエストを行うと、章の冒頭で見かけたエラーがブラウザに現れることに注意してください。
ライブラリは、まだ実際に`execute`に渡されたクロージャを呼び出していないのです！

<!--
> Note: A saying you might hear about languages with strict compilers, such as
> Haskell and Rust, is “if the code compiles, it works.” But this saying is not
> universally true. Our project compiles, but it does absolutely nothing! If we
> were building a real, complete project, this would be a good time to start
> writing unit tests to check that the code compiles *and* has the behavior we
> want.
-->

> 注釈: HaskellやRustなどの厳密なコンパイラがある言語についての格言として「コードがコンパイルできたら、
> 動作する」というものをお聴きになったことがある可能性があります。ですが、この格言は普遍的に当てはまるものではありません。
> このプロジェクトはコンパイルできますが、全く何もしません！本物の完璧なプロジェクトを構築しようとしているのなら、
> ここが単体テストを書き始めて、コードがコンパイルでき、*かつ*欲しい振る舞いを保持していることを確認するのに良い機会でしょう。

<!--
#### Validating the Number of Threads in `new`
-->

#### `new`でスレッド数を検査する

<!--
We aren’t doing anything with the parameters to `new` and `execute`. Let’s
implement the bodies of these functions with the behavior we want. To start,
let’s think about `new`. Earlier we chose an unsigned type for the `size`
parameter, because a pool with a negative number of threads makes no sense.
However, a pool with zero threads also makes no sense, yet zero is a perfectly
valid `usize`. We’ll add code to check that `size` is greater than zero before
we return a `ThreadPool` instance and have the program panic if it receives a
zero by using the `assert!` macro, as shown in Listing 20-13.
-->

`new`と`execute`への引数は、何にも使用していません。欲しい振る舞いでこれらの関数の本体を実装しましょう。
まずはじめに、`new`を考えましょう。先刻、`size`引数に非負整数型を選択しました。負のスレッド数のプールは、
全く道理が通らないからです。しかしながら、0スレッドのプールも全く意味がわかりませんが、0も完全に合法な`usize`です。
`ThreadPool`インスタンスを返す前に`size`が0よりも大きいことを確認するコードを追加し、リスト20-13に示したように、
`assert!`マクロを使用することで0を受け取った時にプログラムをパニックさせます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-13: Implementing `ThreadPool::new` to panic if
`size` is zero</span>
-->

<span class="caption">リスト20-13: `ThreadPool::new`を実装して`size`が0ならパニックする</span>

<!--
2行目後半、calls outを声高に叫ぶとした。叫ぶだけでは何か物足りない気がするので
-->

<!--
We’ve also added some documentation for our `ThreadPool` with doc comments.
Note that we followed good documentation practices by adding a section that
calls out the situations in which our function can panic, as discussed in
Chapter 14. Try running `cargo doc --open` and clicking the `ThreadPool` struct
to see what the generated docs for `new` look like!
-->

doc commentで`ThreadPool`にドキュメンテーションも追加しました。第14章で議論したように、
関数がパニックすることもある場面を声高に叫ぶセクションを追加することで、
いいドキュメンテーションの実践に<ruby>倣<rp>(</rp><rt>なら</rt><rp>)</rp></ruby>っていることに注意してください。
試しに`cargo doc --open`を実行し、`ThreadPool`構造体をクリックして、`new`の生成されるドキュメンテーションがどんな見た目か確かめてください！

<!--
Instead of adding the `assert!` macro as we’ve done here, we could change `new`
into `build` and return a `Result` like we did with `Config::build` in the I/O
project in Listing 12-9. But we’ve decided in this case that trying to create a
thread pool without any threads should be an unrecoverable error. If you’re
feeling ambitious, try to write a function named `build` with the following
signature to compare with the `new` function:
-->

ここでしたように`assert!`マクロを追加する代わりに、リスト12-9のI/Oプロジェクトの`Config::build`のように、
`new`を`build`に変更し、`Result`を返させることもできるでしょう。しかし、今回の場合、スレッドなしでスレッドプールを作成しようとするのは、
回復不能なエラーであるべきと決定しました。野心を感じるのなら、以下のシグニチャを持つ`build`関数を書いてみて、`new`関数と比較してみてください:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

<!--
#### Creating Space to Store the Threads
-->

#### スレッドを格納するスペースを生成する

<!--
Now that we have a way to know we have a valid number of threads to store in
the pool, we can create those threads and store them in the `ThreadPool` struct
before returning the struct. But how do we “store” a thread? Let’s take another
look at the `thread::spawn` signature:
-->

今や、プールに格納する合法なスレッド数を知る方法ができたので、`ThreadPool`構造体を返す前にスレッドを作成して格納できます。
ですが、どのようにスレッドを「格納」するのでしょうか？もう一度、`thread::spawn`シグニチャを眺めてみましょう:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `spawn` function returns a `JoinHandle<T>`, where `T` is the type that the
closure returns. Let’s try using `JoinHandle` too and see what happens. In our
case, the closures we’re passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.
-->

`spawn`関数は、`JoinHandle<T>`を返し、ここで`T`は、クロージャが返す型です。試しに同じように`JoinHandle`を使ってみて、
どうなるか見てみましょう。我々の場合、スレッドプールに渡すクロージャは接続を扱い、何も返さないので、
`T`はユニット型`()`になるでしょう。

<!--
The code in Listing 20-14 will compile but doesn’t create any threads yet.
We’ve changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them.
-->

リスト20-14のコードはコンパイルできますが、まだスレッドは何も生成しません。`ThreadPool`の定義を変更して、
`thread::JoinHandle<()>`インスタンスのベクタを保持し、`size`キャパシティのベクタを初期化し、
スレッドを生成する何らかのコードを実行する`for`ループを設定し、それらを含む`ThreadPool`インスタンスを返します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-14/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-14: Creating a vector for `ThreadPool` to hold
the threads</span>
-->

<span class="caption">リスト20-14: `ThreadPool`にスレッドを保持するベクタを生成する</span>

<!--
We’ve brought `std::thread` into scope in the library crate, because we’re
using `thread::JoinHandle` as the type of the items in the vector in
`ThreadPool`.
-->

ライブラリクレート内で`std::thread`をスコープに導入しました。`ThreadPool`のベクタの要素の型として、
`thread::JoinHandle`を使用しているからです。

<!--
Once a valid size is received, our `ThreadPool` creates a new vector that can
hold `size` items. The `with_capacity` function performs the same task as
`Vec::new` but with an important difference: it preallocates space in the
vector. Because we know we need to store `size` elements in the vector, doing
this allocation up front is slightly more efficient than using `Vec::new`,
which resizes itself as elements are inserted.
-->

一旦、合法なサイズを受け取ったら、`ThreadPool`は`size`個の要素を保持できる新しいベクタを生成します。
`with_capacity`関数は`Vec::new`と同じ作業をしますが、重要な違いがあります:
ベクタに予めスペースを確保しておくのです。ベクタに`size`個の要素を格納する必要があることはわかっているので、
このメモリ確保を前もってしておくと、`Vec::new`よりも少しだけ効率的になります。`Vec::new`は、
要素が挿入されるにつれて、自身のサイズを変更します。

<!--
When you run `cargo check` again, it should succeed.
-->

再び`cargo check`を実行すると、成功するはずです。

<!--
#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a Thread
-->

#### `ThreadPool`からスレッドにコードを送信する責任を負う`Worker`構造体

<!--
We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. Here, we’ll look at how we actually create threads. The standard
library provides `thread::spawn` as a way to create threads, and
`thread::spawn` expects to get some code the thread should run as soon as the
thread is created. However, in our case, we want to create the threads and have
them *wait* for code that we’ll send later. The standard library’s
implementation of threads doesn’t include any way to do that; we have to
implement it manually.
-->

リスト20-14の`for`ループにスレッドの生成に関するコメントを残しました。ここでは、実際にスレッドを生成する方法に目を向けます。
標準ライブラリはスレッドを生成する手段として`thread::spawn`を提供し、`thread::spawn`は、
生成されるとすぐにスレッドが実行すべき何らかのコードを得ることを予期します。ところが、我々の場合、
スレッドを生成して、後ほど送信するコードを*待機*してほしいです。標準ライブラリのスレッドの実装は、
それをするいかなる方法も含んでいません; それを手動で実装しなければなりません。

<!--
We’ll implement this behavior by introducing a new data structure between the
`ThreadPool` and the threads that will manage this new behavior. We’ll call
this data structure *Worker*, which is a common term in pooling
implementations. The Worker picks up code that needs to be run and runs the
code in the Worker’s thread. Think of people working in the kitchen at a
restaurant: the workers wait until orders come in from customers, and then
they’re responsible for taking those orders and fulfilling them.
-->

この新しい振る舞いを管理するスレッドと`ThreadPool`間に新しいデータ構造を導入することでこの振る舞いを実装します。
このデータ構造を*ワーカー*と呼び、プール実装では一般的な用語です。
ワーカーは実行する必要のあるコードを受け取り、ワーカーのスレッドでそのコードを実行します。
レストランのキッチンで働く人々を思い浮かべてください:
労働者は、お客さんからオーダーが来るまで待機し、それからそれらのオーダーを取り、満たすことに責任を負います。

<!--
Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we’ll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then we’ll implement a method on `Worker` that will
take a closure of code to run and send it to the already running thread for
execution. We’ll also give each worker an `id` so we can distinguish between
the different workers in the pool when logging or debugging.
-->

スレッドプールに`JoinHandle<()>`インスタンスのベクタを格納する代わりに、`Worker`構造体のインスタンスを格納します。
各`Worker`が単独の`JoinHandle<()>`インスタンスを格納します。そして、`Worker`に実行するコードのクロージャを取り、
既に走っているスレッドに実行してもらうために送信するメソッドを実装します。ログを取ったり、デバッグする際にプールの異なるワーカーを区別できるように、
各ワーカーに`id`も付与します。

<!--
Here is the new process that will happen when we create a `ThreadPool`. We’ll
implement the code that sends the closure to the thread after we have `Worker`
set up in this way:
-->

`ThreadPool`を作成する際に発生する新しいプロセスは、以下のようになります。
このように`Worker`をセットアップした後に、スレッドにクロージャを送信するコードを実装します:

<!--
1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`.
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the `id` and a thread spawned with an empty
   closure.
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the worker in the vector.
-->

1. `id`と`JoinHandle<()>`を保持する`Worker`構造体を定義する。
2. `ThreadPool`を変更し、`Worker`インスタンスのベクタを保持する。
3. `id`番号を取り、`id`と空のクロージャで大量生産されるスレッドを保持する`Worker`インスタンスを返す`Worker::new`関数を定義する。
4. `ThreadPool::new`で`for`ループカウンタを使用して`id`を生成し、その`id`で新しい`Worker`を生成し、ベクタにワーカーを格納する。

<!--
If you’re up for a challenge, try implementing these changes on your own before
looking at the code in Listing 20-15.
-->

挑戦に積極的ならば、リスト20-15のコードを見る前にご自身でこれらの変更を実装してみてください。

<!--
Ready? Here is Listing 20-15 with one way to make the preceding modifications.
-->

いいですか？こちらが先ほどの変更を行う1つの方法を行ったリスト20-15です。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-15: Modifying `ThreadPool` to hold `Worker`
instances instead of holding threads directly</span>
-->

<span class="caption">リスト20-15: `ThreadPool`を変更してスレッドを直接保持するのではなく、`Worker`インスタンスを保持する</span>

<!--
We’ve changed the name of the field on `ThreadPool` from `threads` to `workers`
because it’s now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.
-->

`ThreadPool`のフィールド名を`threads`から`workers`に変更しました。`JoinHandle<()>`インスタンスではなく、
`Worker`インスタンスを保持するようになったからです。`for`ループのカウンタを`Worker::new`への引数として使用し、
それぞれの新しい`Worker`を`workers`というベクタに格納します。

<!--
External code (like our server in *src/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that is created by spawning a new thread using an empty closure.
-->

外部のコード(*src/main.rs*のサーバなど)は、`ThreadPool`内で`Worker`構造体を使用していることに関する実装の詳細を知る必要はないので、
`Worker`構造体とその`new`関数は非公開にしています。`Worker::new`関数は与えた`id`を使用し、
空のクロージャを使って新しいスレッドを立ち上げることで生成される`JoinHandle<()>`インスタンスを格納します。

<!--
> Note: If the operating system can’t create a thread because there aren’t
> enough system resources, `thread::spawn` will panic. That will cause our
> whole server to panic, even though the creation of some threads might
> succeed. For simplicity’s sake, this behavior is fine, but in a production
> thread pool implementation, you’d likely want to use
> [`std::thread::Builder`][builder] and its
> [`spawn`][builder-spawn] method that returns `Result` instead.
-->

> 注釈: 十分なシステムリソースが無いためにOSがスレッドを作成できない場合、`thread::spawn`はパニックするでしょう。
> これは、一部のスレッドが作成に成功したとしても、サーバ全体のパニックを引き起こすでしょう。
> 簡潔性を優先してこの挙動はまあよしとしますが、実運用のスレッドプール実装では、
> [`std::thread::Builder`][builder]と、パニックするのではなく`Result`を返す[`spawn`][builder-spawn]メソッドを使用するのがよいでしょう。

<!--
This code will compile and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`. But we’re *still* not processing
the closure that we get in `execute`. Let’s look at how to do that next.
-->

このコードはコンパイルでき、`ThreadPool::new`への引数として指定した数の`Worker`インスタンスを格納します。
ですが*それでも*、`execute`で得るクロージャを処理してはいません。次は、それをする方法に目を向けましょう。

<!--
#### Sending Requests to Threads via Channels
-->

#### チャンネル経由でスレッドにリクエストを送信する

<!--
The next problem we’ll tackle is that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method. But we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.
-->

次に取り組む問題は、`thread::spawn`に与えられたクロージャが全く何もしない問題です。現在、
`execute`メソッドで実行したいクロージャを得ています。ですが、`ThreadPool`の生成中、`Worker`それぞれを生成する際に、
実行するクロージャを`thread::spawn`に与える必要があります。

<!--
We want the `Worker` structs that we just created to fetch the code to run from
a queue held in the `ThreadPool` and send that code to its thread to run.
-->

作ったばかりの`Worker`構造体に`ThreadPool`が保持するキューから実行するコードをフェッチして、
そのコードをスレッドが実行できるように送信してほしいです。

<!--
The channels we learned about in Chapter 16—a simple way to communicate between
two threads—would be perfect for this use case. We’ll use a channel to function
as the queue of jobs, and `execute` will send a job from the `ThreadPool` to
the `Worker` instances, which will send the job to its thread. Here is the plan:
-->

第16章で学んだチャンネル—2スレッド間コミュニケーションをとる単純な方法—は、このユースケースにぴったりでしょう。
チャンネルをキューの仕事として機能させ、`execute`は`ThreadPool`から`Worker`インスタンスに仕事を送り、
これが仕事をスレッドに送信します。こちらが計画です:

<!--
1. The `ThreadPool` will create a channel and hold on to the sender.
2. Each `Worker` will hold on to the receiver.
3. We’ll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute through the
   sender.
5. In its thread, the `Worker` will loop over its receiver and execute the
   closures of any jobs it receives.
-->

1. `ThreadPool`はチャンネルを生成し、チャンネルの送信機に就く。
2. `Worker`それぞれは、チャンネルの受信機に就く。
3. チャンネルに送信したいクロージャを保持する新しい`Job`構造体を生成する。
4. `execute`メソッドは、実行したい仕事をチャンネルの送信機を通して送信する。
5. スレッド内で、`Worker`はチャンネルの受信機をループし、受け取ったあらゆる仕事のクロージャを実行する。

<!--
Let’s start by creating a channel in `ThreadPool::new` and holding the sender
in the `ThreadPool` instance, as shown in Listing 20-16. The `Job` struct
doesn’t hold anything for now but will be the type of item we’re sending down
the channel.
-->

`ThreadPool::new`内でチャンネルを生成し、`ThreadPool`インスタンスに送信機を保持することから始めましょう。リスト20-16のようにですね。
今の所、`Job`構造体は何も保持しませんが、チャンネルに送信する種類の要素になります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sender of a channel that transmits `Job` instances</span>
-->

<span class="caption">リスト20-18: `ThreadPool`を変更して`Job`インスタンスを転送するチャンネルの送信機を格納する</span>

<!--
In `ThreadPool::new`, we create our new channel and have the pool hold the
sender. This will successfully compile.
-->

`ThreadPool::new`内で新しいチャンネルを生成し、プールに送信機を保持させています。
これはコンパイルに成功します。

<!--
Let’s try passing a receiver of the channel into each worker as the thread pool
creates the channel. We know we want to use the receiver in the thread that the
workers spawn, so we’ll reference the `receiver` parameter in the closure. The
code in Listing 20-17 won’t quite compile yet.
-->

スレッドプールがチャンネルを生成する際に、各ワーカーにチャンネルの受信機を渡してみましょう。
受信機はワーカーが大量生産するスレッド内で使用したいことがわかっているので、クロージャ内で`receiver`引数を参照します。
リスト20-17のコードはまだ完璧にはコンパイルできません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-17: Passing the receiver to the workers</span>
-->

<span class="caption">リスト20-17: チャンネルの受信機をワーカーに渡す</span>

<!--
We’ve made some small and straightforward changes: we pass the receiver into
`Worker::new`, and then we use it inside the closure.
-->

多少些細で単純な変更を行いました: チャンネルの受信機を`Worker::new`に渡し、それからクロージャの内側で使用しています。

<!--
When we try to check this code, we get this error:
-->

このコードのチェックを試みると、このようなエラーが出ます:

```console
{{#include ../listings/ch20-web-server/listing-20-17/output.txt}}
```

<!--
The code is trying to pass `receiver` to multiple `Worker` instances. This
won’t work, as you’ll recall from Chapter 16: the channel implementation that
Rust provides is multiple *producer*, single *consumer*. This means we can’t
just clone the consuming end of the channel to fix this code. We also don’t
want to send a message multiple times to multiple consumers; we want one list
of messages with multiple workers such that each message gets processed once.
-->

このコードは、`receiver`を複数の`Worker`インスタンスに渡そうとしています。第16章を思い出すように、これは動作しません:
Rustが提供するチャンネル実装は、複数の*生成者*、単独の*消費者*です。要するに、
チャンネルの消費側をクローンするだけでこのコードを修正することはできません。
また、メッセージを複数の消費者に複数回送信したくはありません;
欲しいのは、各メッセージが一度だけ処理されるような、複数のワーカーを備えた、メッセージの単一のリストです。

<!--
Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`;
otherwise, we might get race conditions (as covered in Chapter 16).
-->

さらに、チャンネルキューから仕事を取り出すことは、`receiver`を可変化することに関連するので、
スレッドには、`receiver`を共有して変更する安全な方法が必要です; さもなくば、
競合状態に陥る可能性があります(第16章で講義しました)。

<!--
Recall the thread-safe smart pointers discussed in Chapter 16: to share
ownership across multiple threads and allow the threads to mutate the value, we
need to use `Arc<Mutex<T>>`. The `Arc` type will let multiple workers own the
receiver, and `Mutex` will ensure that only one worker gets a job from the
receiver at a time. Listing 20-18 shows the changes we need to make.
-->

第16章で議論したスレッド安全なスマートポインタを思い出してください: 複数のスレッドで所有権を共有しつつ、
スレッドに値を可変化させるためには、`Arc<Mutex<T>>`を使用する必要があります。`Arc`型は、
複数のワーカーに受信者を所有させ、`Mutex`により、1度に受信者から1つの仕事をたった1つのワーカーが受け取ることを保証します。
リスト20-18は、行う必要のある変更を示しています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-18/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-18: Sharing the receiver among the workers
using `Arc` and `Mutex`</span>
-->

<span class="caption">リスト20-18: `Arc`と`Mutex`を使用してワーカー間でチャンネルの受信機を共有する</span>

<!--
In `ThreadPool::new`, we put the receiver in an `Arc` and a `Mutex`. For each
new worker, we clone the `Arc` to bump the reference count so the workers can
share ownership of the receiver.
-->

`ThreadPool::new`で、チャンネルの受信機を`Arc`と`Mutex`に置いています。新しいワーカーそれぞれに対して、
`Arc`をクローンして参照カウントを跳ね上げているので、ワーカーは受信機の所有権を共有することができます。

<!--
With these changes, the code compiles! We’re getting there!
-->

これらの変更でコードはコンパイルできます！ゴールはもうすぐそこです！

<!--
#### Implementing the `execute` Method
-->

#### `execute`メソッドを実装する

<!--
Let’s finally implement the `execute` method on `ThreadPool`. We’ll also change
`Job` from a struct to a type alias for a trait object that holds the type of
closure that `execute` receives. As discussed in the [“Creating Type Synonyms
with Type Aliases”][creating-type-synonyms-with-type-aliases]
section of Chapter 19, type aliases allow us to make long types shorter for
ease of use. Look at Listing 20-19.
-->

最後に`ThreadPool`に`execute`メソッドを実装しましょう。
`Job`も構造体から`execute`が受け取るクロージャの型を保持するトレイトオブジェクトの型エイリアスに変更します。
第19章の[「型エイリアスで型同義語を生成する」][creating-type-synonyms-with-type-aliases]節で議論したように、
使いやすさのために、型エイリアスを利用して長い型を短くすることができます。
リスト20-19をご覧ください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-19/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-19: Creating a `Job` type alias for a `Box`
that holds each closure and then sending the job down the channel</span>
-->

<span class="caption">リスト20-19: 各クロージャを保持する`Box`に対して`Job`型エイリアスを生成し、それからチャンネルに仕事を送信する</span>

<!--
After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We’re calling `unwrap` on
`send` for the case that sending fails. This might happen if, for example, we
stop all our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, we can’t stop our threads from
executing: our threads continue executing as long as the pool exists. The
reason we use `unwrap` is that we know the failure case won’t happen, but the
compiler doesn’t know that.
-->

`execute`で得たクロージャを使用して新しい`Job`インスタンスを生成した後、その仕事をチャンネルの送信側に送信しています。
送信が失敗した時のために`send`に対して`unwrap`を呼び出しています。これは例えば、全スレッドの実行を停止させるなど、
受信側が新しいメッセージを受け取るのをやめてしまったときなどに起こる可能性があります。現時点では、
スレッドの実行を止めることはできません: スレッドは、プールが存在する限り実行し続けます。
`unwrap`を使用している理由は、失敗する場合が起こらないとわかっているからですが、コンパイラにはわかりません。

<!--
But we’re not quite done yet! In the worker, our closure being passed to
`thread::spawn` still only *references* the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job and running the job when it gets one. Let’s make the change
shown in Listing 20-20 to `Worker::new`.
-->

ですが、まだやり終えたわけではありませんよ！ワーカー内で`thread::spawn`に渡されているクロージャは、
それでもチャンネルの受信側を*参照*しているだけです。その代わりに、クロージャには永遠にループし、
チャンネルの受信側に仕事を要求し、仕事を得たらその仕事を実行してもらう必要があります。
リスト20-20に示した変更を`Worker::new`に行いましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-20: Receiving and executing the jobs in the
worker’s thread</span>
-->

<span class="caption">リスト20-20: ワーカーのスレッドで仕事を受け取り、実行する</span>

<!--
Here, we first call `lock` on the `receiver` to acquire the mutex, and then we
call `unwrap` to panic on any errors. Acquiring a lock might fail if the mutex
is in a *poisoned* state, which can happen if some other thread panicked while
holding the lock rather than releasing the lock. In this situation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you.
-->

ここで、まず`receiver`に対して`lock`を呼び出してミューテックスを獲得し、それから`unwrap`を呼び出して、
エラーの際にはパニックします。ロックの獲得は、ミューテックスが*毒された*状態なら失敗する可能性があり、
これは、他のどれかのスレッドがロックを保持している間に、解放するのではなく、パニックした場合に起き得ます。
この場面では、`unwrap`を呼び出してこのスレッドをパニックさせるのは、取るべき正当な行動です。
この`unwrap`をあなたにとって意味のあるエラーメッセージを伴う`expect`に変更することは、ご自由に行なってください。

<!--
If we get the lock on the mutex, we call `recv` to receive a `Job` from the
channel. A final `unwrap` moves past any errors here as well, which might occur
if the thread holding the sender has shut down, similar to how the `send`
method returns `Err` if the receiver shuts down.
-->

ミューテックスのロックを獲得できたら、`recv`を呼び出してチャンネルから`Job`を受け取ります。
最後の`unwrap`もここであらゆるエラーを超えていき、これはチャンネルの送信機を保持するスレッドが閉じた場合に発生する可能性があり、
受信機が閉じた場合に`send`メソッドが`Err`を返すのと似ています。

<!--
The call to `recv` blocks, so if there is no job yet, the current thread will
wait until a job becomes available. The `Mutex<T>` ensures that only one
`Worker` thread at a time is trying to request a job.
-->

`recv`の呼び出しはブロックするので、まだ仕事がなければ、現在のスレッドは、仕事が利用可能になるまで待機します。
`Mutex<T>`により、ただ1つの`Worker`スレッドのみが一度に仕事の要求を試みることを保証します。

<!--
Our thread pool is now in a working state! Give it a `cargo run` and make some
requests:
-->

これで私たちのスレッドプールは動作する状態になりました！
`cargo run`して、リクエストを送ってみてください:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never read: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: field is never read: `id`
  --> src/lib.rs:48:5
   |
48 |     id: usize,
   |     ^^^^^^^^^

warning: field is never read: `thread`
  --> src/lib.rs:49:5
   |
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `hello` (lib) generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

<!--
Success! We now have a thread pool that executes connections asynchronously.
There are never more than four threads created, so our system won’t get
overloaded if the server receives a lot of requests. If we make a request to
*/sleep*, the server will be able to serve other requests by having another
thread run them.
-->

成功！もう非同期に接続を実行するスレッドプールができました。絶対に4つ以上のスレッドが生成されないので、
サーバが多くのリクエストを受け取っても、システムは過負荷にならないでしょう。*/sleep*にリクエストを行なっても、
サーバは他のスレッドに実行させることで他のリクエストを提供できるでしょう。

<!--
> Note: if you open */sleep* in multiple browser windows simultaneously, they
> might load one at a time in 5 second intervals. Some web browsers execute
> multiple instances of the same request sequentially for caching reasons. This
> limitation is not caused by our web server.
-->

> 注釈: */sleep* を複数のブラウザウィンドウで同時に開くと、5秒間隔でひとつずつロードするかもしれません。
> webブラウザによっては、キャッシュのために、複数の同一リクエストを逐次的に実行します。
> この制限は私たちのwebサーバによって引き起こされているものではありません。

<!--
After learning about the `while let` loop in Chapter 18, you might be wondering
why we didn’t write the worker thread code as shown in Listing 20-21.
-->

第18章で`while let`ループを学んだ後で、なぜリスト20-21に示したようにワーカースレッドのコードを記述しなかったのか、
不思議に思っている可能性があります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-21: An alternative implementation of
`Worker::new` using `while let`</span>
-->

<span class="caption">リスト20-21: `while let`を使用したもう1つの`Worker::new`の実装</span>

<!--
This code compiles and runs but doesn’t result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. At compile time, the borrow checker can then enforce the rule
that a resource guarded by a `Mutex` cannot be accessed unless we hold the
lock. However, this implementation can also result in the lock being held
longer than intended if we aren’t mindful of the lifetime of the
`MutexGuard<T>`.
-->

このコードはコンパイルでき、動きますが、望み通りのスレッドの振る舞いにはなりません:
遅いリクエストがそれでも、他のリクエストが処理されるのを待機させてしまうのです。理由はどこか捉えがたいものです:
`Mutex`構造体には公開の`unlock`メソッドがありません。ロックの所有権が、
`lock`メソッドが返す`LockResult<MutexGuard<T>>`内の`MutexGuard<T>`のライフタイムに基づくからです。
コンパイル時には、ロックを保持していない限り、借用チェッカーはそうしたら、`Mutex`に保護されるリソースにはアクセスできないという規則を強制できます。
しかし、この実装は、`MutexGuard<T>`のライフタイムを心に留めなければ、
意図したよりもロックが長い間保持される結果になり得ます。

<!--
The code in Listing 20-20 that uses `let job =
receiver.lock().unwrap().recv().unwrap();` works because with `let`, any
temporary values used in the expression on the right hand side of the equals
sign are immediately dropped when the `let` statement ends. However, `while
let` (and `if let` and `match`) does not drop temporary values until the end of
the associated block. In Listing 20-21, the lock remains held for the duration
of the call to `job()`, meaning other workers cannot receive jobs.
-->

`let job = receiver.lock().unwrap().recv().unwrap();`を使用するリスト20-20のコードは、機能します。
`let`を使用する場合、等号の右辺の式内で使用される任意の一時値は、`let`文の終わりの直後にドロップされるからです。
しかしながら、`while let`(と`if let`と`match`)は、関連するブロックの終わりまで一時値をドロップしません。
リスト20-21では、`job()`への呼び出しの間ロックは保持されたままになり、
他のワーカーがジョブを受信できないことになります。

<!--
[creating-type-synonyms-with-type-aliases]:
ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[fn-traits]:
ch13-01-closures.html#moving-captured-values-out-of-the-closure-and-the-fn-traits
[builder]: ../std/thread/struct.Builder.html
[builder-spawn]: ../std/thread/struct.Builder.html#method.spawn
-->

[creating-type-synonyms-with-type-aliases]:
ch19-04-advanced-types.html#型エイリアスで型同義語を生成する
[integer-types]: ch03-02-data-types.html#整数型
[fn-traits]:
ch13-01-closures.html#キャプチャされた値のクロージャからのムーブとfn系トレイト
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
