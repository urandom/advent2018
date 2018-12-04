require "bit_array"

class Shift
	getter id, awake

	def initialize(@id : Int32)
		@awake = BitArray.new(60, true)
	end

	def add_activity(time : Time, awake? : Bool)
		m = time.minute
		m.upto(@awake.size-1) { |i| @awake[i] = awake? }
	end
end

lines = Array(String).new
File.each_line("day4.input", chomp: true) do |line|
	lines.push line
end

shifts = Array(Shift).new
lines.sort.each do |line|
	if md = /\[(.*)\] (?:Guard #([\d]+)|(falls)|(wakes))/.match(line)
		time = Time.parse(md[1], "%Y-%m-%d %H:%M", Time::Location::UTC)
		shift : Shift
		if md[2]?
			time = time.add_span(3600, 0).at_beginning_of_day
			shifts.push Shift.new(md[2].to_i32)
		end

		shift = shifts[-1]
		shift.add_activity(time, md[3]? == nil)
	end
end

guards = shifts.group_by { |s| s.id }
sorted_guards = guards.map do |id, shifts|
	{id, shifts.map { |s| s.awake.count { |v| !v } }.sum}
end

max_awake = sorted_guards.max_by { |v| v[1] }
puts "Most sleepy guard #{max_awake[0]} with minutes #{max_awake[1]}"

shifts = guards[max_awake[0]].map{ |s| s.awake}
max_minute = {0, 0}
0.upto(shifts[0].size-1).each do |m|
	c = shifts.map { |s| s[m] }.count { |v| !v }
	if c > max_minute[1]
		max_minute = {m, c}
	end
end

puts "Max sleepy minute #{max_minute[0]}"
puts "Answer #{max_awake[0] * max_minute[0]}"
