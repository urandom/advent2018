def reduce(data : Array(Char)) : Array(Char) | Nil
	data[1..-1].each_index do |i|
		if data[i] != data[i+1] && data[i].upcase == data[i+1].upcase
			res = data[0...i].concat(data[i+2..-1])
			return reduce(res) || res
		end
	end

	return nil
end

def filter(data : Array(Char))
	charset = Set.new data.join("").downcase.chars

	return charset.map { |c|
		dup = data.dup
		dup.delete(c)
		dup.delete(c.upcase)
		reduce(dup) || Array(Char).new
	}.map(&.size).select{ |s| s > 0 }.min
end

data = File.open("day5.input", &.gets).not_nil!.chars

data = reduce(data).not_nil!

puts "Reduced size: #{data.size}"

puts "Further reduced size #{filter(data)}"
