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
      sha256 "FILL_IN_AFTER_RELEASE"
      version "1.0.0"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-x86_64-apple-darwin.tar.gz"
      sha256 "FILL_IN_AFTER_RELEASE"
      version "1.0.0"
    end
  end
  on_linux do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "FILL_IN_AFTER_RELEASE"
      version "1.0.0"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.0/rowq-v1.0.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "FILL_IN_AFTER_RELEASE"
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
