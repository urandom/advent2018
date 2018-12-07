steps = File.read_lines("day7.input").map do |line|
/Step ([^ ]+).*step ([^ ]+) can begin./.match(line).try do |m|
 {m[1][0], m[2][0]}
end
end.map &.not_nil!

parents = Hash(Char, Set(Char)).new
children = Hash(Char, Set(Char)).new

steps.each do |(p, c)|
	set = parents.fetch(c, Set(Char).new)
	set.add p
	parents[c] = set

	set = children.fetch(p, Set(Char).new)
	set.add c
	children[p] = set
end

chars = Array(Char).new
available = children.select { |key| !parents[key]? }.keys.sort

while available.size > 0
	p = available.shift
	c = (children[p]? || Set(Char).new).to_a.sort

	if chars.includes? p
		next
	end

	chars.push p
	available = available.to_set
	c.each do |v|
		add = true
		if parents[v]?
			add = parents[v].select{ |ch| !chars.includes? ch }.size == 0
		end
		available.add v if add
	end

	available = available.to_a.sort
end

puts "Directions #{chars.join("")}"
