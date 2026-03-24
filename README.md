
<div class="image-chunk">
  <img src="logo.svg" alt="Logo">
</div>

NexisCore is a custom command-line environment and scripting system built in Rust.
It features its own structured command syntax and supports executable `.nxcr` script files.

---

## 🚀 Features

* Custom NexisCore command syntax:

  ```
  make_directory (\name = ("example")/)
  ```
* Sandboxed file system (`environment/`)
* `.nxcr` scripting support
* Double-click script execution
* Config system (`config.lst`)
* Safe mode + delete confirmations

---

## 📦 Installation

### 1. Clone the repository

```
git clone https://github.com/zeerak587-cloud/NexisCore-v0.0.2.git
cd NexisCore-v0.0.2
```

---

### 2. Install Rust

Download and install Rust from:
https://rust-lang.org

Verify installation:

```
rustc --version
```

---

### 3. Build NexisCore (Release mode)

```
cargo build --release
```

Your executable will be located at:

```
target\release\nexiscore.exe
```

---

### 4. Install NexisCore (recommended)

Move the executable to a permanent location:

```
C:\Program Files\NexisCore\nexiscore.exe
```

> ⚠️ Administrator permissions may be required

---

### 5. Add NexisCore to PATH

1. Press **Win + S**
2. Search: `Environment Variables`
3. Click **Edit the system environment variables**
4. Click **Environment Variables**
5. Under **System variables**, find `Path`
6. Click **Edit → New**
7. Add:

```
C:\Program Files\NexisCore
```

8. Click OK

---

### 6. Verify Installation

Open a new terminal and run:

```
nexiscore
```

You should see NexisCore start.

---

## 📄 `.nxcr` Script Setup (Double Click)

To make `.nxcr` files executable:

1. Right-click any `.nxcr` file
2. Click **Open with → Choose another app**
3. Click **More apps → Look for another app on this PC**
4. Select:

```
C:\Program Files\NexisCore\nexiscore.exe
```

5. ✅ Check **Always use this app**

Now `.nxcr` files will run when double-clicked.

---

## 🧠 Usage

### Interactive Mode

Run NexisCore:

```
nexiscore
```

---

### 📜 Available Commands

```
make_directory (\name = ("example")/)
open_directory (\name = ("example")/)
write_file (\name = ("file.txt")/)
open_file (\name = ("file.txt")/)
delete_file (\name = ("file.txt")/)
delete_directory (\name = ("folder")/)
view_directory
back
exit
```

---

### 📁 Example Session

```
make_directory (\name = ("projects")/)
open_directory (\name = ("projects")/)
write_file (\name = ("hello.txt")/)
```

---

## 📄 `.nxcr` Script Files

NexisCore supports executable script files.

### Example: `script.nxcr`

```
make_directory (\name = ("projects")/)
make_directory (\name = ("projects/test")/)
```

---

### ▶ Running Scripts

#### Option 1 (double-click)

Double-click the `.nxcr` file.

#### Option 2 (CLI)

```
nexiscore script.nxcr
```

---

## ⚙️ Configuration (`config.lst`)

NexisCore automatically generates a config file:

```
safe_mode = true
confirm_delete = true
symbol_for_root = "R"
```

### Options

* `safe_mode` → enables safety restrictions
* `confirm_delete` → asks before deleting files/folders
* `symbol_for_root` → prompt symbol

---

## 🛠 Project Structure

```
NexisCore/
├── src/
│   └── main.rs
├── environment/        # sandboxed file system
├── config.lst
├── Cargo.toml
```

---

## ⚠️ Notes

* All file operations are restricted to the `environment/` folder
* `.nxcr` scripts run inside the same environment
* Installing in `Program Files` may require admin rights to update the executable

---

## 🔮 Future Plans

* Variables in `.nxcr`
* Multiple parameters
* Script chaining
* Plugin system

---

## 📜 License

This project is currently under development. License to be determined.

---

## 👑 Author

Created by **Zeerak Khan**
NexisCore Project
