# computor_ru
Rust version of my computor


## Integrate & Debugging under VS Code
For enable full capacity of rust under VS Code, you first have to download the source code equivalent to your rustc version, somewhere on your hard drive.

You also have to install some dependancies with cargo, and make sure that binaries install from cargo are accessible via $PATH

	cargo install racer
	cargo install rustfmt
	cargo install rustsym

In VS Code, you have to install Rusty Code and Native Debug extension and add "rust.rustLangSrcPath" in your preference, with the path to rust source code previously download.

If you are on mac os, you have to make sure that lldb-mi is enable on your mac. You can simply acheive that by launch

	ln -s /Applications/Xcode.app/Contents/Developer/usr/bin/lldb-mi /usr/local/bin/lldb-mi
