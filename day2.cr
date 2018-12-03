list = Array(String).new
File.each_line("day2.input", chomp: true) do |line|
	i = line
	list.push i
end

twos = 0
threes = 0
chars = Array(Array(Char)).new
list.each do |line|
	counts = line.each_char.group_by{ |c| c }.map{ |k, v| v.size }.to_set
	chars.push line.each_char.to_a
	twos+=1 if counts.includes? 2
	threes+=1 if counts.includes? 3
end

puts "Check: #{twos * threes}"

stop = false
while chars.size > 0 && !stop
	c = chars.shift
	chars.each do |c2|
		at = 0
		c.each_index do |i|
			if c.zip(c2).map { |v| v[0] != v[1] }.count { |v| v } == 1
				at = c.zip(c2).map { |v| v[0] != v[1] }.index(true)
				at = 0 if at.nil?
				break
			end
		end

		if at != 0
			c.delete_at(at)
			puts "Common: #{c.join("")}"
			stop = true
			break
		end
	end
end
