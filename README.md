# Raspberry PI GUI for the Thermal Camera (TAURI)

This project is a project of the Control Systems and Instrumentation Department
of King Monkut's University of Technology Thonburi.

Subject Code INC472.

This software can only be used in Windows OS, but in the future it will enable
other platforms to use this software.

---

### Before starting to clone the software

1. Download MS build tools 2022 or later

2. Download Rust

3. Get the latest rustup toolchain 1.77.2 or later

4. Download latest NodeJS

5. Download scoop:

   ```
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
   ```

6. After Downloading scoop, you will have to download both opencv and llvm
   ```
   scoop install opencv llvm
   ```

### After cloning the software

Don't forget to download the javascript dependencies, you can do this by
writting the command in the terminal of the same directory of the clone:

```
npm install
```
