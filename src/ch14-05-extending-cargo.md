<!--
## Extending Cargo with Custom Commands
-->

## 独自のコマンドで Cargo を拡張する

<!--
Cargo is designed so you can extend it with new subcommands without having to
modify Cargo. If a binary in your `$PATH` is named `cargo-something`, you can
run it as if it was a Cargo subcommand by running `cargo something`. Custom
commands like this are also listed when you run `cargo --list`. Being able to
use `cargo install` to install extensions and then run them just like the
built-in Cargo tools is a super convenient benefit of Cargo’s design!
-->

Cargo は変更する必要なく、新しいサブコマンドで拡張できるように設計されています。
`$PATH`にあるバイナリが`cargo-something`という名前なら、`cargo something`を実行することで、
Cargo のサブコマンドであるかのように実行することができます。このような独自のコマンドは、
`cargo --list`を実行すると、列挙もされます。`cargo install`を使用して拡張をインストールし、
それから組み込みの Cargo ツール同様に実行できることは、Cargo の設計上の非常に便利な恩恵です！

<!--
## Summary
-->

## まとめ

<!--
Sharing code with Cargo and [crates.io](https://crates.io) is
part of what makes the Rust ecosystem useful for many different tasks. Rust’s
standard library is small and stable, but crates are easy to share, use, and
improve on a timeline different from that of the language. Don’t be shy about
sharing code that’s useful to you on [crates.io](https://crates.io);
it’s likely that it will be useful to someone else as well!
-->

Cargo で[crates.io](https://crates.io)とコードを共有することは、
Rust のエコシステムを多くの異なる作業に有用にするものの一部です。Rust の標準ライブラリは、
小さく安定的ですが、クレートは共有および使用しやすく、言語とは異なるタイムラインで進化します。
積極的に[crates.io](https://crates.io)で自分にとって有用なコードを共有してください;
他の誰かにとっても、役に立つものであることでしょう！
