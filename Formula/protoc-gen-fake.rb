class ProtocGenFake < Formula
  desc "A protobuf compiler plugin to generate fake data based on protobuf schema definitions"
  homepage "https://github.com/lazarillo/protoc-gen-fake"
  url "https://github.com/lazarillo/protoc-gen-fake/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "3fc8d66ac2839554bb535062eb6970107ced56837949d5c8e135ce7f9547f3dd"
  license "Apache-2.0"

  depends_on "rust" => :build
  depends_on "protobuf"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system bin/"protoc-gen-fake", "--version"
  end
end
