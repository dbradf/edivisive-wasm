# WASM binding for E-Divisive means 

WASM binding for this [E-Divisive Means implementation](https://github.com/dbradf/edivisive-rs).

## Usage

```javascript
import {change_points} from "wasm-edivisive";

const series = [0, 0, 0, 0, 0, 1, 1, 1];

const cp = change_points(series);

console.log(cp);  // [5]
```
