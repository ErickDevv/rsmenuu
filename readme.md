# rsmenuu

## **Easily create menus** on your terminal.

## **Installation**

Add to the dependencies section of your Cargo.toml file:

```toml
rsmenuu = { git = "https://github.com/ErickDevv/rsmenuu" }
```

### **Example of use**

```rust
use rsmenuu::create_menu;
use rsmenuu::MenuResult;

fn main() {
   let options: Vec<&str> = vec!["Option 1", "Option 2"];

    let menu_results: MenuResult = create_menu!(
        "Menu",
        "Select an option",
        vec!["Press 'e' to exit"],
        options,
        vec!['e']
    );
}
```
