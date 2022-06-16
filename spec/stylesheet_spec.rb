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

  define "#minify" do
    let :stylesheet do
      Classmate::Stylesheet.parse(<<~CSS)
        .foo {
          padding-top: 5px;
          padding-left: 5px;
          padding-right: 5px;
          padding-bottom: 5px;
        }

        .bar {
          padding: 5px;
        }
      CSS
    end

    it "minifies the stylesheet" do
      expect { stylesheet.minify }.to change { stylesheet.to_s }.to(<<~CSS)
        .foo, .bar {
          padding: 5px;
        }
      CSS
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

  shared_examples "#to_css" do |name|
    subject { stylesheet.method(name) }

    let(:stylesheet) { Classmate::Stylesheet.parse("body { font-size: 16px; }") }

    context "when minify: is not provided" do
      it "serializes the stylesheet to CSS" do
        expect(subject.call).to eq("body {\n  font-size: 16px;\n}\n")
      end
    end

    context "when minify: is true" do
      it "minifies the output" do
        expect(subject.call(minify: true)).to eq("body{font-size:16px}")
      end
    end

    context "when minify: is truthy" do
      it "minifies the output" do
        expect(subject.call(minify: :foo)).to eq("body{font-size:16px}")
      end
    end

    context "when minify: is false" do
      it "does not minify the output" do
        expect(subject.call(minify: false)).to eq("body {\n  font-size: 16px;\n}\n")
      end
    end
  end

  describe "#to_css" do
    it_behaves_like "#to_css", :to_css
  end

  describe "#to_s" do
    it_behaves_like "#to_css", :to_s
  end
end
