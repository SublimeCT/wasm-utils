# wasm-utils

## Contribution
此项目使用 `rust` 编写, 使用 [wasm-pack](https://rustwasm.github.io/wasm-pack/) 打包为前端 `modules`, 并在前端项目中使用

运行此项目需要安装:
- `node.js`
- `pnpm` or `npm`
- `rust`(需要添加 `wasm32-unknown-unknown` 构建目标(`target`), 如果没有此 `target`, 需要手动执行 `rustup target add wasm32-unknown-unknown`)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```bash
# 安装打包工具 wasm-pack
cargo install wasm-pack
```

### cli
```bash
# 打包
wasm-pack build --target web

# 启动前端项目
cd vite-demo && pnpm run dev
```

## 参考
- [rust wasm book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen examples](https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html)
- [wasm-pack book](https://rustwasm.github.io/docs/wasm-pack/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)