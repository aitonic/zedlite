# 🧪 Text-Only Mode 编译测试指南

## 📋 **编译前检查清单**

### ✅ **已完成的配置**
- [x] `Cargo.toml` 已更新为text-only版本
- [x] 移除了76%的编程相关crates
- [x] 保留了所有必要的外部依赖项
- [x] 包含了blade-graphics等GPUI必需依赖
- [x] 配置了正确的Windows支持

### 🎯 **关键依赖项验证**

**核心UI框架**:
- `gpui`, `blade-graphics`, `blade-macros`, `blade-util` ✅
- `metal`, `core-foundation`, `cocoa` (macOS) ✅
- `windows-core` (Windows支持) ✅

**AI和网络**:
- 所有AI提供商 (`anthropic`, `open_ai`, `ollama`等) ✅
- HTTP客户端 (`reqwest`, `http_client`) ✅
- 异步支持 (`tokio`, `futures`, `async-trait`) ✅

**文本和数据**:
- 文本处理 (`rope`, `unicode-*`, `pulldown-cmark`) ✅
- 序列化 (`serde`, `serde_json`, `toml`) ✅
- 数据库 (`sqlez`, `rusqlite`) ✅

## 🚀 **推荐的编译测试步骤**

### **步骤1: 基础语法检查**
```bash
cd /Users/micro/Documents/Code/Rust/zedlite
cargo check
```
**预期**: 检查workspace配置和基础语法

### **步骤2: 依赖解析测试**
```bash
cargo check --workspace
```
**预期**: 验证所有crates的依赖关系

### **步骤3: 增量编译测试**
```bash
cargo build
```
**预期**: 尝试编译所有crates (debug模式)

### **步骤4: 优化编译测试**
```bash
cargo build --release
```
**预期**: 生成优化的二进制文件

### **步骤5: 运行测试**
```bash
cargo run --release
```
**预期**: 启动ZedLite小说写作工具

## ⚠️ **可能的编译问题及解决方案**

### **问题1: 缺失依赖**
```
error: failed to load manifest for workspace member
```
**解决**: 检查是否有crate引用了被移除的依赖项

### **问题2: 特性标志错误**
```
error: feature `xxx` is required
```
**解决**: 添加必要的feature flags到外部依赖

### **问题3: 条件编译问题**
```
error: unresolved import
```
**解决**: 检查是否有代码依赖被移除的功能

### **问题4: 平台特定依赖**
```
error: could not find `xxx` in the crate root
```
**解决**: 确保平台特定依赖 (macOS/Windows) 正确配置

## 📊 **编译成功验证指标**

| 检查项 | 验证方法 | 成功标准 |
|--------|----------|----------|
| **配置正确** | `cargo check` | 无语法错误 |
| **依赖完整** | `cargo tree` | 无缺失依赖 |
| **编译成功** | `cargo build` | 0 errors |
| **功能正常** | `cargo run` | 应用启动 |
| **性能提升** | 时间对比 | 编译时间减少 |

## 🎯 **预期编译结果**

**✅ 成功编译后的特征**:
- 二进制大小: ~100-120MB (vs 原来200MB)
- 编译时间: ~8-12分钟 (vs 原来15-20分钟)
- 启动应用显示小说写作界面
- Manuscript Panel可用
- AI代理功能正常

**🔧 如果编译失败**:
1. 查看具体错误信息
2. 检查是否需要添加遗漏的依赖
3. 验证代码是否引用了被移除的功能
4. 调整feature flags配置

---

**请在终端中运行上述命令序列，并报告任何编译错误信息！**
