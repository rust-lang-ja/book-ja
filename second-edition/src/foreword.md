<!-- # Foreword -->

# まえがき

<!-- It wasn’t always so clear, but the Rust programming language is fundamentally -->
<!-- about *empowerment*: no matter what kind of code you are writing now, Rust -->
<!-- empowers you to reach farther, to program with confidence in a wider variety of -->
<!-- domains than you did before. -->

常に明確だったわけではありませんが、Rustプログラミング言語は、根本的には、エンパワーメント(empowerment)に取り組んでいます:
どんな種類のコードを現在書いているにせよ、Rustは幅広い領域で以前よりもより遠くへ到達すること、
自信を持ってプログラムを組むことへの原動力になります。

<!-- Take, for example, “systems-level” work that deals with low-level details of -->
<!-- memory management, data representation, and concurrency. Traditionally, this -->
<!-- realm of programming is seen as arcane, accessible only to a select few who -->
<!-- have devoted the necessary years learning to avoid its infamous pitfalls. And -->
<!-- even those who practice it do so with caution, lest their code be open to -->
<!-- exploits, crashes, or corruption. -->

一例を挙げると、メモリ管理やデータ表現、並行などの低レベルな詳細を扱う「システムレベル」の作業があります。
伝統的にこの分野のプログラミングは、難解で、悪名高い落とし穴を回避する術を数年の年月をかけて習得した選ばれし者にだけ可能と見なされています。
そのように鍛錬を積んだ者でさえ書いたコードがクラッキングの糸口になったりクラッシュやデータ破損を引き起こしたりしないよう注意深くプログラミングするのです。

<!-- Rust breaks down these barriers by eliminating the old pitfalls and providing a -->
<!-- friendly, polished set of tools to help you along the way. Programmers who need -->
<!-- to “dip down” into lower-level control can do so with Rust, without taking on -->
<!-- the customary risk of crashes or security holes, and without having to learn -->
<!-- the fine points of a fickle toolchain. Better yet, the language is designed to -->
<!-- guide you naturally towards reliable code that is efficient in terms of speed -->
<!-- and memory usage. -->

Rustは、古い落とし穴を排除し、その過程で助けになる親しみ深い洗練された一連のツールを提供することで、
これらの障壁を取り除きます。低レベルな制御に「下がる」必要があるプログラマは、お決まりのクラッシュやセキュリティホールのリスクを負わず、
気まぐれなツールチェーンのデリケートな部分を学ぶ必要なくRustでそうすることができます。さらにいいことに、
Rustは、スピードとメモリ使用の観点で効率的な信頼性の高いコードへと自然に導くよう設計されています。

<!-- Programmers who are already working with low-level code can use Rust to raise -->
<!-- their ambitions. For example, introducing parallelism in Rust is a relatively -->
<!-- low-risk operation: the compiler will catch the classical mistakes for you. And -->
<!-- you can tackle more aggressive optimizations in your code with the confidence -->
<!-- that you won’t accidentally introduce crashes or exploits. -->

既に低レベルコードに取り組んでいるプログラマは、Rustを使用して野望を高めることができます。例えば、
Rustで並行性を導入することは、比較的低リスクな処理です: コンパイラが伝統的なミスを捕捉してくれるのです。
そして、クラッシュやクラッキングの糸口を誤って導入しないという自信を持ってコードの積極的な最適化に取り組むことができるのです。

<!-- But Rust isn’t limited to low-level systems programming. It’s expressive and -->
<!-- ergonomic enough to make CLI apps, web servers, and many other kinds of code -->
<!-- quite pleasant to write — you’ll find simple examples of both later in the -->
<!-- book. Working with Rust allows you to build skills that transfer from one -->
<!-- domain to another; you can learn Rust by writing a web app, then apply those -->
<!-- same skills to target your Raspberry Pi. -->

ですが、Rustは低レベルなシステムプログラミングに限定されているわけではありません。十分表現力豊かでエルゴノミックなので、
コマンドラインアプリやWebサーバ、その他様々な楽しいコードを書けます。この本の後半に両者の単純な例が見つかるでしょう。
Rustで作業をすることで1つの領域から他の領域へと移動する技術を身につけることができます;
ウェブアプリを書くことでRustを学び、それからその同じ技術をラズベリーパイを対象にして適用できるのです。

<!-- This book fully embraces the potential of Rust to empower its users. It’s a -->
<!-- friendly and approachable text intended to help you level up not just your -->
<!-- knowledge of Rust, but also your reach and confidence as a programmer in -->
<!-- general. So dive in, get ready to learn—and welcome to the Rust community! -->

この本は、ユーザの動力源となるRustのポテンシャルを全て抱えています。あなたのRustの知識のみをレベルアップさせるだけでなく、
プログラマとして全般的なリーチや自信をもレベルアップさせる手助けをすることを意図した親しみやすく近寄りやすいテキストです。
従って、飛び込んで学ぶ準備をしてください。Rustコミュニティへようこそ！

<!-- — Nicholas Matsakis and Aaron Turon -->

- ニコラス・マットサキス(Nicholas Matsakis)とアーロン・チューロン(Aaron Turon)
