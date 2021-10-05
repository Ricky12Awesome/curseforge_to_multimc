# Link To MultiMC

A simple application to link CurseForge to MultiMC 

Useful when you just want to use CurseForge for updating modpacks/mods

TODO
----
I haven't updated this in a while, here's some things I want to add

- [ ] **Update info/warning text in app**
- [ ] **Command support**
- [ ] **FTB Launcher modpacks support**
- [ ] **Dark theme**
- [ ] **MacOS support** *(Installing via .deb or homebrew, I don't use mac so this is a bit annoying to test if it actually works or not)*
- [ ] **Linux Support** *(Building yourself should work)*

Install
-------

**Windows**
You can get the executable from [releases](https://github.com/Ricky12Awesome/curseforge_to_multimc/releases),
their will be an installer *(`.msi`)* and a standalone *(`.exe`)*

**MacOS**
Currently not implemented

**Linux**
Build it yourself, this will be updated in the future for more clear instructions.

How it will work
----------------
Uses 
[symbolic links](https://en.wikipedia.org/wiki/Symbolic_link) 
to link the "minecraft" folder to MultiMCs "minecraft" folder for a given instance

It will try and keep the same forge/fabric version when linking,
though if the version does change outside MultiMC (like when updating a modpack)
you need to make sure it's also reflected in MultiMC,
re-linking it should work as well

**Example for CurseForge**

```
{CurseForge Home}\minecraft\Instances\{instance} -> {MultiMC Home}\instances\{instance}\.minecraft
```

<details>
  <summary>Images (Last Updated: v0.5.2)</summary>

  ![image](https://user-images.githubusercontent.com/29931568/135523491-ed60eecd-fa5f-415e-b619-107b2724d0b2.png)
  ![image](https://user-images.githubusercontent.com/29931568/135523975-c8fa837f-cb92-4fd1-ad83-1ae27226f657.png)
  ![image](https://user-images.githubusercontent.com/29931568/135524032-4e613e18-5b06-42ef-a45d-88327edae2a5.png)
</details>

Limitations
-----------

- When Linking CurseForge Instances to MultiMC, Icons won't be transferred over due to how CurseForge handles instances.

Build
-----
```
git clone https://github.com/Ricky12Awesome/curseforge_to_multimc.git
cd curseforge_to_multimc
cargo build
```
