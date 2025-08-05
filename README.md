# PrayerWay

**PrayerWay** is a Rust-based Waybar module that displays Islamic prayer times using the [Aladhan API](https://aladhan.com/prayer-times-api).
It is a fork of [Onizuka893/prayerbar](https://github.com/Onizuka893/prayerbar) with enhanced options, cleaner output, and prebuilt binaries for easier setup.

---

## ✨ Features

* Fast, native **Rust binary**
* Clean JSON output for **Waybar integration**
* Customizable location, method, and time offsets
* Arabic Hijri date display & 12/24-hour clock toggle
* No external dependencies

---

## 📦 Installation

### Option 1: Download Prebuilt Binary

> Go to [Releases](https://github.com/mmBesar/PrayerWay/releases) and download the latest binary for your platform.
> Then move it somewhere in your `PATH`, like `~/.local/bin/`.

```bash
install -Dm755 prayerway ~/.local/bin/prayerway
```

### Option 2: Build From Source

```bash
git clone https://github.com/mmBesar/PrayerWay.git
cd PrayerWay
cargo build --release
install -Dm755 target/release/prayerway ~/.local/bin/prayerway
```

---

## 🧪 Usage

```bash
prayerway --city Cairo --country Egypt --method 5 --ar --am-pm
```

### Sample Output

```json
{
  "text": "󱠧",
  "tooltip": "الثلاثاء 11 صَفَر 1447\n\nمواقيت الصلاة في Cairo\n\nالآن: العصر (04:38 م)\nالمغرب بعد: 2 ساعة و 7 دقيقة\n\nالفجر    : 04:39 ص\nالشروق  : 06:16 ص\nالظهر    : 01:01 م\nالعصر    : 04:38 م\nالمغرب   : 07:45 م\nالعشاء   : 09:11 م"
}
```

---

## 🧩 Waybar Integration

Add this to your Waybar config:

```json
"custom/prayer": {
  "format": "{}",
  "tooltip": true,
  "interval": 60,
  "exec": "~/.local/bin/prayerway --city Cairo --country Egypt --method 5 --ar --am-pm",
  "return-type": "json"
}
```

---

## 🧠 CLI Options

| Flag        | Description                          |
| ----------- | ------------------------------------ |
| `--city`    | City name (required)                 |
| `--country` | Country code or name (required)      |
| `--method`  | Calculation method (0–11, or custom) |
| `--ar`      | Display Hijri date in Arabic         |
| `--am-pm`   | Use 12-hour format (default is 24h)  |

---

## 🙏 Credits

This is a modified version of [Onizuka893/prayerbar](https://github.com/Onizuka893/prayerbar) with additional features and improvements.
Thanks to their clean and efficient original Rust implementation!

---

## 📄 License

**PrayerWay** is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**.

> You are free to use, modify, and share this software.
> However, if you run a modified version publicly (e.g., on a server), you **must also publish your changes**.

See [`LICENSE`](./LICENSE) for full details.
