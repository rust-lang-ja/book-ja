<!-- # Fearless Concurrency -->

# 恐れるな！非同期処理

<!-- Handling concurrent programming safely and efficiently is another of Rust’s -->
<!-- major goals. *Concurrent programming*, where different parts of a program -->
<!-- execute independently, and *parallel programming*, where different parts of a -->
<!-- program execute at the same time, are becoming increasingly important as more -->
<!-- computers take advantage of their multiple processors. Historically, -->
<!-- programming in these contexts has been difficult and error prone: Rust hopes to -->
<!-- change that. -->

非同期処理を安全かつ効率的に扱うことは、Rustの別の主な目標です。非同期プログラミングは、プログラムの異なる部分が個別に実行することであり、
並列プログラミングはプログラムの異なる部分が同時に実行することですが、多くのコンピュータが複数のプロセッサの利点を生かすようになるにつれ、
重要度を増しています。歴史的に、これらの文脈で行うプログラミングは困難で、エラーが起きやすいものでした:
Rustはこれを変えると期待されています。

<!-- Initially, the Rust team thought that ensuring memory safety and preventing -->
<!-- concurrency problems were two separate challenges to be solved with different -->
<!-- methods. Over time, the team discovered that the ownership and type systems are -->
<!-- a powerful set of tools to help manage memory safety *and* concurrency -->
<!-- problems! By leveraging ownership and type checking, many concurrency errors -->
<!-- are compile-time errors in Rust rather than runtime errors. Therefore, rather -->
<!-- than making you spend lots of time trying to reproduce the exact circumstances -->
<!-- under which a runtime concurrency bug occurs, incorrect code will refuse to -->
<!-- compile and present an error explaining the problem. As a result, you can fix -->
<!-- your code while you’re working on it rather than potentially after it has been -->
<!-- shipped to production. We’ve nicknamed this aspect of Rust *fearless* -->
<!-- *concurrency*. Fearless concurrency allows you to write code that is free of -->
<!-- subtle bugs and is easy to refactor without introducing new bugs. -->

当初、Rustチームは、メモリ安全性を保証することと、非同期処理問題を回避することは、
異なる方法で解決すべき別々の課題だと考えていました。時間とともに、チームは、所有権と型システムは、
メモリ安全性*と*非同期処理問題を管理する役に立つ一連の強力な道具であることを発見しました。
所有権と型チェックを活用することで、多くの非同期処理エラーは、実行時エラーではなくコンパイル時エラーになります。
故に、実行時に非同期処理のバグが起きた状況と全く同じ状況を再現しようと時間を浪費させるよりも、
不正なコードはコンパイルを拒み、問題を説明するエラーを提示するでしょう。結果として、
プロダクトになった後でなく、作業中にコードを修正できます。
Rustのこの方向性を*恐れるな！非同期処理*とニックネーム付けしました。これにより、潜在的なバグがなく、
新しいバグを導入することなく簡単にリファクタリングできるコードを書くことができます。

<!-- Note: For simplicity’s sake, we’ll refer to many of the problems as -->
<!-- *concurrent* rather than being more precise by saying *concurrent and/or -->
<!-- parallel*. If this book were about concurrency and/or parallelism, we'd be -->
<!-- more specific. For this chapter, please mentally substitute *concurrent -->
<!-- and/or parallel* whenever we use *concurrent*. -->

> 注釈: 簡潔性のため、非同期または並列と述べることで正確を期するのではなく、
> 多くの問題を*非同期*と割り切ってしまいます。この本がもし*非同期あるいは並列性*に関した本ならば、
> 詳述していたでしょう。この章に対しては、*非同期*を使ったら、
> 脳内で*非同期または並列性*と置き換えてください。

<!-- Many languages are dogmatic about the solutions they offer for handling -->
<!-- concurrent problems. For example, Erlang has elegant functionality for -->
<!-- message-passing concurrency but has only obscure ways to share state between -->
<!-- threads. Supporting only a subset of possible solutions is a reasonable -->
<!-- strategy for higher-level languages, because a higher-level language promises -->
<!-- benefits from giving up some control to gain abstractions. However, lower-level -->
<!-- languages are expected to provide the solution with the best performance in any -->
<!-- given situation and have fewer abstractions over the hardware. Therefore, Rust -->
<!-- offers a variety of tools for modeling problems in whatever way is appropriate -->
<!-- for your situation and requirements. -->

多くの言語は、自分が提供する非同期処理問題を扱う解決策について独断的です。例えば、Erlangには、
メッセージ受け渡しの非同期処理に関する素晴らしい機能がありますが、スレッド間で状態を共有することに関しては、
曖昧な方法しかありません。可能な解決策の一部のみをサポートすることは、高級言語にとっては合理的な施策です。
なぜなら、高級言語は一部の制御を失う代わりに抽象化することから恩恵を受けるからです。ところが、
低級言語は、どんな場面でも最高のパフォーマンスで解決策を提供すると想定され、ハードウェアに関してほとんど抽象化はしません。
そのため、Rustは、自分の状況と必要性に適した方法が何であれ、問題をモデル化するためのいろんな道具を備えています。

<!-- Here are the topics we’ll cover in this chapter: -->

こちらが、この章で講義する話題です:

<!-- * How to create threads to run multiple pieces of code at the same time -->
<!-- * *Message-passing* concurrency, where channels send messages between threads -->
<!-- * *Shared-state* concurrency, where multiple threads have access to some piece -->
<!--   of data -->
<!-- * The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to -->
<!--   user-defined types as well as types provided by the standard library -->

* スレッドを生成して、複数のコードを同時に走らせる方法
* チャンネルがスレッド間でメッセージを送る*メッセージ受け渡し*非同期処理
* 複数のスレッドが何らかのデータにアクセスする*状態共有*非同期処理
* 標準ライブラリが提供する型だけでなく、ユーザが定義した型に対してもRustの非同期処理の安全保証を拡張する`Sync`と`Send`トレイト
