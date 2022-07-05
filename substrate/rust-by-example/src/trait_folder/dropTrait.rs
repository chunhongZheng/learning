

//Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该 方法。Drop trait 的主要作用是释放实现者的实例拥有的资源。
//
// Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放 资源的类型。Drop trait 也可以为任何自定义数据类型手动实现。
//
// 下面示例给 drop 函数增加了打印到控制台的功能，用于宣布它在什么时候被调用。