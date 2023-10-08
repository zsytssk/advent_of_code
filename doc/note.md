https://adventofcode.com/2022/day/4

ownerproof-3155442-1695261524-344d6e3a17b9

https://song.xlog.app/aoc-zh

## 2023-10-07 19:06:10

- @ques 如何优化内存

  - path.clone() ？

- 贪婪算法 ｜ 迟钝算法 ?
- 有什么更好的算法？

- 如何加快 step2 -> 可以优化下

  - 从 end 找其他的，可以共用 map_space, 也许可以更快些
  - 多线程？
  - 算法 -> https://zhuanlan.zhihu.com/p/385733813
  - ***
  - 可寻找的点是一个数组不停的增加，然后在过程中可以更新各个点的优先级

- @ques hashMap -> target -> space 如果更长就不用处理了

### end

- @ques 寻路怎么才能 不后退

## 2023-10-04 11:20:55

- @ques Point 如何自定义乘法+加法
- @ques Point Copy

### 2023-10-04 15:04:47

- @ques Point 相等

## 2023-09-28 22:29:15

- @ques vscode rust debug

- @ques 用 `MyStruct(Rc<RefCell<NodeType>>)` 改写 Dir 等

- @ques 为什么

- @ques `&*wrap.borrow()`

- @ques rust 这种嵌套 然后再解析 太麻烦了

  - 然后要对里面做处理 如果我能根据需要给他自动生成不同的类型，这个问题是不是就解决了
  - `Rc<RefCell<NodeType>>`

- @ques impl deref from NodeType

- @ques weakRef

- @ques rust 如何在 loop 中改变 vec 的值

```
可能while loop 可以
```

- @ques rust 双向绑定

  - https://rust-unofficial.github.io/too-many-lists/fourth-final.html

- @todo 优化 is_marker 返回重复的 index

  - 如果有多个重复怎么处理
  - ??

- @ques slice 中怎么 remove 多个
  - drain

```rs
// 这段代码为什么会报错
fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    let arr = a.drain(0..2);
    println!("{:?} {:?}", a, arr);
}
```

```rs
// 这个怎么没有log
let m = re.captures_iter(hay).map(|str| {
  println!("{:?}", &str.get(1).unwrap());
});
```

```rs
re.captures_iter(hay)
```

```rs
// 报错？exclusive range pattern syntax is experimental
fn is_item(c: &char) -> bool {
    match c {
        'A'..'Z' => true,
        _ => false,
    }
}
```

## 2023-09-22 09:42:11

- @ques destruct vec

- @ques 如何获得 match`_`中的选项
- @ques rust cell 获得内部改动的原因是什么？
- @ques sizeOf(struct)

- 对比 stack 和 heap 读写的消耗？

### end

- @ques 如何 split with regex
