require "spec_helper"

describe Classmate::StyleAttribute do
  describe ".parse" do
    it "parses a valid style attribute" do
      attribute = Classmate::StyleAttribute.parse("font-size: 16px;")
      expect(attribute).to be_a(Classmate::StyleAttribute)
    end

    it "fails to parse an invalid style attribute" do
      expect { Classmate::StyleAttribute.parse("font") }
        .to raise_error(Classmate::ParseError, "Unexpected end of input at :0:5")
    end
  end

  describe ".new" do
    it "is not implemented" do
      expect { Classmate::StyleAttribute.new }.to raise_error(NotImplementedError, "not implemented")
    end
  end

  define "#minify" do
    let :attribute do
      Classmate::StyleAttribute.parse(<<~STRING)
        padding-top: 5px;
        padding-left: 5px;
        padding-right: 5px;
        padding-bottom: 5px;
      STRING
    end

    it "minifies the style attribute" do
      expect { attribute.minify }.to change { stylesheet.to_s }.to("padding: 5px;")
    end
  end

  describe "#proxy" do
    let(:attribute) {
      Classmate::StyleAttribute.parse('background-image: url("https://example.com/foo.png")') }

    it "replaces subresource URLs when the block returns new URLs" do
      expect { attribute.proxy { |url| "https://proxy.example.com/#{url}" } }
        .to change { attribute.to_s }
          .to('background-image: url("https://proxy.example.com/https://example.com/foo.png")')
    end

    it "preserves subresource URLs when the block returns nil" do
      expect { attribute.proxy { nil } }.not_to change { attribute.to_s }
    end

    it "returns self" do
      expect(attribute.proxy { |url| "https://proxy.example.com/#{url}" }).to eq(attribute)
    end

    it "raises when the block returns a non-String" do
      expect { attribute.proxy { 1234 } }
        .to raise_error(TypeError, "no implicit conversion of Integer into String")
    end

    it "raises when the block raises" do
      expect { attribute.proxy { raise "Boom!" } }.to raise_error(RuntimeError, "Boom!")
    end
  end
end
