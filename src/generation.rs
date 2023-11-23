


pub fn arange(start: i64, end: i64, step: i64) -> Vec<i64>{


    let mut count =start;

    let mut output = Vec::with_capacity(((end - start) / step) as usize);
    output.push(count);
    loop{
        count = count + step;
        if count >= end{
            break;
        }
        output.push(count);
    }

    return output
}