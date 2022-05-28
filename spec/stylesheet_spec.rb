require "spec_helper"

describe Classmate::Stylesheet do
  describe ".parse" do
    it "parses a valid stylesheet" do
      stylesheet = Classmate::Stylesheet.parse("body { font-size: 16px; }")
      expect(stylesheet).to be_a(Classmate::Stylesheet)
    end

    it "fails to parse an invalid stylesheet" do
      expect { Classmate::Stylesheet.parse("body") }
        .to raise_error(Classmate::ParseError, "Unexpected end of input at (unknown):0:5")
    end
  end

  describe ".new" do
    it "is not implemented" do
      expect { Classmate::Stylesheet.new }.to raise_error(NotImplementedError, "not implemented")
    end
  end

  describe "#proxy" do
    let :stylesheet do
      Classmate::Stylesheet.parse(<<~CSS)
        @font-face {
          font-family: Inter;
          src: url("https://fonts.example.com/Inter.woff2");
          font-weight: 300;
        }

        @media screen and (min-width: 900px) {
          body {
            background-image: url("https://example.com/foo.png");
          }
        }

        body {
          background-image: url("https://example.com/bar.png");
        }
      CSS
    end

    it "replaces subresource URLs when the block returns new URLs" do
      expect { stylesheet.proxy { |url| "https://proxy.example.com/#{url}" } }
        .to change { stylesheet.to_s }.to(<<~CSS)
          @font-face {
            font-family: Inter;
            src: url("https://proxy.example.com/https://fonts.example.com/Inter.woff2");
            font-weight: 300;
          }

          @media screen and (min-width: 900px) {
            body {
              background-image: url("https://proxy.example.com/https://example.com/foo.png");
            }
          }

          body {
            background-image: url("https://proxy.example.com/https://example.com/bar.png");
          }
        CSS
    end

    it "preserves subresource URLs when the block returns nil" do
      expect { stylesheet.proxy { nil } }.not_to change { stylesheet.to_s }
    end

    it "returns self" do
      expect(stylesheet.proxy { |url| "https://proxy.example.com/#{url}" }).to eq(stylesheet)
    end

    it "raises when the block returns a non-String" do
      expect { stylesheet.proxy { 1234 } }
        .to raise_error(TypeError, "no implicit conversion of Integer into String")
    end

    it "raises when the block raises" do
      expect { stylesheet.proxy { raise "Boom!" } }.to raise_error(RuntimeError, "Boom!")
    end
  end

  describe "#to_css" do
    let(:stylesheet) { Classmate::Stylesheet.parse("body { font-size: 16px; }") }

    it "serializes the stylesheet to CSS" do
      expect(stylesheet.to_css).to eq("body {\n  font-size: 16px;\n}\n")
    end
  end

  describe "#to_s" do
    let(:stylesheet) { Classmate::Stylesheet.parse("body { font-size: 16px; }") }

    it "serializes the stylesheet to CSS" do
      expect(stylesheet.to_s).to eq("body {\n  font-size: 16px;\n}\n")
    end
  end
end
