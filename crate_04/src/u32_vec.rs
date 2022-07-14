fn main() {        
    let list1: [u32; 3] = [1, 2, 3];
    let sum1 = sum(&list1);
    println!("{:#?}", sum1.unwrap());

    let list2: [u32; 2] = [1, u32::MAX];
    let sum2 = sum(&list2);
    println!("{:#?}", sum2.unwrap());
}

fn sum(list: &[u32]) -> Option<u32> {    
    let mut total: u32 = 0;
    for item in list {        
        let (t, is_error) = total.overflowing_add(*item);          
        if is_error {
            return Some(0);
        } else {
            total = t;
        }
    }
    return Some(total);
}
