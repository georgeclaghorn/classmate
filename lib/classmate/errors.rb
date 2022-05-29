module Classmate
  class Error < StandardError
  end

  class ParseError < Error
  end

  class MinifyError < Error
  end

  class PrintError < Error
  end
end
