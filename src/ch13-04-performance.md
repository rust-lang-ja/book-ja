<!--
## Comparing Performance: Loops vs. Iterators
-->

## パフォーマンス比較：ループ VS イテレータ

<!--
To determine whether to use loops or iterators, you need to know which version
of our `search` functions is faster: the version with an explicit `for` loop or
the version with iterators.
-->

ループを使うべきかイテレータを使うべきか決定するために、`search`関数のうち、どちらのバージョンが速いか知る必要があります：
明示的な`for`ループがあるバージョンと、イテレータのバージョンです。

<!--
We ran a benchmark by loading the entire contents of *The Adventures of
Sherlock Holmes* by Sir Arthur Conan Doyle into a `String` and looking for the
word *the* in the contents. Here are the results of the benchmark on the
version of `search` using the `for` loop and the version using iterators:
-->

サー・アーサー・コナン・ドイル (Sir Arthur Conan Doyle) の、
*シャーロックホームズの冒険*(The Adventures of Sherlock Homes) 全体を`String`に読み込み、
そのコンテンツで*the*という単語を検索することでベンチマークを行いました。
こちらが、`for`を使用した`search`関数のバージョンと、イテレータを使用したバージョンに関するベンチマーク結果です。

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

<!--
The iterator version was slightly faster! We won’t explain the benchmark code
here, because the point is not to prove that the two versions are equivalent
but to get a general sense of how these two implementations compare
performance-wise.
-->

イテレータバージョンの方が<ruby>些<rp>(</rp><rt>いささ</rt><rp>)</rp></ruby>か高速ですね！ここでは、ベンチマークのコードは説明しません。
なぜなら、要点は、2 つのバージョンが等価であることを証明することではなく、
これら 2 つの実装がパフォーマンス的にどう比較されるかを大まかに把握することだからです。

<!--
For a more comprehensive benchmark, you should check various texts of
various sizes as the `contents`, different words and words of different lengths
as the `query`, and all kinds of other variations. The point is this:
iterators, although a high-level abstraction, get compiled down to roughly the
same code as if you’d written the lower-level code yourself. Iterators are one
of Rust’s *zero-cost abstractions*, by which we mean using the abstraction
imposes no additional runtime overhead. This is analogous to how Bjarne
Stroustrup, the original designer and implementor of C++, defines
*zero-overhead* in “Foundations of C++” (2012):
-->

より包括的なベンチマークとするためには、いろんなサイズの様々なテキストを`contents`として、異なる単語、異なる長さの単語を`query`として、
他のあらゆる種類のバリエーションを確認するべきです。重要なのは：イテレータは、
高度な抽象化にも関わらず、低レベルのコードを自身で書いているかのように、ほぼ同じコードにコンパイルされることです。
イテレータは、Rust の*ゼロコスト抽象化*の一つであり、これは、抽象化を使うことが追加の実行時オーバーヘッドを生まないことを意味しています。
このことは、C++の元の設計者であり実装者のビャーネ・ストロヴストルップ (Bjarne Stroustrup) が、
*ゼロオーバーヘッド*を「C++の基礎 (2012)」で定義したのと類似しています。

<!--
> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.
-->

> 一般的に、C++の実装は、ゼロオーバーヘッド原則を遵守します：使用しないものには、支払わなくてよい。
> さらに：実際に使っているものに対して、コードをそれ以上うまく渡すことはできない。

<!--
As another example, the following code is taken from an audio decoder. The
decoding algorithm uses the linear prediction mathematical operation to
estimate future values based on a linear function of the previous samples. This
code uses an iterator chain to do some math on three variables in scope: a
`buffer` slice of data, an array of 12 `coefficients`, and an amount by which
to shift data in `qlp_shift`. We’ve declared the variables within this example
but not given them any values; although this code doesn’t have much meaning
outside of its context, it’s still a concise, real-world example of how Rust
translates high-level ideas to low-level code:
-->

別の例として、以下のコードは、オーディオデコーダから取ってきました。デコードアルゴリズムは、
線形予測数学演算を使用して、以前のサンプルの線形関数に基づいて未来の値を予測します。このコードは、
イテレータ連結をしてスコープにある 3 つの変数に計算を行っています：`buffer`というデータのスライス、
12 の`coefficients`(係数) の配列、`qlp_shift`でデータをシフトする量です。この例の中で変数を宣言しましたが、
値は与えていません; このコードは、文脈の外では大して意味を持ちませんが、
それでも Rust が高レベルな考えを低レベルなコードに翻訳する簡潔で現実的な例になっています：

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

<!--
To calculate the value of `prediction`, this code iterates through each of the
12 values in `coefficients` and uses the `zip` method to pair the coefficient
values with the previous 12 values in `buffer`. Then, for each pair, we
multiply the values together, sum all the results, and shift the bits in the
sum `qlp_shift` bits to the right.
-->

`prediction`の値を算出するために、このコードは、`coefficients`の 12 の値を繰り返し、`zip`メソッドを使用して、
係数値を前の`buffer`の 12 の値と組にします。それから各組について、その値をかけ合わせ、結果を全て合計し、
合計のビットを`qlp_shift`ビット分だけ右にシフトさせます。

<!--
Calculations in applications like audio decoders often prioritize performance
most highly. Here, we’re creating an iterator, using two adaptors, and then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you’d write by hand.
There’s no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are 12 iterations, so it “unrolls” the
loop. *Unrolling* is an optimization that removes the overhead of the loop
controlling code and instead generates repetitive code for each iteration of
the loop.
-->

オーディオデコーダのようなアプリケーションの計算は、しばしばパフォーマンスに最も重きを置きます。
ここでは、イテレータを作成し、2 つのアダプタを使用し、それから値を消費しています。
この Rust コードは、どんな機械語コードにコンパイルされるのでしょうか？えー、執筆時点では、
手作業で書いたものと同じ機械語にコンパイルされます。`coefficients`の値の繰り返しに対応するループは全く存在しません：
コンパイラは、12 回繰り返しがあることを把握しているので、ループを「展開」します。
*ループの展開*は、ループ制御コードのオーバーヘッドを除去し、代わりにループの繰り返しごとに同じコードを生成する最適化です。

<!--
All of the coefficients get stored in registers, which means accessing the
values is very fast. There are no bounds checks on the array access at runtime.
All these optimizations that Rust is able to apply make the resulting code
extremely efficient. Now that you know this, you can use iterators and closures
without fear! They make code seem like it’s higher level but don’t impose a
runtime performance penalty for doing so.
-->

係数は全てレジスタに保存されます。つまり、値に非常に高速にアクセスします。実行時に配列の境界チェックをすることもありません。
コンパイラが適用可能なこれらの最適化全てにより、結果のコードは究極的に効率化されます。このことがわかったので、
もうイテレータとクロージャを恐れなしに使用することができますね！それらのおかげでコードは、高レベルだけれども、
そうすることに対して実行時のパフォーマンスを犠牲にしないようになります。

<!--
## Summary
-->

## まとめ

<!--
Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust’s capability to clearly express
high-level ideas at low-level performance. The implementations of closures and
iterators are such that runtime performance is not affected. This is part of
Rust’s goal to strive to provide zero-cost abstractions.
-->

クロージャとイテレータは、関数型言語の考えに着想を得た Rust の機能です。低レベルのパフォーマンスで、
高レベルの考えを明確に表現するという Rust の能力に貢献しています。クロージャとイテレータの実装は、
実行時のパフォーマンスが影響されないようなものです。これは、ゼロ代償抽象化を提供するのに努力を惜しまない Rust の目標の一部です。

<!--
Now that we’ve improved the expressiveness of our I/O project, let’s look at
some more features of `cargo` that will help us share the project with the
world.
-->

今や入出力プロジェクトの表現力を改善したので、プロジェクトを世界と共有するのに役に立つ`cargo`の機能にもっと目を向けましょう。
