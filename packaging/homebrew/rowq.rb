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
      url "https://github.com/denisotree/rowq/releases/download/v1.0.2/rowq-v1.0.2-aarch64-apple-darwin.tar.gz"
      sha256 "7fa0c534ac48ec19d1c56f6cb1e6a62178e96ca0b38016cf616d353cdc68ae62"
      version "1.0.2"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.2/rowq-v1.0.2-x86_64-apple-darwin.tar.gz"
      sha256 "8062224b69f874fddd521e3d51fa1055e61e7c45b8e1c3737ea15abfd83b5e20"
      version "1.0.2"
    end
  end
  on_linux do
    on_arm do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.2/rowq-v1.0.2-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "13d6d75ed84fa488772fea51bcfbe0eaed7308ae5d4d90d6383858328b3885f9"
      version "1.0.2"
    end
    on_intel do
      url "https://github.com/denisotree/rowq/releases/download/v1.0.2/rowq-v1.0.2-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "72710e2fc6e83d297d210d08f4c5964f8e3536c50acf2dee58333d3cf41d5b50"
      version "1.0.2"
    end
  end

  def install
    bin.install "rowq"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rowq --version")
  end
end
