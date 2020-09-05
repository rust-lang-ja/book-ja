<!--
# Foreword
-->

# まえがき

<!--
It wasn’t always so clear, but the Rust programming language is fundamentally
about *empowerment*: no matter what kind of code you are writing now, Rust
empowers you to reach farther, to program with confidence in a wider variety of
domains than you did before.
-->

すぐにはわかりにくいかもしれませんが、Rustプログラミング言語は、エンパワーメント(empowerment)を根本原理としています:
どんな種類のコードを現在書いているにせよ、Rustは幅広い領域で以前よりも遠くへ到達し、
自信を持ってプログラムを組む力を与え(empower)ます。

<!--
Take, for example, “systems-level” work that deals with low-level details of
memory management, data representation, and concurrency. Traditionally, this
realm of programming is seen as arcane, accessible only to a select few who
have devoted the necessary years learning to avoid its infamous pitfalls. And
even those who practice it do so with caution, lest their code be open to
exploits, crashes, or corruption.
-->

一例を挙げると、メモリ管理やデータ表現、並行性などの低レベルな詳細を扱う「システムレベル」のプログラミングがあります。
伝統的にこの分野は難解で、年月をかけてやっかいな落とし穴を回避する術を習得した選ばれし者にだけ可能と見なされています。
そのように鍛錬を積んだ者でさえ注意が必要で、さもないと書いたコードがクラッキングの糸口になったりクラッシュやデータ破損を引き起こしかねないのです。

<!--
Rust breaks down these barriers by eliminating the old pitfalls and providing a
friendly, polished set of tools to help you along the way. Programmers who need
to “dip down” into lower-level control can do so with Rust, without taking on
the customary risk of crashes or security holes, and without having to learn
the fine points of a fickle toolchain. Better yet, the language is designed to
guide you naturally towards reliable code that is efficient in terms of speed
and memory usage.
-->

この難しさを取り除くために、Rustは、古い落とし穴を排除し、その過程で使いやすく役に立つ洗練された一連のツールを提供します。
低レベルな制御に「下がる」必要があるプログラマは、お決まりのクラッシュやセキュリティホールのリスクを負わず、
気まぐれなツールチェーンのデリケートな部分を学ぶ必要なくRustで同じことができます。さらにいいことに、
Rustは、スピードとメモリ使用の観点で効率的な信頼性の高いコードへと自然に導くよう設計されています。

<!--
Programmers who are already working with low-level code can use Rust to raise
their ambitions. For example, introducing parallelism in Rust is a relatively
low-risk operation: the compiler will catch the classical mistakes for you. And
you can tackle more aggressive optimizations in your code with the confidence
that you won’t accidentally introduce crashes or exploits.
-->

既に低レベルコードに取り組んでいるプログラマは、Rustを使用してさらなる高みを目指せます。例えば、
Rustで並列性を導入することは、比較的低リスクです: コンパイラが伝統的なミスを捕捉してくれるのです。
そして、クラッシュやクラッキングの糸口を誤って導入しないという自信を持ってコードの大胆な最適化に取り組めるのです。

<!--
But Rust isn’t limited to low-level systems programming. It’s expressive and
ergonomic enough to make CLI apps, web servers, and many other kinds of code
quite pleasant to write — you’ll find simple examples of both later in the
book. Working with Rust allows you to build skills that transfer from one
domain to another; you can learn Rust by writing a web app, then apply those
same skills to target your Raspberry Pi.
-->

ですが、Rustは低レベルなシステムプログラミングに限定されているわけではありません。十分に表現力豊かでエルゴノミックなので、
コマンドラインアプリやWebサーバ、その他様々な楽しいコードを書けます。この本の後半に両者の単純な例が見つかるでしょう。
Rustを使うことで1つの領域から他の領域へと使い回せる技術を身につけられます;
ウェブアプリを書いてRustを学び、それからその同じ技術をラズベリーパイを対象に適用できるのです。

<!--
This book fully embraces the potential of Rust to empower its users. It’s a
friendly and approachable text intended to help you level up not just your
knowledge of Rust, but also your reach and confidence as a programmer in
general. So dive in, get ready to learn—and welcome to the Rust community!
-->

この本は、ユーザに力を与え(empower)るRustのポテンシャルを全て含んでいます。あなたのRustの知識のみをレベルアップさせるだけでなく、
プログラマとしての全般的な能力や自信をもレベルアップさせる手助けを意図した親しみやすくわかりやすいテキストです。
さあ、飛び込んで学ぶ準備をしてください。Rustコミュニティへようこそ！

<!--
— Nicholas Matsakis and Aaron Turon
-->

- ニコラス・マットサキス(Nicholas Matsakis)とアーロン・チューロン(Aaron Turon)
