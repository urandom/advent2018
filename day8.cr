class Node
	property children, meta

	def self.from(input : Array(Int32)) : Node
		num_children, num_meta = input.shift, input.shift

		n = Node.new num_children, num_meta

		0.upto(num_children-1).each {
			n.children.push Node.from(input) 
		}

		0.upto(num_meta-1).each {
			n.meta.push input.shift
		}

		return n
	end

	def initialize(num_children : Int32, num_meta : Int32)
		@children = Array(Node).new num_children
		@meta = Array(Int32).new num_meta
	end

	def sum_meta : Int32
		@meta.sum + (@children.size > 0 ? @children.sum { |c| c.sum_meta } : 0)
	end

	def value : Int32
		return @meta.sum if @children.size == 0
		return @meta.map{ |m| m-1 }.select { |i| i >=0 }.map { |i| @children[i]? ? @children[i].value : 0 }.sum
	end

	def inspect(io : IO)
		to_s io
	end

	def to_s(io : IO)
		executed = exec_recursive(:to_s) do
			io << "[Meta: "
			io << @meta.join ", "
			io << "; Children: "
			@children.each &.inspect(io)
		    io << ']'
		end
	end
end

tree = Node.from File.read_lines("day8.input")[0].split(" ").map &.to_i

puts "Metadata sum: #{tree.sum_meta}"
puts "Value: #{tree.value}"
