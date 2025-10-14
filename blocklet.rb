class Blocklet < Formula
  desc "ASCII art generator using Unicode block characters"
  homepage "https://github.com/tanav-malhotra/blocklet"
  url "https://github.com/tanav-malhotra/blocklet/archive/v0.1.3.tar.gz"
  sha256 "REPLACE_WITH_SOURCE_HASH"
  license "GPL-3.0"
  head "https://github.com/tanav-malhotra/blocklet.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    output = shell_output("#{bin}/blocklet HELLO 2>&1")
    assert_match "█", output
  end
end


