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
```bash
git clone https://github.com/arsalanyavari/scal.git
```
```bash
cd scal
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
```bash
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
```bash
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
