require "spec_helper"

describe Classmate::DeclarationList do
  describe ".parse" do
    it "parses a valid declaration list" do
      declarations = Classmate::DeclarationList.parse("font-size: 16px;")
      expect(declarations).to be_a(Classmate::DeclarationList)
    end

    it "fails to parse an invalid declaration list" do
      expect { Classmate::DeclarationList.parse("font") }
        .to raise_error(Classmate::ParseError, "error parsing declaration list")
    end
  end

  describe ".new" do
    it "is not implemented" do
      expect { Classmate::DeclarationList.new }.to raise_error(NotImplementedError, "not implemented")
    end
  end

  describe "#proxy" do
    let(:declarations) {
      Classmate::DeclarationList.parse('background-image: url("https://example.com/foo.png")') }

    it "replaces subresource URLs when the block returns new URLs" do
      expect { declarations.proxy { |url| "https://proxy.example.com/#{url}" } }
        .to change { declarations.to_s }
          .to('background-image: url("https://proxy.example.com/https://example.com/foo.png")')
    end

    it "preserves subresource URLs when the block returns nil" do
      expect { declarations.proxy { nil } }.not_to change { declarations.to_s }
    end

    it "raises when the block returns a non-String" do
      expect { declarations.proxy { 1234 } }
        .to raise_error(TypeError, "no implicit conversion of Integer into String")
    end

    it "raises when the block raises" do
      expect { declarations.proxy { raise "Boom!" } }.to raise_error(RuntimeError, "Boom!")
    end
  end
end
