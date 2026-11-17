# Homebrew formula for rowq.
#
# Lives at Formula/rowq.rb in the tap repository
#   https://github.com/denisotree/homebrew-rowq
# (tuitab keeps its own tap, denisotree/homebrew-tuitab, at 0.11.3).
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
      url "https://github.com/denisotree/rowq/releases/download/v1.0.3/rowq-v1.0.3-aarch64-apple-darwin.tar.gz"
      sha256 "e2e91cdfe82fb027f21a7a102d28137d8adbb23ecde73c0b4de383240f5d6ed5"
      version "1.0.3"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.3/rowq-v1.0.3-x86_64-apple-darwin.tar.gz"
      sha256 "89d5ecf92fa9dc01baf8523dd4df94dc179555bec053bc0e87fcd66fe17c937f"
      version "1.0.3"
    end
  end
  on_linux do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.3/rowq-v1.0.3-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "d9a098a2b6d4ba068399abd6d1286f9e9fedb028cf87cf51e012e91658ba8bdf"
      version "1.0.3"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.3/rowq-v1.0.3-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "f75ad0655a43201e254c2748abb95f3e996b8cf20d163c066be56a0cebad5ef2"
      version "1.0.3"
    end
  end

  def install
    bin.install "rowq"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rowq --version")
  end
end
