# rsmenuu

## Create menus easily on your terminal using Rust

### Installation

Add this to your `Cargo.toml`:

```toml
rsmenuu = { git = "https://github.com/ErickDevv/rsmenuu" }
```

### Usage

```rust
use rsmenuu::create_menu;
use rsmenuu::MenuResult;

fn main() {
    let options = vec!["Option 1", "Option 2", "Option 3"];
    let keys = vec!['h'];
    let menu: MenuResult = create_menu("Title", options, keys);
    println!("Selected: {}", menu.index);
}
```
