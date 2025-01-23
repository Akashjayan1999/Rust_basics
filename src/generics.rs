struct Points<T,U> {
    x: T,
    y: U,
}

enum Option<T>{
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T,U> Points<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl<T> Points<f64,T> { // T is not bound to <T,U> in Points<T,U>
    fn x_f64(&self) -> f64 {
        self.x
    }
}

impl<T,U> Points<T, U> {
    fn mix<V>(self, other: Points<T, V>) -> Points<T, V> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T,U> Points<T, U> {
    fn mix2<V,W>(self, other: Points<V,W>) -> Points<V,W> {
        Points {
            x: other.x,
            y: other.y,
        }
    }
}

impl<T,U> Points<T, U> {
    fn mix3<W,V>(self, other: Points<W,V>) -> Points<T, V> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}


pub fn main() {
    let num_list = vec![1, 2, 3, 4, 5];
    let mut largest  = num_list[0];
    for num in num_list {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is {}", largest);

    let num_list = vec![102, 2, 3, 4, 5];
    let mut largest  = num_list[0];
    for num in num_list {
        if num > largest {
            largest = num;
        }
    }
    fn find_largest(list: &Vec<i32>) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() { 
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn find_largest_any<T:PartialOrd+Copy>(list: &Vec<T>) -> T {
        let mut largest = list[0];
        for &item in list.iter() { 
            if item > largest {
                largest = item;
            }
        }
        largest
}

    
        let num_list = vec![1, 2, 3, 4, 5];
        let largest = find_largest(&num_list);
        println!("The largest number is {}", largest);

        let num_list = vec![102, 2, 3, 4, 5];
        let largest = find_largest_any(&num_list);
        println!("The largest number is {}", largest);
    
       let char_vec = vec!['a', 'b', 'c', 'd', 'e'];
       let mut largest_char = find_largest_any(&char_vec);
       print!("The largest char is {}", largest_char);

     let  pl1 = Points{x: 10, y: 20};
      let pl2 = Points{x: 5.0, y: 40};
      let  pl = Points{x: 10, y: 2.0};
      let xpl1 = pl1.x();
      let xpl2 = &pl2.x();
      let pl3 = pl1.mix(pl);
      let pl4 = pl2.mix2(Points{x: 10, y: 20});

      let pl5 = Points{x: "sdf", y: "a"};
      let pl6 =pl5.mix3(Points{x: 10, y: 90});
}