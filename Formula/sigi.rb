class Sigi < Formula
  desc "Organizing tool for terminal lovers that hate organizing"
  license "GPL-2.0-only"
  homepage "https://github.com/hiljusti/sigi"
  url "https://crates.io/api/v1/crates/sigi/3.1.1/download"
  sha256 "0f0b35c1d21492eff7b90bee47651293b11a48dba86780586082ae686af9a9ba"
  head "https://github.com/hiljusti/sigi.git"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"

    bin.install "target/release/sigi"

    man1.install "sigi.1"
  end

  test do
    system "#{bin}/sigi", "--version"
  end
end
