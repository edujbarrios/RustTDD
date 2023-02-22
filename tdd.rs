#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 2), 4);
        assert_eq!(sum(5, 5), 10);
        assert_eq!(sum(-1, 1), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_string_concatenation() {
        let hello = String::from("hello");
        let world = String::from("world");
        let hello_world = String::from("hello world");
        assert_eq!(string_concatenation(hello, world), hello_world);
    }

    #[test]
    fn test_struct_equality() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 1, y: 2 };
        assert_eq!(point1, point2);
    }

    #[test]
    fn test_vector_contains() {
        let vector = vec!["one", "two", "three"];
        let value = "two";
        assert!(vector.contains(&value));
    }

    #[test]
    fn test_floating_point_comparisons() {
        let a = 0.1 + 0.2;
        let b = 0.3;
        assert!(approx_equal(a, b));
    }
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn string_concatenation(a: String, b: String) -> String {
    format!("{} {}", a, b)
}

fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

// Tests en rust
#[test]
fn test_sorting() {
    let mut unsorted = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9];
    sort(&mut unsorted);
    assert_eq!(unsorted, sorted);
}

#[test]
fn test_error_handling() {
    let result = divide(10, 0);
    assert!(result.is_err());
    match result {
        Err(e) => assert_eq!(e, "division by zero"),
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_parallel_processing() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = process_data(&data, 4);
    assert_eq!(result, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
}

#[test]
fn test_count_vowels() {
    let s = String::from("hello world");
    assert_eq!(count_vowels(&s), 3);
}

#[test]
fn test_addition_with_overflow() {
    let a = std::u32::MAX;
    let b = 1;
    let result = add(a, b);
    assert!(result.is_err());
    match result {
        Err(e) => assert_eq!(e, "addition overflow"),
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_fizz_buzz() {
    let result = fizz_buzz(20);
    let expected = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
        "11", "Fizz", "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz"
    ];
    assert_eq!(result, expected);
}


#[test]
fn test_palindrome_detection() {
    let s1 = String::from("racecar");
    let s2 = String::from("hello");
    assert_eq!(is_palindrome(&s1), true);
    assert_eq!(is_palindrome(&s2), false);
}

#[test]
fn test_mean_median_mode() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(mean(&v), 5.5);
    assert_eq!(median(&v), 5.5);
    assert_eq!(mode(&v), None);

    let v = vec![1, 1, 2, 3, 3, 3, 4, 4, 4, 4];
    assert_eq!(mean(&v), 2.9);
    assert_eq!(median(&v), 3.0);
    assert_eq!(mode(&v), Some(4));
}

#[test]
fn test_caesar_cipher() {
    let plaintext = String::from("HELLO WORLD");
    let ciphertext = caesar_cipher(&plaintext, 3);
    let expected = String::from("KHOOR ZRUOG");
    assert_eq!(ciphertext, expected);
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
}

#[test]
fn test_is_armstrong_number() {
    assert_eq!(is_armstrong_number(153), true);
    assert_eq!(is_armstrong_number(9474), true);
    assert_eq!(is_armstrong_number(1634), true);
    assert_eq!(is_armstrong_number(8208), true);
    assert_eq!(is_armstrong_number(371), true);
    assert_eq!(is_armstrong_number(1633), false);
}

#[test]
fn test_binary_search() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(binary_search(&v, &1), Some(0));
    assert_eq!(binary_search(&v, &5), Some(4));
    assert_eq!(binary_search(&v, &10), Some(9));
    assert_eq!(binary_search(&v, &11), None);
}


#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome("racecar"), true);
    assert_eq!(is_palindrome("level"), true);
    assert_eq!(is_palindrome("hello"), false);
    assert_eq!(is_palindrome("Was it a car or a cat I saw?"), true);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn test_quick_sort() {
    let mut v1 = vec![3, 2, 1, 5, 4];
    let v2 = vec![1, 2, 3, 4, 5];
    quick_sort(&mut v1);
    assert_eq!(v1, v2);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(4, 8), 4);
    assert_eq!(gcd(17, 23), 1);
}

#[test]
fn test_largest_subarray_sum() {
    let arr1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let arr2 = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    let arr3 = vec![-2, -3, -4, -1, -2, -1, -5, -3];
    assert_eq!(largest_subarray_sum(&arr1), 6);
    assert_eq!(largest_subarray_sum(&arr2), 7);
    assert_eq!(largest_subarray_sum(&arr3), -1);
}

#[test]
fn test_matrix_multiplication() {
    let mat1 = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let mat2 = Matrix::new(vec![vec![5, 6], vec![7, 8]]);
    let mat3 = Matrix::new(vec![vec![19, 22], vec![43, 50]]);
    assert_eq!(mat1 * mat2, mat3);
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
}

#[test]
fn test_knapsack() {
    let items = vec![
        Item::new(60, 10),
        Item::new(100, 20),
        Item::new(120, 30),
    ];
    assert_eq!(knapsack(&items, 50), 220);
}

#[test]
fn test_dijkstra() {
    let graph = Graph::new(5);
    graph.add_edge(0, 1, 10);
    graph.add_edge(0, 4, 5);
    graph.add_edge(1, 2, 1);
    graph.add_edge(1, 4, 2);
    graph.add_edge(2, 3, 4);
    graph.add_edge(3, 2, 6);
    graph.add_edge(3, 0, 7);
    graph.add_edge(4, 1, 3);
    graph.add_edge(4, 2, 9);
    graph.add_edge(4, 3, 2);
    let distances = dijkstra(&graph, 0);
    assert_eq!(distances, vec![Some(0), Some(8), Some(9), Some(5), Some(5)]);
}


#[test]
fn test_regex_matching() {
    assert!(regex_match("aa", "a*"));
    assert!(!regex_match("mississippi", "mis*is*p*."));
    assert!(regex_match("aab", "c*a*b"));
    assert!(regex_match("aaa", "a*a"));
    assert!(regex_match("aaa", "ab*a*c*a"));
    assert!(!regex_match("aaa", "ab*a"));
}

#[test]
fn test_sorting_algorithms() {
    let mut arr1 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr2 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr3 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr4 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr5 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr6 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr7 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let mut arr8 = vec![3, 7, 4, 9, 5, 2, 6, 1];
    let sorted_arr = vec![1, 2, 3, 4, 5, 6, 7, 9];
    bubble_sort(&mut arr1);
    assert_eq!(arr1, sorted_arr);
    selection_sort(&mut arr2);
    assert_eq!(arr2, sorted_arr);
    insertion_sort(&mut arr3);
    assert_eq!(arr3, sorted_arr);
    shell_sort(&mut arr4);
    assert_eq!(arr4, sorted_arr);
    merge_sort(&mut arr5);
    assert_eq!(arr5, sorted_arr);
    quick_sort(&mut arr6);
    assert_eq!(arr6, sorted_arr);
    heap_sort(&mut arr7);
    assert_eq!(arr7, sorted_arr);
    counting_sort(&mut arr8);
    assert_eq!(arr8, sorted_arr);
}

#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("three"), Some(&3));
    assert_eq!(map.get("four"), None);
    assert_eq!(map.remove("two"), Some(2));
    assert_eq!(map.len(), 2);
    assert_eq!(map.contains_key("two"), false);
    map.clear();
    assert_eq!(map.is_empty(), true);
}

#use regex::Regex;

#[test]
fn test_regex_captures() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2023-02-22";
    let caps = re.captures(text).unwrap();
    assert_eq!(caps.get(1).unwrap().as_str(), "2023");
    assert_eq!(caps.get(2).unwrap().as_str(), "02");
    assert_eq!(caps.get(3).unwrap().as_str(), "22");
}

#use std::thread;
#Doing 5 tests;
#[test]
fn test_threaded_functions() {
    let mut handles = vec![];

    // Test #1: Thread creation and joining
    let handle = thread::spawn(|| {
        println!("Thread #1 is running");
    });
    handles.push(handle);

    // Test #2: Thread communication using channels
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send("Hello from Thread #2").unwrap();
    });
    handles.push(handle);
    assert_eq!(rx.recv().unwrap(), "Hello from Thread #2");

    // Test #3: Thread-safe mutable state using Mutex
    let counter = std::sync::Mutex::new(0);
    let mut thread_handles = vec![];
    for _ in 0..5 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    assert_eq!(*counter.lock().unwrap(), 5);

    // Test #4: Atomic integer operations
    let counter = std::sync::atomic::AtomicUsize::new(0);
    let mut thread_handles = vec![];
    for _ in 0..5 {
        let handle = thread::spawn(move || {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 5);

    // Test #5: Thread-local storage
    thread_local!(static THREAD_LOCAL_COUNTER: std::cell::RefCell<usize> = std::cell::RefCell::new(0));
    let mut thread_handles = vec![];
    for _ in 0..5 {
        let handle = thread::spawn(|| {
            THREAD_LOCAL_COUNTER.with(|counter| {
                let mut count = counter.borrow_mut();
                *count += 1;
            });
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    THREAD_LOCAL_COUNTER.with(|counter| {
        assert_eq!(*counter.borrow(), 1);
    });
}

#use diesel::prelude::*;
#use diesel::r2d2::{ConnectionManager, Pool};
#use std::sync::Arc;

#[test]
fn test_database_connection() {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Arc::new(Pool::builder().build(manager).unwrap());
    let conn = pool.get().unwrap();
    let result = diesel::sql_query("SELECT 1").execute(&conn);
    assert!(result.is_ok());
}

#[test]
fn test_database_insert() {
    use crate::schema::users::dsl::*;

    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Arc::new(Pool::builder().build(manager).unwrap());
    let conn = pool.get().unwrap();

    let new_user = NewUser {
        username: "testuser".to_string(),
        password_hash: "123456".to_string(),
    };

    let result = diesel::insert_into(users).values(&new_user).execute(&conn);
    assert!(result.is_ok());

    let user = users.filter(username.eq("testuser")).first::<User>(&conn);
    assert!(user.is_ok());
    assert_eq!(user.unwrap().password_hash, "123456");
}

#[test]
fn test_database_update() {
    use crate::schema::users::dsl::*;

    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Arc::new(Pool::builder().build(manager).unwrap());
    let conn = pool.get().unwrap();

    let new_user = NewUser {
        username: "testuser".to_string(),
        password_hash: "123456".to_string(),
    };

    let result = diesel::insert_into(users).values(&new_user).execute(&conn);
    assert!(result.is_ok());

    let updated_user = diesel::update(users.filter(username.eq("testuser")))
        .set(password_hash.eq("abcdef"))
        .execute(&conn);
    assert!(updated_user.is_ok());

    let user = users.filter(username.eq("testuser")).first::<User>(&conn);
    assert!(user.is_ok());
    assert_eq!(user.unwrap().password_hash, "abcdef");
}

#[test]
fn test_database_delete() {
    use crate::schema::users::dsl::*;

    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Arc::new(Pool::builder().build(manager).unwrap());
    let conn = pool.get().unwrap();

    let new_user = NewUser {
        username: "testuser".to_string(),
        password_hash: "123456".to_string(),
    };

    let result = diesel::insert_into(users).values(&new_user).execute(&conn);
    assert!(result.is_ok());

    let deleted_user = diesel::delete(users.filter(username.eq("testuser"))).execute(&conn);
    assert!(deleted_user.is_ok());

    let user = users.filter(username.eq("testuser")).first::<User>(&conn);
    assert!(user.is_err());
}



func TestMain(m *testing.M) {
	// Run all tests
	m.Run()
}
