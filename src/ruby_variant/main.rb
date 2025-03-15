require_relative 'visitors/Visitor'
require_relative 'visitors/Evaluator'
require_relative 'visitors/Serializer'
require_relative 'Expressions'

ast = Add.new(
	Multiply.new(
		IntegerType.new(5),
		Subtract.new(FloatType.new(10.5), IntegerType.new(2))
	),
	Lor.new(
		Land.new(BooleanType.new(true), BooleanType.new(false)),
		Lnot.new(BooleanType.new(false))
	)
)
evaluator = Evaluator.new
serializer = Serializer.new
puts "Serialized Expression:"
puts ast.accept(serializer)
puts "\nEvaluated Result:"
begin
puts ast.accept(evaluator)
rescue
    puts "Evaluation failed due to some unknown error"
end
