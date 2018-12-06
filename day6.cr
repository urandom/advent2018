coords = File.read_lines("day6.input").map { |l| l.split(", ").map &.to_i }
max_x = coords.max_by(&.[0])[0]
max_y = coords.max_by(&.[1])[1]

sizes = Array(Int32).new coords.size, 0
infs = Array(Bool).new coords.size, false

0.upto(max_x).each do |x|
	0.upto(max_y).each do |y|
		dists = coords.map_with_index { |(px, py), i|
			{i, (px - x).abs + (py - y).abs}
		}.sort { |a, b| a[1] <=> b[1] }

		if dists[0][0] != dists[1][0] && dists[0][1] != dists[1][1]
			sizes[dists[0][0]] += 1

			if x == 0 || x == max_x || y == 0 || y == max_y
				infs[dists[0][0]] = true
			end
		end
	end
end

infs.each_with_index { |v, i| sizes[i] = 0 if v }
puts "Size of largest area: #{sizes.max}"
