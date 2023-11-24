use std::fmt::Display;

pub fn arange<T>(start: T, end: T, step: T) -> Option<Vec<T>>
where
    T: std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::PartialOrd
        + Copy
        + Default
        + Display,
{
    if step == T::default() {
        return None;
    }

    // let vec_range = if start > end {
    //     start - end
    // } else {
    //     end - start
    // };

    // let count_usize = match (vec_range / step).try_into() {
    //     Ok(x) => x,
    //     Err(_) => 100_usize,
    // };

    // let mut output = Vec::with_capacity(count_usize);

    let mut output = Vec::new(); // Would like capacity... but numbers
    let mut count = start;
    output.push(count);

    let step_negative = step < T::default();

    loop {
        count = count + step;

        if step_negative {
            if count <= end {
                break;
            }
        } else {
            if count >= end {
                break;
            }
        }
        output.push(count);
    }

    return Some(output);
}
