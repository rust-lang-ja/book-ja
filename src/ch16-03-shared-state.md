<!--
## Shared-State Concurrency
-->

## 状態共有並行性

<!--
Message passing is a fine way of handling concurrency, but it’s not the only
one. Consider this part of the slogan from the Go language documentation again:
“communicate by sharing memory.”
-->

メッセージ受け渡しは、並行性を扱う素晴らしい方法ですが、唯一の方法ではありません。
Go 言語ドキュメンテーションのスローガンのこの部分を再び考えてください：
「メモリを共有することでやり取りする。」

<!--
What would communicating by sharing memory look like? In addition, why would
message-passing enthusiasts not use it and do the opposite instead?
-->

メモリを共有することでやり取りするとはどんな感じなのでしょうか？さらに、
なぜメッセージ受け渡しに熱狂的な人は、それを使わず、代わりに全く反対のことをするのでしょうか？

<!--
In a way, channels in any programming language are similar to single ownership,
because once you transfer a value down a channel, you should no longer use that
value. Shared memory concurrency is like multiple ownership: multiple threads
can access the same memory location at the same time. As you saw in Chapter 15,
where smart pointers made multiple ownership possible, multiple ownership can
add complexity because these different owners need managing. Rust’s type system
and ownership rules greatly assist in getting this management correct. For an
example, let’s look at mutexes, one of the more common concurrency primitives
for shared memory.
-->

ある意味では、どんなプログラミング言語のチャンネルも単独の所有権に類似しています。
一旦チャンネルに値を転送したら、その値は最早使用することがないからです。
メモリ共有並行性は、複数の所有権に似ています：複数のスレッドが同時に同じメモリ位置にアクセスできるのです。
第 15 章でスマートポインタが複数の所有権を可能にするのを目の当たりにしたように、
異なる所有者を管理する必要があるので、複数の所有権は複雑度を増させます。
Rust の型システムと所有権規則は、この管理を正しく行う大きな助けになります。
例として、メモリ共有を行うより一般的な並行性の基本型の一つであるミューテックスを見てみましょう。

<!--
### Using Mutexes to Allow Access to Data from One Thread at a Time
-->

### ミューテックスを使用して一度に 1 つのスレッドからデータにアクセスすることを許可する

<!--
1 行目、as in，が肝だが、in の後は普通名詞に相当するものが来るはずだが、文になっている
-->

<!--
*Mutex* is an abbreviation for “mutual exclusion,” as in, a mutex allows only
one thread to access some data at any given time. To access the data in a
mutex, a thread must first signal that it wants access by asking to acquire the
mutex's *lock*. The lock is a data structure that is part of the mutex that
keeps track of who currently has exclusive access to the data. Therefore, the
mutex is described as *guarding* the data it holds via the locking system.
-->

ミューテックスは、どんな時も 1 つのスレッドにしかなんらかのデータへのアクセスを許可しないというように、
"mutual exclusion"(相互排他) の省略形です。ミューテックスにあるデータにアクセスするには、
ミューテックスのロックを所望することでアクセスしたいことをまず、スレッドは通知しなければなりません。
ロックとは、現在誰がデータへの排他的アクセスを行なっているかを追跡するミューテックスの一部をなすデータ構造です。
故に、ミューテックスはロックシステム経由で保持しているデータを*死守する*(guarding) と解説されます。

<!--
Mutexes have a reputation for being difficult to use because you have to
remember two rules:
-->

ミューテックスは、2 つの規則を覚えておく必要があるため、難しいという評判があります：

<!--
* You must attempt to acquire the lock before using the data.
* When you’re done with the data that the mutex guards, you must unlock the
data so other threads can acquire the lock.
-->

* データを使用する前にロックの獲得を試みなければならない。
* ミューテックスが死守しているデータの使用が終わったら、他のスレッドがロックを獲得できるように、
  データをアンロックしなければならない。

<!--
For a real-world metaphor for a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist can speak, they have to
ask or signal that they want to use the microphone. When they get the
microphone, they can talk for as long as they want to and then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they’re finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel won’t work
as planned!
-->

ミューテックスを現実世界の物で例えるなら、マイクが 1 つしかない会議のパネルディスカッションを思い浮かべてください。
パネリストが発言できる前に、マイクを使用したいと申し出たり、通知しなければなりません。マイクを受け取ったら、
話したいだけ話し、それから次に発言を申し出たパネリストにマイクを手渡します。パネリストが発言し終わった時に、
マイクを手渡すのを忘れていたら、誰も他の人は発言できません。共有されているマイクの管理がうまくいかなければ、
パネルは予定通りに機能しないでしょう！

<!--
Management of mutexes can be incredibly tricky to get right, which is why so
many people are enthusiastic about channels. However, thanks to Rust’s type
system and ownership rules, you can’t get locking and unlocking wrong.
-->

ミューテックスの管理は、正しく行うのに著しく技巧を要することがあるので、多くの人がチャンネルに熱狂的になるわけです。
しかしながら、Rust の型システムと所有権規則のおかげで、ロックとアンロックをおかしくすることはありません。

<!--
#### The API of `Mutex<T>`
-->

#### `Mutex<T>`の API

<!--
As an example of how to use a mutex, let’s start by using a mutex in a
single-threaded context, as shown in Listing 16-12:
-->

ミューテックスの使用方法の例として、ミューテックスをシングルスレッドの文脈で使うことから始めましょう。
リスト 16-12 のようにですね：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

<!--
<span class="caption">Listing 16-12: Exploring the API of `Mutex<T>` in a
single-threaded context for simplicity</span>
-->

<span class="caption">リスト 16-12: 簡潔性のために`Mutex<T>`の API をシングルスレッドの文脈で探究する</span>

<!--
As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so it can’t do any work until
it’s our turn to have the lock.
-->

多くの型同様、`new`という関連関数を使用して`Mutex<T>`を生成します。ミューテックス内部のデータにアクセスするには、
`lock`メソッドを使用してロックを獲得します。この呼び出しは、現在のスレッドをブロックするので、
ロックを得られる順番が来るまで何も作業はできません。

<!--
The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we’ve chosen to
`unwrap` and have this thread panic if we’re in that situation.
-->

ロックを保持している他のスレッドがパニックしたら、`lock`の呼び出しは失敗するでしょう。その場合、
誰もロックを取得することは叶わないので、`unwrap`すると決定し、そのような状況になったら、
このスレッドをパニックさせます。

<!--
After we’ve acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using the value in `m`: `Mutex<i32>` is not an
`i32`, so we *must* acquire the lock to be able to use the `i32` value. We
can’t forget; the type system won’t let us access the inner `i32` otherwise.
-->

ロックを獲得した後、今回の場合、`num`と名付けられていますが、戻り値を中に入っているデータへの可変参照として扱うことができます。
型システムにより、`m`の値を使用する前にロックを獲得していることが確認されます：`Mutex<i32>`は`i32`ではないので、
`i32`を使用できるようにするには、ロックを獲得し*なければならない*のです。忘れることはあり得ません;
型システムにより、それ以外の場合に内部の`i32`にアクセスすることは許されません。

<!--
As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call
to `lock` *returns* a smart pointer called `MutexGuard`. This smart pointer
implements `Deref` to point at our inner data; the smart pointer also has a
`Drop` implementation that releases the lock automatically when a `MutexGuard`
goes out of scope, which happens at the end of the inner scope in Listing
16-12. As a result, we don’t risk forgetting to release the lock and blocking
the mutex from being used by other threads because the lock release happens
automatically.
-->

お察しかもしれませんが、`Mutex<T>`はスマートポインタです。より正確を期すなら、
`lock`の呼び出しが`MutexGuard`というスマートポインタを*返却*します。このスマートポインタが、
内部のデータを指す`Deref`を実装しています; このスマートポインタはさらに`MutexGuard`がスコープを外れた時に、
自動的にロックを解除する`Drop`実装もしていて、これがリスト 16-12 の内部スコープの終わりで発生します。
結果として、ロックの解除が自動的に行われるので、ロックの解除を忘れ、
ミューテックスが他のスレッドで使用されるのを阻害するリスクを負いません。

<!--
After dropping the lock, we can print the mutex value and see that we were able
to change the inner `i32` to 6.
-->

ロックをドロップした後、ミューテックスの値を出力し、内部の`i32`の値を 6 に変更できたことが確かめられるのです。

<!--
#### Sharing a `Mutex<T>` Between Multiple Threads
-->

#### 複数のスレッド間で`Mutex<T>`を共有する

<!--
Now, let’s try to share a value between multiple threads using `Mutex<T>`.
We’ll spin up 10 threads and have them each increment a counter value by 1, so
the counter goes from 0 to 10. Note that the next few examples will have
compiler errors, and we’ll use those errors to learn more about using
`Mutex<T>` and how Rust helps us use it correctly. Listing 16-13 has our
starting example:
-->

さて、`Mutex<T>`を使って複数のスレッド間で値を共有してみましょう。10 個のスレッドを立ち上げ、
各々カウンタの値を 1 ずつインクリメントさせるので、カウンタは 0 から 10 まで上がります。
以下の数例は、コンパイルエラーになることに注意し、そのエラーを使用して`Mutex<T>`の使用法と、
コンパイラがそれを正しく活用する手助けをしてくれる方法について学びます。リスト 16-13 が最初の例です：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<!--
<span class="caption">Listing 16-13: Ten threads each increment a counter
guarded by a `Mutex<T>`</span>
-->

<span class="caption">リスト 16-13: `Mutex<T>`により死守されているカウンタを 10 個のスレッドがそれぞれインクリメントする</span>

<!--
We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we
did in Listing 16-12. Next, we create 10 threads by mapping over a range
of numbers. We use `thread::spawn` and give all the threads the same closure,
one that moves the counter into the thread, acquires a lock on the `Mutex<T>`
by calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so another thread can acquire it.
-->

リスト 16-12 のように、`counter`変数を生成して`Mutex<T>`の内部に`i32`を保持しています。
次に、数値の範囲をマッピングして 10 個のスレッドを生成しています。`thread::spawn`を使用して、
全スレッドに同じクロージャを与えています。このクロージャは、スレッド内にカウンタをムーブし、
`lock`メソッドを呼ぶことで`Mutex<T>`のロックを獲得し、それからミューテックスの値に 1 を足します。
スレッドがクロージャを実行し終わったら、`num`はスコープ外に出てロックを解除するので、
他のスレッドが獲得できるわけです。

<!--
In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.
-->

メインスレッドで全ての join ハンドルを収集します。それからリスト 16-2 のように、各々に対して`join`を呼び出し、
全スレッドが終了するのを確かめています。その時点で、メインスレッドはロックを獲得し、このプログラムの結果を出力します。

<!--
We hinted that this example wouldn’t compile. Now let’s find out why!
-->

この例はコンパイルできないでしょうと仄めかしました。では、理由を探りましょう！

```text
error[E0382]: capture of moved value: `counter`
(エラー: ムーブされた値をキャプチャしています：`counter`)
  --> src/main.rs:10:27
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:21:29
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
(エラー: 前述の 2 つのエラーによりアボート)
```

<!--
The error message states that the `counter` value is moved into the closure and
then captured when we call `lock`. That description sounds like what we wanted,
but it’s not allowed!
-->

エラーメッセージは、`counter`値はクロージャにムーブされ、それから`lock`を呼び出したときにキャプチャされていると述べています。
その説明は、所望した動作のように聞こえますが、許可されていないのです！

<!--
Let’s figure this out by simplifying the program. Instead of making 10 threads
in a `for` loop, let’s just make two threads without a loop and see what
happens. Replace the first `for` loop in Listing 16-13 with this code instead:
-->

プログラムを単純化してこれを理解しましょう。`for`ループで 10 個スレッドを生成する代わりに、
ループなしで 2 つのスレッドを作るだけにしてどうなるか確認しましょう。
リスト 16-13 の最初の`for`ループを代わりにこのコードと置き換えてください：

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<!--
We make two threads and change the variable names used with the second thread
to `handle2` and `num2`. When we run the code this time, compiling gives us the
following:
-->

2 つのスレッドを生成し、2 番目のスレッドの変数名を`handle2`と`num2`に変更しています。
今回このコードを走らせると、コンパイラは以下の出力をします：

```text
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:16:24
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:26:29
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

<!--
Aha! The first error message indicates that `counter` is moved into the closure
for the thread associated with `handle`. That move is preventing us from
capturing `counter` when we try to call `lock` on it and store the result in
`num2` in the second thread! So Rust is telling us that we can’t move ownership
of `counter` into multiple threads. This was hard to see earlier because our
threads were in a loop, and Rust can’t point to different threads in different
iterations of the loop. Let’s fix the compiler error with a multiple-ownership
method we discussed in Chapter 15.
-->

なるほど！最初のエラーメッセージは、`handle`に紐づけられたスレッドのクロージャに`counter`がムーブされていることを示唆しています。
そのムーブにより、それに対して`lock`を呼び出し、結果を 2 番目のスレッドの`num2`に保持しようとした時に、
`counter`をキャプチャすることを妨げています！ゆえに、コンパイラは、`counter`の所有権を複数のスレッドに移すことはできないと教えてくれています。
これは、以前では確認しづらかったことです。なぜなら、スレッドはループの中にあり、
ループの違う繰り返しにある違うスレッドをコンパイラは指し示せないからです。
第 15 章で議論した複数所有権メソッドによりコンパイルエラーを修正しましょう。

<!--
#### Multiple Ownership with Multiple Threads
-->

#### 複数のスレッドで複数の所有権

<!--
In Chapter 15, we gave a value multiple owners by using the smart pointer
`Rc<T>` to create a reference counted value. Let’s do the same here and see
what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread. Now that we’ve seen the
errors, we’ll also switch back to using the `for` loop, and we’ll keep the
`move` keyword with the closure.
-->

第 15 章で、スマートポインタの`Rc<T>`を使用して参照カウントの値を作ることで、1 つの値に複数の所有者を与えました。
同じことをここでもして、どうなるか見ましょう。リスト 16-14 で`Rc<T>`に`Mutex<T>`を包含し、
所有権をスレッドに移す前に`Rc<T>`をクローンします。今やエラーを確認したので、
`for`ループの使用に立ち戻り、クロージャに`move`キーワードを使用し続けます。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<!--
<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>
-->

<span class="caption">リスト 16-14: `Rc<T>`を使用して複数のスレッドに`Mutex<T>`を所有させようとする</span>

<!--
Once again, we compile and get... different errors! The compiler is teaching us
a lot.
-->

再三、コンパイルし……別のエラーが出ました！コンパイラはいろんなことを教えてくれています。

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied in `[closure@src/main.rs:11:36:
15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
(エラー: トレイト境界`std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send`は`[closure@src/main.rs:11:36:15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`で満たされていません)
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
                          (`std::rc::Rc<std::sync::Mutex<i32>>`は、スレッド間で安全に送信できません)
   |
   = help: within `[closure@src/main.rs:11:36: 15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is
not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
     (ヘルプ: `[closure@src/main.rs:11:36 15:10
     counter:std::rc::Rc<std::sync::Mutex<i32>>]`内でトレイト`std::marker::Send`は、
     `std::rc::Rc<std::sync::Mutex<i32>>`に対して実装されていません)
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
     (注釈: 型`[closure@src/main.rs:11:36 15:10
     counter:std::rc::Rc<std::sync::Mutex<i32>>]`内に出現するので必要です)
   = note: required by `std::thread::spawn`
     (注釈: `std::thread::spawn`により必要とされています)
```

<!--
Wow, that error message is very wordy! Here are some important parts to focus
on: the first inline error says `` `std::rc::Rc<std::sync::Mutex<i32>>` cannot
be sent between threads safely ``. The reason for this is in the next important
part to focus on, the error message. The distilled error message says `` the
trait bound `Send` is not satisfied ``. We’ll talk about `Send` in the next
section: it’s one of the traits that ensures the types we use with threads are
meant for use in concurrent situations.
-->

おお、このエラーメッセージはとても長ったらしいですね！こちらが、注目すべき重要な部分です：
最初のインラインエラーは`` `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent
between threads safely``と述べています。この理由は、エラーメッセージの次に注目すべき重要な部分にあります。
洗練されたエラーメッセージは、`` the trait bound `Send` is not satisfied``と述べています。
`Send`については、次の節で語ります：
スレッドとともに使用している型が並行な場面で使われることを意図したものであることを保証するトレイトの 1 つです。

<!--
Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we’re done
with it. What we need is a type exactly like `Rc<T>` but one that makes changes
to the reference count in a thread-safe way.
-->

残念ながら、`Rc<T>`はスレッド間で共有するには安全ではないのです。`Rc<T>`が参照カウントを管理する際、
`clone`が呼び出されるたびにカウントを追加し、クローンがドロップされるたびにカウントを差し引きます。
しかし、並行基本型を使用してカウントの変更が別のスレッドに妨害されないことを確認していないのです。
これは間違ったカウントにつながる可能性があり、今度はメモリリークや、使用し終わる前に値がドロップされることにつながる可能性のある潜在的なバグです。
必要なのは、いかにも`Rc<T>`のようだけれども、参照カウントへの変更をスレッドセーフに行うものです。

<!--
#### Atomic Reference Counting with `Arc<T>`
-->

#### `Arc<T>`で原子的な参照カウント

<!--
Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for `std::sync::atomic` for more details. At this point, you just
need to know that atomics work like primitive types but are safe to share
across threads.
-->

幸いなことに、`Arc<T>`は`Rc<T>`のような並行な状況で安全に使用できる型*です*。
*a*は*atomic*を表し、原子的に参照カウントする型を意味します。アトミックは、
ここでは詳しく講義しない並行性の別の基本型です：詳細は、
`std::sync::atomic`の標準ライブラリドキュメンテーションを参照されたし。現時点では、
アトミックは、基本型のように動くけれども、スレッド間で共有しても安全なことだけ知っていれば良いです。

<!--
You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.
-->

そうしたらあなたは、なぜ全ての基本型がアトミックでなく、標準ライブラリの型も標準で`Arc<T>`を使って実装されていないのか疑問に思う可能性があります。
その理由は、スレッド安全性が、本当に必要な時だけ支払いたいパフォーマンスの犠牲とともに得られるものだからです。
シングルスレッドで値に処理を施すだけなら、アトミックが提供する保証を強制する必要がない方がコードはより速く走るのです。

<!--
Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run:
-->

例に回帰しましょう：`Arc<T>`と`Rc<T>`の API は同じなので、`use`行と`new`の呼び出しと`clone`の呼び出しを変更して、
プログラムを修正します。リスト 16-15 は、ようやくコンパイルでき、動作します：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<!--
<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>
-->

<span class="caption">リスト 16-15: `Arc<T>`を使用して`Mutex<T>`をラップし、所有権を複数のスレッド間で共有できるようにする</span>

<!--
This code will print the following:
-->

このコードは、以下のように出力します：

```text
Result: 10
```

<!--
We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program’s structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.
-->

やりました！0 から 10 まで数え上げました。これは、あまり印象的ではないように思えるかもしれませんが、
本当に`Mutex<T>`とスレッド安全性についていろんなことを教えてくれました。このプログラムの構造を使用して、
カウンタをインクリメントする以上の複雑な処理を行うこともできるでしょう。この手法を使えば、
計算を独立した部分に小分けにし、その部分をスレッドに分割し、それから`Mutex<T>`を使用して、
各スレッドに最終結果を更新させることができます。

<!--
### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
-->

### `RefCell<T>`/`Rc<T>`と`Mutex<T>`/`Arc<T>`の類似性

<!--
You might have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in
Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>`
to mutate contents inside an `Arc<T>`.
-->

`counter`は不変なのに、その内部にある値への可変参照を得ることができたことに気付いたでしょうか;
つまり、`Mutex<T>`は、`Cell`系のように内部可変性を提供するわけです。
第 15 章で`RefCell<T>`を使用して`Rc<T>`の内容を可変化できるようにしたのと同様に、
`Mutex<T>`を使用して`Arc<T>`の内容を可変化しているのです。

<!--
Another detail to note is that Rust can’t protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating *deadlocks*. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.
-->

気付いておくべき別の詳細は、`Mutex<T>`を使用する際にあらゆる種類のロジックエラーからは、
コンパイラは保護してくれないということです。第 15 章で`Rc<T>`は、循環参照を生成してしまうリスクを伴い、
そうすると、2 つの`Rc<T>`の値がお互いを参照し合い、メモリリークを引き起こしてしまうことを思い出してください。
同様に、`Mutex<T>`は*デッドロック*を生成するリスクを伴っています。これは、処理が 2 つのリソースをロックする必要があり、
2 つのスレッドがそれぞれにロックを 1 つ獲得して永久にお互いを待ちあってしまうときに起こります。
デッドロックに興味があるのなら、デッドロックのある Rust プログラムを組んでみてください;
それからどんな言語でもいいので、ミューテックスに対してデッドロックを緩和する方法を調べて、
Rust で是非、それを実装してみてください。`Mutex<T>`と`MutexGuard`に関する標準ライブラリの API ドキュメンテーションは、
役に立つ情報を提供してくれます。

<!--
We’ll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.
-->

`Send`と`Sync`トレイトと、それらを独自の型で使用する方法について語って、この章を締めくくります。
