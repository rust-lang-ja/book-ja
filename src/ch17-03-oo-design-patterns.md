<!--
## Implementing an Object-Oriented Design Pattern
-->

## オブジェクト指向デザインパターンを実装する

<!--
The *state pattern* is an object-oriented design pattern. The crux of the
pattern is that a value has some internal state, which is represented by a set
of *state objects*, and the value’s behavior changes based on the internal
state. The state objects share functionality: in Rust, of course, we use
structs and traits rather than objects and inheritance. Each state object is
responsible for its own behavior and for governing when it should change into
another state. The value that holds a state object knows nothing about the
different behavior of the states or when to transition between states.
-->

ステートパターンは、オブジェクト指向デザインパターンの 1 つです。このパターンの肝は、
値が一連の*ステートオブジェクト*で表されるなんらかの内部状態を持ち、
その内部の状態に基づいて値の振る舞いが変化するというものです。ステートオブジェクトは、
機能を共有します：Rust では、もちろん、オブジェクトと継承ではなく、構造体とトレイトを使用します。
各ステートオブジェクトは、自身の振る舞いと別の状態に変化すべき時を司ることに責任を持ちます。
ステートオブジェクトを保持する値は、状態ごとの異なる振る舞いや、いつ状態が移行するかについては何も知りません。

<!--
Using the state pattern means when the business requirements of the program
change, we won’t need to change the code of the value holding the state or the
code that uses the value. We’ll only need to update the code inside one of the
state objects to change its rules or perhaps add more state objects. Let’s look
at an example of the state design pattern and how to use it in Rust.
-->

ステートパターンを使用することは、プログラムの業務要件が変わる時、状態を保持する値のコードや、
値を使用するコードを変更する必要はないことを意味します。ステートオブジェクトの 1 つのコードを更新して、
規則を変更したり、あるいはおそらくステートオブジェクトを追加する必要しかないのです。
ステートデザインパターンの例と、その Rust での使用方法を見ましょう。

<!--
We’ll implement a blog post workflow in an incremental way. The blog’s final
functionality will look like this:
-->

ブログ記事のワークフローを少しずつ実装していきます。ブログの最終的な機能は以下のような感じになるでしょう：

<!--
1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t
accidentally be published.
-->

1. ブログ記事は、空の草稿から始まる。
2. 草稿ができたら、査読が要求される。
3. 記事が承認されたら、公開される。
4. 公開されたブログ記事だけが表示する内容を返すので、未承認の記事は、誤って公開されない。

<!--
Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we’ve requested a review, the post
should remain an unpublished draft.
-->

それ以外の記事に対する変更は、効果を持つべきではありません。例えば、査読を要求する前にブログ記事の草稿を承認しようとしたら、
記事は、非公開の草稿のままになるべきです。

<!--
Listing 17-11 shows this workflow in code form: this is an example usage of the
API we’ll implement in a library crate named `blog`. This won’t compile yet
because we haven’t implemented the `blog` crate yet.
-->

リスト 17-11 は、このワークフローをコードの形で示しています：これは、
`blog`というライブラリクレートに実装する API の使用例です。まだ`blog`クレートを実装していないので、
コンパイルはできません。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<!--
<span class="caption">Listing 17-11: Code that demonstrates the desired
behavior we want our `blog` crate to have</span>
-->

<span class="caption">リスト 17-11: `blog`クレートに欲しい振る舞いをデモするコード</span>

<!--
We want to allow the user to create a new draft blog post with `Post::new`.
Then we want to allow text to be added to the blog post while it’s in the draft
state. If we try to get the post’s content immediately, before approval,
nothing should happen because the post is still a draft. We’ve added
`assert_eq!` in the code for demonstration purposes. An excellent unit test for
this would be to assert that a draft blog post returns an empty string from the
`content` method, but we’re not going to write tests for this example.
-->

ユーザが`Post::new`で新しいブログ記事の草稿を作成できるようにしたいです。それから、
草稿状態の間にブログ記事にテキストを追加できるようにしたいです。承認前に記事の内容を即座に得ようとしたら、
記事はまだ草稿なので、何も起きるべきではありません。デモ目的でコードに`assert_eq!`を追加しました。
これに対する素晴らしい単体テストは、ブログ記事の草稿が`content`メソッドから空の文字列を返すことをアサートすることでしょうが、
この例に対してテストを書くつもりはありません。

<!--
Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. When the post
receives approval, it should get published, meaning the text of the post will
be returned when `content` is called.
-->

次に、記事の査読を要求できるようにしたく、また査読を待機している間は`content`に空の文字列を返してほしいです。
記事が承認を受けたら、公開されるべきです。つまり、`content`を呼んだ時に記事のテキストが返されるということです。

<!--
Notice that the only type we’re interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in—draft, waiting for review, or published. Changing from one state to another
will be managed internally within the `Post` type. The states change in
response to the methods called by our library’s users on the `Post` instance,
but they don’t have to manage the state changes directly. Also, users can’t
make a mistake with the states, like publishing a post before it’s reviewed.
-->

クレートから相互作用している唯一の型は、`Post`だけであることに注意してください。
この型はステートパターンを使用し、記事がなり得る種々の状態を表す 3 つのステートオブジェクトのうちの 1 つになる値を保持します。
草稿、査読待ち、公開中です。1 つの状態から別の状態への変更は、`Post`型内部で管理されます。
`Post`インスタンスのライブラリ使用者が呼び出すメソッドに呼応して状態は変化しますが、
状態の変化を直接管理する必要はありません。また、ユーザは、
査読前に記事を公開するなど状態を誤ることはありません。

<!--
### Defining `Post` and Creating a New Instance in the Draft State
-->

### `Post`を定義し、草稿状態で新しいインスタンスを生成する

<!--
Let’s get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so we’ll start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 17-12. We’ll also make a private
`State` trait. Then `Post` will hold a trait object of `Box<State>` inside an
`Option` in a private field named `state`. You’ll see why the `Option` is
necessary in a bit.
-->

ライブラリの実装に取り掛かりましょう！なんらかの内容を保持する公開の`Post`構造体が必要なことはわかるので、
構造体の定義と、関連する公開の`Post`インスタンスを生成する`new`関数から始めましょう。リスト 17-12 のようにですね。
また、非公開の`State`トレイトも作成します。それから、`Post`は`state`という非公開のフィールドに、
`Option`で`Box<State>`のトレイトオブジェクトを保持します。`Option`が必要な理由はすぐわかります。

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

<!--
<span class="caption">Listing 17-12: Definition of a `Post` struct and a `new`
function that creates a new `Post` instance, a `State` trait, and a `Draft`
struct</span>
-->

<span class="caption">リスト 17-12: `Post`構造体、新規`Post`インスタンスを生成する`new`関数、
`State`トレイト、`Draft`構造体の定義</span>

<!--
The `State` trait defines the behavior shared by different post states, and the
`Draft`, `PendingReview`, and `Published` states will all implement the `State`
trait. For now, the trait doesn’t have any methods, and we’ll start by defining
just the `Draft` state because that is the state we want a post to start in.
-->

`State`トレイトは、異なる記事の状態で共有される振る舞いを定義し、`Draft`、`PendingReview`、`Published`状態は全て、
`State`トレイトを実装します。今は、トレイトにメソッドは何もなく、`Draft`が記事の初期状態にしたい状態なので、
その状態だけを定義することから始めます。

<!--
When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures whenever we create a new instance of `Post`, it will start out as a
draft. Because the `state` field of `Post` is private, there is no way to
create a `Post` in any other state! In the `Post::new` function, we set the
`content` field to a new, empty `String`
-->

新しい`Post`を作る時、`state`フィールドは、`Box`を保持する`Some`値にセットします。
この`Box`が`Draft`構造体の新しいインスタンスを指します。これにより、
新しい`Post`を作る度に、草稿から始まることが保証されます。`Post`の`state`フィールドは非公開なので、
`Post`を他の状態で作成する方法はないのです！`Post::new`関数では、`content`フィールドを新しい空の`String`にセットしています。

<!--
### Storing the Text of the Post Content
-->

### 記事の内容のテキストを格納する

<!--
Listing 17-11 showed that we want to be able to call a method named
`add_text` and pass it a `&str` that is then added to the text content of the
blog post. We implement this as a method rather than exposing the `content`
field as `pub`. This means we can implement a method later that will control
how the `content` field’s data is read. The `add_text` method is pretty
straightforward, so let’s add the implementation in Listing 17-13 to the `impl
Post` block:
-->

リスト 17-11 は、`add_text`というメソッドを呼び出し、ブログ記事のテキスト内容に追加される`&str`を渡せるようになりたいことを示しました。
これを`content`フィールドを`pub`にして晒すのではなく、メソッドとして実装しています。
これは、後ほど`content`フィールドデータの読まれ方を制御するメソッドを実装できることを意味しています。
`add_text`メソッドは非常に素直なので、リスト 17-13 の実装を`impl Post`ブロックに追加しましょう：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<!--
<span class="caption">Listing 17-13: Implementing the `add_text` method to add
text to a post’s `content`</span>
-->

<span class="caption">リスト 17-13: 記事の`content`にテキストを追加する`add_text`メソッドを実装する</span>

<!--
The `add_text` method takes a mutable reference to `self`, because we’re
changing the `Post` instance that we’re calling `add_text` on. We then call
`push_str` on the `String` in `content` and pass the `text` argument to add to
the saved `content`. This behavior doesn’t depend on the state the post is in,
so it’s not part of the state pattern. The `add_text` method doesn’t interact
with the `state` field at all, but it is part of the behavior we want to
support.
-->

`add_text`メソッドは、`self`への可変参照を取ります。というのも、`add_text`を呼び出した`Post`インスタンスを変更しているからです。
それから`content`の`String`に対して`push_str`を呼び出し、`text`引数を渡して保存された`content`に追加しています。
この振る舞いは、記事の状態によらないので、ステートパターンの一部ではありません。`add_text`メソッドは、
`state`フィールドと全く相互作用しませんが、サポートしたい振る舞いの一部ではあります。

<!--
### Ensuring the Content of a Draft Post Is Empty
-->

### 草稿の記事の内容は空であることを保証する

<!--
Even after we’ve called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice because the post is
still in the draft state, as shown on line 8 of Listing 17-11. For now, let’s
implement the `content` method with the simplest thing that will fulfill this
requirement: always returning an empty string slice. We’ll change this later
once we implement the ability to change a post’s state so it can be published.
So far, posts can only be in the draft state, so the post content should always
be empty. Listing 17-14 shows this placeholder implementation:
-->

`add_text`を呼び出して記事に内容を追加した後でさえ、記事はまだ草稿状態なので、
それでも`content`メソッドには空の文字列スライスを返してほしいです。
リスト 17-11 の 8 行目で示したようにですね。とりあえず、この要求を実現する最も単純な方法で`content`メソッドを実装しましょう：
常に空の文字列スライスを返すことです。一旦、記事の状態を変更する能力を実装したら、公開できるように、
これを後ほど変更します。ここまで、記事は草稿状態にしかなり得ないので、記事の内容は常に空のはずです。
リスト 17-14 は、この仮の実装を表示しています：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

<!--
<span class="caption">Listing 17-14: Adding a placeholder implementation for
the `content` method on `Post` that always returns an empty string slice</span>
-->

<span class="caption">リスト 17-14: `Post`に常に空の文字列スライスを返す`content`の仮の実装を追加する</span>

<!--
With this added `content` method, everything in Listing 17-11 up to line 8
works as intended.
-->

この追加された`content`メソッドとともに、リスト 17-11 の 8 行目までのコードは、想定通り動きます。

<!--
[Requesting a Review of the Post] Changes ... とも Requesting that [a Review ...] とも読める。どちらがふさわしいだろうか
次の 1 行目に request a review of a post, which とあるので、前者で訳しておく。
-->

<!--
### Requesting a Review of the Post Changes Its State
-->

### 記事の査読を要求すると、状態が変化する

<!--
Next, we need to add functionality to request a review of a post, which should
change its state from `Draft` to `PendingReview`. Listing 17-15 shows this code:
-->

次に、記事の査読を要求する機能を追加する必要があり、これをすると、状態が`Draft`から`PendingReview`に変わるはずです。
リスト 17-15 はこのコードを示しています：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<!--
<span class="caption">Listing 17-15: Implementing `request_review` methods on
`Post` and the `State` trait</span>
-->

<span class="caption">リスト 17-15: `Post`と`State`トレイトに`request_review`メソッドを実装する</span>

<!--
We give `Post` a public method named `request_review` that will take a mutable
reference to `self`. Then we call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method consumes the
current state and returns a new state.
-->

`Post`に`self`への可変参照を取る`request_review`という公開メソッドを与えます。それから、
`Post`の現在の状態に対して内部の`request_review`メソッドを呼び出し、
この 2 番目の`request_review`が現在の状態を消費し、新しい状態を返します。

<!--
We’ve added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, invalidating the old state so the state value of the
`Post` can transform into a new state.
-->

`State`トレイトに`request_review`メソッドを追加しました; このトレイトを実装する型は全て、
これで`request_review`メソッドを実装する必要があります。メソッドの第 1 引数に`self`、`&self`、`&mut self`ではなく、
`self: Box<Self>`としていることに注意してください。この記法は、型を保持する`Box`に対して呼ばれた時のみ、
このメソッドが合法になることを意味しています。この記法は、`Box<Self>`の所有権を奪い、古い状態を無効化するので、
`Post`の状態値は、新しい状態に変形できます。

<!--
To consume the old state, the `request_review` method needs to take ownership
of the state value. This is where the `Option` in the `state` field of `Post`
comes in: we call the `take` method to take the `Some` value out of the `state`
field and leave a `None` in its place, because Rust doesn’t let us have
unpopulated fields in structs. This lets us move the `state` value out of
`Post` rather than borrowing it. Then we’ll set the post’s `state` value to the
result of this operation.
-->

古い状態を消費するために、`request_review`メソッドは、状態値の所有権を奪う必要があります。
ここで`Post`の`state`フィールドの`Option`が問題になるのです：`take`メソッドを呼び出して、
`state`フィールドから`Some`値を取り出し、その箇所に`None`を残します。なぜなら、Rust は、
構造体に未代入のフィールドを持たせてくれないからです。これにより、借用するのではなく、
`Post`の`state`値をムーブすることができます。それから、記事の`state`値をこの処理の結果にセットするのです。

<!--
We need to set `state` to `None` temporarily rather than setting it directly
with code like `self.state = self.state.request_review();` to get ownership of
the `state` value. This ensures `Post` can’t use the old `state` value after
we’ve transformed it into a new state.
-->

`self.state = self.state.request_review();`のようなコードで直接`state`値の所有権を得るよう設定するのではなく、
一時的に`None`に`state`をセットする必要があります。これにより、新しい状態に変形した後に、
`Post`が古い`state`値を使えないことが保証されるのです。

<!--
The `request_review` method on `Draft` needs to return a new, boxed instance of
a new `PendingReview` struct, which represents the state when a post is waiting
for a review. The `PendingReview` struct also implements the `request_review`
method but doesn’t do any transformations. Rather, it returns itself, because
when we request a review on a post already in the `PendingReview` state, it
should stay in the `PendingReview` state.
-->

`Draft`の`request_review`メソッドは、新しい`PendingReview`構造体の新しいボックスのインスタンスを返す必要があり、
これが、記事が査読待ちの時の状態を表します。`PendingReview`構造体も`request_review`メソッドを実装しますが、
何も変形はしません。むしろ、自身を返します。というのも、既に`PendingReview`状態にある記事の査読を要求したら、
`PendingReview`状態に留まるべきだからです。

<!--
Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.
-->

ようやくステートパターンの利点が見えてき始めました：`state`値が何であれ、`Post`の`request_review`メソッドは同じです。
各状態は、独自の規則にのみ責任を持ちます。

<!--
We’ll leave the `content` method on `Post` as is, returning an empty string
slice. We can now have a `Post` in the `PendingReview` state as well as in the
`Draft` state, but we want the same behavior in the `PendingReview` state.
Listing 17-11 now works up to line 11!
-->

`Post`の`content`メソッドを空の文字列スライスを返してそのままにします。
これで`Post`は`PendingReview`と`Draft`状態になり得ますが、`PendingReview`状態でも、
同じ振る舞いが欲しいです。もうリスト 17-11 は 11 行目まで動くようになりました！

<!--
### Adding the `approve` Method that Changes the Behavior of `content`
-->

### `content`の振る舞いを変化させる`approve`メソッドを追加する

<!--
The `approve` method will be similar to the `request_review` method: it will
set `state` to the value that the current state says it should have when that
state is approved, as shown in Listing 17-16:
-->

`approve`メソッドは、`request_review`メソッドと類似するでしょう：状態が承認された時に、
現在の状態があるべきと言う値に`state`をセットします。リスト 17-16 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         Box::new(PendingReview {})
#     }
#
    // --snip--
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         self
#     }
#
    // --snip--
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<!--
<span class="caption">Listing 17-16: Implementing the `approve` method on
`Post` and the `State` trait</span>
-->

<span class="caption">リスト 17-16: `Post`と`State`トレイトに`approve`メソッドを実装する</span>

<!--
We add the `approve` method to the `State` trait and add a new struct that
implements `State`, the `Published` state.
-->

`State`トレイトに`approve`メソッドを追加し、`Published`状態という`State`を実装する新しい構造体を追加します。

<!--
Similar to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect because it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself, because
the post should stay in the `Published` state in those cases.
-->

`request_review`のように、`Draft`に対して`approve`メソッドを呼び出したら、`self`を返すので、
何も効果はありません。`PendingReview`に対して`approve`を呼び出すと、
`Published`構造体の新しいボックス化されたインスタンスを返します。`Published`構造体は`State`トレイトを実装し、
`request_review`メソッドと`approve`メソッド両方に対して、自身を返します。
そのような場合に記事は、`Published`状態に留まるべきだからです。

<!--
Now we need to update the `content` method on `Post`: if the state is
`Published`, we want to return the value in the post’s `content` field;
otherwise, we want to return an empty string slice, as shown in Listing 17-17:
-->

さて、`Post`の`content`メソッドを更新する必要が出てきました：状態が`Published`なら、
記事の`content`フィールドの値を返したいのです; それ以外なら、空の文字列スライスを返したいです。
リスト 17-17 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# trait State {
#     fn content<'a>(&self, post: &'a Post) -> &'a str;
# }
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // --snip--
}
```

<!--
<span class="caption">Listing 17-17: Updating the `content` method on `Post` to
delegate to a `content` method on `State`</span>
-->

<span class="caption">リスト 17-17: `Post`の`content`メソッドを更新して`State`の`content`メソッドに委譲する</span>

<!--
Because the goal is to keep all these rules inside the structs that implement
`State`, we call a `content` method on the value in `state` and pass the post
instance (that is, `self`) as an argument. Then we return the value that is
returned from using the `content` method on the `state` value.
-->

目的は、これらの規則全てを`State`を実装する構造体の内部に押し留めることなので、`state`の値に対して`content`メソッドを呼び出し、
記事のインスタンス (要するに、`self`) を引数として渡します。そして、`state`値の`content`メソッドを使用したことから返ってきた値を返します。

<!--
We call the `as_ref` method on the `Option` because we want a reference to the
value inside the `Option` rather than ownership of the value. Because `state`
is an `Option<Box<State>>`, when we call `as_ref`, an `Option<&Box<State>>` is
returned. If we didn’t call `as_ref`, we would get an error because we can’t
move `state` out of the borrowed `&self` of the function parameter.
-->

`Option`に対して`as_ref`メソッドを呼び出します。値の所有権ではなく、`Option`内部の値への参照が欲しいからです。
`state`は`Option<Box<State>>`なので、`as_ref`を呼び出すと、`Option<&Box<State>>`が返ってきます。
`as_ref`を呼ばなければ、`state`を関数引数の借用した`&self`からムーブできないので、エラーになるでしょう。

<!--
We then call the `unwrap` method, which we know will never panic, because we
know the methods on `Post` ensure that `state` will always contain a `Some`
value when those methods are done. This is one of the cases we talked about in
the “Cases When You Have More Information Than the Compiler” section of Chapter
9 when we know that a `None` value is never possible, even though the compiler
isn’t able to understand that.
-->

さらに`unwrap`メソッドを呼び出し、これは絶対にパニックしないことがわかっています。何故なら、
`Post`のメソッドが、それらのメソッドが完了した際に`state`は常に`Some`値を含んでいることを保証するからです。
これは、コンパイラには理解不能であるものの、
`None`値が絶対にあり得ないとわかる第 9 章の「コンパイラよりも情報を握っている場合」節で語った一例です。

<!--
At this point, when we call `content` on the `&Box<State>`, deref coercion will
take effect on the `&` and the `Box` so the `content` method will ultimately be
called on the type that implements the `State` trait. That means we need to add
`content` to the `State` trait definition, and that is where we’ll put the
logic for what content to return depending on which state we have, as shown in
Listing 17-18:
-->

この時点で、`&Box<State>`に対して`content`を呼び出すと、参照外し型強制が`&`と`Box`に働くので、
究極的に`content`メソッドが`State`トレイトを実装する型に対して呼び出されることになります。
つまり、`content`を`State`トレイト定義に追加する必要があり、そこが現在の状態に応じてどの内容を返すべきかというロジックを配置する場所です。
リスト 17-18 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     content: String
# }
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

<!--
<span class="caption">Listing 17-18: Adding the `content` method to the `State`
trait</span>
-->

<span class="caption">リスト 17-18: `State`トレイトに`content`メソッドを追加する</span>

<!--
We add a default implementation for the `content` method that returns an empty
string slice. That means we don’t need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`.
-->

空の文字列スライスを返すデフォルト実装を`content`メソッドに追加しています。これにより、
`Draft`と`PendingReview`構造体に`content`を実装する必要はありません。`Published`構造体は、
`content`メソッドをオーバーライドし、`post.content`の値を返します。

<!--
Note that we need lifetime annotations on this method, as we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument and returning a
reference to part of that `post`, so the lifetime of the returned reference is
related to the lifetime of the `post` argument.
-->

第 10 章で議論したように、このメソッドにはライフタイム注釈が必要なことに注意してください。
`post`への参照を引数として取り、その`post`の一部への参照を返しているので、
返却される参照のライフタイムは、`post`引数のライフタイムに関連します。

<!--
And we’re done—all of Listing 17-11 now works! We’ve implemented the state
pattern with the rules of the blog post workflow. The logic related to the
rules lives in the state objects rather than being scattered throughout `Post`.
-->

出来上がりました。要するに、リスト 17-11 はもう動くようになったのです！ブログ記事ワークフローの規則でステートパターンを実装しました。
その規則に関連するロジックは、`Post`中に散乱するのではなく、ステートオブジェクトに息づいています。

<!--
### Trade-offs of the State Pattern
-->

### ステートパターンの代償

<!--
The way で「・・・の仕方によれば」という意味になるらしい
-->

<!--
We’ve shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the various behaviors. The
way we organized the code, we have to look in only one place to know the
different ways a published post can behave: the implementation of the `State`
trait on the `Published` struct.
-->

オブジェクト指向のステートパターンを実装して各状態の時に記事がなり得る異なる種類の振る舞いをカプセル化する能力が、
Rust にあることを示してきました。`Post`のメソッドは、種々の振る舞いについては何も知りません。
コードを体系化する仕方によれば、公開された記事が振る舞うことのある様々な方法を知るには、1 箇所のみを調べればいいのです：
`Published`構造体の`State`トレイトの実装です。

<!--
FIX: 5 行目末尾がよくわからない。The more ..., the more ...のような感じで訳し文脈には合ってそうだが、合ってる自信がない
つまり、This would only increase, the more states we added のように訳している
-->

<!--
If we were to create an alternative implementation that didn’t use the state
pattern, we might instead use `match` expressions in the methods on `Post` or
even in the `main` code that checks the state of the post and changes behavior
in those places. That would mean we would have to look in several places to
understand all the implications of a post being in the published state! This
would only increase the more states we added: each of those `match` expressions
would need another arm.
-->

ステートパターンを使用しない対立的な実装を作ることになったら、代わりに`Post`のメソッドか、
あるいは記事の状態を確認し、それらの箇所 (`編注`: `Post`のメソッドのことか) の振る舞いを変更する`main`コードでさえ、
`match`式を使用したかもしれません。そうなると、複数個所を調べて記事が公開状態にあることの裏の意味全てを理解しなければならなくなります！
これは、追加した状態が増えれば、さらに上がるだけでしょう：各`match`式には、別のアームが必要になるのです。

<!--
With the state pattern, the `Post` methods and the places we use `Post` don’t
need `match` expressions, and to add a new state, we would only need to add a
new struct and implement the trait methods on that one struct.
-->

ステートパターンでは、`Post`のメソッドと`Post`を使用する箇所で、`match`式が必要になることはなく、
新しい状態を追加するのにも、新しい構造体を追加し、その 1 つの構造体にトレイトメソッドを実装するだけでいいわけです。

<!--
The implementation using the state pattern is easy to extend to add more
functionality. To see the simplicity of maintaining code that uses the state
pattern, try a few of these suggestions:
-->

ステートパターンを使用した実装は、拡張して機能を増やすことが容易です。
ステートパターンを使用するコードの管理の単純さを確認するために、以下の提言を試してみてください：

<!--
* Add a `reject` method that changes the post’s state from `PendingReview` back
to `Draft`.
* Require two calls to `approve` before the state can be changed to `Published`.
* Allow users to add text content only when a post is in the `Draft` state.
Hint: have the state object responsible for what might change about the
content but not responsible for modifying the `Post`.
-->

* 記事の状態を`PendingReview`から`Draft`に戻す`reject`メソッドを追加する。
* 状態が`Published`に変化させられる前に`approve`を 2 回呼び出す必要があるようにする。
* 記事が`Draft`状態の時のみテキスト内容をユーザが追加できるようにする。
  ヒント：ステートオブジェクトに内容について変わる可能性のあるものの責任を持たせつつも、
  `Post`を変更することには責任を持たせない。

<!--
One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` didn’t need to
change with the addition of a new state, but that would mean switching to
another design pattern.
-->

ステートパターンの欠点の 1 つは、状態が状態間の遷移を実装しているので、状態の一部が密に結合した状態になってしまうことです。
`PendingReview`と`Published`の間に、`Scheduled`のような別の状態を追加したら、
代わりに`PendingReview`のコードを`Scheduled`に遷移するように変更しなければならないでしょう。
状態が追加されても`PendingReview`を変更する必要がなければ、作業が減りますが、
そうすれば別のデザインパターンに切り替えることになるでしょう。

<!--
Another downside is that we’ve duplicated some logic. To eliminate some of the
duplication, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`;
however, this would violate object safety, because the trait doesn’t know what
the concrete `self` will be exactly. We want to be able to use `State` as a
trait object, so we need its methods to be object safe.
-->

別の欠点は、ロジックの一部を重複させてしまうことです。重複を除くためには、
`State`トレイトの`request_review`と`approve`メソッドに`self`を返すデフォルト実装を試みる可能性があります;
ですが、これはオブジェクト安全性を侵害するでしょう。というのも、具体的な`self`が一体なんなのかトレイトには知りようがないからです。
`State`をトレイトオブジェクトとして使用できるようにしたいので、メソッドにはオブジェクト安全になってもらう必要があるのです。

<!--
Other duplication includes the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods delegate to the implementation of
the same method on the value in the `state` field of `Option` and set the new
value of the `state` field to the result. If we had a lot of methods on `Post`
that followed this pattern, we might consider defining a macro to eliminate the
repetition (see Appendix D for more on Macros).
-->

他の重複には、`Post`の`request_review`と`approve`メソッドの実装が似ていることが含まれます。
両メソッドは`Option`の`state`の値に対する同じメソッドの実装に委譲していて、`state`フィールドの新しい値を結果にセットします。
このパターンに従う`Post`のメソッドが多くあれば、マクロを定義して繰り返しを排除することも考慮する可能性があります (マクロについては付録 D を参照)。

<!--
By implementing the state pattern exactly as it’s defined for object-oriented
languages, we’re not taking as full advantage of Rust’s strengths as we could.
Let’s look at some changes we can make to the `blog` crate that can make
invalid states and transitions into compile time errors.
-->

オブジェクト指向言語で定義されている通り忠実にステートパターンを実装することで、
Rust の強みをできるだけ発揮していません。`blog`クレートに対して行える無効な状態と遷移をコンパイルエラーにできる変更に目を向けましょう。

<!--
#### Encoding States and Behavior as Types
-->

#### 状態と振る舞いを型としてコード化する

<!--
We’ll show you how to rethink the state pattern to get a different set of
trade-offs. Rather than encapsulating the states and transitions completely so
outside code has no knowledge of them, we’ll encode the states into different
types. Consequently, Rust’s type checking system will prevent attempts to use
draft posts where only published posts are allowed by issuing a compiler error.
-->

ステートパターンを再考して別の代償を得る方法をお見せします。状態と遷移を完全にカプセル化して、
外部のコードに知らせないようにするよりも、状態を異なる型にコード化します。結果的に、
Rust の型検査システムが、公開記事のみが許可される箇所で草稿記事の使用を試みることをコンパイルエラーを発して阻止します。

<!--
Let’s consider the first part of `main` in Listing 17-11:
-->

リスト 17-11 の`main`の最初の部分を考えましょう：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

<!--
We still enable the creation of new posts in the draft state using `Post::new`
and the ability to add text to the post’s content. But instead of having a
`content` method on a draft post that returns an empty string, we’ll make it so
draft posts don’t have the `content` method at all. That way, if we try to get
a draft post’s content, we’ll get a compiler error telling us the method
doesn’t exist. As a result, it will be impossible for us to accidentally
display draft post content in production, because that code won’t even compile.
Listing 17-19 shows the definition of a `Post` struct and a `DraftPost` struct,
as well as methods on each:
-->

それでも、`Post::new`で草稿状態の新しい記事を生成することと記事の内容にテキストを追加する能力は可能にします。
しかし、空の文字列を返す草稿記事の`content`メソッドを保持する代わりに、草稿記事は、
`content`メソッドを全く持たないようにします。そうすると、草稿記事の内容を得ようとしたら、
メソッドが存在しないというコンパイルエラーになるでしょう。その結果、
誤ってプロダクションコードで草稿記事の内容を表示することが不可能になります。
そのようなコードは、コンパイルさえできないからです。リスト 17-19 は`Post`構造体、`DraftPost`構造体、
さらにメソッドの定義を示しています：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<!--
<span class="caption">Listing 17-19: A `Post` with a `content` method and a
`DraftPost` without a `content` method</span>
-->

<span class="caption">リスト 17-19: `content`メソッドのある`Post`と`content`メソッドのない`DraftPost`</span>

<!--
Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field because
we’re moving the encoding of the state to the types of the structs. The `Post`
struct will represent a published post, and it has a `content` method that
returns the `content`.
-->

`Post`と`DraftPost`構造体どちらにもブログ記事のテキストを格納する非公開の`content`フィールドがあります。
状態のコード化を構造体の型に移動したので、この構造体は最早`state`フィールドを持ちません。
`Post`は公開された記事を表し、`content`を返す`content`メソッドがあります。

<!--
We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. Because `content` is private
and there aren’t any functions that return `Post`, it’s not possible to create
an instance of `Post` right now.
-->

それでも`Post::new`関数はありますが、`Post`のインスタンスを返すのではなく、`DraftPost`のインスタンスを返します。
`content`は非公開であり、`Post`を返す関数も存在しないので、現状`Post`のインスタンスを生成することは不可能です。

<!--
The `DraftPost` struct has an `add_text` method, so we can add text to
`content` as before, but note that `DraftPost` does not have a `content` method
defined! So now the program ensures all posts start as draft posts, and draft
posts don’t have their content available for display. Any attempt to get around
these constraints will result in a compiler error.
-->

`DraftPost`構造体には、以前のようにテキストを`content`に追加できるよう`add_text`メソッドがありますが、
`DraftPost`には`content`メソッドが定義されていないことに注目してください！
従って、これでプログラムは、全ての記事が草稿記事から始まり、草稿記事は表示できる内容がないことを保証します。
この制限をかいくぐる試みは、全てコンパイルエラーに落ち着くでしょう。

<!--
#### Implementing Transitions as Transformations into Different Types
-->

#### 遷移を異なる型への変形として実装する

<!--
1 行目 the rule that は同格で訳しているが、the rule that a draft post has (the rule) to be...でも意味は通りそう
-->

<!--
So how do we get a published post? We want to enforce the rule that a draft
post has to be reviewed and approved before it can be published. A post in the
pending review state should still not display any content. Let’s implement
these constraints by adding another struct, `PendingReviewPost`, defining the
`request_review` method on `DraftPost` to return a `PendingReviewPost`, and
defining an `approve` method on `PendingReviewPost` to return a `Post`, as
shown in Listing 17-20:
-->

では、どうやって公開された記事を得るのでしょうか？公開される前に草稿記事は査読され、
承認されなければならないという規則を強制したいです。査読待ち状態の記事は、それでも内容を表示するべきではありません。
別の構造体`PendingReviewPost`を追加し、`DraftPost`に`PendingReviewPost`を返す`request_review`メソッドを定義し、
`PendingReviewPost`に`Post`を返す`approve`メソッドを定義してこれらの制限を実装しましょう。リスト 17-20 のようにですね：

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">ファイル名：src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
# pub struct DraftPost {
#     content: String,
# }
#
impl DraftPost {
    // --snip--

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

<!--
<span class="caption">Listing 17-20: A `PendingReviewPost` that gets created by
calling `request_review` on `DraftPost` and an `approve` method that turns a
`PendingReviewPost` into a published `Post`</span>
-->

<span class="caption">リスト 17-20: `DraftPost`の`request_review`を呼び出すことで生成される`PendingReviewPost`と、
`PendingReviewPost`を公開された`Post`に変換する`approve`メソッド</span>

<!--
The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any lingering `DraftPost` instances after we’ve called
`request_review` on them, and so forth. The `PendingReviewPost` struct doesn’t
have a `content` method defined on it, so attempting to read its content
results in a compiler error, as with `DraftPost`. Because the only way to get a
published `Post` instance that does have a `content` method defined is to call
the `approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.
-->

`request_review`と`approve`メソッドは`self`の所有権を奪い、故に`DraftPost`と`PendingReviewPost`インスタンスを消費し、
それぞれ`PendingReviewPost`と公開された`Post`に変形します。このように、
`DraftPost`インスタンスに`request_review`を呼んだ後には、`DraftPost`インスタンスは生きながらえず、
以下同様です。`PendingReviewPost`構造体には、`content`メソッドが定義されていないので、
`DraftPost`同様に、その内容を読もうとするとコンパイルエラーに落ち着きます。
`content`メソッドが確かに定義された公開された`Post`インスタンスを得る唯一の方法が、
`PendingReviewPost`に対して`approve`を呼び出すことであり、`PendingReviewPost`を得る唯一の方法が、
`DraftPost`に`request_review`を呼び出すことなので、これでブログ記事のワークフローを型システムにコード化しました。

<!--
But we also have to make some small changes to `main`. The `request_review` and
`approve` methods return new instances rather than modifying the struct they’re
called on, so we need to add more `let post =` shadowing assignments to save
the returned instances. We also can’t have the assertions about the draft and
pending review post’s contents be empty strings, nor do we need them: we can’t
compile code that tries to use the content of posts in those states any longer.
The updated code in `main` is shown in Listing 17-21:
-->

ですが、さらに`main`にも多少小さな変更を行わなければなりません。`request_review`と`approve`メソッドは、
呼ばれた構造体を変更するのではなく、新しいインスタンスを返すので、`let post =`というシャドーイング代入をもっと追加し、
返却されたインスタンスを保存する必要があります。また、草稿と査読待ち記事の内容を空の文字列でアサートすることも、
する必要もありません：最早、その状態にある記事の内容を使用しようとするコードはコンパイル不可能だからです。
`main`の更新されたコードは、リスト 17-21 に示されています：

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名：src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<!--
<span class="caption">Listing 17-21: Modifications to `main` to use the new
implementation of the blog post workflow</span>
-->

<span class="caption">リスト 17-21: ブログ記事ワークフローの新しい実装を使う`main`の変更</span>

<!--
The changes we needed to make to `main` to reassign `post` mean that this
implementation doesn’t quite follow the object-oriented state pattern anymore:
the transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as display of the content of
an unpublished post, will be discovered before they make it to production.
-->

`post`を再代入するために`main`に行う必要のあった変更は、この実装がもう、
全くオブジェクト指向のステートパターンに沿っていないことを意味します：
状態間の変形は最早、`Post`実装内に完全にカプセル化されていません。
ですが、型システムとコンパイル時に起きる型チェックのおかげでもう無効な状態があり得なくなりました。
これにより、未公開の記事の内容が表示されるなどの特定のバグが、プロダクションコードに移る前に発見されることが保証されます。

<!--
特に 2 行目中盤、as it is after がよくわからない
as it is, after で訳した
-->

<!--
Try the tasks suggested for additional requirements that we mentioned at the
start of this section on the `blog` crate as it is after Listing 17-20 to see
what you think about the design of this version of the code. Note that some of
the tasks might be completed already in this design.
-->

`blog`クレートに関してこの節の冒頭で触れた追加の要求に提言される作業をそのままリスト 17-20 の後に試してみて、
このバージョンのコードについてどう思うか確かめてください。この設計では、
既に作業の一部が達成されている可能性があることに注意してください。

<!--
We’ve seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns, such as encoding state into the type system,
are also available in Rust. These patterns have different trade-offs. Although
you might be very familiar with object-oriented patterns, rethinking the
problem to take advantage of Rust’s features can provide benefits, such as
preventing some bugs at compile time. Object-oriented patterns won’t always be
the best solution in Rust due to certain features, like ownership, that
object-oriented languages don’t have.
-->

Rust は、オブジェクト指向のデザインパターンを実装する能力があるものの、状態を型システムにコード化するなどの他のパターンも、
Rust では利用可能なことを確かめました。これらのパターンには、異なる代償があります。
あなたが、オブジェクト指向のパターンには非常に馴染み深い可能性があるものの、問題を再考して Rust の機能の強みを活かすと、
コンパイル時に一部のバグを回避できるなどの利益が得られることもあります。オブジェクト指向のパターンは、
オブジェクト指向言語にはない所有権などの特定の機能により Rust では、必ずしも最善の解決策ではないでしょう。

<!--
## Summary
-->

## まとめ

<!--
No matter whether or not you think Rust is an object-oriented language after
reading this chapter, you now know that you can use trait objects to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. You can use this
flexibility to implement object-oriented patterns that can help your code’s
maintainability. Rust also has other features, like ownership, that
object-oriented languages don’t have. An object-oriented pattern won’t always
be the best way to take advantage of Rust’s strengths, but is an available
option.
-->

この章読了後に、あなたが Rust はオブジェクト指向言語であると考えるかどうかに関わらず、
もうトレイトオブジェクトを使用して Rust でオブジェクト指向の機能の一部を得ることができると知っています。
ダイナミックディスパッチは、多少の実行時性能と引き換えにコードに柔軟性を<ruby>齎<rp>(</rp><rt>もたら</rt><rp>)</rp></ruby>してくれます。
この柔軟性を利用してコードのメンテナンス性に寄与するオブジェクト指向パターンを実装することができます。
Rust にはまた、オブジェクト指向言語にはない所有権などの他の機能もあります。オブジェクト指向パターンは、
必ずしも Rust の強みを活かす最善の方法にはなりませんが、利用可能な選択肢の 1 つではあります。

<!--
Next, we’ll look at patterns, which are another of Rust’s features that enable
lots of flexibility. We’ve looked at them briefly throughout the book but
haven’t seen their full capability yet. Let’s go!
-->

次は、パターンを見ます。パターンも多くの柔軟性を可能にする Rust の別の機能です。
本全体を通して僅かに見かけましたが、まだその全能力は目の当たりにしていません。さあ、行きましょう！
