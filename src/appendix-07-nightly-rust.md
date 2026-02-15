<!--
## Appendix G - How Rust is Made and “Nightly Rust”
-->

## 付録G: Rustの作られ方と“Nightly Rust”

<!--
This appendix is about how Rust is made and how that affects you as a Rust
developer.
-->

この付録は、Rustのでき方と、それがRust開発者としてあなたにどう影響するかについてです。

<!--
### Stability Without Stagnation
-->

### 停滞なしの安定性

<!--
As a language, Rust cares a *lot* about the stability of your code. We want
Rust to be a rock-solid foundation you can build on, and if things were
constantly changing, that would be impossible. At the same time, if we can’t
experiment with new features, we may not find out important flaws until after
their release, when we can no longer change things.
-->

言語として、Rustはコードの安定性について*大い*に注意しています。Rustには、その上に建築できる岩のように硬い基礎であってほしく、
物事が定期的に変わっていたら、それは実現できません。同時に新しい機能で実験できなければ、もはや何も変更できないリリースの時まで、
重大な<ruby>瑕疵<rp>(</rp><rt>かし</rt><rp>)</rp></ruby>を発見できなくなるかもしれません。

<!--
Our solution to this problem is what we call “stability without stagnation”,
and our guiding principle is this: you should never have to fear upgrading to a
new version of stable Rust. Each upgrade should be painless, but should also
bring you new features, fewer bugs, and faster compile times.
-->

この問題に対する我々の解決策は「停滞なしの安定性」と呼ばれるもので、ガイドの原則は以下の通りです:
安定版Rustの新しいバージョンにアップグレードするのを恐れる必要は何もないはずです。各アップグレードは痛みのないもののはずですが、
新しい機能、より少ないバグ、高速なコンパイル時間も齎すべきです。

<!--
### Choo, Choo! Release Channels and Riding the Trains
-->

### シュポシュポ！リリースチャンネルと列車に乗ること

<!--
Rust development operates on a *train schedule*. That is, all development is
done on the `master` branch of the Rust repository. Releases follow a software
release train model, which has been used by Cisco IOS and other software
projects. There are three *release channels* for Rust:
-->

Rust開発は、*電車のダイヤ*に合わせて処理されます。つまり、全開発はRustリポジトリの`master`ブランチで行われます。
リリースはソフトウェアのリリーストレインモデル(software release train model)に従い、これはCisco IOSや他のソフトウェアプロジェクトで活用されています。
Rustには*リリースチャンネル*が3つあります:

> 注釈: software release train modelとは、あるバージョンのソフトウェアリリースの順番を列車に見立て、
> 列車のダイヤのように、決まった間隔でリリースに持って行く手法のことの模様。一つの列車は、Rustの場合、
> ナイトリー、ベータ、安定版の順に「駅」に停車していくものと思われる。

<!--
* Nightly
* Beta
* Stable
-->

* ナイトリー
* ベータ
* 安定版

<!--
Most Rust developers primarily use the stable channel, but those who want to
try out experimental new features may use nightly or beta.
-->

多くのRust開発者は主に安定版チャンネルを使用しますが、新しい実験的な機能を試したい方は、
ナイトリーやベータを使用するかもしれません。

<!--
Here’s an example of how the development and release process works: let’s
assume that the Rust team is working on the release of Rust 1.5. That release
happened in December of 2015, but it will provide us with realistic version
numbers. A new feature is added to Rust: a new commit lands on the `master`
branch. Each night, a new nightly version of Rust is produced. Every day is a
release day, and these releases are created by our release infrastructure
automatically. So as time passes, our releases look like this, once a night:
-->

こちらが、開発とリリースプロセスの動き方の例です: RustチームがRust1.5のリリースに取り掛かっていると想定しましょう。
そのリリースは、2015年の11月に発生しましたが、現実的なバージョンナンバーを与えてくれるでしょう。
新しい機能がRustに追加されます: 新しいコミットが`master`ブランチに着地します。毎晩、新しいナイトリ版のRustが生成されます。
毎日がリリース日で、これらのリリースは、リリースインフラにより自動で作成されます。故に、
時間が経てばリリースは、毎晩1回、以下のような見た目になります:

```text
nightly: * - - * - - *
```

<!--
Every six weeks, it’s time to prepare a new release! The `beta` branch of the
Rust repository branches off from the `master` branch used by nightly. Now,
there are two releases:
-->

6週間ごとに、新しいリリースを準備するタイミングになります！Rustリポジトリの`beta`ブランチが、
ナイトリで使用される`master`ブランチから枝分かれします。さて、リリースが二つになりました:

```text
nightly: * - - * - - *
                     |
beta:                *
```

<!--
Most Rust users do not use beta releases actively, but test against beta in
their CI system to help Rust discover possible regressions. In the meantime,
there’s still a nightly release every night:
-->

ほとんどのRustユーザはベータリリースを積極的には使用しませんが、自身のCIシステム内でベータに対してテストを行い、
Rustが不具合の可能性を発見するのを手伝います。その間も、やはりナイトリリリースは毎晩あります:

> 注釈: CIはContinuous Integration(継続統合といったところか)のことと思われる。開発者のコードを1日に何度も、
> メインのブランチに統合することらしい。

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

<!--
Let’s say a regression is found. Good thing we had some time to test the beta
release before the regression snuck into a stable release! The fix is applied
to `master`, so that nightly is fixed, and then the fix is backported to the
`beta` branch, and a new release of beta is produced:
-->

不具合が見つかったとしましょう。よいことに、不具合が安定版のリリースにこっそり持ち込まれる前にベータリリースをテストする時間がありました！
修正が`master`に適用されるので、ナイトリは修正され、それから修正が`beta`ブランチにバックポートされ、
ベータの新しいリリースが生成されます:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

<!--
Six weeks after the first beta was created, it’s time for a stable release! The
`stable` branch is produced from the `beta` branch:
-->

最初のベータが作成されてから6週間後、安定版のリリースの時間です！`stable`ブランチが`beta`ブランチから生成されます:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

<!--
Hooray! Rust 1.5 is done! However, we’ve forgotten one thing: because the six
weeks have gone by, we also need a new beta of the *next* version of Rust, 1.6.
So after `stable` branches off of `beta`, the next version of `beta` branches
off of `nightly` again:
-->

やりました！Rust1.5が完了しました！ですが、1つ忘れていることがあります: 6週間が経過したので、
*次*のバージョンのRust(1.6)の新しいベータも必要です。従って、`stable`が`beta`から枝分かれした後に、
次のバージョンの`beta`が`nightly`から再度枝分かれします:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

<!--
This is called the “train model” because every six weeks, a release “leaves the
station”, but still has to take a journey through the beta channel before it
arrives as a stable release.
-->

これが「トレイン・モデル」と呼ばれます。6週間ごとにリリースが「駅を出発する」からですが、
安定版リリースとして到着する前にベータチャンネルの旅をそれでもしなければなりません。

<!--
Rust releases every six weeks, like clockwork. If you know the date of one Rust
release, you can know the date of the next one: it’s six weeks later. A nice
aspect of having releases scheduled every six weeks is that the next train is
coming soon. If a feature happens to miss a particular release, there’s no need
to worry: another one is happening in a short time! This helps reduce pressure
to sneak possibly unpolished features in close to the release deadline.
-->

Rustは6週間ごとに時計仕掛けのようにリリースされます。あるRustリリースの日付を知っていれば、
次のリリースの日付もわかります: 6週間後です。6週間ごとにリリースを組むことのいい側面は、次の列車がすぐにやってくることです。
ある機能が偶然、特定のリリースを逃しても、心配する必要はありません: 別のリリースがすぐに起きます！
これにより、リリースの締め切りが近い洗練されていない可能性のある機能をこっそり持ち込むプレッシャーが減る助けになるのです。

<!--
Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it’s easy to upgrade to: if a beta release doesn’t
work as expected, you can report it to the team and get it fixed before the
next stable release happens! Breakage in a beta release is relatively rare, but
`rustc` is still a piece of software, and bugs do exist.
-->

このプロセスのおかげで、Rustの次のビルドを常に確認し、アップグレードするのが容易であると自身に対して確かめることができます:
ベータリリースが予想した通りに動かなければ、チームに報告して、次の安定版のリリースが起きる前に直してもらうことができるのです！
ベータリリースでの破損はどちらかといえば稀ですが、`rustc`もソフトウェアの一種であり、バグは確実に存在します。

<!--
### Unstable Features
-->

### 安定しない機能

<!--
There’s one more catch with this release model: unstable features. Rust uses a
technique called “feature flags” to determine what features are enabled in a
given release. If a new feature is under active development, it lands on
`master`, and therefore, in nightly, but behind a *feature flag*. If you, as a
user, wish to try out the work-in-progress feature, you can, but you must be
using a nightly release of Rust and annotate your source code with the
appropriate flag to opt in.
-->

このリリースモデルにはもう一つ掴み所があります: 安定しない機能です。Rustは「機能フラグ」と呼ばれるテクニックを使用して、
あるリリースで有効にする機能を決定します。新しい機能が活発に開発中なら、`master`に着地し、
故にナイトリーでは*機能フラグ*の背後に存在します。ユーザとして、絶賛作業中の機能を試したいとお望みならば、
可能ですが、ナイトリリリースのRustを使用し、ソースコードに適切なフラグを注釈して同意しなければなりません。

<!--
If you’re using a beta or stable release of Rust, you can’t use any feature
flags. This is the key that allows us to get practical use with new features
before we declare them stable forever. Those who wish to opt into the bleeding
edge can do so, and those who want a rock-solid experience can stick with
stable and know that their code won’t break. Stability without stagnation.
-->

ベータか安定リリースのRustを使用しているなら、機能フラグは使用できません。これが、永遠に安定であると宣言する前に、
新しい機能を実用に供することができる鍵になっています。最先端を選択するのをお望みの方はそうすることができ、
岩のように硬い経験をお望みの方は、安定版に執着し自分のコードが壊れることはないとわかります。停滞なしの安定性です。

<!--
This book only contains information about stable features, as in-progress
features are still changing, and surely they’ll be different between when this
book was written and when they get enabled in stable builds. You can find
documentation for nightly-only features online.
-->

この本は安定な機能についての情報のみ含んでいます。現在進行形の機能は、変化中であり、
確実にこの本が執筆された時と安定版ビルドで有効化された時で異なるからです。ナイトリ限定の機能についてのドキュメンテーションは、
オンラインで発見できます。

<!--
### Rustup and the Role of Rust Nightly
-->

### RustupとRustナイトリの役目

<!--
Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis. By default, you’ll have stable Rust installed. To
install nightly, for example:
-->

rustupは、グローバルかプロジェクトごとにRustのリリースチャンネルを変更しやすくしてくれます。
標準では、安定版のRustがインストールされます。例えば、ナイトリをインストールするには:

```console
$ rustup toolchain install nightly
```

<!--
You can see all of the *toolchains* (releases of Rust and associated
components) you have installed with `rustup` as well. Here’s an example on one
of your authors’ Windows computer:
-->

`rustup`でインストールした全ツールチェーン(Rustのリリースと関連するコンポーネント)も確認できます。
こちらは、著者の一人のWindowsコンピュータの例です:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

<!--
As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project’s directory to set the
nightly toolchain as the one `rustup` should use when you’re in that directory:
-->

おわかりのように、安定版のツールチェーンが標準です。ほとんどのRustユーザは、ほとんどの場合、安定版を使用します。
あなたもほとんどの場合安定版を使用したい可能性がありますが、最前線の機能が気になるので、特定のプロジェクトではナイトリを使用したいかもしれません。
そうするためには、そのプロジェクトのディレクトリで`rustup override`を使用して、そのディレクトリにいる時に、
`rustup`が使用するべきツールチェーンとしてナイトリ版のものをセットします。

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

<!--
Now, every time you call `rustc` or `cargo` inside of
*~/projects/needs-nightly*, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust. This comes in handy when you
have a lot of Rust projects!
-->

これで *~/projects/needs-nightly*内で`rustc`や`cargo`を呼び出す度に、`rustup`は既定の安定版のRustではなく、
ナイトリRustを使用していることを確かめます。Rustプロジェクトが大量にある時には、重宝します。

<!--
### The RFC Process and Teams
-->

### RFCプロセスとチーム

<!--
So how do you learn about these new features? Rust’s development model follows
a *Request For Comments (RFC) process*. If you’d like an improvement in Rust,
you can write up a proposal, called an RFC.
-->

では、これらの新しい機能をどう習うのでしょうか？Rustの開発モデルは、*Request For Comments (RFC; コメントの要求)プロセス*に従っています。
Rustに改善を行いたければ、RFCと呼ばれる提案を書き上げます。

<!--
Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many topic subteams. There’s
a full list of the teams [on Rust’s
website](https://www.rust-lang.org/governance), which includes teams for
each area of the project: language design, compiler implementation,
infrastructure, documentation, and more. The appropriate team reads the
proposal and the comments, writes some comments of their own, and eventually,
there’s consensus to accept or reject the feature.
-->

誰もがRFCを書いてRustを改善でき、提案はRustチームにより査読され議論され、このチームは多くの話題のサブチームから構成されています。
[RustのWebサイト](https://www.rust-lang.org/en-US/team.html)にはチームの完全なリストがあり、
プロジェクトの各分野のチームも含みます: 言語設計、コンパイラ実装、インフラ、ドキュメンテーションなどです。
適切なチームが提案とコメントを読み、自身のコメントを書き、最終的にその機能を受け入れるか拒否するかの同意があります。

<!--
If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be the
person who proposed the feature in the first place! When the implementation is
ready, it lands on the `master` branch behind a feature gate, as we discussed
in the [“Unstable Features”](#unstable-features) section.
-->

機能が受け入れられれば、Rustリポジトリでissueが開かれ、誰かがそれを実装します。うまく実装できる人は、
そもそもその機能を提案した人ではないかもしれません！実装の準備ができたら、
[「安定しない機能」](#安定しない機能)節で議論したように、機能ゲートの背後の`master`に着地します。

<!--
After some time, once Rust developers who use nightly releases have been able
to try out the new feature, team members will discuss the feature, how it’s
worked out on nightly, and decide if it should make it into stable Rust or not.
If the decision is to move forward, the feature gate is removed, and the
feature is now considered stable! It rides the trains into a new stable release
of Rust.
-->

時間経過後、一旦ナイトリリリースを使用するRust開発者が新しい機能を試すことができたら、チームのメンバーがその機能と、
ナイトリでどう機能しているかについて議論し、安定版のRustに導入すべきかどうか決定します。
決定が進行させることだったら、機能ゲートは取り除かれ、その機能はもう安定と考えられます！
Rustの新しい安定版リリースまで、列車に乗っているのです。
