# CurseForge to MultiMC

A simple application to link CurseForge to MultiMC 

Useful when you just want to use CurseForge for updating modpacks/mods

⚠️ IMPORTANT ⚠️
--------------
This project will go through a rewrite using tauri or potentially be implemented in PolyMC

How it will work
----------------
It will use symbolic links to link CurseForge directory to MultiMC directory
for the selected instance

<details>
  <summary>Images (Last Updated: v0.5.2)</summary>

  ![image](https://user-images.githubusercontent.com/29931568/135523491-ed60eecd-fa5f-415e-b619-107b2724d0b2.png)
  ![image](https://user-images.githubusercontent.com/29931568/135523975-c8fa837f-cb92-4fd1-ad83-1ae27226f657.png)
  ![image](https://user-images.githubusercontent.com/29931568/135524032-4e613e18-5b06-42ef-a45d-88327edae2a5.png)
</details>

Install
-------

**Windows**
You can get the executable from [releases](https://github.com/Ricky12Awesome/curseforge_to_multimc/releases),
their will be an installer *(`.msi`)* and a standalone *(`.exe`)*

**MacOS**
Currently not implemented

**Linux**
Build it yourself, this will be updated in the future for more clear instructions.

Build
-----
Not tested, but this should work.
You need to have rust installed to build.
```
git clone https://github.com/Ricky12Awesome/curseforge_to_multimc.git
cd curseforge_to_multimc
cargo build
```
