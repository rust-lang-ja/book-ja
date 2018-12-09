# Rust言語

このリポジトリには、Rust本第1版と第2版両方がありますが、今回は第2版のみの翻訳です。
第1版以上に、量が多そうなので、大変そうですが、以下の2点を重視して翻訳していきます。

* 逐語訳に拘らず、読者にとってわかりやすい訳を目指す(適宜、脚注を挟む、文意に沿った訳語を選択するなど)
* 原文の語順を極力尊重する。(..., like so)みたいな句を文中に持っていかず、(...。こんな感じに)
のような形で訳す。つまり、ですます調で訳しますが、あまり堅すぎる文章にはしたくないという意図です。

僭越ながら、頑張りますので、よろしくお願いいたします。

κeenさん作成の、第1版の[対訳表][translation-table]を参考に翻訳を進めますが、上記の方針により、
その通りになるとは限りません。

以下、[κeenさんのCONTRIBUTING.md][contributing]より引用、加筆修正したものを掲載します。

__2週間、修正がなかったので、原語版は完成したものと思われます。当リポジトリももう大きな変更はないはずです(2018/6/4現在)。__

## 翻訳時の指針

### 書式類

* `mdbook`の仕様上、英文をコメントアウトして直下に1行空けてから、和訳を書く
   * ただし、先頭が%で始まるタイトルだけはrustbookの制約の関係上、原文を直下に置く
* 1パラグラフ単位で翻訳する
* ただし、rustのコードブロック（バッククォート3つで始まる別行立てのもの）中のコメントについてはコピペして原文をコメントアウトし、
日本語訳が後に来るようにする。
   * コードのブロック内の文字列に含まれる英文は、1行上にコメント形式で和訳を記す
   * テキストコードブロックは、基本的に1行下にカッコ書きで和訳を書き記す
* 標準ライブラリのリファレンスへのリンクは相対リンクのままとして、英語版から変更しない
* クォート(')やダブルクォート(")は鉤括弧(「」)にする
   * ただし、原文をそのまま引用する場合は、`"`にする
* 句読点には、。を、感嘆符は全角のエクスクラメーションマーク(！)を、疑問符は全角のクエスチョンマーク(？)を用いる。
これらの記号の後にスペースは入れない（See Issue #82）。
* 括弧は基本的に半角の`(`と`)`を使用する
* どんなに1行が長くなっても日本語の文の途中で改行しない。レンダリングで余計な空白が入ってしまう。句点(。)、最悪でも読点(、)の後で改行する。
* 空白/空行を空けなくても処理出来るかはrustbookの気分にかなりよるので統一的に空白を空ける。
* 訳注を入れる際はインラインなら（訳注: ...）のようにし、別行なら > 訳注: ... のように囲み形式にする
### 日本語

* 敬体を基本とする
* 用語の訳は対訳表に従う
* 用語や厳密な意味論を話してる部分以外はある程度は意訳でよい
   * むしろ意訳の方が多いので、注意
   * 同じ単語の連続使用はカッコ悪く飽きる原因になるので、同じ単語でも訳が違う場合があります
* むしろ変に原文に忠実で、日本語として読みづらいよりも意味が伝わって日本語として分かりやすい方がいい。
   * ただし元の英文の意味と異なる場合（誤訳の場合）は修正が入る
* 継続用法の関係代名詞やコロン(:)、セミコロン(;)など日本語と対応取りづらい文は無理に1文に詰めず、2文に分けてもよい。 また、継続用法の関係代名詞でも限定修飾のように訳してもよい（日本語でどちらの意味にも読み取れるため。英語のテストでないので読み手の好意的解釈をあてにしてよい。）
   * `..., ~ing ...`などの文は、文意によって2文に分けて訳しています
   * 逆に原文では2文のものを、1文の和訳にしているケースもあります
   * `:`、`;`は基本的に変更せずそのままにするが、`;`は意味が伝わりづらいと考えられるので、可能ならば`つまり`などと訳出して、削除する
* 英語だとit, thatなどの指示語が多用されるが日本語だと繰り返した方が自然なことが多いので無理に指示語を使わずに自然な方を使う
* theに関しては文意によって(`この`、`その`)などと訳出していることがあります。(`a`も同様)
* someについて。someは、漠然と(ぼやけてるけど)あるというイメージを表す単語。通常学校文法などでは[`いくつか`]などと訳されますが、
その訳語では、イメージを表せないので、some people say that ...などなら(...と述べる人もいる)などのように訳します。
some of the peopleも(一部の人々)などと訳し、[`いくつか`]は`a few`や`several`などの訳語にします。
`何か`、`何らかの`などと訳していることも多いです。
* 逆にyou, your, we, ourなどの英語の文法上仕方なく出てくる人称代名詞は日本語には訳さない方が自然なことが多いので無理に訳に出さない。 特に、一般論を語る時のyouは 訳してはならない 参考 【雑談】"あなた"と訳さない"you" ~ einzelzelle
   * (こんな訳し方はしないので、大丈夫です)
   * このような代名詞に関しては、訳出しないと不自然な場合のみ、訳します
* 「している」「していく」を「*してる*」「*してく*」と訳さないこと。後者は会話などの口語では一般的ですが、あくまでもこれは公式の文書なので、似つかわしくありません。
* doの強調用法(I do appreciate it!)は、「実は」「本当に」「実際に」などと訳出します。
* 元来、日本語は無生物主語を嫌う言語なので、主語位置にある無生物は、「により」「によって」「のおかげで」などと訳出し、無生物主語を基本的に避けます。
* 基本動詞のhaveですが、上記の方針により、基本的に「AにBがある」「AにBが存在する」などと訳し、「持つ」という訳語を極力避けます(Rust has several wonderful features.のような文がよく出てくるので)。
* allow, letについて。無生物主語で使われることが多いので、allowは「できる」、letは「させてくれる」などと訳していることが多いです。本来の意味はどちらも「許す」です。(My wife allows me to spend up to 1000 yen on lunch.: 妻は、お昼に1000円まで使わせてくれる)(Let me in!: 入れさせて！)(ちなみにletの方が許す度合いはゆるいです)(余談ですが、Let's doは"let us do": 「〜させてよ」という意味です)
* 命令文は、基本的に「〜してください」と訳します。
* 時制について。日本語の時制は、英語ほど厳密ではなく、曖昧なことが多いため、あまり気にする必要はないと考えています。
   * 完了形は基本的に本来のイメージに近い「〜してきました」と訳していますが、単純に「〜しました」「〜しています」のように訳している箇所もあります。
   * 現在形を「〜しています」と訳している箇所もあります。
   * あと、文意から考えて、過去形を過去で訳していない箇所もあったようななかったような・・・
* 助動詞について。助動詞の過去形についてですが、助動詞が過去を表すのは時制の一致を受けているときぐらいであり、普通助動詞を使いつつ過去の意味を表したい場合、助動詞の後を完了形にするので、その代わりに意味を弱めて訳しています(If I were(was) there, I could've done it.)。一部、意味を弱めていない場合もあります。
   * `will`(でしょう) ↔︎ `would`(でしょう): `would`は日本語に訳すなら(かもなぁ)ぐらいの意味ですが、敬体で表すことができないため、同じにしています。
   * `can`(できます) ↔︎ `could`(できるでしょう)
   * `may`(かもしれません) ↔︎ `might`(可能性があります)
   * `should`の現在形は`shall`ですが、現代英語ではほぼ使用されないため、除外します。`should`は訳すなら、「べき」「はず」と訳します。
   * `must`に過去形はありません(辞書には、過去形は`had to`と書いてありましたが)。
   * 助動詞っぽいですが、`have to`は例外です。`had to`で普通、過去の意味を表します。
   * あと、`ought to`という表現もありますが、あまり見かけたことはありません。
* nowについて。今を表す以外に、`さあ`、`さて`などの感動詞を表すこともあり、そちらの意味で主に訳しています。「今」の意味で訳していても、「これで」「今では」など、ちょっと意訳が入っている場合が多いので注意。
* 漢字について。常用外の漢字について、特に難読と思われるものについては`ruby`タグでルビを振ります。常用漢字であっても、常用外の読みの場合には、ルビを振るよう心がけます。
* 形容詞および形容動詞の叙述用法について、和訳として認め、使用していきます。

### PRのマージについて
* PRをマージする際にレビューしていただいた部分について、議論が必要そうと感じた部分については、「TODO:」コメントを付します。これを目安に議論する際には、検索をかけてください

### mdbookについて
* HTML自動生成のツールとして使用されているmdbookは、正式版が出ているはずですが、正式版に移行する際に仕様が変わった部分があり、そこが解決できていないため、原語版のリポジトリでも、正式版以前のものを使用しているようです。最新版を入れてしまうと意図した通りのレンダリングができないのでご注意ください。

[translation-table]: https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/TranslationTable.md
[contributing]: https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/CONTRIBUTING.md

# NOTICE ABOUT STATUS

The second edition of The Rust Programming Language is getting ever closer to being printed!
This means we're not able to make large changes to chapters that are in any column to the
right of, and including, the "Frozen" column [on our Project board][proj]. Issues or pull
requests submitted for frozen chapters are welcome but will be closed until we start work
on a third edition. Thank you!

[proj]: https://github.com/rust-lang/book/projects/1

# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

This repository contains the source of all editions of "the Rust Programming
Language".

The second edition will also be available in dead-tree form by No Starch
Press, available around June 2018. Check [the No Starch Page][nostarch] for
the latest information on the release date and how to order.

[nostarch]: https://nostarch.com/rust

You can read all editions of the book for free online! Please see the book as
shipped with the latest [stable], [beta], or [nightly] Rust releases. Be
aware that issues in those versions may have been fixed in this repository
already, as those releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

## Requirements

Building the book requires [mdBook], ideally the same version that
[rust-lang/rust uses in this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/azerupi/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```

## Building

To build the book, first `cd` into the directory of the edition of the
book you'd like to build. For example, the `first-edition` or
`second-edition` directory. Then type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

### 2018 Edition

The "2018" Edition is in the process of being updated with the language changes 
that will be available with the 2018 Edition of the Rust language. All new 
contributions should be to this edition.

### Second Edition

No Starch Press has brought the second edition to print. Pull requests fixing
factual errors will be accepted and documented as errata; pull requests changing
wording or other small corrections should be made against the 2018 edition instead.

### First Edition

The first edition is frozen, and is not accepting any changes at this time.


[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations

We'd especially love help translating the second edition or 2018 edition of the book! See the
[Translations] label to join in efforts that are currently in progress. Open
a new issue to start working on a new language! We're waiting on [mdbook
support] for multiple languages before we merge any in, but feel free to
start! The second edition is frozen and won't see major
changes, so if you start with that, you won't have to redo work :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/azerupi/mdBook/issues/5

## No Starch

As the second edition of the book will be published by No Starch, we first
iterate here, then ship the text off to No Starch. Then they do editing, and we
fold it back in.

As such, there’s a directory, *nostarch*, which corresponds to the text in No
Starch’s system.

When we've started working with No Starch in a word doc, we will also check
those into the repo in the *nostarch/odt* directory. To extract the text from
the word doc as markdown in order to backport changes to the online book:

1. Open the doc file in LibreOffice
1. Accept all tracked changes
1. Save as Microsoft Word 2007-2013 XML (.docx) in the *tmp* directory
1. Run `./doc-to-md.sh`
1. Inspect changes made to the markdown file in the *nostarch* directory and
   copy the changes to the *src* directory as appropriate.

## Graphviz dot

We're using [Graphviz](http://graphviz.org/) for some of the diagrams in the
book. The source for those files live in the `dot` directory. To turn a `dot`
file, for example, `dot/trpl04-01.dot` into an `svg`, run:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

In the generated SVG, remove the width and the height attributes from the `svg`
element and set the `viewBox` attribute to `0.00 0.00 1000.00 1000.00` or other
values that don't cut off the image.

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script. It needs a dictionary of valid words, which is provided in
`dictionary.txt`. If the script produces a false positive (say, you used word
`BTreeMap` which the script considers invalid), you need to add this word to
`dictionary.txt` (keep the sorted order for consistency).
