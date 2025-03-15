module Expressions
	def accept(visitor)
		method_name = "visit_#{self.class.name.downcase}"
		visitor.public_send(method_name, self)
	end
end

class IntegerType
	include Expressions
	attr_reader :value

    def initialize(value)
		@value = value
	end
end

class FloatType
	include Expressions
	attr_reader :value

    def initialize(value)
		@value = value
	end
end

class BooleanType
	include Expressions
	attr_reader :value

    def initialize(value)
		@value = value
	end
end

class StringType
	include Expressions
	attr_reader :value

    def initialize(value)
		@value = value
	end
end

class CellReference
	include Expressions
	attr_reader :value

    def initialize(value)
		raise NotImplementedError, "CellReference evaluation not implemented"
	end
end

class Add
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Subtract
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Multiply
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Divide
    include Expressions
    attr_reader :left, :right

    def initialize(left, right)
        @left = left
        @right = right
    end
end

class Modulo
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Exponent
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Negate
	include Expressions
	attr_reader :operand

	def initialize(operand)
		@operand = operand
	end
end

class Land
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Lor
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Lnot
	include Expressions
	attr_reader :operand

    def initialize(operand)
		@operand = operand
	end
end

class Band
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Bor
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Bxor
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Bnot
	include Expressions
	attr_reader :operand

    def initialize(operand)
		@operand = operand
	end
end

class Lshift
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Rshift
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Eq
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Neq
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Lt
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Lteq
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Gt
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Gteq
	include Expressions
	attr_reader :left, :right

    def initialize(left, right)
		@left = left
		@right = right
	end
end

class Function
	include Expressions
	attr_reader :name, :args

    def initialize(name, *args)
		@name = name
		@args = args
	end
end
