# 引用与借用

> **一句话**：`&T` 是不可变借用（可以有多个），`&mut T` 是可变借用（同时只能有一个）——这是 Rust "读写锁在编译期"的体现。

## 与 JS/TS 的关键差异

JS/TS 对引用没有任何编译期约束——你可以同时持有多个可变引用而无编译错误。Rust 的借用规则确保：要么多个读者，要么一个写者，绝不可能同时有两者。这条规则在编译期消除数据竞争。

## 三条借用规则

1. **任意时刻，只能拥有一个可变引用或任意数量的不可变引用**（不能同时）
2. **引用必须始终有效**（不能悬垂）
3. **引用的生命周期不能超过它指向的数据**

## 代码对比表

### 不可变借用 `&T`

```rust
let s = String::from("hello");
let r1 = &s;   // 不可变借用
let r2 = &s;   // 可以同时有多个不可变借用
println!("{r1}, {r2}");  // 同时使用
// r1, r2 的生命周期在这里结束
println!("{s}");  // s 仍然可用
```

```typescript
// TypeScript — 无限制的引用共享
const s = { text: "hello" };
const r1 = s;
const r2 = s;
console.log(r1.text, r2.text);
```

### 可变借用 `&mut T`

```rust
let mut s = String::from("hello");
let r = &mut s;  // 可变借用
r.push_str(", world");
// println!("{s}");  // ❌ 不能在 r 存活期间读 s
println!("{r}");     // r 的最后一次使用
// r 的生命周期结束
println!("{s}");     // ✅ 现在可以读 s 了
```

### Non-Lexical Lifetime (NLL)

```rust
// 编译器追踪引用的实际使用范围，而非词法作用域
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1}, {r2}");  // r1, r2 的最后一次使用

let r3 = &mut s;  // ✅ NLL：r1/r2 已不再使用，可以创建可变借用
r3.push_str(", world");
```

### 悬垂引用的预防

```rust
// ❌ 编译错误：返回局部变量的引用
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s 在函数结束时被 drop
}  // error: missing lifetime specifier / cannot return reference to local variable
```

## 容易踩的坑

1. **可变引用是排他的**——`let r1 = &mut s; let r2 = &mut s;` ❌
2. **不可变借用的不可变性传染**——通过 `&T` 拿到的引用不能修改原值
3. **借用和所有权的交互**——有借用存在时，不能 move 原值
4. **函数参数选 `&T` 还是 `T`**——只需要读时用 `&T`，需要所有权时用 `T`
5. **`&String` vs `&str`**——参数尽量用 `&str`（可接受 `&String` 和字符串字面量）

## 交叉链接

- → [所有权模型](ownership.md) — 借用是暂时转移使用权而非所有权
- → [生命周期基础](lifetime-basic.md) — 借用与生命周期标注的关系
- → [智能指针](smart-pointer.md) — 当借用规则不够灵活时，用 Rc/RefCell
