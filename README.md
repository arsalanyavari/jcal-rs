# Scal

### Modern Rust implementation of the classic [jcal](nongnu.org/jcal) utilities

This is the reimplementation of jcal/jdate unix command in Rust that originally written by [Ashkan Ghasemi in C](https://github.com/ashkang/jcal)

> [!Note]
> I have fixed the bugs in Ashkan's original code related to leap years in the new implementation and add sstat command also. This version supports the `scal`, `sdate` and `sstat` commands for all possible years.

# Installation:
Using pre‑built packages (recommended)

Head over to the [**Releases**](https://github.com/arsalanyavari/scal/releases) page and grab the asset that matches your system:

| Distro / OS     | Architecture            | File                                           |
| --------------- | ----------------------- | ---------------------------------------------- |
| Debian / Ubuntu | `x86_64`                | `{scal,sdate,sstat}-amd64.deb`                       |
|                 | `arm64`                 | `{scal,sdate,sstat}-arm64.deb`                       |
| Fedora / RHEL   | `x86_64`                | `{scal,sdate,sstat}-x86_64.rpm`                    |
|                 | `arm64`                 | `{scal,sdate,sstat}-aarch64.rpm`                   |
| Linux (any)     | `x86_64`, `arm64`       | `<arch>-unknown-linux-gnu.zip` |
| Windows 10+     | `x86_64`                | `x86_64-pc-windows-gnu.zip`      |
| macOS 11+       | `arm64` (Apple Silicon) | `aarch64-apple-darwin.zip`        |
|                 | `x86_64` (Intel)        | `x86_64-apple-darwin.zip`         |

If you download Linux packages:
```bash
# Debian / Ubuntu
sudo dpkg -i scal-*.deb
sudo dpkg -i sdate-*.deb
sudo dpkg -i sstat-*.deb

# Fedora / RHEL
sudo rpm -i scal-*.rpm
sudo rpm -i sdate-*.rpm
sudo rpm -i sstat-*.rpm
```
If you download zip file:
unzip the file then put `scal`, `sdate` and `sstat` in executable PATH or run then relative (`./scal`, `./sdate` or `./sstat`)

## Building From Source:
```
git clone https://github.com/arsalanyavari/scal.git
```
```
cd scal
```
```
cargo build --release
```
- $ **scal** or **sdate** in `target/release/` path

<br>

### scal is Shamsi (Jalali) calendar like cal command
```
$ scal -h
Usage: scal [OPTIONS] [YEAR]

Arguments:
  [YEAR]  

Options:
  -P, --pahlavi            Display year based on Pahlavi year
  -p, --persian-output     Display Farsi numbers and names
  -e, --english-days       Display English weekday names (Sa, Su, ...)
  -y, --current-year-view  Display the calendar for the current year
  -j, --julian-days        Display Julian dates (day of year)
  -h, --help               Print help
  -V, --version            Print version
```

### sdate Converts between Shamsi (Jalali) and Gregorian dates
```
$ sdate -h
Usage: sdate [OPTIONS]

Options:
  -g, --jalali-to-gregorian <YYYY/MM/DD>       Convert Jalali to Gregorian date
  -j, --gregorian-to-jalali <YYYY/MM/DD>       Convert Gregorian to Jalali date
  -u, --utc                                    Display time in UTC
  -z, --timezone <TIMEZONE>                    Set a specific timezone
  -R, --rfc2822                                Output in RFC 2822 format
  -I, --iso8601[=<PRECISION>]                  Output in ISO 8601 format
  -v, --adjustments <[+|-]val[y|m|w|d|H|M|S]>  Adjust date/time
  -h, --help                                   Print help
  -V, --version                                Print version
```

### sstat shows the stats of file in Shamsi (Jalali)
```
$ sstat -h
Usage: sstat [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  File(s) or directory(s) to get status of

Options:
  -p, --persian-output  Display Farsi numbers and names
  -l, --ls-format       Display in ls -lT format
  -n, --no-newline      Do not print trailing newline
  -r, --raw-format      Display raw numerical information
  -x, --verbose-format  Display verbose information
  -s, --shell-format    Display in shell-friendly format for `eval`
  -h, --help            Print help
  -V, --version         Print version
```

## Screenshots

<table>
  <tr>
    <td rowspan="3" width="35%">
      <img src="https://github.com/user-attachments/assets/4c9b78f3-d855-4870-94e2-d118d5ea6f5c" width="100%" alt="scal example">
    </td>
    <td width="50%">
      <img src="https://github.com/user-attachments/assets/53f3f630-fe8e-49dc-8c11-43719afc6179" width="60%" alt="scal example">
    </td>
  </tr>
  <tr>
    <td>
      <img src="https://github.com/user-attachments/assets/bfa719c7-7daf-466b-b7e0-7f5c1bb9e6cf" width="67%" alt="sdate example">
    </td>
  </tr>
  <tr>
    <td>
      <img src="https://github.com/user-attachments/assets/c7d4836d-9212-4f65-a806-4fce4ef018ce" width="100%" alt="sstat example">
    </td>
  </tr>
</table>

## Example

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
$ sstat /path/to file   # show the stats of file
$ sstat -h              # sstat help
```

## Contributing
Feel free to contribute! If you like this project, giving it a star 🌟 would make me happy and might motivate me to improve the code even further.

- Fork and Develop : Feel free to fork the repository and make your changes. If you think your changes are useful, submit a pull request so I can review and merge them into this codebase.
- Report Issues : If you notice any bugs or have suggestions for improvements, [open an issue](https://github.com/arsalanyavari/scal/issues/new). Your feedback is valuable!

`Every little contribution helps, and I appreciate your support.`

## License
[LGPLv3 LICENSE](LICENSE) © Amir Arsalan Yavari
