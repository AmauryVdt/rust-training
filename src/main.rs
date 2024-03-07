// pub mod test;

// #[derive(Debug)]
// pub enum Option<T> {
//     Some(T),
//     None,
// }

// impl<T> Option<T> {
//     pub fn map<U, F>(self, f: F) -> Option<U>
//     where F: Other<T, Output = U>,
//     {
//         match self {
//             Option::Some(x) => Option::Some(f.function(x)),
//             Option::None => Option::None,
//         }
//     }
// }

// pub trait Other<T> {
//     type Output;

//     fn function(self, smthg: T) -> Self::Output;
// }

// struct MaClosureAjoutPlusVal {
//     x: i32
// }

// impl Other<i32> for MaClosureAjoutPlusVal {
//     type Output = i32;
//     fn function(self, val: i32) -> Self::Output {
//         self.x + val
//     }
// }

// fn main() {
//     let option = Option::Some(3);
//     let x = 3i32;
//     let result = option.map(MaClosureAjoutPlusVal{ x });
//     println!("{:?}", result)
// }

// ---------------------------------

// Refaite le iter_my()
// for_each()
// map()
// filter()
// collect()

// fn main (){
//     let vec = vec![1, 2, 3, 4];
//     vec.iter_custom()
//         .for_custom(|x| print!("{}", x));

//     vec.iter_custom()
//         .map_custom(|x| x*x)
//         .filter_custom(|&x| x > 2)
//         .for_custom(|x| print!("{}", x))
// }

use std::fmt::Debug;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug)]
pub struct MonIterator<T> 
where T: Clone {
    x: Vec<T>,
    state: usize,
}

impl<T> Iterator for MonIterator<T> 
where T: Clone {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.x.get(self.state + 1).map(|el| {
            self.state += 1;
            el.clone()
        })
    }
}

pub trait Custom {
    type Iterator;

    fn iter_custom(self) -> Self::Iterator;
}

impl<T> Custom for Vec<T>
where T:Clone {
    type Iterator = MonIterator<T>;

    fn iter_custom(self) -> Self::Iterator {
        MonIterator{ x: self, state: 0usize }
    }
}

pub trait Producer<T> {
    type Item;

    fn for_custom<F>(&mut self, f: F)
    where F: FnMut(T);
}

impl<T> Producer<T> for MonIterator<T>
where T: Clone + Debug {
    type Item = T;

    fn for_custom<F>(&mut self, mut f: F)
    where F: FnMut(Self::Item) {
        loop {
            match self.next() {
                Some(x) => f(x),
                None => break,
            }
        }
    }
}

fn main (){
    let vec = vec![1, 2, 3, 4];
    vec
        .iter_custom()
        .for_custom(|x| println!("{:?}", x));

//     vec.iter_custom()
//         .map_custom(|x| x*x)
//         .filter_custom(|&x| x > 2)
//         .for_custom(|x| print!("{}", x))
}