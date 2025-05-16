# [Scal](nongnu.org/jcal) 

### Modern Rust implementation of the classic scal/sdate utilities

This is the reimplementation of scal unix command in Rust that originally written by [Ashkan Ghasemi in C](https://github.com/ashkang/jcal)

> [!Note]
> I have fixed the bugs in Ashkan's original code related to leap years in the new implementation. However, this version only supports the `scal` and `sdate` commands with the `-j` and `-g` options for all possible years. The other options have not been implemented yet.

# Installation:
Using pre‑built packages (recommended)

Head over to the [**Releases**](https://github.com/arsalanyavari/jcal-rs/releases/tag/1.1.0) page and grab the asset that matches your system:

| Distro / OS     | Architecture            | File                                           |
| --------------- | ----------------------- | ---------------------------------------------- |
| Debian / Ubuntu | `x86_64`                | `scal-amd64.deb`                       |
|                 | `arm64`                 | `scal-arm64.deb`                       |
| Fedora / RHEL   | `x86_64`                | `scal-x86_64.rpm`                    |
|                 | `arm64`                 | `scal-aarch64.rpm`                   |
| Linux (any)     | `x86_64`, `arm64`       | `<arch>-unknown-linux-gnu.zip` |
| Windows 10+     | `x86_64`                | `x86_64-pc-windows-gnu.zip`      |
| macOS 11+       | `arm64` (Apple Silicon) | `aarch64-apple-darwin.zip`        |
|                 | `x86_64` (Intel)        | `x86_64-apple-darwin.zip`         |

If you download Linux packages:
```bash
# Debian / Ubuntu
sudo dpkg -i scal-*.deb
sudo dpkg -i sdate-*.deb

# Fedora / RHEL
sudo rpm -i scal-*.rpm
sudo rpm -i sdate-*.rpm
```
If you download zip file:
unzip the file then put `scal` and `sdate` in executable PATH or run then relative (`./scal` or `./sdate`)

## Building From Source:
```bash
git clone https://github.com/arsalanyavari/jcal-rs.git
```
```bash
cd jcal-rs
```
```bash
cargo build --release
```
- $ **scal** or **sdate** in `target/release/` path

<br>

### scal is Shamsi (Jalali) calendar like cal command
```bash
$ scal -h
Usage: scal [OPTIONS] [YEAR]

Arguments:
  [YEAR]  Shamsi (Jalali) year to display (defaults to current)

Options:
  -P, --pahlavi            Display year based on Pahlavi era
  -p, --persian-output     Use Persian numerals and month names
  -e, --english-days       Show English weekday names (Sa, Su, …)
  -y, --current-year-view  Print the calendar for the current year
  -j, --julian-days        Print Julian day‑of‑year in calendar cells
  -h, --help               Print help
  -V, --version            Show version information
```

### sdate Converts between Shamsi (Jalali) and Gregorian dates
```bash
$ sdate -h
Usage: sdate [OPTIONS]

Options:
  -g, --jalali-to-gregorian <YYYY/MM/DD>    Convert Shamsi (Jalali) to Gregorian date
  -j, --gregorian-to-jalali <YYYY/MM/DD>    Convert Gregorian to Shamsi (Jalali) date
  -h, --help                              Print help
  -V, --version                           Show version information
```

## Screenshots

<table class="table">
  <tbody>
    <tr>
      <td>
        <img src="https://github.com/user-attachments/assets/54c4707c-99f2-46aa-a719-aa5cd4863d29" width="100%" alt="images">
      </td>
      <td>
        <img src="https://github.com/user-attachments/assets/b6bfc131-12ad-4d29-bfcf-a88af05eb6ab" width="100%" alt="images">
      </td>
      <td>
        <img src="https://github.com/user-attachments/assets/21e11bc7-ce58-481a-9e7d-ac55c40cfee3" width="100%" alt="images">
      </td>
    </tr>
  </tbody>
</table>

```bash
$ scal                  # current Shamsi (Jalali) month
$ scal -h               # scal help
$ scal -y               # whole current year
$ scal 1403             # year 1403
$ scal -p -e 1303       # 1303 with Persian digits & English weekdays
$ scal 404 -P           # year 404 in Pahlavi format (1584 Pahlavi)
$ scal -j 1398 -e -P    # year 1398 in Pahlavi format with English weekdays and dayes numbers starting from 1 Farvardin
$ sdate -g 1404/01/01   # convert 1404/01/01 to 2025/03/21
$ sdate -j 2025/12/31   # convert 2025/12/31 to 1404/10/10
$ sdate -h              # sdate help
```

## Contributing
Feel free to contribute! If you like this project, giving it a star 🌟 would make me happy and might motivate me to improve the code even further.

- Fork and Develop : Feel free to fork the repository and make your changes. If you think your changes are useful, submit a pull request so I can review and merge them into this codebase.
- Report Issues : If you notice any bugs or have suggestions for improvements, [open an issue](https://github.com/arsalanyavari/jcal-rs/issues/new). Your feedback is valuable!

`Every little contribution helps, and I appreciate your support.`

## License
[LGPLv3 LICENSE](LICENSE) © Amir Arsalan Yavari
