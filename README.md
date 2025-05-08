# [jcal](nongnu.org/jcal) unix command in Rust
This is the reimplementation of jcal unix command in Rust that originally written by [Ashkan Ghasemi in C](https://github.com/ashkang/jcal)

> [!Note]
> I have fixed the bugs in Ashkan's original code related to leap years in the new implementation. However, this version only supports the `jcal` and `jdate` commands with the `-j` and `-g` options for all possible years. The other options have not been implemented yet.

## Setting up:
- run `cargo build --release`
- run `jcal` or `jdate` in `target/release/` path

## Example
<img width="400" alt="image" src="https://github.com/user-attachments/assets/54c4707c-99f2-46aa-a719-aa5cd4863d29" />

<img width="833" alt="image" src="https://github.com/user-attachments/assets/b6bfc131-12ad-4d29-bfcf-a88af05eb6ab" />

<img width="586" alt="image" src="https://github.com/user-attachments/assets/21e11bc7-ce58-481a-9e7d-ac55c40cfee3" />

## Contributing
Feel free to contribute! If you like this project, giving it a star would make me happy and might motivate me to improve the code even further.

- Fork and Develop : Feel free to fork the repository and make your changes. If you think your changes are useful, submit a pull request so I can review and merge them into this codebase.
- Report Issues : If you notice any bugs or have suggestions for improvements, [open an issue](https://github.com/arsalanyavari/jcal-rs/issues/new). Your feedback is valuable!

`Every little contribution helps, and I appreciate your support.`

## License
[MIT LICENSE](LICENSE) Amir Arsalan Yavari
