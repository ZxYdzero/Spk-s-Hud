# Spk's Hud

一个简单的 **CS:Source (CS起源)** 游戏的比赛Hud 使用Tauri和Sourcemod编写

---
## 功能

- 可以为比赛满十服务器提供基本的Hud服务以替代原本的Hud
- 方便部署, 设置服务端和客户端的端口密码打开服务器, 就可以打开软件直接连接
- 采用TCP协议
- 动态显示玩家的武器 血量 击杀 阵亡

---
## 预览

![image](https://github.com/ZxYdzero/Spk-s-Hud/blob/master/imgs/prew1.png)


---
## 使用说明

  1. 首先将Sourcemod插件导入到您的服务器内, __请确保您的服务器内有Socket拓展!__ 若没有Socket拓展则无法正常使用
  2. 然后配置服务器Cfg 在Cfg内写入sm_spkhud_port "_您想要开放的端口_ "sm_spkhud_passwd "_您想要设置的密码_"
  3. 去服务器的防火墙开放对应的端口, 要使用**TCP协议**
  4. 启动客户端, 客户端在启动的时候会尝试寻找config.toml, 若没有则会按照模版自动创建, 在config内修改各项配置
  5. 修改完毕重新打开, 若配置正确, 在起源服务器的界面应该可以看到 客户端验证成功 这样的消息, 此时就配置完毕了
  6. 在客户端内, 由于一开始是不显示背景的, 所以此时你的鼠标操作不会被无视, 单击显示的菜单, 点击快捷键可以控制菜单和hud的显示, hud显示的时候鼠标操作才会被无视
  7. 在使用的时候如果发现F1不管用, 请用Alt+Tab选择该软件, 因为F1快捷键不是全局快捷键
 
> **在Obs内的使用**
> 已经于CS起源2025/2/19的更新内出现全屏窗口化的选项, 选择该项即可显示
 
---
## 开发

#### Tauri部分:

您首先需要一个Rust Tauri和 Node.js的环境,  详细请见 [这里](https://v1.tauri.app/zh-cn/v1/guides/getting-started/setup/)

使用npm进行安装:
```
npm i
```

调试:
```
npm run tauri dev
```

使用Tauri CLI进行构建:
```
cargo tauri build
```

### Sourcemod部分

您要确保在您的scripting/include文件夹内安装了Socket和Json对应的头文件, 然后将此sp文件通过scripting内的compile编译

---
# 若您有更好的点子 请提Issue或者PR 十分感谢!!!

---
## License

GPL-3.0 License. 查看[这里](https://github.com/ZxYdzero/Spk-s-Hud/blob/master/LICENSE)获取更多信息

---

