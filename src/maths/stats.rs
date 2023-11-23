use std::{cmp::Ordering, collections::HashMap, fmt::Debug, str::FromStr};

pub trait Stats<I, O>
where
    I: std::str::FromStr,
{
    fn mean(&self) -> Option<O>; //f64--> f64, f32-> f32,
                                 // fn nan_mean(&self) -> Option<O>;
    fn median(self) -> Option<O>
    where
        I: Operations;
    // fn nan_median(&self) -> Option<O>;
    fn mode(&self) -> Option<O>
    where
        I: Operations + std::fmt::Display,
        <O as FromStr>::Err: Debug,
        O: std::str::FromStr;

    // fn nan_mode(&self) -> Option<O>
    // where
    //     <I as FromStr>::Err: Debug;

    // fn variance(&self) -> Option<O>;
    // fn nan_variance(&self) -> Option<O>;
    // fn std(&self) -> Option<O>;
    // fn nan_std(&self) -> Option<O>;
}

pub trait Operations
// Into<f64>
// + Sized
// + Clone
// + ToString
// + FromStr
// + Copy
// + for<'a> std::iter::Sum<&'a Self>
// + std::iter::Sum
{
    fn _is_nan(&self) -> bool;
    fn _total_cmp(&self, b: &Self) -> Ordering;
    // fn _powf(&self, p: &Self) -> &Self;
    // fn _sqrt(&self) -> &Self;
}

macro_rules! impl_float_functions {
    ($float:ty) => {
        impl Operations for $float {
            fn _is_nan(&self) -> bool {
                self.is_nan()
            }

            fn _total_cmp(&self, b: &Self) -> Ordering {
                self.total_cmp(b)
            }

            // fn _powf(&self, p: &Self) -> $float {
            //     self.powf(*p)
            // }

            // fn _sqrt(&self) -> $float {
            //     self.sqrt()
            // }
        }
    };
}
macro_rules! impl_non_float_functions {
    ($non_float:ty) => {
        impl Operations for $non_float {
            fn _is_nan(&self) -> bool {
                false
            }

            fn _total_cmp(&self, b: &Self) -> Ordering {
                self.cmp(b)
            }

            // fn _powf(&self, p: &Self) -> $non_float {
            //     self.pow(p.into())
            // }

            // fn _sqrt(&self) -> $non_float {
            //     self.sqrt()
            // }
        }
    };
}

impl_float_functions!(f64);
impl_float_functions!(f32);
impl_non_float_functions!(i64);
impl_non_float_functions!(i32);
impl_non_float_functions!(u64);
impl_non_float_functions!(u32);

impl<I, O> Stats<I, O> for Vec<I>
where
    I: Copy + std::str::FromStr + std::default::Default + std::ops::Add<Output = I> + Into<O>,
    O: Copy
        + std::default::Default
        + From<u8>
        + std::ops::Add<Output = O>
        + std::ops::Div<Output = O>,
{
    fn mean(&self) -> Option<O> {
        let mut count: O = Default::default();
        let mut sum: I = Default::default();

        let add = Into::<O>::into(1_u8);

        if self.is_empty() {
            return None;
        }

        for item in self {
            sum = sum + *item;
            count = count + add;
        }

        return Some(Into::<O>::into(sum) / count);
    }

    // fn nan_mean(&self) -> Option<O> {
    //     if self.len() > 0 {
    //         return Some(
    //             self.iter()
    //                 .map(|x| x)
    //                 .filter(|x| !x._is_nan())
    //                 .sum::<I>()
    //                 .into()
    //                 / self.iter().filter(|x| !x._is_nan()).count() as f64,
    //         );
    //     } else {
    //         return None;
    //     }
    // }

    fn median(mut self) -> Option<O>
    where
        I: Operations
            + Copy
            + std::str::FromStr
            + std::default::Default
            + std::ops::Add<Output = I>
            + Into<O>,
        O: Copy
            + std::default::Default
            + std::ops::Add<Output = O>
            + std::ops::Div<Output = O>
            + From<u8>,
    {
        // Median consumes self because we need to sort the vec
        // This means the programmer can choose whether to `.clone().median()` for a performance hit
        // Or `.median()` if they no longer need the `Vec` after this

        let n = self.len();

        if n == 0 {
            return None;
        }

        // See https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
        // `total_cmp` has been implemented on f32 and f64 since 1.62.0
        self.sort_by(|a, b| a._total_cmp(b));
        let mid_index = n / 2; // Note, this is automatically a floor division because of how Rust usize works
                               // In Python you would do something like `mid_index = n // 2`

        if n % 2 == 1 {
            return Some(self[mid_index].into());
        } else {
            return Some((self[mid_index - 1] + self[mid_index + 1]).into() / 2_u8.into());
        }
    }

    // fn nan_median(&self) -> Option<O> {
    //     // Unlike median, I think we need to make a new vec in memory here, so
    //     // there would be no performance benefit of passing by value. Thus,
    //     // unlike `median`, we take a reference.

    //     let n = self.len();

    //     if n == 0 {
    //         return None;
    //     }

    //     let no_nans = self
    //         .iter()
    //         .filter(|&x| !x._is_nan())
    //         .cloned()
    //         .collect::<Vec<T>>();

    //     return no_nans.median();
    // }

    fn mode(&self) -> Option<O>
    where
        I: Operations
            + Copy
            + std::str::FromStr
            + std::default::Default
            + std::ops::Add<Output = I>
            + Into<O>
            + ToString
            + std::fmt::Display
            + FromStr,
        O: Copy
            + std::default::Default
            + std::ops::Add<Output = O>
            + std::ops::Div<Output = O>
            + From<u8>
            + std::str::FromStr,
        <O as FromStr>::Err: Debug,
    {
        fn insert_map(num: String, m: &mut HashMap<String, usize>) {
            if let Some(x) = m.get_mut(&num) {
                *x += 1;
            } else {
                m.insert(num, 1);
            }
        }

        if self.len() == 0 {
            return None;
        }

        let mut m: HashMap<String, usize> = HashMap::new();

        self.iter().for_each(|x| insert_map(x.to_string(), &mut m));

        let mut mode_float = "".to_string();
        let mut mode_count = 0;

        for (k, v) in m.iter() {
            if v > &mode_count {
                mode_float = k.clone();
                mode_count = *v;
            }
        }

        return Some(mode_float.parse::<O>().unwrap());
    }

    // fn nan_mode(&self) -> Option<O>
    // where
    //     <I as FromStr>::Err: Debug,
    // {
    //     let n = self.len();

    //     if n == 0 {
    //         return None;
    //     }

    //     let no_nans = self
    //         .iter()
    //         .filter(|&x| !x._is_nan())
    //         .cloned()
    //         .collect::<Vec<T>>();

    //     return no_nans.mode();
    // }

    // fn variance(&self) -> Option<O> {
    //     let n = self.len();

    //     if n == 0 {
    //         return None;
    //     }

    //     let mean = self.mean()?;

    //     return Some(
    //         self.iter()
    //             .map(|x| (Into::<f64>::into(*x) - mean).powf(2.0))
    //             .sum::<f64>()
    //             / (n as f64),
    //     );
    // }

    // fn nan_variance(&self) -> Option<O> {
    //     let n = self.len();

    //     if n == 0 {
    //         return None;
    //     }

    //     let no_nan: Vec<T> = self.iter().filter(|x| !x._is_nan()).cloned().collect();

    //     return no_nan.variance();
    // }

    // fn std(&self) -> Option<O> {
    //     return match self.variance() {
    //         Some(x) => Some(x.sqrt()),
    //         None => None,
    //     };
    // }

    // fn nan_std(&self) -> Option<I> {
    //     let n = self.len();

    //     if n == 0 {
    //         return None;
    //     }

    //     let no_nan: Vec<I> = self.iter().filter(|x| !x._is_nan()).cloned().collect();

    //     return match no_nan.variance() {
    //         Some(x) => Some(x.sqrt()),
    //         None => None,
    //     };
    // }
}
