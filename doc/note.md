https://adventofcode.com/2022/day/4

ownerproof-3155442-1695261524-344d6e3a17b9

https://song.xlog.app/aoc-zh

## day16（难）2

- @ques 所有的可能性 ->（有沒有漏掉的）
- @ques 能不能優化性能 -> get_max_score 有問題

- @ques 時間 減少的有些問題
- @ques 怎麼改了幾個問題，時間就變了這個多

### end

- @ques 這種 左一步 右一步 的會不會導致某些情況沒有被用到

  - 應該不會，這樣理論上是最大值

- @ques 怎麼錯了

  - short_path 有問題
  - 計算 rate 有問題

- @ques 如何 loop_key

- @ques 便利数组
- @todo
  - MapKey
  - complete_path

find_short_path 优化如果已经出现的 map 就直接 return
find_short_path 优化如果已经出现的 map 就直接 return

## day16（难）

- @ques input 为什么会慢这么多？

  - 内存占用太高 20G

- @todo

  - find_path 没有解决一开始进来的问题
    - cur_time 多算了
    - 好像也没有什么问题
  - 计算 input 在不同的 cur_space 的速度 1 最小...
  - ***
  - get_top_path 只取一个
  - 将递归去掉...
  - 将已经结束的小于最大值的全部删除...
  - 将已经计算过的 排除在外 `(Vec<(String, bool)>, usize, i32) 最后一个标识`
  - 删除多余的 key
  - get_top_path 对比 time

- @ques input 好像进入了死循环一样

- @ques 每次跑一格..., 在 loop 中 insert

- @ques 怎么最后卡住了， 任务没有结束

- @ques 寻路怎么保证最高优先级的是最快的
- @ques 穷举所有的可能，然后计算所有的结果，求出最大的值

- @todo

  - key -> string
  - ***
  - 每次只走取最大
  - 排除掉分数最小的
  - 每次走 5 格
  - 不走回头路

- @ques 将 path_map 中不需要的删除 会不会就快很多？

- @ques 有没有可能找到最高的 rate，按顺序排列 一个个

- @ques 为什么不同的逻辑 走的次数差了这么多

- @ques `time_space` 能不能删除
- @todo `impl PathKey Debug` -> 需要包一层

- @ques `let mut arr = cur_value` 能不能用 borrow

- PathKey + PathMap 可以写成一个 struct

- @ques 能不能转换成一个简单的问题？

- @ques for 循环不停的去找，按照优先级排序

-> 可以把寻路算法优化下 day12

-> 开始的时候 AA 算不算 time

- @ques 如果有一个完美的算法 那在 `find_path` 中的 sort 还有必要吗？

- @opt clone 的地方太多了

### end

- @ques 为什么每次跑两格结果不对？

- @ques 寻路的优先级如何处理？-> 看看前面寻路是怎么做的

  - 先比较 time，再比较 rate
  - 比较重复的次数
  - ?

- @ques 如果每次跑一 munite 怎么样

- @ques 能不能像寻路一样同时找几个，然后不停的累积数字，最后求出最大的一个

  - 这需要一个 hashMap
  - 还有三十分钟的限制

```txt
(size, time)
DD-EE
```

- @ques 回退如何处理？

  - 存不存在，如果存在怎么处理

- @ques 感觉这样计算快了很多 这是为啥？

- @ques 在 foreach 中将不需要的去掉？

- @todo

  - test 计算分数 -> 是不是我计算的有问题

- @ques hash_map 如何区分两者的不同

- @ques 跑过去但是不 open

  - 这。。。我可如何处理？
  - 我怎么计算这种情况 -> 可能性又变大了
  - 暴力计算？ -> 两种情况都计算进去
    - path_map 现在这样已经不行了
    - 会不会 出现 `2*n` 次方 导致卡住？

- @ques 怎么把这两种可能都加进去

  - 直接用 loop，这样就不用在 for 循环中瞎搞了

- @ques 如果换成不同的算法得出的结果不一样，那就说明遍历 没有完全

- @ques A->B 有没有可能走了多次？

- @ques 如果把所有的可能都跑完会怎样？

- @ques 为啥可能性 这么多？

  - 感觉哪里出了问题，不可能有这么多的？ -> 有很多死循环在里面？

- @ques 为什么 `0..6 - 5` 没有跑满

## day15

地图太大了如何处理？
看各个 range 能不能合并 -> range 的计算
range 求 10 个元素的并集

- @ques 如何求 range 得并集

  - 集合之间的关系 -> 分离 + 相交 + 包含
  - 集合的运算
  - reduce -> 最终返回的是没有任何交集的数组
    - 如果一个发生改变时 导致其他的也发生变化如何处理
    - 能不能只把相交的去掉，然后留下不相连的部分，这样就会成为一个个相互分割的区间，不会出现重复计算的问题

- 获取 range 中的空内容

## day14

- @ques get_next_pos = [down, left, right]..

- `map.add_bottom(200);` 写死了

## day13

- @ques parse_tokens 有没有更好的写法, 下面都是凑出来
  - +1 实在是太 low 了
  - arr.remove(0)
  - peek + take_while 我可以自己写一个软件

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
