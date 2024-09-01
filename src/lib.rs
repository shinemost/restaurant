/** `add_two` 将指定值加2

文档注释里还可以写测试用例，也是没谁了

```
let arg = 5;
let answer = restaurant::add_two(arg);

assert_eq!(7, answer);
```
*/
/// `add_one` 返回一个[`Option`]类型
pub fn add_two(x: i32) -> i32 {
    x + 2
}
