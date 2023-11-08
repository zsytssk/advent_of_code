https://adventofcode.com/2022/day/17

ownerproof-3155442-1695261524-344d6e3a17b9

https://song.xlog.app/aoc-zh

- @ques 可以看看答案 -> ...

- @todo

  - 优化简历
  - 朋友内推
  - 基本算法
  - 微前端 + ts 基础题目
  - 基础题目...

- font emoji

## day24(没有实现)

- @note 感觉我的方向有些问题， 有没有可能直接一步到位，这样就不需要计算所有的可能性

  - 就像那个寻路的步数一样，每次取优先级最高的，最后必然都是最快的
  - 不用遍历所有的可能

- @ques 感觉我 loop 有问题

  - 我在 loop 中取更新节点 每次都更新节点，但是在 calc_top_keys 改变的 item
  - 可能导致每次的 loop,和 loop_keys 不匹配
  - 我需要在每一个 key 中去记录他的步数，同时 loop_keys 中的步数应该是一样的
  - 同时 update_map 要能直接根据步数来变化(方向长度是周期)

- @learn

  - `breadth-first search (BFS)`
  - Copy trait 是做什么的
  - split_off
  - `last_key.1.abs_diff(end.1)`
  - 怎么 split_off 前面的 item - extend to the

- @ques 计算每一个格子出现空白的时间

- @ques 或者 loop

- @ques calc_top_pos 已经移动的距离 + 距离中终点的距离

- @ques get_nest_pos 可以原地等待

- @ques 地图的形状 print 的时候能不能做成幻灯片的形式

  - println replace

- @ques 感觉这里可能会有性能问题

  - 在某个确切时间点 到达某个点的 -> 如果已经存在不做处理了
  - 找下一个点，只要找附近的就可以了
  - 如果我有办法 只更新附近的点就好了 -> 就会更快
    - 也许有的 开始位置 + 时间 + 结束位置的公式 就好了

- @ques `for in in 0..0 {}` 会执行吗 -> 不会 -> 那个为啥会出问题

### end

- @ 整个风的运动是不是周期性的

  - 是的

- 要想不到一个好的逻辑来解决这个问题，应该没有办法解决

  - 每一个缝隙的可能性 -> 看不到方向

- @ques quick_step

- @ques 计算每一个循环的消耗

  - loop_keys n > get_nearby_pos(5)
  - calc_top_keys > n + n

  - update_map 中的 loop 可以指更新当前位置附近关联风的变化（我需要计算每一个风的公式）
    - loop 也要正好是当前的步数

- @ques 减少在原地不动的选项

  - 有一点用，没有改变多少

- @ques 有没有更好的方式

- get_next + calc_top

- @ques wind 移动清除怎么处理
- @ques 风的移动逻辑怎么处理

- @ques 每一个点都要找到
- @ques 多个风合在一起我怎么处理

## day23

- @que 性能优化 -> `https://github.com/Gobbel2000/advent-of-code/blob/master/src/bin/day23-1.rs`

- @todo learn

  - 如何在过程中动态的更新地图大小 -> `map -> RefCell`
  - ***
  - 同时要修改多个 point 的坐标，这怎么处理？
  - 怎么删除多个相同的坐标

### end

- @ques nearby_points 变成 hashmap 反而变慢了，可能是 clone 导致的

  - get().is_some 导致的

- 性能 ->

  - `loop` >
    - `for (n * 4)` >
      - `not_need_move(8*n) + can_move(3 * n)`
    - `remove_dul_p(n*n) + for(n)`
  - 优化后 `loop` > `for (n*n)` + `remove_dul_p(n*n) + for(n)`
  - 还能不能再优化 -> nearby vec -> HasMap

- @todo 优化性能

  - 只要对比附近的点就可以了 -> get_rel_point
  - 所有 loop 的地方

- @ques 怎么移动
- @ques 怎么判断两个同时移动到一个地方？
- @ques 同时要修改多个 point 的坐标，这怎么处理？
  - `Vec<RefCell>` ?

## day22(part2 可以参考下别人的做法)

- @rem

  - Regex split | split_keep
  - 把 enum 转换称 number, 然后再转换回来

- @ques 画图

- 懵逼了 -> 完成搞不清楚 位置的对应关系

  - 我要能自己做一个 平面和立方体对应关系 3d 模型就好了

- @ques

  - 怎么确定 立方体 每一个面对应的位置
    - 先写死坐标
  - 怎么在不同的面中跳转（三围立体坐标）
  - ***
  - 简单的做法 写一个 超出 坐标转换 | 一一 对应写死的
  - 复杂的做法 -> 直接建立一个立方体，用这个来转换坐标，在立方体上做逻辑，最后转换称平面坐标

- @ques get_opposite_p 只有这个需要改

### end

- @ques test 上下左右 get_opposite_p

- @todo 把走过的路画出来

## day21

- @todo 试试去掉 clone 能不能提高性能

### end

- @ques loop 中 `OperateWrap::Unknown` 的处理

- @ques 已经知道结果 value，怎么反推表达式的 value

- @ques 怎么 loop
- @ques 怎么计算表达式的值

- @ques 难点 -> 怎么记录结果

- @ques loop 的优先级如何处理？

  - 已经有结果了

- 只有两个都有结果了才能 remove

## day19 遍历所有 机器人数量

- 感觉我的数学能力还是比较欠缺

- 这有一个条件：在 ore 机器人数量为 n 时求 clay 机器人的范围

  - 这时候是不是先把 ore 机器人创建好了，clay 机器人将有最大的范围
  - 同理在计算 obsidian+geode 范围时是不是也要先把其他的计算好
  - 假如不是这样，那我这些假设都没有什么用

- 思路 求 ore 机器人的范围 -> for ore 的范围 求 clay 的范围 ->
  - for clay 范围 -> obsidian 的范围 -> ... geode 的范围

```rs
blueprint -> robotRange type: [0, big_num]

矿物数量
机器人数量
```

## day19 (part1 都没有做出来)

- @ques 用方程解决？

  - 假如只用生成 ore, 在 24 分钟内怎么做 最多生成 ore 的个数最多 -> 不会
  - 可以写一个程序 穷举出来
  - 最多采集多少个 -> clay 矿物 -> obsidian -> geode
  - @ques 先创建好机器人 就一定 采集的矿物就一定多吗 -> 假如是对的

- @ques 24 分钟内最多能生成 142 个 ore 机器人

- @ques 可能把每次时间变化 调整为生产机器人的变化

  - 最快生成 A,B,C,D 的速度 -> 转换成那个 rate 的值
  - 这样也许可以减少不必要的 loop

- @ques

  - 有可能是 rate 让 ore 生产的太多了 -> init_rate
  - 哪里计算有问题 + 能不能优化效率

- @ques 开始的是不是最大的

- @ques 如果我能计算出最大值就好了

- @ques `target_robot.cost` 和 下面的 loop 能不能合并

- @ques 如果要计算多层要怎么处理？如果只计算一层会怎样？

  - 数组中每一项都相加如何处理

- @ques 如何计算优先级

  - 取数组的最小值，就可以了
  - 所有比例都是一个数组 -> `[1, 0]`
  - ***
  - 我怎么知道数组中有几个可能性呢？ -> 数组中的几个元素，每一个值分别对应数组中的哪一个？
  - ***
  - 机器比例 = `矿物比例 * 时间就行了`
  - rate_map 中 key 如果已经存在 怎么处理 -> + 新的
  - 两个合在一起才有优先级，不然没有。。。
  - 矿的优先级 + 机器的优先级 机器的优先级要乘以剩余时间 才有效
  - 两边直接 \* 1/2 会不会更有效 (这样会导致可能一直取积蓄 ore 去了)

- @ques 如果不计算优先级，这题还能做出来吗？

- @learn `pub struct Global;`

- @ques 题目的关键是什么？

  - @ques 怎么遍历所有的可能
  - 选择是什么时候生产什么？

- loop 时间 | HashMap 保存数据 计算可能性

### end

- @todo loop 后 测试下 rate 看看有没有问题

  - 调整下比例，让他优先生产机器人
  - 生产 clay 太多但是不会创建 Obsidian
  - 也不会自己生成 ore

- @ques 要花时间才能创建 robot

- @ques 感觉这有些问题， Blueprint 1 clay

- @ques calc_top_list

- @ques demo Blueprint1 如果我有两个 ore,我可以生成一个 clay, 也可以继续等待去生产 ore

  - 这两种可能我怎么区分 -> 怎么遍历这种可能性

- 所有的可能性，

  - 必须记录资源 ore + clay + obsidian
  - 记录机器人的个数
  - 记录时间

- @ques 怎么去判断优先级？

  - 可能优先级直接可以把答案得出来
  - 步数 + 比例
  - geode
    - geode -> ore 1/2| obsidian 1/7
    - obsidian -> ore 1/3 | clay 1/14
    - clay -> ore 1/2
    - 取两边的最小值...
    - 算出每一个个对应的比例

- @ques loop

  - 递进关系怎么处理？
  - 什么是很结束？ -> rate_map.len() == robots.len()

```rs
for (cost_type, cost_num) in cost.iter() {
  rate_arr.clone().iter_mut().map(|x| {
      *x *= *cost_num as f64
  })
  let new_rate = 1.0 / (*cost_num as f32 * base_rate);
  rate_map
    .entry(cost_type.clone())
    .and_modify(|(ore_rate, robot_rate)| {
        *ore_rate += new_rate;
    })
    .or_insert((new_rate, 0.0));
}
```

- @ques 如果能创建多个可能，我要怎么处理？

  - 怎么把可能一一的列举出来 -> loop 然后一个个的列举吗
  - 能创建多少个

- @ques 从五个中取 4 个的可能性，如何用程序实现？

- @ques `Blueprint (\d+):\n?\s+(Each (\w+) robot costs (\d+) ore( and (\d+) (clay|obsidian)+)?\.\s*)+`
  - 正则表达式无法匹配所有
  - 因为这无法重复

### 怎么列举一个个的可能

- @ques 有四个机器人 -> 知道每一个的消耗
- @ques 资源

- 可能性 可能会爆炸

- @ques 将能创建的扔到数组中，用这个去 loop

  - 直接用 loop 也许可以更好的解决这个问题

- @ques 如果无法新建 robot,这种情况有没有被忽略

- 举一个例子 -> ...
- @fan
  - 数字拆分 10 -> 1,2,3 的组合总共有多少种可能

## day 18 （part2 没做出来）

- @ques 关键的地方是 怎么判断是被包围的区域还是公开的区域

- @ques 我对题目的理解是不是正确的？

  - 把 demo 画出来

- @ques 立方体填充？

- @ques 立方体之间 只有两个交点 ->

- @ques 如何判断一个分割区域

- @ques 线段的关系 面的关系

- @ques 如何求一个封闭区域

- @ques 能不能建立一个三维模型，然后很容易去理解这里面的关系

- @ques 也许用立方体 更方便

  - 用立方体来填充目标区域 -> 怎么填充

- @ques 能不能用线来表示各种关系

- @ques next_arr.len() = 8 是什么情况

  - == 4 并不能保证是包围区域
  - 有效的=4 个就可以了
  - save_arr 中所有的 每一个都能获取四个，同时

- 所有相交的面要把 overlap 的排除掉

- @ques 所有面的关系

- @ques 重叠的如何处理？-> 如何排除掉

- @ques 怎么跑完所有的 surface -> 判断 surface 是 true

  - 只要有一个为 false,就全部 wei false

- @opt 可以通过立方体的坐标 取特定范围内的立方体 -> 优化性能

- @ques 怎么知道一个面的关联的面

  - 有两个重叠的点

- @ques 游戏中如何 描述一个面 描述一个立方体

  - 如果有一个好的表达方式，这题应该会容易些

- @ques 3 个中取两个 这个如何用程序去描述

- @ques 怎么知道立方体被包围

  - 虽然没有接触，但是这个面无法被外界接触到？ ->

- @ques 如果包围的只是小立方体，那判断面存在

  - 四边有相交的，且对面的面也是一样

- @ques 所有的立方体是不是只存在中间有一个小立方体的空间

  - 如果是这样的怎么计算
  - 如果不是这样的如何去计算?

- @ques 相交的边

- @ques 怎么知道有一个（多个）包围的空面？

  - 必然也是一个立方体的组合 -> 无限的放大这个组合 -> 怎么扩张立方体
  - 计算这个扩张立方体 的表面积就行了
  - 这个组合所有的面都被包围了

- @ques 面的扩张

  - 面的四个边 都有对应的的面 在立方体中， 包围区域
  - 只要有一个没有就是 放开区域

- @ques 面如何描述

  - 要保证唯一
  - 四个点
  - 坐标？
  - 一个点有三个面
  - 能不能用向量表示

- @ques 能不能用更优雅的方式生成面

### 扩张的方法

- @ques 每一个面单独去 loop

  - 将所有遍历到的面都放到一个数组中去，一旦有结果，所有的都为 false 或者都为 true

- @ques 怎么知道遍历完成了？

  - 所有下一个都在数组中了吗？

- 8 个点 各自创建立方体 -> 如果已经有了排除 ->

- @ques 怎么排除外面的面

  - 找相邻的立方体（有边相交的）

- @ques 能不能直接面扩张 -> 这样可能能更简单些

### end

- @ques loop 时有三种数据 最后都要一起处理

  - save_arr 已经遍历过的 item
  - loop_arr 正在遍历过的 item
  - find_next 将要遍历过的 item

- @ques 排除已经计算过的了
- @ques 怎么把所有的情况都遍历到
- @ques 怎么知道立方体之间相交的部分
- 总面积 - 重叠的部分

  - 直线相交
  - 平面相交
  - 立方体相交

- 立方体有八个点（有四个点重合就是相交）

## day17（part2 没做出来）

- @ques 哪些需要熟练的？

  - debug + test + 字符串加减

- @ques 还能怎么优化

  - 这个难道是可以重复的？直接不用计算就知道结果？
  - 1000000000000 即使是简单的 print 也要花费很长时间啊
  - 应该是每隔多少 就有一个重复的 -> 然后重复的直接加上新增的
  - 把不需要处理的直接排除掉（连在一起的... 我怎么知道连在一起）

- @ques 看不出有什么规律
  - 看看代码
  - 对比

```
66
40 - 66
80 - 125 (40 * 2 + 3)
120 - 184
160 - 248 (80 * 2 - 2)
320 - 490 (160 * 2 - 6)
640 - 973 (320 * 2 - 7)

1 - 15410
2 - 30809
3 - 46211
4 - 61628
8 - 123257
12 - 184879

```

- @ques 每一个石头的形状
- @ques 石头和石头 和墙之间的碰撞如何处理
- @ques 堆积之后又是什么形状

- @ques convertLocalPos

### save

- @ques rust print with `\n` -> ` println!("{}", chamber.get_fmt_str());`

### end

- @ques 能不能把 chamber 画出来
- @ques 能不能把 石头换成点 -> 这样就方便多了
- @ques vscode duplicate selection
- @ques rock status ？

- @ques 添加的时候有问题
- @ques 还能怎么的优化性能

  - 一开始就去计算，到时候直接拿就行了
  - 将 rock_list 分割 成 100 份数 -> 计算的时候直接去取特定区间内的就行了

- get_rock_in_range

- @ques 画出 champer
- @note 第一象限坐标系 坐标系 -> rock 要变换

- @ques 画出 Rock

## day12

- @todo 每次求可能性最大的那一块

- @todo 每次求可能性最大的那一块

  - 代价 = 已经进行的步数 + 可能的步数

- @todo 每次求一步
- @ques `Ref<'_, XX>` 中间的 lifetime 是做什么的
- @ques 有没有 min_by 这种方法

- @ques left_time 怎么处理

  - 其实这所有的 lifetime 都是在 map 下面的

- @ques 为什么有那么多的可能性

  - 最短的路径判断难道有问题
  - 都还没有出门

- @ques 已经经常

### save

原来 step1 -> 31.854178153s

## day16（难）2

- @opt 能不能優化性能 -> get_max_score 有問題
- @opt MapKey 的數據格式能不能

### end

- @ques 所有的可能性 ->（有沒有漏掉的）

- @ques 如何把可能性 用代碼表示出來

- @ques 等於 0 時爲什麼那麼奇怪 -> 死循環？

  - 這種情況如何排除？

- @ques 這種可能性會不會導致 某些選項沒有被匹配到

  - 直接把這種情況排除掉...

- @ques 時間 減少的有些問題
- @ques 怎麼改了幾個問題，時間就變了這個多
- @ques 感覺我這邏輯哪裏有問題

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
