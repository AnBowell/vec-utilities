use std::{cmp::Ordering, collections::HashMap, fmt::Debug, str::FromStr};

pub trait Stats<T>
where
    T: std::str::FromStr,
{
    fn mean(&self) -> Option<f64>; //f64--> f64, f32-> f32,
    fn nan_mean(&self) -> Option<f64>;
    fn median(self) -> Option<f64>;
    fn nan_median(&self) -> Option<f64>;
    fn mode(&self) -> Option<T>
    where
        <T as FromStr>::Err: Debug;

    fn nan_mode(&self) -> Option<T>
    where
        <T as FromStr>::Err: Debug;

    fn variance(&self) -> Option<f64>;
    fn nan_variance(&self) -> Option<f64>;
    fn std(&self) -> Option<f64>;
    fn nan_std(&self) -> Option<f64>;
}

trait Operations:
    Into<f64>
    + Sized
    + Clone
    + ToString
    + FromStr
    + Copy
    + for<'a> std::iter::Sum<&'a Self>
    + std::iter::Sum
{
    fn _is_nan(&self) -> bool;
    fn _total_cmp(&self, b: &Self) -> Ordering;
    fn _powf(&self, p: f64) -> f64;
    fn _sqrt(&self) -> f64;
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

            fn _powf(&self, p: f64) -> f64 {
                Into::<f64>::into(*self).powf(p)
            }

            fn _sqrt(&self) -> f64 {
                Into::<f64>::into(*self).sqrt()
            }
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

            fn _powf(&self, p: f64) -> f64 {
                Into::<f64>::into(*self).powf(p)
            }

            fn _sqrt(&self) -> f64 {
                Into::<f64>::into(*self).sqrt()
            }
        }
    };
}

impl_float_functions!(f64);
impl_float_functions!(f32);
// impl_non_float_functions!(i64);
impl_non_float_functions!(i32);
// impl_non_float_functions!(u64);
impl_non_float_functions!(u32);

impl<T> Stats<T> for Vec<T>
where
    T: Operations,
{
    fn mean(&self) -> Option<f64> {
        if self.len() > 0 {
            return Some(self.iter().map(|x| x).sum::<T>().into() / self.len() as f64);
        } else {
            return None;
        }
    }

    fn nan_mean(&self) -> Option<f64> {
        if self.len() > 0 {
            return Some(
                self.iter()
                    .map(|x| x)
                    .filter(|x| !x._is_nan())
                    .sum::<T>()
                    .into()
                    / self.iter().filter(|x| !x._is_nan()).count() as f64,
            );
        } else {
            return None;
        }
    }

    fn median(mut self) -> Option<f64> {
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
            return Some((self[mid_index - 1].into() + self[mid_index + 1].into()) / 2.0);
        }
    }

    fn nan_median(&self) -> Option<f64> {
        // Unlike median, I think we need to make a new vec in memory here, so
        // there would be no performance benefit of passing by value. Thus,
        // unlike `median`, we take a reference.

        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nans = self
            .iter()
            .filter(|&x| !x._is_nan())
            .cloned()
            .collect::<Vec<T>>();

        return no_nans.median();
    }

    fn mode(&self) -> Option<T>
    where
        <T as FromStr>::Err: Debug,
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

        return Some(mode_float.parse::<T>().unwrap());
    }

    fn nan_mode(&self) -> Option<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nans = self
            .iter()
            .filter(|&x| !x._is_nan())
            .cloned()
            .collect::<Vec<T>>();

        return no_nans.mode();
    }

    fn variance(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let mean = self.mean()?;

        return Some(
            self.iter()
                .map(|x| (Into::<f64>::into(*x) - mean).powf(2.0))
                .sum::<f64>()
                / (n as f64),
        );
    }

    fn nan_variance(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<T> = self.iter().filter(|x| !x._is_nan()).cloned().collect();

        return no_nan.variance();
    }

    fn std(&self) -> Option<f64> {
        return match self.variance() {
            Some(x) => Some(x.sqrt()),
            None => None,
        };
    }

    fn nan_std(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<T> = self.iter().filter(|x| !x._is_nan()).cloned().collect();

        return match no_nan.variance() {
            Some(x) => Some(x.sqrt()),
            None => None,
        };
    }
}
