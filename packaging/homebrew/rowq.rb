# Homebrew formula for rowq.
#
# Lives at Formula/rowq.rb in the tap repository
#   https://github.com/denisotree/homebrew-rowq
# (tuitab keeps its own tap, denisotree/homebrew-tuitab, at 0.11.1).
#
# Users install via:
#   brew tap denisotree/rowq
#   brew install rowq
#
# /release fills in the version, URLs and sha256 values for each release.

class Rowq < Formula
  desc "Query rows in tables, documents and databases from the terminal"
  homepage "https://rowq.dev"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-aarch64-apple-darwin.tar.gz"
      sha256 "68c9ccf59ccf82f6445cffae158b7f5edca19b7470be6deb818e3ff679914f0d"
      version "1.0.0"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-x86_64-apple-darwin.tar.gz"
      sha256 "8aee249d67d2362640925bfe1a3599e06d991016bdb6b57b7dca4886979d431b"
      version "1.0.0"
    end
  end
  on_linux do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "4676aa1b301d690be418e6bc0b07808c03157b0e19e0ffe4d452494a777af4d1"
      version "1.0.0"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "7c4a769130633b5c2dccdcc921e04340322237725ecec4d6bba190779704876a"
      version "1.0.0"
    end
  end

  def install
    bin.install "rowq"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rowq --version")
  end
end
