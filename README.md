# substrate_practice

```
//求和函数
fn sum(list: &[u32]) -> Option<u32> {    
    let mut total: u32 = 0;
    //使用 for 循环 累加
    for item in list {        
        let (t, is_error) = total.overflowing_add(*item); //使用带溢出检查的加法运算         
        if is_error {
            return Some(0);
        } else {
            total = t;
        }
    }
    Some(total)
}
```
