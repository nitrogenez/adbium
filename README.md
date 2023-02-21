<h1 align=center>
    <img width=15% src=.github/adbium_icon.webp>
    <p><b>ADBIUM</b></p>
</h1>

> **NOTICE**  
> ADBIUM is now in earliest stage of development, because I'm still learning Rust after many years with C/C++. I'm not giving you a guarantee that the library will work properly, so I still highly recommend you to either use other project or help in development by contributing.


## **Why ADBIUM?**
Rust libraries handling *adb server stuff* are either **broken** or **overcomplicated**.  
My goal is to make an easy-to-use library that will handle complicated server-side stuff for you.

**ADBIUM** is partially-based on [mozdevice](https://docs.rs/crate/mozdevice/latest), grabbing pieces of code from there. Not a big deal I think.


## **How to install**
+ **Add the following into your Cargo.toml**:
  - ```toml
    [dependencies.adbium]
    git = "https://github.com/nitrogenez/adbium"
    branch = "main"
    ```
    or
  - ```toml
    [dependencies.adbium]
    version = "0.1.0"
    ```

Don't worry, Cargo will do you a favor and install dependency all by itself.

**ADBIUM** for now is dependency-less library, so it will not overbloat your project.

## **Authors**
+ [nitrogenez](https://github.com/nitrogenez) - Lead developer
+ [Rikonardo](https://github.com/Rikonardo) - Sub developer

## **License**
This software is licensed under **GNU Affero General Public License v3-or-later**