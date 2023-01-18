<!--
## Appendix B: Operators and Symbols
-->

## 付録 B: 演算子と記号

<!--
This appendix contains a glossary of Rust’s syntax, including operators and
other symbols that appear by themselves or in the context of paths, generics,
trait bounds, macros, attributes, comments, tuples, and brackets.
-->

この付録は、演算子や、単独で現れたり、パス、ジェネリクス、トレイト境界、マクロ、属性、コメント、タプル、
かっこの文脈で現れる他の記号を含む Rust の記法の用語集を含んでいます。

<!--
### Operators
-->

### 演算子

<!--
Table B-1 contains the operators in Rust, an example of how the operator would
appear in context, a short explanation, and whether that operator is
overloadable. If an operator is overloadable, the relevant trait to use to
overload that operator is listed.
-->

表 B-1 は、Rust の演算子、演算子が文脈で現れる例、短い説明、その演算子がオーバーロード可能かどうかを含んでいます。
演算子がオーバーロード可能ならば、オーバーロードするのに使用する関係のあるトレイトも列挙されています。

<!--
<span class="caption">Table B-1: Operators</span>
-->

<span class="caption">表 B-1: 演算子</span>

<!--
| Operator | Example | Explanation | Overloadable? |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | Macro expansion | |
| `!` | `!expr` | Bitwise or logical complement | `Not` |
| `!=` | `var != expr` | Nonequality comparison | `PartialEq` |
| `%` | `expr % expr` | Arithmetic remainder | `Rem` |
| `%=` | `var %= expr` | Arithmetic remainder and assignment | `RemAssign` |
| `&` | `&expr`, `&mut expr` | Borrow | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Borrowed pointer type | |
| `&` | `expr & expr` | Bitwise AND | `BitAnd` |
| `&=` | `var &= expr` | Bitwise AND and assignment | `BitAndAssign` |
| `&&` | `expr && expr` | Logical AND | |
| `*` | `expr * expr` | Arithmetic multiplication | `Mul` |
| `*=` | `var *= expr` | Arithmetic multiplication and assignment | `MulAssign` |
| `*` | `*expr` | Dereference | |
| `*` | `*const type`, `*mut type` | Raw pointer | |
| `+` | `trait + trait`, `'a + trait` | Compound type constraint | |
| `+` | `expr + expr` | Arithmetic addition | `Add` |
| `+=` | `var += expr` | Arithmetic addition and assignment | `AddAssign` |
| `,` | `expr, expr` | Argument and element separator | |
| `-` | `- expr` | Arithmetic negation | `Neg` |
| `-` | `expr - expr` | Arithmetic subtraction | `Sub` |
| `-=` | `var -= expr` | Arithmetic subtraction and assignment | `SubAssign` |
| `->` | `fn(...) -> type`, <code>\|...\| -> type</code> | Function and closure return type | |
| `.` | `expr.ident` | Member access | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | Right-exclusive range literal | |
| `..` | `..expr` | Struct literal update syntax | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “And the rest” pattern binding | |
| `...` | `expr...expr` | In a pattern: inclusive range pattern | |
| `/` | `expr / expr` | Arithmetic division | `Div` |
| `/=` | `var /= expr` | Arithmetic division and assignment | `DivAssign` |
| `:` | `pat: type`, `ident: type` | Constraints | |
| `:` | `ident: expr` | Struct field initializer | |
| `:` | `'a: loop {...}` | Loop label | |
| `;` | `expr;` | Statement and item terminator | |
| `;` | `[...; len]` | Part of fixed-size array syntax | |
| `<<` | `expr << expr` | Left-shift | `Shl` |
| `<<=` | `var <<= expr` | Left-shift and assignment | `ShlAssign` |
| `<` | `expr < expr` | Less than comparison | `PartialOrd` |
| `<=` | `expr <= expr` | Less than or equal to comparison | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | Assignment/equivalence | |
| `==` | `expr == expr` | Equality comparison | `PartialEq` |
| `=>` | `pat => expr` | Part of match arm syntax | |
| `>` | `expr > expr` | Greater than comparison | `PartialOrd` |
| `>=` | `expr >= expr` | Greater than or equal to comparison | `PartialOrd` |
| `>>` | `expr >> expr` | Right-shift | `Shr` |
| `>>=` | `var >>= expr` | Right-shift and assignment | `ShrAssign` |
| `@` | `ident @ pat` | Pattern binding | |
| `^` | `expr ^ expr` | Bitwise exclusive OR | `BitXor` |
| `^=` | `var ^= expr` | Bitwise exclusive OR and assignment | `BitXorAssign` |
| <code>\|</code> | <code>pat \| pat</code> | Pattern alternatives | |
| <code>\|</code> | <code>expr \| expr</code> | Bitwise OR | `BitOr` |
| <code>\|=</code> | <code>var \|= expr</code> | Bitwise OR and assignment | `BitOrAssign` |
| <code>\|\|</code> | <code>expr \|\| expr</code> | Logical OR | |
| `?` | `expr?` | Error propagation | |
-->

| 演算子             | 例                                               | 説明                      | オーバーロードできる？ |
|-------------------|--------------------------------------------------|--------------------------|---------------------|
| `!`               | `ident!(...)`, `ident!{...}`, `ident![...]`      | マクロ展開                ||
| `!`               | `!expr`                                          | ビット反転、または論理反転   | `Not` |
| `!=`              | `var != expr`                                    | 非等価比較                 | `PartialEq` |
| `%`               | `expr % expr`                                    | 余り演算                   | `Rem` |
| `%=`              | `var %= expr`                                    | 余り演算後に代入            | `RemAssign` |
| `&`               | `&expr`, `&mut expr`                             | 借用                      ||
| `&`               | `&type`, `&mut type`, `&'a type`, `&'a mut type` | 借用されたポインタ型        ||
| `&`               | `expr & expr`                                    | ビット AND                 | `BitAnd` |
| `&=`              | `var &= expr`                                    | ビット AND 後に代入          | `BitAndAssign` |
| `&&`              | `expr && expr`                                   | 論理 AND                   ||
| `*`               | `expr * expr`                                    | 掛け算                    | `Mul` |
| `*`               | `*expr`                                          | 参照外し                  ||
| `*`               | `*const type`, `*mut type`                       | 生ポインタ                ||
| `*=`              | `var *= expr`                                    | 掛け算後に代入             | `MulAssign` |
| `+`               | `trait + trait`, `'a + trait`                    | 型制限の複合化             ||
| `+`               | `expr + expr`                                    | 足し算                    | `Add` |
| `+=`              | `var += expr`                                    | 足し算後に代入             | `AddAssign` |
| `,`               | `expr, expr`                                     | 引数と要素の区別           ||
| `-`               | `- expr`                                         | 算術否定                  | `Neg` |
| `-`               | `expr - expr`                                    | 引き算                    | `Sub` |
| `-=`              | `var -= expr`                                    | 引き算後に代入             | `SubAssign` |
| `->`              | `fn(...) -> type`, <code>\|...\| -> type</code>  | 関数とクロージャの戻り値型   ||
| `.`               | `expr.ident`                                     | メンバーアクセス            ||
| `..`              | `..`, `expr..`, `..expr`, `expr..expr`           | 未満範囲リテラル            ||
| `..`              | `..expr`                                         | 構造体リテラル更新記法       ||
| `..`              | `variant(x, ..)`, `struct_type { x, .. }`        | 「残り全部」パターン束縛     ||
| `...`             | `expr...expr`                                    | パターンで：以下範囲パターン ||
| `/`               | `expr / expr`                                    | 割り算                    | `Div` |
| `/=`              | `var /= expr`                                    | 割り算後に代入             | `DivAssign` |
| `:`               | `pat: type`, `ident: type`                       | 型制約                    ||
| `:`               | `ident: expr`                                    | 構造体フィールド初期化子     ||
| `:`               | `'a: loop {...}`                                 | ループラベル               ||
| `;`               | `expr;`                                          | 文、要素終端子             ||
| `;`               | `[...; len]`                                     | 固定長配列記法の一部        ||
| `<<`              | `expr << expr`                                   | 左シフト                  | `Shl` |
| `<<=`             | `var <<= expr`                                   | 左シフト後に代入           | `ShlAssign` |
| `<`               | `expr < expr`                                    | 未満比較                  | `PartialOrd` |
| `<=`              | `expr <= expr`                                   | 以下比較                  | `PartialOrd` |
| `=`               | `var = expr`, `ident = type`                     | 代入/等価                 ||
| `==`              | `expr == expr`                                   | 等価比較                  | `PartialEq` |
| `=>`              | `pat => expr`                                    | match アーム記法の一部      ||
| `>`               | `expr > expr`                                    | より大きい比較             | `PartialOrd` |
| `>=`              | `expr >= expr`                                   | 以上比較                  | `PartialOrd` |
| `>>`              | `expr >> expr`                                   | 右シフト                  | `Shr` |
| `>>=`             | `var >>= expr`                                   | 右シフト後に代入           | `ShrAssign` |
| `@`               | `ident @ pat`                                    | パターン束縛              ||
| `^`               | `expr ^ expr`                                    | ビット XOR                | `BitXor` |
| `^=`              | `var ^= expr`                                    | ビット XOR 後に代入         | `BitXorAssign` |
| <code>\|</code>   | <code>pat \| pat</code>                          | パターン OR               ||
| <code>\|</code>   | <code>\|…\| expr</code>                          | クロージャ               ||
| <code>\|</code>   | <code>expr \| expr</code>                        | ビット OR                 | `BitOr` |
| <code>\|=</code>  | <code>var \|= expr</code>                        | ビット OR 後に代入          | `BitOrAssign`|
| <code>\|\|</code> | <code>expr \|\| expr</code>                      | 論理 OR                   ||
| `?`               | `expr?`                                          | エラー委譲                ||

<!--
### Non-operator Symbols
-->

### 演算子以外のシンボル

<!--
The following list contains all non-letters that don’t function as operators;
that is, they don’t behave like a function or method call.
-->

以下のリストは、演算子として機能しない記号全部を含んでいます; つまり、関数やメソッド呼び出しのようには、
振る舞わないということです。

<!--
Table B-2 shows symbols that appear on their own and are valid in a variety of
locations.
-->

表 B-2 は、単独で出現し、いろんな箇所で合法になる記号を示しています。

<!--
<span class="caption">Table B-2: Stand-Alone Syntax</span>
-->

<span class="caption">表 B-2: スタンドアローン記法</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `'ident` | Named lifetime or loop label |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Numeric literal of specific type |
| `"..."` | String literal |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Raw string literal, escape characters not processed |
| `b"..."` | Byte string literal; constructs a `[u8]` instead of a string |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Raw byte string literal, combination of raw and byte string literal |
| `'...'` | Character literal |
| `b'...'` | ASCII byte literal |
| <code>\|...\| expr</code> | Closure |
| `!` | Always empty bottom type for diverging functions |
| `_` | “Ignored” pattern binding; also used to make integer literals readable |
-->

| シンボル                                         | 説明  |
|-------------------------------------------------|------|
| `'ident`                                        | 名前付きのライフタイム、あるいはループラベル |
| `...u8`, `...i32`, `...f64`, `...usize`など     | 特定の型の数値リテラル |
| `"..."`                                         | 文字列リテラル|
| `r"..."`, `r#"..."#`, `r##"..."##`など          | 生文字列リテラル、エスケープ文字は処理されません |
| `b"..."`                                        | バイト文字列リテラル、文字列の代わりに`[u8]`を構築します |
| `br"..."`, `br#"..."#`, `br##"..."##`など       | 生バイト文字列リテラル、生文字列とバイト文字列の組み合わせ |
| `'...'`                                         | 文字リテラル |
| `b'...'`                                        | ASCII バイトリテラル |
| <code>\|...\| expr</code>                       | クロージャ |
| `!`                                             | 常に発散関数の空のボトム型 |
| `_`                                             | 「無視」パターン束縛：整数リテラルを見やすくするのにも使われる|

<!--
Table B-3 shows symbols that appear in the context of a path through the module
hierarchy to an item.
-->

表 B-3 は、要素へのモジュール階層を通したパスの文脈で出現する記号を示しています。

<!--
<span class="caption">Table B-3: Path-Related Syntax</span>
-->

<span class="caption">表 B-3: パス関連記法</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `ident::ident` | Namespace path |
| `::path` | Path relative to the crate root (i.e., an explicitly absolute path) |
| `self::path` | Path relative to the current module (i.e., an explicitly relative path).
| `super::path` | Path relative to the parent of the current module |
| `type::ident`, `<type as trait>::ident` | Associated constants, functions, and types |
| `<type>::...` | Associated item for a type that cannot be directly named (e.g., `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Disambiguating a method call by naming the trait that defines it |
| `type::method(...)` | Disambiguating a method call by naming the type for which it’s defined |
| `<type as trait>::method(...)` | Disambiguating a method call by naming the trait and type |
-->

| シンボル                                 | 説明 |
|-----------------------------------------|------|
| `ident::ident`                          | 名前空間パス |
| `::path`                                | クレートルートに相対的なパス (すなわち、明示的な絶対パス) |
| `self::path`                            | 現在のモジュールに相対的なパス (すなわち、明示的な相対パス) |
| `super::path`                           | 現在のモジュールの親モジュールに相対的なパス |
| `type::ident`, `<type as trait>::ident` | 関連定数、関数、型 |
| `<type>::...`                           | 直接名前付けできない型の関連要素 (例，`<&T>::...`, `<[T]>::...`など) |
| `trait::method(...)`                    | 定義したトレイトを名指ししてメソッド呼び出しを明確化する |
| `type::method(...)`                     | 定義されている型を名指ししてメソッド呼び出しを明確化する |
| `<type as trait>::method(...)`          | トレイト*と*型を名指ししてメソッド呼び出しを明確化する|

<!--
Table B-4 shows symbols that appear in the context of using generic type
parameters.
-->

表 B-4 は、ジェネリックな型引数の文脈で出現する記号を示しています。

<!--
<span class="caption">Table B-4: Generics</span>
-->

<span class="caption">表 B-4: ジェネリクス</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `path<...>` | Specifies parameters to generic type in a type (e.g., `Vec<u8>`) |
| `path::<...>`, `method::<...>` | Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish (e.g., `"42".parse::<i32>()`) |
| `fn ident<...> ...` | Define generic function |
| `struct ident<...> ...` | Define generic structure |
| `enum ident<...> ...` | Define generic enumeration |
| `impl<...> ...` | Define generic implementation |
| `for<...> type` | Higher-ranked lifetime bounds |
| `type<ident=type>` | A generic type where one or more associated types have specific assignments (e.g., `Iterator<Item=T>`) |
-->

| シンボル                        | 説明 |
|--------------------------------|-----|
| `path<...>`                    | 型の内部のジェネリック型への引数を指定する (例、`Vec<u8>`) |
| `path::<...>`, `method::<...>` | 式中のジェネリックな型、関数、メソッドへの引数を指定する。しばしばターボ・フィッシュ (turbofish) と称される。(例、`"42".parse::<i32>()`) |
| `fn ident<...> ...`            | ジェネリックな関数を定義する |
| `struct ident<...> ...`        | ジェネリックな構造体を定義する |
| `enum ident<...> ...`          | ジェネリックな列挙型を定義する |
| `impl<...> ...`                | ジェネリックな実装を定義する |
| `for<...> type`                | 高階ライフタイム境界 |
| `type<ident=type>`             | 1 つ以上の関連型に代入されたジェネリックな型 (例、`Iterator<Item=T>`) |

<!--
Table B-5 shows symbols that appear in the context of constraining generic type
parameters with trait bounds.
-->

表 B-5 は、ジェネリック型引数をトレイト境界で制約する文脈で出現する記号を示しています。

<!--
<span class="caption">Table B-5: Trait Bound Constraints</span>
-->

<span class="caption">表 B-5: トレイト境界制約</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `T: U` | Generic parameter `T` constrained to types that implement `U` |
| `T: 'a` | Generic type `T` must outlive lifetime `'a` (meaning the type cannot transitively contain any references with lifetimes shorter than `'a`) |
| `T : 'static` | Generic type `T` contains no borrowed references other than `'static` ones |
| `'b: 'a` | Generic lifetime `'b` must outlive lifetime `'a` |
| `T: ?Sized` | Allow generic type parameter to be a dynamically sized type |
| `'a + trait`, `trait + trait` | Compound type constraint |
-->

| シンボル                       | 説明 |
|-------------------------------|-----|
| `T: U`                        | `U`を実装する型に制約されるジェネリック引数`T` |
| `T: 'a`                       | ライフタイム`'a`よりも長生きしなければならないジェネリック型`T`(型がライフタイムより長生きするとは、`'a`よりも短いライフタイムの参照を何も遷移的に含められないことを意味する)|
| `T : 'static`                 | ジェネリック型`T`が`'static`なもの以外の借用された参照を何も含まない |
| `'b: 'a`                      | ジェネリックなライフタイム`'b`がライフタイム`'a`より長生きしなければならない |
| `T: ?Sized`                   | ジェネリック型引数が動的サイズ決定型であることを許容する |
| `'a + trait`, `trait + trait` | 複合型制約 |

<!--
Table B-6 shows symbols that appear in the context of calling or defining
macros and specifying attributes on an item.
-->

表 B-6 は、マクロの呼び出しや定義、要素に属性を指定する文脈で出現する記号を示しています。

<!--
<span class="caption">Table B-6: Macros and Attributes</span>
-->

<span class="caption">表 B-6: マクロと属性</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `#[meta]` | Outer attribute |
| `#![meta]` | Inner attribute |
| `$ident` | Macro substitution |
| `$ident:kind` | Macro capture |
| `$(…)…` | Macro repetition |
-->

| シンボル       | 説明 |
|---------------|-----|
| `#[meta]`     | 外部属性 |
| `#![meta]`    | 内部属性 |
| `$ident`      | マクロ代用 |
| `$ident:kind` | マクロキャプチャ |
| `$(…)…`       | マクロの繰り返し |

<!--
Table B-7 shows symbols that create comments.
-->

表 B-7 は、コメントを生成する記号を示しています。

<!--
<span class="caption">Table B-7: Comments</span>
-->

<span class="caption">表 B-7: コメント</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `//` | Line comment |
| `//!` | Inner line doc comment |
| `///` | Outer line doc comment |
| `/*...*/` | Block comment |
| `/*!...*/` | Inner block doc comment |
| `/**...*/` | Outer block doc comment |
-->

| シンボル    | 説明 |
|------------|-----|
| `//`       | 行コメント |
| `//!`      | 内部行 doc コメント |
| `///`      | 外部行 doc コメント |
| `/*...*/`  | ブロックコメント |
| `/*!...*/` | 内部ブロック doc コメント |
| `/**...*/` | 外部ブロック doc コメント |

<!--
#### Tuples
-->

#### タプル

<!--
Table B-8 shows symbols that appear in the context of using tuples.
-->

表 B-8 は、タプルの文脈で出現する記号を示しています。

<!--
<span class="caption">Table B-8: Tuples</span>
-->

<span class="caption">表 B-8: タプル</span>

<!--
| Symbol | Explanation |
|--------|-------------|
| `()` | Empty tuple (aka unit), both literal and type |
| `(expr)` | Parenthesized expression |
| `(expr,)` | Single-element tuple expression |
| `(type,)` | Single-element tuple type |
| `(expr, ...)` | Tuple expression |
| `(type, ...)` | Tuple type |
| `expr(expr, ...)` | Function call expression; also used to initialize tuple `struct`s and tuple `enum` variants |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Macro invocation |
| `expr.0`, `expr.1`, etc. | Tuple indexing |
-->

| シンボル                                     | 説明 |
|---------------------------------------------|-----|
| `()`                                        | 空のタプル (ユニットとしても知られる)、リテラル、型両方 |
| `(expr)`                                    | 括弧付きの式 |
| `(expr,)`                                   | 1 要素タプル式 |
| `(type,)`                                   | 1 要素タプル型 |
| `(expr, ...)`                               | タプル式 |
| `(type, ...)`                               | タプル型 |
| `expr(expr, ...)`                           | 関数呼び出し式; タプル`struct`やタプル`enum`列挙子を初期化するのにも使用される |
| `ident!(...)`, `ident!{...}`, `ident![...]` | マクロ呼び出し |
| `expr.0`, `expr.1`, など                     | タプル添え字アクセス |

<!--
Table B-9 shows the contexts in which curly braces are used.
-->

表 B-9 は、波括弧が使用される文脈を表示しています。

<!--
<span class="caption">Table B-9: Curly Brackets</span>
-->

<span class="caption">表 B-9: 波括弧</span>

<!--
| Context | Explanation |
|---------|-------------|
| `{...}` | Block expression |
| `Type {...}` | `struct` literal |
-->

| 文脈          | 説明 |
|--------------|------|
| `{...}`      | ブロック式 |
| `Type {...}` | `struct`リテラル |

<!--
Table B-10 shows the contexts in which square brackets are used.
-->

表 B-10 は、角括弧が使用される文脈を表示しています。

<!--
<span class="caption">Table B-10: Square Brackets</span>
-->

<span class="caption">表 B-10: 角括弧</span>

<!--
| Context | Explanation |
|---------|-------------|
| `[...]` | Array literal |
| `[expr; len]` | Array literal containing `len` copies of `expr` |
| `[type; len]` | Array type containing `len` instances of `type` |
| `expr[expr]` | Collection indexing. Overloadable (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or `RangeFull` as the “index” |
-->

| 文脈                                               | 説明 |
|----------------------------------------------------|------|
| `[...]`                                            | 配列リテラル |
| `[expr; len]`                                      | `len`個`expr`を含む配列リテラル |
| `[type; len]`                                      | `len`個の`type`のインスタンスを含む配列型 |
| `expr[expr]`                                       | コレクション添え字アクセス。オーバーロード可能 (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | `Range`、`RangeFrom`、`RangeTo`、`RangeFull`を「添え字」として使用してコレクション・スライシングの振りをするコレクション添え字アクセス |
