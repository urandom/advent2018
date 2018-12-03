freq: Int64 = 0
list = Array(Int64).new
File.each_line("day1.input", chomp: true) do |line|
	i = line.to_i64
	list.push i
	freq += i
end

puts "Frequency: #{freq}"

set  = Set(Int64){0}
stop = false
freq = 0
until stop
	list.each do |i|
		freq += i
		if set.includes? freq
			puts "First frequency: #{freq}"
			stop = true
			break
		end
		set.add(freq)
	end
	puts "end of iteration"
end

