# rsmenuu

## **Easily create menus** on your terminal using ***Rust***

![rsmenuu](./rsmenuu.png)

## **Installation**

Add to the dependencies section of your Cargo.toml file:

```toml
rsmenuu = { git = "https://github.com/ErickDevv/rsmenuu" }
```

### **Example of use**

```rust
use rand::Rng;
use rsmenuu::create_menu;
use rsmenuu::Key;
use rsmenuu::MenuResult;

fn main() {
    let options: Vec<&str> = vec!["Option 1", "Option 2", "Option 3"];
    let keys: Vec<Key> = vec![Key {
        key: 'r',
        description: String::from("Press r to generate a random boolean"),
    }];
    let menu_results: MenuResult = create_menu("Hello, this is the title!", options, keys, true);

    println!("Selected: {}", menu_results.index);

    if menu_results.key == 'r' {
        println!("Random Boolean: {}", rand::thread_rng().gen::<bool>());
    }
}
```
