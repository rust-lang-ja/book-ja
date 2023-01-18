<!--
## Graceful Shutdown and Cleanup
-->

## 正常なシャットダウンと片付け

<!--
The code in Listing 20-21 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we’re not using in a direct way that reminds us
we’re not cleaning up anything. When we use the less elegant <span
class="keystroke">ctrl-c</span> method to halt the main thread, all other
threads are stopped immediately as well, even if they’re in the middle of
serving a request.
-->

リスト 20-21 のコードは、意図した通り、スレッドプールの使用を通してリクエストに非同期に応答できます。
直接使用していない`workers`、`id`、`thread`フィールドについて警告が出ます。この警告は、現在のコードは何も片付けていないことを思い出させてくれます。
優美さに欠ける<span class="keystroke">ctrl-c</span>を使用してメインスレッドを停止させる方法を使用すると、
リクエストの処理中であっても、他のスレッドも停止します。

<!--
Now we’ll implement the `Drop` trait to call `join` on each of the threads in
the pool so they can finish the requests they’re working on before closing.
Then we’ll implement a way to tell the threads they should stop accepting new
requests and shut down. To see this code in action, we’ll modify our server to
accept only two requests before gracefully shutting down its thread pool.
-->

では、閉じる前に取り掛かっているリクエストを完了できるように、プールの各スレッドに対して`join`を呼び出す`Drop`トレイトを実装します。
そして、スレッドに新しいリクエストの受付を停止し、終了するように教える方法を実装します。
このコードが動いているのを確かめるために、サーバを変更して正常にスレッドプールを終了する前に 2 つしかリクエストを受け付けないようにします。

<!--
### Implementing the `Drop` Trait on `ThreadPool`
-->

### `ThreadPool`に`Drop`トレイトを実装する

<!--
Let’s start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 20-23 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet.
-->

スレッドプールに`Drop`を実装するところから始めましょう。プールがドロップされると、
スレッドは全て join して、作業を完了するのを確かめるべきです。リスト 20-23 は、`Drop`実装の最初の試みを表示しています;
このコードはまだ完全には動きません。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // ワーカー{}を終了します
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

<!--
<span class="caption">Listing 20-23: Joining each thread when the thread pool
goes out of scope</span>
-->

<span class="caption">リスト 20-23: スレッドプールがスコープを抜けた時にスレッドを join させる</span>

<!--
First, we loop through each of the thread pool `workers`. We use `&mut` for
this because `self` is a mutable reference, and we also need to be able to
mutate `worker`. For each worker, we print a message saying that this
particular worker is shutting down, and then we call `join` on that worker’s
thread. If the call to `join` fails, we use `unwrap` to make Rust panic and go
into an ungraceful shutdown.
-->

まず、スレッドプール`workers`それぞれを走査します。`self`は可変参照であり、`worker`を可変化できる必要もあるので、
これには`&mut`を使用しています。ワーカーそれぞれに対して、特定のワーカーを終了する旨のメッセージを出力し、
それから`join`をワーカースレッドに対して呼び出しています。`join`の呼び出しが失敗したら、
`unwrap`を使用して Rust をパニックさせ、正常でないシャットダウンに移行します。

<!--
Here is the error we get when we compile this code:
-->

こちらが、このコードをコンパイルする際に出るエラーです：

```text
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

<!--
The error tells us we can’t call `join` because we only have a mutable borrow
of each `worker` and `join` takes ownership of its argument. To solve this
issue, we need to move the thread out of the `Worker` instance that owns
`thread` so `join` can consume the thread. We did this in Listing 17-15: if
`Worker` holds an `Option<thread::JoinHandle<()>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a
`Worker` we’ll replace `Some` with `None` so the worker doesn’t have a
thread to run.
-->

各`worker`の可変参照しかなく、`join`は引数の所有権を奪うためにこのエラーは`join`を呼び出せないと教えてくれています。
この問題を解決するには、`join`がスレッドを消費できるように、`thread`を所有する`Worker`インスタンスからスレッドをムーブする必要があります。
これをリスト 17-15 では行いました：`Worker`が代わりに`Option<thread::JoinHandle<()>>`を保持していれば、
`Option`に対して`take`メソッドを呼び出し、`Some`列挙子から値をムーブし、その場所に`None`列挙子を残すことができます。
言い換えれば、実行中の`Worker`には`thread`に`Some`列挙子があり、`Worker`を片付けたい時には、
ワーカーが実行するスレッドがないように`Some`を`None`で置き換えるのです。

<!--
So we know we want to update the definition of `Worker` like this:
-->

従って、`Worker`の定義を以下のように更新したいことがわかります：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# use std::thread;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

<!--
Now let’s lean on the compiler to find the other places that need to change.
Checking this code, we get two errors:
-->

さて、コンパイラを頼りにして他に変更する必要がある箇所を探しましょう。このコードをチェックすると、
2 つのエラーが出ます：

```text
error[E0599]: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
  --> src/lib.rs:65:27
   |
65 |             worker.thread.join().unwrap();
   |                           ^^^^

error[E0308]: mismatched types
  --> src/lib.rs:89:13
   |
89 |             thread,
   |             ^^^^^^
   |             |
   |             expected enum `std::option::Option`, found struct
   `std::thread::JoinHandle`
   |             help: try using a variant of the expected type: `Some(thread)`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

<!--
Let’s address the second error, which points to the code at the end of
`Worker::new`; we need to wrap the `thread` value in `Some` when we create a
new `Worker`. Make the following changes to fix this error:
-->

2 番目のエラーを扱いましょう。これは、`Worker::new`の最後のコードを指しています; 新しい`Worker`を作成する際に、
`Some`に`thread`の値を包む必要があります。このエラーを修正するために以下の変更を行なってください：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<!--
The first error is in our `Drop` implementation. We mentioned earlier that we
intended to call `take` on the `Option` value to move `thread` out of `worker`.
The following changes will do so:
-->

最初のエラーは`Drop`実装内にあります。先ほど、`Option`値に対して`take`を呼び出し、
`thread`を`worker`からムーブする意図があることに触れました。以下の変更がそれを行います：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

<!--
As discussed in Chapter 17, the `take` method on `Option` takes the `Some`
variant out and leaves `None` in its place. We’re using `if let` to destructure
the `Some` and get the thread; then we call `join` on the thread. If a worker’s
thread is already `None`, we know that worker has already had its thread
cleaned up, so nothing happens in that case.
-->

第 17 章で議論したように、`Option`の`take`メソッドは、`Some`列挙子を取り出し、その箇所に`None`を残します。
`if let`を使用して`Some`を分配し、スレッドを得ています; そして、スレッドに対して`join`を呼び出します。
ワーカーのスレッドが既に`None`なら、ワーカーはスレッドを既に片付け済みであることがわかるので、
その場合には何も起きません。

<!--
### Signaling to the Threads to Stop Listening for Jobs
-->

### スレッドに仕事をリッスンするのを止めるよう通知する

<!--
With all the changes we’ve made, our code compiles without any warnings. But
the bad news is this code doesn’t function the way we want it to yet. The key
is the logic in the closures run by the threads of the `Worker` instances: at
the moment, we call `join`, but that won’t shut down the threads because they
`loop` forever looking for jobs. If we try to drop our `ThreadPool` with our
current implementation of `drop`, the main thread will block forever waiting
for the first thread to finish.
-->

これらの変更によって、コードは警告なしでコンパイルできます。ですが悪い知らせは、このコードが期待したようにはまだ機能しないことです。
鍵は、`Worker`インスタンスのスレッドで実行されるクロージャのロジックです：現時点で`join`を呼び出していますが、
仕事を求めて永遠に`loop`するので、スレッドを終了しません。現在の`drop`の実装で`ThreadPool`をドロップしようとしたら、
最初のスレッドが完了するのを待機してメインスレッドは永遠にブロックされるでしょう。

<!--
To fix this problem, we’ll modify the threads so they listen for either a `Job`
to run or a signal that they should stop listening and exit the infinite loop.
Instead of `Job` instances, our channel will send one of these two enum
variants.
-->

この問題を修正するには、スレッドが、実行すべき`Job`か、リッスンをやめて無限ループを抜ける通知をリッスンするように、
変更します。`Job`インスタンスの代わりに、チャンネルはこれら 2 つの enum 列挙子の一方を送信します。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# struct Job;
enum Message {
    NewJob(Job),
    Terminate,
}
```

<!--
This `Message` enum will either be a `NewJob` variant that holds the `Job` the
thread should run, or it will be a `Terminate` variant that will cause the
thread to exit its loop and stop.
-->

この`Message` enum はスレッドが実行すべき`Job`を保持する`NewJob`列挙子か、スレッドをループから抜けさせ、
停止させる`Terminate`列挙子のどちらかになります。

<!--
We need to adjust the channel to use values of type `Message` rather than type
`Job`, as shown in Listing 20-24.
-->

チャンネルを調整し、型`Job`ではなく、型`Message`を使用するようにする必要があります。リスト 20-24 のようにですね。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// --snip--

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        // ワーカー{}は停止するよう指示された
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<!--
<span class="caption">Listing 20-24: Sending and receiving `Message` values and
exiting the loop if a `Worker` receives `Message::Terminate`</span>
-->

<span class="caption">リスト 20-24: `Message`値を送受信し、`Worker`が`Message::Terminate`を受け取ったら、ループを抜ける</span>

<!--
To incorporate the `Message` enum, we need to change `Job` to `Message` in two
places: the definition of `ThreadPool` and the signature of `Worker::new`. The
`execute` method of `ThreadPool` needs to send jobs wrapped in the
`Message::NewJob` variant. Then, in `Worker::new` where a `Message` is received
from the channel, the job will be processed if the `NewJob` variant is
received, and the thread will break out of the loop if the `Terminate` variant
is received.
-->

`Message` enum を具体化するために、2 箇所で`Job`を`Message`に変更する必要があります：
`ThreadPool`の定義と`Worker::new`のシグニチャです。`ThreadPool`の`execute`メソッドは、
仕事を`Message::NewJob`列挙子に包んで送信する必要があります。それから、
`Message`がチャンネルから受け取られる`Worker::new`で、`NewJob`列挙子が受け取られたら、
仕事が処理され、`Terminate`列挙子が受け取られたら、スレッドはループを抜けます。

<!--
With these changes, the code will compile and continue to function in the same
way as it did after Listing 20-21. But we’ll get a warning because we aren’t
creating any messages of the `Terminate` variety. Let’s fix this warning by
changing our `Drop` implementation to look like Listing 20-25.
-->

これらの変更と共に、コードはコンパイルでき、リスト 20-21 の後と同じように機能し続けます。ですが、
`Terminate`のメッセージを何も生成していないので、警告が出るでしょう。
`Drop`実装をリスト 20-25 のような見た目に変更してこの警告を修正しましょう。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // 全ワーカーを閉じます
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            // ワーカー{}を閉じます
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

<!--
<span class="caption">Listing 20-25: Sending `Message::Terminate` to the
workers before calling `join` on each worker thread</span>
-->

<span class="caption">リスト 20-25: 各ワーカースレッドに対して`join`を呼び出す前にワーカーに`Message::Terminate`を送信する</span>

<!--
We’re now iterating over the workers twice: once to send one `Terminate`
message for each worker and once to call `join` on each worker’s thread. If we
tried to send a message and `join` immediately in the same loop, we couldn’t
guarantee that the worker in the current iteration would be the one to get the
message from the channel.
-->

今では、ワーカーを 2 回走査しています：各ワーカーに`Terminate`メッセージを送信するために 1 回と、
各ワーカースレッドに`join`を呼び出すために 1 回です。メッセージ送信と`join`を同じループで即座に行おうとすると、
現在の繰り返しのワーカーがチャンネルからメッセージを受け取っているものであるか保証できなくなってしまいます。

<!--
To better understand why we need two separate loops, imagine a scenario with
two workers. If we used a single loop to iterate through each worker, on the
first iteration a terminate message would be sent down the channel and `join`
called on the first worker’s thread. If that first worker was busy processing a
request at that moment, the second worker would pick up the terminate message
from the channel and shut down. We would be left waiting on the first worker to
shut down, but it never would because the second thread picked up the terminate
message. Deadlock!
-->

2 つの個別のループが必要な理由をよりよく理解するために、2 つのワーカーがある筋書きを想像してください。
単独のループで各ワーカーを走査すると、最初の繰り返しでチャンネルに停止メッセージが送信され、
`join`が最初のワーカースレッドで呼び出されます。その最初のワーカーが現在、リクエストの処理で忙しければ、
2 番目のワーカーがチャンネルから停止メッセージを受け取り、閉じます。最初のワーカーの終了待ちをしたままですが、
2 番目のスレッドが停止メッセージを拾ってしまったので、終了することは絶対にありません。デッドロックです！

<!--
To prevent this scenario, we first put all of our `Terminate` messages on the
channel in one loop; then we join on all the threads in another loop. Each
worker will stop receiving requests on the channel once it gets a terminate
message. So, we can be sure that if we send the same number of terminate
messages as there are workers, each worker will receive a terminate message
before `join` is called on its thread.
-->

この筋書きを回避するために、1 つのループでまず、チャンネルに対して全ての`Terminate`メッセージを送信します;
そして、別のループで全スレッドの join を待ちます。一旦停止メッセージを受け取ったら、各ワーカーはチャンネルからのリクエストの受付をやめます。
故に、存在するワーカーと同じ数だけ停止メッセージを送れば、`join`がスレッドに対して呼び出される前に、
停止メッセージを各ワーカーが受け取ると確信できるわけです。

<!--
To see this code in action, let’s modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 20-26.
-->

このコードが動いているところを確認するために、`main`を変更してサーバを正常に閉じる前に 2 つしかリクエストを受け付けないようにしましょう。
リスト 20-26 のようにですね。

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">ファイル名：src/bin/main.rs</span>

```rust,ignore
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
```

<!--
<span class="caption">Listing 20-26: Shut down the server after serving two
requests by exiting the loop</span>
-->

<span class="caption">リスト 20-26: ループを抜けることで、2 つのリクエストを処理した後にサーバを閉じる</span>

<!--
You wouldn’t want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.
-->

現実世界の Web サーバには、たった 2 つリクエストを受け付けた後にシャットダウンしてほしくはないでしょう。
このコードは、単に正常なシャットダウンとクリーンアップが正しく機能することを示すだけです。

<!--
The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.
-->

`take`メソッドは、`Iterator`トレイトで定義されていて、最大でも繰り返しを最初の 2 つの要素だけに制限します。
`ThreadPool`は`main`の末端でスコープを抜け、`drop`実装が実行されます。

<!--
Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output similar to this:
-->

`cargo run`でサーバを開始し、3 つリクエストを行なってください。3 番目のリクエストはエラーになるはずで、
端末にはこのような出力が目撃できるはずです：

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

<!--
You might see a different ordering of workers and messages printed. We can see
how this code works from the messages: workers 0 and 3 got the first two
requests, and then on the third request, the server stopped accepting
connections. When the `ThreadPool` goes out of scope at the end of `main`, its
`Drop` implementation kicks in, and the pool tells all workers to terminate.
The workers each print a message when they see the terminate message, and then
the thread pool calls `join` to shut down each worker thread.
-->

ワーカーとメッセージの順番は異なる可能性があります。どうやってこのコードが動くのかメッセージからわかります：
ワーカー0 と 3 が最初の 2 つのリクエストを受け付け、そして 3 番目のリクエストではサーバは接続の受け入れをやめます。
`main`の最後で`ThreadPool`がスコープを抜ける際、`Drop`実装が割り込み、プールが全ワーカーに停止するよう指示します。
ワーカーはそれぞれ、停止メッセージを確認した時にメッセージを出力し、それからスレッドプールは各ワーカースレッドを閉じる`join`を呼び出します。

<!--
Notice one interesting aspect of this particular execution: the `ThreadPool`
sent the terminate messages down the channel, and before any worker received
the messages, we tried to join worker 0. Worker 0 had not yet received the
terminate message, so the main thread blocked waiting for worker 0 to finish.
In the meantime, each of the workers received the termination messages. When
worker 0 finished, the main thread waited for the rest of the workers to
finish. At that point, they had all received the termination message and were
able to shut down.
-->

この特定の実行のある面白い側面に注目してください：`ThreadPool`はチャンネルに停止メッセージを送信しますが、
どのワーカーがそのメッセージを受け取るよりも前に、ワーカー0 の join を試みています。ワーカー0 はまだ停止メッセージを受け取っていなかったので、
メインスレッドはワーカー0 が完了するまで待機してブロックされます。その間に、各ワーカーは停止メッセージを受け取ります。
ワーカー0 が完了したら、メインスレッドは残りのワーカーが完了するのを待機します。その時点で全ワーカーは停止メッセージを受け取った後で、
閉じることができたのです。

<!--
Congrats! We’ve now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We’re able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.
-->

おめでとうございます！プロジェクトを完成させました; スレッドプールを使用して非同期に応答する基本的な Web サーバができました。
サーバの正常なシャットダウンを行うことができ、プールの全スレッドを片付けます。

<!--
Here’s the full code for reference:
-->

参考までに、こちらが全コードです：

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">ファイル名：src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // 閉じます
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

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

     let mut file = File::open(filename).unwrap();
     let mut contents = String::new();

     file.read_to_string(&mut contents).unwrap();

     let response = format!("{}{}", status_line, contents);

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();
}
```

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
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

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<!--
We could do more here! If you want to continue enhancing this project, here are
some ideas:
-->

ここでできることはまだあるでしょう！よりこのプロジェクトを改善したいのなら、こちらがアイディアの一部です：

<!--
* Add more documentation to `ThreadPool` and its public methods.
* Add tests of the library’s functionality.
* Change calls to `unwrap` to more robust error handling.
* Use `ThreadPool` to perform some task other than serving web requests.
* Find a thread pool crate on *https://crates.io/* and implement a similar web
server using the crate instead. Then compare its API and robustness to the
thread pool we implemented.
-->

* `ThreadPool`とその公開メソッドにもっとドキュメンテーションを追加する。
* ライブラリの機能のテストを追加する。
* `unwrap`の呼び出しをもっと頑健なエラー処理に変更する。
* `ThreadPool`を使用して Web リクエスト以外のなんらかの作業を行う。
* *https://crates.io* でスレッドプールのクレートを探して、そのクレートを代わりに使用して似た Web サーバを実装する。
  そして、API と頑健性を我々が実装したものと比較する。

<!--
## Summary
-->

## まとめ

<!--
Well done! You’ve made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You’re now ready to implement your own Rust
projects and help with other peoples’ projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
-->

よくやりました！本の最後に到達しました！Rust のツアーに参加していただき、感謝の辞を述べたいです。
もう、ご自身の Rust プロジェクトや他の方のプロジェクトのお手伝いをする準備ができています。
あなたがこれからの Rust の旅で遭遇する、あらゆる困難の手助けを是非とも行いたい Rustacean たちの温かいコミュニティがあることを心に留めておいてくださいね。
