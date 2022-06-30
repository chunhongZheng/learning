//生命周期 lifetime
//生命周期（lifetime）是这样一种概念，编译器（中的借用检查器）用它来保证所有的借用都是有效的。确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候结束。虽然生命周期和作用域经常被一起提到，但它们并不相同。
//例如考虑这种情况，我们通过 & 来借用一个变量。该借用拥有一个生命周期，此生命周期由它声明的位置决定。于是，只要该借用在出借者（lender）被销毁前结束，借用就是有效的。然而，借用的作用域则是由使用引用的位置决定的。

//显示标注

//借用检查器使用显式的生命周期标记来明确引用的有效时间应该持续多久。在生命周期没有省略1的情况下，
// Rust 需要显式标注来确定引用的生命周期应该是什么样的。可以用撇号显式地标出生命周期，语法如下：


// foo<'a>
// `foo` 带有一个生命周期参数 `'a`