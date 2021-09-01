# HTTPie in Rust

对应课程章节: `04｜get hands dirty：来写个实用的CLI小工具`

## 功能需求

- 命令行解析
- 根据解析好的参数，发送一个 HTTP 请求，获得响应
- 用对用户友好的方式输出响应

## 相关库

- clap: 命令行解析
- reqwest: HTTP客户端
- colored: 格式化输出
