<!-- ## Appendix B: Operators and Symbols -->

## 付録B: 演算子とシンボル

<!-- This appendix contains a glossary of Rust’s syntax, including operators and -->
<!-- other symbols that appear by themselves or in the context of paths, generics, -->
<!-- trait bounds, macros, attributes, comments, tuples, and brackets. -->

この付録は、演算子や、単独で現れたり、パス、ジェネリクス、トレイト境界、マクロ、アトリビュート、コメント、タプル、
かっこの文脈で現れる他のシンボルを含むRustの記法の用語集を含んでいます。

<!-- ### Operators -->

### 演算子

<!-- The following list contains the operators in Rust, an example of how the -->
<!-- operator would appear in context, a short explanation, and whether that -->
<!-- operator is overloadable. If an operator is overloadable, the relevant trait to -->
<!-- use to overload that operator is listed. -->

以下のリストは、Rustの演算子、演算子が文脈で現れる例、短い説明、その演算子がオーバーロード可能かどうかを含んでいます。
演算子がオーバーロード可能ならば、オーバーロードするのに使用する関係のあるトレイトも列挙されています。


<!-- * `!` (`ident!(...)`, `ident!{...}`, `ident![...]`): denotes macro -->
<!-- expansion. -->
<!-- * `!` (`!expr`): bitwise or logical complement. Overloadable (`Not`). -->
<!-- * `!=` (`var != expr`): nonequality comparison. Overloadable (`PartialEq`). -->
<!-- * `%` (`expr % expr`): arithmetic remainder. Overloadable (`Rem`). -->
<!-- * `%=` (`var %= expr`): arithmetic remainder and assignment. Overloadable -->
<!-- (`RemAssign`). -->
<!-- * `&` (`&expr`, `&mut expr`): borrow. -->
<!-- * `&` (`&type`, `&mut type`, `&'a type`, `&'a mut type`): borrowed pointer type. -->
<!-- * `&` (`expr & expr`): bitwise AND. Overloadable (`BitAnd`). -->
<!-- * `&=` (`var &= expr`): bitwise AND and assignment. Overloadable -->
<!-- (`BitAndAssign`). -->
<!-- * `&&` (`expr && expr`): logical AND. -->
<!-- * `*` (`expr * expr`): arithmetic multiplication. Overloadable (`Mul`). -->
<!-- * `*` (`*expr`): dereference. -->
<!-- * `*` (`*const type`, `*mut type`): raw pointer. -->
<!-- * `*=` (`var *= expr`): arithmetic multiplication and assignment. Overloadable -->
<!-- (`MulAssign`). -->
<!-- * `+` (`trait + trait`, `'a + trait`): compound type constraint. -->
<!-- * `+` (`expr + expr`): arithmetic addition. Overloadable (`Add`). -->
<!-- * `+=` (`var += expr`): arithmetic addition and assignment. Overloadable -->
<!-- (`AddAssign`). -->
<!-- * `,`: argument and element separator. -->
<!-- * `-` (`- expr`): arithmetic negation. Overloadable (`Neg`). -->
<!-- * `-` (`expr - expr`): arithmetic subtraction. Overloadable (`Sub`). -->
<!-- * `-=` (`var -= expr`): arithmetic subtraction and assignment. Overloadable -->
<!-- (`SubAssign`). -->
<!-- * `->` (`fn(...) -> type`, `|...| -> type`): function and closure -->
<!-- return type. -->
<!-- * `.` (`expr.ident`): member access. -->
<!-- * `..` (`..`, `expr..`, `..expr`, `expr..expr`): right-exclusive range literal. -->
<!-- * `..` (`..expr`): struct literal update syntax. -->
<!-- * `..` (`variant(x, ..)`, `struct_type { x, .. }`): “and the rest” pattern -->
<!-- binding. -->
<!-- * `...` (`expr...expr`) *in a pattern*: inclusive range pattern. -->
<!-- * `/` (`expr / expr`): arithmetic division. Overloadable (`Div`). -->
<!-- * `/=` (`var /= expr`): arithmetic division and assignment. Overloadable -->
<!-- (`DivAssign`). -->
<!-- * `:` (`pat: type`, `ident: type`): constraints. -->
<!-- * `:` (`ident: expr`): struct field initializer. -->
<!-- * `:` (`'a: loop {...}`): loop label. -->
<!-- * `;`: statement and item terminator. -->
<!-- * `;` (`[...; len]`): part of fixed-size array syntax -->
<!-- * `<<` (`expr << expr`): left-shift. Overloadable (`Shl`). -->
<!-- * `<<=` (`var <<= expr`): left-shift and assignment. Overloadable (`ShlAssign`). -->
<!-- * `<` (`expr < expr`): less-than comparison. Overloadable (`PartialOrd`). -->
<!-- * `<=` (`expr <= expr`): less-than or equal-to comparison. Overloadable -->
<!-- (`PartialOrd`). -->
<!-- * `=` (`var = expr`, `ident = type`): assignment/equivalence. -->
<!-- * `==` (`expr == expr`): equality comparison. Overloadable (`PartialEq`). -->
<!-- * `=>` (`pat => expr`): part of match arm syntax. -->
<!-- * `>` (`expr > expr`): greater-than comparison. Overloadable (`PartialOrd`). -->
<!-- * `>=` (`expr >= expr`): greater-than or equal-to comparison. Overloadable -->
<!-- (`PartialOrd`). -->
<!-- * `>>` (`expr >> expr`): right-shift. Overloadable (`Shr`). -->
<!-- * `>>=` (`var >>= expr`): right-shift and assignment. Overloadable -->
<!-- (`ShrAssign`). -->
<!-- * `@` (`ident @ pat`): pattern binding. -->
<!-- * `^` (`expr ^ expr`): bitwise exclusive OR. Overloadable (`BitXor`). -->
<!-- * `^=` (`var ^= expr`): bitwise exclusive OR and assignment. Overloadable -->
<!-- (`BitXorAssign`). -->
<!-- * `|` (`pat | pat`): pattern alternatives. -->
<!-- * `|` (`|…| expr`): closures. -->
<!-- * `|` (`expr | expr`): bitwise OR. Overloadable (`BitOr`). -->
<!-- * `|=` (`var |= expr`): bitwise OR and assignment. Overloadable (`BitOrAssign`). -->
<!-- * `||` (`expr || expr`): logical OR. -->
<!-- * `_`: “ignored” pattern binding. Also used to make integer literals readable. -->
<!-- * `?` (`expr?`): error propagation. -->

* `!` (`ident!(...)`, `ident!{...}`, `ident![...]`): マクロ展開を意味します
* `!` (`!expr`): ビット反転、または論理反転。オーバーロード可能 (`Not`)。
* `!=` (`var != expr`): 非等価比較。オーバーロード可能 (`PartialEq`)。
* `%` (`expr % expr`): 余り演算。オーバーロード可能 (`Rem`)。
* `%=` (`var %= expr`): 余り演算後に代入。オーバーロード可能(`RemAssign`)。
* `&` (`&expr`, `&mut expr`): 借用。
* `&` (`&type`, `&mut type`, `&'a type`, `&'a mut type`): 借用されたポインタ型。
* `&` (`expr & expr`): ビットAND。オーバーロード可能 (`BitAnd`)。
* `&=` (`var &= expr`): ビットAND後に代入。オーバーロード可能(`BitAndAssign`)。
* `&&` (`expr && expr`): 論理AND.
* `*` (`expr * expr`): 掛け算。オーバーロード可能 (`Mul`)。
* `*` (`*expr`): 参照外し。
* `*` (`*const type`, `*mut type`): 生ポインタ。
* `*=` (`var *= expr`): 掛け算後に代入。オーバーロード可能(`MulAssign`)。
* `+` (`trait + trait`, `'a + trait`): 型制限の複合化。
* `+` (`expr + expr`): 足し算。オーバーロード可能 (`Add`)。
* `+=` (`var += expr`): 足し算後に代入。オーバーロード可能(`AddAssign`)。
* `,`: 引数と要素の区別。
* `-` (`- expr`): 算術否定。オーバーロード可能 (`Neg`)。
* `-` (`expr - expr`): 引き算。オーバーロード可能 (`Sub`)。
* `-=` (`var -= expr`): 引き算後に代入。オーバーロード可能(`SubAssign`)。
* `->` (`fn(...) -> type`, `|...| -> type`): 関数とクロージャの戻り値型。
* `.` (`expr.ident`): メンバーアクセス。
* `..` (`..`, `expr..`, `..expr`, `expr..expr`): 未満範囲リテラル。
* `..` (`..expr`): 構造体リテラル更新記法。
* `..` (`variant(x, ..)`, `struct_type { x, .. }`): 「残り全部」パターン束縛。
* `...` (`expr...expr`) *in a pattern*: 以下範囲パターン。
* `/` (`expr / expr`): 割り算。オーバーロード可能 (`Div`)。
* `/=` (`var /= expr`): 割り算後に代入。オーバーロード可能(`DivAssign`)。
* `:` (`pat: type`, `ident: type`): 型制約。
* `:` (`ident: expr`): 構造体フィールド初期化子。
* `:` (`'a: loop {...}`): ループラベル。
* `;`: 文、要素終端子。
* `;` (`[...; len]`): 固定長配列記法の一部。
* `<<` (`expr << expr`): 左シフト。オーバーロード可能 (`Shl`)。
* `<<=` (`var <<= expr`): 左シフト後に代入。オーバーロード可能 (`ShlAssign`)。
* `<` (`expr < expr`): 未満比較。オーバーロード可能 (`PartialOrd`)。
* `<=` (`expr <= expr`): 以下比較。オーバーロード可能(`PartialOrd`)。
* `=` (`var = expr`, `ident = type`): 代入/等価。
* `==` (`expr == expr`): 等価比較。オーバーロード可能 (`PartialEq`)。
* `=>` (`pat => expr`): matchアーム記法の一部
* `>` (`expr > expr`): より大きい比較。オーバーロード可能 (`PartialOrd`)。
* `>=` (`expr >= expr`): 以上比較。オーバーロード可能(`PartialOrd`)。
* `>>` (`expr >> expr`): 右シフト。オーバーロード可能 (`Shr`)。
* `>>=` (`var >>= expr`): 右シフト後に代入。オーバーロード可能(`ShrAssign`)。
* `@` (`ident @ pat`): パターン束縛。
* `^` (`expr ^ expr`): ビットXOR。オーバーロード可能 (`BitXor`)。
* `^=` (`var ^= expr`): ビットXOR後に代入。オーバーロード可能(`BitXorAssign`)。
* `|` (`pat | pat`): パターンOR。
* `|` (`|…| expr`): クロージャ。
* `|` (`expr | expr`): ビットOR。オーバーロード可能 (`BitOr`)。
* `|=` (`var |= expr`): ビットOR後に代入。オーバーロード可能 (`BitOrAssign`)。
* `||` (`expr || expr`): 論理OR。
* `_`: 「無視」パターン束縛。整数リテラルを見やすくもします。
* `?` (`expr?`): エラー委譲。

<!-- ### Non-operator Symbols -->

### 演算子以外のシンボル

<!-- The following list contains all non-letters that don’t function as operators; -->
<!-- that is, they don’t behave like a function or method call. -->

以下のリストは、演算子として機能しない記号全部を含んでいます; つまり、関数やメソッド呼び出しのようには、
振る舞わないということです。

<!-- #### Stand-Alone Syntax -->

#### スタンドアローン記法

<!-- * `'ident`: named lifetime or loop label. -->
<!-- * `...u8`, `...i32`, `...f64`, `...usize`, *etc.*: numeric literal of -->
<!-- specific type. -->
<!-- * `"..."`: string literal. -->
<!-- * `r"..."`, `r#"..."#`, `r##"..."##`, *etc.*: raw string literal, -->
<!-- escape characters are not processed. -->
<!-- * `b"..."`: byte string literal, constructs a `[u8]` instead of a string. -->
<!-- * `br"..."`, `br#"..."#`, `br##"..."##`, *etc.*: raw byte string -->
<!-- literal, combination of raw and byte string literal. -->
<!-- * `'...'`: character literal. -->
<!-- * `b'...'`: ASCII byte literal. -->
<!-- * `|...| expr`: closure. -->
<!-- * `!`: always empty bottom type for diverging functions. -->

* `'ident`: 名前付きのライフタイム、あるいはループラベル。
* `...u8`, `...i32`, `...f64`, `...usize`, *etc.*: 特定の型の数値リテラル。
* `"..."`: 文字列リテラル。
* `r"..."`, `r#"..."#`, `r##"..."##`, *etc.*: 生文字列リテラル、
エスケープ文字は処理されません。
* `b"..."`: バイト文字列リテラル、文字列の代わりに`[u8]`を構築します。。
* `br"..."`, `br#"..."#`, `br##"..."##`, *etc.*: 生バイト文字列リテラル。
生文字列とバイト文字列の組み合わせ。
* `'...'`: 文字リテラル。
* `b'...'`: ASCIIバイトリテラル。
* `|...| expr`: クロージャ。
* `!`: 常に発散関数の空のボトム型。

<!-- #### Path-Related Syntax -->

#### パス関連の記法

<!-- * `ident::ident`: namespace path. -->
<!-- * `::path`: path relative to the crate root (*i.e.*, an explicitly absolute -->
<!-- path). -->
<!-- * `self::path`: path relative to the current module (*i.e.*, an explicitly -->
<!-- relative path). -->
<!-- * `super::path`: path relative to the parent of the current module. -->
<!-- * `type::ident`, `<type as trait>::ident`: associated constants, functions, and -->
<!-- types. -->
<!-- * `<type>::...`: associated item for a type that cannot be directly named -->
<!-- (*e.g.*, `<&T>::...`, `<[T]>::...`, *etc.*). -->
<!-- * `trait::method(...)`: disambiguating a method call by naming the trait -->
<!-- that defines it. -->
<!-- * `type::method(...)`: disambiguating a method call by naming the type for -->
<!-- which it’s defined. -->
<!-- * `<type as trait>::method(...)`: disambiguating a method call by naming -->
<!-- the trait *and* type. -->

* `ident::ident`: 名前空間パス。
* `::path`: クレートルートに相対的なパス(*すなわち*、明示的な絶対パス)。
* `self::path`: 現在のモジュールに相対的なパス(*すなわち*、明示的な相対パス)。
* `super::path`: 現在のモジュールの親モジュールに相対的なパス。
* `type::ident`, `<type as trait>::ident`: 関連定数、関数、型。
* `<type>::...`: 直接名前付けできない型の関連要素
(*例*, `<&T>::...`, `<[T]>::...`, *など*).
* `trait::method(...)`: 定義したトレイトを名指ししてメソッド呼び出しを明確化する。
* `type::method(...)`: 定義されている型を名指ししてメソッド呼び出しを明確化する。
* `<type as trait>::method(...)`: トレイト*と*型を名指ししてメソッド呼び出しを明確化する。

<!-- #### Generics -->

#### ジェネリクス

<!-- * `path<...>` (*e.g.*, `Vec<u8>`): specifies parameters to generic type *in -->
<!-- a type*. -->
<!-- * `path::<...>`, `method::<...>` (*e.g.*, `"42".parse::<i32>()`): -->
<!-- specifies parameters to generic type, function, or method *in an expression*. -->
<!-- Often referred to as *turbofish*. -->
<!-- * `fn ident<...> ...`: define generic function. -->
<!-- * `struct ident<...> ...`: define generic structure. -->
<!-- * `enum ident<...> ...`: define generic enumeration. -->
<!-- * `impl<...> ...`: define generic implementation. -->
<!-- * `for<...> type`: higher-ranked lifetime bounds. -->
<!-- * `type<ident=type>` (*e.g.*, `Iterator<Item=T>`): a generic type where one or -->
<!-- more associated types have specific assignments. -->

* `path<...>` (*例*, `Vec<u8>`): *型の内部の*ジェネリック型への引数を指定する。
* `path::<...>`, `method::<...>` (*例*, `"42".parse::<i32>()`):
*式中の*ジェネリックな型、関数、メソッドへの引数を指定する。
しばしば*turbofish*(ターボ・フィッシュ)と称される。
* `fn ident<...> ...`: ジェネリックな関数を定義する。
* `struct ident<...> ...`: ジェネリックな構造体を定義する。
* `enum ident<...> ...`: ジェネリックな列挙型を定義する。
* `impl<...> ...`: ジェネリックな実装を定義する。
* `for<...> type`: 高階ライフタイム境界。
* `type<ident=type>` (*例*, `Iterator<Item=T>`): 1つ以上の関連型に代入された
ジェネリックな型。

<!-- #### Trait Bound Constraints -->

#### トレイト境界制約

<!-- * `T: U`: generic parameter `T` constrained to types that implement `U`. -->
<!-- * `T: 'a`: generic type `T` must outlive lifetime `'a`. When we say that a type -->
<!-- “outlives” the lifetime, we mean it cannot transitively contain any references -->
<!-- with lifetimes shorter than `'a`. -->
<!-- * `T : 'static`: the generic type `T` contains no borrowed references other -->
<!-- than `'static` ones. -->
<!-- * `'b: 'a`: generic lifetime `'b` must outlive lifetime `'a`. -->
<!-- * `T: ?Sized`: allow generic type parameter to be a dynamically sized type. -->
<!-- * `'a + trait`, `trait + trait`: compound type constraint. -->

* `T: U`: `Uを実装する型に制約されるジェネリック引数`T`。
* `T: 'a`: ライフタイム`'a`よりも長生きしなければならないジェネリック型`T`。
型がライフタイムより長生きするとは、`'a`よりも短いライフタイムの参照を何も遷移的に含めないことを意味する。
* `T : 'static`: ジェネリック型`T`が`'static`なもの以外の借用された参照を何も含まない。
* `'b: 'a`: ジェネリックなライフタイム`'b`がライフタイム`'a`より長生きしなければならない。
* `T: ?Sized`: ジェネリック型引数が動的サイズ付け型であることを許容する。
* `'a + trait`, `trait + trait`: 複合型制約。

<!-- #### Macros and Attributes -->

#### マクロとアトリビュート

<!-- * `#[meta]`: outer attribute. -->
<!-- * `#![meta]`: inner attribute. -->
<!-- * `$ident`: macro substitution. -->
<!-- * `$ident:kind`: macro capture. -->
<!-- * `$(…)…`: macro repetition. -->

* `#[meta]`: 外部アトリビュート。
* `#![meta]`: 内部アトリビュート。
* `$ident`: マクロ代用。
* `$ident:kind`: マクロキャプチャ。
* `$(…)…`: マクロの繰り返し。

<!-- #### Comments -->

#### コメント

<!-- * `//`: line comment. -->
<!-- * `//!`: inner line doc comment. -->
<!-- * `///`: outer line doc comment. -->
<!-- * `/*...*/`: block comment. -->
<!-- * `/*!...*/`: inner block doc comment. -->
<!-- * `/**...*/`: outer block doc comment. -->

* `//`: 行コメント。
* `//!`: 内部行docコメント。
* `///`: 外部行docコメント。
* `/*...*/`: ブロックコメント。
* `/*!...*/`: 内部ブロックdocコメント。
* `/**...*/`: 外部ブロックdocコメント。

<!-- #### Tuples -->

#### タプル

<!-- * `()`: empty tuple (*aka* unit), both literal and type. -->
<!-- * `(expr)`: parenthesized expression. -->
<!-- * `(expr,)`: single-element tuple expression. -->
<!-- * `(type,)`: single-element tuple type. -->
<!-- * `(expr, ...)`: tuple expression. -->
<!-- * `(type, ...)`: tuple type. -->
<!-- * `expr(expr, ...)`: function call expression. Also used to initialize -->
<!-- tuple `struct`s and tuple `enum` variants. -->
<!-- * `ident!(...)`, `ident!{...}`, `ident![...]`: macro invocation. -->
<!-- * `expr.0`, `expr.1`, *etc.*: tuple indexing. -->

* `()`: 空のタプル (unit*としても知られる*)、リテラル、型両方。
* `(expr)`: 括弧付きの式。
* `(expr,)`: 1要素タプル式。
* `(type,)`: 1要素タプル型。
* `(expr, ...)`: タプル式。
* `(type, ...)`: タプル型。
* `expr(expr, ...)`: 関数呼び出し式。
tuple `struct`やtuple `enum`列挙子を初期化するのにも使用される。
* `ident!(...)`, `ident!{...}`, `ident![...]`: マクロ呼び出し。
* `expr.0`, `expr.1`, *etc.*: タプル添え字アクセス。

<!-- #### Curly Brackets -->

#### 波括弧

<!-- * `{...}`: block expression. -->
<!-- * `Type {...}`: `struct` literal. -->

* `{...}`: ブロック式。
* `Type {...}`: `struct`リテラル。

<!-- #### Square Brackets -->

#### 角括弧

<!-- * `[...]`: array literal. -->
<!-- * `[expr; len]`: array literal containing `len` copies of `expr`. -->
<!-- * `[type; len]`: array type containing `len` instances of `type`. -->
<!-- * `expr[expr]`: collection indexing. Overloadable (`Index`, `IndexMut`). -->
<!-- * `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`: collection indexing -->
<!-- pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, or -->
<!-- `RangeFull` as the “index.” -->

* `[...]`: 配列リテラル。
* `[expr; len]`: `len`個`expr`を含む配列リテラル。
* `[type; len]`: `len`個の`type`のインスタンスを含む配列型。
* `expr[expr]`: コレクション添え字アクセス。オーバーロード可能 (`Index`, `IndexMut`)。
* `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`: `Range`、`RangeFrom`、`RangeTo`、
`RangeFull`を「添え字」として使用してコレクション・スライシングの振りをするコレクション添え字アクセス。
