程序代码
```
fn main() {        
    let list1: [u32; 3] = [1, 2, 3];
    let sum1 = sum(&list1);
    println!("array:{:#?} sum:{:#?}", &list1, sum1.unwrap());

    let list2: [u32; 2] = [1, u32::MAX];
    let sum2 = sum(&list2);
    println!("array:{:#?} sum:{:#?}", &list2, sum2.unwrap());
}

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

运行命令 </BR>
cargo run --bin u32_vec </BR>   

运行结果 </BR>
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_04/sum.png)
