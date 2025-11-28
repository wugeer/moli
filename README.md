# MoLi

MoLi 是一个基于 [ratatui](https://github.com/ratatui-org/ratatui) 的终端日历，聚焦于快速浏览公历与农历信息。应用提供整月视图、节假日与节气标注，以及一个带提示的详情面板，帮助你在无鼠标环境下完成日期导航与查询。

## 功能亮点
- **整月网格**：以周为单位展示当前月份，并突出显示今日与选中日期。
- **农历/节日细节**：在侧栏中显示干支纪年、生肖、农历月日、已覆盖的节日以及 24 节气。
- **年份与月份跨越**：使用快捷键快速切换月份、年份，或跳回今天。
- **日期跳转**：按 `g d` 打开输入框，键入 `YYYY-MM-DD` 即可跳到任意日期（支持 1900–2100 年）。
- **可配置键位**：键位提示始终展示在底部，支持通过 RON 配置覆盖默认绑定。

## 环境要求
- Rust 稳定版工具链（Edition 2024，建议 `rustup` 1.77 及以上）
- 支持 ANSI 的终端（例如 iTerm2、Windows Terminal、Alacritty 等）

## 快速开始
```bash
git clone <repo-url> moli
cd moli
cargo run
```

## 常用命令
- `cargo check`：快速语义检查。
- `cargo fmt`：格式化代码。
- `cargo clippy --all-targets --all-features`：运行 lint，捕捉常见问题。
- `cargo test`：执行所有单元与集成测试。
- `cargo run`：启动 MoLi TUI。

## 键位与操作
| 操作 | 默认键 | 说明 |
| --- | --- | --- |
| 向左/右移动 | `h` `H` / `l` `L` | 在当月网格中移动光标 |
| 向上/下移动 | `k` `K` / `j` `J` `Ctrl+j` | 按周为单位移动 |
| 上个月/下个月 | `←` `Ctrl+h` / `→` `Ctrl+l` | 跨月浏览 |
| 上一年/下一年 | `↑` / `↓` | 跨年浏览 |
| 回到今天 | `t` `T` `g g` | 光标与视图回到当前日期 |
| 跳转日期 | `g d` | 打开日期输入框，录入 `YYYY-MM-DD` |
| 退出 | `Esc` `q` `Q` | 立即退出 MoLi |

底部“快捷键”面板会根据实际绑定自动更新标签，方便在不同配置间切换。

## 自定义键位
1. 复制示例文件：
   ```bash
   mkdir -p ~/.config/moli
   cp key_bindings.example.ron ~/.config/moli/key_bindings.ron
   ```
2. 修改 `~/.config/moli/key_bindings.ron` 中对应动作的键列表（RON 语法，字符串数组）。
3. 支持设置环境变量 `MOLI_KEY_CONFIG=/path/to/key_bindings.ron` 指向任意位置。

多键序列用 `+` 连接（例如 `g+d`），MoLi 会逐键解析。配置解析失败时，应用会在终端输出错误与回退信息，请根据提示修复。

## 日期跳转提示
- 触发：按 `g d`。
- 输入：以 `YYYY-MM-DD` 录入目标日期。
- `Enter` 确认，`Esc` 取消，`Backspace` 删除字符。
- 若日期超出支持范围（1900-01-31 至 2100 年末），界面会用红色错误提示。

## 工程结构
```
src/
  main.rs        // 程序入口，负责事件循环
  app.rs         // 状态管理与农历/节日计算
  ui.rs          // ratatui 布局、控件与帮助提示
  config.rs      // 键位解析与加载
  lunar.rs       // 农历、干支、生肖与节气逻辑
key_bindings.example.ron  // 默认键位示例
```

## 贡献
欢迎通过 Issue 或 Pull Request 反馈。提交前建议依次执行：
```bash
cargo fmt &&
cargo clippy --all-targets --all-features &&
cargo test
```
如需共享键位或节日数据，也可以附上说明文件，便于其他终端用户快速复用。

## License
MoLi 以 MIT License 授权；详见仓库中的 `LICENSE` 文件。
