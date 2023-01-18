<!--
## Extensible Concurrency with the `Sync` and `Send` Traits
-->

## `Sync`と`Send`トレイトで拡張可能な並行性

<!--
Interestingly, the Rust language has *very* few concurrency features. Almost
every concurrency feature we’ve talked about so far in this chapter has been
part of the standard library, not the language. Your options for handling
concurrency are not limited to the language or the standard library; you can
write your own concurrency features or use those written by others.
-->

面白いことに、Rust 言語には、*寡*少な並行性機能があります。この章でここまでに語った並行性機能のほとんどは、
標準ライブラリの一部であり、言語ではありません。並行性を扱う選択肢は、言語や標準ライブラリに制限されません;
独自の並行性機能を書いたり、他人が書いたものを利用したりできるのです。

<!--
However, two concurrency concepts are embedded in the language: the
`std::marker` traits `Sync` and `Send`.
-->

ですが、2 つの並行性概念が言語に埋め込まれています：`std::marker`トレイトの`Sync`と`Send`です。

<!--
### Allowing Transference of Ownership Between Threads with `Send`
-->

### `Send`でスレッド間の所有権の転送を許可する

<!--
最後から 2 行目、single-threaded situations の situations を環境と訳すのが自然なのでそうしている
-->

<!--
The `Send` marker trait indicates that ownership of the type implementing
`Send` can be transferred between threads. Almost every Rust type is `Send`,
but there are some exceptions, including `Rc<T>`: this cannot be `Send` because
if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to
another thread, both threads might update the reference count at the same time.
For this reason, `Rc<T>` is implemented for use in single-threaded situations
where you don’t want to pay the thread-safe performance penalty.
-->

`Send`マーカートレイトは、`Send`を実装した型の所有権をスレッド間で転送できることを示唆します。
Rust のほとんどの型は`Send`ですが、`Rc<T>`を含めて一部例外があります：この型は、`Rc<T>`の値をクローンし、
クローンしたものの所有権を別のスレッドに転送しようとしたら、両方のスレッドが同時に参照カウントを更新できてしまうので、
`Send`になり得ません。このため、`Rc<T>`はスレッド安全性のためのパフォーマンスの犠牲を支払わなくても済む、
シングルスレッド環境で使用するために実装されているわけです。

<!--
Therefore, Rust’s type system and trait bounds ensure that you can never
accidentally send an `Rc<T>` value across threads unsafely. When we tried to do
this in Listing 16-14, we got the error `the trait Send is not implemented for
Rc<Mutex<i32>>`. When we switched to `Arc<T>`, which is `Send`, the code
compiled.
-->

故に、Rust の型システムとトレイト境界により、`Rc<T>`の値を不安全にスレッド間で誤って送信することが絶対ないよう保証してくれるのです。
リスト 16-14 でこれを試みた時には、`the trait Send is not implemented for Rc<Mutex<i32>>`というエラーが出ました。
`Send`の`Arc<T>`に切り替えたら、コードはコンパイルできたわけです。

<!--
Any type composed entirely of `Send` types is automatically marked as `Send` as
well. Almost all primitive types are `Send`, aside from raw pointers, which
we’ll discuss in Chapter 19.
-->

完全に`Send`の型からなる型も全て自動的に`Send`と印付けされます。生ポインタを除くほとんどの基本型も`Send`で、
生ポインタについては第 19 章で議論します。

<!--
### Allowing Access from Multiple Threads with `Sync`
-->

### `Sync`で複数のスレッドからのアクセスを許可する

<!--
The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads. In other words, any type `T` is
`Sync` if `&T` (a reference to `T`) is `Send`, meaning the reference can be
sent safely to another thread. Similar to `Send`, primitive types are `Sync`,
and types composed entirely of types that are `Sync` are also `Sync`.
-->

`Sync`マーカートレイトは、`Sync`を実装した型は、複数のスレッドから参照されても安全であることを示唆します。
言い換えると、`&T`(`T`への参照) が`Send`なら、型`T`は`Sync`であり、参照が他のスレッドに安全に送信できることを意味します。
`Send`同様、基本型は`Sync`であり、`Sync`の型からのみ構成される型もまた`Sync`です。

<!--
The smart pointer `Rc<T>` is also not `Sync` for the same reasons that it’s not
`Send`. The `RefCell<T>` type (which we talked about in Chapter 15) and the
family of related `Cell<T>` types are not `Sync`. The implementation of borrow
checking that `RefCell<T>` does at runtime is not thread-safe. The smart
pointer `Mutex<T>` is `Sync` and can be used to share access with multiple
threads as you saw in the “Sharing a `Mutex<T>` Between Multiple Threads”
section.
-->

`Send`ではなかったのと同じ理由で、スマートポインタの`Rc<T>`もまた`Sync`ではありません。
`RefCell<T>`型 (これについては第 15 章で話しました) と関連する`Cell<T>`系についても`Sync`ではありません。
`RefCell<T>`が実行時に行う借用チェックの実装は、スレッド安全ではないのです。
スマートポインタの`Mutex<T>`は`Sync`で、「複数のスレッド間で`Mutex<T>`を共有する」節で見たように、
複数のスレッドでアクセスを共有するのに使用することができます。

<!--
### Implementing `Send` and `Sync` Manually Is Unsafe
-->

### `Send`と`Sync`を手動で実装するのは非安全である

<!--
Because types that are made up of `Send` and `Sync` traits are automatically
also `Send` and `Sync`, we don’t have to implement those traits manually. As
marker traits, they don’t even have any methods to implement. They’re just
useful for enforcing invariants related to concurrency.
-->

`Send`と`Sync`トレイトから構成される型は自動的に`Send`と`Sync`にもなるので、
それらのトレイトを手動で実装する必要はありません。マーカートレイトとして、
実装すべきメソッドさえも何もありません。並行性に関連する不変条件を強制することに役立つだけなのです。

<!--
Manually implementing these traits involves implementing unsafe Rust code.
We’ll talk about using unsafe Rust code in Chapter 19; for now, the important
information is that building new concurrent types not made up of `Send` and
`Sync` parts requires careful thought to uphold the safety guarantees.
[The Rustonomicon] has more information about these guarantees and how to
uphold them.
-->

これらのトレイトを手動で実装するには、unsafe な Rust コードを実装することが関わってきます。
unsafe な Rust コードを使用することについては第 19 章で語ります; とりあえず、重要な情報は、
`Send`と`Sync`ではない部品からなる新しい並行な型を構成するには、安全性保証を保持するために、
注意深い思考が必要になるということです。[The Rustonomicon]には、
これらの保証とそれを保持する方法についての情報がより多くあります。

> 訳注：日本語版の The Rustonomicon は[こちら][nomicon-ja]です。

[The Rustonomicon]: https://doc.rust-lang.org/stable/nomicon/
[nomicon-ja]: https://doc.rust-jp.rs/rust-nomicon-ja/index.html

<!--
## Summary
-->

## まとめ

<!--
This isn’t the last you’ll see of concurrency in this book: the project in
Chapter 20 will use the concepts in this chapter in a more realistic situation
than the smaller examples discussed here.
-->

この本において並行性を見かけるのは、これで最後ではありません：第 20 章のプロジェクトでは、
この章の概念をここで議論した微小な例よりもより現実的な場面で使用するでしょう。

<!--
最後は mutithreaded situations となっているが、situation を環境と訳した方が自然なので、そうしている
-->

<!--
As mentioned earlier, because very little of how Rust handles concurrency is
part of the language, many concurrency solutions are implemented as crates.
These evolve more quickly than the standard library, so be sure to search
online for the current, state-of-the-art crates to use in multithreaded
situations.
-->

前述のように、Rust による並行性の取扱いのごく一部のみが言語仕様なので、多くの並行性の解決策は
クレートとして実装されています。これらは標準ライブラリよりも迅速に進化するので、
マルチスレッド環境で使用すべき現在の最先端のクレートを必ずネットで検索してください。

<!--
The Rust standard library provides channels for message passing and smart
pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in
concurrent contexts. The type system and the borrow checker ensure that the
code using these solutions won’t end up with data races or invalid references.
Once you get our code to compile, you can rest assured that it will happily
run on multiple threads without the kinds of hard-to-track-down bugs common in
other languages. Concurrent programming is no longer a concept to be afraid of:
go forth and make your programs concurrent, fearlessly!
-->

Rust の標準ライブラリは、メッセージ受け渡しにチャンネルを、並行の文脈で安全に使用できる、
`Mutex<T>`や`Arc<T>`などのスマートポインタ型を提供しています。型システムと借用チェッカーにより、
これらの解決策を使用するコードがデータ競合や無効な参照に行き着かないことを保証してくれます。
一旦コードをコンパイルすることができたら、他の言語ではありふれている追跡困難な類のバグなしに、
複数のスレッドでも喜んで動くので安心できます。並行プログラミングは、もはや恐れるべき概念ではありません：
恐れることなく前進し、プログラムを並行にしてください！

<!--
Next, we’ll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we’ll discuss how Rust’s idioms
relate to those you might be familiar with from object oriented programming.
-->

次は、Rust プログラムが肥大化するにつれて問題をモデル化し、解決策を構造化する慣例的な方法について話します。
さらに、Rust のイディオムがオブジェクト指向プログラミングで馴染み深いかもしれないイディオムにどのように関連しているかについても議論します。
