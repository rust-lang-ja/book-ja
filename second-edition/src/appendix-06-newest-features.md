<!-- # Appendix F - Newest Features -->

# 付録F: 最新の機能

<!-- This appendix documents features that have been added to stable Rust since the -->
<!-- main part of the book was completed. -->

この付録は、本の主な部分が完成してから安定版Rustに追加された機能をドキュメント化しています。


<!-- ## Field init shorthand -->

## フィールド初期化省略

<!-- We can initialize a data structure (struct, enum, union) with named -->
<!-- fields, by writing `fieldname` as a shorthand for `fieldname: fieldname`. -->
<!-- This allows a compact syntax for initialization, with less duplication: -->

`fieldname`を`fieldname: fieldname`の省略として記述することでデータ構造(構造体、enum、ユニオン)を名前付きのフィールドで、
初期化することができます。これにより、重複を減らし、コンパクトな記法の初期化が許容されます。

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // フル記法:
    // Using full syntax:
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // フィールド初期化省略:
    // Using field init shorthand:
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```


<!-- ## Returning from loops -->

## ループから戻る

<!-- One of the uses of a `loop` is to retry an operation you know can fail, such as -->
<!-- checking if a thread completed its job. However, you might need to pass the -->
<!-- result of that operation to the rest of your code. If you add it to the `break` -->
<!-- expression you use to stop the loop, it will be returned by the broken loop: -->

`loop`の1つの使用法は、スレッドが仕事を終えたか確認するなど、失敗する可能性のあることを知っている処理を再試行することです。
ですが、その処理の結果を残りのコードに渡す必要がある可能性があります。それをループを停止させるために使用する`break`式に追加したら、
breakしたループから返ってきます。

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

<!-- ## Nested groups in `use` declarations -->

## `use`宣言のネストされたグループ

<!-- If you have a complex module tree with many different submodules and you need -->
<!-- to import a few items from each one, it might be useful to group all the -->
<!-- imports in the same declaration to keep your code clean and avoid repeating the -->
<!-- base modules’ name. -->

多くの異なるサブモジュールがある複雑なモジュール木があり、それぞれからいくつかの要素をインポートする必要があるなら、
同じ宣言の全インポートをグループ化し、コードを綺麗に保ち、ベースモジュールの名前を繰り返すのを回避するのが有用になる可能性があります。

<!-- The `use` declaration supports nesting to help you in those cases, both with -->
<!-- simple imports and glob ones. For example this snippets imports `bar`, `Foo`, -->
<!-- all the items in `baz` and `Bar`: -->

`use`宣言は、単純なインポートとグロブを使用したもの両方に対して、そのような場合に手助けになるネストをサポートしています。
例を挙げれば、このコード片は、`bar`、`Foo`、`baz`の全要素、`Bar`をインポートします。

```rust
# #![allow(unused_imports, dead_code)]
#
# mod foo {
#     pub mod bar {
#         pub type Foo = ();
#     }
#     pub mod baz {
#         pub mod quux {
#             pub type Bar = ();
#         }
#     }
# }
#
use foo::{
    bar::{self, Foo},
    baz::{*, quux::Bar},
};
#
# fn main() {}
```

<!-- ## Inclusive ranges -->

## 境界を含む範囲

<!-- Previously, when a range (`..` or `...`) was used as an expression, it had to be -->
<!-- `..`, which is exclusive of the upper bound, while patterns had to use `...`, -->
<!-- which is inclusive of the upper bound. Now, `..=` is accepted as syntax for -->
<!-- inclusive ranges in both expression and range context: -->

以前は、範囲を式として使用する際、`..`でなければならず、これは上限を含まない一方、パターンは`...`を使用しなければならず、
これは、上限を含みます。現在では、`..=`が両方の式と範囲の文脈で上限を含む範囲の記法として受け付けられます。

```rust
fn main() {
    for i in 0 ..= 10 {
        match i {
            0 ..= 5 => println!("{}: low", i),
            6 ..= 10 => println!("{}: high", i),
            _ => println!("{}: out of range", i),
        }
    }
}
```

<!-- The `...` syntax is still accepted in matches, but it is not accepted in -->
<!-- expressions. `..=` should be preferred. -->

`...`記法はそれでも、matchでは受け付けられますが、式では受け付けられません。`..=`を使用すべきです。

<!-- ## 128-bit integers -->

## 128ビット整数

<!-- Rust 1.26.0 added 128-bit integer primitives: -->

Rust1.26.0で128ビットの整数が追加されました:

<!-- - `u128`: A 128-bit unsigned integer with range [0, 2^128 - 1] -->
<!-- - `i128`: A 128-bit signed integer with range [-(2^127), 2^127 - 1] -->

- `u128`: 範囲[0, 2^128 - 1]の128ビットの非負整数
- `i128`: 範囲[-(2^127), 2^127 - 1]の128ビットの符号付き整数

<!-- These primitives are implemented efficiently via LLVM support. They are -->
<!-- available even on platforms that don’t natively support 128-bit integers and -->
<!-- can be used like the other integer types. -->

これらの基本型は、LLVMサポート経由で効率的に実装されています。ネイティブに128ビット整数をサポートしないプラットフォームですら利用可能で、
他の整数型のように使用できます。

<!-- These primitives can be very useful for algorithms that need to use very large -->
<!-- integers efficiently, such as certain cryptographic algorithms. -->

これらの基本型は、特定の暗号化アルゴリズムなど、非常に大きな整数を効率的に使用する必要のあるアルゴリズムで、とても有用です。
