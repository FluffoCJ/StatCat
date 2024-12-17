# StatCat - Work in progress system fetch written in rust.

Install
```
curl -sL https://raw.githubusercontent.com/fluffocj/statcat/main/install.sh | bash
```

Until I create documentation, you can see available config options [here](https://github.com/FluffoCJ/StatCat/blob/main/src/config.rs)

<details>
  <summary>Image 1</summary>
  
  ![Example](/images/image.png)
</details>

<details>
  <summary>Image 2</summary>
  
  ![Example](/images/image2.png)
</details>

<details>
  <summary>Image 3</summary>
  
  ![Example](/images/image3.png)
</details>


<details>
  <summary>Current release</summary>

# Features
- Figlet
- Allow hex codes for colors
- Custom separator in config
- Colors module
- Distro Name
- Hostname
- Username
- Package count
- CPU Module
- Shell
- Gpu Module
- Uptime module
- Terminal module
- Icons
- Color
- Date time module

</details>


<details>
  <summary>Rewrite Roadmap</summary>

# Implemented
- Modules: OS, Hostname, CPU, Packages, Kernel,
Terminal, Uptime, Username, Shell, Desktop
- Config rework: Completely custom config, users decide exactly how it is printed.
- Figlet

# To do
- Images
- Custom ASCII
- Cache figlet
- Modules: Memory, Local IP, Media, Battery, Storage, GPU,
- Add bars to memory usage

# Other info
- Bordered decoration will no longer be an option, instead users will have to insert the border icons manually in the config. (This may change)


</details>

