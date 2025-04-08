# Utility functions for converting color in Rust

## Functions provided

### From RGBA
- rgba to hex
- rgba to hsla
- rgba to hsva
- rgba to cmyka

### To RGBA
- hex to rgba
- hsla to rgba
- hsva to rgba
- cmyka to rgba


## Sample Usage
```
fn main() -> anyhow::Result<()> {
    // rgba to hex
    let rgba = (134, 131, 213, Some(0.94));
    let hex_string = rgba_to_hex(rgba, Some(false))?;
    println!("rgba to hex: {}", hex_string);

    // rgba to hsla
    let hsla = rgba_to_hsla(rgba)?;
    println!("rgba to hsla: {:?}", hsla);

    // rgba to hsva
    let hsva = rgba_to_hsva(rgba)?;
    println!("rgba to hsva: {:?}", hsva);

    // rgba to cmyka
    let cmyka = rgba_to_cmyka(rgba)?;
    println!("rgba to cmyka: {:?}", cmyka);

    // hex to rgba
    let rgba = hex_to_rgba("8683D5f0", Some(false))?;
    println!("hex to rgba: {:?}", rgba);

    // hsla to rgba
    let hsla = (242.0, 49.0, 67.0, Some(0.94));
    let rgba = hsla_to_rgba(hsla)?;
    println!("hsla to rgba: {:?}", rgba);

    // hsva to rgba
    let hsva = (242.0, 39.0, 83.0, Some(0.94));
    let rgba = hsva_to_rgba(hsva)?;
    println!("hsva to rgba: {:?}", rgba);

    // cmyka to rgba
    let cmyka = (47.0, 0.0, 16.0, 0.0, Some(0.94));
    let rgba = cmyka_to_rgba(cmyka)?;
    println!("cmyka to rgba: {:?}", rgba);

    Ok(())
}
```
