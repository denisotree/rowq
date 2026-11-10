# Homebrew formula for rowq.
#
# Lives at Formula/rowq.rb in the tap repository
#   https://github.com/denisotree/homebrew-rowq
# (tuitab keeps its own tap, denisotree/homebrew-tuitab, at 0.11.2).
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
      url "https://github.com/denisotree/rowq/releases/download/v1.0.1/rowq-v1.0.1-aarch64-apple-darwin.tar.gz"
      sha256 "35db3ba5da8c12910b300f39c332fcd260f4bd0847581eb74b254d39d0b8273a"
      version "1.0.1"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.1/rowq-v1.0.1-x86_64-apple-darwin.tar.gz"
      sha256 "6dd4a3d92850d18d7f5ccc29b1215f2318734eb1256d742ffee85790d4bb946f"
      version "1.0.1"
    end
  end
  on_linux do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.1/rowq-v1.0.1-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "32a5f81ee971fad7694e32a688d7b6d6d9c52f262fbde3b0985fb13a6ad91deb"
      version "1.0.1"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.1/rowq-v1.0.1-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b1f83c857605e31f8c4a3235ee19d133f87a75423aab5458763af9311c1b303b"
      version "1.0.1"
    end
  end

  def install
    bin.install "rowq"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rowq --version")
  end
end
