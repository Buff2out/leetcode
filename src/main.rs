mod binsearch;
mod quicksort;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("binsearch") => binsearch::run(),
        Some("quicksort") => quicksort::run(),
        _ => println!("Доступные задачи: binsearch, quicksort"),
    }
}

/*
int ** arr = malloc(());
int*** ptr = &arr;
*(*(*ptr+j)+i) *ptr[i][j]

arr[i*n +m] = 
*/