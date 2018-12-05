def reduce(data : Array(Char)) : Array(Char) | Nil
	data[1..-1].each_index do |i|
		if data[i] != data[i+1] && data[i].upcase == data[i+1].upcase
			res = data[0...i].concat(data[i+2..-1])
			return reduce(res) || res
		end
	end

	return nil
end

data = File.open("day5.input", &.gets).not_nil!.chars

puts "Reduced size: #{reduce(data).try &.size}"
