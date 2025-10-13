Name:           blocklet
Version:        0.1.2
Release:        1%{?dist}
Summary:        ASCII art generator using Unicode block characters

License:        GPLv3
URL:            https://github.com/tanav-malhotra/blocklet
Source0:        https://github.com/tanav-malhotra/blocklet/archive/v%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo

%description
Blocklet is a modern ASCII art generator that uses Unicode block
characters to create beautiful text art. Unlike traditional tools
like figlet, Blocklet uses solid blocks and box-drawing characters
for a more polished look.

Features include drop shadows, proper typographic descenders,
multi-line support, and automatic word wrapping.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 LICENSE %{buildroot}%{_docdir}/%{name}/LICENSE
install -Dm644 CHANGELOG.md %{buildroot}%{_docdir}/%{name}/CHANGELOG.md

%check
cargo test --release

%files
%license LICENSE
%doc README.md CHANGELOG.md
%{_bindir}/%{name}

%changelog
* Sun Oct 13 2024 Tanav Malhotra <tanavm2009@gmail.com> - 0.1.0-1
- Initial package release
- Features: Unicode block ASCII art with shadows
- Multi-line support via multiple arguments
- Proper typographic descenders
- Word wrapping support
- Three font variants

