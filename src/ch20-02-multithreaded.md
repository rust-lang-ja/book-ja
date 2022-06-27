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

```rust
use std::thread;
use std::time::Duration;
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 1024];
#     stream.read(&mut buffer).unwrap();
    // --snip--

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // --snip--
}
```

<!--
<span class="caption">Listing 20-10: Simulating a slow request by recognizing
*/sleep* and sleeping for 5 seconds</span>
-->

<span class="caption">リスト20-10: */sleep*を認識して5秒間スリープすることで遅いリクエストをシミュレーションする</span>

<!--
This code is a bit messy, but it’s good enough for simulation purposes. We
created a second request `sleep`, whose data our server recognizes. We added an
`else if` after the `if` block to check for the request to */sleep*. When that
request is received, the server will sleep for 5 seconds before rendering the
successful HTML page.
-->

このコードはちょっと汚いですが、シミュレーション目的には十分です。2番目のリクエスト`sleep`を作成し、
そのデータをサーバは認識します。`if`ブロックの後に`else if`を追加し、*/sleep*へのリクエストを確認しています。
そのリクエストが受け付けられると、サーバは成功のHTMLページを描画する前に5秒間スリープします。

<!--
You can see how primitive our server is: real libraries would handle the
recognition of multiple requests in a much less verbose way!
-->

我々のサーバがどれだけ基礎的か見て取れます: 本物のライブラリは、もっと冗長でない方法で複数のリクエストの認識を扱うでしょう！

<!--
Start the server using `cargo run`. Then open two browser windows: one for
*http://localhost:7878/* and the other for *http://localhost:7878/sleep*. If
you enter the */* URI a few times, as before, you’ll see it respond quickly.
But if you enter */sleep*, and then load */*, you’ll see that */* waits until
`sleep` has slept for its full 5 seconds before loading.
-->

`cargo run`でサーバを開始してください。それから2つブラウザのウインドウを開いてください: 1つは、
*http://localhost:7878/* 用、そしてもう1つは*http://localhost:7878/sleep* 用です。
以前のように */* URIを数回入力したら、素早く応答するでしょう。しかし、*/sleep*を入力し、それから */* をロードしたら、
`sleep`がロードする前にきっかり5秒スリープし終わるまで、*/* は待機するのを目撃するでしょう。

<!--
There are multiple ways we could change how our web server works to avoid
having more requests back up behind a slow request; the one we’ll implement is
a thread pool.
-->

より多くのリクエストが遅いリクエストの背後に回ってしまうのを回避するようWebサーバが動く方法を変える方法は複数あります;
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
Rather than spawning unlimited threads, we’ll have a fixed number of threads
waiting in the pool. As requests come in, they’ll be sent to the pool for
processing. The pool will maintain a queue of incoming requests. Each of the
threads in the pool will pop off a request from this queue, handle the request,
and then ask the queue for another request. With this design, we can process
`N` requests concurrently, where `N` is the number of threads. If each thread
is responding to a long-running request, subsequent requests can still back up
in the queue, but we’ve increased the number of long-running requests we can
handle before reaching that point.
-->

無制限にスレッドを大量生産するのではなく、プールに固定された数のスレッドを待機させます。リクエストが来る度に、
処理するためにプールに送られます。プールは、やって来るリクエストのキューを管理します。
プールの各スレッドがこのキューからリクエストを取り出し、リクエストを処理し、そして、別のリクエストをキューに要求します。
この設計により、`N`リクエストを並行して処理でき、ここで`N`はスレッド数です。各スレッドが実行に時間のかかるリクエストに応答していたら、
続くリクエストはそれでも、キュー内で待機させられてしまうこともありますが、その地点に到達する前に扱える時間のかかるリクエスト数を増加させました。

<!--
This technique is just one of many ways to improve the throughput of a web
server. Other options you might explore are the fork/join model and the
single-threaded async I/O model. If you’re interested in this topic, you can
read more about other solutions and try to implement them in Rust; with a
low-level language like Rust, all of these options are possible.
-->

このテクニックは、Webサーバのスループットを向上させる多くの方法の1つに過ぎません。探究する可能性のある他の選択肢は、
fork/joinモデルと、シングルスレッドの非同期I/Oモデルです。この話題にご興味があれば、他の解決策についてもっと読み、
Rustで実装を試みることができます; Rustのような低レベル言語であれば、これらの選択肢全部が可能なのです。

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
what we should change next to get the code to work.
-->

第12章のプロジェクトでTDDを使用したように、ここではCompiler Driven Development(コンパイラ駆動開発)を使用します。
欲しい関数を呼び出すコードを書き、それからコンパイラの出すエラーを見てコードが動くように次に何を変更すべきかを決定します。

<!--
#### Code Structure If We Could Spawn a Thread for Each Request
-->

#### 各リクエストに対してスレッドを立ち上げられる場合のコードの構造

<!--
First, let’s explore how our code might look if it did create a new thread for
every connection. As mentioned earlier, this isn’t our final plan due to the
problems with potentially spawning an unlimited number of threads, but it is a
starting point. Listing 20-11 shows the changes to make to `main` to spawn a
new thread to handle each stream within the `for` loop.
-->

まず、全接続に対して新しいスレッドを確かに生成した場合にコードがどんな見た目になるかを探究しましょう。
先ほど述べたように、無制限にスレッドを大量生産する可能性があるという問題のため、これは最終的な計画ではありませんが、
開始点です。リスト20-11は、新しいスレッドを立ち上げて`for`ループ内で各ストリームを扱うために`main`に行う変更を示しています。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
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
that the requests to */* don’t have to wait for */sleep* to finish. But as we
mentioned, this will eventually overwhelm the system because you'd making
new threads without any limit.
-->

第16章で学んだように、`thread::spawn`は新しいスレッドを生成し、それからクロージャ内のコードを新しいスレッドで実行します。
このコードを実行してブラウザで */sleep*をロードし、それからもう2つのブラウザのタブで */* をロードしたら、
確かに */* へのリクエストは、*/sleep*が完了するのを待機しなくても済むことがわかるでしょう。
ですが、前述したように、無制限にスレッドを生成することになるので、これは最終的にシステムを参らせてしまうでしょう。

<!--
#### Creating a Similar Interface for a Finite Number of Threads
-->

#### 有限数のスレッド用に似たインターフェイスを作成する

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

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# struct ThreadPool;
# impl ThreadPool {
#    fn new(size: u32) -> ThreadPool { ThreadPool }
#    fn execute<F>(&self, f: F)
#        where F: FnOnce() + Send + 'static {}
# }
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
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

<!--
#### Building the `ThreadPool` Struct Using Compiler Driven Development
-->

#### コンパイラ駆動開発で`ThreadPool`構造体を構築する

<!--
Make the changes in Listing 20-12 to *src/main.rs*, and then let’s use the
compiler errors from `cargo check` to drive our development. Here is the first
error we get:
-->

リスト20-12の変更を*src/main.rs*に行い、それから開発を駆動するために`cargo check`からのコンパイラエラーを活用しましょう。
こちらが得られる最初のエラーです:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
(エラー: 解決に失敗しました。未定義の型またはモジュール`ThreadPool`を使用しています)
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
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

```rust
pub struct ThreadPool;
```

<!--
Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. Doing so will make the library crate the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run`. After moving the *main.rs* file, edit it
to bring the library crate in and bring `ThreadPool` into scope by adding the
following code to the top of *src/bin/main.rs*:
-->

それから新しいディレクトリ、*src/bin*を作成し、*src/main.rs*に根付くバイナリクレートを*src/bin/main.rs*に移動してください。
そうすると、ライブラリクレートが*hello*ディレクトリ内で主要クレートになります; それでも、
`cargo run`で*src/bin/main.rs*のバイナリを実行することはできます。*main.rs*ファイルを移動後、
編集してライブラリクレートを持ち込み、以下のコードを*src/bin/main.rs*の先頭に追記して`ThreadPool`をスコープに導入してください:

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">ファイル名: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

<!--
This code still won’t work, but let’s check it again to get the next error that
we need to address:
-->

このコードはまだ動きませんが、再度それを確認して扱う必要のある次のエラーを手に入れましょう:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0599]: no function or associated item named `new` found for type
`hello::ThreadPool` in the current scope
(エラー: 現在のスコープで型`hello::ThreadPool`の関数または関連アイテムに`new`というものが見つかりません)
 --> src/bin/main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ function or associated item not found in
   `hello::ThreadPool`
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

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

<!--
We chose `usize` as the type of the `size` parameter, because we know that a
negative number of threads doesn’t make any sense. We also know we’ll use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the “Integer Types” section of Chapter 3.
-->

`size`引数の型として、`usize`を選択しました。何故なら、マイナスのスレッド数は、何も筋が通らないことを知っているからです。
また、この4をスレッドのコレクションの要素数として使用し、第3章の「整数型」節で議論したように、これは`usize`のあるべき姿であることも知っています。

<!--
Let’s check the code again:
-->

コードを再度確認しましょう:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
(警告: 未使用の変数: `size`)
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

error[E0599]: no method named `execute` found for type `hello::ThreadPool` in the current scope
  --> src/bin/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

<!--
Now we get a warning and an error. Ignoring the warning for a moment, the error
occurs because we don’t have an `execute` method on `ThreadPool`. Recall from
the “Creating a Similar Interface for a Finite Number of Threads” section that
we decided our thread pool should have an interface similar to `thread::spawn`.
In addition, we’ll implement the `execute` function so it takes the closure
it’s given and gives it to an idle thread in the pool to run.
-->

今度は、警告とエラーが出ました。一時的に警告は無視して、`ThreadPool`に`execute`メソッドがないためにエラーが発生しました。
「有限数のスレッド用に似たインターフェイスを作成する」節で我々のスレッドプールは、
`thread::spawn`と似たインターフェイスにするべきと決定したことを思い出してください。
さらに、`execute`関数を実装するので、与えられたクロージャを取り、実行するようにプールの待機中のスレッドに渡します。

<!--
We’ll define the `execute` method on `ThreadPool` to take a closure as a
parameter. Recall from the “Storing Closures Using Generic Parameters and the
`Fn` Traits” section in Chapter 13 that we can take closures as parameters with
three different traits: `Fn`, `FnMut`, and `FnOnce`. We need to decide which
kind of closure to use here. We know we’ll end up doing something similar to
the standard library `thread::spawn` implementation, so we can look at what
bounds the signature of `thread::spawn` has on its parameter. The documentation
shows us the following:
-->

`ThreadPool`に`execute`メソッドをクロージャを引数として受け取るように定義します。
第13章の「ジェネリック引数と`Fn`トレイトを使用してクロージャを保存する」節から、
3つの異なるトレイトでクロージャを引数として取ることができることを思い出してください: `Fn`、`FnMut`、`FnOnce`です。
ここでは、どの種類のクロージャを使用するか決定する必要があります。最終的には、
標準ライブラリの`thread::spawn`実装に似たことをすることがわかっているので、
`thread::spawn`のシグニチャで引数にどんな境界があるか見ることができます。ドキュメンテーションは、以下のものを示しています:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
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

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

<!--
We still use the `()` after `FnOnce` because this `FnOnce` represents a closure
that takes no parameters and doesn’t return a value. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.
-->

それでも、`FnOnce`の後に`()`を使用しています。この`FnOnce`は引数を取らず、値も返さないクロージャを表すからです。
関数定義同様に、戻り値の型はシグニチャから省略できますが、引数がなくても、カッコは必要です。

<!--
Again, this is the simplest implementation of the `execute` method: it does
nothing, but we’re trying only to make our code compile. Let’s check it again:
-->

またもや、これが`execute`メソッドの最も単純な実装です: 何もしませんが、
コードがコンパイルできるようにしようとしているだけです。再確認しましょう:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

warning: unused variable: `f`
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
  |
  = note: to avoid this warning, consider using `_f` instead
```

<!--
We’re receiving only warnings now, which means it compiles! But note that if
you try `cargo run` and make a request in the browser, you’ll see the errors in
the browser that we saw at the beginning of the chapter. Our library isn’t
actually calling the closure passed to `execute` yet!
-->

これで警告を受け取るだけになり、コンパイルできるようになりました！しかし、`cargo run`を試して、
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
We’ll continue to get warnings because we aren’t doing anything with the
parameters to `new` and `execute`. Let’s implement the bodies of these
functions with the behavior we want. To start, let’s think about `new`. Earlier
we chose an unsigned type for the `size` parameter, because a pool with a
negative number of threads makes no sense. However, a pool with zero threads
also makes no sense, yet zero is a perfectly valid `usize`. We’ll add code to
check that `size` is greater than zero before we return a `ThreadPool` instance
and have the program panic if it receives a zero by using the `assert!` macro,
as shown in Listing 20-13.
-->

`new`と`execute`の引数で何もしていないので、警告が出続けます。欲しい振る舞いでこれらの関数の本体を実装しましょう。
まずはじめに、`new`を考えましょう。先刻、`size`引数に非負整数型を選択しました。負のスレッド数のプールは、
全く道理が通らないからです。しかしながら、0スレッドのプールも全く意味がわかりませんが、0も完全に合法な`usize`です。
`ThreadPool`インスタンスを返す前に`size`が0よりも大きいことを確認するコードを追加し、リスト20-13に示したように、
`assert!`マクロを使用することで0を受け取った時にプログラムをパニックさせます。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    ///
    /// sizeがプールのスレッド数です。
    ///
    /// # パニック
    ///
    /// sizeが0なら、`new`関数はパニックします。
    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // --snip--
}
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
We’ve added some documentation for our `ThreadPool` with doc comments. Note
that we followed good documentation practices by adding a section that calls
out the situations in which our function can panic, as discussed in Chapter 14.
Try running `cargo doc --open` and clicking the `ThreadPool` struct to see what
the generated docs for `new` look like!
-->

doc commentで`ThreadPool`にドキュメンテーションを追加しました。第14章で議論したように、
関数がパニックすることもある場面を声高に叫ぶセクションを追加することで、
いいドキュメンテーションの実践に<ruby>倣<rp>(</rp><rt>なら</rt><rp>)</rp></ruby>っていることに注意してください。
試しに`cargo doc --open`を実行し、`ThreadPool`構造体をクリックして、`new`の生成されるドキュメンテーションがどんな見た目か確かめてください！

<!--
Instead of adding the `assert!` macro as we’ve done here, we could make `new`
return a `Result` like we did with `Config::new` in the I/O project in Listing
12-9. But we’ve decided in this case that trying to create a thread pool
without any threads should be an unrecoverable error. If you’re feeling
ambitious, try to write a version of `new` with the following signature to
compare both versions:
-->

ここでしたように`assert!`マクロを追加する代わりに、リスト12-9のI/Oプロジェクトの`Config::new`のように、
`new`に`Result`を返させることもできるでしょう。しかし、今回の場合、スレッドなしでスレッドプールを作成しようとするのは、
回復不能なエラーであるべきと決定しました。野心を感じるのなら、以下のシグニチャの`new`も書いてみて、両者を比較してみてください:

```rust,ignore
pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

<!--
#### Creating Space to Store the Threads
-->

#### スレッドを格納するスペースを生成する

<!--
Now that we have a way to know we have a valid number of threads to store in
the pool, we can create those threads and store them in the `ThreadPool` struct
before returning it. But how do we “store” a thread? Let’s take another look at
the `thread::spawn` signature:
-->

今や、プールに格納する合法なスレッド数を知る方法ができたので、`ThreadPool`構造体を返す前にスレッドを作成して格納できます。
ですが、どのようにスレッドを「格納」するのでしょうか？もう一度、`thread::spawn`シグニチャを眺めてみましょう:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
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

```rust,ignore
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // スレッドを生成してベクタに格納する
            // create some threads and store them in the vector
        }

        ThreadPool {
            threads
        }
    }

    // --snip--
}
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
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which performs the same task as `Vec::new` but with an important
difference: it preallocates space in the vector. Because we know we need to
store `size` elements in the vector, doing this allocation up front is slightly
more efficient than using `Vec::new`, which resizes itself as elements are
inserted.
-->

一旦、合法なサイズを受け取ったら、`ThreadPool`は`size`個の要素を保持できる新しいベクタを生成します。
この本ではまだ、`with_capacity`関数を使用したことがありませんが、これは`Vec::new`と同じ作業をしつつ、
重要な違いがあります: ベクタに予めスペースを確保しておくのです。ベクタに`size`個の要素を格納する必要があることはわかっているので、
このメモリ確保を前もってしておくと、`Vec::new`よりも少しだけ効率的になります。`Vec::new`は、
要素が挿入されるにつれて、自身のサイズを変更します。

<!--
When you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.
-->

再び`cargo check`を実行すると、もういくつか警告が出るものの、成功するはずです。

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
this data structure `Worker`, which is a common term in pooling
implementations. Think of people working in the kitchen at a restaurant: the
workers wait until orders come in from customers, and then they’re responsible
for taking those orders and filling them.
-->

この新しい振る舞いを管理するスレッドと`ThreadPool`間に新しいデータ構造を導入することでこの振る舞いを実装します。
このデータ構造を`Worker`と呼び、プール実装では一般的な用語です。レストランのキッチンで働く人々を思い浮かべてください:
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
Let’s make the following changes to what happens when we create a `ThreadPool`.
We’ll implement the code that sends the closure to the thread after we have
`Worker` set up in this way:
-->

`ThreadPool`を生成する際に発生することに以下の変更を加えましょう。このように`Worker`をセットアップした後に、
スレッドにクロージャを送信するコードを実装します:

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

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }
    // --snip--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
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
External code (like our server in *src/bin/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that is created by spawning a new thread using an empty closure.
-->

外部のコード(*src/bin/main.rs*のサーバなど)は、`ThreadPool`内で`Worker`構造体を使用していることに関する実装の詳細を知る必要はないので、
`Worker`構造体とその`new`関数は非公開にしています。`Worker::new`関数は与えた`id`を使用し、
空のクロージャを使って新しいスレッドを立ち上げることで生成される`JoinHandle<()>`インスタンスを格納します。

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
Now we’ll tackle the problem that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method. But we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.
-->

さて、`thread::spawn`に与えられたクロージャが全く何もしないという問題に取り組みましょう。現在、
`execute`メソッドで実行したいクロージャを得ています。ですが、`ThreadPool`の生成中、`Worker`それぞれを生成する際に、
実行するクロージャを`thread::spawn`に与える必要があります。

<!--
We want the `Worker` structs that we just created to fetch code to run from a
queue held in the `ThreadPool` and send that code to its thread to run.
-->

作ったばかりの`Worker`構造体に`ThreadPool`が保持するキューから実行するコードをフェッチして、
そのコードをスレッドが実行できるように送信してほしいです。

<!--
In Chapter 16, you learned about *channels*—a simple way to communicate between
two threads—that would be perfect for this use case. We’ll use a channel to
function as the queue of jobs, and `execute` will send a job from the
`ThreadPool` to the `Worker` instances, which will send the job to its thread.
Here is the plan:
-->

第16章でこのユースケースにぴったりであろう*チャンネル*(2スレッド間コミュニケーションをとる単純な方法)について学びました。
チャンネルをキューの仕事として機能させ、`execute`は`ThreadPool`から`Worker`インスタンスに仕事を送り、
これが仕事をスレッドに送信します。こちらが計画です:

<!--
1. The `ThreadPool` will create a channel and hold on to the sending side of
the channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We’ll create a new `Job` struct that will hold the closures we want to send
down the channel.
4. The `execute` method will send the job it wants to execute down the sending
side of the channel.
5. In its thread, the `Worker` will loop over its receiving side of the channel
and execute the closures of any jobs it receives.
-->

1. `ThreadPool`はチャンネルを生成し、チャンネルの送信側に就く。
2. `Worker`それぞれは、チャンネルの受信側に就く。
3. チャンネルに送信したいクロージャを保持する新しい`Job`構造体を生成する。
4. `execute`メソッドは、実行したい仕事をチャンネルの送信側に送信する。
5. スレッド内で、`Worker`はチャンネルの受信側をループし、受け取ったあらゆる仕事のクロージャを実行する。

<!--
Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. The `Job` struct
doesn’t hold anything for now but will be the type of item we’re sending down
the channel.
-->

`ThreadPool::new`内でチャンネルを生成し、`ThreadPool`インスタンスに送信側を保持することから始めましょう。リスト20-16のようにですね。
今の所、`Job`構造体は何も保持しませんが、チャンネルに送信する種類の要素になります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
# use std::thread;
// --snip--
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --snip--
}
#
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
# impl Worker {
#     fn new(id: usize) -> Worker {
#         let thread = thread::spawn(|| {});
#
#         Worker {
#             id,
#             thread,
#         }
#     }
# }
```

<!--
<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sending end of a channel that sends `Job` instances</span>
-->

<span class="caption">リスト20-18: `ThreadPool`を変更して`Job`インスタンスを送信するチャンネルの送信側を格納する</span>

<!--
In `ThreadPool::new`, we create our new channel and have the pool hold the
sending end. This will successfully compile, still with warnings.
-->

`ThreadPool::new`内で新しいチャンネルを生成し、プールに送信側を保持させています。これはコンパイルに成功しますが、
まだ警告があります。

<!--
Let’s try passing a receiving end of the channel into each worker as the thread
pool creates them. We know we want to use the receiving end in the
thread that the workers spawn, so we’ll reference the `receiver` parameter in
the closure. The code in Listing 20-17 won’t quite compile yet.
-->

スレッドプールがワーカーを生成する際に各ワーカーにチャンネルの受信側を試しに渡してみましょう。
受信側はワーカーが大量生産するスレッド内で使用したいことがわかっているので、クロージャ内で`receiver`引数を参照します。
リスト20-17のコードはまだ完璧にはコンパイルできません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --snip--
}

// --snip--

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<!--
<span class="caption">Listing 20-17: Passing the receiving end of the channel
to the workers</span>
-->

<span class="caption">リスト20-17: チャンネルの受信側をワーカーに渡す</span>

<!--
We’ve made some small and straightforward changes: we pass the receiving end of
the channel into `Worker::new`, and then we use it inside the closure.
-->

多少些細で単純な変更を行いました: チャンネルの受信側を`Worker::new`に渡し、それからクロージャの内側で使用しています。

<!--
When we try to check this code, we get this error:
-->

このコードのチェックを試みると、このようなエラーが出ます:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

<!--
The code is trying to pass `receiver` to multiple `Worker` instances. This
won’t work, as you’ll recall from Chapter 16: the channel implementation that
Rust provides is multiple *producer*, single *consumer*. This means we can’t
just clone the consuming end of the channel to fix this code. Even if we could,
that is not the technique we would want to use; instead, we want to distribute
the jobs across threads by sharing the single `receiver` among all the workers.
-->

このコードは、`receiver`を複数の`Worker`インスタンスに渡そうとしています。第16章を思い出すように、これは動作しません:
Rustが提供するチャンネル実装は、複数の*生成者*、単独の*消費者*です。要するに、
チャンネルの消費側をクローンするだけでこのコードを修正することはできません。たとえできたとしても、
使用したいテクニックではありません; 代わりに、全ワーカー間で単独の`receiver`を共有することで、
スレッド間に仕事を分配したいです。

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

```rust
# use std::thread;
# use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// --snip--

# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# struct Job;
#
impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // --snip--
}

# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
#         let thread = thread::spawn(|| {
#            receiver;
#         });
#
#         Worker {
#             id,
#             thread,
#         }
    }
}
```

<!--
<span class="caption">Listing 20-18: Sharing the receiving end of the channel
among the workers using `Arc` and `Mutex`</span>
-->

<span class="caption">リスト20-18: `Arc`と`Mutex`を使用してワーカー間でチャンネルの受信側を共有する</span>

<!--
In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.
-->

`ThreadPool::new`で、チャンネルの受信側を`Arc`と`Mutex`に置いています。新しいワーカーそれぞれに対して、
`Arc`をクローンして参照カウントを跳ね上げているので、ワーカーは受信側の所有権を共有することができます。

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
closure that `execute` receives. As discussed in the “Creating Type Synonyms
with Type Aliases” section of Chapter 19, type aliases allow us to make long
types shorter. Look at Listing 20-19.
-->

最後に`ThreadPool`に`execute`メソッドを実装しましょう。
`Job`も構造体から`execute`が受け取るクロージャの型を保持するトレイトオブジェクトの型エイリアスに変更します。
第19章の「型エイリアスで型同義語を生成する」節で議論したように、型エイリアスにより長い型を短くできます。
リスト20-19をご覧ください。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust
// --snip--
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# use std::sync::mpsc;
# struct Worker {}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// --snip--
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

```rust,ignore
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                // ワーカー{}は仕事を得ました; 実行します
                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
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
if the thread holding the sending side of the channel has shut down, similar to
how the `send` method returns `Err` if the receiving side shuts down.
-->

ミューテックスのロックを獲得できたら、`recv`を呼び出してチャンネルから`Job`を受け取ります。
最後の`unwrap`もここであらゆるエラーを超えていき、これはチャンネルの送信側を保持するスレッドが閉じた場合に発生する可能性があり、
受信側が閉じた場合に`send`メソッドが`Err`を返すのと似ています。

<!--
The call to `recv` blocks, so if there is no job yet, the current thread will
wait until a job becomes available. The `Mutex<T>` ensures that only one
`Worker` thread at a time is trying to request a job.
-->

`recv`の呼び出しはブロックするので、まだ仕事がなければ、現在のスレッドは、仕事が利用可能になるまで待機します。
`Mutex<T>`により、ただ1つの`Worker`スレッドのみが一度に仕事の要求を試みることを保証します。

<!--
Theoretically, this code should compile. Unfortunately, the Rust compiler isn’t
perfect yet, and we get this error:
-->

理論的には、このコードはコンパイルできるはずです。残念ながら、Rustコンパイラはまだ完全ではなく、
このようなエラーが出ます:

```text
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send: the size of std::ops::FnOnce() + std::marker::Send cannot be
statically determined
(エラー: std::ops::FnOnce() + std::marker::Sendの値をムーブできません:
std::ops::FnOnce() + std::marker::Sendのサイズを静的に決定できません)
  --> src/lib.rs:63:17
   |
63 |                 (*job)();
   |                 ^^^^^^
```

<!--
This error is fairly cryptic because the problem is fairly cryptic. To call a
`FnOnce` closure that is stored in a `Box<T>` (which is what our `Job` type
alias is), the closure needs to move itself *out* of the `Box<T>` because the
closure takes ownership of `self` when we call it. In general, Rust doesn’t
allow us to move a value out of a `Box<T>` because Rust doesn’t know how big
the value inside the `Box<T>` will be: recall in Chapter 15 that we used
`Box<T>` precisely because we had something of an unknown size that we wanted
to store in a `Box<T>` to get a value of a known size.
-->

問題が非常に謎めいているので、エラーも非常に謎めいています。`Box<T>`に格納された`FnOnce`クロージャを呼び出すためには(`Job`型エイリアスがそう)、
呼び出す際にクロージャが`self`の所有権を奪うので、
クロージャは自身を`Box<T>`*から*ムーブする必要があります。一般的に、Rustは`Box<T>`から値をムーブすることを許可しません。
コンパイラには、`Box<T>`の内側の値がどれほどの大きさなのか見当がつかないからです: 
第15章で`Box<T>`に格納して既知のサイズの値を得たい未知のサイズの何かがあるために`Box<T>`を正確に使用したことを思い出してください。

<!--
As you saw in Listing 17-15, we can write methods that use the syntax `self:
Box<Self>`, which allows the method to take ownership of a `Self` value stored
in a `Box<T>`. That’s exactly what we want to do here, but unfortunately Rust
won’t let us: the part of Rust that implements behavior when a closure is
called isn’t implemented using `self: Box<Self>`. So Rust doesn’t yet
understand that it could use `self: Box<Self>` in this situation to take
ownership of the closure and move the closure out of the `Box<T>`.
-->

リスト17-15で見かけたように、記法`self: Box<Self>`を使用するメソッドを書くことができ、
これにより、メソッドは`Box<T>`に格納された`Self`値の所有権を奪うことができます。
それがまさしくここで行いたいことですが、残念ながらコンパイラはさせてくれません:
クロージャが呼び出された際に振る舞いを実装するRustの一部は、`self: Box<Self>`を使用して実装されていないのです。
故に、コンパイラはまだこの場面において`self: Box<Self>`を使用してクロージャの所有権を奪い、
クロージャを`Box<T>`からムーブできることを理解していないのです。

<!--
Rust is still a work in progress with places where the compiler could be
improved, but in the future, the code in Listing 20-20 should work just fine.
People just like you are working to fix this and other issues! After you’ve
finished this book, we would love for you to join in.
-->

Rustはまだコンパイラの改善途上にあり、リスト20-20のコードは、
将来的にうまく動くようになるべきです。まさしくあなたのような方がこれや他の問題を修正しています！この本を完了したら、
是非ともあなたにも参加していただきたいです。

<!--
But for now, let’s work around this problem using a handy trick. We can tell
Rust explicitly that in this case we can take ownership of the value inside the
`Box<T>` using `self: Box<Self>`; then, once we have ownership of the closure,
we can call it. This involves defining a new trait `FnBox` with the method
`call_box` that will use `self: Box<Self>` in its signature, defining `FnBox`
for any type that implements `FnOnce()`, changing our type alias to use the new
trait, and changing `Worker` to use the `call_box` method. These changes are
shown in Listing 20-21.
-->

ですがとりあえず、手頃なトリックを使ってこの問題を回避しましょう。この場合、`self: Box<Self>`で、
`Box<T>`の内部の値の所有権を奪うことができることをコンパイラに明示的に教えてあげます;
そして、一旦クロージャの所有権を得たら、呼び出せます。これには、
シグニチャに`self: Box<Self>`を使用する`call_box`というメソッドのある新しいトレイト`FnBox`を定義すること、
`FnOnce()`を実装する任意の型に対して`FnBox`を定義すること、型エイリアスを新しいトレイトを使用するように変更すること、
`Worker`を`call_box`メソッドを使用するように変更することが関連します。これらの変更は、
リスト20-21に表示されています。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<!--
<span class="caption">Listing 20-21: Adding a new trait `FnBox` to work around
the current limitations of `Box<FnOnce()>`</span>
-->

<span class="caption">リスト20-21: 新しいトレイト`FnBox`を追加して`Box<FnOnce()>`の現在の制限を回避する</span>

<!--
First, we create a new trait named `FnBox`. This trait has the one method
`call_box`, which is similar to the `call` methods on the other `Fn*` traits
except that it takes `self: Box<Self>` to take ownership of `self` and move the
value out of the `Box<T>`.
-->

まず、`FnBox`という新しいトレイトを作成します。このトレイトには`call_box`という1つのメソッドがあり、
これは、`self: Box<Self>`を取って`self`の所有権を奪い、`Box<T>`から値をムーブする点を除いて、
他の`Fn*`トレイトの`call`メソッドと類似しています。

<!--
Next, we implement the `FnBox` trait for any type `F` that implements the
`FnOnce()` trait. Effectively, this means that any `FnOnce()` closures can use
our `call_box` method. The implementation of `call_box` uses `(*self)()` to
move the closure out of the `Box<T>` and call the closure.
-->

次に、`FnOnce()`トレイトを実装する任意の型`F`に対して`FnBox`トレイトを実装します。実質的にこれは、
あらゆる`FnOnce()`クロージャが`call_box`メソッドを使用できることを意味します。`call_box`の実装は、
`(*self)()`を使用して`Box<T>`からクロージャをムーブし、クロージャを呼び出します。

<!--
We now need our `Job` type alias to be a `Box` of anything that implements our
new trait `FnBox`. This will allow us to use `call_box` in `Worker` when we get
a `Job` value instead of invoking the closure directly. Implementing the
`FnBox` trait for any `FnOnce()` closure means we don’t have to change anything
about the actual values we’re sending down the channel. Now Rust is able to
recognize that what we want to do is fine.
-->

これで`Job`型エイリアスには、新しいトレイトの`FnBox`を実装する何かの`Box`である必要が出てきました。
これにより、クロージャを直接呼び出す代わりに`Job`値を得た時に`Worker`の`call_box`を使えます。
任意の`FnOnce()`クロージャに対して`FnBox`トレイトを実装することは、チャンネルに送信する実際の値は何も変えなくてもいいことを意味します。
もうコンパイラは、我々が行おうとしていることが平気なことであると認識できます。

<!--
This trick is very sneaky and complicated. Don’t worry if it doesn’t make
perfect sense; someday, it will be completely unnecessary.
-->

このトリックは非常にこそこそしていて複雑です。完璧に筋が通らなくても心配しないでください;
いつの日か、完全に不要になるでしょう。

<!--
With the implementation of this trick, our thread pool is in a working state!
Give it a `cargo run`, and make some requests:
-->

このトリックの実装で、スレッドプールは動く状態になります！`cargo run`を実行し、
リクエストを行なってください:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `id`
  --> src/lib.rs:61:5
   |
61 |     id: usize,
   |     ^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `thread`
  --> src/lib.rs:62:5
   |
62 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
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
After learning about the `while let` loop in Chapter 18, you might be wondering
why we didn’t write the worker thread code as shown in Listing 20-22.
-->

第18章で`while let`ループを学んだ後で、なぜリスト20-22に示したようにワーカースレッドのコードを記述しなかったのか、
不思議に思っている可能性があります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名: src/lib.rs</span>

```rust,ignore
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<!--
<span class="caption">Listing 20-22: An alternative implementation of
`Worker::new` using `while let`</span>
-->

<span class="caption">リスト20-22: `while let`を使用したもう1つの`Worker::new`の実装</span>

<!--
This code compiles and runs but doesn’t result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. At compile time, the borrow checker can then enforce the rule
that a resource guarded by a `Mutex` cannot be accessed unless we hold the
lock. But this implementation can also result in the lock being held longer
than intended if we don’t think carefully about the lifetime of the
`MutexGuard<T>`. Because the values in the `while` expression remain in scope
for the duration of the block, the lock remains held for the duration of the
call to `job.call_box()`, meaning other workers cannot receive jobs.
-->

このコードはコンパイルでき、動きますが、望み通りのスレッドの振る舞いにはなりません:
遅いリクエストがそれでも、他のリクエストが処理されるのを待機させてしまうのです。理由はどこか捉えがたいものです:
`Mutex`構造体には公開の`unlock`メソッドがありません。ロックの所有権が、
`lock`メソッドが返す`LockResult<MutexGuard<T>>`内の`MutexGuard<T>`のライフタイムに基づくからです。
コンパイル時には、ロックを保持していない限り、借用チェッカーはそうしたら、`Mutex`に保護されるリソースにはアクセスできないという規則を強制できます。
しかし、この実装は、`MutexGuard<T>`のライフタイムについて熟考しなければ、
意図したよりもロックが長い間保持される結果になり得ます。`while`式の値がブロックの間中スコープに残り続けるので、
ロックは`job.call_box`の呼び出し中保持されたままになり、つまり、他のワーカーが仕事を受け取れなくなるのです。

<!--
By using `loop` instead and acquiring the lock and a job within the block
rather than outside it, the `MutexGuard` returned from the `lock` method is
dropped as soon as the `let job` statement ends. This ensures that the lock is
held during the call to `recv`, but it is released before the call to
`job.call_box()`, allowing multiple requests to be serviced concurrently.
-->

代わりに`loop`を使用し、ロックと仕事をブロックの外ではなく、内側で獲得することで、
`lock`メソッドが返す`MutexGuard`は`let job`文が終わると同時にドロップされます。
これにより、複数のリクエストを並行で提供し、ロックは`recv`の呼び出しの間は保持されるけれども、
`job.call_box`の呼び出しの前には解放されることを保証します。
